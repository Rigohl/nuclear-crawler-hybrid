@echo off
echo ===========================================
echo 🔬 RUST PARALLEL CODE EDITOR - HPC POWER
echo ===========================================
echo.

if "%1"=="help" goto :help
if "%1"=="" goto :help

echo 🔨 Compilando herramienta Rust con paralelismo ultra-HPC...
echo.

cd /d "%~dp0"

REM Compilar con optimizaciones de release para máximo rendimiento
echo 📦 Compilando con optimizaciones LTO y paralelismo...
cargo build --release --bin rust-parallel-editor

if %errorlevel% neq 0 (
    echo ❌ Error durante la compilación
    echo.
    echo 💡 Posibles soluciones:
    echo    • Verifica que tienes Rust instalado: rustc --version
    echo    • Actualiza las dependencias: cargo update
    echo    • Limpia el cache: cargo clean
    echo.
    pause
    exit /b 1
)

echo.
echo ✅ ¡Compilación exitosa!
echo.
echo 🎯 Ejecutando herramienta...
echo.

REM Ejecutar con los parámetros proporcionados
target\release\rust-parallel-editor.exe %*

goto :end

:help
echo 💡 RUST PARALLEL CODE EDITOR - HPC POWER
echo ========================================
echo.
echo 📖 USO:
echo    build_rust_editor.bat [directorio] [patrón_búsqueda] [patrón_reemplazo] [opciones]
echo.
echo 📚 EJEMPLOS:
echo.
echo    # Reemplazo básico
echo    build_rust_editor.bat . "old_text" "new_text"
echo.
echo    # Solo archivos Rust
echo    build_rust_editor.bat /src "fn old_func" "fn new_func" --ext rs
echo.
echo    # Dry-run (solo mostrar cambios)
echo    build_rust_editor.bat . "TODO" "FIXME" --dry-run
echo.
echo    # Regex avanzado
echo    build_rust_editor.bat . "fn\s+\w+" "fn optimized_function" --ext rs
echo.
echo 🎯 CARACTERÍSTICAS ULTRA-HPC:
echo    • Procesamiento paralelo con Rayon
echo    • Load balancing automático
echo    • Regex avanzado
echo    • Barra de progreso en tiempo real
echo    • Estadísticas detalladas de rendimiento
echo    • Modo dry-run seguro
echo.
echo ⚡ POTENCIA: Paralelismo Rust + Rayon para rendimiento máximo
echo.

:end
pause