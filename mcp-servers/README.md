# MCP Servers Integration

This directory contains Model Context Protocol (MCP) servers integrated into the nuclear-crawler-hybrid project.

## Available Servers

### GitHub MCP Server (`github/`)

**Source**: https://github.com/modelcontextprotocol/servers (official GitHub MCP server)

**Description**: Full-featured GitHub integration providing access to repositories, issues, PRs, code search, and more through MCP protocol.

**Features**:
- Repository management (list, create, get details)
- File operations (read, search, push)
- Issues and Pull Requests
- Code search with filters
- Branch and commit operations
- GitHub Actions workflows
- Dependabot and security alerts

**Usage**:
```bash
cd mcp-servers/github
go build -o github-mcp-server
./github-mcp-server
```

**Configuration**: Set `GITHUB_TOKEN` environment variable with your GitHub personal access token.

## Integration with Nuclear Crawler

These MCP servers can be used by:
- The Chapel AI training pipeline for dataset generation
- Rust MCP tools in `src/mcp/`
- OSINT and data mining operations
- Automated workflows in `.github/workflows/`

## Adding New MCP Servers

To add a new MCP server:

1. Create a subdirectory: `mcp-servers/<server-name>/`
2. Copy or clone the server code
3. Add build instructions to this README
4. Update `.gitignore` if needed
5. Document usage and configuration

## Notes

- Each server runs independently
- Servers communicate via stdio MCP protocol
- Can be used with Claude Desktop, Cursor, or other MCP clients
- See individual server directories for specific documentation
