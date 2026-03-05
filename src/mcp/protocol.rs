//! 🔥 MCP PROTOCOL - JSON-RPC 2.0 Implementation
//!
//! Follows Model Context Protocol 2026 specification
//! EXPONE EXACTAMENTE 7 TOOLS CON MÁXIMO PODER REAL
//!
//! 1. WEBSEARCH - Búsqueda internet con scraping integrado (55+ motores + WASM)
//! 2. PREMIUM - Libros, cursos, Medium, ArXiv (link o frase)
//! 3. FILE_SEARCH - Líneas exactas, palabras, errores, warnings (SIMD)
//! 4. SCAN - Escanea todo: archivo, doc, carpeta, workspace (Go paralelo)
//! 5. AI_DATASET_TRAINER - Entrenar IA con datasets (FFI multi-language)
//! 6. PARALLEL_ENGINE - Motor paralelo que potencia TODAS las tools (Go+SIMD+GPU+Chapel)
//! 7. OSINT_INTELLIGENCE - Inteligencia OSINT con Chapel AI (mining real)

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Tool profile determines which subset of tools a binary exposes.
/// Combining all 3 profiles covers all 7 tools (MAX POWER).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolProfile {
    /// Full power: all 7 MCP tools
    Full,
    /// Pro: 5 tools (websearch, premium, file_search, scan, ai_dataset_trainer)
    Pro,
    /// Lite: 2 tools (websearch, scan)
    Lite,
}
/// MCP Request (JSON-RPC 2.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

/// MCP Response (JSON-RPC 2.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPResponse {
    pub jsonrpc: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<MCPError>,
}

/// MCP Error - JSON-RPC 2.0 compliant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// Standard JSON-RPC 2.0 error codes
pub mod error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;
    // Custom codes
    pub const TOOL_EXECUTION_ERROR: i32 = -32000;
    pub const RATE_LIMITED: i32 = -32001;
    pub const UNAUTHORIZED: i32 = -32002;
}

/// Tool definition for MCP - JSON Schema compliant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: serde_json::Value,
}

impl MCPRequest {
    /// Validate request structure
    pub fn validate(&self) -> Result<(), MCPError> {
        if self.jsonrpc != "2.0" {
            return Err(MCPError {
                code: error_codes::INVALID_REQUEST,
                message: "jsonrpc must be '2.0'".to_string(),
                data: None,
            });
        }
        if self.id.is_empty() {
            return Err(MCPError {
                code: error_codes::INVALID_REQUEST,
                message: "id cannot be empty".to_string(),
                data: None,
            });
        }
        if self.method.is_empty() {
            return Err(MCPError {
                code: error_codes::INVALID_REQUEST,
                message: "method cannot be empty".to_string(),
                data: None,
            });
        }
        Ok(())
    }

    /// Create tools list request
    pub fn list_tools() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: uuid::Uuid::new_v4().to_string(),
            method: "tools/list".to_string(),
            params: json!({}),
        }
    }

    /// Create tool call request
    pub fn call_tool(name: &str, arguments: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: uuid::Uuid::new_v4().to_string(),
            method: "tools/call".to_string(),
            params: json!({
                "name": name,
                "arguments": arguments
            }),
        }
    }

    /// Extract tool name from params (safe)
    pub fn get_tool_name(&self) -> Option<&str> {
        self.params.get("name").and_then(|v| v.as_str())
    }

    /// Extract arguments from params (safe)
    pub fn get_arguments(&self) -> Value {
        self.params.get("arguments").cloned().unwrap_or(Value::Null)
    }
}

impl MCPResponse {
    /// Create success response
    pub fn success(id: String, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create error response
    pub fn error(id: String, code: i32, message: String) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(MCPError {
                code,
                message,
                data: None,
            }),
        }
    }

    /// Create error response with data
    pub fn error_with_data(id: String, code: i32, message: String, data: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(MCPError {
                code,
                message,
                data: Some(data),
            }),
        }
    }

    /// Create method not found error
    pub fn method_not_found(id: String, method: &str) -> Self {
        Self::error(
            id,
            error_codes::METHOD_NOT_FOUND,
            format!("Method not found: {}", method),
        )
    }

    /// Create invalid params error
    pub fn invalid_params(id: String, reason: &str) -> Self {
        Self::error(
            id,
            error_codes::INVALID_PARAMS,
            format!("Invalid params: {}", reason),
        )
    }

    /// Create parse error
    pub fn parse_error(id: String) -> Self {
        Self::error(
            id,
            error_codes::PARSE_ERROR,
            "Parse error: Invalid JSON".to_string(),
        )
    }

    /// Create internal error
    pub fn internal_error(id: String, details: &str) -> Self {
        Self::error(
            id,
            error_codes::INTERNAL_ERROR,
            format!("Internal error: {}", details),
        )
    }

    /// Create rate limited error
    pub fn rate_limited(id: String) -> Self {
        Self::error(
            id,
            error_codes::RATE_LIMITED,
            "Rate limited: Too many requests".to_string(),
        )
    }
}

