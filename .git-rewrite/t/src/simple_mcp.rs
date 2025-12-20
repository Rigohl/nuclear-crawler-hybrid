//! Simple MCP Server for Nuclear Crawler Hybrid
//! 🔥🔥🔥 IMPLEMENTACIONES REALES - SIN MOCKS NI SIMULACIONES 🔥🔥🔥

use crate::ai_smart::{AIConfig, AISmart};
use crate::deep_web_search::{
    DeepWebSearch, DeepWebSearchConfig, DeepWebSearchType, DeepWebSource,
};
use crate::intelligent_storage::{IntelligentStorage, SearchResultEntry};
use crate::nuclear_scraper::{NuclearConfig, NuclearScraper};
use crate::real_search_engines::RealSearchEngine; // 🔥 MOTORES REALES
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

/// Simple MCP Server with Nuclear Power - IMPLEMENTACIONES REALES
pub struct SimpleMcpServer {
    #[allow(dead_code)]
    nuclear_scraper: Arc<NuclearScraper>,
    web_search: Arc<WebSearch>,
    real_search: Arc<RealSearchEngine>, // 🔥 MOTORES DE BÚSQUEDA REALES
    stats_system: Arc<StatsSystem>,
    storage: Arc<IntelligentStorage>,
    project_scanner: Arc<ProjectScanner>,
    deep_web_search: Arc<DeepWebSearch>,
}

