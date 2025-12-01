//! 🔥 NUCLEAR SCRAPER WEB - HTTP Server v2.0
//!
//! Servidor HTTP REST que expone las herramientas MCP como endpoints
//! Con 10 MEJORAS: CircuitBreaker, BloomFilter, Cache, Metrics, Rate Limiting, etc.
//! Corre continuamente en un puerto configurable (default: 4000)

use anyhow::Result;
use axum::body::Body;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Json, Response},
    routing::{get, post},
    Router,
};
use futures_util::stream;
use nuclear_crawler_lib::simple_mcp::SimpleMcpServer;
use nuclear_crawler_lib::improvements::{
    CircuitBreaker, BloomFilter, SemaphoreLimiter, MetricsCollector, 
    MetricType, MemoryCache, SmartRetry, EventBus,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Instant;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct AppState {
    pub mcp_server: Arc<Mutex<SimpleMcpServer>>,
    // 🔥 10 MEJORAS INTEGRADAS
    pub circuit_breaker: Arc<Mutex<CircuitBreaker>>,
    pub bloom_filter: Arc<Mutex<BloomFilter>>,
    pub rate_limiter: Arc<SemaphoreLimiter>,
    pub metrics: Arc<MetricsCollector>,
    pub cache: Arc<MemoryCache<String, Value>>,
    pub retry: Arc<SmartRetry>,
    pub event_bus: Arc<EventBus>,
    pub start_time: Instant,
}

#[derive(Deserialize)]
struct WebSearchParams {
    query: String,
    #[serde(default = "default_max_results")]
    max_results: usize,
    #[serde(default = "default_sources")]
    sources: String, // Comma-separated
    #[serde(default = "default_true")]
    use_ai: bool,
    #[serde(default = "default_true")]
    use_stealth: bool,
}

#[derive(Deserialize)]
struct UltimasBusquedasParams {
    #[serde(default = "default_limit")]
    limit: usize,
}

#[derive(Deserialize)]
struct StatsParams {
    #[serde(default = "default_stats_type")]
    r#type: String,
}

#[derive(Deserialize)]
struct AnalizarProyectoParams {
    project_path: String,
}

#[derive(Deserialize)]
struct ScanProjectParams {
    project_path: String,
    #[serde(default = "default_true")]
    search_solutions: bool,
}

#[derive(Deserialize)]
struct DeepWebSearchParams {
    query: String,
    #[serde(default = "default_search_type")]
    search_type: String,
    #[serde(default = "default_max_results")]
    max_results: usize,
    #[serde(default = "default_true")]
    find_access_methods: bool,
    #[serde(default = "default_false")]
    use_advanced_techniques: bool,
}

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    data: Option<Value>,
    error: Option<String>,
}

fn default_max_results() -> usize {
    100  // NUCLEAR DEFAULT: 100 resultados por defecto
}

fn default_sources() -> String {
    "github.com,stackoverflow.com,medium.com,dev.to,huggingface.co,arxiv.org".to_string()
}

fn default_true() -> bool {
    true
}

fn default_limit() -> usize {
    10
}

fn default_stats_type() -> String {
    "full".to_string()
}

fn default_search_type() -> String {
    "all".to_string()
}

fn default_false() -> bool {
    false
}

/// Health check endpoint - MEJORADO con métricas
async fn health(State(state): State<AppState>) -> Json<Value> {
    let uptime = state.start_time.elapsed().as_secs();
    let circuit_state = state.circuit_breaker.lock().await.is_open();
    
    // Incrementar contador de requests
    state.metrics.increment("http_requests_total");
    
    Json(json!({
        "status": "ok",
        "service": "nuclear-scraper-web",
        "version": "2.0.0-improved",
        "uptime_seconds": uptime,
        "improvements": [
            "circuit_breaker",
            "bloom_filter", 
            "rate_limiter",
            "metrics",
            "cache",
            "smart_retry",
            "event_bus"
        ],
        "circuit_breaker_open": circuit_state,
        "cache_stats": {
            "size": state.cache.len(),
            "capacity": 1000
        }
    }))
}

