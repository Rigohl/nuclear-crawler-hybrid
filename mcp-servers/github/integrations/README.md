# Advanced Integrations

This directory contains advanced integration examples and tools for github-mcp-server.

## Directory Structure

```
integrations/
├── copilot-cli/          # GitHub Copilot CLI integration
│   └── mcp.json          # MCP configuration for Copilot CLI
├── dotnet/               # .NET agent integration
│   ├── GitHubMcpAgent.cs # .NET agent implementation
│   └── GitHubMcpAgent.csproj
├── powershell/           # PowerShell 2026 integration
│   ├── GitHubMcp.psm1   # PowerShell module
│   └── README.md        # PowerShell usage guide
└── README.md            # This file
```

## Quick Start

### Copilot CLI

1. Copy `copilot-cli/mcp.json` to `~/.copilot/mcp.json`
2. Set `GH_TOKEN` environment variable
3. Launch `copilot` CLI

### .NET Agent

1. Navigate to `dotnet/` directory
2. Run `dotnet build`
3. Use `GitHubMcpAgent` class in your .NET application

### PowerShell

1. Import module: `Import-Module ./powershell/GitHubMcp.psm1`
2. Set `$env:GITHUB_PERSONAL_ACCESS_TOKEN`
3. Use cmdlets: `Get-GitHubIssues`, `New-GitHubIssue`, etc.

## Documentation

See [docs/advanced-integrations.md](../docs/advanced-integrations.md) for detailed documentation.

## Examples

### Multi-Agent Workflow

```powershell
# PowerShell orchestrator
$issues = Get-GitHubIssues -Owner "org" -Repo "repo" -State "open"
foreach ($issue in $issues) {
    # Use Copilot CLI to analyze
    $analysis = copilot "Analyze this issue: $($issue.title)"
    
    # Use .NET agent to process
    $processed = Invoke-DotNetAgent -Input $analysis
    
    # Update issue with results
    # ...
}
```

## Contributing

When adding new integrations:

1. Create a new directory under `integrations/`
2. Include a README.md with usage instructions
3. Add examples and configuration files
4. Update this README.md