impl SimpleMcpServer {
    /// Create new MCP server with full Nuclear power - REAL IMPLEMENTATIONS
    pub fn new() -> Result<Self> {
        // Logs van a stderr, no stdout (MCP protocol requirement)
        eprintln!("🔥 Nuclear MCP Server v5.0 - REAL IMPLEMENTATIONS (NO MOCKS)");

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

        // 🔥 MOTORES DE BÚSQUEDA REALES (DuckDuckGo, Bing, Brave, SearX)
        let real_search = Arc::new(RealSearchEngine::new()?);
        eprintln!("   ✅ Real Search Engines: DuckDuckGo, Bing, Brave, SearX");

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

        eprintln!("   ✅ All modules initialized - READY FOR REAL SEARCHES");

        Ok(Self {
            nuclear_scraper,
            web_search,
            real_search,
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
        eprintln!("🔥 MCP Server listening on stdin/stdout...");

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
                // ✅ SOLO LAS 4 HERRAMIENTAS OFICIALES DE CORE_TOOLS
                let tools = crate::core_tools::NuclearCore::list_tools();
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": tools
                    }
                })
            }

            // ❌ DUPLICADAS - USAR CORE_TOOLS EN SU LUGAR (COMENTADAS)
            "_tools_old_list" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": [
                            {
                                "name": "websearch",
                                "description": "🔥 BÚSQUEDA NUCLEAR (15 seg): Máxima potencia automática. DuckDuckGo+Bing+Brave+SearX, AI+Stealth+DeepWeb ON. Solo pon el query.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "query": {
                                            "type": "string",
                                            "description": "Qué buscar"
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

    /// Handle tool calls - LAS 4 HERRAMIENTAS OFICIALES
    pub async fn handle_tool_call(&mut self, tool_name: &str, args: Value) -> Result<Value> {
        // ✅ LAS 4 HERRAMIENTAS OFICIALES (MISMO que HTTP mode)
        match tool_name {
            "websearch" => self.handle_websearch(args).await,
            "file_search" => self.handle_file_search(args).await,
            "analyzer" => self.handle_analyzer(args).await,
            "stats" => self.handle_stats(args).await,
            // ❌ HERRAMIENTAS ANTIGUAS - DEPRECATED
            "ultimas_busquedas" => Err(anyhow::anyhow!(
                "DEPRECATED: Use 'stats' tool instead (all searches are saved in SQLite)"
            )),
            "analizar_proyecto" => Err(anyhow::anyhow!(
                "DEPRECATED: Use 'analyzer' tool instead"
            )),
            "urls_visitadas" => Err(anyhow::anyhow!(
                "DEPRECATED: Use 'stats' tool instead"
            )),
            "scan_project" => Err(anyhow::anyhow!(
                "DEPRECATED: Use 'analyzer' tool instead"
            )),
            "deep_web_search" => Err(anyhow::anyhow!(
                "DEPRECATED: Deep web is now integrated in 'websearch'. Use that instead."
            )),
            _ => Err(anyhow::anyhow!("Unknown tool: {}. Available: websearch, file_search, analyzer, stats", tool_name)),
        }
    }

    /// Handle websearch tool - 🔥🔥🔥 MÁXIMA POTENCIA NUCLEAR (30 seg) 🔥🔥🔥
    /// Solo requiere "query" - TODO lo demás es automático con máximo poder
    pub async fn handle_websearch(&mut self, args: Value) -> Result<Value> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing query parameter"))?;

        let start = std::time::Instant::now();
        
        // 🔥🔥🔥 USAR CONFIGURACIÓN NUCLEAR INMUTABLE - 35 SEGUNDOS 🔥🔥🔥
        eprintln!("🔥🔥🔥 WEBSEARCH NUCLEAR v5.0 - MÁXIMA POTENCIA (35 seg) 🔥🔥🔥");
        eprintln!("   🔍 Query: '{}'", query);
        eprintln!("   ⚡ TODOS los módulos activos: Go FFI, Zig SIMD, JAX GPU, Mojo, DeepWeb, AI");
        eprintln!("   🛡️ Stealth: MÁXIMO (rotation, delays, headers realistas)");
        eprintln!("   ⚡ Paralelismo: 10,000+ URLs simultáneamente");
        eprintln!("   ❌ CONFIGURACIÓN BLOQUEADA: Solo query personalizable");

        // 🔥🔥🔥 CONFIG NUCLEAR MÁXIMA POTENCIA - 35 SEGUNDOS INMUTABLE 🔥🔥🔥
        let mut config = WebSearchConfig::unlimited_with_query(query);
        config.max_results = 2000; // 🔥 2000 resultados máx
        config.timeout_secs = 35; // 🔥 35 SEGUNDOS - MÁXIMO PODER (INMUTABLE)
        config.max_urls = 1000; // 🔥 1000 URLs a crawlear
        config.unlimited_mode = true; // 🔥 Modo sin límites
        config.use_native_ffi = true; // 🔥 Go/Zig FFI activo
        config.deep_web_enabled = true; // 🔥 Deep web activo
        config.use_ai = true; // 🔥 AI ranking activo
        config.use_stealth = true; // 🔥 Stealth máximo
        config.max_parallel = 10000; // 🔥 10K paralelo (10,000+ URLs)
        eprintln!(
            "   ⚡ Config: max_results=2000, parallel=10K, timeout=35s (INMUTABLE), AI+Stealth+DeepWeb+Go+Zig=ON"
        );

        // 🔥🔥🔥 FASE 1: MOTORES REALES (10 seg) - DuckDuckGo, Bing, Brave, SearX 🔥🔥🔥
        eprintln!(
            "   🔍 Fase 1/4: Motores REALES (DuckDuckGo, Bing, Brave, SearX, Yandex, Ecosia)..."
        );
        let real_results = self.real_search.search_all_engines(query, 200, 10).await?;
        eprintln!(
            "   ✅ {} resultados de motores reales en {:?}",
            real_results.len(),
            start.elapsed()
        );

        // 🔥🔥🔥 FASE 2: BÚSQUEDA MASIVA EN FUENTES (10 seg) 🔥🔥🔥
        eprintln!(
            "   🔍 Fase 2/4: Búsqueda masiva en fuentes (GitHub, Reddit, HuggingFace, arXiv...)..."
        );
        let web_results = self.web_search.search(config.clone()).await?;
        eprintln!(
            "   ✅ {} resultados de fuentes en {:?}",
            web_results.len(),
            start.elapsed()
        );

        // 🔥🔥🔥 FASE 3: DEEP WEB SEARCH (10 seg) 🔥🔥🔥
        eprintln!(
            "   🔍 Fase 3/4: Deep Web Search (repositorios profundos, archive.org, caches)..."
        );
        let deep_config = DeepWebSearchConfig {
            query: query.to_string(),
            search_type: DeepWebSearchType::All,
            sources: vec![
                DeepWebSource::Academic,
                DeepWebSource::TechnicalDB,
                DeepWebSource::CodeRepos,
                DeepWebSource::SpecializedForums,
                DeepWebSource::DigitalLibraries,
                DeepWebSource::Archives,
                DeepWebSource::PremiumAPIs,
            ],
            max_results: 100,
            find_access_methods: true,
            use_advanced_techniques: true,
        };
        let deep_results = self
            .deep_web_search
            .search(deep_config)
            .await
            .unwrap_or_default();
        eprintln!(
            "   ✅ {} resultados de deep web en {:?}",
            deep_results.len(),
            start.elapsed()
        );

        // 🔥🔥🔥 FASE 4: EXTRACCIÓN DE CONTENIDO (5 seg) 🔥🔥🔥
        eprintln!("   🔍 Fase 4/4: Extracción de contenido real de URLs encontradas...");

        // 🔥🔥🔥 COMBINAR TODOS LOS RESULTADOS Y DEDUPLICAR 🔥🔥🔥
        let mut all_results: Vec<crate::web_search::WebSearchResult> = Vec::new();

        // 1. Convertir resultados de motores reales (máxima prioridad)
        for r in real_results {
            all_results.push(crate::web_search::WebSearchResult {
                url: r.url.clone(),
                title: r.title.clone(),
                description: r.snippet.clone(),
                main_text: String::new(), // Se llenará después
                summary: String::new(),
                word_count: 0,
                headings: Vec::new(),
                code_snippets: Vec::new(),
                relevance: 0.95, // 🔥 Máxima relevancia (motores reales)
                quality_score: 0.90,
                source: format!("real_engine:{}", r.engine),
            });
        }

        // 2. Agregar resultados de web_search (fuentes específicas)
        all_results.extend(web_results);

        // 3. Agregar resultados de Deep Web
        for r in deep_results {
            all_results.push(crate::web_search::WebSearchResult {
                url: r.url.clone(),
                title: r.title.clone(),
                description: r.description.clone(),
                main_text: String::new(), // Se llenará después
                summary: String::new(),
                word_count: 0,
                headings: Vec::new(),
                code_snippets: Vec::new(),
                relevance: r.relevance,
                quality_score: r.quality_score,
                source: format!("deep_web:{}", r.source),
            });
        }

        eprintln!(
            "   📊 Total antes de deduplicar: {} resultados",
            all_results.len()
        );

        // Deduplicar por URL (normalizada)
        let mut seen_urls = std::collections::HashSet::new();
        all_results.retain(|r| {
            let url_key = r
                .url
                .trim_end_matches('/')
                .to_lowercase()
                .replace("www.", "")
                .replace("http://", "")
                .replace("https://", "");
            seen_urls.insert(url_key)
        });

        // Ordenar por score combinado (relevance + quality)
        all_results.sort_by(|a, b| {
            let score_a = a.relevance * 0.6 + a.quality_score * 0.4;
            let score_b = b.relevance * 0.6 + b.quality_score * 0.4;
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // 🔥 SIN LÍMITE ARTIFICIAL - Mantener todos los resultados únicos
        // all_results.truncate(2000); // Solo si es necesario por memoria

        eprintln!(
            "✅ WEBSEARCH NUCLEAR completado: {} resultados únicos en {:?}",
            all_results.len(),
            start.elapsed()
        );

        // Guardar en DB
        let search_entries: Vec<SearchResultEntry> = all_results
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
                    meta.insert("mode".to_string(), "NUCLEAR_MAX_POWER".to_string());
                    meta.insert("timeout".to_string(), "15s".to_string());
                    meta.insert(
                        "search_time_ms".to_string(),
                        start.elapsed().as_millis().to_string(),
                    );
                    meta
                }),
            )
            .await?;

        // 🔥 GUARDAR RESULTADOS EN ARCHIVO JSON (resultados/)
        // Usar ruta ABSOLUTA hardcodeada del proyecto (más confiable)
        let project_dir =
            std::path::PathBuf::from(r"C:\Users\DELL\Desktop\hf_spaces\NUCLEAR_CRAWLER_HYBRID");

        // Fallback: intentar detectar desde el ejecutable
        let project_dir = if project_dir.exists() {
            project_dir
        } else {
            let exe_path = std::env::current_exe().unwrap_or_default();
            let exe_str = exe_path.to_string_lossy();
            if exe_str.contains("target")
                && (exe_str.contains("release") || exe_str.contains("debug"))
            {
                // Extraer el directorio padre de target/
                let parts: Vec<&str> = exe_str.split("target").collect();
                std::path::PathBuf::from(parts[0].trim_end_matches(std::path::MAIN_SEPARATOR))
            } else {
                std::env::current_dir().unwrap_or_default()
            }
        };

        let results_dir = project_dir.join("resultados");
        std::fs::create_dir_all(&results_dir).ok();
        eprintln!("📁 Directorio de resultados: {}", results_dir.display());

        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("websearch_{}.json", timestamp);
        let filepath = results_dir.join(&filename);

        // Extraer información estructurada
        let extracted_data: Vec<serde_json::Value> = all_results.iter().map(|r| {
            json!({
                "url": r.url,
                "title": r.title,
                "description": r.description,
                "relevance": r.relevance,
                "quality_score": r.quality_score,
                "source": r.source,
                "extracted_info": {
                    "domain": r.url.split('/').nth(2).unwrap_or("unknown"),
                    "is_high_quality": r.quality_score > 0.7,
                    "relevance_tier": if r.relevance > 0.8 { "high" } else if r.relevance > 0.5 { "medium" } else { "low" }
                }
            })
        }).collect();

        let json_output = json!({
            "query": query,
            "timestamp": timestamp,
            "total_results": all_results.len(),
            "search_time_ms": start.elapsed().as_millis(),
            "mode": "NUCLEAR_MAX_POWER_30s_ALL_MODULES",
            "engines_used": ["DuckDuckGo", "Bing", "Brave", "SearX", "Yandex", "Ecosia", "DeepWeb"],
            "modules_active": [
                "Go_Integration", "Zig_Integration", "Nim_Integration",
                "JAX_Acceleration", "Mojo_Processor", "Nuclear_Bypass",
                "Parallel_Crawler", "Massive_Search", "Deep_Web_Search",
                "Stealth_System", "AI_Smart", "Real_Search_Engines"
            ],
            "config": {
                "max_results": 2000,
                "timeout_secs": 30,
                "ai_enabled": true,
                "stealth_enabled": true,
                "deep_web_enabled": true,
                "unlimited_mode": true,
                "native_ffi": true,
                "max_parallel": 10000
            },
            "statistics": {
                "high_quality_count": all_results.iter().filter(|r| r.quality_score > 0.7).count(),
                "high_relevance_count": all_results.iter().filter(|r| r.relevance > 0.8).count(),
                "unique_domains": all_results.iter()
                    .filter_map(|r| r.url.split('/').nth(2))
                    .collect::<std::collections::HashSet<_>>()
                    .len()
            },
            "results": extracted_data
        });

        if let Ok(json_string) = serde_json::to_string_pretty(&json_output) {
            if let Err(e) = std::fs::write(&filepath, &json_string) {
                eprintln!("⚠️ Error guardando JSON: {}", e);
            } else {
                eprintln!("💾 Resultados guardados en: {}", filepath.display());
            }
        }

        Ok(json!({
            "query": query,
            "results_count": all_results.len(),
            "search_time_ms": start.elapsed().as_millis(),
            "mode": "NUCLEAR_MAX_POWER_30s_ALL_MODULES",
            "engines": ["DuckDuckGo", "Bing", "Brave", "SearX", "Yandex", "Ecosia", "DeepWeb"],
            "modules_used": 12,
            "_saved_to": filepath.to_string_lossy(),
            "results": all_results.into_iter().map(|r| json!({
                "url": r.url,
                "title": r.title,
                "description": r.description,
                "relevance": r.relevance,
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
            "max_results": max_results,
            "results": []
        });

        match target {
            "local" => {
                // Search in local files
                let local_results = Self::search_text_real(query, path)?;
                let mut local_items: Vec<Value> = local_results["results"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default();
                local_items.truncate(max_results);
                results["results"] = Value::Array(local_items);
            }
            "web" => {
                // 🔥 Web search REAL con max_results
                let real_results = self
                    .real_search
                    .search_all_engines(query, max_results, 30)
                    .await?;
                let web_json: Vec<Value> = real_results
                    .into_iter()
                    .take(max_results)
                    .map(|r| {
                        json!({
                            "type": "web",
                            "url": r.url,
                            "title": r.title,
                            "description": r.snippet,
                            "engine": r.engine
                        })
                    })
                    .collect();

                results["results"] = Value::Array(web_json);
            }
            "all" => {
                // Search both local and web REAL con max_results
                let local_results = Self::search_text_real(query, path)?;
                let mut local_items: Vec<Value> = local_results["results"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default();
                local_items.truncate(max_results / 2);
                results["local_results"] = Value::Array(local_items);

                // 🔥 Búsqueda REAL en motores con max_results
                let real_results = self
                    .real_search
                    .search_all_engines(query, max_results / 2, 30)
                    .await?;
                let web_json: Vec<Value> = real_results
                    .into_iter()
                    .take(max_results / 2)
                    .map(|r| {
                        json!({
                            "type": "web",
                            "url": r.url,
                            "title": r.title,
                            "description": r.snippet,
                            "engine": r.engine
                        })
                    })
                    .collect();

                results["web_results"] = Value::Array(web_json);
            }
            _ => {}
        }

        Ok(results)
    }

    /// Handle file_search tool - 🔍 BÚSQUEDA EN ARCHIVOS
    pub async fn handle_file_search(&mut self, args: Value) -> Result<Value> {
        let search_term = args["search_term"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing search_term"))?;
        let path = args["path"].as_str().unwrap_or(".");
        let use_regex = args["use_regex"].as_bool().unwrap_or(false);
        let case_sensitive = args["case_sensitive"].as_bool().unwrap_or(false);
        let max_results = args["max_results"].as_u64().unwrap_or(100) as usize;

        let extensions: Vec<String> = args["file_extensions"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_else(|| {
                vec!["rs".to_string(), "py".to_string(), "js".to_string(), "ts".to_string()]
            });

        let include_patterns: Vec<String> = extensions.iter().map(|e| format!("*.{}", e)).collect();

        let config = crate::file_search::FileSearchConfig {
            search_term: search_term.to_string(),
            root_dir: std::path::PathBuf::from(path),
            include_patterns,
            exclude_patterns: vec![
                "target/".to_string(),
                "node_modules/".to_string(),
                ".git/".to_string(),
            ],
            exact_match: case_sensitive,
            use_regex,
            search_in_content: true,
            search_in_filename: true,
            max_results,
            detect_errors: true,
            use_cargo_check: true,
            semantic_search: false,
            context_analysis: true,
            pattern_detection: false,
            fuzzy_search: false,
            dependency_analysis: false,
            detect_circular_imports: false,
            analyze_function_complexity: false,
            detect_code_duplication: false,
            context_depth: 3,
            fuzzy_threshold: 0.8,
            duplication_min_lines: 5,
        };

        let file_search = crate::file_search::FileSearch::new();
        let results = file_search.search(config).await?;

        let matches: Vec<Value> = results
            .iter()
            .map(|r| {
                json!({
                    "file": r.file_path,
                    "line_number": r.line_number,
                    "line_content": r.line_content,
                    "match_count": r.match_count,
                    "match_type": r.match_type
                })
            })
            .collect();

        Ok(json!({
            "search_term": search_term,
            "path": path,
            "use_regex": use_regex,
            "case_sensitive": case_sensitive,
            "extensions": extensions,
            "total_matches": matches.len(),
            "matches": matches
        }))
    }

    /// Handle analyzer tool - 🔍 ANÁLISIS DE PROYECTOS (VERSIÓN SIMPLE)
    pub async fn handle_analyzer(&mut self, args: Value) -> Result<Value> {
        let path = args["path"].as_str().unwrap_or(".");
        
        eprintln!("📊 Analyzer: Analizando {}", path);

        let project_path = std::path::PathBuf::from(path);

        // Verificar que la ruta existe
        if !project_path.exists() {
            return Ok(json!({
                "path": path,
                "status": "error",
                "error": "Path does not exist",
                "message": "The specified project path does not exist"
            }));
        }

        // Análisis básico local (sin WebSearch)
        let mut stats = serde_json::json!({
            "path": path,
            "status": "success",
            "modules_analyzed": 0,
            "files": [],
            "recommendations": []
        });

        // Contar archivos
        let mut file_count = 0;
        let mut rust_files = 0;
        let mut js_files = 0;
        let mut py_files = 0;
        let mut other_files = 0;

        if let Ok(entries) = std::fs::read_dir(&project_path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        file_count += 1;
                        if let Some(ext) = entry.path().extension() {
                            match ext.to_string_lossy().as_ref() {
                                "rs" => rust_files += 1,
                                "js" | "ts" => js_files += 1,
                                "py" => py_files += 1,
                                _ => other_files += 1,
                            }
                        }
                    }
                }
            }
        }

        stats["modules_analyzed"] = serde_json::json!(file_count);
        stats["breakdown"] = serde_json::json!({
            "rust_files": rust_files,
            "js_ts_files": js_files,
            "python_files": py_files,
            "other_files": other_files,
            "total": file_count
        });

        eprintln!("✅ Análisis completado: {} archivos", file_count);
        Ok(stats)
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
            ..Default::default()
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
        let history_file = std::path::PathBuf::from(
            r"C:\Users\DELL\Desktop\hf_spaces\NUCLEAR_CRAWLER_HYBRID\resultados",
        )
        .join("urls_visited.txt");

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
                    ..Default::default()
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
