    ---
    trigger: always_on
    ---

    # Instrucciones Rust

    ## Reglas Fundamentales
    - NUNCA uses mocks ni simulaciones
    - NUNCA asumas dependencias - lee Cargo.toml
    - SIEMPRE verifica con compilación real
    - SIEMPRE busca archivos conectados antes de cambiar algo
    - SIEMPRE maneja Result y Option explícitamente

    ## Antes de Cualquier Cambio
    1. Ejecuta `cargo check --all-targets 2>&1`
    2. Lee Cargo.toml para conocer versiones y features reales
    3. Usa @workspace para encontrar TODOS los usos del código afectado
    4. Verifica que el cambio no rompa otros archivos

    ## Estilo de Código
    - snake_case para funciones y variables
    - PascalCase para tipos, traits, enums
    - SCREAMING_SNAKE_CASE para constantes
    - Prefiere &str sobre String en parámetros
    - Prefiere impl Trait sobre dyn Trait cuando sea posible
    - Máximo 100 caracteres por línea

    ## Manejo de Errores
    - Usa Result<T, E> para operaciones que pueden fallar
    - Usa thiserror para errores custom
    - Usa anyhow para aplicaciones, thiserror para librerías
    - NUNCA uses .unwrap() en producción sin justificación
    - NUNCA uses .expect() sin mensaje descriptivo
    - Propaga errores con ? cuando sea apropiado

    ## Ownership y Borrowing
    - Prefiere borrowing (&T, &mut T) sobre clonación
    - Usa Clone solo cuando sea necesario
    - Evita lifetime annotations innecesarias
    - Si el compilador pide lifetimes, entiende por qué antes de agregarlos

    ## Concurrencia
    - Prefiere canales (mpsc) sobre mutex compartidos
    - Usa Arc<Mutex<T>> solo cuando sea necesario
    - Verifica versión de tokio y sus features en Cargo.toml
    - No mezcles runtimes async
    ## Testing
    - Tests con datos reales, no mocks
    - Nombres descriptivos: test_should_X_when_Y
    - Un assert por test cuando sea posible
    - Tests de integración para flujos completos

    ## Dependencias
    - Lee Cargo.toml antes de proponer código
    - Usa la API de la versión que el proyecto tiene
    - No agregues dependencias sin justificación
    - Verifica compatibilidad de features

    ## Cuando Hay Errores
    1. Lee el error completo del compilador
    2. Identifica el archivo y línea exacta
    3. Busca qué otros archivos usan ese código
    4. Propón fix mínimo que no rompa nada
    5. Verifica con cargo check

    ## Prohibido
    - Mocks de cualquier tipo
    - Datos simulados o hardcodeados para tests
    - Ignorar warnings del compilador se deben reparar y no debe existir deathcoode
    - Código unsafe sin documentar
    - .unwrap() sin justificación
    - Asumir versiones de crates

---

# Reglas Específicas de NUCLEAR CRAWLER HYBRID

    ## Arquitectura y Módulos
    - **Usa TODOS los 11 módulos integrados**: Core (web_search, file_search, nuclear_core, url_helpers), FFI (go_integration, zig_integration, nim_integration, jax_integration), Infra (intelligent_storage, cache, rate_limit).
    - **MCP Protocolo HTTP Axum 2025**: HTTP only (Axum on :8079). Herramientas: websearch (max 5 queries, 2100+ URLs, premium content scraping de Medium/ArXiv), file_search (Zig SIMD, líneas exactas de errores), analyzer (análisis completo workspace incluyendo .md, con web search para mejoras), stats (métricas internas). Usa Tokio async, timeouts, rate limits. Result: `json!(
        {"status": "success", ...}
    )`. Nunca unwrap en handlers.

