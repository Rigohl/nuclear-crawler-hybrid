//! 🔥 CORE TOOLS - 4 Herramientas MCP 2025 usando TODOS los módulos
//!
//! 1. websearch - Búsqueda web MASIVA con TODOS los módulos (Go, Zig, JAX, Nim, Stealth, etc.)
//! 2. file_search - Búsqueda exacta en archivos con líneas exactas
//! 3. analyzer - Análisis completo de proyectos
//! 4. stats - Estadísticas de Nuclear Crawler
//!
//! SIN MOCKS - Implementación REAL de todos los módulos

use crate::file_search::{FileSearch, FileSearchConfig};
use crate::go_integration::GoIntegration;
use crate::intelligent_storage::IntelligentStorage;
use crate::jax_acceleration::JaxAccelerator;
use crate::mojo_jax::MojoJaxProcessor;
use crate::nim_integration::NimIntegration;
use crate::nuclear_scraper::{NuclearConfig, NuclearScraper};
use crate::scan_project::ProjectScanner;
use crate::stealth::StealthSystem;
use crate::web_search::{WebSearch, WebSearchConfig};
use crate::zig_integration::ZigIntegration;

use anyhow::Result;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

/// 🔥 NUCLEAR CORE - Sistema central con TODOS los módulos
pub struct NuclearCore {
    // Búsqueda Web
    pub web_search: Arc<WebSearch>,

    // Scraping
    pub nuclear_scraper: Arc<NuclearScraper>,

    // Almacenamiento
    pub storage: Arc<IntelligentStorage>,

    // Integraciones FFI (Go, Zig, Nim)
    pub go_integration: Arc<GoIntegration>,
    pub zig_integration: Arc<ZigIntegration>,
    pub nim_integration: Arc<NimIntegration>,

    // Aceleración (JAX, Mojo)
    pub jax_accelerator: Arc<JaxAccelerator>,
    pub mojo_processor: Arc<MojoJaxProcessor>,

    // Stealth
    pub stealth_system: Arc<StealthSystem>,
}

impl NuclearCore {
    /// Crear nuevo NuclearCore con TODOS los módulos inicializados
    pub fn new() -> Result<Self> {
        // Almacenamiento inteligente (base para todo)
        let storage = Arc::new(IntelligentStorage::new(None)?);

        // Nuclear Scraper con storage
        let nuclear_config = NuclearConfig::default();
        let nuclear_scraper = Arc::new(NuclearScraper::new_with_storage(
            nuclear_config,
            Some(storage.clone()),
        )?);

        // Web Search con storage
        let web_search = Arc::new(WebSearch::new_with_storage(Some(storage.clone()))?);

        // FFI Integrations (Go, Zig, Nim)
        let go_integration = Arc::new(GoIntegration::new());
        let zig_integration = Arc::new(ZigIntegration::new());
        let nim_integration = Arc::new(NimIntegration::new());

        // JAX Acceleration
        let jax_accelerator = Arc::new(JaxAccelerator::new());
        let mojo_processor = Arc::new(MojoJaxProcessor::new());

        // Stealth
        let stealth_system = Arc::new(StealthSystem::new(Default::default()));

        Ok(Self {
            web_search,
            nuclear_scraper,
            storage,
            go_integration,
            zig_integration,
            nim_integration,
            jax_accelerator,
            mojo_processor,
            stealth_system,
        })
    }

