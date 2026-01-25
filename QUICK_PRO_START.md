# ⚡ QUICK PRO START (5 minutos)

Setup profesional rápido para empezar a trabajar YA.

---

## 1️⃣ Instalar Extensiones (1 min)

```
Cmd/Ctrl + Shift + P
→ "Extensions: Show Recommended Extensions"
→ Click "Install All Workspace Recommendations"
```

**Resultado**: Rust Analyzer, LLDB debugger, GitLens, Error Lens instalados.

---

## 2️⃣ Setup Pre-Commit Hooks (1 min)

### Windows (PowerShell como Admin):
```powershell
cd d:\nuclear-crawler-hybrid

# Instalar husky
npm install husky --save-dev
npx husky install

# Hacer ejecutable
icacls .husky\pre-commit /grant Everyone:RX
```

### Linux/Mac:
```bash
cd ~/nuclear-crawler-hybrid

# Instalar husky
npm install husky --save-dev
npx husky install

# Hacer ejecutable
chmod +x .husky/pre-commit
```

**Resultado**: Validación automática antes de cada commit.

---

## 3️⃣ Test Professional Tools (2 min)

### A) Test Keyboard Shortcuts

```
Presiona: Ctrl+Shift+B
→ Verás el build corriendo automáticamente
```

### B) Test Visual Debugging

```
1. Abre: src/mcp/protocol.rs
2. Click en número de línea 10 (breakpoint rojo aparece)
3. Presiona: F5
4. El código se detiene en línea 10
5. Inspecciona variables en panel izquierdo
```

### C) Test Task Runner

```
Cmd/Ctrl + Shift + P
→ "Tasks: Run Task"
→ Selecciona "✅ Validate 5 MCP Tools (CRITICAL)"
→ Ve el output
```

### D) Test Automation Script

```powershell
# Windows
.\scripts\dev-workflow.ps1 -Action validate

# Linux/Mac
pwsh scripts/dev-workflow.ps1 -Action validate
```

**Resultado**: Verás output colorizado con ✅/❌

---

## 4️⃣ Test Makefile Pro (1 min)

```bash
# Ver ayuda
make -f Makefile.pro help

# Build rápido
make -f Makefile.pro build-rust

# Test crítico
make -f Makefile.pro validate
```

**Resultado**: Colored output con emojis 🔨 🧪 ✅

---

## ✅ Verification Checklist

- [ ] Extensiones instaladas (rust-analyzer, lldb visible)
- [ ] Shortcuts funcionan (`Ctrl+Shift+B` builds)
- [ ] Debug funciona (F5 → breakpoint stops)
- [ ] Pre-commit hook setup (`.husky/pre-commit` existe)
- [ ] Makefile.pro funciona (`make -f Makefile.pro help` muestra comandos)
- [ ] Script automation funciona (`dev-workflow.ps1` runs)

---

## 🚀 Ready to Go!

### Development Flow:

```
1. Presiona F5 → Debuggea con breakpoints
2. Ctrl+Shift+B → Build rápido
3. Ctrl+Shift+T → Valida 5 tools
4. git commit → Pre-commit auto-valida
5. git push → CI corre automáticamente
```

### Archivos Clave:

- **`.vscode/launch.json`**: Debug configs (F5)
- **`.vscode/tasks.json`**: Build tasks (Ctrl+Shift+B)
- **`.vscode/keybindings.json`**: Shortcuts personalizados
- **`scripts/dev-workflow.ps1`**: Automation script PRO
- **`Makefile.pro`**: Build system multi-lenguaje
- **`.github/workflows/ci.yml`**: CI/CD automático

### Documentación:

- **`PRO_FEATURES.md`**: Guía completa de features PRO
- **`PROMPTS.md`**: Templates de prompts para Cursor
- **`COMMANDS_CHEATSHEET.md`**: Comandos de una línea

---

## 🆘 Problemas?

### Debug no funciona:
```bash
cargo build  # Build en modo debug (sin --release)
# Luego F5
```

### Tasks no aparecen:
```
Cmd/Ctrl + Shift + P
→ "Developer: Reload Window"
```

### Pre-commit hook no se ejecuta:
```bash
# Windows
icacls .husky\pre-commit /grant Everyone:RX

# Linux/Mac
chmod +x .husky/pre-commit
```

---

**⏱️ Total time: 5 minutos**  
**🎯 Result: Professional development environment activated!**

**Siguiente paso**: Abre `PRO_FEATURES.md` para features avanzadas.