/// 🔥 GET EXACTLY 7 TOOL DEFINITIONS - MÁXIMO PODER AMPLIFICADO
/// Siguiendo MCP 2026 protocol: 7 tools profesionales con WASM + Chapel AI
pub fn get_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        // ═══════════════════════════════════════════════════════════════════════
        // 1️⃣ WEBSEARCH - Búsqueda internet MÁXIMO PODER (55+ motores)
        // ═══════════════════════════════════════════════════════════════════════
        ToolDefinition {
            name: "websearch".to_string(),
            description: "🔍 BÚSQUEDA WEB - 55+ motores (DuckDuckGo, Bing, Brave, Yandex, etc). HTTP real, bypass automático, stealth activado, rate limiting invisible. Max 100 resultados, <2s response. Entrada: query (requerido).".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Búsqueda (palabras clave, pregunta, URL, tema). Soporta: 'python machine learning', 'site:github.com rust async', etc."
                    }
                },
                "required": ["query"],
                "additionalProperties": false
            }),
        },

        // ═══════════════════════════════════════════════════════════════════════
        // 2️⃣ PREMIUM - Extracción de contenido premium (BYPASS REAL)
        // ═══════════════════════════════════════════════════════════════════════
        ToolDefinition {
            name: "premium".to_string(),
            description: "📚 CONTENIDO PREMIUM - Extrae paywalls (Medium, ArXiv, O'Reilly, GitHub, etc). HTTP real, bypass quantum_bypass 100% Coursera, stealth total, extracción completa de módulos/lecciones. Entrada: URL o búsqueda.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "input": {
                        "type": "string",
                        "description": "URL exacta (https://...) O búsqueda para encontrar (ej: 'machine learning coursera', 'arxiv quantum computing')"
                    }
                },
                "required": ["input"],
                "additionalProperties": false
            }),
        },

        // ═══════════════════════════════════════════════════════════════════════
        // 3️⃣ FILE_SEARCH - Análisis avanzado de archivos (SIMD acelerado)
        // ═══════════════════════════════════════════════════════════════════════
        ToolDefinition {
            name: "file_search".to_string(),
            description: "📄 BÚSQUEDA EN ARCHIVOS - Análisis avanzado: detecta errores, warnings, TODOs, mocks, código sin usar. Zig SIMD (<1s Blake3), Nim HTML parsing, búsqueda exacta/regex. Entrada: path + query.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Archivo o carpeta a buscar (ej: 'src/', 'file.rs', '.')"
                    },
                    "query": {
                        "type": "string",
                        "description": "Qué buscar (texto, regex, o keywords: 'TODO', 'ERROR', 'FIXME', 'mock')"
                    }
                },
                "required": ["path", "query"],
                "additionalProperties": false
            }),
        },

        // ═══════════════════════════════════════════════════════════════════════
        // 4️⃣ SCAN - Escaneo profundo de workspace (1000 goroutines Go)
        // ═══════════════════════════════════════════════════════════════════════
        ToolDefinition {
            name: "scan".to_string(),
            description: "🔬 ESCANEO PROFUNDO - Paralelo con Go (1000 goroutines), detecta: errores, warnings, TODOs, mocks, complejidad ciclomática, health score. Entrada: path (opcional, default: '.').".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Qué escanear: archivo, carpeta, '.' para workspace actual, o 'project' para todo. Default: '.'",
                        "default": "."
                    }
                },
                "required": [],
                "additionalProperties": false
            }),
        },

        // ═══════════════════════════════════════════════════════════════════════
        // 5️⃣ AI_DATASET_TRAINER - Entrenar IA con FFI (4 procesadores paralelos)
        // ═══════════════════════════════════════════════════════════════════════
        ToolDefinition {
            name: "ai_dataset_trainer".to_string(),
            description: "🧠 AI DATASET TRAINER - Crea datasets para entrenar IA. Pipeline paralelo: Go (búsqueda 1000 concurrent) → Zig (SIMD dedup) → Nim (HTML parse) → JAX (GPU vectorization 1536-dim). Produce embeddings listos para training. Entrada: nombre + tamaño (opcional).".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "dataset_name": {
                        "type": "string",
                        "description": "Nombre del dataset (ej: 'ml_papers_2024')",
                        "default": "training_data"
                    },
                    "target_size": {
                        "type": "integer",
                        "description": "Datapoints objetivo: 1000, 10000, 100000. Default 10000 (~5 min)",
                        "default": 10000
                    }
                },
                "required": [],
                "additionalProperties": false
            }),
        },

        // ═══════════════════════════════════════════════════════════════════════
        // 6️⃣ PARALLEL_ENGINE - Motor paralelo que potencia TODAS las herramientas
        // ═══════════════════════════════════════════════════════════════════════
        ToolDefinition {
            name: "parallel_engine".to_string(),
            description: "🚀 PARALLEL ENGINE - Motor de procesamiento paralelo ultra-rápido que potencia TODAS las tools. Rayon multi-thread, Go goroutines, SIMD (Zig), GPU/TPU (JAX), distributed (Chapel multi-locale), WASM compilation. Usa este motor para: batch processing, parallel map/reduce, GPU acceleration, distributed computing. 100x más rápido que procesamiento serial.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "operation": {
                        "type": "string",
                        "description": "Operación a ejecutar: 'batch_process', 'parallel_map', 'parallel_reduce', 'gpu_accelerate', 'distribute'"
                    },
                    "data": {
                        "description": "Datos a procesar (array, object, o string)"
                    },
                    "workers": {
                        "type": "integer",
                        "description": "Número de workers paralelos (default: num_cpus)",
                        "default": num_cpus::get()
                    },
                    "use_gpu": {
                        "type": "boolean",
                        "description": "Usar aceleración GPU via JAX (default: false)",
                        "default": false
                    },
                    "use_simd": {
                        "type": "boolean",
                        "description": "Usar SIMD via Zig para operaciones vectoriales (default: true)",
                        "default": true
                    }
                },
                "required": ["operation", "data"],
                "additionalProperties": false
            }),
        },

        // ═══════════════════════════════════════════════════════════════════════
        // 7️⃣ OSINT_INTELLIGENCE - Inteligencia OSINT con Chapel AI (mining real)
        // ═══════════════════════════════════════════════════════════════════════
        ToolDefinition {
            name: "osint_intelligence".to_string(),
            description: "🔍 OSINT INTELLIGENCE - Minería de inteligencia de código abierto con Chapel AI. Búsqueda distribuida en 100+ fuentes (redes sociales, registros públicos, GitHub, leaks, dark web). Chapel AI analiza patrones, correlaciona datos, detecta anomalías. GPU-accelerated para grandes volúmenes. Entrada: target + tipo.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "target": {
                        "type": "string",
                        "description": "Objetivo a investigar: email, username, domain, IP, company, etc."
                    },
                    "search_type": {
                        "type": "string",
                        "description": "Tipo de búsqueda: 'person', 'company', 'domain', 'email', 'username', 'ip', 'phone', 'all'. Default: 'all'",
                        "default": "all"
                    },
                    "depth": {
                        "type": "string",
                        "description": "Profundidad: 'basic' (rápido, 10 fuentes), 'deep' (completo, 50+ fuentes), 'maximum' (exhaustivo, 100+ fuentes). Default: 'deep'",
                        "default": "deep"
                    },
                    "include_darkweb": {
                        "type": "boolean",
                        "description": "Incluir búsqueda en dark web (TOR). Default: false",
                        "default": false
                    }
                },
                "required": ["target"],
                "additionalProperties": false
            }),
        },
    ]
}

