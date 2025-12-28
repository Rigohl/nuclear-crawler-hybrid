# Script para instalar TRAE CLI en el proyecto Nuclear Crawler Hybrid
# Uso: .\install_trae_cli.ps1

Write-Host "Instalando TRAE CLI para Nuclear Crawler Hybrid..." -ForegroundColor Yellow

# Verificar si ya está instalado
$traeExists = Get-Command trae -ErrorAction SilentlyContinue
if ($traeExists) {
    Write-Host "TRAE CLI ya está instalado. Versión:" -ForegroundColor Green
    trae --version
    exit 0
}

# Intentar instalar desde crates.io
Write-Host "Instalando TRAE CLI desde crates.io..." -ForegroundColor Cyan
cargo install trae-cli

if ($LASTEXITCODE -eq 0) {
    Write-Host "TRAE CLI instalado exitosamente!" -ForegroundColor Green
    trae --version
} else {
    # Si falla, intentar desde el directorio local
    $localPath = "$env:USERPROFILE\Desktop\CLI\trae-cli"
    if (Test-Path $localPath) {
        Write-Host "Instalando TRAE CLI desde directorio local..." -ForegroundColor Cyan
        Push-Location $localPath
        cargo install --path .
        Pop-Location

        if ($LASTEXITCODE -eq 0) {
            Write-Host "TRAE CLI instalado exitosamente desde directorio local!" -ForegroundColor Green
            trae --version
        } else {
            Write-Host "Error: No se pudo instalar TRAE CLI." -ForegroundColor Red
            exit 1
        }
    } else {
        Write-Host "Error: No se pudo instalar TRAE CLI desde crates.io ni desde directorio local." -ForegroundColor Red
        Write-Host "Verifica que tienes acceso a $localPath o conexión a internet." -ForegroundColor Yellow
        exit 1
    }
}

Write-Host "`nTRAE CLI está listo para usar en Nuclear Crawler Hybrid!" -ForegroundColor Green

# Configurar variable de entorno para JARVIXSERVER
Write-Host "Configurando JARVIXSERVER en puerto 5050..." -ForegroundColor Cyan
$env:JARVIX_URL = "http://localhost:5050"
[Environment]::SetEnvironmentVariable("JARVIX_URL", "http://localhost:5050", "User")

Write-Host "Configuración:" -ForegroundColor Cyan
Write-Host "  JARVIXSERVER: http://localhost:5050" -ForegroundColor White
Write-Host "  TRAE CLI API: http://localhost:8080" -ForegroundColor White

Write-Host "`nComandos útiles:" -ForegroundColor Cyan
Write-Host "  trae repair          - Reparar código automáticamente" -ForegroundColor White
Write-Host "  trae clippy --strict - Análisis estricto de calidad" -ForegroundColor White
Write-Host "  trae test --all      - Ejecutar todos los tests" -ForegroundColor White
