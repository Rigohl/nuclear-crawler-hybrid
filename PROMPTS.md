# 🎯 Cursor AI - Prompts Inteligentes

Copia y pega estos prompts en el chat de Cursor para obtener respuestas precisas.

---

## 🚀 Prompts de Inicio Rápido

### 📖 Entender el Proyecto
```
@.cursorrules @README.md @INTEGRATION_STATUS.md 

Dame un resumen completo de este proyecto:
- Arquitectura de 4 componentes
- Los 8 sistemas de Chapel
- Las 3 restricciones críticas
- Cómo están integrados los datasets
```

### 🔍 Análisis de Componente
```
@.cursorrules 

Analiza el componente [Rust MCP / Chapel AI / Go GitHub / Datasets]:
- Archivos clave
- Comandos principales
- Restricciones específicas
- Cómo se integra con otros componentes
```

---

## 🔨 Prompts de Desarrollo

### ➕ Agregar Nueva Funcionalidad
```
@.cursorrules @src/mcp/protocol.rs 

Quiero agregar [funcionalidad] a [componente].
1. ¿Esto afecta las 5 MCP tools?
2. ¿Necesito modificar FFI?
3. ¿Qué archivos debo editar?
4. ¿Qué tests debo ejecutar?
5. Dame el código sin mocks
```

### 🐛 Debug de Error
```
@.cursorrules 

Tengo este error:
[pega el error completo]

En el archivo: [ruta]
Componente: [Rust/Chapel/Go]

1. ¿Cuál es la causa?
2. ¿Cómo lo soluciono?
3. ¿Qué comando ejecuto para verificar?
4. ¿Hay problemas relacionados? (bincode, FFI, etc)
```

### 🧪 Testing
```
@.cursorrules 

Necesito testear [componente/funcionalidad]:
1. ¿Qué tipo de test? (unit/integration/5-tools)
2. Dame el comando exacto
3. ¿Qué verificaciones adicionales necesito?
4. ¿Hay tests críticos que siempre debo correr?
```

---

## 🏗️ Prompts por Componente

### 🦀 Rust MCP Server

#### Modificar Tool Existente
```
@src/mcp/protocol.rs @src/mcp/tools/[tool].rs @.cursorrules

Quiero mejorar el tool [websearch/premium/file_search/scan/ai_dataset_trainer]:
- Nueva funcionalidad: [describe]
- Sin agregar tools (mantener 5)
- Sin mocks (implementación real)

Dame:
1. Código actualizado
2. Tests necesarios
3. Comando de validación: cargo test test_exactly_5_tools
```

#### Crear Nueva Función en Tool
```
@src/mcp/tools/[tool].rs @.cursorrules

Agregar función auxiliar en [tool]:
Propósito: [describe]

Requisitos:
- NO MOCKS (implementación real)
- Maneja errores correctamente
- Tests incluidos
```

### 🧠 Chapel AI Training

#### Nueva Pipeline de Training
```
@ffi/chapel/training/ @.cursorrules

Crear pipeline de training para:
Dataset: [ubicación en models/]
Objetivo: [describe]

Requisitos:
- Usa coforall para paralelismo
- Accede datasets: ../../../models/
- Integra con nuclear_chapel_ai.chpl
- Target en Makefile
```

#### Optimizar Performance
```
@ffi/chapel/[archivo].chpl @.cursorrules

Optimizar performance de [sistema]:
- Usar multi-locale si aplica
- BLAS3 optimization
- Distributed computing

Dame código optimizado y targets de Makefile
```

### 🐹 Go GitHub MCP

#### Nueva Funcionalidad GitHub
```
@mcp-servers/github/pkg/github/ @.cursorrules

Agregar funcionalidad GitHub:
[describe funcionalidad]

Requisitos:
- MCP protocol (stdio)
- Usa GITHUB_TOKEN
- Error handling robusto
- Tests con go test
```

### 📊 Datasets

#### Procesar Nuevo Dataset
```
@models/ @.cursorrules

Integrar nuevo dataset:
Formato: [jsonl/json/pkl]
Tamaño: [samples]
Ubicación: models/[carpeta]

Cómo:
1. Procesar con [Mojo/Julia/Python]
2. Acceder desde Chapel
3. Integrar en training pipeline
```

---

## ⚙️ Prompts de Configuración

### 🔧 Setup Inicial
```
@QUICK_START.md @.cursorrules

Necesito configurar el ambiente para [Windows/Linux/Mac]:
1. Dependencias necesarias
2. Variables de entorno
3. Build steps
4. Verificación que funciona
```

