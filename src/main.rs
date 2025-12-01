//! 🔥 NUCLEAR CRAWLER HYBRID - MCP Server
//!
//! Servidor Model Context Protocol con todo el poder de Nuclear Crawler
//! Funciona como stdin/stdout JSON-RPC server para Claude, VSCode, etc.

use anyhow::Result;
use nuclear_crawler_lib::simple_mcp::SimpleMcpServer;

#[tokio::main]
async fn main() -> Result<()> {
    // MCP Server corre en modo silencioso (sin stdout para mantener JSON-RPC limpio)
    // Toda la comunicación es a través de stdin/stdout JSON-RPC
    // Los mensajes de error van a stderr

    SimpleMcpServer::run_async().await?;
    Ok(())
}
