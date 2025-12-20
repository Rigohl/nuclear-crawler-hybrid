# 🚀 NUCLEAR MCP AUTOMATION - GitHub CLI Master Script
# Uso: .\nuclear_mcp_automation.ps1 -Action [check|analyze|create-issues|create-pr|release|monitor]

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("check", "analyze", "create-issues", "create-pr", "release", "monitor", "cleanup")]
    [string]$Action,

    [string]$Branch = "main",
    [string]$Title,
    [string]$Body,
    [switch]$Force,
    [switch]$Detailed
)

# Configuración
$REPO_NAME = "nuclear-crawler-hybrid"
$REPO_OWNER = "tu-usuario"  # Cambiar por tu usuario real
$WORKFLOW_FILES = @("ci.yml", "nuclear-advanced-pipeline.yml", "security.yml")

function Write-Status {
    param([string]$Message, [string]$Color = "Cyan")
    if ($Detailed) {
        Write-Host "[$((Get-Date).ToString('HH:mm:ss'))] $Message" -ForegroundColor $Color
    }
}

function Test-GitHubCLI {
    try {
        $null = gh --version
        return $true
    } catch {
        Write-Host "❌ GitHub CLI no está instalado o no está en PATH" -ForegroundColor Red
        return $false
    }
}

function Test-Authentication {
    try {
        $auth = gh auth status 2>&1
        if ($auth -match "Logged in") {
            Write-Status "✅ Autenticado en GitHub CLI" "Green"
            return $true
        } else {
            Write-Host "❌ No autenticado en GitHub CLI" -ForegroundColor Red
            Write-Host "Ejecuta: gh auth login" -ForegroundColor Yellow
            return $false
        }
    } catch {
        Write-Host "❌ Error verificando autenticación: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
}

function Get-RepoStatus {
    Write-Status "🔍 Verificando estado del repositorio..."

    try {
        # Estado de git
        $gitStatus = git status --porcelain
        $hasChanges = $gitStatus.Length -gt 0

        # Último commit
        $lastCommit = git log -1 --oneline

        # Branch actual
        $currentBranch = git rev-parse --abbrev-ref HEAD

        # Issues abiertas
        $openIssues = gh issue list --state open --limit 10 --json number,title,labels | ConvertFrom-Json

        # PRs abiertas
        $openPRs = gh pr list --state open --limit 10 --json number,title,headRefName | ConvertFrom-Json

        # Workflows recientes
        $recentWorkflows = gh run list --limit 5 --json status,conclusion,createdAt,headSha | ConvertFrom-Json

        return @{
            HasChanges = $hasChanges
            GitStatus = $gitStatus
            LastCommit = $lastCommit
            CurrentBranch = $currentBranch
            OpenIssues = $openIssues
            OpenPRs = $openPRs
            RecentWorkflows = $recentWorkflows
        }
    } catch {
        Write-Host "❌ Error obteniendo estado del repo: $($_.Exception.Message)" -ForegroundColor Red
        return $null
    }
}

function Invoke-CodeAnalysis {
    Write-Status "🔬 Ejecutando análisis de código extremo..."

    try {
        # Ejecutar cargo check
        $cargoCheck = & cargo check 2>&1
        $cargoExitCode = $LASTEXITCODE

        # Buscar warnings y errors
        $warnings = ($cargoCheck | Select-String -Pattern "warning:").Count
        $errors = ($cargoCheck | Select-String -Pattern "error:").Count

        # Análisis de archivos grandes
        $largeFiles = Get-ChildItem -Recurse -File | Where-Object {
            $_.Length -gt 1MB -and $_.Extension -match '\.(rs|go|nim|zig)$'
        } | Select-Object FullName, Length

        # Análisis de dependencias
        $dependencies = Get-Content "Cargo.toml" -ErrorAction SilentlyContinue |
            Select-String -Pattern '^\s*[\w-]+ = ' |
            ForEach-Object { $_.Line.Trim() }

        return @{
            CargoCheck = $cargoCheck
            CargoExitCode = $cargoExitCode
            Warnings = $warnings
            Errors = $errors
            LargeFiles = $largeFiles
            Dependencies = $dependencies
        }
    } catch {
        Write-Host "❌ Error en análisis de código: $($_.Exception.Message)" -ForegroundColor Red
        return $null
    }
}

function New-AutoIssues {
    param([hashtable]$Analysis)

    Write-Status "📝 Creando issues automáticamente basados en análisis..."

    $issuesCreated = @()

    # Issue por errores de compilación
    if ($Analysis.Errors -gt 0) {
        $title = "🚨 Errores de Compilación Detectados"
        $body = @"
## 🚨 Errores de Compilación Críticos

**Errores encontrados:** $($Analysis.Errors)
**Warnings:** $($Analysis.Warnings)

### Detalles del Error:
```
$($Analysis.CargoCheck -join "`n")
```

### Acción Requerida:
- Revisar y corregir errores de compilación
- Ejecutar `cargo check` localmente
- Verificar dependencias en `Cargo.toml`

**Prioridad:** Alta
**Etiquetas:** bug,compilation,urgent
"@

        try {
            $issue = gh issue create --title $title --body $body --label "bug,compilation,urgent"
            $issuesCreated += @{Type="Compilation Errors"; URL=$issue}
            Write-Status "✅ Issue de errores de compilación creado" "Green"
        } catch {
            Write-Host "❌ Error creando issue de compilación: $($_.Exception.Message)" -ForegroundColor Red
        }
    }

    # Issue por archivos grandes
    if ($Analysis.LargeFiles.Count -gt 0) {
        $title = "📁 Archivos Grandes Detectados - Optimización Requerida"
        $body = @"
## 📁 Archivos Grandes que Requieren Optimización

Se encontraron $($Analysis.LargeFiles.Count) archivos grandes que pueden afectar el rendimiento:

$(foreach ($file in $Analysis.LargeFiles) {
    "- **$($file.FullName)**: $([math]::Round($file.Length / 1MB, 2)) MB`n"
})

### Recomendaciones:
- Considerar dividir archivos grandes en módulos más pequeños
- Revisar si hay código duplicado
- Evaluar uso de bibliotecas externas para funcionalidades complejas

**Etiquetas:** optimization,performance,refactor
"@

        try {
            $issue = gh issue create --title $title --body $body --label "optimization,performance,refactor"
            $issuesCreated += @{Type="Large Files"; URL=$issue}
            Write-Status "✅ Issue de archivos grandes creado" "Green"
        } catch {
            Write-Host "❌ Error creando issue de archivos grandes: $($_.Exception.Message)" -ForegroundColor Red
        }
    }

    return $issuesCreated
}

