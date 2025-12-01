//! Nuclear Crawler Hybrid - Crawler/Scraper avanzado con WebAssembly
//!
//! Sistema de alto rendimiento para web scraping y crawling
//! con WebAssembly potente para MCP (Model Context Protocol)

#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

pub mod ai_smart;
pub mod cache;
pub mod config;
pub mod crawler;
pub mod file_search;
pub mod intelligence;
pub mod mass_capture;
pub mod massive_parallel_search;
pub mod nuclear_scraper;
// parser.rs consolidado en scraper.rs
pub mod rate_limit;
pub mod scraper;
pub mod simple_mcp;
pub mod mcp_axum_server;
pub mod stats;
pub mod stealth;
pub mod utils;
pub mod wasm;
// wasm_mass_capture.rs consolidado en wasm.rs
pub mod intelligent_storage;
pub mod web_search;

// Integraciones en Rust puro (sin FFI externo)
pub mod go_integration;
pub mod nim_integration;
pub mod zig_integration;
pub mod hf_integration;
pub mod jax_acceleration;
pub mod jax_pipeline;
pub mod mojo_jax;
pub mod orchestration;
pub mod project_analyzer;
pub mod scan_project;
pub mod deep_web_search;
pub mod improvements;
pub mod nuclear_bypass;

// 🔥💥 NUEVOS MÓDULOS NUCLEARES EXTREMOS
pub mod nuclear_unified_scraper;   // Scraper unificado modular
// nuclear_extreme_crawler y nuclear_max_power consolidados en nuclear_unified_scraper

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
