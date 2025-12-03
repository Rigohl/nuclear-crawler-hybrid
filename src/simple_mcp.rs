//! Simple MCP Server for Nuclear Crawler Hybrid
//! Minimal implementation for testing MCP functionality

use crate::ai_smart::{AIConfig, AISmart};
use crate::deep_web_search::{
    DeepWebSearch, DeepWebSearchConfig, DeepWebSearchType, DeepWebSource,
};
use crate::intelligent_storage::{IntelligentStorage, SearchResultEntry};
use crate::nuclear_scraper::{NuclearConfig, NuclearScraper};
use crate::scan_project::ProjectScanner;
use crate::stats::StatsSystem;
use crate::web_search::{WebSearch, WebSearchConfig};
use anyhow::Result;
use regex::Regex;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
#[allow(unused_imports)]
use std::io::BufRead;
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

/// Simple MCP Server with Nuclear Power
pub struct SimpleMcpServer {
    #[allow(dead_code)]
    nuclear_scraper: Arc<NuclearScraper>,
    web_search: Arc<WebSearch>,
    stats_system: Arc<StatsSystem>,
    storage: Arc<IntelligentStorage>,
    project_scanner: Arc<ProjectScanner>,
    deep_web_search: Arc<DeepWebSearch>,
}

impl SimpleMcpServer {
    /// Create new MCP server with full Nuclear power
    pub fn new() -> Result<Self> {
        // Logs van a stderr, no stdout (MCP protocol requirement)
        // Silent init (RUST_LOG=off)

        // Almacenamiento Inteligente (crear primero para pasarlo al scraper)
        let storage = Arc::new(IntelligentStorage::new(None)?);

        // Nuclear Scraper con storage
        let nuclear_config = NuclearConfig::default();
        let nuclear_scraper = Arc::new(NuclearScraper::new_with_storage(
            nuclear_config,
            Some(storage.clone()),
        )?);

        // Web Search con storage
        let web_search = Arc::new(WebSearch::new_with_storage(Some(storage.clone()))?);

        // AI Smart
        let ai_config = AIConfig::default();
        let ai_smart = Arc::new(AISmart::new(ai_config));

        // Stats System
        let stats_system = Arc::new(StatsSystem::new(
            web_search.clone(),
            nuclear_scraper.clone(),
            ai_smart,
        ));

        // Project Scanner
        let project_scanner = Arc::new(ProjectScanner::new(web_search.clone()));

        // Deep Web Search
        let deep_web_search = Arc::new(DeepWebSearch::new(
            nuclear_scraper.clone(),
            web_search.clone(),
        ));

        // Ready

        Ok(Self {
            nuclear_scraper,
            web_search,
            stats_system,
            storage,
            project_scanner,
            deep_web_search,
        })
    }

    /// Run the MCP server (async version for when runtime already exists)
    pub async fn run_async() -> Result<()> {
        let server = Self::new()?;
        server.run_server().await
    }

    /// Run async server
    async fn run_server(mut self) -> Result<()> {
        // Logs van a stderr, no stdout (MCP protocol requirement)
        // Listening

        let stdin = tokio::io::stdin();
        let mut stdin = tokio::io::BufReader::new(stdin);
        let mut stdout = tokio::io::stdout();
        let mut line = String::new();
        let mut _initialized = false;

        loop {
            line.clear();
            let bytes_read = stdin.read_line(&mut line).await?;

            if bytes_read == 0 {
                break; // EOF
            }

            // Skip empty lines
            if line.trim().is_empty() {
                continue;
            }

            let request: Value = match serde_json::from_str(&line) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!(
                        "Error parsing JSON-RPC request: {} | Line: {}",
                        e,
                        line.trim()
                    );
                    continue;
                }
            };

            let method = request["method"].as_str().unwrap_or("");

            // Handle notifications (no response needed)
            if request["id"].is_null() {
                if method == "notifications/initialized" {
                    _initialized = true;
                    // Client OK
                }
                continue;
            }

            // Clone request before moving it
            let request_clone = request.clone();
            let response = self.handle_request(request_clone).await;

            // Always send response if it has content (request with id or error)
            if !response["id"].is_null() || response.get("error").is_some() {
                let response_json = serde_json::to_string(&response)?;
                stdout.write_all(response_json.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
                // Flush inmediatamente para asegurar que Cursor reciba la respuesta
                stdout.flush().await?;
                // Log solo en stderr para no contaminar stdout
                // Response sent
            }
        }