function New-AutoPR {
    param([string]$BranchName, [string]$Title, [string]$Body)

    Write-Status "🔄 Creando Pull Request automático..."

    try {
        # Verificar que hay cambios
        $status = Get-RepoStatus
        if (-not $status.HasChanges) {
            Write-Host "⚠️ No hay cambios para crear PR" -ForegroundColor Yellow
            return $null
        }

        # Crear y hacer push de la rama si no existe
        $branchExists = git branch --list $BranchName
        if (-not $branchExists) {
            git checkout -b $BranchName
            git add .
            git commit -m "Auto-commit: $Title"
            git push -u origin $BranchName
        }

        # Crear PR
        $pr = gh pr create --title $Title --body $Body --base main --head $BranchName --draft
        Write-Status "✅ PR creado: $pr" "Green"

        return $pr
    } catch {
        Write-Host "❌ Error creando PR: $($_.Exception.Message)" -ForegroundColor Red
        return $null
    }
}

function New-AutoRelease {
    param([string]$Version, [string]$Notes)

    Write-Status "📦 Creando release automático v$Version..."

    try {
        # Verificar que estamos en main y sin cambios pendientes
        $currentBranch = git rev-parse --abbrev-ref HEAD
        if ($currentBranch -ne "main") {
            Write-Host "❌ Debes estar en la rama main para crear releases" -ForegroundColor Red
            return $null
        }

        $status = Get-RepoStatus
        if ($status.HasChanges) {
            Write-Host "❌ Hay cambios pendientes sin commitear" -ForegroundColor Red
            return $null
        }

        # Crear tag
        git tag "v$Version"
        git push origin "v$Version"

        # Crear release
        $release = gh release create "v$Version" --title "Release v$Version" --notes $Notes
        Write-Status "✅ Release v$Version creado: $release" "Green"

        return $release
    } catch {
        Write-Host "❌ Error creando release: $($_.Exception.Message)" -ForegroundColor Red
        return $null
    }
}

