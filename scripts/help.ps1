#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Interactive help system for Nuclear Crawler Hybrid
.DESCRIPTION
    Shows available tools, commands, and documentation in an interactive menu
#>

function Show-Header {
    Clear-Host
    Write-Host @"

╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║   🚀  NUCLEAR CRAWLER HYBRID - HELP SYSTEM                   ║
║                                                               ║
║   Professional Development Tools & Documentation             ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝

"@ -ForegroundColor Cyan
}

function Show-MainMenu {
    Show-Header
    
    Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor DarkGray
    Write-Host ""
    Write-Host "  📚 CATEGORIES" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  [1] ⌨️  Keyboard Shortcuts" -ForegroundColor Green
    Write-Host "  [2] 🐛 Visual Debugging" -ForegroundColor Green
    Write-Host "  [3] ▶️  Task Runner" -ForegroundColor Green
    Write-Host "  [4] ✨ Code Snippets" -ForegroundColor Green
    Write-Host "  [5] 🤖 PowerShell Automation" -ForegroundColor Green
    Write-Host "  [6] 🔨 Makefile Pro" -ForegroundColor Green
    Write-Host "  [7] 📚 Documentation" -ForegroundColor Green
    Write-Host "  [8] 🚦 CI/CD & Pre-Commit" -ForegroundColor Green
    Write-Host "  [9] 🎯 Common Workflows" -ForegroundColor Green
    Write-Host "  [D] 🌐 Open Dashboard (HTML)" -ForegroundColor Cyan
    Write-Host "  [Q] ❌ Quit" -ForegroundColor Red
    Write-Host ""
    Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor DarkGray
    Write-Host ""
}

function Show-KeyboardShortcuts {
    Show-Header
    Write-Host "⌨️  KEYBOARD SHORTCUTS" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  Ctrl+Shift+B     → Build All (Rust+Chapel+Go)" -ForegroundColor White
    Write-Host "  Ctrl+Shift+T     → Validate 5 MCP Tools (CRITICAL)" -ForegroundColor White
    Write-Host "  Ctrl+Shift+R     → Run MCP Server" -ForegroundColor White
    Write-Host "  Ctrl+Shift+C     → Clean All" -ForegroundColor White
    Write-Host "  F5               → Start Debugging" -ForegroundColor White
    Write-Host "  Ctrl+K Ctrl+D    → Debug MCP Server" -ForegroundColor White
    Write-Host "  Ctrl+K Ctrl+T    → Debug Test: 5 MCP Tools" -ForegroundColor White
    Write-Host ""
    Write-Host "  📝 Archivo: .vscode/keybindings.json" -ForegroundColor DarkGray
    Write-Host ""
    Read-Host "Presiona Enter para continuar"
}

function Show-Debugging {
    Show-Header
    Write-Host "🐛 VISUAL DEBUGGING" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  Cómo usar:" -ForegroundColor Cyan
    Write-Host "    1. Abre cualquier archivo .rs" -ForegroundColor White
    Write-Host "    2. Click en número de línea (aparece círculo rojo)" -ForegroundColor White
    Write-Host "    3. Presiona F5" -ForegroundColor White
    Write-Host "    4. El código se detiene en el breakpoint" -ForegroundColor White
    Write-Host "    5. Inspecciona variables en panel izquierdo" -ForegroundColor White
    Write-Host ""
    Write-Host "  Configuraciones disponibles:" -ForegroundColor Cyan
    Write-Host "    • 🚀 Debug MCP Server" -ForegroundColor White
    Write-Host "    • 🧪 Debug Test: 5 MCP Tools" -ForegroundColor White
    Write-Host "    • 🔬 Debug Integration Test" -ForegroundColor White
    Write-Host "    • 🐹 Debug Go GitHub MCP" -ForegroundColor White
    Write-Host "    • ⚡ Attach to Running Process" -ForegroundColor White
    Write-Host "    • 🎯 Debug Full Stack (Rust + Go)" -ForegroundColor White
    Write-Host ""
    Write-Host "  Controles:" -ForegroundColor Cyan
    Write-Host "    F5        → Continue" -ForegroundColor White
    Write-Host "    F10       → Step Over" -ForegroundColor White
    Write-Host "    F11       → Step Into" -ForegroundColor White
    Write-Host "    Shift+F11 → Step Out" -ForegroundColor White
    Write-Host "    F9        → Toggle Breakpoint" -ForegroundColor White
    Write-Host ""
    Write-Host "  📝 Archivo: .vscode/launch.json" -ForegroundColor DarkGray
    Write-Host ""
    Read-Host "Presiona Enter para continuar"
}

