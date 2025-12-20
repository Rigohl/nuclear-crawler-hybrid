#!/usr/bin/env pwsh
# Script de compilación seguro que maneja file locks

Write-Host "🔧 NUCLEAR MCP - Compilación Segura" -ForegroundColor Cyan
Write-Host ""

# Esperar a que se liberen los archivos
Write-Host "⏳ Esperando liberación de archivos..." -ForegroundColor Yellow
Start-Sleep -Seconds 5

# Intentar compilación
Write-Host "🔨 Compilando librería..." -ForegroundColor Green
$output = cargo build --lib 2>&1 | Out-String

# Mostrar resultado
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ COMPILACIÓN EXITOSA" -ForegroundColor Green
    Write-Host $output
}
else {
    Write-Host "❌ ERRORES DE COMPILACIÓN" -ForegroundColor Red
    Write-Host $output
    
    # Guardar errores
    $output | Out-File -FilePath "compilation_errors.txt" -Encoding UTF8
    Write-Host ""
    Write-Host "📝 Errores guardados en: compilation_errors.txt" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "🔍 Verificando binarios..." -ForegroundColor Cyan
cargo build --bin nuclear-mcp 2>&1 | Out-String

exit $LASTEXITCODE
