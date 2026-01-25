# .NET GitHub MCP Agent

.NET 9 agent for interacting with GitHub MCP Server.

## Prerequisites

- .NET 9 SDK or later
- GitHub Personal Access Token

## Build

```bash
dotnet build
```

## Configuration

Set environment variables:

```bash
export GITHUB_PERSONAL_ACCESS_TOKEN=your_token_here
export GITHUB_MCP_SERVER_URL=http://localhost:8080/mcp  # Optional
```

## Usage

### As a Library

```csharp
using GitHubMcpAgent;

var httpClient = new HttpClient();
var logger = LoggerFactory.Create(b => b.AddConsole()).CreateLogger<GitHubMcpAgent>();
var agent = new GitHubMcpAgent(
    httpClient,
    logger,
    "http://localhost:8080/mcp",
    Environment.GetEnvironmentVariable("GITHUB_PERSONAL_ACCESS_TOKEN")
);

// Get file contents
var contents = await agent.GetFileContentsAsync("octocat", "Hello-World", "README.md");

// List issues
var issues = await agent.ListIssuesAsync("octocat", "Hello-World", "open");

// Create issue
await agent.CreateIssueAsync("octocat", "Hello-World", "Bug Report", "Description");
```

### As a Standalone Application

```bash
dotnet run octocat Hello-World README.md
```

### Database Inspection (DuckDB)

The agent includes built-in support for inspecting local DuckDB databases (e.g., `d:\DATABASES\master_database.duckdb`).

```bash
# List all tables in the default database
dotnet run inspect-db

# Execute a specific SQL query
dotnet run query-db "SELECT * FROM development_tools LIMIT 5"
```

## Integration with Azure

See [Microsoft's documentation](https://learn.microsoft.com/azure/developer/ai/build-openai-mcp-server-dotnet) for deploying to Azure Container Apps.

## API Reference

### Methods

- `GetFileContentsAsync(string owner, string repo, string path, string? ref = null)`
- `ListIssuesAsync(string owner, string repo, string? state = null)`
- `CreateIssueAsync(string owner, string repo, string title, string body)`
- `GetPullRequestAsync(string owner, string repo, int pullNumber)`
- `ListWorkflowRunsAsync(string owner, string repo, string workflowId)`

## Examples

See `Program.cs` for a complete example.