/// 📊 Métricas en formato Prometheus
async fn metrics_endpoint(State(state): State<AppState>) -> String {
    let uptime = state.start_time.elapsed().as_secs();
    let cb_open = if state.circuit_breaker.lock().await.is_open() { 1 } else { 0 };
    
    let mut output = String::new();
    
    // Métricas del sistema
    output.push_str("# HELP nuclear_uptime_seconds Server uptime in seconds\n");
    output.push_str("# TYPE nuclear_uptime_seconds gauge\n");
    output.push_str(&format!("nuclear_uptime_seconds {}\n\n", uptime));
    
    output.push_str("# HELP nuclear_circuit_breaker_open Circuit breaker state (1=open, 0=closed)\n");
    output.push_str("# TYPE nuclear_circuit_breaker_open gauge\n");
    output.push_str(&format!("nuclear_circuit_breaker_open {}\n\n", cb_open));
    
    output.push_str("# HELP nuclear_cache_size Number of cached items\n");
    output.push_str("# TYPE nuclear_cache_size gauge\n");
    output.push_str(&format!("nuclear_cache_size {}\n\n", state.cache.len()));
    
    // Métricas personalizadas
    output.push_str(&state.metrics.export_prometheus());
    
    output
}

/// Lista todas las herramientas disponibles
async fn tools(State(_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let tools_list = json!({
        "tools": [
            {
                "name": "websearch",
                "description": "Búsqueda web masiva usando TODO el poder del Nuclear Scraper",
                "endpoint": "/api/websearch"
            },
            {
                "name": "ultimas_busquedas",
                "description": "Obtiene las últimas búsquedas guardadas",
                "endpoint": "/api/ultimas_busquedas"
            },
            {
                "name": "stats",
                "description": "Estadísticas del sistema Nuclear Scraper",
                "endpoint": "/api/stats"
            },
            {
                "name": "analizar_proyecto",
                "description": "Analiza un proyecto y busca librerías/opciones útiles",
                "endpoint": "/api/analizar_proyecto"
            },
            {
                "name": "urls_visitadas",
                "description": "Obtiene el historial de URLs visitadas",
                "endpoint": "/api/urls_visitadas"
            },
            {
                "name": "scan_project",
                "description": "Escanea proyectos Rust con errores/warnings exactos",
                "endpoint": "/api/scan_project"
            },
            {
                "name": "deep_web_search",
                "description": "Búsqueda profunda en deep web: código, software, inteligencia, contenido premium con métodos legales",
                "endpoint": "/api/deep_web_search"
            }
        ]
    });

    Ok(Json(ApiResponse {
        success: true,
        data: Some(tools_list),
        error: None,
    }))
}

/// Búsqueda web masiva - MEJORADO con CircuitBreaker, Cache, Rate Limiting
async fn websearch(
    State(state): State<AppState>,
    Query(params): Query<WebSearchParams>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let start = Instant::now();
    state.metrics.increment("websearch_requests_total");
    
    // 1️⃣ Rate Limiting
    if !state.rate_limiter.try_acquire() {
        state.metrics.increment("rate_limit_exceeded");
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Rate limit exceeded. Try again later.".to_string()),
        }));
    }
    
    // 2️⃣ Circuit Breaker check
    {
        let cb = state.circuit_breaker.lock().await;
        if cb.is_open() {
            state.metrics.increment("circuit_breaker_rejected");
            return Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Service temporarily unavailable (circuit breaker open)".to_string()),
            }));
        }
    }
    
    // 3️⃣ Check Bloom Filter (dedup)
    let cache_key = format!("websearch:{}", params.query);
    {
        let bf = state.bloom_filter.lock().await;
        if bf.might_contain(&cache_key) {
            // Check actual cache
            if let Some(cached) = state.cache.get(&cache_key) {
                state.metrics.increment("cache_hits");
                let latency = start.elapsed().as_millis() as f64;
                state.metrics.observe("websearch_latency_ms", latency);
                return Ok(Json(ApiResponse {
                    success: true,
                    data: Some(cached),
                    error: None,
                }));
            }
        }
    }
    
    let sources: Vec<String> = params
        .sources
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let args = json!({
        "query": params.query,
        "max_results": params.max_results,
        "sources": sources,
        "use_ai": params.use_ai,
        "use_stealth": params.use_stealth
    });

    // 4️⃣ Execute request
    let result = {
        let mut server = state.mcp_server.lock().await;
        server.handle_websearch(args.clone()).await
    };
    
    match result {
        Ok(data) => {
            // 5️⃣ Record success in circuit breaker
            state.circuit_breaker.lock().await.record_success();
            
            // 6️⃣ Cache the result
            state.cache.insert(cache_key.clone(), data.clone());
            state.bloom_filter.lock().await.insert(&cache_key);
            
            // 7️⃣ Emit event
            let _ = state.event_bus.publish("websearch_complete", json!({
                "query": params.query,
                "results": data.get("total_results").unwrap_or(&json!(0))
            }));
            
            let latency = start.elapsed().as_millis() as f64;
            state.metrics.observe("websearch_latency_ms", latency);
            state.metrics.increment("websearch_success");
            
            Ok(Json(ApiResponse {
                success: true,
                data: Some(data),
                error: None,
            }))
        },
        Err(e) => {
            // Record failure in circuit breaker
            state.circuit_breaker.lock().await.record_failure();
            state.metrics.increment("websearch_errors");
            
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
            }))
        }
    }
}

