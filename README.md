# 🔥 Nuclear Crawler Hybrid

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![MCP](https://img.shields.io/badge/MCP-2025--01--01-blue.svg)](https://modelcontextprotocol.io)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-green.svg)](LICENSE)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://hub.docker.com)

**High-performance Model Context Protocol (MCP) server with advanced web search, deep web exploration, premium content scraping, and intelligent file analysis capabilities.**

⚡ **100K parallel goroutines** | 🌐 **55+ search engines** | ⏱️ **<2s query completion** | 📊 **2100+ URLs per search**

---

## 🚀 Quick Start

### Using Docker (Recommended)

```bash
# Pull and run the server
docker-compose up -d

# Server will be available at http://localhost:8079
```

### From Source

```bash
# Build optimized release
cargo build --release

# Run MCP server
./target/release/nuclear-mcp --port 8079
```

### Connect to Your AI Assistant

Add to your MCP client configuration (VS Code, Cursor, Windsurf, Claude Desktop):

```json
{
  "mcpServers": {
    "nuclear-crawler-hybrid": {
      "url": "http://127.0.0.1:8079"
    }
  }
}
```

See [Configuration](#-configuration) for detailed setup instructions.

---

## 📐 Architecture

Nuclear Crawler Hybrid is built on a modern, high-performance architecture:

- **MCP Protocol 2025-01-01**: HTTP-only transport with Server-Sent Events (SSE)
- **Rust Core**: Async runtime with Tokio and Axum web framework
- **FFI Integration**: Go (100K goroutines), Zig (SIMD processing), Nim (HTML parsing)
- **Docker Support**: Multi-platform containers (linux/amd64, linux/arm64)

```
┌─────────────────────────────────────────┐
│   MCP Client (VS Code, Cursor, etc)    │
└────────────────┬────────────────────────┘
                 │ HTTP/SSE
┌────────────────▼────────────────────────┐
│     Nuclear MCP Server (Rust/Axum)     │
│  ┌──────────┬──────────┬──────────┐    │
│  │ WebSearch│ DeepWeb  │ Premium  │    │
│  │   Tool   │   Tool   │   Tool   │    │
│  └──────────┴──────────┴──────────┘    │
│  ┌──────────────────────────────────┐  │
│  │    File Search & Analysis Tool   │  │
│  └──────────────────────────────────┘  │
└───┬──────────┬──────────┬──────────┬───┘
    │          │          │          │
┌───▼───┐  ┌──▼───┐  ┌───▼───┐  ┌──▼───┐
│  Go   │  │ Zig  │  │  Nim  │  │ Jax  │
│  FFI  │  │ SIMD │  │ HTML  │  │ Accel│
└───────┘  └──────┘  └───────┘  └──────┘
```

For detailed architecture documentation, see [ARCHITECTURE.md](ARCHITECTURE.md).

---

## ✨ Features

### 🌐 Web Search Tool (`websearch`)
- **55+ search engines**: DuckDuckGo, Bing, Brave, Yandex, Ecosia, SearX, and more
- **Repository search**: GitHub, GitLab, Codeberg, Gitee, BitBucket, SourceForge
- **Community forums**: Stack Overflow, Reddit, Dev.to, Medium, Hashnode
- **Package registries**: crates.io, npm, PyPI, docs.rs
- **Academic sources**: arXiv, Papers with Code, HuggingFace
- **Configuration**: Max 50 queries, 5-second timeout per request

### 🕵️ Deep Web Search Tool (`deepweb_search`)
- **Tor integration**: Anonymous browsing and .onion domain support
- **Stealth mode**: Randomized headers and fingerprint obfuscation
- **Underground sources**: Hidden wikis, forums, and marketplaces
- **Configuration**: Max 20 queries, 10-second timeout per request

### 📰 Premium Content Scraper (`premium_content_scraper`)
- **Paywall bypass**: Medium, ArXiv, research papers, and academic content
- **Smart extraction**: Article text, metadata, citations, and references
- **Multi-format support**: HTML, PDF, academic formats
- **Configuration**: Max 20 URLs, 15-second timeout per request

### 🔍 File Search Tool (`file_search`)
- **Zig SIMD acceleration**: Ultra-fast pattern matching
- **Advanced analysis**: Code complexity, circular imports, duplicates
- **Semantic search**: Fuzzy matching and intelligent suggestions
- **Precise results**: Line numbers, context, and automatic edit suggestions
- **Configuration**: Max 10 searches, 8-second timeout per request

For complete tool documentation, see [MCP_TOOLS_REFERENCE.md](MCP_TOOLS_REFERENCE.md).

---

## 💻 Installation

### Prerequisites

- **Rust 1.75+**: Install from [rustup.rs](https://rustup.rs)
- **Docker** (optional): For containerized deployment
- **FFI Dependencies** (optional): Go 1.21+, Zig 0.11+, Nim 2.0+ for maximum performance

### Windows

```powershell
# Using PowerShell
.\setup_windows.ps1

# Or using batch script
setup_windows.bat
```

### Linux/macOS

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the server
cargo build --release

# Optional: Install FFI dependencies
# See CONTRIBUTING.md for FFI setup instructions
```

### Docker

```bash
# Build multi-platform image
docker build -t nuclear-crawler-hybrid .

# Or use docker-compose
docker-compose up -d
```

For detailed deployment instructions, see [DEPLOYMENT.md](DEPLOYMENT.md).

---

## ⚙️ Configuration

### VS Code

**Location**: `%APPDATA%\Code\User\settings.json` (Windows) or `~/.config/Code/User/settings.json` (Linux)

```json
{
  "modelContextProtocol.servers": {
    "nuclear-crawler-hybrid": {
      "url": "http://127.0.0.1:8079"
    }
  }
}
```

### Cursor

**Location**: `%APPDATA%\Cursor\User\settings.json` (Windows) or `~/Library/Application Support/Cursor/User/settings.json` (macOS)

```json
{
  "mcp": {
    "servers": {
      "nuclear-crawler-hybrid": {
        "url": "http://127.0.0.1:8079"
      }
    }
  }
}
```

### Windsurf (Codeium)

**Location**: `%APPDATA%\Windsurf\User\settings.json` (Windows) or `~/.config/Windsurf/User/settings.json` (Linux)

```json
{
  "codeium.mcpServers": {
    "nuclear-crawler-hybrid": {
      "url": "http://127.0.0.1:8079"
    }
  }
}
```

### Claude Desktop

**Location**: `%APPDATA%\Claude\config.json` (Windows) or `~/Library/Application Support/Claude/config.json` (macOS)

```json
{
  "mcpServers": {
    "nuclear-crawler-hybrid": {
      "url": "http://127.0.0.1:8079"
    }
  }
}
```

For detailed configuration instructions, see [MCP_SETUP_GUIDE.md](MCP_SETUP_GUIDE.md) (archived).

---

## 🛠️ Development

### Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# With FFI features
cargo build --release --features go_integration,zig_integration,nim_integration
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality

```bash
# Format code
cargo fmt

# Linting
cargo clippy -- -D warnings

# Security audit
cargo audit
```

### Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:

- Development setup
- Coding standards
- Testing procedures
- Pull request guidelines

---

## 📚 Documentation

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Technical deep dive into MCP protocol and FFI integration
- **[DEPLOYMENT.md](DEPLOYMENT.md)** - Docker, production setup, and docker-compose usage
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Development guidelines and local setup
- **[MCP_TOOLS_REFERENCE.md](MCP_TOOLS_REFERENCE.md)** - Detailed tool specifications and examples

---

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

## 🙏 Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org) - Systems programming language
- [Tokio](https://tokio.rs) - Asynchronous runtime
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Model Context Protocol](https://modelcontextprotocol.io) - AI tool integration standard

---

**Status**: ✅ Production Ready | **Version**: 0.1.0 | **MCP Protocol**: 2025-01-01
