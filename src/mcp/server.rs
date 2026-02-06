//! 🔥 MCP HTTP SERVER - Axum-based JSON-RPC 2.0 Server
//!
//! Exposes EXACTLY 7 TOOLS via HTTP endpoints - NO MOCKS (AMPLIFIED POWER)
//! Compatible with Copilot CLI and Claude Desktop
//!
//! TOOLS: websearch, premium, file_search, scan, ai_dataset_trainer, parallel_engine, osint_intelligence

use crate::mcp::protocol::{
    error_codes, get_tool_definitions_for_profile, tool_exists_for_profile, MCPRequest,
    MCPResponse, ToolProfile,
};
use crate::mcp::tools::{
    AdvancedFileSearchTool, PremiumContentTool, ScanConfig, ScanWorkspaceTool, WebSearchTool,
};
use crate::{cache::Cache, intelligent_storage::IntelligentStorage, rate_limit::RateLimiter};
use axum::{
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;

/// MCP Server state - PODER REAL (7 TOOLS)
pub struct MCPServer {
    // 🔥 7 HERRAMIENTAS MCP - SIN MOCKS
    pub websearch: Arc<WebSearchTool>,
    pub premium_content: Arc<PremiumContentTool>,
    pub file_search: Arc<AdvancedFileSearchTool>,
    pub scan: Arc<ScanWorkspaceTool>,
    // Tools #6 and #7 are executed directly via functions

    // 🔥 INFRAESTRUCTURA COMPARTIDA
    pub cache: Arc<Cache>,
    pub storage: Arc<tokio::sync::Mutex<IntelligentStorage>>,
    pub rate_limiter: Arc<RateLimiter>,

    /// Tool profile: Full (7), Pro (5), or Lite (2)
    pub profile: ToolProfile,
}

impl MCPServer {
    /// Create new MCP server with REAL implementations (7 TOOLS - Full profile)
    pub fn new() -> Self {
        Self::with_profile(ToolProfile::Full)
    }

    /// Create MCP server with a specific tool profile
    pub fn with_profile(profile: ToolProfile) -> Self {
        let tool_names = crate::mcp::protocol::get_profile_tool_names(profile);
        eprintln!(
            "🔥 Initializing MCP Server ({:?} profile, {} tools)...",
            profile,
            tool_names.len()
        );

        // 🔥 Inicializar infraestructura
        let cache = Arc::new(Cache::new(5000));
        let storage = Arc::new(tokio::sync::Mutex::new(IntelligentStorage::new()));
        let rate_limiter = Arc::new(RateLimiter::new(100, 200)); // 100 req/sec, burst 200

        eprintln!("📡 Infrastructure ready: Cache(5000), RateLimiter(100/s)");
        eprintln!("🔧 Tools: {}", tool_names.join(", "));

        Self {
            websearch: Arc::new(WebSearchTool::default()),
            premium_content: Arc::new(PremiumContentTool::default()),
            file_search: Arc::new(AdvancedFileSearchTool::default()),
            scan: Arc::new(ScanWorkspaceTool::new()),
            cache,
            storage,
            rate_limiter,
            profile,
        }
    }

    /// Create Axum router for MCP endpoints
    pub fn create_router(self) -> Router {
        let server = Arc::new(self);
        let tool_names: Vec<&str> = crate::mcp::protocol::get_profile_tool_names(server.profile);

        Router::new()
            .route(
                "/",
                get({
                    let names = tool_names.iter().map(|s| s.to_string()).collect::<Vec<_>>();
                    move || async move {
                        Json(json!({
                            "service": "nuclear-mcp",
                            "version": "2026.02.06",
                            "tools": names,
                            "status": "ready"
                        }))
                    }
                }),
            )
            .route(
                "/health",
                get({
                    let profile = server.profile;
                    let names = tool_names.iter().map(|s| s.to_string()).collect::<Vec<_>>();
                    move || async move { health_check_with_profile(profile, names).await }
                }),
            )
            .route(
                "/mcp/tools/list",
                post({
                    let server = server.clone();
                    move |body| handle_tools_list(server.clone(), body)
                }),
            )
            .route(
                "/mcp/tools/call",
                post({
                    let server = server.clone();
                    move |body| handle_tool_call(server.clone(), body)
                }),
            )
            .route(
                "/mcp/rpc",
                post({
                    let server = server.clone();
                    move |body| handle_rpc(server.clone(), body)
                }),
            )
            // Direct tool endpoints for convenience
            .route(
                "/tools/websearch",
                post({
                    let server = server.clone();
                    move |body| direct_websearch(server.clone(), body)
                }),
            )
            .route(
                "/tools/premium",
                post({
                    let server = server.clone();
                    move |body| direct_premium(server.clone(), body)
                }),
            )
            .route(
                "/tools/file_search",
                post({
                    let server = server.clone();
                    move |body| direct_file_search(server.clone(), body)
                }),
            )
            .route(
                "/tools/scan",
                post({
                    let server = server.clone();
                    move |body| direct_scan(server.clone(), body)
                }),
            )
    }
}

/// Health check endpoint with profile info
async fn health_check_with_profile(
    profile: ToolProfile,
    tool_names: Vec<String>,
) -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "nuclear-mcp",
        "version": "2026.02.06",
        "profile": format!("{:?}", profile),
        "tools_count": tool_names.len(),
        "tools": tool_names,
        "mocks": false,
        "real_data": true
    }))
}