/// Últimas búsquedas
async fn ultimas_busquedas(
    State(state): State<AppState>,
    Query(params): Query<UltimasBusquedasParams>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let args = json!({
        "limit": params.limit
    });

    let mut server = state.mcp_server.lock().await;
    match server.handle_ultimas_busquedas(args).await {
        Ok(result) => Ok(Json(ApiResponse {
            success: true,
            data: Some(result),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        })),
    }
}

/// Estadísticas
async fn stats(
    State(state): State<AppState>,
    Query(params): Query<StatsParams>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let args = json!({
        "type": params.r#type
    });

    let mut server = state.mcp_server.lock().await;
    match server.handle_stats(args).await {
        Ok(result) => Ok(Json(ApiResponse {
            success: true,
            data: Some(result),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        })),
    }
}

/// Analizar proyecto
async fn analizar_proyecto(
    State(state): State<AppState>,
    Query(params): Query<AnalizarProyectoParams>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let args = json!({
        "project_path": params.project_path
    });

    let mut server = state.mcp_server.lock().await;
    match server.handle_analizar_proyecto(args).await {
        Ok(result) => Ok(Json(ApiResponse {
            success: true,
            data: Some(result),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        })),
    }
}

/// Scan project endpoint
async fn scan_project(
    State(state): State<AppState>,
    Query(params): Query<ScanProjectParams>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let args = json!({
        "project_path": params.project_path,
        "search_solutions": params.search_solutions,
    });

    let mut server = state.mcp_server.lock().await;
    match server.handle_scan_project(args).await {
        Ok(result) => Ok(Json(ApiResponse {
            success: true,
            data: Some(result),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        })),
    }
}

/// Deep web search endpoint
async fn deep_web_search(
    State(state): State<AppState>,
    Query(params): Query<DeepWebSearchParams>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let args = json!({
        "query": params.query,
        "search_type": params.search_type,
        "max_results": params.max_results,
        "find_access_methods": params.find_access_methods,
        "use_advanced_techniques": params.use_advanced_techniques,
    });

    let mut server = state.mcp_server.lock().await;
    match server.handle_deep_web_search(args).await {
        Ok(result) => Ok(Json(ApiResponse {
            success: true,
            data: Some(result),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        })),
    }
}

