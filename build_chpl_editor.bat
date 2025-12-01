@echo off
echo ===========================================
echo 🚀 CHAPEL PARALLEL CODE EDITOR - ULTRA HPC
echo ===========================================
echo.

if "%1"=="help" goto :help
if "%1"=="" goto :help

echo 🔨 Compilando herramienta Chapel...
echo.

REM Verificar si chpl está instalado
chpl --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Error: Chapel compiler (chpl) no está instalado
    echo.
    echo 💡 Instalación:
    echo    • Descarga: https://chapel-lang.org/download.html
    echo    • Windows: Instala desde MSI
    echo    • Linux/Mac: make install
    echo.
    pause
    exit /b 1
)

REM Compilar con optimizaciones HPC
echo 📦 Compilando con optimizaciones ultra-HPC...
chpl --fast --specialize --optimize-forall-unordered-ops chpl_parallel_editor.chpl -o chpl_parallel_editor.exe

if %errorlevel% neq 0 (
    echo ❌ Error durante la compilación
    pause
    exit /b 1
)

echo.
echo ✅ ¡Compilación exitosa!
echo.
echo 🎯 Ejecutando herramienta...
echo.

REM Ejecutar con los parámetros proporcionados
chpl_parallel_editor.exe %*

goto :end

:help
echo 💡 HERRAMIENTA CHAPEL PARALLEL CODE EDITOR
echo ==========================================
echo.
echo 📖 USO:
echo    build_chpl_editor.bat [directorio] [patrón_búsqueda] [patrón_reemplazo]
echo.
echo 📚 EJEMPLOS:
echo.
echo    # Reemplazar funciones en código Rust
echo    build_chpl_editor.bat . "fn old_function" "fn new_function"
echo.
echo    # Cambiar imports en Python
echo    build_chpl_editor.bat /src "import old_module" "import new_module"
echo.
echo    # Optimización masiva en Chapel
echo    build_chpl_editor.bat . "writeln" "writefln"
echo.
echo    # Reemplazo en archivos específicos
echo    build_chpl_editor.bat . "*.rs" "println!" "writeln!"
echo.
echo 🎯 CARACTERÍSTICAS ULTRA-HPC:
echo    • Procesamiento paralelo automático
echo    • Load balancing inteligente
echo    • Memoria distribuida nativa
echo    • Optimizaciones Chapel especializadas
echo    • Reportes de rendimiento detallados
echo.
echo ⚡ POTENCIA: Aprovecha el 200%% del paralelismo de Chapel
echo.

:end
pause