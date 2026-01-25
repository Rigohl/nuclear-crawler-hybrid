# 🚀 START HERE - Nuclear Crawler Hybrid

**Tu punto de entrada único para todo el proyecto.**

---

## ⚡ QUICK ACCESS

### 🎯 Para Empezar (First Time)
```
1. Abre: QUICK_PRO_START.md (5 minutos setup)
2. Luego: Ctrl+Shift+P → "Help: Show Nuclear Dashboard"
```

### 🔨 Desarrollo Diario (Day to Day)

| Necesito | Presiona | Alternativamente |
|----------|----------|------------------|
| **Build todo** | `Ctrl+Shift+B` | `make -f Makefile.pro all` |
| **Validar 5 tools** | `Ctrl+Shift+T` | `.\scripts\dev-workflow.ps1 -Action validate` |
| **Run server** | `Ctrl+Shift+R` | `cargo run --bin nuclear-mcp` |
| **Debug** | `F5` | Click línea → Breakpoint → F5 |
| **Ver comandos** | `Ctrl+Shift+P` → "Tasks" | `make -f Makefile.pro help` |

---

## 📚 DOCUMENTACIÓN COMPLETA

### 🎓 Aprende las Herramientas
1. **[PRO_FEATURES.md](PRO_FEATURES.md)** - Guía completa de features profesionales
2. **[QUICK_PRO_START.md](QUICK_PRO_START.md)** - Setup en 5 minutos
3. **[Dashboard HTML](docs/dashboard.html)** - Abre en navegador

### 📖 Referencias Rápidas
4. **[PROMPTS.md](PROMPTS.md)** - +50 prompts para Cursor AI (copia/pega)
5. **[COMMANDS_CHEATSHEET.md](COMMANDS_CHEATSHEET.md)** - Comandos de una línea

### 🏗️ Entender el Proyecto
6. **[README.md](README.md)** - Overview del proyecto
7. **[INTEGRATION_STATUS.md](INTEGRATION_STATUS.md)** - Estado de integración
8. **[QUICK_START.md](QUICK_START.md)** - Instalación y setup básico

---

## 🎯 HERRAMIENTAS PROFESIONALES

### 1. Visual Debugging
```
Archivo: .vscode/launch.json
Uso: Presiona F5 → Breakpoints visuales
```

**Configuraciones disponibles**:
- 🚀 Debug MCP Server (`F5`)
- 🧪 Debug Test: 5 MCP Tools (`Ctrl+K Ctrl+T`)
- 🔬 Debug Integration Test
- 🐹 Debug Go GitHub MCP
- 🎯 Debug Full Stack (Rust + Go)

### 2. Task Runner
```
Archivo: .vscode/tasks.json
Uso: Ctrl+Shift+P → "Tasks: Run Task"
```

**Tasks disponibles** (con iconos):
- 🔨 Build All (Rust+Chapel+Go)
- ✅ Validate 5 MCP Tools (CRITICAL)
- 🚀 Run MCP Server
- 🧠 Train Chapel AI
- ⚡ Run All Chapel Systems
- 🔬 Data Mining
- 📊 Scientific Analysis
- 🧪 Integration Tests
- 🧹 Clean All
- 📝 Format Code

### 3. Code Snippets
```
Archivo: .vscode/nuclear.code-snippets
Uso: En archivo .rs escribe "mcp-tool" → Tab
```

**Snippets disponibles**:
- `mcp-tool` - Template de MCP tool
- `chapel-train` - Función Chapel con coforall
- `mcp-test` - Test que verifica 5 tools
- `coforall` - Loop paralelo Chapel
- `error-handle` - Manejo de errores Rust

### 4. Keyboard Shortcuts
```
Archivo: .vscode/keybindings.json
```

| Shortcut | Acción |
|----------|--------|
| `Ctrl+Shift+B` | Build All |
| `Ctrl+Shift+T` | Validate 5 Tools |
| `Ctrl+Shift+R` | Run Server |
| `Ctrl+Shift+C` | Clean All |
| `F5` | Debug |
| `Ctrl+K Ctrl+D` | Debug MCP |
| `Ctrl+K Ctrl+T` | Debug Test |

### 5. PowerShell Automation
```
Archivo: scripts/dev-workflow.ps1
```

```powershell
# Validar proyecto
.\scripts\dev-workflow.ps1 -Action validate

# Build específico
.\scripts\dev-workflow.ps1 -Action build -Component rust

# Pre-commit checks
.\scripts\dev-workflow.ps1 -Action pre-commit

# Watch mode
.\scripts\dev-workflow.ps1 -Action watch
```

### 6. Professional Makefile
```
Archivo: Makefile.pro
```

```bash
make -f Makefile.pro help       # Ver comandos
make -f Makefile.pro all        # Build + test
make -f Makefile.pro validate   # Valida constraints
make -f Makefile.pro pre-commit # Pre-commit checks
make -f Makefile.pro watch      # Auto-rebuild
make -f Makefile.pro ci         # Simula CI
```

