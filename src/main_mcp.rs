//! 🔥 NUCLEAR CRAWLER HYBRID - MCP Server
//!
//! Servidor Model Context Protocol con todo el poder de Nuclear Crawler
//! Soporta dos modos:
//! - stdio: Para Claude Desktop, VS Code (JSON-RPC via stdin/stdout)
//! - http: Para clientes HTTP/WebSocket (MCP 2025)
//!
//! Features:
//! - Búsqueda web masiva (50+ concurrent)
//! - Web scraping nuclear
//! - Análisis de proyectos
//! - Estadísticas y monitoreo
//! - Auto-gestión de dependencias

use anyhow::Result;
use clap::{Parser, Subcommand};
use nuclear_crawler_lib::mcp_axum_server::{start_server, McpServerConfig};
use nuclear_crawler_lib::simple_mcp::SimpleMcpServer;

#[derive(Parser)]
#[command(name = "nuclear-mcp")]
#[command(about = "Nuclear Crawler MCP Server - MCP 2025 Protocol", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run in stdio mode (for Claude Desktop, VS Code)
    Stdio,
    /// Run HTTP server (for web clients, MCP 2025)
    Http {
        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// Port to bind to
        #[arg(long, default_value = "3000")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Stdio) | None => {
            // Default: stdio mode for MCP clients like Claude Desktop
            eprintln!("🔥 Nuclear MCP Server - stdio mode");
            eprintln!("📡 Listening on stdin/stdout for JSON-RPC");
            SimpleMcpServer::run_async().await?;
        }
        Some(Commands::Http { host, port }) => {
            // HTTP mode for web clients
            let config = McpServerConfig {
                host,
                port,
                enable_cors: true,
                max_request_size: 10 * 1024 * 1024,
            };
            start_server(config).await?;
        }
    }

    Ok(())
}
