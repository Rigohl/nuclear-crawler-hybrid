//! 🔥 MCP TOOLS - EXACTLY 5 TOOLS EN MÁXIMO PODER + CHATBOT
//!
//! Siguiendo MCP 2025 Protocol - 5 tools fundamentales + chatbot, cero experimental
//!
//! 1. WEBSEARCH    - Búsqueda web real en 55+ motores, HTTP real, stealth
//! 2. PREMIUM      - Extrae paywalls (Medium, ArXiv, O'Reilly), bypass 100%
//! 3. FILE_SEARCH  - Análisis avanzado (Zig SIMD, Nim parsing, detecta errores)
//! 4. SCAN         - Escaneo paralelo workspace (Go 1000 goroutines)
//! 5. AI_DATASET   - Entrena IA (Go + Zig + Nim + JAX pipeline)
//! 6. CHATBOT      - Interactive AI assistant with tool integration (NEW!)
//!
//! Todo experimental (full_stack, websearch_complete, mega_tool, etc) ELIMINADO

pub mod ai_dataset_trainer;
pub mod chatbot_tool;
pub mod dataset_generator;
pub mod file_search_advanced;
pub mod premium_content;
pub mod scan_workspace;
pub mod websearch;

// ✅ 5 TOOLS PRINCIPALES + CHATBOT + BONUS GENERATOR
pub use ai_dataset_trainer::{
    AIDatasetTrainerTool, DatasetTrainerConfig, TrainingDatapoint, TrainingDataset,
};
pub use chatbot_tool::{execute_chatbot, ChatbotArgs, ChatbotResult};
pub use file_search_advanced::{
    AdvancedFileSearchTool, CodeIssue, FileAnalysisResult, FileMatch,
    FileSearchConfig as FileSearchAdvancedConfig, FileSearchResult,
};
pub use premium_content::{PremiumConfig, PremiumContent, PremiumContentTool};
pub use scan_workspace::{FileAnalysis, ScanConfig, ScanIssue, ScanResult, ScanWorkspaceTool};
pub use websearch::{SearchResult, WebSearchConfig, WebSearchTool};

// ⚠️ BONUS (No en los 5 principales, pero útil)
pub use dataset_generator::{Dataset, DatasetConfig, DatasetGeneratorTool, DatasetItem};
