# 🔥 Script de Ejecución NUCLEAR MCP - Servidor HTTP
# Ejecuta el servidor MCP en modo HTTP con Axum

param(
    [int]$Port = 8080,
    [string]$Mode = "http"
)

Write-Host "═══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "   🔥🔥🔥 NUCLEAR MCP - Servidor HTTP 🔥🔥🔥" -ForegroundColor Yellow
Write-Host "═══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

# Verificar si el binario existe
$binaryPath = ".\target\debug\nuclear-mcp.exe"
if (-not (Test-Path $binaryPath)) {
    Write-Host "❌ Error: Binario no encontrado en $binaryPath" -ForegroundColor Red
    Write-Host ""
    Write-Host "💡 Primero compila el proyecto:" -ForegroundColor Yellow
    Write-Host "   .\build_http.ps1" -ForegroundColor White
    Write-Host ""
    exit 1
}

Write-Host "📦 Binario encontrado: $binaryPath" -ForegroundColor Green
Write-Host "🌐 Puerto: $Port" -ForegroundColor Cyan
Write-Host "🎯 Modo: $Mode" -ForegroundColor Cyan
Write-Host ""
Write-Host "🚀 Iniciando servidor..." -ForegroundColor Yellow
Write-Host "   (Presiona Ctrl+C para detener)" -ForegroundColor Gray
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

# Ejecutar el servidor
& $binaryPath --mode $Mode --port $Port
