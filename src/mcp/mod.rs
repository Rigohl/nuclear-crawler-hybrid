//! 🔥 MCP PROTOCOL - Model Context Protocol HTTP Server
//! 
//! Implements MCP 2025 specification with JSON-RPC 2.0
//! Exposes tools via HTTP endpoints

pub mod tools;
pub mod protocol;
pub mod server;

pub use protocol::{MCPRequest, MCPResponse, ToolDefinition};
pub use server::MCPServer;
pub use tools::{WebSearchTool, PremiumContentTool, AdvancedFileSearchTool, DatasetGeneratorTool};
