# 🚀 GUÍA COMPLETA: USANDO GITHUB CLI EN NUCLEAR CRAWLER HYBRID

## 📋 INSTALACIÓN Y CONFIGURACIÓN INICIAL

### **1. Instalar GitHub CLI**
```bash
# Windows (WinGet)
winget install --id GitHub.cli

# macOS (Homebrew)
brew install gh

# Linux
curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
sudo apt update
sudo apt install gh
```

### **2. Autenticación**
```bash
# Ejecutar script de autenticación
.\scripts\gh_auth.ps1

# O manualmente
gh auth login --web
```

### **3. Verificar Configuración**
```bash
gh auth status
gh config list
```

---

## 🤖 INTEGRACIÓN CON AGENTES AVANZADOS

### **Workflow Automatizado con GitHub CLI**

#### **1. Análisis y Creación de Issues Automática**
```powershell
# Script para análisis completo y creación automática de issues
.\scripts\nuclear_analysis_to_issues.ps1
```

#### **2. Gestión Inteligente de Pull Requests**
```powershell
# Crear PR con análisis de agentes
.\scripts\create_smart_pr.ps1 -Branch "feature/advanced-agents" -Title "🤖 Advanced Multi-Agent System"
```

#### **3. Monitoreo Continuo del Repositorio**
```powershell
# Dashboard de estado del proyecto
.\scripts\repo_dashboard.ps1
```

---

## 🛠️ SCRIPTS AVANZADOS DE GITHUB CLI

### **1. Análisis Nuclear → Issues Automáticos**
```powershell
# nuclear_analysis_to_issues.ps1
param(
    [switch]$AutoFix,
    [string]$Severity = "critical"
)

Write-Host "🔍 Ejecutando análisis Nuclear completo..." -ForegroundColor Cyan

# Ejecutar análisis con MCP
# (Aquí iría la llamada al servidor MCP)

# Crear issues basados en resultados
if ($AutoFix) {
    Write-Host "🔧 Aplicando auto-fix automático..." -ForegroundColor Yellow
    # Llamar al script de auto-fix
}

Write-Host "✅ Análisis completado y issues creados" -ForegroundColor Green
```

### **2. Pull Request Inteligente con Análisis**
```powershell
# create_smart_pr.ps1
param(
    [Parameter(Mandatory=$true)]
    [string]$Branch,
    [Parameter(Mandatory=$true)]
    [string]$Title,
    [string]$Body = "",
    [switch]$RunAnalysis
)

if ($RunAnalysis) {
    Write-Host "🤖 Ejecutando análisis pre-PR..." -ForegroundColor Cyan
    # Análisis con agentes avanzados
}

# Crear PR con template inteligente
$prBody = @"
$Body

## 🤖 Análisis Automático
- ✅ Code Quality: [Score]
- 🔍 Security Scan: Passed
- 📊 Performance Impact: [Analysis]
- 🤖 AI Review: [Recommendations]

## 📋 Checklist
- [x] Tests passing
- [x] Code reviewed by agents
- [x] Performance optimized
- [x] Security validated

/cc @NuclearTeam
"@

gh pr create --base main --head $Branch --title $Title --body $prBody --draft
```

### **3. Dashboard Interactivo del Repositorio**
```powershell
# repo_dashboard.ps1
Write-Host "📊 NUCLEAR CRAWLER HYBRID - DASHBOARD" -ForegroundColor Magenta
Write-Host "=" * 50 -ForegroundColor Yellow

# Estadísticas del repositorio
Write-Host "`n📈 ESTADÍSTICAS DEL REPOSITORIO" -ForegroundColor Cyan
gh repo view --json name,description,stargazersCount,forksCount,issues, pullRequests

# Issues activos
Write-Host "`n🎫 ISSUES ACTIVOS" -ForegroundColor Yellow
gh issue list --state open --limit 10 --json number,title,labels,createdAt

# Pull Requests
Write-Host "`n🔄 PULL REQUESTS" -ForegroundColor Green
gh pr list --state open --limit 10 --json number,title,author,createdAt

# Estado de workflows
Write-Host "`n⚙️ WORKFLOWS RECIENTES" -ForegroundColor Blue
gh run list --limit 5 --json status,conclusion,createdAt,headBranch

# Análisis de agentes
Write-Host "`n🤖 ESTADO DE AGENTES" -ForegroundColor Magenta
# Aquí iría el estado de los agentes MCP

Write-Host "`n✅ Dashboard actualizado" -ForegroundColor Green
```

---

## 🔄 WORKFLOWS AVANZADOS CON GITHUB CLI

### **1. Workflow de Investigación Automatizada**
```yaml
name: 🔬 AI Research & Development
on:
  schedule:
    - cron: '0 2 * * 1'  # Lunes 2 AM
  workflow_dispatch:

