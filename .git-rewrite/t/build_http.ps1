# 🔥 Script de Compilación NUCLEAR MCP - Modo HTTP
# Limpia y compila el binario nuclear-mcp con soporte HTTP Axum

Write-Host "═══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "   🔥🔥🔥 NUCLEAR MCP - Compilación HTTP Axum 🔥🔥🔥" -ForegroundColor Yellow
Write-Host "═══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

# 1. Matar procesos de Rust que puedan estar bloqueando archivos
Write-Host "🔧 Paso 1: Limpiando procesos..." -ForegroundColor Green
$processes = @("nuclear-mcp", "rust-analyzer", "rls", "cargo")
foreach ($proc in $processes) {
    try {
        Get-Process -Name $proc -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
        Write-Host "   ✅ Detenido: $proc" -ForegroundColor Gray
    } catch {
        # Ignorar si no existe
    }
}
Start-Sleep -Seconds 2

# 2. Limpiar target
Write-Host ""
Write-Host "🧹 Paso 2: Limpiando directorio target..." -ForegroundColor Green
try {
    cargo clean
    Write-Host "   ✅ Limpieza completada" -ForegroundColor Gray
} catch {
    Write-Host "   ⚠️  Error en limpieza: $_" -ForegroundColor Yellow
}
Start-Sleep -Seconds 1

# 3. Compilar en modo debug (más rápido para pruebas)
Write-Host ""
Write-Host "🔨 Paso 3: Compilando nuclear-mcp (debug)..." -ForegroundColor Green
Write-Host "   (Esto puede tomar varios minutos...)" -ForegroundColor Gray
Write-Host ""

$compileResult = cargo build --bin nuclear-mcp 2>&1
$exitCode = $LASTEXITCODE

if ($exitCode -eq 0) {
    Write-Host ""
    Write-Host "═══════════════════════════════════════════════════════════════════════" -ForegroundColor Green
    Write-Host "   ✅ COMPILACIÓN EXITOSA" -ForegroundColor Green
    Write-Host "═══════════════════════════════════════════════════════════════════════" -ForegroundColor Green
    Write-Host ""
    Write-Host "📦 Binario compilado en:" -ForegroundColor Cyan
    Write-Host "   .\target\debug\nuclear-mcp.exe" -ForegroundColor White
    Write-Host ""
    Write-Host "🚀 Para ejecutar en modo HTTP:" -ForegroundColor Cyan
    Write-Host "   .\target\debug\nuclear-mcp.exe --mode http --port 8080" -ForegroundColor White
    Write-Host ""
    Write-Host "🎯 Para ejecutar en modo Studio (stdio):" -ForegroundColor Cyan
    Write-Host "   .\target\debug\nuclear-mcp.exe --mode studio" -ForegroundColor White
    Write-Host ""
    Write-Host "📚 Endpoints HTTP disponibles:" -ForegroundColor Cyan
    Write-Host "   GET  http://localhost:8080/health" -ForegroundColor White
    Write-Host "   GET  http://localhost:8080/tools" -ForegroundColor White
    Write-Host "   POST http://localhost:8080/call" -ForegroundColor White
    Write-Host "   POST http://localhost:8080/mcp" -ForegroundColor White
    Write-Host ""
    
    # Preguntar si quiere ejecutar
    $response = Read-Host "¿Quieres ejecutar el servidor HTTP ahora? (s/n)"
    if ($response -eq "s" -or $response -eq "S") {
        Write-Host ""
        Write-Host "🚀 Iniciando servidor HTTP en puerto 8080..." -ForegroundColor Yellow
        Write-Host "   (Presiona Ctrl+C para detener)" -ForegroundColor Gray
        Write-Host ""
        .\target\debug\nuclear-mcp.exe --mode http --port 8080
    }
} else {
    Write-Host ""
    Write-Host "═══════════════════════════════════════════════════════════════════════" -ForegroundColor Red
    Write-Host "   ❌ ERROR EN COMPILACIÓN" -ForegroundColor Red
    Write-Host "═══════════════════════════════════════════════════════════════════════" -ForegroundColor Red
    Write-Host ""
    Write-Host "Errores:" -ForegroundColor Yellow
    Write-Host $compileResult -ForegroundColor Gray
    Write-Host ""
    Write-Host "💡 Soluciones:" -ForegroundColor Cyan
    Write-Host "   1. Cerrar todos los procesos de Rust (IDEs, rust-analyzer, etc.)" -ForegroundColor White
    Write-Host "   2. Ejecutar: cargo clean" -ForegroundColor White
    Write-Host "   3. Reintentar compilación" -ForegroundColor White
    Write-Host ""
    exit 1
}
