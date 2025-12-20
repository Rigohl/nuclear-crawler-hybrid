# Script para listar Pull Requests en GitHub usando GitHub CLI
# Uso: .\gh_list_prs.ps1 -State "open" -Labels "enhancement" -Limit 10

param(
    [string]$State = "open",

    [string]$Labels = "",

    [int]$Limit = 10,

    [string]$Assignee = "",

    [switch]$Json
)

# Verificar que estamos en un repositorio git
if (!(Test-Path .git)) {
    Write-Host "Error: No se encuentra un repositorio git en el directorio actual." -ForegroundColor Red
    exit 1
}

# Construir el comando gh
$command = "gh pr list --state $State --limit $Limit"

if ($Labels) {
    $command += " --label `"$Labels`""
}

if ($Assignee) {
    $command += " --assignee `"$Assignee`""
}

if ($Json) {
    $command += " --json number,title,labels,assignees,createdAt,updatedAt,mergeable"
}

Write-Host "Ejecutando: $command" -ForegroundColor Yellow

# Ejecutar el comando
try {
    if ($Json) {
        $result = Invoke-Expression $command | ConvertFrom-Json
        $result | Format-Table -AutoSize
    } else {
        $result = Invoke-Expression $command
        Write-Host $result
    }
} catch {
    Write-Host "Error al listar PRs: $($_.Exception.Message)" -ForegroundColor Red
}
