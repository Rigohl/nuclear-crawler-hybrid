# PowerShell AI Shell Advanced Integration Module
# Integración avanzada con PowerShell AI Shell basada en documentación oficial
# Version: 1.0.0 - Advanced AI Shell features

$ErrorActionPreference = 'Stop'

# Configuración de AI Shell
$script:AIShellConfigPath = "$HOME\.aish"
$script:AIShellMcpConfig = "$script:AIShellConfigPath\mcp.json"
$script:AIShellCustomInstructions = "$script:AIShellConfigPath\custom-instructions.md"

function Initialize-AIShellIntegration {
    <#
    .SYNOPSIS
    Inicializa integración completa con AI Shell
    #>
    [CmdletBinding()]
    param()
    
    Write-Host "Initializing AI Shell integration..." -ForegroundColor Cyan
    
    # Crear directorio de configuración
    if (-not (Test-Path $script:AIShellConfigPath)) {
        New-Item -ItemType Directory -Path $script:AIShellConfigPath -Force | Out-Null
        Write-Host "  ✓ Created AI Shell config directory" -ForegroundColor Green
    }
    
    # Configurar MCP
    Initialize-AIShellMCP
    
    # Configurar custom instructions
    Initialize-AIShellCustomInstructions
    
    # Configurar hooks
    Initialize-AIShellHooks
    
    Write-Host "✓ AI Shell integration initialized" -ForegroundColor Green
}

function Initialize-AIShellMCP {
    <#
    .SYNOPSIS
    Configura MCP servers para AI Shell (Preview 6+)
    #>
    [CmdletBinding()]
    param()
    
    if (-not (Test-Path $script:AIShellMcpConfig)) {
        $mcpConfig = @{
            servers = @{
                github = @{
                    command = "docker"
                    args = @(
                        "run", "-i", "--rm",
                        "-e", "GITHUB_PERSONAL_ACCESS_TOKEN",
                        "-e", "GITHUB_TOOLSETS",
                        "ghcr.io/github/github-mcp-server"
                    )
                    env = @{
                        GITHUB_PERSONAL_ACCESS_TOKEN = "`$env:GH_TOKEN"
                        GITHUB_TOOLSETS = "context,repos,issues,pull_requests,actions"
                    }
                }
            }
        }
        
        $mcpConfig | ConvertTo-Json -Depth 10 | Out-File -FilePath $script:AIShellMcpConfig -Encoding UTF8
        Write-Host "  ✓ MCP configuration created" -ForegroundColor Green
    }
    else {
        Write-Host "  ℹ️  MCP configuration already exists" -ForegroundColor Yellow
    }
}

function Initialize-AIShellCustomInstructions {
    <#
    .SYNOPSIS
    Crea custom instructions para AI Shell
    #>
    [CmdletBinding()]
    param()
    
    if (-not (Test-Path $script:AIShellCustomInstructions)) {
        $instructions = @"
# Custom Instructions for AI Shell - GitHub MCP Integration

## Project Context
This PowerShell session is integrated with GitHub MCP Server for enhanced GitHub operations.

## Available Tools
- GitHub MCP Server: Access to repositories, issues, pull requests, workflows
- PowerShell Module: GitHubMcp.psm1 with advanced features
- Built-in Tools: get_working_directory, get_command_history, get_terminal_content, etc.

## Best Practices
1. Use GitHub MCP tools for GitHub operations when available
2. Leverage PowerShell pipeline for data processing
3. Use caching for frequently accessed data
4. Validate inputs before making requests
5. Use Resolve-Error command when errors occur for AI-powered resolution

## Common Workflows
- Get repository information: Use GitHub MCP `get_file_contents` or `list_issues`
- Create issues: Use `New-GitHubIssue` PowerShell cmdlet
- Analyze repositories: Use `Get-GitHubRepositoryAnalysis`
- Search code: Use `Search-GitHubCode` or GitHub MCP `search_code`
- Resolve errors: Use `Resolve-Error` (alias: `fixit`) for AI-powered error resolution

## Error Handling
- Always check connection with `Test-GitHubConnection`
- Use retry logic for transient failures
- Log errors for debugging
- Use Resolve-Error command to get AI-powered solutions

## Security
- Never expose tokens in output
- Validate all inputs
- Use rate limiting for API calls
- Protect sensitive data in logs

## Built-in Tools Usage
- get_working_directory: Get current directory context
- get_command_history: Review recent commands for context
- get_terminal_content: Access terminal output
- post_code_to_terminal: Insert code suggestions
- run_command_in_terminal: Execute commands (Windows only)
"@
        
        $instructions | Out-File -FilePath $script:AIShellCustomInstructions -Encoding UTF8
        Write-Host "  ✓ Custom instructions created" -ForegroundColor Green
    }
    else {
        # Actualizar instrucciones existentes si es necesario
        $current = Get-Content $script:AIShellCustomInstructions -Raw
        if ($current -notmatch "Resolve-Error") {
            $updated = $current + "`n`n## Error Resolution`n- Use Resolve-Error command (alias: fixit) for AI-powered error resolution`n"
            $updated | Out-File -FilePath $script:AIShellCustomInstructions -Encoding UTF8
            Write-Host "  ✓ Custom instructions updated" -ForegroundColor Green
        }
    }
}

