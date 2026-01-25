# ⚡ QUICK ACCESS - Acceso Rápido

**Guía ultra-rápida para NO OLVIDAR las herramientas PRO.**

---

## 🎯 3 Formas de Acceder

### 1️⃣ Dashboard HTML (Recomendado) 🌟
```bash
# Windows
start docs\dashboard.html

# Mac
open docs/dashboard.html

# Linux
xdg-open docs/dashboard.html
```
**→ Abre en tu navegador → PIN LA PESTAÑA → Siempre accesible**

### 2️⃣ Terminal Interactivo
```powershell
.\scripts\help.ps1
```
**→ Menú interactivo con todas las opciones**

### 3️⃣ Archivo Maestro
```
Abre: START_HERE.md
```
**→ Tu punto de entrada con TODO documentado**

---

## ⚡ Acciones Más Comunes (Memoriza Estas)

| Necesito | Hago | En 3 Segundos |
|----------|------|---------------|
| **Build todo** | `Ctrl+Shift+B` | ✅ Build automático |
| **Validar 5 tools** | `Ctrl+Shift+T` | ✅ Test crítico |
| **Run server** | `Ctrl+Shift+R` | ✅ Server corriendo |
| **Debug** | Click línea → `F5` | ✅ Breakpoint activo |
| **Ver tareas** | `Ctrl+Shift+P` → "Tasks" | ✅ Menú con iconos |
| **Help rápido** | `.\scripts\help.ps1` | ✅ Menú interactivo |

---

## 📌 PIN ESTO EN TU ESCRITORIO

### Windows (Script de Acceso Directo)
```powershell
# Crear acceso directo al dashboard
$WshShell = New-Object -comObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$env:USERPROFILE\Desktop\Nuclear Dashboard.lnk")
$Shortcut.TargetPath = "d:\nuclear-crawler-hybrid\docs\dashboard.html"
$Shortcut.Save()
```

### Mac/Linux (Alias)
```bash
# Agregar a ~/.bashrc o ~/.zshrc
alias nuclear-dash='open ~/nuclear-crawler-hybrid/docs/dashboard.html'
alias nuclear-help='pwsh ~/nuclear-crawler-hybrid/scripts/help.ps1'
alias mkp='make -f ~/nuclear-crawler-hybrid/Makefile.pro'
```

---

## 🔥 Shortcuts de Cursor (LOS MÁS ÚTILES)

```
Ctrl+Shift+B  →  Build All (úsalo TODO el tiempo)
Ctrl+Shift+T  →  Validate 5 Tools (antes de commit)
F5            →  Debug (mejor que println!)
Ctrl+Shift+P  →  Command Palette (todo está aquí)
```

**¡MEMORIZA ESTOS 4 Y YA ESTÁS!**

---

## 📚 Si Necesitas Más Info

| Archivo | Cuándo Usar |
|---------|-------------|
| **[START_HERE.md](START_HERE.md)** | Primera vez, o si olvidaste todo |
| **[PRO_FEATURES.md](PRO_FEATURES.md)** | Quiero aprender todas las features |
| **[PROMPTS.md](PROMPTS.md)** | Necesito preguntar algo a Cursor AI |
| **[COMMANDS_CHEATSHEET.md](COMMANDS_CHEATSHEET.md)** | Busco un comando específico |
| **[QUICK_PRO_START.md](QUICK_PRO_START.md)** | Setup desde cero (5 min) |

---

## 💡 Pro Tips para NO OLVIDAR

1. **PIN el dashboard en navegador**:
   ```
   Abre docs/dashboard.html → Click derecho pestaña → "Pin Tab"
   ```

2. **Agrega a Cursor bookmarks**:
   ```
   Ctrl+P → START_HERE.md → Botón ⭐ (agregar a favoritos)
   ```

3. **Configura startup**:
   ```json
   // En .vscode/settings.json
   "workbench.startupEditor": "readme"
   ```

4. **Post-it virtual** (Windows):
   ```
   Win+W → Sticky Notes → Escribe:
   
   Nuclear Tools:
   - Dashboard: start docs\dashboard.html
   - Help: .\scripts\help.ps1
   - Build: Ctrl+Shift+B
   - Validate: Ctrl+Shift+T
   ```

5. **Wallpaper reminder**:
   - Toma screenshot del dashboard
   - Usa como fondo de pantalla (transparente)

---

## 🎬 Quick Start (30 segundos)

```bash
# 1. Abre dashboard (PIN EN NAVEGADOR)
start docs\dashboard.html

# 2. En Cursor, presiona:
Ctrl+Shift+B

# 3. Si funciona, estás listo!
```

---

## 🆘 Si OLVIDASTE TODO

```powershell
# Ejecuta esto:
.\scripts\help.ps1

# O abre esto:
code START_HERE.md

# O abre dashboard:
start docs\dashboard.html
```

**Uno de estos 3 te recordará EVERYTHING.**

---

## ✅ Checklist: "¿Qué tengo disponible?"

- [x] Visual debugging (F5)
- [x] Custom shortcuts (Ctrl+Shift+X)
- [x] Task runner (menu con iconos)
- [x] Code snippets (`mcp-tool` + Tab)
- [x] PowerShell automation
- [x] Professional Makefile
- [x] GitHub Actions CI/CD
- [x] Pre-commit hooks
- [x] AI Prompts (50+)
- [x] Dashboard HTML
- [x] Interactive help

**¡TIENES TODO ESTO! No lo olvides 🚀**

---

**Acceso rápido desde terminal**:
```bash
code QUICK_ACCESS.md  # Este archivo
```

**Acceso rápido desde navegador**:
```
📌 Pin: docs/dashboard.html
```

**Acceso rápido desde Cursor**:
```
Ctrl+P → "quick" → Enter
```

---

**Bottom line**: Si solo recuerdas UNA cosa, que sea esta:

```
start docs\dashboard.html
```

**TODO lo demás está ahí. Pin esa pestaña y nunca más olvides. 📌**
