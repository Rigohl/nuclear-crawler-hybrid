#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Professional development workflow automation for Nuclear Crawler Hybrid
.DESCRIPTION
    Automates common development tasks with validation, error handling, and logging
.PARAMETER Action
    The action to perform: validate, build, test, deploy, or watch
.PARAMETER Component
    Specific component: rust, chapel, go, or all
.PARAMETER Verbose
    Enable verbose output
.EXAMPLE
    .\dev-workflow.ps1 -Action validate
    .\dev-workflow.ps1 -Action build -Component rust
    .\dev-workflow.ps1 -Action watch
#>

param(
    [Parameter(Mandatory = $false)]
    [ValidateSet('validate', 'build', 'test', 'deploy', 'watch', 'clean', 'format', 'pre-commit')]
    [string]$Action = 'validate',
    
    [Parameter(Mandatory = $false)]
    [ValidateSet('rust', 'chapel', 'go', 'all')]
    [string]$Component = 'all',
    
    [switch]$Verbose
)

$ErrorActionPreference = 'Stop'
$script:RootDir = Split-Path -Parent $PSScriptRoot
$script:LogFile = Join-Path $RootDir "logs/dev-workflow-$(Get-Date -Format 'yyyyMMdd-HHmmss').log"

# Ensure logs directory exists
New-Item -ItemType Directory -Force -Path (Join-Path $RootDir "logs") | Out-Null

function Write-Log {
    param([string]$Message, [string]$Level = "INFO")
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logMessage = "[$timestamp] [$Level] $Message"
    
    switch ($Level) {
        "ERROR" { Write-Host $logMessage -ForegroundColor Red }
        "WARN"  { Write-Host $logMessage -ForegroundColor Yellow }
        "SUCCESS" { Write-Host $logMessage -ForegroundColor Green }
        default { Write-Host $logMessage -ForegroundColor White }
    }
    
    Add-Content -Path $script:LogFile -Value $logMessage
}

function Test-Prerequisites {
    Write-Log "🔍 Checking prerequisites..."
    
    $tools = @(
        @{ Name = "cargo"; Command = "cargo --version"; Required = $true },
        @{ Name = "rustc"; Command = "rustc --version"; Required = $true },
        @{ Name = "chpl"; Command = "chpl --version"; Required = $true },
        @{ Name = "go"; Command = "go version"; Required = $true }
    )
    
    $allPresent = $true
    foreach ($tool in $tools) {
        try {
            $version = Invoke-Expression $tool.Command 2>&1
            Write-Log "✅ $($tool.Name): $version" "SUCCESS"
        } catch {
            if ($tool.Required) {
                Write-Log "❌ $($tool.Name) not found or not working" "ERROR"
                $allPresent = $false
            } else {
                Write-Log "⚠️ $($tool.Name) not found (optional)" "WARN"
            }
        }
    }
    
    if (-not $allPresent) {
        throw "Missing required tools. Check QUICK_START.md for installation instructions."
    }
    
    Write-Log "✅ All prerequisites met" "SUCCESS"
}

function Invoke-Validation {
    Write-Log "🔍 Running validation checks..." "INFO"
    
    # Check directory structure
    $requiredDirs = @("src", "ffi/chapel", "mcp-servers/github", "models")
    foreach ($dir in $requiredDirs) {
        $path = Join-Path $script:RootDir $dir
        if (-not (Test-Path $path)) {
            Write-Log "❌ Required directory missing: $dir" "ERROR"
            throw "Directory structure validation failed"
        }
    }
    Write-Log "✅ Directory structure valid" "SUCCESS"
    
    # Validate 5 MCP tools constraint
    Write-Log "🔍 Validating 5 MCP tools constraint..." "INFO"
    Push-Location $script:RootDir
    try {
        $result = cargo test test_exactly_5_tools --lib 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Log "✅ 5 MCP tools constraint verified" "SUCCESS"
        } else {
            Write-Log "❌ 5 MCP tools constraint VIOLATED!" "ERROR"
            Write-Log $result "ERROR"
            throw "Critical constraint violated"
        }
    } finally {
        Pop-Location
    }
    
    # Check for mocks (should have NONE)
    Write-Log "🔍 Checking for mock code..." "INFO"
    $mockPatterns = @("mock_", "Mock", "TODO: implement", "unimplemented!")
    $foundMocks = @()
    
    foreach ($pattern in $mockPatterns) {
        $results = Get-ChildItem -Path (Join-Path $script:RootDir "src") -Filter "*.rs" -Recurse | 
            Select-String -Pattern $pattern -SimpleMatch
        if ($results) {
            $foundMocks += $results
        }
    }
    
    if ($foundMocks.Count -gt 0) {
        Write-Log "⚠️ Potential mock code found (NO MOCKS policy):" "WARN"
        foreach ($match in $foundMocks[0..4]) {  # Show first 5
            Write-Log "  - $($match.Path):$($match.LineNumber)" "WARN"
        }
    } else {
        Write-Log "✅ No mock code detected" "SUCCESS"
    }
}