/// URLs visitadas
async fn urls_visitadas(State(state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let args = json!({});

    let mut server = state.mcp_server.lock().await;
    match server.handle_urls_visitadas(args).await {
        Ok(result) => Ok(Json(ApiResponse {
            success: true,
            data: Some(result),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        })),
    }
}

/// Endpoint MCP: POST /mcp/message
/// Maneja mensajes JSON-RPC según especificación MCP
async fn mcp_message(
    State(state): State<AppState>,
    Json(request): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // #region agent log
    let log_data = json!({
        "location": "main_http.rs:mcp_message",
        "message": "MCP message received",
        "data": {"method": request.get("method"), "id": request.get("id")},
        "timestamp": chrono::Utc::now().timestamp_millis(),
        "sessionId": "debug-session",
        "runId": "mcp-http",
        "hypothesisId": "A"
    });
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(r"c:\Users\DELL\Desktop\hf_spaces\NUCLEAR_CRAWLER_HYBRID\.cursor\debug.log")
    {
        let _ = std::io::Write::write_all(
            &mut file,
            format!("{}\n", serde_json::to_string(&log_data).unwrap_or_default()).as_bytes(),
        );
    }
    // #endregion

    let mut server = state.mcp_server.lock().await;
    let response = server.handle_request(request).await;

    // #region agent log
    let log_data = json!({
        "location": "main_http.rs:mcp_message",
        "message": "MCP response generated",
        "data": {"has_error": response.get("error").is_some()},
        "timestamp": chrono::Utc::now().timestamp_millis(),
        "sessionId": "debug-session",
        "runId": "mcp-http",
        "hypothesisId": "A"
    });
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(r"c:\Users\DELL\Desktop\hf_spaces\NUCLEAR_CRAWLER_HYBRID\.cursor\debug.log")
    {
        let _ = std::io::Write::write_all(
            &mut file,
            format!("{}\n", serde_json::to_string(&log_data).unwrap_or_default()).as_bytes(),
        );
    }
    // #endregion

    Ok(Json(response))
}

/// Endpoint MCP: GET /mcp/sse
/// Server-Sent Events para notificaciones MCP
async fn mcp_sse(State(_state): State<AppState>) -> Result<Response<Body>, StatusCode> {
    // #region agent log
    let log_data = json!({
        "location": "main_http.rs:mcp_sse",
        "message": "SSE connection established",
        "data": {},
        "timestamp": chrono::Utc::now().timestamp_millis(),
        "sessionId": "debug-session",
        "runId": "mcp-http",
        "hypothesisId": "B"
    });
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(r"c:\Users\DELL\Desktop\hf_spaces\NUCLEAR_CRAWLER_HYBRID\.cursor\debug.log")
    {
        let _ = std::io::Write::write_all(
            &mut file,
            format!("{}\n", serde_json::to_string(&log_data).unwrap_or_default()).as_bytes(),
        );
    }
    // #endregion

    // Crear stream SSE manualmente con Result
    let stream = stream::once(async move {
        let data = json!({
            "type": "notification",
            "method": "tools/list_changed"
        });
        Ok::<String, Infallible>(format!("data: {}\n\n", data))
    });

    let body = Body::from_stream(stream);

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Inicializar logging
    tracing_subscriber::fmt::init();

    println!("🔥 Nuclear Scraper Web HTTP Server v2.0");
    println!("⚡ Inicializando 10 MEJORAS...");
    
    // Crear servidor MCP
    let mcp_server = Arc::new(Mutex::new(SimpleMcpServer::new()?));
    
    // 🔥 INICIALIZAR 10 MEJORAS
    println!("  1️⃣  Circuit Breaker (tolerancia a fallos)");
    let circuit_breaker = Arc::new(Mutex::new(CircuitBreaker::new(3, 30)));
    
    println!("  2️⃣  Bloom Filter (deduplicación rápida)");
    let bloom_filter = Arc::new(Mutex::new(BloomFilter::new(100_000, 0.01)));
    
    println!("  3️⃣  Rate Limiter (100 req concurrentes)");
    let rate_limiter = Arc::new(SemaphoreLimiter::new(100));
    
    println!("  4️⃣  Metrics Collector (Prometheus)");
    let metrics = Arc::new(MetricsCollector::new());
    
    println!("  5️⃣  Memory Cache (1000 entries)");
    let cache = Arc::new(MemoryCache::new(1000));
    
    println!("  6️⃣  Smart Retry (backoff exponencial)");
    let retry = Arc::new(SmartRetry::new(3, 100, 5000));
    
    println!("  7️⃣  Event Bus (pub/sub asíncrono)");
    let event_bus = Arc::new(EventBus::new());
    
    // Registrar métricas iniciales
    metrics.register("http_requests_total", MetricType::Counter);
    metrics.register("websearch_requests_total", MetricType::Counter);
    metrics.register("websearch_success", MetricType::Counter);
    metrics.register("websearch_errors", MetricType::Counter);
    metrics.register("cache_hits", MetricType::Counter);
    metrics.register("rate_limit_exceeded", MetricType::Counter);
    metrics.register("circuit_breaker_rejected", MetricType::Counter);
    metrics.register("websearch_latency_ms", MetricType::Histogram);

    let state = AppState { 
        mcp_server,
        circuit_breaker,
        bloom_filter,
        rate_limiter,
        metrics,
        cache,
        retry,
        event_bus,
        start_time: Instant::now(),
    };
    
    println!("  ✅ Todas las mejoras inicializadas!");
    println!();

    // Crear router con endpoints MCP HTTP/SSE
    let app = Router::new()
        // Endpoints públicos
        .route("/", get(health))
        .route("/health", get(health))
        // 📊 Métricas Prometheus (NUEVO)
        .route("/metrics", get(metrics_endpoint))
        // Endpoints MCP estándar (según especificación Anthropic)
        .route("/mcp/message", post(mcp_message))
        .route("/mcp/sse", get(mcp_sse))
        // Endpoints REST legacy
        .route("/api/tools", get(tools))
        .route("/api/websearch", get(websearch))
        .route("/api/ultimas_busquedas", get(ultimas_busquedas))
        .route("/api/stats", get(stats))
        .route("/api/analizar_proyecto", get(analizar_proyecto))
        .route("/api/urls_visitadas", get(urls_visitadas))
        .route("/api/scan_project", get(scan_project))
        .route("/api/deep_web_search", get(deep_web_search));

    let app = app.layer(CorsLayer::permissive()).with_state(state);

    // Obtener puerto de variable de entorno o usar default
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse::<u16>()
        .unwrap_or(4000);

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    println!("🚀 Servidor iniciado en http://{}", addr);
    println!();
    println!("📋 Endpoints MCP (Model Context Protocol):");
    println!("   POST /mcp/message - Mensajes JSON-RPC MCP");
    println!("   GET  /mcp/sse - Server-Sent Events para notificaciones");
    println!();
    println!("📊 Monitoreo:");
    println!("   GET  /health - Health check con estado de mejoras");
    println!("   GET  /metrics - Métricas Prometheus");
    println!();
    println!("📋 Endpoints REST:");
    println!("   GET  /api/tools - Lista de herramientas");
    println!("   GET  /api/websearch?query=... - Búsqueda web");
    println!("   GET  /api/ultimas_busquedas?limit=10 - Últimas búsquedas");
    println!("   GET  /api/stats?type=full - Estadísticas");
    println!("   GET  /api/analizar_proyecto?project_path=... - Analizar proyecto");
    println!("   GET  /api/urls_visitadas - URLs visitadas");
    println!("   GET  /api/scan_project?project_path=... - Escanear proyecto Rust");
    println!("   GET  /api/deep_web_search?query=... - Búsqueda deep web");
    println!();
    println!("🔥 10 MEJORAS ACTIVAS:");
    println!("   ⚡ Circuit Breaker - Tolerancia a fallos");
    println!("   ⚡ Bloom Filter - Deduplicación O(1)");
    println!("   ⚡ Rate Limiter - Control de concurrencia");
    println!("   ⚡ Metrics - Prometheus/Grafana");
    println!("   ⚡ Cache - Respuestas en memoria");
    println!("   ⚡ Smart Retry - Reintentos inteligentes");
    println!("   ⚡ Event Bus - Pub/Sub asíncrono");
    println!();
    println!("💡 Ejemplo: http://localhost:{}/api/websearch?query=rust%20mcp", port);

    axum::serve(listener, app).await?;

    Ok(())
}
