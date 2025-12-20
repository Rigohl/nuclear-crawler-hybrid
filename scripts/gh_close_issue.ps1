# Script para cerrar un issue en GitHub usando GitHub CLI
# Uso: .\gh_close_issue.ps1 -Number 123 -Reason "completed"

param(
    [Parameter(Mandatory=$true)]
    [int]$Number,

    [string]$Reason = "completed",

    [string]$Comment = ""
)

# Verificar que estamos en un repositorio git
if (!(Test-Path .git)) {
    Write-Host "Error: No se encuentra un repositorio git en el directorio actual." -ForegroundColor Red
    exit 1
}

# Construir el comando gh
$command = "gh issue close $Number --reason $Reason"

if ($Comment) {
    # Primero agregar comentario si se proporciona
    $commentCommand = "gh issue comment $Number --body `"$Comment`""
    Write-Host "Agregando comentario: $commentCommand" -ForegroundColor Yellow
    Invoke-Expression $commentCommand
}

Write-Host "Ejecutando: $command" -ForegroundColor Yellow

# Ejecutar el comando
try {
    $result = Invoke-Expression $command
    Write-Host "Issue #$Number cerrado exitosamente." -ForegroundColor Green
    Write-Host $result
} catch {
    Write-Host "Error al cerrar el issue: $($_.Exception.Message)" -ForegroundColor Red
}
