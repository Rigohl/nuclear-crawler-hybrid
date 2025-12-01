//! 🔥 MCP Axum Server - MCP 2025 Protocol Implementation
//!
//! Modern HTTP/WebSocket server for Model Context Protocol
//! Following MCP 2025 specification

use crate::simple_mcp::SimpleMcpServer;
use anyhow::Result;
use axum::{
    extract::{Json, State, WebSocketUpgrade},
    http::{header, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

/// MCP Axum Server State
#[derive(Clone)]
pub struct McpServerState {
    mcp: Arc<RwLock<SimpleMcpServer>>,
    config: Arc<McpServerConfig>,
}

/// Server Configuration
#[derive(Clone)]
pub struct McpServerConfig {
    pub host: String,
    pub port: u16,
    pub enable_cors: bool,
    pub max_request_size: usize,
}

impl Default for McpServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            enable_cors: true,
            max_request_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

impl McpServerState {
    pub fn new(config: McpServerConfig) -> Result<Self> {
        let mcp = SimpleMcpServer::new()?;
        Ok(Self {
            mcp: Arc::new(RwLock::new(mcp)),
            config: Arc::new(config),
        })
    }
}

/// Create Axum router with all MCP endpoints
pub fn create_router(state: McpServerState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    Router::new()
        // Health check
        .route("/health", get(health_check))
        // MCP 2025 endpoints
        .route("/mcp/initialize", post(mcp_initialize))
        .route("/mcp/tools/list", post(mcp_tools_list))
        .route("/mcp/tools/call", post(mcp_tools_call))
        // WebSocket for bidirectional communication (MCP 2025)
        .route("/mcp/ws", get(mcp_websocket))
        // Legacy JSON-RPC endpoint (compatibility)
        .route("/mcp/jsonrpc", post(mcp_jsonrpc))
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "nuclear-mcp",
        "version": env!("CARGO_PKG_VERSION"),
        "protocol": "MCP 2025"
    }))
}