function Show-TaskRunner {
    Show-Header
    Write-Host "▶️  TASK RUNNER" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  Cómo usar:" -ForegroundColor Cyan
    Write-Host "    Ctrl+Shift+P → 'Tasks: Run Task'" -ForegroundColor White
    Write-Host ""
    Write-Host "  Tasks disponibles:" -ForegroundColor Cyan
    Write-Host "    🔨 Build All (Rust+Chapel+Go)" -ForegroundColor White
    Write-Host "    ✅ Validate 5 MCP Tools (CRITICAL)" -ForegroundColor White
    Write-Host "    🚀 Run MCP Server" -ForegroundColor White
    Write-Host "    🧠 Train Chapel AI" -ForegroundColor White
    Write-Host "    ⚡ Run All Chapel Systems" -ForegroundColor White
    Write-Host "    🔬 Data Mining" -ForegroundColor White
    Write-Host "    📊 Scientific Analysis" -ForegroundColor White
    Write-Host "    🧪 Integration Tests (Real Server)" -ForegroundColor White
    Write-Host "    🧹 Clean All" -ForegroundColor White
    Write-Host "    📝 Format Code (Rust+Go)" -ForegroundColor White
    Write-Host ""
    Write-Host "  📝 Archivo: .vscode/tasks.json" -ForegroundColor DarkGray
    Write-Host ""
    Read-Host "Presiona Enter para continuar"
}

function Show-Snippets {
    Show-Header
    Write-Host "✨ CODE SNIPPETS" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  En archivo .rs escribe:" -ForegroundColor Cyan
    Write-Host "    mcp-tool      + Tab → Template de MCP tool (NO MOCKS)" -ForegroundColor White
    Write-Host "    mcp-test      + Tab → Test que verifica 5 tools" -ForegroundColor White
    Write-Host "    error-handle  + Tab → Manejo de errores Rust" -ForegroundColor White
    Write-Host ""
    Write-Host "  En archivo .chpl escribe:" -ForegroundColor Cyan
    Write-Host "    chapel-train  + Tab → Función Chapel con coforall" -ForegroundColor White
    Write-Host "    coforall      + Tab → Loop paralelo Chapel" -ForegroundColor White
    Write-Host ""
    Write-Host "  📝 Archivo: .vscode/nuclear.code-snippets" -ForegroundColor DarkGray
    Write-Host ""
    Read-Host "Presiona Enter para continuar"
}

function Show-Automation {
    Show-Header
    Write-Host "🤖 POWERSHELL AUTOMATION" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  Script: .\scripts\dev-workflow.ps1" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  Comandos:" -ForegroundColor Cyan
    Write-Host "    -Action validate    → Valida constraints críticos" -ForegroundColor White
    Write-Host "    -Action build       → Build completo o por componente" -ForegroundColor White
    Write-Host "    -Action test        → Run tests (all/rust/chapel/go)" -ForegroundColor White
    Write-Host "    -Action pre-commit  → Checks completos pre-commit" -ForegroundColor White
    Write-Host "    -Action watch       → Auto-rebuild on changes" -ForegroundColor White
    Write-Host "    -Action clean       → Limpia build artifacts" -ForegroundColor White
    Write-Host "    -Action format      → Format código" -ForegroundColor White
    Write-Host ""
    Write-Host "  Ejemplos:" -ForegroundColor Cyan
    Write-Host "    .\scripts\dev-workflow.ps1 -Action validate" -ForegroundColor Green
    Write-Host "    .\scripts\dev-workflow.ps1 -Action build -Component rust" -ForegroundColor Green
    Write-Host "    .\scripts\dev-workflow.ps1 -Action pre-commit" -ForegroundColor Green
    Write-Host ""
    Write-Host "  Features:" -ForegroundColor Cyan
    Write-Host "    ✅ Logging automático (logs/dev-workflow-*.log)" -ForegroundColor White
    Write-Host "    ✅ Colored output (errores rojos, success verde)" -ForegroundColor White
    Write-Host "    ✅ Valida 3 restricciones críticas" -ForegroundColor White
    Write-Host "    ✅ Detección de mock code" -ForegroundColor White
    Write-Host ""
    Read-Host "Presiona Enter para continuar"
}