function Watch-Workflows {
    Write-Status "👀 Monitoreando workflows..."

    try {
        # Obtener runs activos
        $activeRuns = gh run list --status in_progress --limit 10 --json number,status,conclusion,createdAt,headSha | ConvertFrom-Json

        if ($activeRuns.Count -eq 0) {
            Write-Host "ℹ️ No hay workflows ejecutándose actualmente" -ForegroundColor Blue
            return
        }

        Write-Host "🔄 Workflows activos:" -ForegroundColor Cyan
        foreach ($run in $activeRuns) {
            Write-Host "  - Run #$($run.number): $($run.status) ($($run.createdAt))" -ForegroundColor Yellow
        }

        # Monitorear progreso
        Write-Host "`n⏳ Esperando completación..." -ForegroundColor Cyan
        do {
            Start-Sleep -Seconds 10
            $updatedRuns = gh run list --status in_progress --limit 10 --json number,status,conclusion | ConvertFrom-Json
            $activeRuns = $updatedRuns | Where-Object { $_.status -eq "in_progress" }
            Write-Host "  🔄 $($activeRuns.Count) workflows aún ejecutándose..." -ForegroundColor Yellow
        } while ($activeRuns.Count -gt 0)

        Write-Host "✅ Todos los workflows completados" -ForegroundColor Green

        # Mostrar resultados finales
        $finalRuns = gh run list --limit 5 --json number,status,conclusion,createdAt | ConvertFrom-Json
        Write-Host "`n📊 Resultados finales:" -ForegroundColor Cyan
        foreach ($run in $finalRuns) {
            $color = if ($run.conclusion -eq "success") { "Green" } elseif ($run.conclusion -eq "failure") { "Red" } else { "Yellow" }
            Write-Host "  - Run #$($run.number): $($run.conclusion)" -ForegroundColor $color
        }

    } catch {
        Write-Host "❌ Error monitoreando workflows: $($_.Exception.Message)" -ForegroundColor Red
    }
}

function Invoke-RepoCleanup {
    Write-Status "🧹 Ejecutando limpieza automática del repositorio..."

    try {
        # Ejecutar script de limpieza existente
        if (Test-Path "scripts\auto_clean_repo.ps1") {
            & ".\scripts\auto_clean_repo.ps1"
        }

        # Limpiar cache de cargo
        cargo cache -a

        # Limpiar target directory
        if (Test-Path "target") {
            Remove-Item "target" -Recurse -Force
        }

        # Limpiar archivos temporales
        Get-ChildItem -Recurse -Include "*.tmp","*.log","*.cache" | Remove-Item -Force

        Write-Status "✅ Limpieza completada" "Green"

    } catch {
        Write-Host "❌ Error en limpieza: $($_.Exception.Message)" -ForegroundColor Red
    }
}