## Estándares para Reparar y Uso Completo
- **Manejo de errores robusto**: Usa `Result<T, E>` y `?`; nunca `.unwrap()` sin justificación. Propaga con contexto. Valida inputs/outputs. Loggea con `eprintln!` en STDIO mode.
- **Workflow para reparar**: Diagnostica con logs/stack traces. Fix mínimo verificado. Rollback plan. Documenta reparaciones. Monitorea post-fix.
- **Uso de módulos completo**: Inicializa TODOS en `SearchEngine::new()`. Referencia en tool handlers. Verifica con `cargo check --all-targets`.
- **FFI y concurrencia**: Go/Zig/Nim via `libloading`. Tokio async con `spawn()`, timeouts, rate limiter. No blocking en hot paths.
- **Prohibido**: Mocks/simulaciones. Dead code/stubs. Unsafe sin doc. `.unwrap()`/`.expect()` sin motivo. Ignorar warnings.

## Patrones Críticos
- **Error handling**: `Result<Value>` en tools; pattern matching, no unwrap. Timeout: `tokio::time::timeout(Duration::from_secs(N), ...)`.
- **Rate limiting**: `self.rate_limiter.acquire().await` antes de bulk ops. Cache con `DashMap`.
- **Build/Release**: `cargo build --release` (opt-level 3, LTO, codegen-units 1). Binary ~20-25MB. **ZERO WARNINGS**.
- **Testing**: Real data, no mocks. Unit/integration tests. `cargo test`.
- **Reparar bugs**: Verifica `Cargo.toml` versiones. Search ALL usages. Fix mínimo, test. No dead code.

## Reglas Esenciales para Reparar y Organizar
- **Diagnostica antes de reparar**: Logs, métricas, causa raíz.
- **Repara mínimo**: Fix pequeño, verifica con pruebas.
- **Usa todos módulos**: Nunca omitas FFI o infra en análisis.
- **Manejo errores**: Result/Option, contexto, logging, recuperación.
- **Organiza código**: Modular, nombres descriptivos, documentación.
- **Evita ediciones innecesarias**: Solo si bug probado o mejora medida.
- **Análisis completo**: Filesystem APIs, clasificación por extensión, recursivo. Incluye .md files.
- **MCP Axum**: HTTP/STDIO, 4 tools, async, timeouts, rate limits.
- **Verifica compatibilidad**: `cargo check`, no warnings.
- **Compila sin warnings**: Siempre `cargo build --release` sin errores ni warnings.
- **Documenta cambios**: Por qué, cómo, impacto.

## Lo Faltante Añadido
- **Tokio patterns**: `spawn()` para concurrente, `timeout()` para límites, `rate_limiter` para control.
- **Result building**: `json!({"status": "success", "data": ..., "execution_ms": ...})`.
- **FFI integration**: Go (100K goroutines), Zig (SIMD hashing), Nim (HTML parsing), Jax (batch processing).
- **Stealth features**: Headers from `self.stealth_system`, cache checks.
    - **Deployment**: HTTP server `--mode http --port 8079`.
- **Performance**: Optimizado para 2s search, 2100+ URLs, 100K goroutines.
- **Security**: No hardcoded secrets, validate URLs, sanitize inputs.
- **Versionado**: Semantic, features flags, `cargo mod tidy`.
- **Debugging**: `eprintln!` para logs, flame graphs para perf, heap dumps para leaks.

## REGLAS QUE SIEMPRE SE DEBEN SEGUIR
- Verifica información antes de presentarla
- Realiza cambios archivo por archivo
- No uses disculpas innecesarias
- Evita sugerencias de espacios en blanco
- No resumes cambios realizados
- No inventes cambios no solicitados
- Preserva código existente no relacionado
- Proporciona ediciones en un solo bloque
- Usa nombres de variables descriptivos y explícitos
- Sigue el estilo de codificación consistente del proyecto
- Prioriza el rendimiento en sugerencias de cambios
- Adopta enfoque de seguridad primero
- Incluye cobertura de pruebas para código nuevo/modificado
- Implementa manejo robusto de errores y logging
- Fomenta diseño modular para mantenibilidad
- Asegura compatibilidad de versiones
- Evita números mágicos, usa constantes nombradas
- Considera casos extremos en la lógica
- Incluye aserciones para validar suposiciones
