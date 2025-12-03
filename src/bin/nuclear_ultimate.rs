//! 🔥 NUCLEAR ULTIMATE MCP 2025 - 5 Herramientas TODO INTEGRADO
//!
//! Binario único que integra TODOS los módulos:
//! - WebSearch masiva (usa TODOS los módulos)
//! - Analyzer (proyectos y documentos)
//! - File Research (búsqueda exacta en archivos)
//! - Stats (estadísticas del MCP)
//! - Orchestrator (orquestación de tareas masivas)
//!
//! Modos: STUDIO (stdio) o HTTP (Axum)
//! Protocolo: MCP 2025-06-18

use anyhow::Result;
use chrono::Utc;
use clap::Parser;
use serde_json::{json, Value};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

// 🔥 NUCLEAR: TODOS los módulos
use nuclear_crawler_lib::ai_smart::{AIConfig, AISmart};
use nuclear_crawler_lib::config::CrawlerConfig;
use nuclear_crawler_lib::deep_web_search::{
    DeepWebSearch, DeepWebSearchConfig, DeepWebSearchType, DeepWebSource,
};
use nuclear_crawler_lib::file_search::{FileSearch, FileSearchConfig};
use nuclear_crawler_lib::improvements::{BloomFilter, CircuitBreaker, MemoryCache};
use nuclear_crawler_lib::intelligent_storage::IntelligentStorage;
use nuclear_crawler_lib::massive_parallel_search::MassiveParallelSearch;
use nuclear_crawler_lib::nuclear_scraper::{NuclearConfig, NuclearScraper};
use nuclear_crawler_lib::orchestration::{OrchestrationConfig, Orchestrator};
use nuclear_crawler_lib::parallel_crawler::ParallelCrawler;
use nuclear_crawler_lib::project_analyzer::ProjectAnalyzer;
use nuclear_crawler_lib::scan_project::ProjectScanner;
use nuclear_crawler_lib::stats::StatsSystem;
use nuclear_crawler_lib::web_search::{WebSearch, WebSearchConfig};

// Versión del protocolo MCP 2025
const MCP_PROTOCOL_VERSION: &str = "2025-06-18";

/// 🔥 NUCLEAR ULTIMATE MCP
#[derive(Parser, Debug)]
#[command(name = "nuclear-ultimate")]
#[command(about = "🔥 NUCLEAR ULTIMATE - 5 Herramientas MCP 2025 con TODO integrado")]
struct Args {
    /// Modo: studio (stdio) o http (axum server)
    #[arg(short, long, default_value = "studio")]
    mode: String,

    /// Puerto para modo HTTP
    #[arg(short, long, default_value = "3000")]
    port: u16,

    /// Directorio de resultados
    #[arg(short, long, default_value = "resultados")]
    results_dir: String,
}

/// 🔥 NUCLEAR ULTIMATE: Sistema completo
pub struct NuclearUltimate {
    web_search: Arc<WebSearch>,
    deep_web_search: Arc<DeepWebSearch>,
    #[allow(dead_code)]
    nuclear_scraper: Arc<NuclearScraper>,
    #[allow(dead_code)]
    parallel_crawler: Arc<ParallelCrawler>,
    massive_search: Arc<MassiveParallelSearch>,
    file_search: Arc<FileSearch>,
    project_scanner: Arc<ProjectScanner>,
    project_analyzer: Arc<ProjectAnalyzer>,
    #[allow(dead_code)]
    orchestrator: Arc<Orchestrator>,
    storage: Arc<IntelligentStorage>,
    stats_system: Arc<StatsSystem>,
    ai_smart: Arc<AISmart>,
    bloom_filter: Arc<std::sync::Mutex<BloomFilter>>,
    circuit_breaker: Arc<std::sync::Mutex<CircuitBreaker>>,
    memory_cache: Arc<MemoryCache<String, String>>,
    results_dir: PathBuf,
    visited_urls_file: PathBuf,
}