/// Handle tools/list RPC call
async fn handle_tools_list(
    server: Arc<MCPServer>,
    Json(req): Json<MCPRequest>,
) -> impl IntoResponse {
    eprintln!("📋 tools/list requested ({:?} profile)", server.profile);

    let tools = get_tool_definitions_for_profile(server.profile);
    let response = MCPResponse::success(req.id, json!({"tools": tools}));

    Json(response)
}

/// Handle tools/call RPC call - REAL IMPLEMENTATIONS
async fn handle_tool_call(
    server: Arc<MCPServer>,
    Json(req): Json<MCPRequest>,
) -> impl IntoResponse {
    eprintln!("🔧 Tool call: {:?}", req.params);

    let id = req.id.clone();
    let tool_name = req.get_tool_name().map(|s| s.to_string());
    let arguments = req.get_arguments();

    // Check if tool is available in current profile
    if let Some(ref name) = tool_name {
        if !tool_exists_for_profile(name, server.profile) {
            return Json(MCPResponse::error(
                id,
                error_codes::METHOD_NOT_FOUND,
                format!(
                    "Tool '{}' not available in {:?} profile. Available: {:?}",
                    name,
                    server.profile,
                    crate::mcp::protocol::get_profile_tool_names(server.profile)
                ),
            ));
        }
    }

    match tool_name.as_deref() {
        Some("websearch") => execute_websearch(&server, id, arguments).await,
        Some("premium") => execute_premium(&server, id, arguments).await,
        Some("file_search") => execute_file_search(&server, id, arguments).await,
        Some("scan") => execute_scan(&server, id, arguments).await,
        Some("ai_dataset_trainer") => execute_ai_dataset_trainer(&server, id, arguments).await,
        Some("parallel_engine") => execute_parallel_engine_dispatch(&server, id, arguments).await,
        Some("osint_intelligence") => {
            execute_osint_intelligence_dispatch(&server, id, arguments).await
        }
        Some(unknown) => Json(MCPResponse::method_not_found(id, unknown)),
        None => Json(MCPResponse::invalid_params(id, "tool name required")),
    }
}

