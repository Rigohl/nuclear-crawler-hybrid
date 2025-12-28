# Nuclear Crawler Hybrid - Complete Setup Script for Windows (PowerShell)
# Run as Administrator

param(
    [string]$InstallDir = "C:\Users\DELL\Desktop\hf_spaces\NUCLEAR_CRAWLER_HYBRID",
    [string]$RepoUrl = "https://github.com/Rigohl/nuclear-crawler-hybrid.git"
)

$AppDataPath = [Environment]::GetFolderPath("ApplicationData")
$VsCodeConfig = "$AppDataPath\Code\User\settings.json"
$VsInsidersConfig = "$AppDataPath\Code - Insiders\User\settings.json"

Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  🔥 NUCLEAR CRAWLER HYBRID - SETUP (Windows PowerShell)" -ForegroundColor Yellow
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

# Step 1: Clone or Update Repository
Write-Host "📥 Step 1: Clone/Update Repository" -ForegroundColor Green

if (Test-Path $InstallDir) {
    Write-Host "📂 Directorio ya existe: $InstallDir" -ForegroundColor Yellow
    Write-Host "Actualizando..." -ForegroundColor Yellow
    Set-Location $InstallDir
    git pull origin main
} else {
    Write-Host "Clonando repositorio..." -ForegroundColor Yellow
    $parentDir = Split-Path -Parent $InstallDir
    New-Item -ItemType Directory -Force -Path $parentDir | Out-Null
    Set-Location $parentDir
    git clone $RepoUrl NUCLEAR_CRAWLER_HYBRID
    Set-Location $InstallDir
}

Write-Host "✅ Repositorio listo" -ForegroundColor Green

# Step 2: Start Docker
Write-Host ""
Write-Host "🐳 Step 2: Docker Compose" -ForegroundColor Green
Write-Host "Iniciando Docker..." -ForegroundColor Yellow

docker-compose up -d

Write-Host "⏳ Esperando que el container inicie..." -ForegroundColor Yellow
Start-Sleep -Seconds 5

# Verify Docker
$dockerStatus = docker ps | Select-String "nuclear-mcp-server"
if ($dockerStatus) {
    Write-Host "✅ Docker ejecutándose correctamente" -ForegroundColor Green
    Write-Host "   URL: http://localhost:8079" -ForegroundColor Cyan
} else {
    Write-Host "❌ Docker no está ejecutándose" -ForegroundColor Red
    Write-Host "   Intenta: docker-compose up -d" -ForegroundColor Yellow
}

# Step 3: Configure VS Code
Write-Host ""
Write-Host "🔧 Step 3: Configure VS Code" -ForegroundColor Green
Write-Host "Configurando: $VsCodeConfig" -ForegroundColor Yellow

$parentDir = Split-Path -Parent $VsCodeConfig
if (-not (Test-Path $parentDir)) {
    New-Item -ItemType Directory -Force -Path $parentDir | Out-Null
}

if (Test-Path $VsCodeConfig) {
    try {
        $config = Get-Content $VsCodeConfig -Raw | ConvertFrom-Json
    } catch {
        $config = @{}
    }
} else {
    $config = @{}
}

if (-not $config."modelContextProtocol.servers") {
    $config | Add-Member -NotePropertyName "modelContextProtocol.servers" -NotePropertyValue @{}
}

$config."modelContextProtocol.servers"."nuclear-crawler-hybrid" = @{
    type = "http"
    url = "http://localhost:8079"
}

$config | ConvertTo-Json -Depth 10 | Set-Content $VsCodeConfig -Encoding UTF8
Write-Host "✅ VS Code configurado" -ForegroundColor Green

# Step 4: Configure VS Code Insiders
Write-Host ""
Write-Host "🔧 Step 4: Configure VS Code Insiders" -ForegroundColor Green
Write-Host "Configurando: $VsInsidersConfig" -ForegroundColor Yellow

$parentDir = Split-Path -Parent $VsInsidersConfig
if (-not (Test-Path $parentDir)) {
    New-Item -ItemType Directory -Force -Path $parentDir | Out-Null
}

if (Test-Path $VsInsidersConfig) {
    try {
        $config = Get-Content $VsInsidersConfig -Raw | ConvertFrom-Json
    } catch {
        $config = @{}
    }
} else {
    $config = @{}
}

if (-not $config."modelContextProtocol.servers") {
    $config | Add-Member -NotePropertyName "modelContextProtocol.servers" -NotePropertyValue @{}
}

$config."modelContextProtocol.servers"."nuclear-crawler-hybrid" = @{
    type = "http"
    url = "http://localhost:8079"
}

$config | ConvertTo-Json -Depth 10 | Set-Content $VsInsidersConfig -Encoding UTF8
Write-Host "✅ VS Code Insiders configurado" -ForegroundColor Green

# Summary
Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  ✅ INSTALACIÓN COMPLETA" -ForegroundColor Green
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "📂 Directorio: $InstallDir" -ForegroundColor Cyan
Write-Host "🐳 Docker: http://localhost:8079" -ForegroundColor Cyan
Write-Host "🔧 VS Code: Configurado" -ForegroundColor Cyan
Write-Host "🔧 VS Code Insiders: Configurado" -ForegroundColor Cyan
Write-Host ""
Write-Host "🚀 Próximos pasos:" -ForegroundColor Yellow
Write-Host "   1. Reinicia VS Code / VS Code Insiders" -ForegroundColor White
Write-Host "   2. Abre la command palette (Ctrl+Shift+P)" -ForegroundColor White
Write-Host "   3. Busca 'MCP' o usa los 4 tools directamente" -ForegroundColor White
Write-Host ""
Write-Host "📚 Tools disponibles:" -ForegroundColor Yellow
Write-Host "   • websearch (5s timeout)" -ForegroundColor Cyan
Write-Host "   • deepweb_search (10s timeout)" -ForegroundColor Cyan
Write-Host "   • premium_content_scraper (15s timeout)" -ForegroundColor Cyan
Write-Host "   • file_search (8s timeout)" -ForegroundColor Cyan
Write-Host ""
Write-Host "💡 Para detener Docker:" -ForegroundColor Yellow
Write-Host "   docker-compose down" -ForegroundColor Gray
Write-Host ""
