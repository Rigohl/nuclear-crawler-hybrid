<div align="center">

<img src="https://img.shields.io/badge/Arvix-Nuclear%20Crawler-ff6b35?style=for-the-badge&logo=rust&logoColor=white" alt="Arvix Nuclear Crawler"/>

# 🔥 NUCLEAR CRAWLER HYBRID

### **Enterprise Web Scraping & Search Engine**

[![CI/CD](https://github.com/Rigohl/nuclear-crawler-hybrid/actions/workflows/ci.yml/badge.svg)](https://github.com/Rigohl/nuclear-crawler-hybrid/actions)
[![Security](https://img.shields.io/badge/Security-Audited-success?style=flat-square)](https://github.com/Rigohl/nuclear-crawler-hybrid/security)
[![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.83+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/Version-0.5.0-green?style=flat-square)](https://github.com/Rigohl/nuclear-crawler-hybrid/releases)
[![Docker](https://img.shields.io/badge/Docker-Ready-2496ED?style=flat-square&logo=docker&logoColor=white)](https://ghcr.io/rigohl/nuclear-crawler-hybrid)

**Rust** · **Go FFI** · **Zig SIMD** · **Stealth** · **MCP Protocol**

[🚀 Quick Start](#-quick-start) · [✨ Features](#-features) · [📦 Installation](#-installation) · [🔧 Usage](#-usage) · [📚 Docs](#-documentation)

---

<a href="https://github.com/Rigohl">
  <img src="https://img.shields.io/badge/Powered%20by-Arvix-ff6b35?style=for-the-badge" alt="Powered by Arvix"/>
</a>

</div>

## 🚀 Quick Start

```bash
# Clone
git clone https://github.com/Rigohl/nuclear-crawler-hybrid.git
cd nuclear-crawler-hybrid

# Build
cargo build --release

# Run
./target/release/nuclear-mcp --help
```

## ✨ Features

<table>
<tr>
<td width="50%">

### 🔥 Core Engine
- **2000** concurrent connections
- **10K** requests/second throughput
- Async Rust with Tokio runtime
- Connection pooling & reuse
- Smart retry with exponential backoff

</td>
<td width="50%">

### 🛡️ Stealth System
- Browser fingerprint rotation
- TLS fingerprint randomization
- User-Agent rotation (500+ agents)
- Request timing randomization
- Anti-bot detection bypass

</td>
</tr>
<tr>
<td width="50%">

### ⚡ FFI Integrations
- **Go FFI** - Goroutines for parallelism
- **Zig FFI** - SIMD HTML parsing
- Native performance, zero overhead
- Cross-platform compatibility

</td>
<td width="50%">

### 🔓 Nuclear Bypass
- LibGen integration
- Sci-Hub connector
- Anna's Archive support
- Internet Archive fallback
- Multi-source aggregation

</td>
</tr>
</table>

## 📦 Installation

### Pre-built Binaries

| Platform | Download |
|----------|----------|
| Windows x64 | [nuclear-windows-x64.zip](https://github.com/Rigohl/nuclear-crawler-hybrid/releases/latest) |
| Linux x64 | [nuclear-linux-x64.tar.gz](https://github.com/Rigohl/nuclear-crawler-hybrid/releases/latest) |
| Linux ARM64 | [nuclear-linux-arm64.tar.gz](https://github.com/Rigohl/nuclear-crawler-hybrid/releases/latest) |
| macOS x64 | [nuclear-macos-x64.tar.gz](https://github.com/Rigohl/nuclear-crawler-hybrid/releases/latest) |
| macOS ARM64 | [nuclear-macos-arm64.tar.gz](https://github.com/Rigohl/nuclear-crawler-hybrid/releases/latest) |

### Docker

```bash
docker pull ghcr.io/rigohl/nuclear-crawler-hybrid:latest
docker run -it ghcr.io/rigohl/nuclear-crawler-hybrid:latest
```

### From Source

```bash
# Prerequisites: Rust 1.83+
cargo install --path .

# Or build release binary
cargo build --release --bin nuclear-mcp
```

## 🔧 Usage

### MCP Server Mode

Configure in your MCP client (Claude Desktop, VS Code, etc.):

```json
{
  "mcpServers": {
    "nuclear": {
      "command": "nuclear-mcp",
      "args": ["--stdio"]
    }
  }
}
```

### Available Tools

| Tool | Description | Example |
|------|-------------|---------|
| `websearch` | Massive parallel web search | Search 100+ sources simultaneously |
| `deep_web_search` | Deep web with bypass | Access paywalled content |
| `crawl` | Recursive crawling | Spider entire domains |
| `scrape` | Extract structured data | Parse HTML/JSON/XML |
| `analizar_proyecto` | Project analysis | Get recommendations |
| `stats` | System statistics | Monitor performance |

### HTTP API Mode

```bash
# Start HTTP server
./nuclear-http --port 8080

# Search endpoint
curl -X POST http://localhost:8080/search \
  -H "Content-Type: application/json" \
  -d '{"query": "rust async", "max_results": 50}'
```

## ⚙️ Configuration

### Environment Variables

```bash
# Performance
NUCLEAR_MAX_CONCURRENT=2000    # Max parallel requests
NUCLEAR_TIMEOUT=180            # Request timeout (seconds)
NUCLEAR_CACHE_SIZE=5000000     # Cache entries

# Features
NUCLEAR_STEALTH=true           # Enable stealth mode
NUCLEAR_BYPASS=true            # Enable paywall bypass

# Logging
RUST_LOG=info                  # Log level
```

### Performance Profiles

```rust
// NUCLEAR Mode (Default) - Maximum Performance
NuclearConfig {
    max_concurrent: 2000,
    max_requests_per_second: 10000,
    timeout_seconds: 180,
    burst_size: 5000,
    cache_size: 5_000_000,
}

// Stealth Mode - Undetectable
StealthConfig {
    max_concurrent: 50,
    request_delay_ms: 500..2000,
    fingerprint_rotation: true,
}
```

## 🏗️ Architecture

```
nuclear-crawler-hybrid/
├── src/
│   ├── lib.rs                 # Core library
│   ├── nuclear_scraper.rs     # Scraper engine
│   ├── web_search.rs          # Search with Go/Zig
│   ├── deep_web_search.rs     # Bypass system
│   ├── nuclear_bypass.rs      # Paywall bypass
│   ├── stealth.rs             # Anti-detection
│   ├── go_integration.rs      # Go FFI bindings
│   ├── zig_integration.rs     # Zig FFI bindings
│   └── improvements.rs        # Enterprise features
├── go/                        # Go modules
├── zig/                       # Zig modules
├── scripts/                   # JAX/Python ML
└── .github/workflows/         # CI/CD
```

## 📊 Benchmarks

| Metric | Value | vs Others |
|--------|-------|-----------|
| URLs/query | 200+ | 50x faster |
| Concurrent | 2000 | Enterprise |
| Req/second | 10K | NUCLEAR |
| Cache | 5M entries | Zero I/O |
| Startup | <100ms | Instant |

## 📚 Documentation

- [Configuration Guide](docs/CONFIG.md)
- [Contributing](CONTRIBUTING.md)
- [Security Policy](SECURITY.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)

## 🔒 Security

- ✅ Automated security audits via GitHub Actions
- ✅ Dependency scanning with `cargo-audit`
- ✅ TLS 1.3 enforced for all connections
- ✅ No sensitive data storage
- ✅ Rootless container execution

Report vulnerabilities: [SECURITY.md](SECURITY.md)

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Development setup
git clone https://github.com/Rigohl/nuclear-crawler-hybrid.git
cd nuclear-crawler-hybrid
cargo test --all-features
cargo clippy
```

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

<div align="center">

**Built with 🔥 by [Arvix](https://github.com/Rigohl)**

[![GitHub Stars](https://img.shields.io/github/stars/Rigohl/nuclear-crawler-hybrid?style=social)](https://github.com/Rigohl/nuclear-crawler-hybrid)
[![GitHub Forks](https://img.shields.io/github/forks/Rigohl/nuclear-crawler-hybrid?style=social)](https://github.com/Rigohl/nuclear-crawler-hybrid/fork)

[⬆ Back to Top](#-nuclear-crawler-hybrid)

</div>
