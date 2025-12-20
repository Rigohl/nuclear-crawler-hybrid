# Script para crear un issue en GitHub usando GitHub CLI
# Uso: .\gh_create_issue.ps1 -Title "Título del issue" -Body "Descripción" -Labels "bug,enhancement"

param(
    [Parameter(Mandatory=$true)]
    [string]$Title,

    [Parameter(Mandatory=$true)]
    [string]$Body,

    [string]$Labels = "",

    [string]$Assignee = ""
)

# Verificar que estamos en un repositorio git
if (!(Test-Path .git)) {
    Write-Host "Error: No se encuentra un repositorio git en el directorio actual." -ForegroundColor Red
    exit 1
}

# Construir el comando gh
$command = "gh issue create --title `"$Title`" --body `"$Body`""

if ($Labels) {
    $command += " --label `"$Labels`""
}

if ($Assignee) {
    $command += " --assignee `"$Assignee`""
}

Write-Host "Ejecutando: $command" -ForegroundColor Yellow

# Ejecutar el comando
try {
    $result = Invoke-Expression $command
    Write-Host "Issue creado exitosamente:" -ForegroundColor Green
    Write-Host $result
} catch {
    Write-Host "Error al crear el issue: $($_.Exception.Message)" -ForegroundColor Red
}