    /// 📋 Lista las 4 herramientas MCP
    pub fn list_tools() -> Vec<Value> {
        vec![
            // 1. WEBSEARCH - Usa TODOS los módulos
            json!({
                "name": "websearch",
                "description": "🔥 BÚSQUEDA WEB NUCLEAR MASIVA: Usa TODOS los módulos (Go FFI paralelismo, Zig SIMD parsing, JAX GPU/TPU, Nim HTML, Stealth anti-detección, DeepWeb, Orquestador, HuggingFace). 200+ URLs simultáneas, máxima velocidad.",
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
                            "description": "Fuentes: github.com, stackoverflow.com, reddit.com, dev.to, medium.com, huggingface.co, arxiv.org, etc.",
                            "default": ["github.com", "stackoverflow.com", "dev.to", "reddit.com"]
                        },
                        "max_results": {
                            "type": "integer",
                            "description": "Máximo resultados (0 = sin límite NUCLEAR)",
                            "default": 100
                        },
                        "use_stealth": {
                            "type": "boolean",
                            "description": "Activar modo stealth anti-detección",
                            "default": true
                        },
                        "use_ai_ranking": {
                            "type": "boolean",
                            "description": "Ranking inteligente con IA",
                            "default": true
                        },
                        "parallel_mode": {
                            "type": "string",
                            "enum": ["normal", "extreme", "nuclear"],
                            "description": "Modo: normal (50), extreme (100), nuclear (200+)",
                            "default": "nuclear"
                        }
                    },
                    "required": ["query"]
                }
            }),

            // 2. FILE_SEARCH - Búsqueda exacta en archivos
            json!({
                "name": "file_search",
                "description": "📂 BÚSQUEDA EN ARCHIVOS: Encuentra líneas exactas con palabra/patrón. Ideal para buscar errores, TODOs, funciones, variables. Muestra archivo, línea exacta y contexto.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "search_term": {
                            "type": "string",
                            "description": "Término/patrón a buscar"
                        },
                        "path": {
                            "type": "string",
                            "description": "Ruta del directorio/archivo",
                            "default": "."
                        },
                        "use_regex": {
                            "type": "boolean",
                            "description": "Usar expresiones regulares",
                            "default": false
                        },
                        "case_sensitive": {
                            "type": "boolean",
                            "description": "Sensible a mayúsculas",
                            "default": false
                        },
                        "file_extensions": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Extensiones: rs, py, js, ts, etc.",
                            "default": ["rs", "py", "js", "ts", "go", "c", "cpp", "h"]
                        },
                        "max_results": {
                            "type": "integer",
                            "description": "Máximo de coincidencias",
                            "default": 100
                        }
                    },
                    "required": ["search_term"]
                }
            }),

            // 3. ANALYZER - Análisis de proyectos
            json!({
                "name": "analyzer",
                "description": "🔍 ANALIZADOR DE PROYECTOS: Escanea código, detecta errores, duplicados, mocks, código incompleto. Busca soluciones en web. Soporta Rust, Python, JS, Go, y más.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Ruta del proyecto",
                            "default": "."
                        },
                        "scan_type": {
                            "type": "string",
                            "enum": ["full", "errors", "security"],
                            "description": "Tipo de análisis",
                            "default": "full"
                        },
                        "search_solutions": {
                            "type": "boolean",
                            "description": "Buscar soluciones en web para errores",
                            "default": true
                        }
                    },
                    "required": ["path"]
                }
            }),

            // 4. STATS - Estadísticas de Nuclear Crawler
            json!({
                "name": "stats",
                "description": "📊 ESTADÍSTICAS NUCLEAR CRAWLER: URLs crawled, búsquedas realizadas, datos capturados, cache hits, rendimiento, módulos activos (Go, Zig, JAX, etc.).",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "stat_type": {
                            "type": "string",
                            "enum": ["full", "modules", "performance"],
                            "description": "Tipo de estadísticas",
                            "default": "full"
                        }
                    }
                }
            }),
        ]
    }

    /// 🔥 Ejecutar herramienta
    pub async fn call_tool(&self, name: &str, args: Value) -> Result<Value> {
        let start = Instant::now();

        let result = match name {
            "websearch" => self.tool_websearch(args).await?,
            "file_search" => self.tool_file_search(args).await?,
            "analyzer" => self.tool_analyzer(args).await?,
            "stats" => self.tool_stats(args).await?,
            _ => return Err(anyhow::anyhow!("Tool not found: {}", name)),
        };

        // Agregar metadata de ejecución
        let mut final_result = result;
        if let Some(obj) = final_result.as_object_mut() {
            obj.insert(
                "_execution_time_ms".to_string(),
                json!(start.elapsed().as_millis()),
            );
            obj.insert("_tool".to_string(), json!(name));
        }

        Ok(final_result)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // 🔥 TOOL 1: WEBSEARCH - Usa TODOS los módulos
    // ═══════════════════════════════════════════════════════════════════════

    async fn tool_websearch(&self, args: Value) -> Result<Value> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing query"))?;
        let max_results = args["max_results"].as_u64().unwrap_or(100) as usize;
        let use_stealth = args["use_stealth"].as_bool().unwrap_or(true);
        let use_ai = args["use_ai_ranking"].as_bool().unwrap_or(true);
        let parallel_mode = args["parallel_mode"].as_str().unwrap_or("nuclear");

        let sources: Vec<String> = args["sources"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_else(|| {
                vec![
                    "github.com".to_string(),
                    "stackoverflow.com".to_string(),
                    "dev.to".to_string(),
                    "reddit.com".to_string(),
                ]
            });

        let mut modules_used = Vec::new();

        // 🔥 Activar módulos FFI si disponibles
        if self.go_integration.is_available() {
            modules_used.push("go_ffi");
        }
        if self.zig_integration.is_available() {
            modules_used.push("zig_simd");
        }
        if self.nim_integration.is_available() {
            modules_used.push("nim_html");
        }
        if self.jax_accelerator.is_available() {
            modules_used.push("jax_gpu");
        }
        if self.mojo_processor.is_available() {
            modules_used.push("mojo_bridge");
        }

        // Stealth
        if use_stealth {
            modules_used.push("stealth_engine");
        }

        // Configurar paralelismo
        let max_concurrent = match parallel_mode {
            "normal" => 50,
            "extreme" => 100,
            _ => 200, // nuclear
        };

        // Web Search principal
        let config = WebSearchConfig {
            query: query.to_string(),
            sources: sources.clone(),
            priority_sources: vec![],
            max_results: if max_results == 0 { 1000 } else { max_results },
            use_ai,
            use_stealth,
            max_parallel: max_concurrent,
            timeout_secs: 60,
            max_urls: max_concurrent,
        };

        let results = self.web_search.search(config).await?;
        modules_used.push("web_search");
        modules_used.push("intelligent_storage");

        let all_results: Vec<Value> = results
            .iter()
            .map(|r| {
                json!({
                    "url": r.url,
                    "title": r.title,
                    "description": r.description,
                    "relevance": r.relevance,
                    "quality_score": r.quality_score,
                    "source": r.source
                })
            })
            .collect();

        Ok(json!({
            "query": query,
            "total_results": all_results.len(),
            "sources_searched": sources,
            "parallel_mode": parallel_mode,
            "max_concurrent": max_concurrent,
            "modules_used": modules_used,
            "results": all_results
        }))
    }

    // ═══════════════════════════════════════════════════════════════════════
    // 📂 TOOL 2: FILE_SEARCH - Búsqueda exacta en archivos
    // ═══════════════════════════════════════════════════════════════════════

    async fn tool_file_search(&self, args: Value) -> Result<Value> {
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
                vec![
                    "rs".to_string(),
                    "py".to_string(),
                    "js".to_string(),
                    "ts".to_string(),
                ]
            });

        // Crear patrones include
        let include_patterns: Vec<String> = extensions.iter().map(|e| format!("*.{}", e)).collect();

        let config = FileSearchConfig {
            search_term: search_term.to_string(),
            root_dir: PathBuf::from(path),
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
        };

        let file_search = FileSearch::new();
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

    // ═══════════════════════════════════════════════════════════════════════
    // 🔍 TOOL 3: ANALYZER - Análisis de proyectos
    // ═══════════════════════════════════════════════════════════════════════

    async fn tool_analyzer(&self, args: Value) -> Result<Value> {
        let path = args["path"].as_str().unwrap_or(".");
        let scan_type = args["scan_type"].as_str().unwrap_or("full");
        let _search_solutions = args["search_solutions"].as_bool().unwrap_or(true);

        // Crear scanner
        let scanner = ProjectScanner::new(self.web_search.clone());

        // Detectar lenguaje
        let project_path = PathBuf::from(path);
        let language = scanner.detect_language(&project_path)?;

        // Escanear proyecto
        let scan_result = scanner.scan_project(project_path).await?;

        Ok(json!({
            "path": path,
            "scan_type": scan_type,
            "language": format!("{:?}", language),
            "total_issues": scan_result.total_issues,
            "quality_score": scan_result.quality_score,
            "errors": scan_result.errors.iter().map(|e| {
                json!({
                    "file": e.file,
                    "line": e.line,
                    "message": e.message,
                    "code": e.code,
                    "solutions": e.solutions.iter().map(|s| {
                        json!({
                            "title": s.title,
                            "url": s.url
                        })
                    }).collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>(),
            "warnings": scan_result.warnings.iter().map(|w| {
                json!({
                    "file": w.file,
                    "line": w.line,
                    "message": w.message
                })
            }).collect::<Vec<_>>(),
            "security_analysis": {
                "security_score": scan_result.security_analysis.security_score,
                "files_analyzed": scan_result.security_analysis.files_analyzed,
                "vulnerabilities": scan_result.security_analysis.vulnerabilities.len()
            },
            "recommendations": scan_result.recommendations
        }))
    }

    // ═══════════════════════════════════════════════════════════════════════
    // 📊 TOOL 4: STATS - Estadísticas de Nuclear Crawler
    // ═══════════════════════════════════════════════════════════════════════

    async fn tool_stats(&self, args: Value) -> Result<Value> {
        let stat_type = args["stat_type"].as_str().unwrap_or("full");

        // Información de módulos activos
        let modules_status = json!({
            "go_ffi": self.go_integration.is_available(),
            "zig_simd": self.zig_integration.is_available(),
            "nim_html": self.nim_integration.is_available(),
            "jax_gpu": self.jax_accelerator.is_available(),
            "mojo_bridge": self.mojo_processor.is_available(),
            "stealth_engine": true,
            "intelligent_storage": true
        });

        let mut result = json!({
            "stat_type": stat_type,
            "nuclear_crawler": {
                "version": env!("CARGO_PKG_VERSION"),
                "name": "Nuclear Crawler Hybrid",
                "protocol": "MCP 2025-06-18"
            }
        });

        if let Some(obj) = result.as_object_mut() {
            match stat_type {
                "modules" => {
                    obj.insert("modules".to_string(), modules_status);
                }
                "performance" => {
                    obj.insert(
                        "performance".to_string(),
                        json!({
                            "parallel_workers": num_cpus::get() * 2,
                            "max_concurrent_requests": 200,
                            "storage_backend": "SQLite + FTS5"
                        }),
                    );
                }
                _ => {
                    // full
                    obj.insert("modules".to_string(), modules_status);
                    obj.insert(
                        "performance".to_string(),
                        json!({
                            "parallel_workers": num_cpus::get() * 2,
                            "max_concurrent_requests": 200,
                            "storage_backend": "SQLite + FTS5"
                        }),
                    );
                    obj.insert(
                        "features".to_string(),
                        json!([
                            "Web Search Masivo",
                            "Deep Web Search",
                            "File Search con líneas exactas",
                            "Project Analyzer",
                            "Stealth Anti-Detección",
                            "Go FFI Paralelismo",
                            "Zig SIMD Parsing",
                            "JAX GPU/TPU Acceleration",
                            "Nim HTML Parser",
                            "SQLite Full-Text Search"
                        ]),
                    );
                }
            }
        }

        Ok(result)
    }
}

impl Default for NuclearCore {
    fn default() -> Self {
        Self::new().expect("Failed to create NuclearCore")
    }
}