function Invoke-Build {
    param([string]$Target)
    
    Write-Log "🔨 Building $Target..." "INFO"
    Push-Location $script:RootDir
    
    try {
        switch ($Target) {
            'rust' {
                Write-Log "Building Rust MCP server..." "INFO"
                cargo build --release
                if ($LASTEXITCODE -ne 0) { throw "Rust build failed" }
                Write-Log "✅ Rust build complete" "SUCCESS"
            }
            'chapel' {
                Write-Log "Building Chapel AI systems..." "INFO"
                Push-Location "ffi/chapel"
                make full-pipeline
                if ($LASTEXITCODE -ne 0) { throw "Chapel build failed" }
                Pop-Location
                Write-Log "✅ Chapel build complete" "SUCCESS"
            }
            'go' {
                Write-Log "Building Go GitHub MCP..." "INFO"
                Push-Location "mcp-servers/github"
                go build
                if ($LASTEXITCODE -ne 0) { throw "Go build failed" }
                Pop-Location
                Write-Log "✅ Go build complete" "SUCCESS"
            }
            'all' {
                Invoke-Build -Target 'rust'
                Invoke-Build -Target 'chapel'
                Invoke-Build -Target 'go'
            }
        }
    } finally {
        Pop-Location
    }
}

function Invoke-Tests {
    param([string]$Target)
    
    Write-Log "🧪 Running tests for $Target..." "INFO"
    Push-Location $script:RootDir
    
    try {
        switch ($Target) {
            'rust' {
                # Critical: 5 tools test
                cargo test test_exactly_5_tools
                if ($LASTEXITCODE -ne 0) { throw "5 tools constraint test FAILED" }
                
                # Unit tests
                cargo test --lib
                if ($LASTEXITCODE -ne 0) { throw "Rust unit tests failed" }
                
                Write-Log "✅ Rust tests passed" "SUCCESS"
            }
            'chapel' {
                Push-Location "ffi/chapel"
                make test
                if ($LASTEXITCODE -ne 0) { throw "Chapel tests failed" }
                Pop-Location
                Write-Log "✅ Chapel tests passed" "SUCCESS"
            }
            'go' {
                Push-Location "mcp-servers/github"
                go test ./...
                if ($LASTEXITCODE -ne 0) { throw "Go tests failed" }
                Pop-Location
                Write-Log "✅ Go tests passed" "SUCCESS"
            }
            'all' {
                Invoke-Tests -Target 'rust'
                Invoke-Tests -Target 'chapel'
                Invoke-Tests -Target 'go'
            }
        }
    } finally {
        Pop-Location
    }
}

function Invoke-Format {
    Write-Log "📝 Formatting code..." "INFO"
    Push-Location $script:RootDir
    
    try {
        # Rust
        cargo fmt
        Write-Log "✅ Rust formatted" "SUCCESS"
        
        # Go
        Push-Location "mcp-servers/github"
        go fmt ./...
        Pop-Location
        Write-Log "✅ Go formatted" "SUCCESS"
    } finally {
        Pop-Location
    }
}

function Invoke-Clean {
    Write-Log "🧹 Cleaning build artifacts..." "INFO"
    Push-Location $script:RootDir
    
    try {
        cargo clean
        Push-Location "ffi/chapel"
        make clean
        Pop-Location
        
        Write-Log "✅ Clean complete" "SUCCESS"
    } finally {
        Pop-Location
    }
}

function Invoke-PreCommit {
    Write-Log "🚦 Running pre-commit checks..." "INFO"
    
    # Format check (don't modify, just verify)
    Push-Location $script:RootDir
    cargo fmt -- --check
    if ($LASTEXITCODE -ne 0) {
        Write-Log "❌ Code not formatted. Run: cargo fmt" "ERROR"
        return $false
    }
    Write-Log "✅ Format check passed" "SUCCESS"
    
    # Validate constraints
    Invoke-Validation
    
    # Build all
    Invoke-Build -Target 'all'
    
    # Test all
    Invoke-Tests -Target 'all'
    
    Write-Log "✅ All pre-commit checks passed! Safe to commit." "SUCCESS"
    Pop-Location
    return $true
}

function Start-Watch {
    Write-Log "👀 Starting file watcher..." "INFO"
    Write-Log "Watching for changes in src/, ffi/chapel/, mcp-servers/github/" "INFO"
    
    # Install cargo-watch if not present
    if (-not (Get-Command cargo-watch -ErrorAction SilentlyContinue)) {
        Write-Log "Installing cargo-watch..." "INFO"
        cargo install cargo-watch
    }
    
    Push-Location $script:RootDir
    cargo watch -x check -x test
    Pop-Location
}

# Main execution
try {
    Write-Log "🚀 Nuclear Crawler Hybrid - Dev Workflow" "INFO"
    Write-Log "Action: $Action | Component: $Component" "INFO"
    Write-Log "Log file: $script:LogFile" "INFO"
    Write-Log "" "INFO"
    
    # Check prerequisites
    Test-Prerequisites
    
    # Execute action
    switch ($Action) {
        'validate' { Invoke-Validation }
        'build'    { Invoke-Build -Target $Component }
        'test'     { Invoke-Tests -Target $Component }
        'clean'    { Invoke-Clean }
        'format'   { Invoke-Format }
        'pre-commit' { Invoke-PreCommit }
        'watch'    { Start-Watch }
        'deploy'   { 
            Write-Log "Deployment requires manual HuggingFace sync" "WARN"
            Write-Log "See QUICK_START.md for deployment instructions" "INFO"
        }
    }
    
    Write-Log "" "INFO"
    Write-Log "✅ Workflow completed successfully!" "SUCCESS"
    
} catch {
    Write-Log "" "INFO"
    Write-Log "❌ Workflow failed: $_" "ERROR"
    Write-Log "Stack trace: $($_.ScriptStackTrace)" "ERROR"
    exit 1
}
