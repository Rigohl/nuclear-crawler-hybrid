---
description: DEBUG MODE
---

// turbo-all

[8:31 a. m., 8/12/2025] Rigohl: ### 1. ANÁLISIS DE DEPENDENCIAS
Usa @workspace para:
- Leer Cargo.toml y TODAS las dependencias
- Verificar versiones compatibles entre crates
- Detectar conflictos de versiones (especialmente tokio, serde, wasm-bindgen)
- Verificar features flags correctos
- Identificar dependencias duplicadas con diferentes versiones

### 2. VERIFICACIÓN DE COMPI…
[8:32 a. m., 8/12/2025] Rigohl: Ahora tengo toda la información necesaria. Te daré los prompts completos:

---

## 📋 PROMPT PRINCIPAL - Centro de Investigación Julia + FFI Multi-Lenguaje

markdown
Crea un centro de investigación científica continuo basado en Julia con interoperabilidad FFI hacia los lenguajes más poderosos. El sistema debe:

## ARQUITECTURA PRINCIPAL

### 1. Estructura del Proyecto

research_center/
├── orchestrators/           # Orquestadores por dominio
│   ├── quantum/
│   ├── bioinformatics/
│   ├── climate/
│   ├── physics/
│   ├── chemistry/
│   ├── materials/
│   ├── finance/
│   └── ai_ml/
├── modules/                 # Módulos Julia core
│   ├── analysis/
│   ├── simulation/
│   ├── optimization/
│   ├── visualization/
│   └── data_processing/
├── ffi_bridges/   …
[9:44 a. m., 8/12/2025] Rigohl: ## Prompt para Debug MCP Rust + WASM (Sin Mocks, Todo Real)

markdown
Necesito que hagas debugging exhaustivo y reparación completa de mi MCP server en Rust + WASM. 

## REGLAS CRÍTICAS
- NO uses mocks bajo ninguna circunstancia
- NO simules respuestas ni datos
- TODO debe ser ejecución real contra servicios reales
- Prueba con datos reales del sistema
- Si algo falla, repáralo inmediatamente

## PROCESO DE DEBUG

### 1. ANÁLISIS DE DEPENDENCIAS
Usa @workspace para:
- Leer Cargo.toml y TODAS las dependencias
- Verificar versiones compatibles entre crates
- Detectar conflictos de versiones (especialmente tokio, serde, wasm-bindgen)
- Verificar features flags correctos
- Identificar dependencias duplicadas con diferentes versiones

### 2. VERIFICACIÓN DE COMPILACIÓN
Ejecuta y analiza errores de:
bash
cargo check --all-targets --all-features 2>&1
cargo build --release 2>&1
cargo build --target wasm32-unknown-unknown 2>&1
wasm-pack build --target web 2>&1


Para CADA error:
- Identifica la causa raíz exacta
- Muestra el archivo y línea
- Proporciona el fix específico
- Verifica que el fix no rompa otras partes

### 3. ERRORES COMUNES A BUSCAR Y REPARAR

#### Rust Core:
- Lifetimes incorrectos o faltantes
- Borrow checker violations
- Move después de borrow
- Referencias dangling
- Tipos no coincidentes
- Traits no implementados
- async/await sin .await
- Mutex deadlocks potenciales
- Race conditions en código concurrente

#### WASM Específico:
- Tipos no compatibles con wasm-bindgen
- Referencias a tipos que no implementan JsCast
- Uso de std features no disponibles en WASM
- Imports de crates que no compilan a WASM
- Memory leaks por no liberar recursos JS
- Panics sin catch en boundary WASM/JS

#### MCP Protocol:
- JSON-RPC mal formado
- Handlers que no retornan el tipo correcto
- Tools sin schema válido
- Resources sin URI correcta
- Errores no propagados correctamente

#### Dependencias:
- tokio features incompatibles con WASM (usar wasm-bindgen-futures)
- reqwest necesita feature "wasm" habilitado
- serde_json vs serde-wasm-bindgen
- chrono no compatible, usar js-sys::Date

### 4. ANÁLISIS DE RUNTIME
Busca en el código:
- unwrap() y expect() sin manejo de error
- panic!() que deberían ser Result
- Loops infinitos potenciales
- Recursión sin caso base
- Buffers sin límite de tamaño
- Conexiones sin timeout
- File handles sin close

### 5. VERIFICACIÓN DE CADA MÓDULO

Para CADA archivo .rs en el proyecto:

Archivo: [nombre]
├── Imports correctos: ✓/✗
├── Tipos correctos: ✓/✗
├── Lifetimes correctos: ✓/✗
├── Error handling: ✓/✗
├── WASM compatible: ✓/✗
├── Tests pasan: ✓/✗
└── Errores encontrados: [lista]


### 6. TESTS REALES (NO MOCKS)
bash
# Ejecutar tests reales
cargo test --all-features -- --nocapture 2>&1

# Tests de integración
cargo test --test '*' -- --nocapture 2>&1

# Tests WASM en browser real
wasm-pack test --headless --chrome 2>&1


### 7. VERIFICACIÓN MCP REAL
bash
# Probar server MCP real
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | cargo run 2>&1

# Listar tools reales
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | cargo run 2>&1

# Llamar tool real
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"[tool]","arguments":{}}}' | cargo run 2>&1


### 8. FORMATO DE REPARACIÓN

Para cada error encontrado, proporciona:


## ERROR #[N]

*Ubicación:* src/[path]/[file].rs:[línea]

*Tipo:* [compilación/runtime/lógica/wasm]

*Error exacto:*

[mensaje de error completo]


*Causa raíz:*
[explicación técnica]

*Archivos afectados:*
- archivo1.rs (importa este módulo)
- archivo2.rs (usa esta función)