/// Get tool by name
pub fn get_tool_definition(name: &str) -> Option<ToolDefinition> {
    get_tool_definitions().into_iter().find(|t| t.name == name)
}

/// Validate tool exists
pub fn tool_exists(name: &str) -> bool {
    get_tool_definitions().iter().any(|t| t.name == name)
}

/// Get list of available tool names
pub fn get_tool_names() -> Vec<String> {
    get_tool_definitions()
        .iter()
        .map(|t| t.name.clone())
        .collect()
}

/// Get tool names that belong to a specific profile
pub fn get_profile_tool_names(profile: ToolProfile) -> Vec<&'static str> {
    match profile {
        ToolProfile::Full => vec![
            "websearch",
            "premium",
            "file_search",
            "scan",
            "ai_dataset_trainer",
            "parallel_engine",
            "osint_intelligence",
        ],
        ToolProfile::Pro => vec![
            "websearch",
            "premium",
            "file_search",
            "scan",
            "ai_dataset_trainer",
        ],
        ToolProfile::Lite => vec!["websearch", "scan"],
    }
}

/// Get tool definitions filtered by profile
pub fn get_tool_definitions_for_profile(profile: ToolProfile) -> Vec<ToolDefinition> {
    let allowed = get_profile_tool_names(profile);
    get_tool_definitions()
        .into_iter()
        .filter(|t| allowed.contains(&t.name.as_str()))
        .collect()
}

