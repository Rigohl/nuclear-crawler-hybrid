# Script para autenticar GitHub CLI
# Uso: .\gh_auth.ps1

Write-Host "Autenticando GitHub CLI..." -ForegroundColor Yellow
Write-Host "Se abrirá el navegador para autenticación. Presiona Enter para continuar." -ForegroundColor Cyan

Read-Host

# Ejecutar autenticación
gh auth login --web

if ($LASTEXITCODE -eq 0) {
    Write-Host "Autenticación exitosa!" -ForegroundColor Green
    gh auth status
} else {
    Write-Host "Error en la autenticación." -ForegroundColor Red
}
