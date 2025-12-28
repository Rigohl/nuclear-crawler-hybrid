# 🔧 MCP Server Configuration Guide

## Nuclear Crawler Hybrid MCP Server Setup

Configuración del servidor MCP para **VS Code**, **Cursor**, **Codeium (Windsurf)** y **Claude Desktop**.

---

## 📋 Server Details

```json
{
  "type": "http",
  "url": "http://127.0.0.1:8079",
  "tools": [
    "websearch",           // 5s timeout, 50 queries max
    "deepweb_search",      // 10s timeout, 20 queries max
    "premium_content_scraper", // 15s timeout, 20 queries max
    "file_search"          // 8s timeout, 10 queries max
  ]
}
```

---

## 🖥️ VS Code Configuration

### Location: 
- **Windows**: `%APPDATA%\Code\User\settings.json`
- **macOS**: `~/Library/Application Support/Code/User/settings.json`
- **Linux**: `~/.config/Code/User/settings.json`

### Add to settings.json:
```json
{
  "modelContextProtocol.servers": {
    "nuclear-crawler-hybrid": {
      "type": "http",
      "url": "http://127.0.0.1:8079"
    }
  }
}
```

---

## 🤖 Cursor Configuration

### Location:
- **Windows**: `%APPDATA%\Cursor\User\settings.json`
- **macOS**: `~/Library/Application Support/Cursor/User/settings.json`
- **Linux**: `~/.config/Cursor/User/settings.json`

### Add to settings.json:
```json
{
  "modelContextProtocol.servers": {
    "nuclear-crawler-hybrid": {
      "type": "http",
      "url": "http://127.0.0.1:8079"
    }
  }
}
```

---

## 🌬️ Windsurf (Codeium) Configuration

### Location:
- **Windows**: `%APPDATA%\Windsurf\User\settings.json`
- **macOS**: `~/Library/Application Support/Windsurf/User/settings.json`
- **Linux**: `~/.config/Windsurf/User/settings.json`

### Add to settings.json:
```json
{
  "modelContextProtocol.servers": {
    "nuclear-crawler-hybrid": {
      "type": "http",
      "url": "http://127.0.0.1:8079"
    }
  }
}
```

---

## 🤖 Claude Desktop Configuration

### Location:
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Linux**: `~/.config/Claude/claude_desktop_config.json`

### Configuration:
```json
{
  "mcpServers": {
    "nuclear-crawler-hybrid": {
      "type": "http",
      "url": "http://127.0.0.1:8079"
    }
  }
}
```

---

## 🚀 Starting the MCP Server

From the nuclear-crawler-hybrid repository:

```bash
# Development mode (outputs logs)
cargo run --bin nuclear-mcp

# Release mode (optimized)
cargo build --release
./target/release/nuclear-mcp

# Background (detached)
cargo run --bin nuclear-mcp > /tmp/mcp.log 2>&1 &
```

---

## ✅ Verification

### Check if server is running:
```bash
curl http://127.0.0.1:8079/
```

Expected response:
```json
{
  "status": "MCP server online",
  "tools": ["websearch", "deepweb_search", "premium_content_scraper", "file_search"],
  "timeouts": {
    "websearch": 5,
    "deepweb_search": 10,
    "premium_content_scraper": 15,
    "file_search": 8
  }
}
```

---

## 🔗 Tool Usage Examples

### WebSearch
```json
{
  "name": "websearch",
  "arguments": {
    "queries": ["machine learning 2024", "rust async"]
  }
}
```

### DeepWeb Search
```json
{
  "name": "deepweb_search",
  "arguments": {
    "queries": ["privacy guides", "cryptocurrency"]
  }
}
```

### Premium Content Scraper
```json
{
  "name": "premium_content_scraper",
  "arguments": {
    "queries": ["rust books", "machine learning papers"]
  }
}
```

### File Search
```json
{
  "name": "file_search",
  "arguments": {
    "queries": ["async fn", "unwrap()", "TODO"]
  }
}
```

---

## 🐛 Troubleshooting

### Server not connecting
1. Verify port 8079 is available: `lsof -i :8079` (macOS/Linux) or `netstat -ano | findstr :8079` (Windows)
2. Check firewall settings
3. Restart the server: `cargo run --bin nuclear-mcp`

### Tools not showing
1. Verify server is responding: `curl http://127.0.0.1:8079/tools/list`
2. Restart the IDE
3. Check IDE MCP settings

### Slow responses
1. Check network latency: `ping 127.0.0.1`
2. Monitor server logs
3. Verify timeouts in tool definitions

---

## 📝 Notes

- Server runs on **localhost:8079**
- Input restricted to `queries` array of strings only
- Timeouts enforced per tool
- No external parameters allowed (strict validation)
- Results cached in memory
- Rate limiting active

---

**Last Updated**: December 28, 2025
**Version**: 0.1.0
