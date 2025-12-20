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
use tower_http::limit::RequestBodyLimitLayer;
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

    let body_limit = RequestBodyLimitLayer::new(state.config.max_request_size);

    Router::new()
        // Health check (con config info)
        .route("/health", get(health_check))
        // Server config endpoint
        .route("/config", get(get_server_config))
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
        .layer(body_limit)
        .layer(TraceLayer::new_for_http())
}

/// Health check endpoint
async fn health_check(State(state): State<McpServerState>) -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "nuclear-mcp",
        "version": env!("CARGO_PKG_VERSION"),
        "protocol": "MCP 2025",
        "config": {
            "host": state.config.host,
            "port": state.config.port,
            "cors_enabled": state.config.enable_cors,
            "max_request_size": state.config.max_request_size
        }
    }))
}

/// Get server configuration endpoint
async fn get_server_config(State(state): State<McpServerState>) -> impl IntoResponse {
    Json(json!({
        "host": state.config.host,
        "port": state.config.port,
        "enable_cors": state.config.enable_cors,
        "max_request_size": state.config.max_request_size,
        "max_request_size_mb": state.config.max_request_size / (1024 * 1024)
    }))
}

/// MCP Initialize endpoint
async fn mcp_initialize(
    State(_state): State<McpServerState>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, McpError> {
    let protocol_version = extract_protocol_version(&payload)?;

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
async fn mcp_tools_list(State(_state): State<McpServerState>) -> Result<Json<Value>, McpError> {
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
    // Validar tamaño de request usando config
    let payload_size = serde_json::to_string(&payload)
        .map(|s| s.len())
        .unwrap_or(0);
    if payload_size > state.config.max_request_size {
        return Err(McpError::PayloadTooLarge {
            size: payload_size,
            max: state.config.max_request_size,
        });
    }

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

    let serialized = serde_json::to_string(&result)
        .map_err(|e| McpError::InternalError(format!("Serialize tool result: {e}")))?;

    Ok(Json(json!({
        "content": [
            {
                "type": "text",
                "text": serialized
            }
        ]
    })))
}

/// MCP WebSocket endpoint (MCP 2025)
async fn mcp_websocket(ws: WebSocketUpgrade, State(state): State<McpServerState>) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

/// Handle WebSocket connection
async fn handle_websocket(mut socket: axum::extract::ws::WebSocket, state: McpServerState) {
    use axum::extract::ws::Message;

    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                // Parse JSON-RPC request
                match serde_json::from_str::<Value>(&text) {
                    Ok(request) => {
                        // 🔑 MCP 2025: Check if it's a notification (no id) or request (with id)
                        let has_id = request.get("id").map(|v| !v.is_null()).unwrap_or(false);

                        // Standard MCP 2025 notifications to ignore
                        let standard_notifications = [
                            "logging/setLevel",
                            "notifications/progress",
                            "notifications/resources/list_changed",
                        ];

                        let method = request["method"].as_str().unwrap_or("");

                        // If it's a known notification, don't process further
                        if !has_id && standard_notifications.contains(&method) {
                            continue;
                        }

                        let mut mcp = state.mcp.write().await;
                        let response = mcp.handle_request(request).await;

                        // Only send response if it's not an empty notification response
                        let should_send =
                            !response.as_object().map(|o| o.is_empty()).unwrap_or(false) || has_id;

                        if should_send {
                            if let Ok(response_text) = serde_json::to_string(&response) {
                                if socket.send(Message::Text(response_text)).await.is_err() {
                                    break;
                                }
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
                        tracing::warn!("WebSocket parse error on MCP message");
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
            "description": "🔥 Búsqueda NUCLEAR (15 seg): Máxima potencia automática. Solo pon el query.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Término de búsqueda"
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
    PayloadTooLarge { size: usize, max: usize },
    ToolError(String),
    InternalError(String),
}

impl IntoResponse for McpError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            McpError::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, -32600, msg),
            McpError::PayloadTooLarge { size, max } => (
                StatusCode::PAYLOAD_TOO_LARGE,
                -32602,
                format!("Request size {} exceeds max {}", size, max),
            ),
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

fn extract_protocol_version(payload: &Value) -> Result<&str, McpError> {
    let protocol_version = payload["protocolVersion"]
        .as_str()
        .ok_or_else(|| McpError::InvalidRequest("Missing protocolVersion".to_string()))?;

    let supported_versions = ["2025-06-18", "2024-11-05"];
    if !supported_versions.contains(&protocol_version) {
        return Err(McpError::InvalidRequest(format!(
            "Unsupported protocol version: {}. Supported: {:?}",
            protocol_version, supported_versions
        )));
    }

    Ok(protocol_version)
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