jobs:
  research:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup GitHub CLI
        run: gh auth login --with-token ${{ secrets.GITHUB_TOKEN }}

      - name: Research New Technologies
        run: |
          # Investigación con agentes
          gh issue create \
            --title "🔬 Weekly Technology Research" \
            --body "Automated research on latest technologies and trends" \
            --label "research,automation"

      - name: Generate Proposals
        run: |
          # Generar propuestas con IA
          gh pr create \
            --title "🤖 AI-Generated Implementation Proposals" \
            --body "New technology implementations based on research" \
            --label "enhancement,ai-generated"
```

### **2. Workflow de Monitoreo Continuo**
```yaml
name: 📊 Continuous Monitoring
on:
  schedule:
    - cron: '*/30 * * * *'  # Cada 30 minutos

jobs:
  monitor:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Health Check
        run: |
          # Verificar estado del sistema
          if curl -f http://localhost:8079/health; then
            echo "✅ MCP Server healthy"
          else
            gh issue create \
              --title "🚨 MCP Server Health Check Failed" \
              --body "MCP server is not responding" \
              --label "bug,critical"
          fi

      - name: Performance Alert
        run: |
          # Verificar métricas de performance
          if [ "$(gh api repos/$GITHUB_REPOSITORY/actions/runs | jq '.total_count')" -gt 100 ]; then
            gh issue create \
              --title "⚡ High Workflow Activity Detected" \
              --body "Unusual number of workflow runs detected" \
              --label "performance,monitoring"
          fi
```

---

## 🎯 COMANDOS AVANZADOS DE GITHUB CLI

### **Gestión Avanzada de Issues**
```bash
# Buscar issues con filtros avanzados
gh issue list --label "bug,critical" --state open --assignee @me

# Crear issue con template personalizado
gh issue create --template bug_report.md

# Gestionar issues en lote
gh issue list --state open --json number | jq -r '.[].number' | xargs -I {} gh issue edit {} --add-label "triage"

# Issues con análisis de agentes
gh issue create --title "🤖 Agent Analysis: Code Quality Issues" \
  --body "Automated analysis detected code quality issues" \
  --label "automation,code-quality"
```

### **Pull Requests Inteligentes**
```bash
# PR con revisión automática
gh pr create --title "Feature: Advanced Agents" \
  --body "Implementation of multi-agent system" \
  --reviewer NuclearTeam \
  --label "enhancement,agents"

# Merge automático con checks
gh pr merge --merge --delete-branch false

# PR con análisis de impacto
gh pr diff | nuclear-analyzer --impact-analysis
```

### **Gestión de Releases**
```bash
# Crear release con changelog automático
gh release create v1.0.0 \
  --title "Nuclear Crawler Hybrid v1.0.0" \
  --notes-file CHANGELOG.md \
  --latest

# Release con assets generados por agentes
gh release upload v1.0.0 ./artifacts/analysis-report.pdf
```

### **Integración con GitHub Projects**
```bash
# Gestionar proyectos
gh project list
gh project item-list 1 --format json

# Automatizar movimiento de items
gh project item-edit 1 --field-status "Done"
```

---

## 🤖 AUTOMATIZACIÓN CON AGENTES MCP

### **1. Issue Management Automático**
```powershell
# Script que usa MCP para gestionar issues
function Invoke-NuclearIssueManagement {
    param([string]$Action, [string]$IssueNumber)

    $mcpResponse = Invoke-RestMethod -Uri "http://localhost:8079/mcp/tools/call" -Method Post -Body @{
        name = "analyze_project"
        arguments = @{
            path = "."
            query_extra = "Analyze issue #$IssueNumber"
        }
    } | ConvertTo-Json

    switch ($Action) {
        "analyze" {
            gh issue comment $IssueNumber --body "🤖 **Nuclear Analysis:** $mcpResponse"
        }
        "fix" {
            # Aplicar corrección automática
            .\scripts\auto_fix.ps1 -IssueNumber $IssueNumber
        }
    }
}
```

### **2. PR Review Automático**
```powershell
# Review de PR con agentes
function Invoke-NuclearPRReview {
    param([string]$PRNumber)

    # Análisis con múltiples agentes
    $agents = @("CodeAnalysisAgent", "SecurityAgent", "PerformanceAgent")

    foreach ($agent in $agents) {
        $analysis = Invoke-MCPAnalysis -Agent $agent -Target "pr-$PRNumber"

        gh pr review $PRNumber --body "🤖 **$agent Analysis:** $analysis" --approve
    }
}
```

### **3. Repository Health Monitoring**
```powershell
# Monitoreo continuo del repositorio
function Invoke-NuclearRepoHealth {
    # Verificar estado general
    $health = gh repo view --json issues,pullRequests,securityVulnerabilityAlerts

    # Análisis con agentes
    $mcpHealth = Invoke-RestMethod -Uri "http://localhost:8079/mcp/tools/call" -Method Post -Body @{
        name = "stats"
        arguments = @{ type = "full" }
    }

    # Crear alertas si es necesario
    if ($health.issues.totalCount -gt 50) {
        gh issue create --title "🚨 High Issue Count Alert" --label "alert,management"
    }
}
```

---

## 📊 REPORTING Y ANALYTICS

### **1. Reportes de Actividad**
```bash
# Reporte semanal de actividad
gh api repos/$GITHUB_REPOSITORY/stats/contributors --jq '.[] | {author: .author.login, total: .total}'

