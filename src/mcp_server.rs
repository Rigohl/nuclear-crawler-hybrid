//! Módulo MCP Server - Servidor Model Context Protocol
//! 
//! Expone las funciones del crawler como herramientas MCP usando WebAssembly

use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use serde_json::{Value, json};
use crate::crawler::Crawler;
use crate::config::CrawlerConfig;
use crate::nuclear_scraper::{NuclearScraper, NuclearConfig};
use crate::web_search::{WebSearch, WebSearchConfig};
use crate::file_search::{FileSearch, FileSearchConfig};
use crate::project_analyzer::{ProjectAnalyzer, ProjectAnalysisConfig};
use crate::stats::StatsSystem;
use crate::ai_smart::{AISmart, AIConfig};
// WasmCrawler solo disponible en wasm32
#[cfg(target_arch = "wasm32")]
use crate::wasm::WasmCrawler;

/// Servidor MCP que expone herramientas de crawling
pub struct McpServer {
    crawler: Arc<RwLock<Option<Crawler>>>,
    nuclear_scraper: Arc<RwLock<Option<NuclearScraper>>>,
    web_search: Arc<RwLock<Option<WebSearch>>>,
    project_analyzer: Arc<RwLock<Option<ProjectAnalyzer>>>,
    stats_system: Arc<RwLock<Option<StatsSystem>>>,
    #[cfg(target_arch = "wasm32")]
    wasm_crawler: Arc<RwLock<Option<WasmCrawler>>>,
}

