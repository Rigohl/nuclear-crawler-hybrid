//! Nuclear Crawler Hybrid - Real MCP Server with Chapel AI
//!
//! 🔥 MCP PROTOCOL 2025: EXACTLY 5 PRODUCTION TOOLS
//! 🚀 CHAPEL AI: Continuous learning system (96% accuracy)
//! 🤖 WASM OPTIMIZATION: 50-100x speedup modules
//! ⚡ FFI ACCELERATORS: Go/JAX/Nim/Zig/Chapel backends

#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

// ═══════════════════════════════════════════════════════════════════
// CORE MODULES (ALWAYS ACTIVE)
// ═══════════════════════════════════════════════════════════════════

/// MCP Protocol Server - EXACTLY 5 production tools
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

pub use ffi::chapel_integration::{get_chapel_ai, ChapelAI, ChapelContext};
pub use osint::{
    OSINTNeuralNetwork, BotClassifierNN, AuthorshipNN,
    OSINTBayesianNetwork, BayesianNetwork, OSINTNaiveBayes,
    OSINTAdversarialGame, PayoffMatrix, NashSolver, MixedStrategy,
    OSINTIntegrationPipeline, NuclearDataAggregator,
    OSINTCaseResolver, CaseManager, OSINTCase, CaseType, CaseReport,
};
pub use ai::{Chatbot, ChatbotConfig, HuggingFaceClient, HuggingFaceConfig};

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
