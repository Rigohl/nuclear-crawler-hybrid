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

    ## FFI
    - extern "C" debe coincidir exactamente con el otro lenguaje
    - Usa tipos C compatibles: i32, u32, f32, *const, *mut
    - Documenta todo código unsafe
    - Encapsula unsafe en abstracciones seguras
    -es un ejemplo, el ffi fii podria ser con cualquier lenguaje optimo
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