# Función principal
function Invoke-Main {
    # Verificaciones iniciales
    if (-not (Test-GitHubCLI)) { exit 1 }
    if (-not (Test-Authentication)) { exit 1 }

    Write-Host "🚀 NUCLEAR MCP AUTOMATION STARTING..." -ForegroundColor Magenta
    Write-Host "=" * 50 -ForegroundColor Magenta

    switch ($Action) {
        "check" {
            $status = Get-RepoStatus
            if ($status) {
                Write-Host "`n📊 ESTADO DEL REPOSITORIO:" -ForegroundColor Cyan
                Write-Host "Branch actual: $($status.CurrentBranch)" -ForegroundColor White
                Write-Host "Último commit: $($status.LastCommit)" -ForegroundColor White
                Write-Host "Cambios pendientes: $(if ($status.HasChanges) { 'Sí' } else { 'No' })" -ForegroundColor $(if ($status.HasChanges) { 'Yellow' } else { 'Green' })
                Write-Host "Issues abiertas: $($status.OpenIssues.Count)" -ForegroundColor White
                Write-Host "PRs abiertas: $($status.OpenPRs.Count)" -ForegroundColor White

                if ($status.RecentWorkflows.Count -gt 0) {
                    Write-Host "`n🔄 Últimos workflows:" -ForegroundColor Cyan
                    foreach ($wf in $status.RecentWorkflows) {
                        $color = switch ($wf.conclusion) {
                            "success" { "Green" }
                            "failure" { "Red" }
                            default { "Yellow" }
                        }
                        Write-Host "  $($wf.status)/$($wf.conclusion) - $($wf.createdAt)" -ForegroundColor $color
                    }
                }
            }
        }

        "analyze" {
            $analysis = Invoke-CodeAnalysis
            if ($analysis) {
                Write-Host "`n🔬 RESULTADOS DEL ANÁLISIS:" -ForegroundColor Cyan
                Write-Host "Errores: $($analysis.Errors)" -ForegroundColor $(if ($analysis.Errors -gt 0) { 'Red' } else { 'Green' })
                Write-Host "Warnings: $($analysis.Warnings)" -ForegroundColor $(if ($analysis.Warnings -gt 0) { 'Yellow' } else { 'Green' })
                Write-Host "Archivos grandes: $($analysis.LargeFiles.Count)" -ForegroundColor $(if ($analysis.LargeFiles.Count -gt 0) { 'Yellow' } else { 'Green' })

                if ($analysis.LargeFiles.Count -gt 0) {
                    Write-Host "`n📁 Archivos grandes:" -ForegroundColor Yellow
                    foreach ($file in $analysis.LargeFiles) {
                        Write-Host "  $($file.FullName) - $([math]::Round($file.Length / 1MB, 2)) MB" -ForegroundColor White
                    }
                }
            }
        }

        "create-issues" {
            $analysis = Invoke-CodeAnalysis
            if ($analysis) {
                $issues = New-AutoIssues -Analysis $analysis
                Write-Host "`n📝 ISSUES CREADOS:" -ForegroundColor Cyan
                foreach ($issue in $issues) {
                    Write-Host "  ✅ $($issue.Type): $($issue.URL)" -ForegroundColor Green
                }
            }
        }

        "create-pr" {
            if (-not $Title -or -not $Body) {
                Write-Host "❌ Se requieren -Title y -Body para crear PR" -ForegroundColor Red
                exit 1
            }

            $pr = New-AutoPR -BranchName $Branch -Title $Title -Body $Body
            if ($pr) {
                Write-Host "`n🔄 PULL REQUEST CREADO:" -ForegroundColor Green
                Write-Host "  $pr" -ForegroundColor White
            }
        }

        "release" {
            if (-not $Title) {
                Write-Host "❌ Se requiere -Title (versión) para crear release" -ForegroundColor Red
                exit 1
            }

            $release = New-AutoRelease -Version $Title -Notes $Body
            if ($release) {
                Write-Host "`n📦 RELEASE CREADO:" -ForegroundColor Green
                Write-Host "  $release" -ForegroundColor White
            }
        }

        "monitor" {
            Watch-Workflows
        }

        "cleanup" {
            Invoke-RepoCleanup
        }
    }

    Write-Host "`n" + "=" * 50 -ForegroundColor Magenta
    Write-Host "✅ NUCLEAR MCP AUTOMATION COMPLETED" -ForegroundColor Magenta
}

# Ejecutar función principal
Invoke-Main