function Initialize-AIShellHooks {
    <#
    .SYNOPSIS
    Configura hooks para AI Shell (Preview 7+)
    #>
    [CmdletBinding()]
    param()
    
    $hooksPath = "$script:AIShellConfigPath\hooks"
    if (-not (Test-Path $hooksPath)) {
        New-Item -ItemType Directory -Path $hooksPath -Force | Out-Null
    }
    
    # Hook pre-command mejorado
    $preCommandHook = @"
# Pre-command hook for GitHub MCP operations
param(`$command)

# Log command execution
if (`$command -match 'Get-GitHub|New-GitHub|Invoke-GitHub|Search-GitHub') {
    Write-Verbose "Executing GitHub MCP command: `$command"
    
    # Validate connection
    if (Get-Command Test-GitHubConnection -ErrorAction SilentlyContinue) {
        if (-not (Test-GitHubConnection -ErrorAction SilentlyContinue)) {
            Write-Warning "GitHub connection not available. Command may fail."
        }
    }
    
    # Rate limiting check
    if (Get-Command Test-RateLimit -ErrorAction SilentlyContinue) {
        try {
            Test-RateLimit -Operation "github_command" -ErrorAction Stop
        }
        catch {
            Write-Warning "Rate limit check failed: `$_"
        }
    }
}
"@
    
    $preCommandHook | Out-File -FilePath "$hooksPath\pre-command.ps1" -Encoding UTF8
    
    # Hook post-command mejorado
    $postCommandHook = @"
# Post-command hook for GitHub MCP operations
param(`$command, `$result, `$error)

if (`$error) {
    Write-Verbose "Command failed: `$error"
    
    # Log error for analysis
    if (Get-Command Write-GitHubMcpLog -ErrorAction SilentlyContinue) {
        Write-GitHubMcpLog -Level "Error" -Message "Command failed in hook" -Data @{
            Command = `$command
            Error = `$error.ToString()
        }
    }
    
    # Sugerir usar Resolve-Error si está disponible
    if (Get-Command Resolve-Error -ErrorAction SilentlyContinue) {
        Write-Host "💡 Tip: Use 'Resolve-Error' or 'fixit' for AI-powered error resolution" -ForegroundColor Cyan
    }
}
else {
    Write-Verbose "Command succeeded"
    
    # Log success
    if (Get-Command Write-GitHubMcpLog -ErrorAction SilentlyContinue) {
        Write-GitHubMcpLog -Level "Debug" -Message "Command succeeded" -Data @{
            Command = `$command
        }
    }
}
"@
    
    $postCommandHook | Out-File -FilePath "$hooksPath\post-command.ps1" -Encoding UTF8
    
    # Hook de error mejorado
    $errorHook = @"
# Error hook for GitHub MCP operations
param(`$error)

if (`$error) {
    # Preparar contexto para Resolve-Error
    `$errorContext = @{
        Error = `$error.ToString()
        Command = `$MyInvocation.HistoryId
        Timestamp = Get-Date
    }
    
    # Log error
    if (Get-Command Write-GitHubMcpLog -ErrorAction SilentlyContinue) {
        Write-GitHubMcpLog -Level "Error" -Message "Error occurred" -Data `$errorContext
    }
}
"@
    
    $errorHook | Out-File -FilePath "$hooksPath\error.ps1" -Encoding UTF8
    
    Write-Host "  ✓ Hooks configured (pre-command, post-command, error)" -ForegroundColor Green
}

function New-AIShellCustomAgent {
    <#
    .SYNOPSIS
    Crea configuración para agente personalizado de AI Shell
    
    .PARAMETER Name
    Nombre del agente
    
    .PARAMETER Description
    Descripción del agente
    
    .PARAMETER SystemPrompt
    System prompt para el agente
    
    .PARAMETER Endpoint
    Endpoint del modelo (Azure OpenAI, Ollama, etc.)
    
    .PARAMETER ApiKey
    API Key (opcional, puede usar variables de entorno)
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)]
        [string]$Name,
        
        [string]$Description,
        [string]$SystemPrompt,
        [string]$Endpoint,
        [string]$ApiKey
    )
    
    $agentConfig = @{
        Name = $Name
        Description = $Description
        SystemPrompt = $SystemPrompt
        Endpoint = $Endpoint
        Type = "Custom"
    }
    
    if ($ApiKey) {
        $agentConfig.ApiKey = $ApiKey
    }
    
    $configPath = "$script:AIShellConfigPath\agents\$Name.json"
    $agentDir = Split-Path $configPath -Parent
    
    if (-not (Test-Path $agentDir)) {
        New-Item -ItemType Directory -Path $agentDir -Force | Out-Null
    }
    
    $agentConfig | ConvertTo-Json -Depth 10 | Out-File -FilePath $configPath -Encoding UTF8
    
    Write-Host "✓ Custom agent created: $Name" -ForegroundColor Green
    Write-Host "  Use in AI Shell: @$Name or /agent use $Name" -ForegroundColor Yellow
}

function New-AIShellSkill {
    <#
    .SYNOPSIS
    Crea una skill personalizada para AI Shell
    
    .PARAMETER Name
    Nombre de la skill
    
    .PARAMETER Description
    Descripción de la skill
    
    .PARAMETER Instructions
    Instrucciones para la skill
    
    .PARAMETER Script
    Script PowerShell para la skill
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)]
        [string]$Name,
        
        [Parameter(Mandatory)]
        [string]$Description,
        
        [string]$Instructions,
        [scriptblock]$Script
    )
    
    $skillPath = "$script:AIShellConfigPath\skills\$Name"
    if (-not (Test-Path $skillPath)) {
        New-Item -ItemType Directory -Path $skillPath -Force | Out-Null
    }
    
    # Crear skill.json
    $skillConfig = @{
        name = $Name
        description = $Description
        instructions = $Instructions
        version = "1.0.0"
    }
    
    $skillConfig | ConvertTo-Json -Depth 10 | Out-File -FilePath "$skillPath\skill.json" -Encoding UTF8
    
    # Crear script si se proporciona
    if ($Script) {
        $Script.ToString() | Out-File -FilePath "$skillPath\script.ps1" -Encoding UTF8
    }
    
    Write-Host "✓ Skill created: $Name" -ForegroundColor Green
}

function Get-AIShellBuiltInTools {
    <#
    .SYNOPSIS
    Lista herramientas built-in disponibles en AI Shell (Preview 6+)
    #>
    [CmdletBinding()]
    param()
    
    return @(
        @{
            Name = "get_working_directory"
            Description = "Get current PowerShell session directory"
            Category = "File System"
        },
        @{
            Name = "get_command_history"
            Description = "Retrieve recent commands"
            Category = "Shell"
        },
        @{
            Name = "get_terminal_content"
            Description = "Get terminal output"
            Category = "Terminal"
        },
        @{
            Name = "get_environment_variables"
            Description = "Access environment variables"
            Category = "System"
        },
        @{
            Name = "copy_text_to_clipboard"
            Description = "Copy text to clipboard"
            Category = "System"
        },
        @{
            Name = "post_code_to_terminal"
            Description = "Insert code into the prompt"
            Category = "Terminal"
        },
        @{
            Name = "run_command_in_terminal"
            Description = "Execute commands in terminal (Windows only)"
            Category = "Terminal"
            Platform = "Windows"
        }
    ) | ForEach-Object { [PSCustomObject]$_ }
}

function Start-AIShellWithGitHub {
    <#
    .SYNOPSIS
    Inicia AI Shell con integración GitHub MCP preconfigurada
    
    .PARAMETER Agent
    Agente a usar (default: azure-openai)
    
    .PARAMETER Sidecar
    Usar modo sidecar con Windows Terminal
    #>
    [CmdletBinding()]
    param(
        [string]$Agent = "azure-openai",
        [switch]$Sidecar
    )
    
    # Asegurar que la integración esté inicializada
    Initialize-AIShellIntegration
    
    if ($Sidecar) {
        if (Get-Command Start-AIShell -ErrorAction SilentlyContinue) {
            Write-Host "Starting AI Shell in sidecar mode..." -ForegroundColor Cyan
            Start-AIShell -Agent $Agent
        }
        else {
            Write-Warning "Start-AIShell not available. Install AI Shell module."
            Write-Host "Install: Install-Module -Name AIShell -Scope CurrentUser" -ForegroundColor Yellow
        }
    }
    else {
        if (Get-Command aish -ErrorAction SilentlyContinue) {
            Write-Host "Starting AI Shell..." -ForegroundColor Cyan
            Write-Host "  GitHub MCP Server: Configured" -ForegroundColor Green
            Write-Host "  Custom Instructions: Loaded" -ForegroundColor Green
            Write-Host "  Use: @github or /mcp to access GitHub tools" -ForegroundColor Yellow
            & aish
        }
        else {
            Write-Warning "AI Shell (aish) not found in PATH"
            Write-Host "Install from: https://github.com/PowerShell/AIShell" -ForegroundColor Yellow
        }
    }
}

function Get-AIShellMCPTools {
    <#
    .SYNOPSIS
    Lista herramientas MCP disponibles en AI Shell
    #>
    [CmdletBinding()]
    param()
    
    if (Get-Command aish -ErrorAction SilentlyContinue) {
        Write-Host "Available MCP tools (use /mcp in AI Shell to view):" -ForegroundColor Cyan
        
        # Leer configuración MCP
        if (Test-Path $script:AIShellMcpConfig) {
            $config = Get-Content $script:AIShellMcpConfig | ConvertFrom-Json
            
            foreach ($server in $config.servers.PSObject.Properties.Name) {
                Write-Host "`n  Server: $server" -ForegroundColor Yellow
                $serverConfig = $config.servers.$server
                Write-Host "    Command: $($serverConfig.command)" -ForegroundColor Gray
                
                if ($serverConfig.env) {
                    Write-Host "    Environment:" -ForegroundColor Gray
                    foreach ($envVar in $serverConfig.env.PSObject.Properties.Name) {
                        Write-Host "      $envVar" -ForegroundColor DarkGray
                    }
                }
            }
        }
    }
    else {
        Write-Warning "AI Shell not available. Install from: https://github.com/PowerShell/AIShell"
    }
}

function Set-AIShellSystemPrompt {
    <#
    .SYNOPSIS
    Configura system prompt para agente de AI Shell
    
    .PARAMETER Agent
    Nombre del agente
    
    .PARAMETER Prompt
    System prompt
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)]
        [string]$Agent,
        
        [Parameter(Mandatory)]
        [string]$Prompt
    )
    
    if (Get-Command aish -ErrorAction SilentlyContinue) {
        # Usar /agent config en AI Shell
        Write-Host "To configure system prompt for agent '$Agent':" -ForegroundColor Cyan
        Write-Host "  1. Start AI Shell: aish" -ForegroundColor Yellow
        Write-Host "  2. Use command: /agent config $Agent" -ForegroundColor Yellow
        Write-Host "  3. Set system prompt: $Prompt" -ForegroundColor Yellow
    }
    else {
        Write-Warning "AI Shell not available"
    }
}

