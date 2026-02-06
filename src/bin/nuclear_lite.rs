//! ⚡ Nuclear MCP Lite - Lightweight Binary (2 TOOLS)
//!
//! Fast and minimal: websearch + scan
//! Combine with nuclear-pro and nuclear-mcp for MAX POWER

use clap::Parser;
use nuclear_crawler_hybrid::mcp::protocol::ToolProfile;
use nuclear_crawler_hybrid::mcp::server::MCPServer;

/// Nuclear MCP Lite - Lightweight Server (2 Tools)
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Nuclear MCP Lite - Lightweight (2 Tools: websearch + scan)"
)]
struct Args {
    /// Port to bind the HTTP server
    #[arg(short, long, default_value_t = 8077)]
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
    let profile = ToolProfile::Lite;
    let tool_names = nuclear_crawler_hybrid::mcp::protocol::get_profile_tool_names(profile);

    if args.verbose {
        eprintln!("⚡ Nuclear MCP Lite Starting...");
        eprintln!("   Host: {}", args.host);
        eprintln!("   Port: {}", args.port);
        eprintln!("   Profile: Lite (2 tools)");
        eprintln!("   Tools: {}", tool_names.join(", "));
        eprintln!();
    }

    let server = MCPServer::with_profile(profile);
    let app = server.create_router();

    let addr = format!("{}:{}", args.host, args.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("⚡ Nuclear MCP Lite running on http://{}", addr);
    println!("🔧 Tools ({}):", tool_names.len());
    for name in &tool_names {
        println!("   • {}", name);
    }
    println!();

    axum::serve(listener, app).await?;

    Ok(())
}
