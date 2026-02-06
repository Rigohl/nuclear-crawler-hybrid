//! Nuclear Crawler Hybrid - Real MCP Server with Chapel AI
//!
//! 🔥 MCP PROTOCOL 2026: EXACTLY 7 PRODUCTION TOOLS
//! 🚀 3 BINARIES: nuclear-mcp (7 tools), nuclear-pro (5), nuclear-lite (2) = MAX POWER
//! 🤖 WASM OPTIMIZATION: 50-100x speedup modules
//! ⚡ FFI ACCELERATORS: Go/JAX/Nim/Zig/Chapel backends

#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

// ═══════════════════════════════════════════════════════════════════
// CORE MODULES (ALWAYS ACTIVE)
// ═══════════════════════════════════════════════════════════════════

/// MCP Protocol Server - EXACTLY 7 production tools, 3 binary profiles
pub mod mcp;

/// Core crawler functionality
pub mod core;

/// Data management, indexing, search
pub use core::data_management;

/// Web search with 55+ search engines
pub use core::web_search;

/// Dataset generation for Chapel AI training
pub use core::dataset_generator;

/// URL utilities
pub use core::url_helpers;

/// Premium content extraction
pub use core::premium_content_scraper;

/// Nuclear core functionality
pub use core::nuclear_core;

// ═══════════════════════════════════════════════════════════════════
// HIGH-PERFORMANCE OPTIMIZATION
// ═══════════════════════════════════════════════════════════════════

/// WASM acceleration - 50-100x speedup
pub mod wasm;

/// Chapel parallel execution framework
pub mod chapel_parallel;

/// Tantivy full-text search engine - REAL SEARCH
pub mod tantivy_search;

// ═══════════════════════════════════════════════════════════════════
// FFI & AI
// ═══════════════════════════════════════════════════════════════════

/// FFI accelerators - Go/JAX/Nim/Zig/Chapel backends
pub mod ffi;

/// AI suite - Chatbot + HuggingFace integration
pub mod ai;

// ═══════════════════════════════════════════════════════════════════
// OSINT SUITE (Advanced Competition Framework A-E)
// ═══════════════════════════════════════════════════════════════════

/// OSINT suite - 5 integrated modules for competitive intelligence
pub mod osint;

// ═══════════════════════════════════════════════════════════════════
// INFRASTRUCTURE & UTILITIES
// ═══════════════════════════════════════════════════════════════════

/// Infrastructure modules - caching, rate limiting, storage, etc.
pub mod infra;

// ═══════════════════════════════════════════════════════════════════
// RE-EXPORTS FOR CONVENIENCE
// ═══════════════════════════════════════════════════════════════════

// ✅ FFI Modules - Re-export at crate level
pub use ffi::chapel_integration;
pub use ffi::chapel_integration::{get_chapel_ai, ChapelAI, ChapelContext};
pub use ffi::go_integration;
pub use ffi::jax_integration;
pub use ffi::nim_integration;
pub use ffi::zig_integration;

// ✅ AI Modules - Re-export at crate level
pub use ai::chatbot;
pub use ai::huggingface_integration;
pub use ai::{Chatbot, ChatbotConfig, HuggingFaceClient, HuggingFaceConfig};

// ✅ Infrastructure Modules - Re-export at crate level
pub use infra::advanced_bypass;
pub use infra::cache;
pub use infra::chromium_rendering;
pub use infra::data_extraction;
pub use infra::deepweb_tor;
pub use infra::intelligent_storage;
pub use infra::proxy_rotation;
pub use infra::rate_limit;

// ✅ OSINT Suite - Re-export at crate level
pub use osint::{
    AuthorshipNN, BayesianNetwork, BotClassifierNN, CaseManager, CaseReport, CaseType,
    MixedStrategy, NashSolver, NuclearDataAggregator, OSINTAdversarialGame, OSINTBayesianNetwork,
    OSINTCase, OSINTCaseResolver, OSINTIntegrationPipeline, OSINTNaiveBayes, OSINTNeuralNetwork,
    PayoffMatrix,
};

// ═══════════════════════════════════════════════════════════════════
// WASM INITIALIZATION
// ═══════════════════════════════════════════════════════════════════

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

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
    }
}
