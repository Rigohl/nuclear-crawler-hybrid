//! 🔥 MCP TOOLS - EXACTLY 7 TOOLS EN MÁXIMO PODER (AMPLIFIED)
//!
//! Siguiendo MCP 2025 Protocol - 7 tools profesionales con WASM + Chapel AI
//!
//! 1. WEBSEARCH           - Búsqueda web real en 55+ motores, HTTP real, stealth
//! 2. PREMIUM             - Extrae paywalls (Medium, ArXiv, O'Reilly), bypass 100%
//! 3. FILE_SEARCH         - Análisis avanzado (Zig SIMD, Nim parsing, detecta errores)
//! 4. SCAN                - Escaneo paralelo workspace (Go 1000 goroutines)
//! 5. AI_DATASET_TRAINER  - Entrena IA (Go + Zig + Nim + JAX pipeline)
//! 6. WASM_SCRAPER        - Scraping ultra-rápido con WASM (100x speedup)
//! 7. OSINT_INTELLIGENCE  - Inteligencia OSINT con Chapel AI (mining real)
//!
//! BONUS: CHATBOT - Interactive AI assistant with tool integration

pub mod ai_dataset_trainer;
pub mod chatbot_tool;
pub mod dataset_generator;
pub mod file_search_advanced;
pub mod osint_intelligence_tool;
pub mod premium_content;
pub mod scan_workspace;
pub mod wasm_scraper_tool;
pub mod websearch;

// ✅ 7 TOOLS PRINCIPALES + BONUS
pub use ai_dataset_trainer::{
    AIDatasetTrainerTool, DatasetTrainerConfig, TrainingDatapoint, TrainingDataset,
};
pub use chatbot_tool::{execute_chatbot, ChatbotArgs, ChatbotResult};
pub use file_search_advanced::{
    AdvancedFileSearchTool, CodeIssue, FileAnalysisResult, FileMatch,
    FileSearchConfig as FileSearchAdvancedConfig, FileSearchResult,
};
pub use osint_intelligence_tool::execute_osint_intelligence;
pub use premium_content::{PremiumConfig, PremiumContent, PremiumContentTool};
pub use scan_workspace::{FileAnalysis, ScanConfig, ScanIssue, ScanResult, ScanWorkspaceTool};
pub use wasm_scraper_tool::execute_wasm_scraper;
pub use websearch::{SearchResult, WebSearchConfig, WebSearchTool};

// ⚠️ BONUS (No en los 7 principales, pero útil)
pub use dataset_generator::{Dataset, DatasetConfig, DatasetGeneratorTool, DatasetItem};