function Show-Makefile {
    Show-Header
    Write-Host "🔨 MAKEFILE PRO" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  Comando base: make -f Makefile.pro" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  Targets principales:" -ForegroundColor Cyan
    Write-Host "    help         → Ver todos los comandos" -ForegroundColor White
    Write-Host "    all          → Build + test todo" -ForegroundColor White
    Write-Host "    validate     → Valida constraints críticos" -ForegroundColor White
    Write-Host "    build        → Build all components" -ForegroundColor White
    Write-Host "    test         → Run all tests" -ForegroundColor White
    Write-Host "    pre-commit   → Pre-commit checks completos" -ForegroundColor White
    Write-Host "    watch        → Auto-rebuild on changes" -ForegroundColor White
    Write-Host "    ci           → Simula CI localmente" -ForegroundColor White
    Write-Host "    stats        → Project statistics" -ForegroundColor White
    Write-Host "    clean        → Limpia build artifacts" -ForegroundColor White
    Write-Host ""
    Write-Host "  Targets por componente:" -ForegroundColor Cyan
    Write-Host "    build-rust   → Build solo Rust" -ForegroundColor White
    Write-Host "    build-chapel → Build solo Chapel" -ForegroundColor White
    Write-Host "    build-go     → Build solo Go" -ForegroundColor White
    Write-Host "    test-rust    → Test solo Rust" -ForegroundColor White
    Write-Host "    test-chapel  → Test solo Chapel" -ForegroundColor White
    Write-Host "    test-go      → Test solo Go" -ForegroundColor White
    Write-Host ""
    Write-Host "  Ejemplo:" -ForegroundColor Cyan
    Write-Host "    make -f Makefile.pro validate" -ForegroundColor Green
    Write-Host ""
    Read-Host "Presiona Enter para continuar"
}

function Show-Documentation {
    Show-Header
    Write-Host "📚 DOCUMENTATION" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  Documentos principales:" -ForegroundColor Cyan
    Write-Host "    START_HERE.md            → Tu punto de entrada único" -ForegroundColor White
    Write-Host "    PRO_FEATURES.md          → Guía completa features PRO" -ForegroundColor White
    Write-Host "    QUICK_PRO_START.md       → Setup en 5 minutos" -ForegroundColor White
    Write-Host "    PROMPTS.md               → +50 prompts para Cursor AI" -ForegroundColor White
    Write-Host "    COMMANDS_CHEATSHEET.md   → Comandos de una línea" -ForegroundColor White
    Write-Host ""
    Write-Host "  Proyecto:" -ForegroundColor Cyan
    Write-Host "    README.md                → Overview del proyecto" -ForegroundColor White
    Write-Host "    INTEGRATION_STATUS.md    → Estado de integración" -ForegroundColor White
    Write-Host "    QUICK_START.md           → Instalación básica" -ForegroundColor White
    Write-Host ""
    Write-Host "  Visual:" -ForegroundColor Cyan
    Write-Host "    docs/dashboard.html      → Dashboard HTML interactivo" -ForegroundColor White
    Write-Host ""
    Write-Host "  Abrir:" -ForegroundColor Cyan
    Write-Host "    code START_HERE.md       → En Cursor" -ForegroundColor Green
    Write-Host "    start docs\dashboard.html → En navegador (Windows)" -ForegroundColor Green
    Write-Host ""
    Read-Host "Presiona Enter para continuar"
}

function Show-CICD {
    Show-Header
    Write-Host "🚦 CI/CD & PRE-COMMIT" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  GitHub Actions CI/CD:" -ForegroundColor Cyan
    Write-Host "    Archivo: .github/workflows/ci.yml" -ForegroundColor White
    Write-Host "    Corre en: git push / pull request" -ForegroundColor White
    Write-Host ""
    Write-Host "    Jobs:" -ForegroundColor Cyan
    Write-Host "      ✅ Valida 5 MCP tools (CRÍTICO)" -ForegroundColor White
    Write-Host "      🚫 Detecta mock code" -ForegroundColor White
    Write-Host "      🔨 Build multi-OS (Linux/Windows/Mac)" -ForegroundColor White
    Write-Host "      🧪 Unit + Integration tests" -ForegroundColor White
    Write-Host "      🔐 Security audit" -ForegroundColor White
    Write-Host ""
    Write-Host "  Pre-Commit Hook:" -ForegroundColor Cyan
    Write-Host "    Archivo: .husky/pre-commit" -ForegroundColor White
    Write-Host "    Corre en: git commit (automático)" -ForegroundColor White
    Write-Host ""
    Write-Host "    Valida:" -ForegroundColor Cyan
    Write-Host "      📝 Format check (cargo fmt --check)" -ForegroundColor White
    Write-Host "      ⚠️  5 MCP tools test (CRÍTICO)" -ForegroundColor White
    Write-Host "      🚫 Mock code detection" -ForegroundColor White
    Write-Host "      🔍 Clippy warnings" -ForegroundColor White
    Write-Host ""
    Write-Host "  Simular CI localmente:" -ForegroundColor Cyan
    Write-Host "    make -f Makefile.pro ci" -ForegroundColor Green
    Write-Host ""
    Write-Host "  Bypass hook (emergencias):" -ForegroundColor Cyan
    Write-Host "    git commit --no-verify -m 'Emergency'" -ForegroundColor Yellow
    Write-Host ""
    Read-Host "Presiona Enter para continuar"
}