impl McpServer {
    /// Crea un nuevo servidor MCP
    pub fn new() -> Self {
        Self {
            crawler: Arc::new(RwLock::new(None)),
            nuclear_scraper: Arc::new(RwLock::new(None)),
            web_search: Arc::new(RwLock::new(None)),
            project_analyzer: Arc::new(RwLock::new(None)),
            stats_system: Arc::new(RwLock::new(None)),
            #[cfg(target_arch = "wasm32")]
            wasm_crawler: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Inicializa sistemas principales
    pub async fn init_systems(&self) -> Result<()> {
        // Inicializar web search
        let web_search = WebSearch::new_with_storage(None)?;
        *self.web_search.write().await = Some(web_search);
        
        // Inicializar project analyzer
        let analyzer = ProjectAnalyzer::new()?;
        *self.project_analyzer.write().await = Some(analyzer);
        
        // Inicializar nuclear scraper
        self.init_nuclear_scraper(None).await?;
        
        // Inicializar stats system
        let web_search_for_stats = Arc::new(WebSearch::new_with_storage(None)?);
        let scraper_for_stats = Arc::new(NuclearScraper::new(NuclearConfig::default())?);
        let ai_for_stats = Arc::new(AISmart::new(AIConfig::default()));
        let stats = StatsSystem::new(web_search_for_stats, scraper_for_stats, ai_for_stats);
        *self.stats_system.write().await = Some(stats);
        
        Ok(())
    }
    
    /// Inicializa el scraper NUCLEAR
    pub async fn init_nuclear_scraper(&self, config: Option<NuclearConfig>) -> Result<()> {
        let config = config.unwrap_or_default();
        let scraper = NuclearScraper::new(config)?;
        *self.nuclear_scraper.write().await = Some(scraper);
        Ok(())
    }
    
    /// Inicializa el crawler
    pub async fn init_crawler(&self, config: Option<CrawlerConfig>) -> Result<()> {
        let config = config.unwrap_or_default();
        let crawler = Crawler::new(config)?;
        *self.crawler.write().await = Some(crawler);
        Ok(())
    }
    
    /// Lista las herramientas disponibles
    pub fn list_tools(&self) -> Vec<Value> {
        vec![
            json!({
                "name": "crawl_url",
                "description": "Crawlea una URL y extrae datos",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "url": {
                            "type": "string",
                            "description": "URL a crawlear"
                        }
                    },
                    "required": ["url"]
                }
            }),
            json!({
                "name": "crawl_many",
                "description": "Crawlea múltiples URLs en paralelo",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "urls": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Lista de URLs a crawlear"
                        }
                    },
                    "required": ["urls"]
                }
            }),
            json!({
                "name": "extract_data",
                "description": "Extrae datos estructurados de HTML",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "html": {
                            "type": "string",
                            "description": "HTML a procesar"
                        },
                        "base_url": {
                            "type": "string",
                            "description": "URL base para resolver links relativos"
                        }
                    },
                    "required": ["html", "base_url"]
                }
            }),
            json!({
                "name": "scrape_selector",
                "description": "Extrae datos usando selector CSS",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "html": {
                            "type": "string",
                            "description": "HTML a procesar"
                        },
                        "selector": {
                            "type": "string",
                            "description": "Selector CSS"
                        }
                    },
                    "required": ["html", "selector"]
                }
            }),
            json!({
                "name": "nuclear_crawl",
                "description": "Scraping NUCLEAR masivo sin límites - Máximo paralelismo",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "urls": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Lista de URLs para scraping masivo"
                        }
                    },
                    "required": ["urls"]
                }
            }),
            json!({
                "name": "nuclear_crawl_recursive",
                "description": "Scraping NUCLEAR recursivo - Sigue TODOS los links sin límite",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "start_urls": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "URLs iniciales para comenzar el crawling recursivo"
                        }
                    },
                    "required": ["start_urls"]
                }
            }),
            json!({
                "name": "web_search",
                "description": "Búsqueda web usando TODO el poder del crawler (Stealth + AI + Paralelismo masivo)",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "Query de búsqueda"
                        },
                        "max_results": {
                            "type": "integer",
                            "description": "Máximo número de resultados",
                            "default": 50
                        }
                    },
                    "required": ["query"]
                }
            }),
            json!({
                "name": "file_search",
                "description": "Busca palabras exactas, errores, regex en archivos locales",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "term": {
                            "type": "string",
                            "description": "Término a buscar"
                        },
                        "dir": {
                            "type": "string",
                            "description": "Directorio raíz para buscar",
                            "default": "."
                        },
                        "exact": {
                            "type": "boolean",
                            "description": "Búsqueda exacta (case sensitive)",
                            "default": false
                        },
                        "regex": {
                            "type": "boolean",
                            "description": "Usar regex",
                            "default": false
                        }
                    },
                    "required": ["term"]
                }
            }),
            json!({
                "name": "analyze_project",
                "description": "Analiza proyecto y busca lo más moderno y poderoso (librerías, mejores prácticas, herramientas)",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "dir": {
                            "type": "string",
                            "description": "Directorio del proyecto",
                            "default": "."
                        }
                    }
                }
            }),
            json!({
                "name": "get_stats",
                "description": "Obtiene estadísticas completas y últimas búsquedas",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "recent": {
                            "type": "integer",
                            "description": "Número de búsquedas recientes a mostrar",
                            "default": 10
                        }
                    }
                }
            }),
        ]
    }
    
    /// Ejecuta una herramienta
    pub async fn call_tool(&self, name: &str, args: Value) -> Result<Value> {
        match name {
            "crawl_url" => {
                let url = args["url"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("URL requerida"))?;
                
                let crawler = self.crawler.read().await;
                if let Some(ref c) = *crawler {
                    let result = c.crawl_url(url).await?;
                    Ok(json!({
                        "url": result.url,
                        "status_code": result.status_code,
                        "content_type": result.content_type,
                        "content_length": result.content_length,
                        "response_time_ms": result.response_time.as_millis(),
                        "links_found": result.links_found,
                        "crawled_at": result.crawled_at.to_rfc3339(),
                    }))
                } else {
                    anyhow::bail!("Crawler no inicializado. Llama a init_crawler primero.")
                }
            }
            
            "crawl_many" => {
                let urls = args["urls"]
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("urls debe ser un array"))?
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>();
                
                let crawler = self.crawler.read().await;
                if let Some(ref c) = *crawler {
                    let results = c.crawl_many(urls).await?;
                    let results_json: Vec<Value> = results.into_iter().map(|r| {
                        json!({
                            "url": r.url,
                            "status_code": r.status_code,
                            "content_type": r.content_type,
                            "content_length": r.content_length,
                            "response_time_ms": r.response_time.as_millis(),
                            "links_found": r.links_found,
                        })
                    }).collect();
                    Ok(json!({ "results": results_json }))
                } else {
                    anyhow::bail!("Crawler no inicializado")
                }
            }
            
            "extract_data" => {
                let html = args["html"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("html requerido"))?;
                let base_url = args["base_url"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("base_url requerido"))?;
                
                use crate::scraper::Scraper;
                let scraper = Scraper::new();
                let data = scraper.extract_all(html, base_url)?;
                
                Ok(json!({
                    "title": data.title,
                    "meta": data.meta,
                    "links": data.links,
                    "images": data.images,
                    "headings": data.headings,
                    "tables": data.tables,
                }))
            }
            
            "scrape_selector" => {
                let html = args["html"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("html requerido"))?;
                let selector = args["selector"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("selector requerido"))?;
                
                use crate::scraper::Scraper;
                let scraper = Scraper::new();
                let results = scraper.extract_by_selector(html, selector)?;
                
                Ok(json!({ "results": results }))
            }
            
            "nuclear_crawl" => {
                let urls = args["urls"]
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("urls debe ser un array"))?
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>();
                
                let scraper = self.nuclear_scraper.read().await;
                if let Some(ref ns) = *scraper {
                    let results = ns.nuclear_crawl(urls).await?;
                    let results_json: Vec<Value> = results.into_iter().map(|r| {
                        json!({
                            "url": r.url,
                            "status_code": r.status_code,
                            "content_length": r.content_length,
                            "response_time_ms": r.response_time.as_millis(),
                            "links_found": r.links_found,
                            "images_found": r.images_found,
                            "error": r.error,
                        })
                    }).collect();
                    
                    // Agregar estadísticas
                    if let Some(stats) = ns.get_stats() {
                        Ok(json!({
                            "results": results_json,
                            "stats": {
                                "total_urls": stats.total_urls_crawled,
                                "successful": stats.successful,
                                "failed": stats.failed,
                                "total_data_bytes": stats.total_data_captured,
                                "total_links": stats.total_links_found,
                                "total_images": stats.total_images_found,
                                "total_time_ms": stats.total_time.as_millis(),
                                "urls_per_second": stats.urls_per_second,
                                "avg_response_time_ms": stats.avg_response_time.as_millis(),
                            }
                        }))
                    } else {
                        Ok(json!({ "results": results_json }))
                    }
                } else {
                    anyhow::bail!("Nuclear scraper no inicializado. Llama a init_nuclear_scraper primero.")
                }
            }
            
            "nuclear_crawl_recursive" => {
                let start_urls = args["start_urls"]
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("start_urls debe ser un array"))?
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>();
                
                let scraper = self.nuclear_scraper.read().await;
                if let Some(ref ns) = *scraper {
                    let results = ns.nuclear_crawl_recursive(start_urls).await?;
                    let results_json: Vec<Value> = results.into_iter().map(|r| {
                        json!({
                            "url": r.url,
                            "status_code": r.status_code,
                            "content_length": r.content_length,
                            "response_time_ms": r.response_time.as_millis(),
                            "links_found": r.links_found,
                            "images_found": r.images_found,
                            "error": r.error,
                        })
                    }).collect();
                    
                    if let Some(stats) = ns.get_stats() {
                        Ok(json!({
                            "results": results_json,
                            "stats": {
                                "total_urls": stats.total_urls_crawled,
                                "successful": stats.successful,
                                "failed": stats.failed,
                                "total_data_bytes": stats.total_data_captured,
                                "total_links": stats.total_links_found,
                                "total_images": stats.total_images_found,
                                "total_time_ms": stats.total_time.as_millis(),
                                "urls_per_second": stats.urls_per_second,
                            }
                        }))
                    } else {
                        Ok(json!({ "results": results_json }))
                    }
                } else {
                    anyhow::bail!("Nuclear scraper no inicializado")
                }
            }
            
            "web_search" => {
                let query = args["query"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("query requerido"))?
                    .to_string();
                let max_results = args["max_results"]
                    .as_u64()
                    .map(|v| v as usize)
                    .unwrap_or(50);
                
                let web_search = self.web_search.read().await;
                if let Some(ref ws) = *web_search {
                    let config = WebSearchConfig {
                        query,
                        max_results,
                        ..Default::default()
                    };
                    let results = ws.search(config).await?;
                    
                    let results_json: Vec<Value> = results.into_iter().map(|r| {
                        json!({
                            "url": r.url,
                            "title": r.title,
                            "description": r.description,
                            "relevance": r.relevance,
                            "quality_score": r.quality_score,
                            "source": r.source,
                        })
                    }).collect();
                    
                    Ok(json!({
                        "results": results_json,
                        "total": results_json.len(),
                    }))
                } else {
                    anyhow::bail!("WebSearch no inicializado. Llama a init_systems primero.")
                }
            }
            
            "file_search" => {
                let term = args["term"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("term requerido"))?
                    .to_string();
                let dir = args["dir"]
                    .as_str()
                    .map(std::path::PathBuf::from)
                    .unwrap_or_else(|| std::path::PathBuf::from("."));
                let exact = args["exact"].as_bool().unwrap_or(false);
                let regex = args["regex"].as_bool().unwrap_or(false);
                
                let config = FileSearchConfig {
                    search_term: term,
                    root_dir: dir,
                    exact_match: exact,
                    use_regex: regex,
                    ..Default::default()
                };
                
                let results = FileSearch::search(config)?;
                
                let results_json: Vec<Value> = results.into_iter().map(|r| {
                    json!({
                        "file_path": r.file_path,
                        "line_number": r.line_number,
                        "line_content": r.line_content,
                        "match_count": r.match_count,
                        "match_type": r.match_type,
                    })
                }).collect();
                
                Ok(json!({
                    "results": results_json,
                    "total": results_json.len(),
                }))
            }
            
            "analyze_project" => {
                let dir = args["dir"]
                    .as_str()
                    .map(std::path::PathBuf::from)
                    .unwrap_or_else(|| std::path::PathBuf::from("."));
                
                let analyzer = self.project_analyzer.read().await;
                if let Some(ref pa) = *analyzer {
                    let config = ProjectAnalysisConfig {
                        project_dir: dir,
                        search_modern_libs: true,
                        search_best_practices: true,
                        search_tools: true,
                        ..Default::default()
                    };
                    
                    let analysis = pa.analyze_project(config).await?;
                    
                    let recommendations_json: Vec<Value> = analysis.recommendations.into_iter().map(|r| {
                        json!({
                            "title": r.title,
                            "description": r.description,
                            "reference": r.reference,
                            "priority": r.priority,
                            "category": r.category,
                        })
                    }).collect();
                    
                    Ok(json!({
                        "analysis_type": analysis.analysis_type,
                        "modernity_score": analysis.modernity_score,
                        "recommendations": recommendations_json,
                        "total_recommendations": recommendations_json.len(),
                    }))
                } else {
                    anyhow::bail!("ProjectAnalyzer no inicializado. Llama a init_systems primero.")
                }
            }
            
            "get_stats" => {
                let recent = args["recent"]
                    .as_u64()
                    .map(|v| v as usize)
                    .unwrap_or(10);
                
                let stats_system = self.stats_system.read().await;
                if let Some(ref stats) = *stats_system {
                    let full_stats = stats.get_full_stats();
                    let recent_searches = stats.get_recent_searches(recent);
                    
                    let recent_json: Vec<Value> = recent_searches.into_iter().map(|s| {
                        json!({
                            "query": s.query,
                            "results_count": s.results_count,
                            "duration_ms": s.duration_ms,
                            "success": s.success,
                        })
                    }).collect();
                    
                    Ok(json!({
                        "web_search": {
                            "total_searches": full_stats.web_search_stats.total_searches,
                            "successful": full_stats.web_search_stats.successful_searches,
                            "failed": full_stats.web_search_stats.failed_searches,
                            "avg_results": full_stats.web_search_stats.avg_results_per_search,
                            "total_results": full_stats.web_search_stats.total_results_found,
                        },
                        "scraping": {
                            "total_urls": full_stats.scraping_stats.total_urls_crawled,
                            "successful": full_stats.scraping_stats.successful,
                            "failed": full_stats.scraping_stats.failed,
                            "data_mb": full_stats.scraping_stats.total_data_captured_mb,
                            "urls_per_second": full_stats.scraping_stats.urls_per_second,
                        },
                        "ai": {
                            "patterns_learned": full_stats.ai_stats.patterns_learned,
                            "domains_analyzed": full_stats.ai_stats.domains_analyzed,
                            "accuracy": full_stats.ai_stats.accuracy,
                        },
                        "uptime_seconds": full_stats.total_uptime.as_secs(),
                        "recent_searches": recent_json,
                    }))
                } else {
                    anyhow::bail!("StatsSystem no inicializado. Llama a init_systems primero.")
                }
            }
            
            _ => {
                anyhow::bail!("Herramienta desconocida: {}", name)
            }
        }
    }
    
    /// Obtiene información del servidor
    pub fn get_info(&self) -> Value {
        json!({
            "name": "nuclear_crawler_hybrid",
            "version": crate::version(),
            "description": "Crawler/Scraper avanzado con WebAssembly para MCP",
            "tools": self.list_tools().len(),
        })
    }
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}

