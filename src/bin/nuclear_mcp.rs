//! 🔥 Nuclear MCP Server - Binary Entry Point
//!
//! Launches the MCP HTTP server on port 8079 (default) or custom port via CLI

use clap::Parser;
use nuclear_crawler_hybrid::mcp::server::MCPServer;

/// Nuclear MCP Server - AI-Powered Web Scraping & Analysis
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to bind the HTTP server
    #[arg(short, long, default_value_t = 8079)]
    port: u16,

    /// Host to bind the HTTP server
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.verbose {
        eprintln!("🔥 Nuclear MCP Server Starting...");
        eprintln!("   Host: {}", args.host);
        eprintln!("   Port: {}", args.port);
        eprintln!("   Protocol: HTTP + JSON-RPC 2.0");
        eprintln!("   Tools: 5 (websearch, premium, file_search, scan, info)");
        eprintln!("");
    }

    // Create MCP server
    let server = MCPServer::new();
    let app = server.create_router();

    // Bind and serve
    let addr = format!("{}:{}", args.host, args.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("✅ Nuclear MCP Server running on http://{}", addr);
    println!("📡 Endpoints:");
    println!("   GET  /health           - Health check");
    println!("   POST /mcp/tools/list   - List available tools");
    println!("   POST /mcp/tools/call   - Execute a tool");
    println!("   POST /mcp/rpc          - Generic JSON-RPC 2.0");
    println!("");
    println!("🔧 Available tools:");
    println!("   • websearch         - Real-time web search");
    println!("   • premium           - Premium content scraping");
    println!("   • file_search       - Advanced file search");
    println!("   • scan              - Workspace analysis");
    println!("   • info              - Project information");
    println!("");

    axum::Server::from_tcp(listener.into_std()?)
        .unwrap()
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
