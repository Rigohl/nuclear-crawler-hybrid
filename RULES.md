# ⛔ REGLAS OBLIGATORIAS PARA GITHUB COPILOT

## RULE 1: 🚫 PROHIBIDA CREACIÓN AUTOMÁTICA DE DOCUMENTACIÓN

**ESTADO**: BLOQUEADO

### ¿Qué está PROHIBIDO?
- ❌ Crear archivos `.md` sin solicitud explícita
- ❌ Generar análisis o reportes automáticamente  
- ❌ Documentar cambios en archivos separados
- ❌ Crear "README", guías o resúmenes sin autorización
- ❌ Generar archivos de documentación que no se pidan

### ¿CUÁNDO está PERMITIDO crear documentos?
✅ **SOLO Y SOLO SI el usuario dice explícitamente:**
- "crea un documento"
- "crea `FILENAME.md`"
- "documenta esto"
- "haz un análisis"
- "escribe una guía"
- "genera un reporte"

### Comportamiento Correcto de Copilot
- ✅ Modificar código Rust directamente
- ✅ Hacer cambios en archivos existentes
- ✅ Ejecutar tests y comandos
- ✅ Buscar y analizar código
- ✅ Responder preguntas EN ESTA CONVERSACIÓN (no en archivos)
- ✅ Documentar EN CÓDIGO (comentarios, docstrings)

---

## RULE 2: 📝 SOLICITUD EXPLÍCITA REQUERIDA

**Copilot DEBE verificar si el usuario pidió documentación ANTES de crear archivos.**

Palabras clave que autorizan documentación:
- `"crea"`, `"create"`
- `"documenta"`, `"document"`
- `"genera"`, `"generate"`
- `"haz"`, `"make"`
- `"escribe"`, `"write"`
- `"añade un archivo"`, `"add a file"`

Palabras clave que NO autorizan:
- `"analiza"` → responde en chat, NO crea archivo
- `"explica"` → responde en chat, NO crea archivo
- `"resume"` → responde en chat, NO crea archivo

---

## RULE 3: 🔴 INCUMPLIMIENTO

Si Copilot crea un documento sin autorización explícita:
1. **VIOLA ESTA REGLA**
2. El usuario puede rechazar la acción
3. Copilot debe confirmar antes de crear archivos

---

## RULE 4: 📂 EXCEPCIONES

Archivos que SÍ se pueden crear/modificar sin solicitud:
- Cambios en código existente (`.rs`, `.py`, `.chpl`)
- Actualizar archivos en workflows (`.github/workflows/`)
- Crear archivos en carpetas configuradas (si es estándar del proyecto)
- Archivos requeridos por la estructura del proyecto

---

## RESUMEN EJECUTIVO

| Acción | Permitido | Condición |
|--------|-----------|-----------|
| Crear `.md` | ❌ NO | ✅ SOLO si usuario lo pide explícitamente |
| Modificar código | ✅ SÍ | Siempre (es el trabajo principal) |
| Generar reportes | ❌ NO | ✅ SOLO si usuario dice "genera reporte" |
| Responder preguntas | ✅ SÍ | En la conversación, nunca en archivos |
| Documentar en código | ✅ SÍ | Comentarios y docstrings |

---

**Última actualización**: 2026-01-24
**Aplicable a**: GitHub Copilot en VS Code
**Estado**: 🔴 ACTIVO Y OBLIGATORIO
