# Test script para nuclear-mcp.exe
Write-Host "🧪 TESTEANDO NUCLEAR-MCP SERVER" -ForegroundColor Green
Write-Host ""

$binPath = ".\target\release\nuclear-mcp.exe"

# Verificar binario
Write-Host "✅ Binario: $binPath"
if (Test-Path $binPath) {
    $size = (Get-Item $binPath).Length
    Write-Host "   Tamaño: $([math]::Round($size / 1MB, 2)) MB"
} else {
    Write-Host "❌ Binario no encontrado"
    exit 1
}

Write-Host ""
Write-Host "🚀 Iniciando servidor MCP..." -ForegroundColor Yellow
Write-Host ""

# Iniciar el proceso
$process = Start-Process -FilePath $binPath -NoNewWindow -PassThru -RedirectStandardInput $null -RedirectStandardOutput $null -RedirectStandardError $null

Write-Host "✅ Proceso iniciado (PID: $($process.Id))" -ForegroundColor Green
Write-Host ""

Start-Sleep -Seconds 2

# Verificar si el proceso sigue corriendo
if ($process.HasExited) {
    Write-Host "❌ El proceso se cerró inesperadamente" -ForegroundColor Red
    exit 1
} else {
    Write-Host "✅ El proceso sigue corriendo correctamente" -ForegroundColor Green
}

Write-Host ""
Write-Host "📋 Información del proceso:" -ForegroundColor Cyan
$process | Select-Object Id, Name, StartTime | Format-Table

Write-Host ""
Write-Host "Para detener: Stop-Process -Id $($process.Id)" -ForegroundColor Yellow