/// MCP Initialize endpoint
async fn mcp_initialize(
    State(_state): State<McpServerState>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, McpError> {
    // Validate protocol version
    let protocol_version = payload["protocolVersion"]
        .as_str()
        .ok_or_else(|| McpError::InvalidRequest("Missing protocolVersion".to_string()))?;

    // Support both 2024-11-05 and 2025-06-18
    let supported_versions = ["2024-11-05", "2025-06-18"];
    if !supported_versions.contains(&protocol_version) {
        return Err(McpError::InvalidRequest(format!(
            "Unsupported protocol version: {}. Supported: {:?}",
            protocol_version, supported_versions
        )));
    }

    Ok(Json(json!({
        "protocolVersion": protocol_version,
        "capabilities": {
            "tools": {
                "listChanged": true
            },
            "resources": {},
            "prompts": {},
            "sampling": {},
            "websocket": true
        },
        "serverInfo": {
            "name": "nuclear-scraper-web",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Enterprise Web Scraping & Search Engine with MCP 2025"
        }
    })))
}

/// MCP Tools List endpoint
async fn mcp_tools_list(
    State(_state): State<McpServerState>,
) -> Result<Json<Value>, McpError> {
    // Get tools from SimpleMcpServer
    let tools = get_mcp_tools();

    Ok(Json(json!({
        "tools": tools
    })))
}

/// MCP Tools Call endpoint
async fn mcp_tools_call(
    State(state): State<McpServerState>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, McpError> {
    let tool_name = payload["name"]
        .as_str()
        .ok_or_else(|| McpError::InvalidRequest("Missing tool name".to_string()))?;

    let args = payload["arguments"].clone();

    // Execute tool
    let mut mcp = state.mcp.write().await;
    let result = mcp
        .handle_tool_call(tool_name, args)
        .await
        .map_err(|e| McpError::ToolError(e.to_string()))?;

    Ok(Json(json!({
        "content": [
            {
                "type": "text",
                "text": serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
            }
        ]
    })))
}

/// MCP WebSocket endpoint (MCP 2025)
async fn mcp_websocket(
    ws: WebSocketUpgrade,
    State(state): State<McpServerState>,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

/// Handle WebSocket connection
async fn handle_websocket(
    mut socket: axum::extract::ws::WebSocket,
    state: McpServerState,
) {
    use axum::extract::ws::Message;

    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                // Parse JSON-RPC request
                match serde_json::from_str::<Value>(&text) {
                    Ok(request) => {
                        let mut mcp = state.mcp.write().await;
                        let response = mcp.handle_request(request).await;
                        
                        if let Ok(response_text) = serde_json::to_string(&response) {
                            if socket.send(Message::Text(response_text)).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(_e) => {
                        let error_response = json!({
                            "jsonrpc": "2.0",
                            "error": {
                                "code": -32700,
                                "message": "Parse error"
                            }
                        });
                        if let Ok(error_text) = serde_json::to_string(&error_response) {
                            let _ = socket.send(Message::Text(error_text)).await;
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => {
                break;
            }
            Err(_e) => {
                break;
            }
            _ => {}
        }
    }
}

/// Legacy JSON-RPC endpoint for compatibility
async fn mcp_jsonrpc(
    State(state): State<McpServerState>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, McpError> {
    let mut mcp = state.mcp.write().await;
    let response = mcp.handle_request(payload).await;
    Ok(Json(response))
}

/// Get MCP tools definition
fn get_mcp_tools() -> Vec<Value> {
    vec![
        json!({
            "name": "websearch",
            "description": "Búsqueda web masiva usando TODO el poder del Nuclear Crawler",
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
                        "description": "Fuentes a buscar",
                        "default": ["github.com", "stackoverflow.com", "dev.to"]
                    },
                    "max_results": {
                        "type": "integer",
                        "description": "Máximo número de resultados",
                        "default": 0
                    }
                },
                "required": ["query"]
            }
        }),
        json!({
            "name": "deep_web_search",
            "description": "Búsqueda profunda en deep web con métodos legales de acceso",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Término de búsqueda"
                    },
                    "search_type": {
                        "type": "string",
                        "enum": ["code", "intelligence", "premium", "all"],
                        "default": "all"
                    },
                    "max_results": {
                        "type": "integer",
                        "default": 20
                    }
                },
                "required": ["query"]
            }
        }),
        json!({
            "name": "stats",
            "description": "Métricas y estadísticas del sistema Nuclear",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "type": {
                        "type": "string",
                        "enum": ["full", "recent", "performance", "storage"],
                        "default": "full"
                    }
                }
            }
        }),
        json!({
            "name": "analizar_proyecto",
            "description": "Analiza un proyecto y busca librerías/opciones relevantes",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Ruta del proyecto",
                        "default": "."
                    },
                    "max_recommendations": {
                        "type": "integer",
                        "default": 5
                    }
                },
                "required": ["path"]
            }
        }),
        json!({
            "name": "scan_project",
            "description": "Escanea proyecto Rust mostrando errores/warnings con soluciones",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_path": {
                        "type": "string",
                        "default": "."
                    },
                    "search_solutions": {
                        "type": "boolean",
                        "default": true
                    }
                },
                "required": ["project_path"]
            }
        }),
        json!({
            "name": "ultimas_busquedas",
            "description": "Obtiene las últimas búsquedas guardadas en el historial",
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
        }),
        json!({
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
        }),
    ]
}

/// MCP Error types
#[derive(Debug)]
pub enum McpError {
    InvalidRequest(String),
    ToolError(String),
    InternalError(String),
}

impl IntoResponse for McpError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            McpError::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, -32600, msg),
            McpError::ToolError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, -32000, msg),
            McpError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, -32603, msg),
        };

        let body = Json(json!({
            "jsonrpc": "2.0",
            "error": {
                "code": code,
                "message": message
            }
        }));

        (status, body).into_response()
    }
}

/// Start the MCP Axum server
pub async fn start_server(config: McpServerConfig) -> Result<()> {
    let addr = format!("{}:{}", config.host, config.port);
    eprintln!("🔥 Starting Nuclear MCP Server on {}", addr);
    eprintln!("📡 Protocol: MCP 2025");

    let state = McpServerState::new(config)?;
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    eprintln!("✅ Server listening on http://{}", addr);
    eprintln!("📚 Endpoints:");
    eprintln!("   - POST /mcp/initialize");
    eprintln!("   - POST /mcp/tools/list");
    eprintln!("   - POST /mcp/tools/call");
    eprintln!("   - GET  /mcp/ws (WebSocket)");
    eprintln!("   - GET  /health");

    axum::serve(listener, app).await?;

    Ok(())
}