/// Validate tool exists for a specific profile
pub fn tool_exists_for_profile(name: &str, profile: ToolProfile) -> bool {
    get_profile_tool_names(profile).contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exactly_7_tools() {
        let tools = get_tool_definitions();
        assert_eq!(
            tools.len(),
            7,
            "Must have EXACTLY 7 tools (amplified power with WASM + OSINT)"
        );
    }

    #[test]
    fn test_tool_names() {
        let names = get_tool_names();
        assert_eq!(names.len(), 7, "Exactly 7 tool names");
        assert!(names.contains(&"websearch".to_string()));
        assert!(names.contains(&"premium".to_string()));
        assert!(names.contains(&"file_search".to_string()));
        assert!(names.contains(&"scan".to_string()));
        assert!(names.contains(&"ai_dataset_trainer".to_string()));
        assert!(names.contains(&"parallel_engine".to_string()));
        assert!(names.contains(&"osint_intelligence".to_string()));

        // MUST NOT contain old/experimental tools
        assert!(!names.contains(&"wasm_scraper".to_string()));
        assert!(!names.contains(&"full_stack_integration".to_string()));
        assert!(!names.contains(&"info".to_string()));
        assert!(!names.contains(&"nuclear_mega_tool".to_string()));
        assert!(!names.contains(&"websearch_complete".to_string()));
    }

    #[test]
    fn test_tool_exists() {
        assert!(tool_exists("websearch"));
        assert!(tool_exists("osint_intelligence"));
        assert!(!tool_exists("non_existent_tool"));
        assert!(!tool_exists(""));
    }

    #[test]
    fn test_get_tool_definition() {
        let def = get_tool_definition("websearch");
        assert!(def.is_some());
        assert_eq!(def.unwrap().name, "websearch");

        let missing = get_tool_definition("fake_tool_123");
        assert!(missing.is_none());
    }

    #[test]
    fn test_get_profile_tool_names() {
        let full_tools = get_profile_tool_names(ToolProfile::Full);
        assert_eq!(full_tools.len(), 7);
        assert!(full_tools.contains(&"websearch"));

        let pro_tools = get_profile_tool_names(ToolProfile::Pro);
        assert_eq!(pro_tools.len(), 5);
        assert!(pro_tools.contains(&"ai_dataset_trainer"));

        let lite_tools = get_profile_tool_names(ToolProfile::Lite);
        assert_eq!(lite_tools.len(), 2);
        assert!(lite_tools.contains(&"scan"));
    }

    #[test]
    fn test_request_validation() {
        let valid = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: "test-123".to_string(),
            method: "tools/list".to_string(),
            params: json!({}),
        };
        assert!(valid.validate().is_ok());

        let invalid = MCPRequest {
            jsonrpc: "1.0".to_string(),
            id: "test".to_string(),
            method: "test".to_string(),
            params: json!({}),
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_response_serialization() {
        let response = MCPResponse::success("123".to_string(), json!({"test": true}));
        let json_str = serde_json::to_string(&response).unwrap();
        assert!(json_str.contains("\"jsonrpc\":\"2.0\""));
        assert!(json_str.contains("\"id\":\"123\""));
        assert!(!json_str.contains("\"error\""));
    }

    #[test]
    fn test_profile_full_has_7_tools() {
        let tools = get_tool_definitions_for_profile(ToolProfile::Full);
        assert_eq!(tools.len(), 7, "Full profile must have 7 tools");
    }

    #[test]
    fn test_profile_pro_has_5_tools() {
        let tools = get_tool_definitions_for_profile(ToolProfile::Pro);
        assert_eq!(tools.len(), 5, "Pro profile must have 5 tools");
        let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"websearch"));
        assert!(names.contains(&"premium"));
        assert!(names.contains(&"file_search"));
        assert!(names.contains(&"scan"));
        assert!(names.contains(&"ai_dataset_trainer"));
    }

    #[test]
    fn test_profile_lite_has_2_tools() {
        let tools = get_tool_definitions_for_profile(ToolProfile::Lite);
        assert_eq!(tools.len(), 2, "Lite profile must have 2 tools");
        let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"websearch"));
        assert!(names.contains(&"scan"));
    }

    #[test]
    fn test_three_profiles_cover_all_7_tools() {
        use std::collections::HashSet;
        let mut all: HashSet<String> = HashSet::new();
        for profile in [ToolProfile::Full, ToolProfile::Pro, ToolProfile::Lite] {
            for name in get_profile_tool_names(profile) {
                all.insert(name.to_string());
            }
        }
        assert_eq!(
            all.len(),
            7,
            "Union of all 3 profiles = MAX POWER (7 tools)"
        );
    }
}
