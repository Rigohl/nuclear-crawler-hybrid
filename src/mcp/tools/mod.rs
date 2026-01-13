//! 🔥 MCP TOOLS - EXACTLY 5 TOOLS EN MÁXIMO PODER
//! 
//! Siguiendo MCP 2025 Protocol - 5 tools fundamentales, cero experimental
//! 
//! 1. WEBSEARCH    - Búsqueda web real en 55+ motores, HTTP real, stealth
//! 2. PREMIUM      - Extrae paywalls (Medium, ArXiv, O'Reilly), bypass 100%
//! 3. FILE_SEARCH  - Análisis avanzado (Zig SIMD, Nim parsing, detecta errores)
//! 4. SCAN         - Escaneo paralelo workspace (Go 1000 goroutines)
//! 5. AI_DATASET   - Entrena IA (Go + Zig + Nim + JAX pipeline)
//!
//! Todo experimental (full_stack, websearch_complete, mega_tool, etc) ELIMINADO

pub mod websearch;
pub mod premium_content;
pub mod file_search_advanced;
pub mod scan_workspace;
pub mod ai_dataset_trainer;
pub mod dataset_generator;

// ✅ 5 TOOLS PRINCIPALES + BONUS GENERATOR
pub use websearch::{WebSearchTool, SearchResult, WebSearchConfig};
pub use premium_content::{PremiumContentTool, PremiumContent, PremiumConfig};
pub use file_search_advanced::{AdvancedFileSearchTool, CodeIssue, FileAnalysisResult, FileSearchConfig as FileSearchAdvancedConfig, FileSearchResult, FileMatch};
pub use scan_workspace::{ScanWorkspaceTool, ScanConfig, ScanResult, ScanIssue, FileAnalysis};
pub use ai_dataset_trainer::{AIDatasetTrainerTool, TrainingDataset, TrainingDatapoint, DatasetTrainerConfig};

// ⚠️ BONUS (No en los 5 principales, pero útil)
pub use dataset_generator::{DatasetGeneratorTool, Dataset, DatasetItem, DatasetConfig};

