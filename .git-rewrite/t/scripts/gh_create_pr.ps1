# Script para crear un Pull Request en GitHub usando GitHub CLI
# Uso: .\gh_create_pr.ps1 -Title "Título del PR" -Body "Descripción" -Branch "feature-branch" -Base "main"

param(
    [Parameter(Mandatory=$true)]
    [string]$Title,

    [Parameter(Mandatory=$true)]
    [string]$Body,

    [Parameter(Mandatory=$true)]
    [string]$Branch,

    [string]$Base = "main",

    [string]$Labels = "",

    [switch]$Draft
)

# Verificar que estamos en un repositorio git
if (!(Test-Path .git)) {
    Write-Host "Error: No se encuentra un repositorio git en el directorio actual." -ForegroundColor Red
    exit 1
}

# Verificar que la rama existe
$branches = git branch --list $Branch
if ($LASTEXITCODE -ne 0 -or !$branches) {
    Write-Host "Error: La rama '$Branch' no existe." -ForegroundColor Red
    exit 1
}

# Cambiar a la rama si no estamos en ella
$currentBranch = git rev-parse --abbrev-ref HEAD
if ($currentBranch -ne $Branch) {
    git checkout $Branch
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Error: No se pudo cambiar a la rama '$Branch'." -ForegroundColor Red
        exit 1
    }
}

# Construir el comando gh
$command = "gh pr create --title `"$Title`" --body `"$Body`" --base $Base --head $Branch"

if ($Labels) {
    $command += " --label `"$Labels`""
}

if ($Draft) {
    $command += " --draft"
}

Write-Host "Ejecutando: $command" -ForegroundColor Yellow

# Ejecutar el comando
try {
    $result = Invoke-Expression $command
    Write-Host "Pull Request creado exitosamente:" -ForegroundColor Green
    Write-Host $result
} catch {
    Write-Host "Error al crear el PR: $($_.Exception.Message)" -ForegroundColor Red
}