impl NuclearUltimate {
    pub fn new(results_dir: &str) -> Result<Self> {
        let results_path = PathBuf::from(results_dir);
        fs::create_dir_all(&results_path)?;
        let visited_urls_file = results_path.join("urls_visited.txt");

        let storage = Arc::new(IntelligentStorage::new(None)?);
        let nuclear_config = NuclearConfig::default();
        let nuclear_scraper = Arc::new(NuclearScraper::new_with_storage(
            nuclear_config,
            Some(storage.clone()),
        )?);
        let web_search = Arc::new(WebSearch::new_with_storage(Some(storage.clone()))?);
        let deep_web_search = Arc::new(DeepWebSearch::new(
            nuclear_scraper.clone(),
            web_search.clone(),
        ));
        let ai_config = AIConfig::default();
        let ai_smart = Arc::new(AISmart::new(ai_config));
        let crawler_config = CrawlerConfig::default();
        let parallel_crawler = Arc::new(ParallelCrawler::new(crawler_config)?);
        let massive_search = Arc::new(MassiveParallelSearch::new(
            nuclear_scraper.clone(),
            ai_smart.clone(),
        ));
        let file_search = Arc::new(FileSearch::new());
        let project_scanner = Arc::new(ProjectScanner::new(web_search.clone()));
        let project_analyzer = Arc::new(ProjectAnalyzer::new(web_search.clone()));
        let orchestrator = Arc::new(Orchestrator::new(OrchestrationConfig::default())?);
        let stats_system = Arc::new(StatsSystem::new(
            web_search.clone(),
            nuclear_scraper.clone(),
            ai_smart.clone(),
        ));
        let bloom_filter = Arc::new(std::sync::Mutex::new(BloomFilter::new(100_000, 0.01)));
        let circuit_breaker = Arc::new(std::sync::Mutex::new(CircuitBreaker::new(5, 30)));
        let memory_cache = Arc::new(MemoryCache::new(10_000));

        Ok(Self {
            web_search,
            deep_web_search,
            nuclear_scraper,
            parallel_crawler,
            massive_search,
            file_search,
            project_scanner,
            project_analyzer,
            orchestrator,
            storage,
            stats_system,
            ai_smart,
            bloom_filter,
            circuit_breaker,
            memory_cache,
            results_dir: results_path,
            visited_urls_file,
        })
    }

