//! Nuclear Crawler Hybrid - Real MCP Server
//!
//! 🔥 ONLY 2 TOOLS: WEB SEARCH & FILE SEARCH
//! 🔥 REAL IMPLEMENTATIONS: No fake FFI or simulations
//! 🔥 WORKING CODE: Web scraping, file analysis, bypass systems
//! 🔥 MINIMAL DEPENDENCIES: Only essential crates for maximum power

#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

// 🔥 ESSENTIAL MODULES FOR 2 TOOLS - NO DEAD CODE
pub mod file_search; // 🔥 REAL file analysis with error detection
pub mod nuclear_core; // 🔥 REAL bypass, extraction, concealment, spider
pub mod premium_content_scraper; // 🔥 REAL premium content extraction
pub mod url_helpers;
pub mod web_search; // 🔥 REAL web search with extraction & scraping // Utility functions

// 🔥 FFI ACCELERATION MODULES - REAL HIGH-PERFORMANCE COMPUTING
pub mod go_integration; // 🔥 REAL Go parallel processing via FFI
pub mod jax_integration; // 🔥 REAL JAX GPU vectorization via FFI
pub mod nim_integration; // 🔥 REAL Nim HTML parsing via FFI
pub mod zig_integration; // 🔥 REAL Zig SIMD hashing via FFI

// 🔥 INFRASTRUCTURE MODULES FOR SERVER
pub mod cache; // 🔥 REAL LRU cache implementation
pub mod deepweb_tor;
pub mod intelligent_storage; // 🔥 REAL file storage for results
pub mod rate_limit; // 🔥 REAL token bucket rate limiter // 🔥 REAL deep web search via Tor

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