### 📦 Agregar Dependencia
```
@Cargo.toml @ffi/chapel/Makefile @mcp-servers/github/go.mod @.cursorrules

Agregar dependencia:
Lenguaje: [Rust/Chapel/Go]
Dependencia: [nombre]

1. ¿Cómo la agrego?
2. ¿Afecta el build?
3. ¿Tests adicionales?
```

---

## 🎓 Prompts de Aprendizaje

### 📚 Explicar Concepto
```
@.cursorrules @.cursor/context.md

Explícame [Chapel parallelism / MCP protocol / FFI integration / coforall]:
- Nivel 1: Básico
- Nivel 2: Intermedio
- Nivel 3: Avanzado
- Ejemplo práctico en este proyecto
```

### 🗺️ Navegar Codebase
```
@.cursorrules @INTEGRATION_STATUS.md

¿Dónde está la implementación de [funcionalidad]?
Dame:
1. Archivo(s) principal(es)
2. Funciones clave
3. Tests relacionados
4. Dependencias
```

---

## ⚡ Prompts de Productividad

### 🚀 Build Rápido
```
@.cursor/commands.json

Dame el comando más rápido para:
- Build solo [Rust/Chapel/Go]
- Build todo en paralelo
- Build sin tests
- Build optimizado (release)
```

### ✅ Checklist Pre-Commit
```
@.cursorrules

Pre-commit checklist:
1. ¿Qué commands de build/test ejecuto?
2. ¿Verificaciones críticas? (5 tools, no mocks)
3. ¿Format code?
4. ¿Warnings a resolver?

Dame commands exactos en orden
```

### 📝 Documentar Cambios
```
@.cursorrules

Hice estos cambios:
[lista cambios]

Ayúdame a:
1. Escribir commit message descriptivo
2. Actualizar docs si necesario
3. Verificar que no rompí constraints
```

---

## 🔐 Prompts de Seguridad

### 🛡️ Security Review
```
@.cursorrules

Review de seguridad para [archivo/componente]:
1. ¿Input validation correcta?
2. ¿Manejo de secrets seguro?
3. ¿Vulnerabilidades conocidas?
4. Sugerencias de hardening
```

---

## 🎯 Prompts Avanzados

### 🔀 Refactoring
```
@[archivo] @.cursorrules

Refactorizar [función/módulo]:
Objetivo: [mejorar performance/legibilidad/mantainability]

Requisitos:
- Mantener comportamiento
- Sin romper constraints (5 tools, NO MOCKS)
- Tests pasando
- Código más limpio
```

### 📊 Performance Analysis
```
@[archivo] @.cursorrules

Analizar performance de [componente]:
1. Bottlenecks actuales
2. Optimizaciones posibles
3. Trade-offs
4. Benchmarks sugeridos
```

### 🔄 Integración de Componentes
```
@.cursorrules @INTEGRATION_STATUS.md

Integrar [componente A] con [componente B]:
- FFI si necesario
- Data flow
- Error handling
- Tests de integración
```

---

## 💡 Tips para Mejores Respuestas

### ✅ DO:
- Usa `@archivo` para dar contexto específico
- Incluye `@.cursorrules` para recordar constraints
- Sé específico sobre el componente (Rust/Chapel/Go)
- Pide comandos exactos, no teoría
- Menciona si es para dev/test/prod

### ❌ DON'T:
- Preguntas vagas: "¿Cómo build?" → Especifica componente
- Sin contexto: Menciona archivo o componente
- Ignorar constraints: Cursor debe recordarte las 3 reglas
- Pedir mocks: Siempre pide implementación real

---

## 🎬 Ejemplo de Conversación Perfecta

```
User: @src/mcp/protocol.rs @.cursorrules

Quiero agregar caching al tool "websearch".
1. ¿Esto requiere agregar un 6º tool? (espero que NO)
2. ¿Cómo lo implemento sin mocks?
3. Dame el código con Redis real
4. ¿Qué tests ejecuto?

Cursor: [respuesta inteligente]
- NO requiere 6º tool, se agrega dentro de websearch ✅
- Implementación con redis crate (sin mocks) ✅
- Código actualizado en src/mcp/tools/websearch.rs
- Tests: cargo test test_exactly_5_tools && cargo test websearch
```

---

**Pro Tip**: Guarda este archivo en favoritos y úsalo como referencia rápida. Los prompts con `@archivo` son más efectivos porque dan contexto directo a Cursor.

**Shortcut**: `Cmd/Ctrl + P` → busca "PROMPTS.md" → acceso rápido a templates
