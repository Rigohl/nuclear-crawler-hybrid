# Ejemplos de uso del Info Agent

# 1. Setup inicial
Write-Host "=== Setup Inicial ===" -ForegroundColor Cyan
& "$PSScriptRoot\..\..\setup-all.ps1" -InfoAgent

# 2. Análisis de URLs
Write-Host "`n=== Análisis de URLs ===" -ForegroundColor Cyan
$urls = @(
    "https://learn.microsoft.com/powershell/utility-modules/aishell",
    "https://docs.github.com/en/copilot/concepts/agents/about-copilot-cli"
)
Start-InfoAgent -Urls $urls

# 3. Consultar free tiers
Write-Host "`n=== Free Tiers por Proveedor ===" -ForegroundColor Cyan

# Azure
Write-Host "`nAzure:" -ForegroundColor Yellow
Get-FreeTierInfo -Provider Azure | Format-Table Service, Description, Limits

# AWS
Write-Host "`nAWS:" -ForegroundColor Yellow
Get-FreeTierInfo -Provider AWS | Format-Table Service, Description, Limits

# Hugging Face
Write-Host "`nHugging Face:" -ForegroundColor Yellow
Get-FreeTierInfo -Provider "Hugging Face" | Format-List

# GitHub
Write-Host "`nGitHub:" -ForegroundColor Yellow
Get-FreeTierInfo -Provider GitHub | Format-Table Service, Description, Limits

# 4. Solo Always Free
Write-Host "`n=== Solo Always Free ===" -ForegroundColor Cyan
Get-FreeTierInfo -AlwaysFreeOnly | Group-Object Provider | ForEach-Object {
    Write-Host "$($_.Name): $($_.Count) servicios" -ForegroundColor Green
}

# 5. Resumen
Write-Host "`n=== Resumen de Free Tiers ===" -ForegroundColor Cyan
Get-FreeTierSummary | Format-Table Provider, TotalServices, AlwaysFree, LimitedFree

# 6. Análisis guardados
Write-Host "`n=== Análisis Guardados ===" -ForegroundColor Cyan
Get-UrlAnalysis | Select-Object -First 10 | Format-Table Url, Category, AnalysisDate

# 7. Exportar a JSON
Write-Host "`n=== Exportar a JSON ===" -ForegroundColor Cyan
$freeTiers = Get-FreeTierInfo
$freeTiers | ConvertTo-Json -Depth 5 | Out-File "free-tiers-export.json" -Encoding UTF8
Write-Host "✓ Exportado a free-tiers-export.json" -ForegroundColor Green

# 8. Agregar integración personalizada
Write-Host "`n=== Agregar Integración ===" -ForegroundColor Cyan
Add-Integration `
    -Name "PowerShell AI Shell" `
    -Description "Shell interactivo con IA para PowerShell" `
    -Provider "Microsoft" `
    -Category "AI/ML" `
    -DocumentationUrl "https://learn.microsoft.com/powershell/utility-modules/aishell" `
    -Status "Available"

# 9. Consultar integraciones
Write-Host "`n=== Integraciones Disponibles ===" -ForegroundColor Cyan
Get-IntegrationInfo | Format-Table Name, Provider, Category, Status

# 10. Actualizar free tiers
Write-Host "`n=== Actualizar Free Tiers ===" -ForegroundColor Cyan
Update-FreeTierInfo

Write-Host "`n✅ Ejemplos completados!" -ForegroundColor Green
