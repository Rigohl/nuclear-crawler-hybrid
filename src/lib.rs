//! Nuclear Crawler Hybrid - Crawler/Scraper avanzado con WebAssembly
//!
//! Sistema de alto rendimiento para web scraping y crawling
//! con WebAssembly potente para MCP (Model Context Protocol)

#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

pub mod ai_smart; // AI unificado (consolidado)
pub mod cache; // Cache simple para nuclear_scraper
pub mod config;
pub mod core_tools; // 4 herramientas MCP consolidadas
pub mod file_search;
pub mod intelligent_storage;
pub mod massive_parallel_search;
pub mod mcp_axum_server;
pub mod nuclear_scraper; // Scraper unificado (consolidado con mass_capture)
pub mod parallel_crawler; // Crawler paralelo
pub mod parser; // HTML parser
pub mod rate_limit;
pub mod simple_mcp;
pub mod stats;
pub mod stealth;
pub mod utils;
pub mod wasm; // WASM unificado (consolidado)
pub mod web_search;

// Integraciones en Rust puro (sin FFI externo)
pub mod deep_web_search;
pub mod go_integration;
pub mod hf_integration;
pub mod improvements;
pub mod jax_acceleration;
pub mod jax_pipeline;
pub mod mojo_jax;
pub mod nim_integration;
pub mod nuclear_bypass;
pub mod orchestration;
pub mod project_analyzer;
pub mod scan_project;
pub mod zig_integration;

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
