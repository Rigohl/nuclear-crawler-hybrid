//! Nuclear Crawler Hybrid - Real MCP Server
//!
//! 🔥 ONLY 2 TOOLS: WEB SEARCH & FILE SEARCH
//! 🔥 REAL IMPLEMENTATIONS: No fake FFI or simulations
//! 🔥 WORKING CODE: Web scraping, file analysis, bypass systems
//! 🔥 MINIMAL DEPENDENCIES: Only essential crates for maximum power

#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

// 🔥 ESSENTIAL MODULES FOR 2 TOOLS - NO DEAD CODE
pub mod ai_smart; // 🔥 AI intelligence for content analysis
pub mod cache; // 🔥 REAL LRU cache implementation
pub mod deepweb_tor;
pub mod file_search; // 🔥 REAL file analysis with error detection
pub mod go_integration; // 🔥 REAL Go parallel processing via FFI
pub mod intelligent_storage; // 🔥 REAL file storage for results
pub mod jax_integration; // 🔥 REAL JAX GPU vectorization via FFI
pub mod nim_integration; // 🔥 REAL Nim HTML parsing via FFI
pub mod nuclear_core; // 🔥 REAL bypass, extraction, concealment, spider
pub mod nuclear_mcp_server; // 🔥 NUCLEAR MCP SERVER 2025 - 3 EXTREME TOOLS
pub mod premium_content_scraper; // 🔥 REAL premium content extraction
pub mod rate_limit; // 🔥 REAL token bucket rate limiter
pub mod stealth; // 🔥 REAL stealth system for anti-detection
pub mod url_helpers;
pub mod web_search; // 🔥 REAL web search with extraction & scraping
pub mod zig_integration; // 🔥 REAL Zig SIMD hashing via FFI

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
