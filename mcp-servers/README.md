# MCP Servers

External MCP server integrations for Nuclear Crawler Hybrid.

## GitHub MCP Server

Location: `github/`

Real Go-based MCP server exposing GitHub API tools over JSON-RPC 2.0 stdio.

### Tools

| Tool | Description |
|------|-------------|
| `search_code` | Search GitHub code across public repos |
| `list_issues` | List issues for a repo |
| `create_issue` | Create a new issue |
| `list_repos` | List repos for a user/org |

### Build & Run

```bash
cd github
export GITHUB_TOKEN="ghp_xxxxx"
make build
./github-mcp-server
```

### Integrate with Claude Desktop

Add to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "github": {
      "command": "/path/to/github-mcp-server",
      "env": {
        "GITHUB_TOKEN": "ghp_xxxxx"
      }
    }
  }
}
```

### Used by Chapel FFI

The `mcp_ffi_bridge.c` calls `mcp_call("github", ...)` which routes to
`https://api.githubcopilot.com/mcp/`. The Go server is for local/stdio usage.