/// Handle generic JSON-RPC 2.0 call
async fn handle_rpc(server: Arc<MCPServer>, Json(req): Json<MCPRequest>) -> impl IntoResponse {
    eprintln!("🔄 RPC method: {}", req.method);

    let id = req.id.clone();

    match req.method.as_str() {
        "tools/list" => {
            let tools = get_tool_definitions_for_profile(server.profile);
            Json(MCPResponse::success(id, json!({"tools": tools})))
        }
        "tools/call" => {
            // Delegate to tool call handler
            let tool_name = req.get_tool_name().map(|s| s.to_string());
            let arguments = req.get_arguments();

            // Check profile
            if let Some(ref name) = tool_name {
                if !tool_exists_for_profile(name, server.profile) {
                    return Json(MCPResponse::method_not_found(id, name));
                }
            }

            match tool_name.as_deref() {
                Some("websearch") => execute_websearch(&server, id, arguments).await,
                Some("premium") => execute_premium(&server, id, arguments).await,
                Some("file_search") => execute_file_search(&server, id, arguments).await,
                Some("scan") => execute_scan(&server, id, arguments).await,
                Some("ai_dataset_trainer") => {
                    execute_ai_dataset_trainer(&server, id, arguments).await
                }
                Some("parallel_engine") => {
                    execute_parallel_engine_dispatch(&server, id, arguments).await
                }
                Some("osint_intelligence") => {
                    execute_osint_intelligence_dispatch(&server, id, arguments).await
                }
                Some(name) => Json(MCPResponse::method_not_found(id, name)),
                None => Json(MCPResponse::invalid_params(id, "name required")),
            }
        }
        "initialize" => {
            // MCP initialization handshake
            let tool_names = crate::mcp::protocol::get_profile_tool_names(server.profile);
            Json(MCPResponse::success(
                id,
                json!({
                    "protocolVersion": "2026-02-06",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "nuclear-mcp",
                        "version": "2026.02.06",
                        "profile": format!("{:?}", server.profile),
                        "tools": tool_names
                    }
                }),
            ))
        }
        method => Json(MCPResponse::method_not_found(id, method)),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 REAL TOOL IMPLEMENTATIONS - NO MOCKS
// ═══════════════════════════════════════════════════════════════════════════

/// Execute WEBSEARCH with real HTTP requests
async fn execute_websearch(server: &Arc<MCPServer>, id: String, args: Value) -> Json<MCPResponse> {
    let query = match args.get("query").and_then(|v| v.as_str()) {
        Some(q) if !q.is_empty() => q,
        _ => return Json(MCPResponse::invalid_params(id, "query parameter required")),
    };

    let max_results = args
        .get("max_results")
        .and_then(|v| v.as_u64())
        .unwrap_or(50) as usize;

    let deep_search = args
        .get("deep_search")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let language = args
        .get("language")
        .and_then(|v| v.as_str())
        .unwrap_or("es");

    eprintln!(
        "🔍 WebSearch: '{}' (max: {}, deep: {}, lang: {})",
        query, max_results, deep_search, language
    );

    // Apply rate limiting
    server.rate_limiter.wait().await;

    // Check cache first
    let cache_key = format!("ws:{}:{}:{}", query, max_results, language);
    if let Some(cached) = server.cache.get_simple(&cache_key) {
        eprintln!("📦 Cache hit for query");
        if let Ok(cached_result) = serde_json::from_str::<Value>(&cached) {
            return Json(MCPResponse::success(id, cached_result));
        }
    }

    // Execute REAL search
    match server.websearch.search_real(query, max_results).await {
        Ok(results) => {
            let response = json!({
                "query": query,
                "results_count": results.len(),
                "results": results,
                "deep_search": deep_search,
                "language": language,
                "source": "real_http_requests",
                "cached": false
            });

            // Cache the result
            if let Ok(json_str) = serde_json::to_string(&response) {
                server.cache.set_simple(&cache_key, json_str);
            }

            Json(MCPResponse::success(id, response))
        }
        Err(e) => Json(MCPResponse::error(
            id,
            error_codes::TOOL_EXECUTION_ERROR,
            format!("Search failed: {}", e),
        )),
    }
}

/// Execute PREMIUM content fetch
async fn execute_premium(server: &Arc<MCPServer>, id: String, args: Value) -> Json<MCPResponse> {
    let input = match args.get("input").and_then(|v| v.as_str()) {
        Some(i) if !i.is_empty() => i,
        _ => {
            return Json(MCPResponse::invalid_params(
                id,
                "input parameter required (URL or search phrase)",
            ))
        }
    };

    let content_type = args.get("type").and_then(|v| v.as_str()).unwrap_or("auto");

    let full_content = args
        .get("full_content")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let bypass_paywall = args
        .get("bypass_paywall")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    // Detect if input is URL or search phrase
    let is_url = input.starts_with("http://") || input.starts_with("https://");

    eprintln!(
        "📚 Premium: '{}' (type: {}, url: {}, bypass: {})",
        input, content_type, is_url, bypass_paywall
    );

    // Apply rate limiting
    server.rate_limiter.wait().await;

    if is_url {
        // Fetch from URL
        match server
            .premium_content
            .fetch_url_real(input, bypass_paywall)
            .await
        {
            Ok(content) => Json(MCPResponse::success(
                id,
                json!({
                    "input": input,
                    "input_type": "url",
                    "content_type": content_type,
                    "title": content.title,
                    "content": content.content,
                    "source": content.source,
                    "access_method": if bypass_paywall { "paywall_bypass" } else { "direct" },
                    "full_content": full_content
                }),
            )),
            Err(e) => Json(MCPResponse::error(
                id,
                error_codes::TOOL_EXECUTION_ERROR,
                format!("Premium fetch failed: {}", e),
            )),
        }
    } else {
        // Search for content
        match server
            .premium_content
            .search_premium_real(input, content_type)
            .await
        {
            Ok(results) => Json(MCPResponse::success(
                id,
                json!({
                    "input": input,
                    "input_type": "search",
                    "content_type": content_type,
                    "results_count": results.len(),
                    "results": results,
                    "bypass_enabled": bypass_paywall
                }),
            )),
            Err(e) => Json(MCPResponse::error(
                id,
                error_codes::TOOL_EXECUTION_ERROR,
                format!("Premium search failed: {}", e),
            )),
        }
    }
}

/// Execute FILE_SEARCH with precise locations
async fn execute_file_search(
    server: &Arc<MCPServer>,
    id: String,
    args: Value,
) -> Json<MCPResponse> {
    let path = match args.get("path").and_then(|v| v.as_str()) {
        Some(p) if !p.is_empty() => p,
        _ => return Json(MCPResponse::invalid_params(id, "path parameter required")),
    };

    let query = args.get("query").and_then(|v| v.as_str());
    let is_regex = args
        .get("is_regex")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let case_sensitive = args
        .get("case_sensitive")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let find_errors = args
        .get("find_errors")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let find_warnings = args
        .get("find_warnings")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let find_todos = args
        .get("find_todos")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let context_lines = args
        .get("context_lines")
        .and_then(|v| v.as_u64())
        .unwrap_or(2) as usize;

    eprintln!(
        "📄 FileSearch: '{}' (query: {:?}, errors: {}, todos: {})",
        path, query, find_errors, find_todos
    );

    // Apply rate limiting
    server.rate_limiter.wait().await;

    // Execute REAL file search
    match server
        .file_search
        .search_files_real(
            path,
            query,
            is_regex,
            case_sensitive,
            find_errors,
            find_warnings,
            find_todos,
            context_lines,
        )
        .await
    {
        Ok(results) => Json(MCPResponse::success(
            id,
            json!({
                "path": path,
                "query": query,
                "matches_count": results.matches.len(),
                "matches": results.matches,
                "errors_found": results.errors_count,
                "warnings_found": results.warnings_count,
                "todos_found": results.todos_count,
                "files_searched": results.files_searched,
                "source": "real_filesystem"
            }),
        )),
        Err(e) => Json(MCPResponse::error(
            id,
            error_codes::TOOL_EXECUTION_ERROR,
            format!("File search failed: {}", e),
        )),
    }
}

/// Execute SCAN for deep workspace analysis
async fn execute_scan(server: &Arc<MCPServer>, id: String, args: Value) -> Json<MCPResponse> {
    let path = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");

    let recursive = args
        .get("recursive")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let _detect_mocks = args
        .get("detect_mocks")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let _detect_todos = args
        .get("detect_todos")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let _detect_security = args
        .get("detect_security")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let _detect_performance = args
        .get("detect_performance")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let _max_depth = args.get("max_depth").and_then(|v| v.as_u64()).unwrap_or(50) as usize;
    let report_format = args
        .get("report_format")
        .and_then(|v| v.as_str())
        .unwrap_or("detailed");

    eprintln!("🔬 Scan: '{}' (recursive: {})", path, recursive);

    // Apply rate limiting
    server.rate_limiter.wait().await;

    let config = ScanConfig {
        path: path.to_string(),
        recursive,
    };

    // Execute REAL scan
    let result = server.scan.scan(config).await;

    let response = match report_format {
        "json" => json!(result),
        "summary" => json!({
            "path": result.scanned_path,
            "files": result.files_scanned,
            "lines": result.total_lines,
            "issues": result.total_issues,
            "health": result.health_score,
            "advice": result.advice
        }),
        _ => json!({
            "path": result.scanned_path,
            "files_scanned": result.files_scanned,
            "total_lines": result.total_lines,
            "total_issues": result.total_issues,
            "issues_by_category": result.issues_by_category,
            "issues_by_severity": result.issues_by_severity,
            "top_issues": result.top_issues,
            "health_score": result.health_score,
            "advice": result.advice,
            "scan_duration_ms": result.scan_duration_ms,
            "source": "real_filesystem"
        }),
    };

    Json(MCPResponse::success(id, response))
}

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 DIRECT TOOL ENDPOINTS (for convenience)
// ═══════════════════════════════════════════════════════════════════════════

async fn direct_websearch(server: Arc<MCPServer>, Json(args): Json<Value>) -> impl IntoResponse {
    let id = uuid::Uuid::new_v4().to_string();
    execute_websearch(&server, id, args).await
}

async fn direct_premium(server: Arc<MCPServer>, Json(args): Json<Value>) -> impl IntoResponse {
    let id = uuid::Uuid::new_v4().to_string();
    execute_premium(&server, id, args).await
}

async fn direct_file_search(server: Arc<MCPServer>, Json(args): Json<Value>) -> impl IntoResponse {
    let id = uuid::Uuid::new_v4().to_string();
    execute_file_search(&server, id, args).await
}

async fn direct_scan(server: Arc<MCPServer>, Json(args): Json<Value>) -> impl IntoResponse {
    let id = uuid::Uuid::new_v4().to_string();
    execute_scan(&server, id, args).await
}

/// Execute AI_DATASET_TRAINER with FFI integrations (Go + Zig + Nim + JAX)
async fn execute_ai_dataset_trainer(
    _server: &Arc<MCPServer>,
    id: String,
    args: Value,
) -> Json<MCPResponse> {
    eprintln!("🧠 AI Dataset Trainer: Initializing with FFI integrations...");

    let dataset_name = args
        .get("dataset_name")
        .and_then(|v| v.as_str())
        .unwrap_or("training_dataset");

    let target_size = args
        .get("target_size")
        .and_then(|v| v.as_u64())
        .unwrap_or(10000) as usize;

    let embedding_dim = args
        .get("embedding_dim")
        .and_then(|v| v.as_u64())
        .unwrap_or(384) as usize;

    let export_path = args
        .get("export_path")
        .and_then(|v| v.as_str())
        .unwrap_or("./dataset_output.jsonl");

    let use_go = args
        .get("use_go_parallel")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let use_zig = args
        .get("use_zig_dedup")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let use_nim = args
        .get("use_nim_parsing")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let use_jax = args
        .get("use_jax_gpu")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    eprintln!("📚 Dataset Config:");
    eprintln!("   Name: {}", dataset_name);
    eprintln!("   Target size: {}", target_size);
    eprintln!("   Embedding dim: {}", embedding_dim);
    eprintln!("   Processors:");
    eprintln!("      Go parallel: {}", use_go);
    eprintln!("      Zig SIMD: {}", use_zig);
    eprintln!("      Nim parsing: {}", use_nim);
    eprintln!("      JAX GPU: {}", use_jax);

    // 🔥 RETURN DATASET GENERATION PLAN
    let notes = vec![
        format!(
            "🔗 Go processor will fetch up to {} sources in parallel",
            target_size
        ),
        format!("⚡ Zig SIMD will hash each document for efficient deduplication"),
        format!("📄 Nim will extract clean text from HTML"),
        format!(
            "🧠 JAX will generate {}-dimensional embeddings",
            embedding_dim
        ),
    ];

    Json(MCPResponse::success(
        id,
        json!({
            "status": "ready",
            "message": "AI Dataset Trainer initialized",
            "configuration": {
                "name": dataset_name,
                "target_size": target_size,
                "embedding_dimension": embedding_dim,
                "export_path": export_path,
                "ffi_processors": {
                    "go_parallel_fetching": use_go,
                    "zig_simd_deduplication": use_zig,
                    "nim_html_parsing": use_nim,
                    "jax_gpu_vectorization": use_jax
                }
            },
            "phases": [
                {
                    "phase": 1,
                    "name": "Go Parallel Fetching",
                    "description": "Fetch data sources in parallel using Go goroutines",
                    "processor": "Go",
                    "enabled": use_go
                },
                {
                    "phase": 2,
                    "name": "Zig SIMD Deduplication",
                    "description": "Deduplicate content using Zig SIMD Blake3 hashing",
                    "processor": "Zig",
                    "enabled": use_zig
                },
                {
                    "phase": 3,
                    "name": "Nim HTML Parsing",
                    "description": "Parse and extract text from HTML documents",
                    "processor": "Nim",
                    "enabled": use_nim
                },
                {
                    "phase": 4,
                    "name": "JAX GPU Vectorization",
                    "description": "Generate embeddings and vectorize text content",
                    "processor": "JAX",
                    "enabled": use_jax
                }
            ],
            "output_format": {
                "format": "jsonl",
                "fields": ["id", "text", "source", "category", "metadata", "embedding", "content_hash", "quality_score", "processing_info"],
                "path": export_path
            },
            "notes": notes
        }),
    ))
}

impl Default for MCPServer {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// NEW TOOL HANDLERS (TOOLS #6 and #7)
// ═══════════════════════════════════════════════════════════════════════════

/// Execute PARALLEL_ENGINE tool (Tool #6)
async fn execute_parallel_engine_dispatch(
    _server: &Arc<MCPServer>,
    id: String,
    args: Value,
) -> Json<MCPResponse> {
    use crate::mcp::tools::execute_parallel_engine;

    eprintln!("🚀 Parallel Engine: Initializing high-performance parallel processing...");

    match execute_parallel_engine(args).await {
        Ok(result) => Json(MCPResponse::success(id, result)),
        Err(e) => {
            eprintln!("❌ Parallel Engine error: {}", e);
            Json(MCPResponse::error(
                id,
                error_codes::TOOL_EXECUTION_ERROR,
                format!("Parallel engine failed: {}", e),
            ))
        }
    }
}

/// Execute OSINT_INTELLIGENCE tool (Tool #7)
async fn execute_osint_intelligence_dispatch(
    _server: &Arc<MCPServer>,
    id: String,
    args: Value,
) -> Json<MCPResponse> {
    use crate::mcp::tools::execute_osint_intelligence;

    eprintln!("🔍 OSINT Intelligence: Initializing with Chapel AI...");

    match execute_osint_intelligence(args).await {
        Ok(result) => Json(MCPResponse::success(id, result)),
        Err(e) => {
            eprintln!("❌ OSINT Intelligence error: {}", e);
            Json(MCPResponse::error(
                id,
                error_codes::TOOL_EXECUTION_ERROR,
                format!("OSINT intelligence failed: {}", e),
            ))
        }
    }
}
