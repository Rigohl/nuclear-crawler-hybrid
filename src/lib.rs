//! Nuclear Crawler Hybrid - Real MCP Server
//!
//! 🔥 MCP PROTOCOL 2025: EXACTLY 5 TOOLS IN MAXIMUM POWER
//! 🔥 REAL IMPLEMENTATIONS: No fake FFI or simulations
//! 🔥 WORKING CODE: Web scraping, file analysis, bypass systems
//! 🔥 ZERO DEAD CODE: All experimental modules removed
//! 🧠 CHAPEL AI: Continuous learning system connected to all tools
//! 🤗 HUGGINGFACE: Model training and deployment integration
//! 🤖 CHATBOT: Interactive AI assistant with tool access

#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

// 🔥 MCP PROTOCOL & 5 TOOLS
pub mod mcp; // Model Context Protocol with exactly 5 production tools

// 🔥 CORE MODULES FOR TOOLS - ONLY ACTIVE CODE
pub mod data_management; // REAL data indexing and search
pub mod nuclear_core; // REAL bypass, extraction, concealment, spider
pub mod premium_content_scraper; // REAL premium content extraction
pub mod url_helpers; // URL utilities
pub mod web_search; // REAL web search with 55+ engines

// 🔥 OPTIONAL ADVANCED MODULES (commented, integrated where needed)
// Uncomment if needed for specific features:
// pub mod chromium_rendering; // Chrome DevTools Protocol (optional)
// pub mod proxy_rotation;      // Proxy rotation (optional)
// pub mod data_extraction;     // Advanced extraction (optional)

// 🔥 FFI ACCELERATION MODULES - REAL HIGH-PERFORMANCE COMPUTING
pub mod chapel_integration; // Chapel AI learning and optimization - NEW!
pub mod go_integration; // Go parallel processing (1000 goroutines)
pub mod jax_integration; // JAX GPU vectorization
pub mod nim_integration; // Nim HTML parsing
pub mod zig_integration; // Zig SIMD hashing

// 🔥 NEW AI & CHATBOT MODULES
pub mod chatbot; // Interactive AI chatbot with tool integration
pub mod huggingface_integration; // HuggingFace model training & deployment

// 🔥 INFRASTRUCTURE MODULES FOR SERVER
pub mod advanced_bypass;
pub mod cache; // LRU cache implementation
pub mod deepweb_tor; // Deep web search via Tor
pub mod intelligent_storage; // JSON file storage
pub mod rate_limit; // Token bucket rate limiter // Advanced lock detection and bypass

// Re-exports for convenience
pub use chapel_integration::{get_chapel_ai, ChapelAI, ChapelContext};
pub use chatbot::{Chatbot, ChatbotConfig};
pub use huggingface_integration::{HuggingFaceClient, HuggingFaceConfig};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Inicializa el sistema de logging para WebAssembly
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Versión del crawler
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let v = version();
        assert!(!v.is_empty());
        assert!(v.starts_with("0."));
    }
}

// Dummy comment to force recompilation
