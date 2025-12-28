# Configure MCP Server for VS Code, Cursor, and Windsurf (Windows)
# Run as Administrator if you have permission issues

param(
    [switch]$ConfigureAll = $true
)

$appdata = [Environment]::GetFolderPath("ApplicationData")

$paths = @{
    "VS Code" = "$appdata\Code\User\settings.json"
    "Cursor" = "$appdata\Cursor\User\settings.json"
    "Windsurf" = "$appdata\Windsurf\User\settings.json"
    "Claude Desktop" = "$appdata\Claude\claude_desktop_config.json"
}

$mcpConfig = @{
    type = "http"
    url = "http://127.0.0.1:8079"
}

Write-Host "🔧 Nuclear Crawler Hybrid - MCP Configuration (Windows)" -ForegroundColor Cyan
Write-Host "=======================================================" -ForegroundColor Cyan

function Configure-Editor {
    param(
        [string]$EditorName,
        [string]$ConfigPath,
        [bool]$IsClaudeDesktop = $false
    )
    
    Write-Host ""
    Write-Host "📍 $EditorName :" -ForegroundColor Yellow
    Write-Host "   Path: $ConfigPath"
    
    # Create parent directory if needed
    $parentDir = Split-Path -Parent $ConfigPath
    if (-not (Test-Path $parentDir)) {
        New-Item -ItemType Directory -Force -Path $parentDir | Out-Null
    }
    
    # Load or create config
    if (Test-Path $ConfigPath) {
        try {
            $config = Get-Content $ConfigPath -Raw | ConvertFrom-Json
        } catch {
            Write-Host "   ⚠️  Invalid JSON, creating new config"
            $config = @{}
        }
    } else {
        $config = @{}
    }
    
    # Add MCP server config
    if ($IsClaudeDesktop) {
        if (-not $config.mcpServers) {
            $config | Add-Member -NotePropertyName "mcpServers" -NotePropertyValue @{}
        }
        $config.mcpServers | Add-Member -NotePropertyName "nuclear-crawler-hybrid" -NotePropertyValue $mcpConfig -Force
    } else {
        if (-not $config."modelContextProtocol.servers") {
            $config | Add-Member -NotePropertyName "modelContextProtocol.servers" -NotePropertyValue @{}
        }
        $config."modelContextProtocol.servers" | Add-Member -NotePropertyName "nuclear-crawler-hybrid" -NotePropertyValue $mcpConfig -Force
    }
    
    # Save config
    try {
        $config | ConvertTo-Json -Depth 10 | Set-Content $ConfigPath -Encoding UTF8
        Write-Host "   ✅ Configured" -ForegroundColor Green
    } catch {
        Write-Host "   ❌ Failed: $_" -ForegroundColor Red
    }
}

# Configure all editors
if ($ConfigureAll) {
    foreach ($editor in $paths.Keys) {
        $isClaudeDesktop = ($editor -eq "Claude Desktop")
        Configure-Editor -EditorName $editor -ConfigPath $paths[$editor] -IsClaudeDesktop $isClaudeDesktop
    }
}

# Summary
Write-Host ""
Write-Host "=======================================================" -ForegroundColor Cyan
Write-Host "✅ Configuration Complete!" -ForegroundColor Green
Write-Host ""
Write-Host "🚀 Start MCP server:" -ForegroundColor Cyan
Write-Host "   cargo run --bin nuclear-mcp"
Write-Host ""
Write-Host "📝 Server Details:" -ForegroundColor Cyan
Write-Host "   URL: http://127.0.0.1:8079"
Write-Host "   Tools: websearch, deepweb_search, premium_content_scraper, file_search"
Write-Host ""
Write-Host "🔄 Restart your editors to apply changes" -ForegroundColor Yellow
