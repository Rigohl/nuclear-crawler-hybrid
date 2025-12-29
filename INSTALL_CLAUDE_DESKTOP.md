# Nuclear MCP - Claude Desktop Installation

## 📦 Contents
- `nuclear-mcp` - Compiled MCP server (Linux binary)
- `mcp.json` - MCP configuration
- This guide

## 🚀 Installation Steps

### Option 1: Linux/macOS
1. Copy `nuclear-mcp` to: `~/.local/bin/nuclear-mcp`
   ```bash
   mkdir -p ~/.local/bin
   cp nuclear-mcp ~/.local/bin/
   chmod +x ~/.local/bin/nuclear-mcp
   ```

2. Update Claude Desktop config at `~/.config/Claude/claude_desktop_config.json`:
   ```json
   {
     "mcpServers": {
       "nuclear-crawler": {
         "command": "~/.local/bin/nuclear-mcp"
       }
     }
   }
   ```

3. Restart Claude Desktop

### Option 2: Windows
1. Copy `nuclear-mcp` to: `C:\Users\<YourUsername>\AppData\Local\Programs\nuclear-mcp\`
   (or any accessible location)

2. Update Claude Desktop config at `%APPDATA%\Claude\claude_desktop_config.json`:
   ```json
   {
     "mcpServers": {
       "nuclear-crawler": {
         "command": "C:\\Path\\To\\nuclear-mcp"
       }
     }
   }
   ```

3. Restart Claude Desktop

## ✅ Verification

After installation, you should see in Claude Desktop:
- Tool: `websearch` - Web search via HTTP
- Tool: `file_search` - File system search
- Tool: `deepweb_search` - Deep web search (Tor)
- Tool: `premium_content_scraper` - Content scraping with WAF bypass

## 🔍 Testing

Send this to Claude:
```
Use the websearch tool to search for "artificial intelligence"
```

Expected: Claude will search and return results.

## 📝 mcp.json Reference

The configuration includes:
- **websearch**: HTTP-based web search (real implementation)
- **file_search**: Filesystem search (real implementation)
- **deepweb_search**: Tor-based deep web search (real implementation)
- **premium_content_scraper**: Content scraping with anti-WAF (real implementation)

## ⚙️ Advanced

To modify tool behavior, edit parameters in prompts or use:
```json
{
  "mcpServers": {
    "nuclear-crawler": {
      "command": "nuclear-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

## 🆘 Troubleshooting

**Tools not showing:**
- Restart Claude Desktop completely
- Check config file exists and is valid JSON
- Verify binary path is correct

**Server connection errors:**
- Ensure `nuclear-mcp` is executable: `chmod +x nuclear-mcp`
- Check logs in Claude Desktop console

**Tool execution fails:**
- Tools require internet connection for websearch/deepweb
- File search needs read permissions on target directories

## 📚 Documentation

- Main docs: `docs/INDEX.md`
- MCP validation: `MCP_REAL_VALIDATION.md`
- Tools reference: `MCP_TOOLS_REFERENCE.md`

---

**Status**: ✅ Production-ready
**All Tools**: ✅ Real (no mocks/simulations)
**Compiled**: Dec 29, 2025
