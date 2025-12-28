# 📁 Carpeta `scripts/` - Scripts de Build y Automatización

## 🎯 Propósito
Scripts de automatización para compilación, testing y mantenimiento del proyecto multi-lenguaje.

## 🏗️ Arquitectura
- **PowerShell scripts** para Windows
- **Rust scripts** para lógica compleja
- **Python scripts** para procesamiento de datos
- **Build automation** para FFI components

## 📂 Contenido
- `build_*.ps1` - Scripts de compilación por lenguaje
- `compile_*.ps1` - Scripts de compilación FFI
- `consolidate_and_cleanup.ps1` - Limpieza y consolidación
- `jax_*.py` - Scripts de procesamiento JAX
- `consolidate_websearch.rs` - Consolidación de resultados
- `gh_*.ps1` - Scripts de automatización GitHub CLI (issues/PRs)

## 🔧 Funciones
- `build_all.ps1` - Build completo del proyecto
- `compile_go_msvc.ps1` - Compilación Go con MSVC
- `compile_zig_msvc.ps1` - Compilación Zig con MSVC
- `consolidate_and_cleanup.ps1` - Mantenimiento del proyecto
- `gh_create_issue.ps1` - Crear issues en GitHub
- `gh_create_pr.ps1` - Crear pull requests
- `gh_list_issues.ps1` - Listar issues
- `gh_list_prs.ps1` - Listar pull requests
- `gh_close_issue.ps1` - Cerrar issues
- `gh_merge_pr.ps1` - Mergear pull requests
- `gh_auth.ps1` - Autenticar GitHub CLI
- `install_trae_cli.ps1` - Instalar TRAE CLI

## 🚀 Uso
```powershell
# Build completo
.\scripts\build_all.ps1

# Compilación específica
.\scripts\compile_go_msvc.ps1

# GitHub CLI - Crear issue
.\scripts\gh_create_issue.ps1 -Title "Bug: Crawler fails on timeout" -Body "Description..." -Labels "bug"

# GitHub CLI - Crear PR
.\scripts\gh_create_pr.ps1 -Title "Add new feature" -Body "Description..." -Branch "feature-branch"

# GitHub CLI - Listar issues/PRs
.\scripts\gh_list_issues.ps1 -State "open" -Labels "enhancement"
.\scripts\gh_list_prs.ps1 -State "open"

# GitHub CLI - Cerrar issue
.\scripts\gh_close_issue.ps1 -Number 123 -Reason "completed"

# GitHub CLI - Autenticar
.\scripts\gh_auth.ps1

# Instalar TRAE CLI
.\scripts\install_trae_cli.ps1
```

## 🤖 Contexto para IA
Scripts que automatizan:

- **Compilación multi-lenguaje** (Go, Zig, Nim, Rust)
- **Generación de bindings** FFI
- **Consolidación de datos** de búsqueda
- **Mantenimiento del proyecto** y cleanup
- **Gestión de GitHub** (issues, PRs, repositorio)

**Patrón**: Scripts idempotentes que pueden ejecutarse múltiples veces sin efectos secundarios.