*Fix:*
rust
// ANTES (líneas X-Y)
[código con error]

// DESPUÉS
[código corregido]


*Verificación:*
bash
[comando para verificar que funciona]


*Impacto del cambio:*
- [lista de otros archivos que podrían necesitar ajustes]


### 9. CARGO.TOML VERIFICADO

Después de analizar, genera Cargo.toml corregido con:
- Versiones compatibles verificadas
- Features correctos para WASM
- Sin dependencias duplicadas
- Ordenado y documentado

### 10. CHECKLIST FINAL

Antes de terminar, verifica:
- [ ] `cargo check` sin errores
- [ ] `cargo build --release` exitoso
- [ ] `cargo build --target wasm32-unknown-unknown` exitoso
- [ ] `cargo test` todos pasan (tests reales)
- [ ] `wasm-pack build` exitoso
- [ ] MCP server responde a initialize
- [ ] MCP server lista tools correctamente
- [ ] Cada tool ejecuta sin error
- [ ] No hay warnings sin resolver
- [ ] No hay unsafe sin documentar
- [ ] Todos los Result manejados
- [ ] Todos los Option manejados

## OUTPUT ESPERADO

1. Lista completa de TODOS los errores encontrados
2. Fix para CADA error con código exacto
3. Cargo.toml corregido completo
4. Lista de archivos modificados
5. Comandos de verificación ejecutados con output real
6. Confirmación de que TODO compila y funciona

Comienza analizando @workspace completo ahora.


---

## Instrucciones de Debug para .github/instructions/debug-mcp-rust.instructions.md

markdown
---
applyTo: "**/*.rs"
description: "Debug exhaustivo MCP Rust + WASM sin mocks"
---

# Debug MCP Rust + WASM

## Reglas Absolutas
- NUNCA uses #[cfg(test)] con mocks
- NUNCA simules respuestas HTTP
- NUNCA uses datos hardcodeados para tests
- SIEMPRE ejecuta contra servicios reales
- SIEMPRE verifica con cargo real, no suposiciones

## Cuando Encuentres un Error

1. **No asumas** - lee el error completo
2. **Traza la raíz** - sigue el stack trace hasta el origen
3. **Verifica contexto** - revisa imports, types, lifetimes
4. **Busca dependencias** - @workspace refs al código afectado
5. **Repara atómicamente** - un fix, una verificación
6. **Confirma real** - cargo check/build/test real

## Patrones de Error Comunes

### Lifetime Errors
rust
// ❌ Error común
fn get_data(&self) -> &str {
    &self.compute_string()  // temporal no puede ser referencia
}

// ✅ Fix
fn get_data(&self) -> String {
    self.compute_string()
}


### WASM Incompatibility
rust
// ❌ No funciona en WASM
use std::time::Instant;
use std::thread;

// ✅ Compatible WASM
use web_sys::Performance;
use wasm_bindgen_futures::spawn_local;


### Async en WASM
rust
// ❌ tokio no funciona en WASM
#[tokio::main]
async fn main() {}

// ✅ Para WASM
use wasm_bindgen_futures::spawn_local;
spawn_local(async move {
    // código async
});


### MCP Response Format
rust
// ❌ Formato incorrecto
return json!({"data": result});

// ✅ Formato MCP correcto
return json!({
    "content": [{
        "type": "text",
        "text": serde_json::to_string(&result)?
    }]
});


## Verificación Obligatoria

Después de CADA fix, ejecuta:
bash
cargo check 2>&1 | head -50


Si pasa, continúa. Si falla, repara antes de seguir.

## Prioridad de Reparación

1. Errores de compilación (bloquean todo)
2. Errores de tipos/lifetimes
3. Errores de WASM compatibility
4. Errores de runtime (panics)
5. Errores de lógica
6. Warnings
7. Optimizaciones


---

## Agente Personalizado .github/agents/rust-debugger.agent.md

markdown
---
name: "RustMCPDebugger"
description: "Agente especializado en debug de MCP Rust + WASM"
tools: ["terminal", "codebase", "file_editor"]
---

# Rust MCP Debugger Agent

## Identidad
Soy un debugger experto en Rust, WebAssembly y MCP Protocol. Mi trabajo es encontrar y reparar TODOS los errores sin usar mocks ni simulaciones.

## Capacidades
- Análisis estático de código Rust
- Detección de errores de compilación
- Verificación de compatibilidad WASM
- Validación de protocolo MCP
- Reparación automática de código

## Flujo de Trabajo

### Paso 1: Recon
bash
# Estructura del proyecto
find . -name "*.rs" -o -name "Cargo.toml" | head -100

# Dependencias
cat Cargo.toml

# Errores actuales
cargo check --all-targets 2>&1


### Paso 2: Análisis
- Leer cada archivo .rs
- Mapear dependencias entre módulos
- Identificar puntos de fallo

### Paso 3: Diagnóstico
Para cada error:
- Tipo de error
- Ubicación exacta
- Causa raíz
- Impacto en otros archivos

### Paso 4: Reparación
- Fix mínimo necesario
- Verificar que compila
- Verificar que no rompe otros módulos

### Paso 5: Validación
bash
cargo build --release
cargo test
wasm-pack build


## Comandos que Uso

bash
# Compilación completa
cargo build --all-targets --all-features

# WASM build
cargo build --target wasm32-unknown-unknown
wasm-pack build --target web

# Tests reales
cargo test -- --nocapture

# Verificar MCP
echo '{"jsonrpc":"2.0","id":1,"method":"initialize"}' | cargo run

# Lints
cargo clippy -- -D warnings


## Nunca Hago
- Usar #[cfg(test)] para mockear
- Simular respuestas de red
- Ignorar warnings
- Dejar unwrap() sin manejar
- Asumir que algo funciona sin probar