### 7. GitHub Actions CI/CD
```
Archivo: .github/workflows/ci.yml
Auto-corre en: git push
```

Ver resultados: GitHub repo → Actions tab

### 8. Pre-Commit Hooks
```
Archivo: .husky/pre-commit
Auto-valida antes de: git commit
```

---

## 🎨 DASHBOARD VISUAL

### Opción 1: HTML Dashboard (Recomendado)
```bash
# Abre en tu navegador
open docs/dashboard.html        # Mac/Linux
start docs/dashboard.html       # Windows
```

### Opción 2: Terminal Dashboard
```powershell
.\scripts\help.ps1              # Muestra menú interactivo
```

### Opción 3: Cursor Command Palette
```
Ctrl+Shift+P → "Help: Show Nuclear Dashboard"
```

---

## 🚦 WORKFLOWS COMUNES

### Workflow 1: Empezar el Día
```bash
# 1. Pull latest
git pull

# 2. Validar todo está bien
make -f Makefile.pro validate

# 3. Ver qué cambió
git status
git diff
```

### Workflow 2: Nueva Feature
```bash
# 1. Branch
git checkout -b feature/my-feature

# 2. Develop con watch
make -f Makefile.pro watch

# 3. Debug visual (F5 en Cursor)

# 4. Pre-commit
make -f Makefile.pro pre-commit

# 5. Commit
git add .
git commit -m "feat: add my feature"

# 6. Push (CI auto-corre)
git push origin feature/my-feature
```

### Workflow 3: Bug Fix
```bash
# 1. Debug visual
# Abre archivo → Breakpoint (click línea) → F5

# 2. Fix code

# 3. Test específico
cargo test test_name

# 4. Validate constraints
Ctrl+Shift+T

# 5. Commit
git commit -m "fix: resolve bug"
```

### Workflow 4: Pre-PR Review
```powershell
# Run full validation
.\scripts\dev-workflow.ps1 -Action pre-commit

# Si pasa ✅ → Safe to PR
# Si falla ❌ → Fix issues
```

---

## 🆘 HELP RÁPIDO

### ❓ "¿Qué comando uso para...?"
```
Abre: COMMANDS_CHEATSHEET.md
Ctrl+F → Busca lo que necesites
```

### ❓ "¿Qué pregunto a Cursor AI?"
```
Abre: PROMPTS.md
Copia/pega el prompt que necesites
```

### ❓ "¿Cómo funciona [feature]?"
```
Abre: PRO_FEATURES.md
Busca la sección relevante
```

### ❓ "Setup no funciona"
```
Abre: QUICK_PRO_START.md
Sección "🆘 Problemas?"
```

---

## 📊 PROJECT STATS

```bash
make -f Makefile.pro stats
```

Output:
```
Rust LOC: 5,234
Chapel LOC: 3,456
Go LOC: 1,234
Datasets: 120,534
```

---

## 🔗 LINKS ÚTILES

### GitHub
- **Repo**: https://github.com/Rigohl/nuclear-crawler-hybrid
- **Actions (CI)**: https://github.com/Rigohl/nuclear-crawler-hybrid/actions
- **Issues**: https://github.com/Rigohl/nuclear-crawler-hybrid/issues

### HuggingFace
- **Chapel AI**: https://huggingface.co/Kimberlyindiva/nuclear-chapel-training
- **Datasets**: https://huggingface.co/Rigohl/mojo-mega-dataset-system

### Documentación Externa
- **Chapel Docs**: https://chapel-lang.org/docs/
- **Rust Docs**: https://doc.rust-lang.org/
- **Go Docs**: https://go.dev/doc/
- **MCP Protocol**: https://modelcontextprotocol.io/

---

## 🎯 SIGUIENTE PASO

### Primera vez aquí?
```
1. Lee: QUICK_PRO_START.md (5 min)
2. Setup extensiones y pre-commit hooks
3. Test: Ctrl+Shift+B (debería buildar)
4. Abre: PRO_FEATURES.md (features avanzadas)
```

### Ya estás setup?
```
1. Presiona: Ctrl+Shift+P → "Tasks: Run Task"
2. Ve el menú con iconos
3. Selecciona task
4. ¡A trabajar! 🚀
```

---

## 💡 PRO TIPS

1. **Dashboard siempre abierto**: Pin `docs/dashboard.html` en tu navegador
2. **PROMPTS.md en favoritos**: Para copiar/pegar rápido a Cursor
3. **Alias el Makefile**: `alias mkp='make -f Makefile.pro'`
4. **Watch mode**: Deja corriendo `make -f Makefile.pro watch`
5. **Error Lens**: Extensión muestra errores inline (instálala)

---

**Status**: ✅ Professional development environment ready!  
**Version**: 1.0.0  
**Last Updated**: 2026-01-24

**Need help?** Abre `docs/dashboard.html` o ejecuta `.\scripts\help.ps1`