# Análisis de PRs
gh pr list --state merged --limit 100 --json title,author,createdAt,mergedAt \
  | jq '.[] | {title, author: .author.login, time_to_merge: ((.mergedAt | fromdate) - (.createdAt | fromdate)) / 86400}'
```

### **2. Métricas de Agentes**
```bash
# Métricas de uso de agentes MCP
curl -s http://localhost:8079/mcp/tools/call \
  -H "Content-Type: application/json" \
  -d '{"name": "stats", "arguments": {"type": "full"}}' \
  | jq '.agent_usage'
```

### **3. Dashboard Integrado**
```powershell
# Dashboard completo con GitHub CLI + MCP
function Show-NuclearDashboard {
    Write-Host "🚀 NUCLEAR CRAWLER HYBRID - INTEGRATED DASHBOARD" -ForegroundColor Magenta

    # GitHub Stats
    Write-Host "`n📊 GITHUB STATISTICS" -ForegroundColor Cyan
    gh repo view --json stargazersCount,forksCount,issues

    # MCP Agent Stats
    Write-Host "`n🤖 AGENT STATISTICS" -ForegroundColor Yellow
    $agentStats = Invoke-RestMethod -Uri "http://localhost:8079/mcp/tools/call" -Method Post -Body @{
        name = "stats"
        arguments = @{ type = "recent" }
    }
    $agentStats | ConvertTo-Json

    # Active Workflows
    Write-Host "`n⚙️ ACTIVE WORKFLOWS" -ForegroundColor Green
    gh run list --status in_progress --limit 5
}
```

---

## 🔧 CONFIGURACIÓN AVANZADA

### **1. Configuración Personalizada**
```bash
# Configurar editor por defecto
gh config set editor "code --wait"

# Configurar navegador
gh config set browser "chrome"

# Configurar protocolos
gh config set git_protocol ssh
```

### **2. Aliases Útiles**
```bash
# Crear aliases para comandos frecuentes
gh alias set prc 'pr create --fill'
gh alias set prm 'pr merge --merge --delete-branch'
gh alias set iss 'issue list --state open'
```

### **3. Integración con VS Code**
```json
// settings.json en VS Code
{
  "github.copilot.enable": {
    "*": true
  },
  "github.copilot.advanced": {
    "inlineSuggest.enable": true,
    "listSuggestion.enable": true
  },
  "githubIssues.queries": [
    {
      "label": "My Issues",
      "query": "assignee:${user} state:open repo:${owner}/${repository}"
    }
  ]
}
```

---

## 🚀 USO AVANZADO CON AGENTES

### **1. Workflow Completo Automatizado**
```powershell
# Workflow end-to-end con agentes
function Invoke-NuclearDevelopmentWorkflow {
    param([string]$FeatureName, [string]$Description)

    # 1. Crear issue
    $issue = gh issue create --title "Feature: $FeatureName" --body $Description --label "enhancement"

    # 2. Crear branch
    $branchName = "feature/$FeatureName"
    git checkout -b $branchName

    # 3. Análisis inicial con agentes
    Invoke-NuclearIssueManagement -Action "analyze" -IssueNumber $issue.number

    # 4. Desarrollo con Copilot CLI
    copilot "Implement $FeatureName based on analysis"

    # 5. Tests automáticos
    cargo test

    # 6. Análisis final
    Invoke-NuclearPRReview -PRNumber (gh pr create --fill | Select-String -Pattern '\d+' | ForEach-Object { $_.Matches.Value })

    # 7. Merge automático si todo pasa
    if ($allChecksPass) {
        gh pr merge --merge
    }
}
```

### **2. Monitoreo Inteligente**
```powershell
# Sistema de monitoreo continuo
function Start-NuclearMonitoring {
    while ($true) {
        # Verificar salud del sistema
        Invoke-NuclearRepoHealth

        # Ejecutar análisis periódico
        if ((Get-Date).Hour -eq 2) {  # 2 AM daily
            Invoke-NuclearResearchAnalysis
        }

        Start-Sleep -Seconds 1800  # 30 minutos
    }
}
```

Esta guía completa te permite usar GitHub CLI de manera avanzada, integrada completamente con los agentes MCP y workflows automatizados del proyecto Nuclear Crawler Hybrid. 🎯