function Show-Workflows {
    Show-Header
    Write-Host "🎯 COMMON WORKFLOWS" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  [1] Nueva Feature" -ForegroundColor Cyan
    Write-Host "      git checkout -b feature/my-feature" -ForegroundColor White
    Write-Host "      make -f Makefile.pro watch  # Auto-rebuild" -ForegroundColor White
    Write-Host "      # Desarrollar con F5 debugging" -ForegroundColor White
    Write-Host "      make -f Makefile.pro pre-commit" -ForegroundColor White
    Write-Host "      git commit -m 'feat: add feature'" -ForegroundColor White
    Write-Host "      git push origin feature/my-feature" -ForegroundColor White
    Write-Host ""
    Write-Host "  [2] Bug Fix" -ForegroundColor Cyan
    Write-Host "      # F5 → Debug con breakpoints" -ForegroundColor White
    Write-Host "      # Fix code" -ForegroundColor White
    Write-Host "      Ctrl+Shift+T  # Validate 5 tools" -ForegroundColor White
    Write-Host "      make -f Makefile.pro format" -ForegroundColor White
    Write-Host "      git commit -m 'fix: resolve bug'" -ForegroundColor White
    Write-Host ""
    Write-Host "  [3] Pre-PR Review" -ForegroundColor Cyan
    Write-Host "      .\scripts\dev-workflow.ps1 -Action pre-commit" -ForegroundColor White
    Write-Host "      # Si pasa ✅ → Safe to PR" -ForegroundColor White
    Write-Host "      # Si falla ❌ → Fix issues" -ForegroundColor White
    Write-Host ""
    Write-Host "  [4] Empezar el Día" -ForegroundColor Cyan
    Write-Host "      git pull" -ForegroundColor White
    Write-Host "      make -f Makefile.pro validate" -ForegroundColor White
    Write-Host "      git status && git diff" -ForegroundColor White
    Write-Host ""
    Read-Host "Presiona Enter para continuar"
}

function Open-Dashboard {
    $dashboardPath = Join-Path $PSScriptRoot "..\docs\dashboard.html"
    if (Test-Path $dashboardPath) {
        Write-Host "🌐 Abriendo dashboard..." -ForegroundColor Green
        Start-Process $dashboardPath
    } else {
        Write-Host "❌ Dashboard not found: $dashboardPath" -ForegroundColor Red
    }
    Start-Sleep -Seconds 2
}

# Main loop
do {
    Show-MainMenu
    $choice = Read-Host "Selecciona una opción"
    
    switch ($choice) {
        '1' { Show-KeyboardShortcuts }
        '2' { Show-Debugging }
        '3' { Show-TaskRunner }
        '4' { Show-Snippets }
        '5' { Show-Automation }
        '6' { Show-Makefile }
        '7' { Show-Documentation }
        '8' { Show-CICD }
        '9' { Show-Workflows }
        'D' { Open-Dashboard }
        'd' { Open-Dashboard }
        'Q' { 
            Write-Host "`n👋 ¡Hasta luego! Happy coding! 🚀`n" -ForegroundColor Cyan
            break
        }
        'q' { 
            Write-Host "`n👋 ¡Hasta luego! Happy coding! 🚀`n" -ForegroundColor Cyan
            break
        }
        default {
            Write-Host "`n❌ Opción inválida. Presiona Enter para continuar." -ForegroundColor Red
            Read-Host
        }
    }
} while ($choice -ne 'Q' -and $choice -ne 'q')