    fn save_visited_url(&self, url: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.visited_urls_file)
        {
            let _ = writeln!(file, "{} | {}", Utc::now().format("%Y-%m-%d %H:%M:%S"), url);
        }
    }

    fn save_result(&self, tool_name: &str, result: &Value) -> Result<PathBuf> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("{}_{}.json", tool_name, timestamp);
        let filepath = self.results_dir.join(&filename);
        let mut file = File::create(&filepath)?;
        file.write_all(serde_json::to_string_pretty(result)?.as_bytes())?;
        Ok(filepath)
    }

    pub fn list_tools(&self) -> Vec<Value> {
        vec![
            json!({
                "name": "websearch",
                "description": "🔥 Búsqueda web MASIVA con AI, Stealth, DeepWeb",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "query": {"type": "string"},
                        "max_results": {"type": "integer"},
                        "deep_web": {"type": "boolean"}
                    },
                    "required": ["query"]
                }
            }),
            json!({
                "name": "analyzer",
                "description": "🔍 Analiza proyectos y documentos",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "path": {"type": "string"},
                        "analyze_type": {"type": "string"}
                    },
                    "required": ["path"]
                }
            }),
            json!({
                "name": "file_research",
                "description": "📂 Búsqueda exacta en archivos",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "search_term": {"type": "string"},
                        "path": {"type": "string"},
                        "use_regex": {"type": "boolean"}
                    },
                    "required": ["search_term"]
                }
            }),
            json!({
                "name": "stats",
                "description": "📊 Estadísticas del MCP",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "stat_type": {"type": "string"}
                    }
                }
            }),
            json!({
                "name": "orchestrate",
                "description": "🎭 Orquestador de tareas masivas",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "tasks": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "type": {"type": "string"},
                                    "query": {"type": "string"},
                                    "term": {"type": "string"}
                                },
                                "required": ["type"]
                            }
                        },
                        "parallel": {"type": "boolean"},
                        "max_concurrent": {"type": "integer"}
                    },
                    "required": ["tasks"]
                }
            }),
        ]
    }

    pub async fn call_tool(&self, name: &str, args: Value) -> Result<Value> {
        let result = match name {
            "websearch" => self.tool_websearch(args).await?,
            "analyzer" => self.tool_analyzer(args).await?,
            "file_research" => self.tool_file_research(args).await?,
            "stats" => self.tool_stats(args).await?,
            "orchestrate" => self.tool_orchestrate(args).await?,
            _ => return Err(anyhow::anyhow!("Tool not found: {}", name)),
        };
        let filepath = self.save_result(name, &result)?;
        let mut final_result = result;
        if let Some(obj) = final_result.as_object_mut() {
            obj.insert("_saved_to".to_string(), json!(filepath.to_string_lossy()));
        }
        Ok(final_result)
    }

    async fn tool_websearch(&self, args: Value) -> Result<Value> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing query"))?;
        let max_results = args["max_results"].as_u64().unwrap_or(100) as usize;
        let deep_web = args["deep_web"].as_bool().unwrap_or(false);
        let use_ai = args["use_ai"].as_bool().unwrap_or(true);

        let can_proceed = {
            let cb = self.circuit_breaker.lock().unwrap();
            !cb.is_open()
        };
        if !can_proceed {
            return Ok(json!({"error": "Circuit breaker open", "status": "blocked"}));
        }

        let cache_key = format!("websearch:{}", query);
        if let Some(cached) = self.memory_cache.get(&cache_key) {
            if let Ok(result) = serde_json::from_str::<Value>(&cached) {
                return Ok(json!({"source": "cache", "results": result}));
            }
        }

        let strategy = if use_ai {
            self.ai_smart.recommend_strategy("websearch")
        } else {
            Default::default()
        };

        // 🔥 USAR MOTORES DE BÚSQUEDA REALES + SITIOS ESPECÍFICOS
        let config = WebSearchConfig {
            query: query.to_string(),
            max_results,
            priority_sources: vec![
                // Motores de búsqueda REALES
                "duckduckgo.com".into(),
                "bing.com".into(),
                "brave.com".into(),
                "ecosia.org".into(),
            ],
            sources: vec![
                // Sitios de código
                "github.com".into(),
                "stackoverflow.com".into(),
                "dev.to".into(),
                "crates.io".into(),
                // AI/ML
                "huggingface.co".into(),
                "arxiv.org".into(),
                // Comunidades
                "reddit.com".into(),
                "medium.com".into(),
            ],
            use_ai,
            use_stealth: true,
            max_parallel: 10_000,
            timeout_secs: 15, // Más tiempo para buscar
            max_urls: 200,
        };

        let web_results = self.web_search.search(config).await?;
        for result in &web_results {
            self.save_visited_url(&result.url);
            if let Ok(mut bloom) = self.bloom_filter.lock() {
                bloom.insert(&result.url);
            }
        }

        let deep_results = if deep_web {
            let deep_config = DeepWebSearchConfig {
                query: query.to_string(),
                search_type: DeepWebSearchType::All,
                sources: vec![DeepWebSource::Academic, DeepWebSource::CodeRepos],
                max_results: 20,
                find_access_methods: true,
                use_advanced_techniques: true,
            };
            self.deep_web_search
                .search(deep_config)
                .await
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        // 🔥 BÚSQUEDA MASIVA CON EL QUERY EN MOTORES DE BÚSQUEDA
        let massive_results = self
            .massive_search
            .search_with_query(
                query,
                vec![
                    // Motores de búsqueda
                    "duckduckgo.com".into(),
                    "bing.com".into(),
                    "brave.com".into(),
                    "searx.be".into(),
                    // Sitios específicos
                    "github.com".into(),
                    "stackoverflow.com".into(),
                    "dev.to".into(),
                    "reddit.com".into(),
                    "huggingface.co".into(),
                    "crates.io".into(),
                    "arxiv.org".into(),
                    "medium.com".into(),
                ],
            )
            .await
            .unwrap_or_default();

        {
            let mut cb = self.circuit_breaker.lock().unwrap();
            cb.record_success();
        }

        let result = json!({
            "tool": "websearch", "query": query, "strategy": format!("{:?}", strategy),
            "web_results": web_results.len(), "deep_web_results": deep_results.len(),
            "massive_results": massive_results.len(),
            "total": web_results.len() + deep_results.len() + massive_results.len(),
            "results": {"web": web_results, "deep_web": deep_results, "massive": massive_results}
        });

        if let Ok(json_str) = serde_json::to_string(&result) {
            self.memory_cache.insert(cache_key, json_str);
        }
        Ok(result)
    }

    async fn tool_analyzer(&self, args: Value) -> Result<Value> {
        let path = args["path"].as_str().unwrap_or(".");
        let analyze_type = args["analyze_type"].as_str().unwrap_or("all");
        let deep_scan = args["deep_scan"].as_bool().unwrap_or(true);
        let path_buf = PathBuf::from(path);

        if !path_buf.exists() {
            return Ok(json!({"error": format!("Path not found: {}", path)}));
        }

        let mut results = serde_json::Map::new();

        if analyze_type == "project" || analyze_type == "all" {
            let scan_result = self.project_scanner.scan_project(path_buf.clone()).await?;
            results.insert("project_scan".to_string(), json!(scan_result));
            if deep_scan {
                let analysis = self.project_analyzer.analyze(&path_buf).await?;
                results.insert("deep_analysis".to_string(), analysis);
            }
        }

        if (analyze_type == "document" || analyze_type == "all") && path_buf.is_file() {
            let content = fs::read_to_string(&path_buf)?;
            results.insert("document".to_string(), json!({
                "size_bytes": content.len(), "lines": content.lines().count(), "words": content.split_whitespace().count()
            }));
        }

        if analyze_type == "security" || analyze_type == "all" {
            let security = self.analyze_security(&path_buf).await?;
            results.insert("security".to_string(), security);
        }

        Ok(
            json!({"tool": "analyzer", "path": path, "analyze_type": analyze_type, "results": results}),
        )
    }

    async fn analyze_security(&self, path: &PathBuf) -> Result<Value> {
        let patterns = [
            ("password", "Posible contraseña"),
            ("api_key", "API key"),
            ("secret", "Secreto"),
            ("token", "Token"),
        ];
        let config = FileSearchConfig {
            search_term: "password|api_key|secret|token".to_string(),
            root_dir: path.clone(),
            include_patterns: vec!["*.rs".into(), "*.py".into(), "*.js".into(), "*.env".into()],
            exclude_patterns: vec!["target/".into(), "node_modules/".into(), ".git/".into()],
            exact_match: false,
            use_regex: true,
            search_in_content: true,
            search_in_filename: false,
            max_results: 100,
        };

        let mut issues = Vec::new();
        if let Ok(results) = self.file_search.search(config).await {
            for r in results {
                for (pat, msg) in &patterns {
                    if r.line_content.to_lowercase().contains(pat) {
                        issues.push(
                            json!({"file": r.file_path, "line": r.line_number, "issue": msg}),
                        );
                    }
                }
            }
        }
        Ok(json!({"issues_found": issues.len(), "issues": issues}))
    }

    async fn tool_file_research(&self, args: Value) -> Result<Value> {
        let search_term = args["search_term"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing search_term"))?;
        let path = args["path"].as_str().unwrap_or(".");
        let use_regex = args["use_regex"].as_bool().unwrap_or(false);
        let case_sensitive = args["case_sensitive"].as_bool().unwrap_or(false);

        let config = FileSearchConfig {
            search_term: search_term.to_string(),
            root_dir: PathBuf::from(path),
            include_patterns: vec!["*".into()],
            exclude_patterns: vec!["target/".into(), "node_modules/".into(), ".git/".into()],
            exact_match: case_sensitive,
            use_regex,
            search_in_content: true,
            search_in_filename: true,
            max_results: 1000,
        };

        let results = self.file_search.search(config).await?;
        Ok(
            json!({"tool": "file_research", "search_term": search_term, "path": path, "results_count": results.len(), "results": results}),
        )
    }

    async fn tool_stats(&self, args: Value) -> Result<Value> {
        let stat_type = args["stat_type"].as_str().unwrap_or("full");
        let system_stats = self.stats_system.get_all_stats();
        let storage_stats =
            json!({"database": "SQLite", "entries": self.storage.count_entries().unwrap_or(0)});
        let visited_count = if self
            .visited_urls_file
            .exists() { {
                fs::read_to_string(&self.visited_urls_file)
                    .map(|c| c.lines().count())
                    .unwrap_or(0)
            } } else { 0 };
        let results_count = fs::read_dir(&self.results_dir)
            .map(|e| e.count())
            .unwrap_or(0);
        let cb_state = {
            let cb = self.circuit_breaker.lock().unwrap();
            format!("{:?}", cb.state())
        };

        let mut stats = json!({
            "tool": "stats", "stat_type": stat_type, "mcp_version": MCP_PROTOCOL_VERSION,
            "system": system_stats, "storage": storage_stats, "visited_urls": visited_count,
            "saved_results": results_count, "results_dir": self.results_dir.to_string_lossy(),
            "cache": {"memory_cache_size": self.memory_cache.len(), "circuit_breaker": cb_state}
        });

        if stat_type != "full" {
            if let Some(obj) = stats.as_object_mut() {
                match stat_type {
                    "recent" => obj
                        .retain(|k, _| k == "tool" || k == "visited_urls" || k == "saved_results"),
                    "performance" => {
                        obj.retain(|k, _| k == "tool" || k == "system" || k == "cache")
                    }
                    "storage" => {
                        obj.retain(|k, _| k == "tool" || k == "storage" || k == "results_dir")
                    }
                    _ => {}
                }
            }
        }
        Ok(stats)
    }

    async fn tool_orchestrate(&self, args: Value) -> Result<Value> {
        let tasks = args["tasks"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Missing tasks"))?;
        let parallel = args["parallel"].as_bool().unwrap_or(true);
        let max_concurrent = args["max_concurrent"].as_u64().unwrap_or(100) as usize;
        let start = std::time::Instant::now();
        let mut results = Vec::new();

        if parallel {
            let sem = Arc::new(tokio::sync::Semaphore::new(max_concurrent));
            let mut handles = Vec::new();

            for (i, task) in tasks.iter().enumerate() {
                let task_clone = task.clone();
                let sem = sem.clone();
                let ws = self.web_search.clone();
                let fs = self.file_search.clone();
                let ps = self.project_scanner.clone();
                let ss = self.stats_system.clone();

                handles.push(tokio::spawn(async move {
                    let _p = sem.acquire().await.unwrap();
                    let tt = task_clone["type"].as_str().unwrap_or("unknown");
                    let res = match tt {
                        "search" => {
                            let q = task_clone["query"].as_str().unwrap_or("");
                            match ws
                                .search(WebSearchConfig {
                                    query: q.to_string(),
                                    max_results: 10,
                                    ..Default::default()
                                })
                                .await
                            {
                                Ok(r) => json!({"status": "ok", "count": r.len(), "mode": "parallel"}),
                                Err(e) => json!({"status": "error", "error": e.to_string()}),
                            }
                        }
                        "file_search" => {
                            let t = task_clone["term"].as_str().unwrap_or("");
                            match fs
                                .search(FileSearchConfig {
                                    search_term: t.to_string(),
                                    ..Default::default()
                                })
                                .await
                            {
                                Ok(r) => json!({"status": "ok", "count": r.len(), "mode": "parallel"}),
                                Err(e) => json!({"status": "error", "error": e.to_string()}),
                            }
                        }
                        "analyzer" => {
                            let path = task_clone["path"].as_str().unwrap_or(".");
                            match ps.scan_project(PathBuf::from(path)).await {
                                Ok(r) => json!({"status": "ok", "issues": r.total_issues, "mode": "parallel"}),
                                Err(e) => json!({"status": "error", "error": e.to_string()}),
                            }
                        }
                        "stats" => {
                            json!({"status": "ok", "stats": ss.get_all_stats(), "mode": "parallel"})
                        }
                        _ => json!({"status": "unknown_type", "type": tt}),
                    };
                    (i, res)
                }));
            }

            for h in handles {
                if let Ok((i, r)) = h.await {
                    results.push(json!({"task_index": i, "result": r}));
                }
            }
        } else {
            // 🔥 MODO SECUENCIAL REAL - Ejecuta tareas una por una
            for (i, task) in tasks.iter().enumerate() {
                let tt = task["type"].as_str().unwrap_or("unknown");
                let res = match tt {
                    "search" => {
                        let q = task["query"].as_str().unwrap_or("");
                        match self.web_search
                            .search(WebSearchConfig {
                                query: q.to_string(),
                                max_results: 10,
                                ..Default::default()
                            })
                            .await
                        {
                            Ok(r) => json!({"status": "ok", "count": r.len(), "mode": "sequential"}),
                            Err(e) => json!({"status": "error", "error": e.to_string()}),
                        }
                    }
                    "file_search" => {
                        let t = task["term"].as_str().unwrap_or("");
                        match self.file_search
                            .search(FileSearchConfig {
                                search_term: t.to_string(),
                                ..Default::default()
                            })
                            .await
                        {
                            Ok(r) => json!({"status": "ok", "count": r.len(), "mode": "sequential"}),
                            Err(e) => json!({"status": "error", "error": e.to_string()}),
                        }
                    }
                    "analyzer" => {
                        let path = task["path"].as_str().unwrap_or(".");
                        match self.project_scanner.scan_project(PathBuf::from(path)).await {
                            Ok(r) => json!({"status": "ok", "issues": r.total_issues, "mode": "sequential"}),
                            Err(e) => json!({"status": "error", "error": e.to_string()}),
                        }
                    }
                    "stats" => {
                        json!({"status": "ok", "stats": self.stats_system.get_all_stats(), "mode": "sequential"})
                    }
                    _ => json!({"status": "unknown_type", "type": tt}),
                };
                results.push(json!({"task_index": i, "result": res}));
            }
        }

        Ok(
            json!({"tool": "orchestrate", "tasks_count": tasks.len(), "parallel": parallel, "execution_time_ms": start.elapsed().as_millis(), "results": results}),
        )
    }

    // ═══════════════════════════════════════════════════════════════════════
    // 🔌 MODO STUDIO (stdio) - MCP 2025
    // ═══════════════════════════════════════════════════════════════════════

    pub async fn run_studio(&self) -> Result<()> {
        if std::env::var("RUST_LOG").is_ok() {
            eprintln!("🔥 NUCLEAR ULTIMATE MCP 2025 - Modo STUDIO");
            eprintln!("   📋 Protocolo: {}", MCP_PROTOCOL_VERSION);
            eprintln!("   📂 Resultados: {}", self.results_dir.display());
        }

        let stdin = tokio::io::stdin();
        let mut stdin = tokio::io::BufReader::new(stdin);
        let mut stdout = tokio::io::stdout();
        let mut line = String::new();

        loop {
            line.clear();
            match stdin.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    let l = line.trim();
                    if l.is_empty() {
                        continue;
                    }
                    let response = self.handle_jsonrpc(l).await;
                    let rs = format!("{}\n", serde_json::to_string(&response)?);
                    stdout.write_all(rs.as_bytes()).await?;
                    stdout.flush().await?;
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
            }
        }
        Ok(())
    }

    async fn handle_jsonrpc(&self, input: &str) -> Value {
        let request: Value = match serde_json::from_str(input) {
            Ok(v) => v,
            Err(e) => {
                return json!({"jsonrpc": "2.0", "error": {"code": -32700, "message": e.to_string()}, "id": null})
            }
        };

        // 🔑 Verificar si es una notificación (sin "id") o request (con "id")
        let has_id = request.get("id").map(|v| !v.is_null()).unwrap_or(false);
        let id = request.get("id").cloned().unwrap_or(Value::Null);
        let method = request["method"].as_str().unwrap_or("");
        let params = request.get("params").cloned().unwrap_or(json!({}));

        // 🔑 Notificaciones estándar MCP 2025 a ignorar silenciosamente
        let standard_notifications = [
            "logging/setLevel",
            "notifications/progress",
            "notifications/resources/list_changed",
        ];

        // Si es una notificación desconocida, ignorar silenciosamente
        if !has_id && standard_notifications.contains(&method) {
            return json!({});
        }

        let result = match method {
            "initialize" => json!({
                "protocolVersion": MCP_PROTOCOL_VERSION,
                "capabilities": {"tools": {"listChanged": true}, "resources": {"subscribe": false}, "prompts": {"listChanged": false}, "logging": {}},
                "serverInfo": {"name": "nuclear-ultimate", "version": env!("CARGO_PKG_VERSION")}
            }),
            "notifications/initialized" | "initialized" => {
                if !has_id {
                    return json!({});
                }
                json!({})
            },
            "tools/list" => json!({"tools": self.list_tools()}),
            "tools/call" => {
                let name = params["name"].as_str().unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));
                match self.call_tool(name, args).await {
                    Ok(r) => {
                        json!({"content": [{"type": "text", "text": serde_json::to_string_pretty(&r).unwrap_or_default()}], "isError": false})
                    }
                    Err(e) => {
                        json!({"content": [{"type": "text", "text": format!("Error: {}", e)}], "isError": true})
                    }
                }
            }
            "resources/list" => json!({"resources": []}),
            "prompts/list" => json!({"prompts": []}),
            "ping" => json!({}),
            "logging/setLevel" => json!({}),  // ✅ Soportar logging/setLevel
            _ => {
                // 🔑 Solo responder con error si tiene "id" (es un request)
                if has_id {
                    return json!({"jsonrpc": "2.0", "error": {"code": -32601, "message": format!("Method not found: {}", method)}, "id": id})
                } else {
                    // Es una notificación desconocida - ignorar silenciosamente
                    return json!({});
                }
            }
        };

        json!({"jsonrpc": "2.0", "result": result, "id": id})
    }

    // ═══════════════════════════════════════════════════════════════════════
    // 🌐 MODO HTTP (Axum) - MCP 2025
    // ═══════════════════════════════════════════════════════════════════════

    pub async fn run_http(&self, port: u16) -> Result<()> {
        use axum::{
            extract::State,
            routing::{get, post},
            Json, Router,
        };
        use std::net::SocketAddr;

        if std::env::var("RUST_LOG").is_ok() {
            eprintln!("🔥 NUCLEAR ULTIMATE MCP 2025 - Modo HTTP");
            eprintln!("   📋 Protocolo: {}", MCP_PROTOCOL_VERSION);
            eprintln!("   🌐 Puerto: {}", port);
        }

        #[derive(Clone)]
        struct AppState {
            web_search: Arc<WebSearch>,
            file_search: Arc<FileSearch>,
            stats_system: Arc<StatsSystem>,
        }

        let state = AppState {
            web_search: self.web_search.clone(),
            file_search: self.file_search.clone(),
            stats_system: self.stats_system.clone(),
        };

        async fn health() -> &'static str {
            "OK"
        }

        async fn tools() -> Json<Value> {
            Json(json!({"tools": [
                {"name": "websearch", "description": "🔥 Búsqueda web masiva"},
                {"name": "analyzer", "description": "🔍 Análisis de proyectos"},
                {"name": "file_research", "description": "📂 Búsqueda en archivos"},
                {"name": "stats", "description": "📊 Estadísticas"},
                {"name": "orchestrate", "description": "🎭 Orquestación"}
            ]}))
        }

        async fn call(State(s): State<AppState>, Json(body): Json<Value>) -> Json<Value> {
            let name = body["name"].as_str().unwrap_or("");
            let args = body.get("arguments").cloned().unwrap_or(json!({}));
            let result = match name {
                "websearch" => {
                    let q = args["query"].as_str().unwrap_or("");
                    match s
                        .web_search
                        .search(WebSearchConfig {
                            query: q.to_string(),
                            max_results: 100,
                            ..Default::default()
                        })
                        .await
                    {
                        Ok(r) => json!({"results": r.len(), "data": r}),
                        Err(e) => json!({"error": e.to_string()}),
                    }
                }
                "file_research" => {
                    let term = args["search_term"].as_str().unwrap_or("");
                    let path = args["path"].as_str().unwrap_or(".");
                    let config = FileSearchConfig {
                        search_term: term.to_string(),
                        root_dir: PathBuf::from(path),
                        include_patterns: vec!["*".into()],
                        exclude_patterns: vec![
                            "target/".into(),
                            "node_modules/".into(),
                            ".git/".into(),
                        ],
                        exact_match: false,
                        use_regex: args["use_regex"].as_bool().unwrap_or(false),
                        search_in_content: true,
                        search_in_filename: true,
                        max_results: 1000,
                    };
                    match s.file_search.search(config).await {
                        Ok(r) => json!({"results": r.len(), "data": r}),
                        Err(e) => json!({"error": e.to_string()}),
                    }
                }
                "stats" => json!({"stats": s.stats_system.get_all_stats()}),
                _ => json!({"error": format!("Tool not implemented: {}", name)}),
            };
            Json(result)
        }

        async fn mcp(State(s): State<AppState>, Json(req): Json<Value>) -> Json<Value> {
            let id = req.get("id").cloned().unwrap_or(Value::Null);
            let method = req["method"].as_str().unwrap_or("");
            let params = req.get("params").cloned().unwrap_or(json!({}));

            let result = match method {
                "initialize" => {
                    json!({"protocolVersion": MCP_PROTOCOL_VERSION, "capabilities": {"tools": {"listChanged": true}}, "serverInfo": {"name": "nuclear-ultimate-http", "version": env!("CARGO_PKG_VERSION")}})
                }
                "tools/list" => json!({"tools": [
                    {"name": "websearch", "description": "🔥 Búsqueda web MASIVA", "inputSchema": {"type": "object", "properties": {"query": {"type": "string"}, "max_results": {"type": "integer"}, "deep_web": {"type": "boolean"}}, "required": ["query"]}},
                    {"name": "file_research", "description": "📂 Búsqueda en archivos", "inputSchema": {"type": "object", "properties": {"search_term": {"type": "string"}, "path": {"type": "string"}, "use_regex": {"type": "boolean"}}, "required": ["search_term"]}},
                    {"name": "stats", "description": "📊 Estadísticas completas", "inputSchema": {"type": "object", "properties": {"stat_type": {"type": "string"}}}}
                ]}),
                "tools/call" => {
                    let name = params["name"].as_str().unwrap_or("");
                    let args = params.get("arguments").cloned().unwrap_or(json!({}));
                    match name {
                        "websearch" => {
                            let q = args["query"].as_str().unwrap_or("");
                            match s
                                .web_search
                                .search(WebSearchConfig {
                                    query: q.to_string(),
                                    max_results: 100,
                                    ..Default::default()
                                })
                                .await
                            {
                                Ok(r) => {
                                    json!({"content": [{"type": "text", "text": format!("Found {} results", r.len())}]})
                                }
                                Err(e) => {
                                    json!({"content": [{"type": "text", "text": format!("Error: {}", e)}], "isError": true})
                                }
                            }
                        }
                        "file_research" => {
                            let term = args["search_term"].as_str().unwrap_or("");
                            let path = args["path"].as_str().unwrap_or(".");
                            let config = FileSearchConfig {
                                search_term: term.to_string(),
                                root_dir: PathBuf::from(path),
                                include_patterns: vec!["*".into()],
                                exclude_patterns: vec![
                                    "target/".into(),
                                    "node_modules/".into(),
                                    ".git/".into(),
                                ],
                                exact_match: false,
                                use_regex: args["use_regex"].as_bool().unwrap_or(false),
                                search_in_content: true,
                                search_in_filename: true,
                                max_results: 1000,
                            };
                            match s.file_search.search(config).await {
                                Ok(r) => {
                                    json!({"content": [{"type": "text", "text": format!("Found {} matches in files", r.len())}]})
                                }
                                Err(e) => {
                                    json!({"content": [{"type": "text", "text": format!("Error: {}", e)}], "isError": true})
                                }
                            }
                        }
                        "stats" => {
                            json!({"content": [{"type": "text", "text": serde_json::to_string_pretty(&s.stats_system.get_all_stats()).unwrap_or_default()}]})
                        }
                        _ => {
                            json!({"content": [{"type": "text", "text": format!("Unknown: {}", name)}], "isError": true})
                        }
                    }
                }
                "ping" => json!({}),
                _ => {
                    return Json(
                        json!({"jsonrpc": "2.0", "error": {"code": -32601, "message": format!("Method not found: {}", method)}, "id": id}),
                    )
                }
            };
            Json(json!({"jsonrpc": "2.0", "result": result, "id": id}))
        }

        let app = Router::new()
            .route("/", get(|| async { "🔥 NUCLEAR ULTIMATE MCP 2025" }))
            .route("/health", get(health))
            .route("/tools", get(tools))
            .route("/call", post(call))
            .route("/mcp", post(mcp))
            .route("/mcp/initialize", post(mcp))
            .route("/mcp/tools/list", post(mcp))
            .route("/mcp/tools/call", post(mcp))
            .with_state(state);

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        eprintln!("   🚀 http://{}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    if std::env::var("RUST_LOG").is_ok() {
        eprintln!("═══════════════════════════════════════════════════════════════");
        eprintln!("   🔥 NUCLEAR ULTIMATE MCP 2025");
        eprintln!("   📋 Protocolo: {}", MCP_PROTOCOL_VERSION);
        eprintln!("   🛠️  5 Herramientas: websearch, analyzer, file_research, stats, orchestrate");
        eprintln!("═══════════════════════════════════════════════════════════════");
    }

    let nuclear = NuclearUltimate::new(&args.results_dir)?;
    match args.mode.as_str() {
        "studio" | "stdio" => nuclear.run_studio().await,
        "http" | "axum" => nuclear.run_http(args.port).await,
        _ => {
            eprintln!("Modo no reconocido: {}. Usa 'studio' o 'http'", args.mode);
            std::process::exit(1);
        }
    }
}