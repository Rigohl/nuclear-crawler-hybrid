# GitHub Copilot CLI Integration

Configuration for integrating github-mcp-server with GitHub Copilot CLI.

## Setup

1. **Install Copilot CLI** (if not already installed):

   ```bash
   # Windows
   winget install GitHub.Copilot
   
   # macOS/Linux
   brew install copilot-cli
   
   # npm (cross-platform)
   npm install -g @github/copilot
   ```

2. **Copy MCP configuration**:

   ```bash
   # Create Copilot CLI config directory
   mkdir -p ~/.copilot
   
   # Copy configuration
   cp mcp.json ~/.copilot/mcp.json
   ```

3. **Set environment variable**:

   ```bash
   export GH_TOKEN=your_github_token_here
   ```

   Or on Windows:

   ```powershell
   $env:GH_TOKEN = "your_github_token_here"
   ```

## Usage

Launch Copilot CLI:

```bash
copilot
```

Now you can use natural language to interact with GitHub:

- "Show me all open issues in my repository"
- "Create a pull request for the current branch"
- "What's the status of the latest workflow run?"
- "Review the code changes in the last commit"

## Customization

Edit `~/.copilot/mcp.json` to customize:

- **Toolsets**: Change `GITHUB_TOOLSETS` to enable specific tool groups
- **Read-only mode**: Set `GITHUB_READ_ONLY=1` for read-only access
- **Custom tools**: Add specific tools via `GITHUB_TOOLS` environment variable

## Examples

### Basic Usage

```bash
# Launch Copilot CLI
copilot

# In the CLI:
> Show me all open issues in octocat/Hello-World
> Create an issue titled "Bug: Login not working"
> What files changed in the last commit?
```

### Advanced Workflows

```bash
# Multi-step workflow
copilot

> Analyze the codebase for security issues
> Create issues for each vulnerability found
> Open a pull request with security fixes
```

## Troubleshooting

### Copilot CLI not finding MCP server

- Verify `~/.copilot/mcp.json` exists and is valid JSON
- Check that `GH_TOKEN` or `GITHUB_TOKEN` is set
- Ensure Docker is running (if using Docker-based MCP server)

### Authentication issues

- Verify your GitHub token has the required scopes
- Check token expiration
- Re-authenticate: `/login` in Copilot CLI

## Resources

- [Copilot CLI Documentation](https://docs.github.com/copilot/concepts/agents/about-copilot-cli)
- [GitHub MCP Server Documentation](../README.md)

