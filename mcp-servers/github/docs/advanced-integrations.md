# Advanced Integrations Guide

This guide covers advanced integration scenarios for github-mcp-server, including Copilot CLI, .NET agents, and PowerShell 2026 enhancements.

## Table of Contents

1. [GitHub Copilot CLI Integration](#github-copilot-cli-integration)
2. [.NET Agents Integration](#net-agents-integration)
3. [PowerShell 2026 Enhancements](#powershell-2026-enhancements)
4. [Multi-Agent Workflows](#multi-agent-workflows)

---

## GitHub Copilot CLI Integration

### Overview

GitHub Copilot CLI brings AI-powered coding assistance directly to your terminal. It ships with GitHub's MCP server by default and supports custom MCP servers.

**Repository**: [github/copilot-cli](https://github.com/github/copilot-cli)

### Installation

```bash
# Windows
winget install GitHub.Copilot

# macOS/Linux
brew install copilot-cli

# npm (cross-platform)
npm install -g @github/copilot
```

### Integration with github-mcp-server

Copilot CLI automatically uses the GitHub MCP server when available. To use a local instance:

1. **Configure Copilot CLI to use local MCP server**

   Create or edit `~/.copilot/mcp.json`:

   ```json
   {
     "servers": {
       "github": {
         "command": "docker",
         "args": [
           "run", "-i", "--rm",
           "-e", "GITHUB_PERSONAL_ACCESS_TOKEN",
           "ghcr.io/github/github-mcp-server"
         ],
         "env": {
           "GITHUB_PERSONAL_ACCESS_TOKEN": "${env:GH_TOKEN}"
         }
       }
     }
   }
   ```

2. **Use Copilot CLI with GitHub context**

   ```bash
   # Launch Copilot CLI
   copilot
   
   # Example prompts:
   # "Show me all open issues in my repository"
   # "Create a pull request for the current branch"
   # "Review the latest commit"
   ```

### Advanced Configuration

**Custom Toolsets**:

```json
{
  "servers": {
    "github": {
      "command": "docker",
      "args": [
        "run", "-i", "--rm",
        "-e", "GITHUB_PERSONAL_ACCESS_TOKEN",
        "-e", "GITHUB_TOOLSETS",
        "ghcr.io/github/github-mcp-server"
      ],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "${env:GH_TOKEN}",
        "GITHUB_TOOLSETS": "repos,issues,pull_requests,actions"
      }
    }
  }
}
```

---

## .NET Agents Integration

### Overview

Microsoft provides comprehensive .NET tooling for building agents with MCP support. You can consume existing MCP servers or create custom ones.

### Prerequisites

- .NET 9 SDK or later
- Microsoft.Agents.AI NuGet package
- MCP C# SDK

### Consuming github-mcp-server from .NET

**1. Install Required Packages**:

```xml
<PackageReference Include="Microsoft.Agents.AI" Version="1.0.0" />
<PackageReference Include="Microsoft.Extensions.Http" Version="9.0.0" />
```

**2. Create MCP Client**:

```csharp
using Microsoft.Agents.AI;
using Microsoft.Agents.AI.Mcp;

var mcpClient = new McpClient(new HttpClient())
{
    ServerUrl = "http://localhost:8080/mcp",
    ApiKey = Environment.GetEnvironmentVariable("GITHUB_PERSONAL_ACCESS_TOKEN")
};

await mcpClient.ConnectAsync();
```

**3. Use GitHub Tools in .NET Agent**:

```csharp
using Microsoft.Agents.AI;
using Microsoft.Agents.AI.Mcp;

public class GitHubAgent
{
    private readonly McpClient _mcpClient;
    
    public GitHubAgent(McpClient mcpClient)
    {
        _mcpClient = mcpClient;
    }
    
    public async Task<string> GetRepositoryInfo(string owner, string repo)
    {
        var result = await _mcpClient.CallToolAsync("get_file_contents", new
        {
            owner = owner,
            repo = repo,
            path = "README.md"
        });
        
        return result.ToString();
    }
    
    public async Task CreateIssue(string owner, string repo, string title, string body)
    {
        await _mcpClient.CallToolAsync("issue_write", new
        {
            owner = owner,
            repo = repo,
            method = "create",
            title = title,
            body = body
        });
    }
}
```

### Creating .NET MCP Server

**Expose .NET Agent as MCP Tool**:

```csharp
using Microsoft.Agents.AI;
using Microsoft.Agents.AI.Mcp;

public class CustomMcpServer : McpServer
{
    public CustomMcpServer()
    {
        // Expose your .NET agent as an MCP tool
        RegisterTool("analyze_code", new McpTool
        {
            Name = "analyze_code",
            Description = "Analyzes code using .NET static analysis",
            InputSchema = new
            {
                type = "object",
                properties = new
                {
                    code = new { type = "string", description = "Code to analyze" },
                    language = new { type = "string", description = "Programming language" }
                },
                required = new[] { "code" }
            }
        });
    }
    
    protected override async Task<object> HandleToolCall(string toolName, object parameters)
    {
        if (toolName == "analyze_code")
        {
            // Your .NET agent logic here
            return await AnalyzeCode(parameters);
        }
        
        throw new NotSupportedException($"Tool {toolName} not supported");
    }
}
```

### Azure Integration

**Deploy .NET MCP Agent to Azure Container Apps**:

```bash
# Use Azure Developer CLI template
azd init --template dotnet-mcp-agent

# Deploy
azd up
```

---

## PowerShell 2026 Enhancements

### Overview

PowerShell's AI Shell (Preview 6+) includes native MCP support, allowing you to connect any MCP server, including github-mcp-server.

### AI Shell Configuration

**1. Install AI Shell** (if not already installed):

```powershell
# Install from PowerShell Gallery
Install-Module -Name AIShell -Scope CurrentUser -Force
```

**2. Configure MCP Server**:

Create `$HOME\.aish\mcp.json`:

```json
{
  "servers": {
    "github": {
      "command": "docker",
      "args": [
        "run", "-i", "--rm",
        "-e", "GITHUB_PERSONAL_ACCESS_TOKEN",
        "ghcr.io/github/github-mcp-server"
      ],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "$env:GH_TOKEN"
      }
    }
  }
}
```

**3. Use AI Shell with GitHub**:

```powershell
# Launch AI Shell
aish

# Example prompts:
# "Show me all open pull requests in my repository"
# "Create an issue for the bug I just found"
# "What's the status of workflow runs?"
```

### Advanced PowerShell Integration

**PowerShell Module for GitHub MCP**:

```powershell
# powershell/GitHubMcp.psm1
function Invoke-GitHubMcpTool {
    param(
        [Parameter(Mandatory)]
        [string]$ToolName,
        
        [Parameter(Mandatory)]
        [hashtable]$Parameters
    )
    
    $mcpClient = Get-McpClient -ServerName "github"
    return $mcpClient.InvokeTool($ToolName, $Parameters)
}

function Get-GitHubIssues {
    param(
        [Parameter(Mandatory)]
        [string]$Owner,
        
        [Parameter(Mandatory)]
        [string]$Repo,
        
        [string]$State = "open"
    )
    
    return Invoke-GitHubMcpTool -ToolName "list_issues" -Parameters @{
        owner = $Owner
        repo = $Repo
        state = $State
    }
}

function New-GitHubIssue {
    param(
        [Parameter(Mandatory)]
        [string]$Owner,
        
        [Parameter(Mandatory)]
        [string]$Repo,
        
        [Parameter(Mandatory)]
        [string]$Title,
        
        [string]$Body
    )
    
    return Invoke-GitHubMcpTool -ToolName "issue_write" -Parameters @{
        owner = $Owner
        repo = $Repo
        method = "create"
        title = $Title
        body = $Body
    }
}

Export-ModuleMember -Function Get-GitHubIssues, New-GitHubIssue
```

**Usage**:

```powershell
# Import module
Import-Module ./powershell/GitHubMcp.psm1

# Get issues
Get-GitHubIssues -Owner "octocat" -Repo "Hello-World"

# Create issue
New-GitHubIssue -Owner "octocat" -Repo "Hello-World" `
    -Title "Bug: Something broken" `
    -Body "Description of the bug"
```

### PowerShell 2026 Features

**Built-in Tools Available in AI Shell**:

- `get_working_directory` - Current directory info
- `get_command_history` - Recent commands
- `get_terminal_content` - Terminal output
- `get_environment_variables` - Environment vars
- `copy_text_to_clipboard` - Clipboard integration
- `post_code_to_terminal` - Code insertion

**Sidecar Mode**:

AI Shell can run in Windows Terminal sidecar mode for deeper integration:

```powershell
# Launch in sidecar mode
aish --sidecar

# Features:
# - Direct code insertion from AI responses
# - Multi-step command support
# - Error recovery
# - Context-aware suggestions
```

---

## Multi-Agent Workflows

### Orchestrating Multiple Agents

You can orchestrate multiple agents (Copilot CLI, .NET agents, PowerShell) to work together:

**Example Workflow**:

```powershell
# PowerShell orchestrator script
function Invoke-MultiAgentWorkflow {
    param(
        [string]$Task
    )
    
    # 1. Use Copilot CLI to analyze code
    $analysis = copilot "Analyze this codebase for security issues"
    
    # 2. Use .NET agent to process results
    $processed = Invoke-DotNetAgent -Input $analysis -Action "categorize"
    
    # 3. Use GitHub MCP to create issues
    foreach ($issue in $processed.Issues) {
        New-GitHubIssue -Owner "org" -Repo "repo" `
            -Title $issue.Title `
            -Body $issue.Description
    }
    
    # 4. Use PowerShell AI Shell for follow-up
    aish "Review the issues I just created and suggest improvements"
}
```

### Best Practices

1. **Use appropriate agent for each task**:
   - Copilot CLI: Code analysis, refactoring
   - .NET Agents: Business logic, data processing
   - PowerShell: System automation, file operations
   - GitHub MCP: Repository management

2. **Maintain context between agents**:
   - Use shared state/storage
   - Pass structured data (JSON)
   - Log interactions for debugging

3. **Error handling**:
   - Validate inputs between agents
   - Implement retry logic
   - Provide fallback mechanisms

---

## Resources

- [GitHub Copilot CLI Documentation](https://docs.github.com/copilot/concepts/agents/about-copilot-cli)
- [.NET AI Agents Documentation](https://learn.microsoft.com/dotnet/ai/get-started-mcp)
- [PowerShell AI Shell Documentation](https://learn.microsoft.com/powershell/utility-modules/aishell/overview)
- [MCP Protocol Specification](https://modelcontextprotocol.io)