function Invoke-AIShellCommand {
    <#
    .SYNOPSIS
    Ejecuta comando en AI Shell de forma programática
    
    .PARAMETER Prompt
    Prompt/comando a ejecutar
    
    .PARAMETER Agent
    Agente a usar
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)]
        [string]$Prompt,
        
        [string]$Agent
    )
    
    if (Get-Command aish -ErrorAction SilentlyContinue) {
        $aishArgs = @("-p", $Prompt)
        
        if ($Agent) {
            $aishArgs += @("-a", $Agent)
        }
        
        & aish @aishArgs
    }
    else {
        Write-Warning "AI Shell (aish) not available"
    }
}

function Get-AIShellStatus {
    <#
    .SYNOPSIS
    Obtiene estado de integración con AI Shell
    #>
    [CmdletBinding()]
    param()
    
    $status = @{
        AIShellInstalled = $false
        AIShellModuleAvailable = $false
        MCPConfigured = $false
        CustomInstructionsConfigured = $false
        HooksConfigured = $false
    }
    
    # Verificar instalación
    if (Get-Command aish -ErrorAction SilentlyContinue) {
        $status.AIShellInstalled = $true
    }
    
    if (Get-Command Start-AIShell -ErrorAction SilentlyContinue) {
        $status.AIShellModuleAvailable = $true
    }
    
    # Verificar configuración
    if (Test-Path $script:AIShellMcpConfig) {
        $status.MCPConfigured = $true
    }
    
    if (Test-Path $script:AIShellCustomInstructions) {
        $status.CustomInstructionsConfigured = $true
    }
    
    if (Test-Path "$script:AIShellConfigPath\hooks") {
        $status.HooksConfigured = $true
    }
    
    return [PSCustomObject]$status
}