        Ok(())
    }

    /// Handle JSON-RPC request (público para uso en servidor HTTP)
    pub async fn handle_request(&mut self, request: Value) -> Value {
        // 🔑 MCP 2025: Distinguish notifications (no id) from requests (with id)
        let has_id = request.get("id").map(|v| !v.is_null()).unwrap_or(false);
        let id = request.get("id").cloned().unwrap_or(Value::Null);
        
        // Standard MCP 2025 notifications to ignore silently
        let standard_notifications = [
            "logging/setLevel",
            "notifications/progress",
            "notifications/resources/list_changed",
        ];
        
        let method = request["method"].as_str().unwrap_or("");
        
        // If it's an unknown notification (no id), ignore silently
        if !has_id && standard_notifications.contains(&method) {
            return json!({});
        }

        // Validate JSON-RPC 2.0 (permitir null para notificaciones)
        if !request["jsonrpc"].is_null() && request["jsonrpc"].as_str() != Some("2.0") {
            return json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32600,
                    "message": "Invalid Request: jsonrpc must be '2.0'"
                }
            });
        }

        match method {
            "initialize" => {
                // Validate params
                let params = &request["params"];
                let protocol_version = params["protocolVersion"].as_str().unwrap_or("");

                // Accept 2025-06-18 (MCP Protocol 2025)
                let supported_versions = ["2025-06-18", "2024-11-05"];
                if !supported_versions.contains(&protocol_version) {
                    return json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {
                            "code": -32602,
                            "message": format!("Invalid protocol version: {}. Supported: 2024-11-05, 2025-06-18", protocol_version)
                        }
                    });
                }

                // Use the client's protocol version in response
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": protocol_version,
                        "capabilities": {
                            "tools": {
                                "listChanged": true
                            },
                            "resources": {},
                            "prompts": {},
                            "sampling": {}
                        },
                        "serverInfo": {
                            "name": "nuclear-scraper-web",
                            "version": "0.1.0"
                        }
                    }
                })
            }

            "tools/list" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": [
                            {
                                "name": "websearch",
                                "description": "🔥 BÚSQUEDA WEB MASIVA NUCLEAR: Usa TODO el poder (Go FFI paralelismo extremo + Zig SIMD parsing ultra-rápido + JAX aceleración GPU/TPU + IA ranking inteligente + Stealth anti-detección). 200+ URLs simultáneas, sin límites, máxima velocidad.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "query": {
                                            "type": "string",
                                            "description": "Término de búsqueda"
                                        },
                                        "sources": {
                                            "type": "array",
                                            "items": {"type": "string"},
                                            "description": "Fuentes a buscar (github.com, stackoverflow.com, reddit.com, dev.to, medium.com, etc.)",
                                            "default": ["github.com", "stackoverflow.com", "dev.to", "reddit.com"]
                                        },
                                        "max_results": {
                                            "type": "integer",
                                            "description": "Máximo número de resultados (0 = sin límite, modo NUCLEAR EXTREMO)",
                                            "default": 0
                                        },
                                        "use_ai_ranking": {
                                            "type": "boolean",
                                            "description": "Usar IA para ranking inteligente y filtrado de resultados",
                                            "default": true
                                        },
                                        "use_stealth": {
                                            "type": "boolean",
                                            "description": "Usar técnicas stealth anti-detección",
                                            "default": true
                                        },
                                        "parallel_mode": {
                                            "type": "string",
                                            "enum": ["normal", "extreme", "nuclear"],
                                            "description": "Modo de paralelismo: normal (50 URLs), extreme (100 URLs), nuclear (200+ URLs)",
                                            "default": "nuclear"
                                        }
                                    },
                                    "required": ["query"]
                                }
                            },
                            {
                                "name": "ultimas_busquedas",
                                "description": "Obtiene las últimas búsquedas guardadas en el historial inteligente",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "limit": {
                                            "type": "integer",
                                            "description": "Número máximo de búsquedas a retornar",
                                            "default": 10
                                        }
                                    }
                                }
                            },
                            {
                                "name": "stats",
                                "description": "📊 Estadísticas y métricas del NUCLEAR CRAWLER WEB: requests totales, URLs crawled, data captured, velocidad, performance, cache hits, errores, etc.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "type": {
                                            "type": "string",
                                            "description": "Tipo de estadísticas",
                                            "enum": ["full", "recent", "performance", "storage", "ai_analysis"],
                                            "default": "full"
                                        },
                                        "period": {
                                            "type": "string",
                                            "enum": ["last_hour", "last_day", "last_week", "all_time"],
                                            "description": "Período de tiempo para las estadísticas",
                                            "default": "all_time"
                                        }
                                    }
                                }
                            },
                            {
                                "name": "analizar_proyecto",
                                "description": "Analiza un proyecto local o remoto y busca librerías/opciones relevantes en la web",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "path": {
                                            "type": "string",
                                            "description": "Ruta local del proyecto a analizar",
                                            "default": "."
                                        },
                                        "query_extra": {
                                            "type": "string",
                                            "description": "Query adicional para la búsqueda web de recomendaciones",
                                            "default": ""
                                        },
                                        "max_recommendations": {
                                            "type": "integer",
                                            "description": "Máximo número de recomendaciones",
                                            "default": 5
                                        }
                                    },
                                    "required": ["path"]
                                }
                            },
                            {
                                "name": "urls_visitadas",
                                "description": "Obtiene el historial de URLs visitadas",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "limit": {
                                            "type": "integer",
                                            "description": "Número máximo de URLs a retornar",
                                            "default": 100
                                        }
                                    }
                                }
                            },
                            {
                                "name": "scan_project",
                                "description": "🔍 SCAN COMPLETO INTELIGENTE: Escanea proyectos/archivos mostrando errores con líneas exactas, detecta duplicados, mocks, código incompleto. Usa IA para recomendaciones y ayuda a finalizar proyectos. Busca en internet info sobre archivos/librerías/ideas. Si es MD, busca cómo implementar la idea.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "project_path": {
                                            "type": "string",
                                            "description": "Ruta del proyecto o archivo a escanear",
                                            "default": "."
                                        },
                                        "scan_type": {
                                            "type": "string",
                                            "enum": ["full", "errors", "duplicates", "mocks", "incomplete", "recommendations"],
                                            "description": "Tipo de scan: full (todo), errors (solo errores), duplicates (duplicados), mocks (detectar mocks), incomplete (código incompleto), recommendations (solo recomendaciones)",
                                            "default": "full"
                                        },
                                        "search_web_info": {
                                            "type": "boolean",
                                            "description": "Buscar en internet información sobre archivos, librerías y ejemplos de código",
                                            "default": true
                                        },
                                        "search_solutions": {
                                            "type": "boolean",
                                            "description": "Buscar soluciones automáticas en la web para errores encontrados",
                                            "default": true
                                        },
                                        "find_duplicates": {
                                            "type": "boolean",
                                            "description": "Buscar líneas duplicadas y código repetido",
                                            "default": true
                                        },
                                        "detect_mocks": {
                                            "type": "boolean",
                                            "description": "Detectar código mock, placeholders y funcionalidades incompletas",
                                            "default": true
                                        },
                                        "ai_recommendations": {
                                            "type": "boolean",
                                            "description": "Usar IA para generar recomendaciones inteligentes de mejora",
                                            "default": true
                                        },
                                        "help_complete_project": {
                                            "type": "boolean",
                                            "description": "Ayudar a finalizar el proyecto con sugerencias concretas",
                                            "default": true
                                        },
                                        "analyze_markdown": {
                                            "type": "boolean",
                                            "description": "Si es MD, analizar la idea y buscar cómo implementarla",
                                            "default": true
                                        },
                                        "max_recommendations": {
                                            "type": "integer",
                                            "description": "Máximo número de recomendaciones",
                                            "default": 10
                                        }
                                    },
                                    "required": ["project_path"]
                                }
                            },
                            {
                                "name": "deep_web_search",
                                "description": "🌐 BÚSQUEDA PROFUNDA PREMIUM: Acceso REAL a contenido premium/pago, papers académicos, repositorios privados, bases de datos técnicas. Usa bypass legal de paywalls, Tor, proxies, técnicas avanzadas + IA para encontrar métodos de acceso.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "query": {
                                            "type": "string",
                                            "description": "Término de búsqueda"
                                        },
                                        "search_type": {
                                            "type": "string",
                                            "enum": ["code", "intelligence", "premium", "academic", "all"],
                                            "description": "Tipo: code (repos privados), intelligence (papers/investigación), premium (contenido pago), academic (papers académicos), all (todo)",
                                            "default": "all"
                                        },
                                        "sources": {
                                            "type": "array",
                                            "items": {
                                                "type": "string",
                                                "enum": ["academic", "technical_db", "code_repos", "specialized_forums", "digital_libraries", "archives", "tor", "premium_apis", "paywalled_sites", "sci_hub", "libgen", "arxiv", "github_private"]
                                            },
                                            "description": "Fuentes específicas de deep web (vacío = todas disponibles)",
                                            "default": []
                                        },
                                        "max_results": {
                                            "type": "integer",
                                            "description": "Máximo número de resultados",
                                            "default": 20
                                        },
                                        "find_access_methods": {
                                            "type": "boolean",
                                            "description": "Buscar métodos legales de acceso al contenido premium (bypass paywalls, mirrors, etc.)",
                                            "default": true
                                        },
                                        "use_bypass_techniques": {
                                            "type": "boolean",
                                            "description": "Usar técnicas de bypass legal (archive.org, outline.com, 12ft.io, etc.)",
                                            "default": true
                                        },
                                        "use_tor": {
                                            "type": "boolean",
                                            "description": "Usar red Tor para acceso a contenido deep web",
                                            "default": false
                                        },
                                        "use_ai_access_finder": {
                                            "type": "boolean",
                                            "description": "Usar IA para encontrar métodos creativos de acceso al contenido",
                                            "default": true
                                        },
                                        "search_alternatives": {
                                            "type": "boolean",
                                            "description": "Buscar alternativas gratuitas/open-source al contenido premium",
                                            "default": true
                                        }
                                    },
                                    "required": ["query"]
                                }
                            }
                        ]
                    }
                })
            }

            "tools/call" => {
                let params = &request["params"];
                let tool_name = params["name"].as_str().unwrap_or("");
                let args = params["arguments"].clone();

                let result = self.handle_tool_call(tool_name, args).await;

                match result {
                    Ok(data) => json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "content": [
                                {
                                    "type": "text",
                                    "text": serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string())
                                }
                            ]
                        }
                    }),
                    Err(e) => json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {
                            "code": -32000,
                            "message": format!("Tool error: {}", e)
                        }
                    }),
                }
            }

            _ => {
                // Only respond with error if it's a request (has id), not a notification
                if has_id {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {
                            "code": -32601,
                            "message": format!("Method not found: {}", method)
                        }
                    })
                } else {
                    // Unknown notification - ignore silently
                    json!({})
                }
            }
        }
    }

    /// Handle tool calls
    pub async fn handle_tool_call(&mut self, tool_name: &str, args: Value) -> Result<Value> {
        match tool_name {
            "websearch" => self.handle_websearch(args).await,
            "ultimas_busquedas" => self.handle_ultimas_busquedas(args).await,
            "stats" => self.handle_stats(args).await,
            "analizar_proyecto" => self.handle_analizar_proyecto(args).await,
            "urls_visitadas" => self.handle_urls_visitadas(args).await,
            "scan_project" => self.handle_scan_project(args).await,
            "deep_web_search" => self.handle_deep_web_search(args).await,
            _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_name)),
        }
    }

    /// Handle websearch tool
    pub async fn handle_websearch(&mut self, args: Value) -> Result<Value> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing query parameter"))?;

        let sources: Vec<String> = args["sources"]
            .as_array()
            .unwrap_or(&vec![
                json!("github.com"),
                json!("stackoverflow.com"),
                json!("dev.to"),
            ])
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        let sources_clone = sources.clone();

        // Fuentes prioritarias (se buscan primero y tienen mayor peso)
        let priority_sources: Vec<String> = args["priority_sources"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        // max_results: 0 = sin límite
        let max_results = args["max_results"]
            .as_u64()
            .map(|v| v as usize)
            .unwrap_or(0); // Default: sin límite

        let use_ai = args["use_ai"].as_bool().unwrap_or(true);
        let use_stealth = args["use_stealth"].as_bool().unwrap_or(true);

        let config = WebSearchConfig {
            query: query.to_string(),
            priority_sources: priority_sources.clone(),
            sources: sources.clone(),
            max_results,
            use_ai,
            use_stealth,
            max_parallel: 10000, // NUCLEAR: 10K paralelo
            timeout_secs: 60,    // 🔥 60 segundos TOTAL
            max_urls: 100,       // 🔥 Máximo 100 URLs
        };

        let results = self.web_search.search(config).await?;

        // Guardar en DB inteligente
        let search_entries: Vec<SearchResultEntry> = results
            .iter()
            .map(|r| SearchResultEntry {
                url: r.url.clone(),
                title: r.title.clone(),
                description: r.description.clone(),
                relevance: r.relevance,
                quality_score: r.quality_score,
                source: r.source.clone(),
            })
            .collect();

        let _search_id = self
            .storage
            .save_search(
                query,
                search_entries,
                Some({
                    let mut meta = HashMap::new();
                    meta.insert(
                        "sources".to_string(),
                        serde_json::to_string(&sources_clone).unwrap_or_default(),
                    );
                    meta.insert("use_ai".to_string(), use_ai.to_string());
                    meta.insert("use_stealth".to_string(), use_stealth.to_string());
                    meta
                }),
            )
            .await?;

        Ok(json!({
            "query": query,
            "results_count": results.len(),
            "results": results.into_iter().map(|r| json!({
                "url": r.url,
                "title": r.title,
                "description": r.description,
                "relevance": r.relevance,
                "quality_score": r.quality_score,
                "source": r.source
            })).collect::<Vec<_>>()
        }))
    }

    /// Handle universal search tool
    #[allow(dead_code)]
    async fn handle_search(&mut self, args: Value) -> Result<Value> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing query parameter"))?;

        let target = args["target"].as_str().unwrap_or("all");
        let path = args["path"].as_str().unwrap_or(".");
        let max_results = args["max_results"].as_u64().unwrap_or(20) as usize;

        let mut results = json!({
            "query": query,
            "target": target,
            "results": []
        });

        match target {
            "local" => {
                // Search in local files
                let local_results = Self::search_text_real(query, path)?;
                results["results"] = local_results["results"].clone();
            }
            "web" => {
                // Web search
                let web_config = WebSearchConfig {
                    query: query.to_string(),
                    priority_sources: vec![],
                    sources: vec!["github.com".to_string(), "stackoverflow.com".to_string()],
                    max_results: max_results / 2,
                    use_ai: true,
                    use_stealth: true,
                    max_parallel: 20,
                    timeout_secs: 7, // 🔥 7 segundos
                    max_urls: 100,   // 🔥 100 URLs
                };

                let web_results = self.web_search.search(web_config).await?;
                let web_json = web_results
                    .into_iter()
                    .map(|r| {
                        json!({
                            "type": "web",
                            "url": r.url,
                            "title": r.title,
                            "description": r.description
                        })
                    })
                    .collect::<Vec<_>>();

                results["results"] = Value::Array(web_json);
            }
            "all" => {
                // Search both local and web
                let local_results = Self::search_text_real(query, path)?;
                results["local_results"] = local_results["results"].clone();

                let web_config = WebSearchConfig {
                    query: query.to_string(),
                    priority_sources: vec![],
                    sources: vec!["github.com".to_string(), "stackoverflow.com".to_string()],
                    max_results: max_results / 2,
                    use_ai: true,
                    use_stealth: true,
                    max_parallel: 20,
                    timeout_secs: 60, // 🔥 60 segundos
                    max_urls: 100,    // 🔥 100 URLs
                };

                let web_results = self.web_search.search(web_config).await?;
                let web_json = web_results
                    .into_iter()
                    .map(|r| {
                        json!({
                            "type": "web",
                            "url": r.url,
                            "title": r.title,
                            "description": r.description
                        })
                    })
                    .collect::<Vec<_>>();

                results["web_results"] = Value::Array(web_json);
            }
            _ => {}
        }

        Ok(results)
    }

    /// Handle stats tool
    pub async fn handle_stats(&mut self, args: Value) -> Result<Value> {
        let stats_type = args["type"].as_str().unwrap_or("full");

        match stats_type {
            "full" => {
                let stats = self.stats_system.get_full_stats();
                Ok(json!({
                    "uptime": stats.total_uptime.as_secs(),
                    "web_search": {
                        "total_searches": stats.web_search_stats.total_searches,
                        "successful": stats.web_search_stats.successful_searches,
                        "failed": stats.web_search_stats.failed_searches,
                        "avg_results": stats.web_search_stats.avg_results_per_search
                    },
                    "scraping": {
                        "urls_crawled": stats.scraping_stats.total_urls_crawled,
                        "successful": stats.scraping_stats.successful,
                        "failed": stats.scraping_stats.failed,
                        "data_mb": stats.scraping_stats.total_data_captured_mb
                    },
                    "ai": {
                        "patterns_learned": stats.ai_stats.patterns_learned,
                        "domains_analyzed": stats.ai_stats.domains_analyzed,
                        "accuracy": stats.ai_stats.accuracy
                    },
                    "recent_searches": stats.recent_searches.into_iter().take(5).map(|s| json!({
                        "query": s.query,
                        "timestamp": s.timestamp,
                        "results_count": s.results_count,
                        "success": s.success
                    })).collect::<Vec<_>>()
                }))
            }
            "recent" => {
                let stats = self.stats_system.get_full_stats();
                Ok(json!({
                    "recent_searches": stats.recent_searches.into_iter().take(10).map(|s| json!({
                        "query": s.query,
                        "timestamp": s.timestamp,
                        "results_count": s.results_count,
                        "success": s.success
                    })).collect::<Vec<_>>()
                }))
            }
            "performance" => {
                let stats = self.stats_system.get_full_stats();
                Ok(json!({
                    "uptime": stats.total_uptime.as_secs(),
                    "web_search": {
                        "total_searches": stats.web_search_stats.total_searches,
                        "successful": stats.web_search_stats.successful_searches,
                        "failed": stats.web_search_stats.failed_searches,
                        "avg_results": stats.web_search_stats.avg_results_per_search
                    },
                    "scraping": {
                        "urls_crawled": stats.scraping_stats.total_urls_crawled,
                        "successful": stats.scraping_stats.successful,
                        "failed": stats.scraping_stats.failed,
                        "data_mb": stats.scraping_stats.total_data_captured_mb
                    }
                }))
            }
            "storage" => Ok(json!(self.storage.get_stats()?)),
            _ => Err(anyhow::anyhow!("Unknown stats type: {}", stats_type)),
        }
    }

    /// Handle nuclear crawl tool
    #[allow(dead_code)]
    async fn handle_nuclear_crawl(&mut self, args: Value) -> Result<Value> {
        let urls: Vec<String> = args["urls"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Missing urls parameter"))?
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        if urls.is_empty() {
            return Err(anyhow::anyhow!("No URLs provided"));
        }

        let results = self.nuclear_scraper.nuclear_crawl(urls).await?;

        Ok(json!({
            "crawled_count": results.len(),
            "results": results.into_iter().map(|r| json!({
                "url": r.url,
                "status": r.status_code,
                "html": r.html,
                "content_length": r.content_length,
                "links_found": r.links_found,
                "images_found": r.images_found,
                "extracted_data": r.extracted_data,
                "error": r.error
            })).collect::<Vec<_>>()
        }))
    }

    /// Real text search in files
    #[allow(dead_code)]
    fn search_text_real(term: &str, dir: &str) -> Result<Value> {
        let mut results = Vec::new();
        let re = Regex::new(&regex::escape(term))?;

        Self::search_in_directory(Path::new(dir), &re, &mut results, 0)?;

        Ok(json!({
            "term": term,
            "directory": dir,
            "status": "success",
            "results_count": results.len(),
            "results": results.into_iter().take(20).collect::<Vec<_>>() // Limit to 20 results
        }))
    }

    #[allow(dead_code)]
    fn search_in_directory(
        dir: &Path,
        re: &Regex,
        results: &mut Vec<Value>,
        depth: usize,
    ) -> Result<()> {
        if depth > 5 {
            return Ok(());
        } // Limit depth

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                Self::search_in_directory(&path, re, results, depth + 1)?;
            } else if let Some(ext) = path.extension() {
                if matches!(
                    ext.to_str(),
                    Some("rs") | Some("txt") | Some("md") | Some("json")
                ) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        for (line_num, line) in content.lines().enumerate() {
                            if re.is_match(line) {
                                results.push(json!({
                                    "file": path.to_string_lossy(),
                                    "line": line_num + 1,
                                    "content": line.trim()
                                }));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Handle ultimas_busquedas tool
    pub async fn handle_ultimas_busquedas(&mut self, args: Value) -> Result<Value> {
        let limit = args["limit"].as_u64().unwrap_or(10) as usize;
        let recent_searches = self.storage.get_recent_searches(limit)?;

        Ok(json!({
            "limit": limit,
            "results_count": recent_searches.len(),
            "searches": recent_searches.into_iter().map(|s| json!({
                "id": s.id,
                "query": s.query,
                "timestamp": s.timestamp.to_rfc3339(),
                "results_count": s.results_count,
                "metadata": s.metadata,
                "results_preview": s.results.into_iter().take(3).collect::<Vec<_>>()
            })).collect::<Vec<_>>()
        }))
    }

    /// Handle analizar_proyecto tool (INTEGRADO CON SCAN_PROJECT)
    pub async fn handle_analizar_proyecto(&mut self, args: Value) -> Result<Value> {
        use std::path::PathBuf;
        let path_str = args["path"]
            .as_str()
            .or_else(|| args["project_path"].as_str())
            .ok_or_else(|| {
                anyhow::anyhow!("Missing project path parameter (use 'path' or 'project_path')")
            })?;
        let query_extra = args["query_extra"].as_str().unwrap_or("");
        let max_recommendations = args["max_recommendations"].as_u64().unwrap_or(5) as usize;

        let project_path = PathBuf::from(path_str);

        if !project_path.exists() {
            return Err(anyhow::anyhow!("Project path does not exist: {}", path_str));
        }

        // 🔥 PASO 1: ESCANEAR EL PROYECTO COMPLETO USANDO SCAN_PROJECT
        let scan_result = self
            .project_scanner
            .scan_project(project_path.clone())
            .await?;

        // 🔥 PASO 2: EXTRAER METADATOS DEL SCAN
        let language_str = format!("{:?}", scan_result.language);

        // Extraer dependencias desde los issues (si se detectan)
        let mut dependencies = Vec::new();
        for issue in &scan_result.errors {
            if issue.issue_type.contains("dependency") || issue.issue_type.contains("import") {
                dependencies.push(issue.code.clone());
            }
        }

        // Contar archivos
        let mut files_count = 0;
        let mut lines_of_code = 0;

        let mut dirs_to_check = vec![project_path.clone()];
        let mut depth = 0;
        const MAX_DEPTH: usize = 3;

        while let Some(dir) = dirs_to_check.pop() {
            if depth > MAX_DEPTH {
                break;
            }
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        files_count += 1;
                        // Contar líneas de código según lenguaje
                        match scan_result.language {
                            crate::scan_project::Language::Rust
                                if path.extension().and_then(|e| e.to_str()) == Some("rs") =>
                            {
                                if let Ok(content) = fs::read_to_string(&path) {
                                    lines_of_code += content.lines().count();
                                }
                            }
                            crate::scan_project::Language::Python
                                if path.extension().and_then(|e| e.to_str()) == Some("py") =>
                            {
                                if let Ok(content) = fs::read_to_string(&path) {
                                    lines_of_code += content.lines().count();
                                }
                            }
                            _ => {}
                        }
                    } else if path.is_dir() && depth < MAX_DEPTH {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if !name.starts_with('.')
                                && name != "target"
                                && name != "node_modules"
                                && name != "__pycache__"
                                && name != "build"
                                && name != "dist"
                            {
                                dirs_to_check.push(path);
                            }
                        }
                    }
                }
            }
            depth += 1;
        }

        let has_readme =
            project_path.join("README.md").exists() || project_path.join("readme.md").exists();
        let _has_git = project_path.join(".git").exists();

        let project_summary = json!({
            "path": path_str,
            "language": language_str,
            "quality_score": scan_result.quality_score,
            "total_issues": scan_result.total_issues,
            "errors_found": scan_result.errors.len(),
            "warnings_found": scan_result.warnings.len(),
            "dependencies": dependencies,
            "files_count": files_count,
            "lines_of_code": lines_of_code,
            "has_readme": has_readme,
            "has_git": _has_git,
        });

        // 🔥 PASO 3: GENERAR BÚSQUEDA MEJORADA basada en ERRORES ENCONTRADOS
        let mut search_query = format!(
            "{} {} best libraries improvements {}",
            language_str,
            if scan_result.errors.is_empty() {
                "advanced"
            } else {
                "fix"
            },
            query_extra
        );

        // Agregar contexto de errores a la búsqueda
        let error_keywords: Vec<String> = scan_result
            .errors
            .iter()
            .take(3)
            .filter_map(|e| e.code.clone())
            .collect();

        if !error_keywords.is_empty() {
            search_query = format!("{} ({})", search_query, error_keywords.join(" OR "));
        }

        // 🔥 PASO 4: BÚSQUEDA WEB MEJORADA CON CONTEXTO
        let web_search_config = crate::web_search::WebSearchConfig {
            query: search_query.clone(),
            priority_sources: vec![
                "github.com".to_string(),
                "crates.io".to_string(),
                "docs.rs".to_string(),
                "stackoverflow.com".to_string(),
            ],
            sources: vec![
                "github.com".to_string(),
                "crates.io".to_string(),
                "docs.rs".to_string(),
                "stackoverflow.com".to_string(),
                "reddit.com".to_string(),
                "dev.to".to_string(),
            ],
            max_results: max_recommendations * 3,
            use_ai: true,
            use_stealth: true,
            max_parallel: 100,
            timeout_secs: 60,
            max_urls: 100,
        };

        let web_results = match self.web_search.search(web_search_config).await {
            Ok(results) => results,
            Err(e) => {
                eprintln!("⚠️ Error en búsqueda web: {}", e);
                Vec::new()
            }
        };

        // 🔥 PASO 5: RANKEAR RECOMENDACIONES CON PESOS MEJORADOS
        let mut recommendations: Vec<Value> = web_results
            .into_iter()
            .filter(|r| r.quality_score > 0.5 && r.relevance > 0.4)
            .map(|r| {
                json!({
                    "name": r.title,
                    "url": r.url,
                    "description": r.description,
                    "relevance_score": r.relevance,
                    "quality_score": r.quality_score,
                    "source": r.source
                })
            })
            .collect();

        recommendations.sort_by(|a, b| {
            let score_a = a["relevance_score"].as_f64().unwrap_or_default() * 0.6
                + a["quality_score"].as_f64().unwrap_or_default() * 0.4;
            let score_b = b["relevance_score"].as_f64().unwrap_or_default() * 0.6
                + b["quality_score"].as_f64().unwrap_or_default() * 0.4;
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        recommendations.truncate(max_recommendations);

        Ok(json!({
            "project_path": path_str,
            "project_summary": project_summary,
            "scan_insights": {
                "language": language_str,
                "quality_score": scan_result.quality_score,
                "critical_issues": scan_result.errors.iter().map(|e| json!({
                    "code": e.code,
                    "message": e.message,
                    "file": e.file,
                    "line": e.line
                })).collect::<Vec<_>>(),
                "recommendations_from_scan": scan_result.recommendations.iter().take(3).collect::<Vec<_>>()
            },
            "search_query_used": search_query,
            "recommendations_count": recommendations.len(),
            "recommendations": recommendations,
        }))
    }

    /// Handle urls_visitadas tool
    pub async fn handle_urls_visitadas(&mut self, args: Value) -> Result<Value> {
        let limit = args["limit"].as_u64().unwrap_or(100) as usize;

        // Ejecutar en thread separado para no bloquear
        let storage = Arc::clone(&self.storage);
        let urls = tokio::task::spawn_blocking(move || storage.get_url_history(Some(limit)))
            .await
            .unwrap_or_else(|_| Ok(vec![]))?;

        // Obtener path del archivo de historial
        let history_file = std::path::PathBuf::from("resultados").join("urls_visited.txt");

        Ok(json!({
            "limit": limit,
            "total_urls": urls.len(),
            "urls": urls,
            "history_file": history_file.to_string_lossy().to_string(),
        }))
    }

    /// Handle scan_project tool (INTEGRADO CON WEB SEARCH PARA SOLUCIONES)
    pub async fn handle_scan_project(&mut self, args: Value) -> Result<Value> {
        use std::path::PathBuf;
        let path_str = args["project_path"].as_str().unwrap_or(".");
        let search_solutions = args["search_solutions"].as_bool().unwrap_or(true);

        let project_path = PathBuf::from(path_str);
        if !project_path.exists() {
            return Err(anyhow::anyhow!("Project path does not exist: {}", path_str));
        }

        // 🔥 PASO 1: ESCANEAR PROYECTO COMPLETO
        let scan_result = self
            .project_scanner
            .scan_rust_project(project_path.clone())
            .await?;

        // 🔥 PASO 2: BUSCAR SOLUCIONES WEB PARA ERRORES CRÍTICOS
        let mut enhanced_errors = Vec::new();
        let mut error_solutions = Vec::new();

        for error in scan_result.errors {
            let mut error_json = json!({
                "type": error.issue_type,
                "code": error.code.clone(),
                "file": error.file.clone(),
                "line": error.line,
                "column": error.column,
                "message": error.message.clone(),
                "source_code": error.source_code,
                "solutions": error.solutions.into_iter().map(|s| json!({
                    "title": s.title,
                    "description": s.description,
                    "url": s.url,
                    "example_code": s.example_code,
                    "priority": s.priority
                })).collect::<Vec<_>>()
            });

            // 🔥 BUSCAR SOLUCIONES ADICIONALES EN WEB SI ESTÁ HABILITADO
            if search_solutions {
                let error_code_str = error.code.clone().unwrap_or_else(|| "unknown".to_string());
                let error_query = format!(
                    "Rust {} {} fix solution {}",
                    error.issue_type,
                    error_code_str,
                    if error.message.len() > 50 {
                        &error.message[..50]
                    } else {
                        &error.message
                    }
                );

                let web_search_config = crate::web_search::WebSearchConfig {
                    query: error_query.clone(),
                    priority_sources: vec![
                        "stackoverflow.com".to_string(),
                        "docs.rs".to_string(),
                        "github.com".to_string(),
                    ],
                    sources: vec![
                        "stackoverflow.com".to_string(),
                        "docs.rs".to_string(),
                        "github.com".to_string(),
                        "reddit.com".to_string(),
                        "dev.to".to_string(),
                    ],
                    max_results: 3,
                    use_ai: true,
                    use_stealth: true,
                    max_parallel: 50,
                    timeout_secs: 30,
                    max_urls: 30,
                };

                // Obtener soluciones (sin fallar si no hay resultados)
                if let Ok(solutions) = self.web_search.search(web_search_config).await {
                    let best_solutions: Vec<_> = solutions
                        .into_iter()
                        .filter(|s| s.quality_score > 0.5)
                        .take(2)
                        .collect();

                    if !best_solutions.is_empty() {
                        for solution in &best_solutions {
                            error_solutions.push(json!({
                                "error_code": error.code,
                                "error_type": error.issue_type,
                                "solution_title": solution.title,
                                "solution_url": solution.url,
                                "solution_quality": solution.quality_score,
                                "source": solution.source
                            }));
                        }

                        // Agregar URL de mejor solución al error
                        if let Some(best) = best_solutions.first() {
                            if let Some(obj) = error_json.as_object_mut() {
                                obj.insert("web_solution_url".to_string(), json!(best.url.clone()));
                                obj.insert(
                                    "web_solution_quality".to_string(),
                                    json!(best.quality_score),
                                );
                            }
                        }
                    }
                }
            }

            enhanced_errors.push(error_json);
        }

        // 🔥 PASO 3: PROCESAR WARNINGS CON MISMA ESTRUCTURA
        let enhanced_warnings: Vec<_> = scan_result
            .warnings
            .into_iter()
            .map(|w| {
                json!({
                    "type": w.issue_type,
                    "code": w.code,
                    "file": w.file,
                    "line": w.line,
                    "column": w.column,
                    "message": w.message,
                    "source_code": w.source_code,
                    "solutions": w.solutions.into_iter().map(|s| json!({
                        "title": s.title,
                        "description": s.description,
                        "url": s.url,
                        "example_code": s.example_code,
                        "priority": s.priority
                    })).collect::<Vec<_>>()
                })
            })
            .collect();

        // 🔥 PASO 4: CALCULAR SCORE MEJORADO
        let error_impact = if enhanced_errors.is_empty() {
            0.0
        } else {
            (enhanced_errors.len() as f64 / 20.0).min(0.3)
        };
        let adjusted_quality = (scan_result.quality_score - error_impact as f32).max(0.0);

        // Convertir a JSON con integración web
        Ok(json!({
            "project_path": path_str,
            "total_issues": scan_result.total_issues,
            "errors_count": enhanced_errors.len(),
            "warnings_count": enhanced_warnings.len(),
            "quality_score": adjusted_quality,
            "search_solutions_enabled": search_solutions,
            "solutions_found": error_solutions.len(),
            "errors": enhanced_errors,
            "warnings": enhanced_warnings,
            "error_solutions": error_solutions,
            "recommendations": scan_result.recommendations
        }))
    }

    /// Handle deep_web_search tool
    pub async fn handle_deep_web_search(&mut self, args: Value) -> Result<Value> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing query parameter"))?;

        let search_type_str = args["search_type"].as_str().unwrap_or("all");
        let search_type = match search_type_str {
            "code" => DeepWebSearchType::Code,
            "intelligence" => DeepWebSearchType::Intelligence,
            "premium" => DeepWebSearchType::Premium,
            _ => DeepWebSearchType::All,
        };

        let sources: Vec<DeepWebSource> = args["sources"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| {
                v.as_str().and_then(|s| match s {
                    "academic" => Some(DeepWebSource::Academic),
                    "technical_db" => Some(DeepWebSource::TechnicalDB),
                    "code_repos" => Some(DeepWebSource::CodeRepos),
                    "specialized_forums" => Some(DeepWebSource::SpecializedForums),
                    "digital_libraries" => Some(DeepWebSource::DigitalLibraries),
                    "archives" => Some(DeepWebSource::Archives),
                    "tor" => Some(DeepWebSource::Tor),
                    "premium_apis" => Some(DeepWebSource::PremiumAPIs),
                    _ => None,
                })
            })
            .collect();

        let max_results = args["max_results"].as_u64().unwrap_or(20) as usize;
        let find_access_methods = args["find_access_methods"].as_bool().unwrap_or(true);
        let use_advanced_techniques = args["use_advanced_techniques"].as_bool().unwrap_or(false);

        let config = DeepWebSearchConfig {
            query: query.to_string(),
            search_type,
            sources,
            max_results,
            find_access_methods,
            use_advanced_techniques,
        };

        let results = self.deep_web_search.search(config).await?;

        Ok(json!({
            "query": query,
            "results_count": results.len(),
            "results": results.into_iter().map(|r| json!({
                "url": r.url,
                "title": r.title,
                "description": r.description,
                "content_type": r.content_type,
                "is_premium": r.is_premium,
                "relevance": r.relevance,
                "quality_score": r.quality_score,
                "source": r.source,
                "metadata": r.metadata,
                "access_methods": r.access_methods.into_iter().map(|m| json!({
                    "name": m.name,
                    "description": m.description,
                    "method_type": format!("{:?}", m.method_type),
                    "instructions": m.instructions,
                    "is_legal": m.is_legal,
                    "requires_auth": m.requires_auth,
                })).collect::<Vec<_>>(),
            })).collect::<Vec<_>>(),
        }))
    }
}
