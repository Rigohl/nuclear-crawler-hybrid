//! 🔥 NUCLEAR CRAWLER HYBRID - MCP Server
//!
//! Servidor Model Context Protocol con todo el poder de Nuclear Crawler
//! Funciona como stdin/stdout JSON-RPC server para Claude, VSCode, etc.
//!
//! Features:
//! - Búsqueda web masiva (50+ concurrent)
//! - Web scraping nuclear
//! - Análisis de proyectos
//! - Estadísticas y monitoreo
//! - Auto-gestión de dependencias

use anyhow::Result;
use nuclear_crawler_lib::simple_mcp::SimpleMcpServer;

#[tokio::main]
async fn main() -> Result<()> {
    // El servidor MCP corre en modo silencioso (no stdout para mantener JSON-RPC limpio)
    // Toda la comunicación es a través de stdin/stdout JSON-RPC
    // Los mensajes de error van a stderr

    SimpleMcpServer::run_async().await?;
    Ok(())
}
