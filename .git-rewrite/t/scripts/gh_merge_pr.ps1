# Script para mergear un Pull Request en GitHub usando GitHub CLI
# Uso: .\gh_merge_pr.ps1 -Number 123 -Method "merge" -DeleteBranch

param(
    [Parameter(Mandatory=$true)]
    [int]$Number,

    [string]$Method = "merge",  # merge, squash, rebase

    [switch]$DeleteBranch,

    [string]$CommitMessage = ""
)

# Verificar que estamos en un repositorio git
if (!(Test-Path .git)) {
    Write-Host "Error: No se encuentra un repositorio git en el directorio actual." -ForegroundColor Red
    exit 1
}

# Construir el comando gh
$command = "gh pr merge $Number --$Method"

if ($DeleteBranch) {
    $command += " --delete-branch"
}

if ($CommitMessage -and $Method -eq "merge") {
    $command += " --merge-commit-message `"$CommitMessage`""
}

Write-Host "Ejecutando: $command" -ForegroundColor Yellow

# Ejecutar el comando
try {
    $result = Invoke-Expression $command
    Write-Host "Pull Request #$Number mergeado exitosamente." -ForegroundColor Green
    Write-Host $result
} catch {
    Write-Host "Error al mergear el PR: $($_.Exception.Message)" -ForegroundColor Red
}