function Update-AIShellMCPConfig {
    <#
    .SYNOPSIS
    Actualiza configuración MCP de AI Shell
    #>
    [CmdletBinding()]
    param(
        [string]$ServerName = "github",
        [hashtable]$AdditionalEnv = @{}
    )
    
    if (-not (Test-Path $script:AIShellMcpConfig)) {
        Initialize-AIShellMCP
    }
    
    $config = Get-Content $script:AIShellMcpConfig | ConvertFrom-Json
    
    if ($config.servers.$ServerName) {
        # Actualizar variables de entorno
        foreach ($key in $AdditionalEnv.Keys) {
            $config.servers.$ServerName.env[$key] = $AdditionalEnv[$key]
        }
        
        $config | ConvertTo-Json -Depth 10 | Out-File -FilePath $script:AIShellMcpConfig -Encoding UTF8
        Write-Host "✓ MCP configuration updated" -ForegroundColor Green
    }
    else {
        Write-Warning "Server '$ServerName' not found in MCP configuration"
    }
}

Export-ModuleMember -Function @(
    'Initialize-AIShellIntegration',
    'Initialize-AIShellMCP',
    'Initialize-AIShellCustomInstructions',
    'Initialize-AIShellHooks',
    'New-AIShellCustomAgent',
    'New-AIShellSkill',
    'Get-AIShellBuiltInTools',
    'Start-AIShellWithGitHub',
    'Get-AIShellMCPTools',
    'Set-AIShellSystemPrompt',
    'Invoke-AIShellCommand',
    'Get-AIShellStatus',
    'Update-AIShellMCPConfig'
)
