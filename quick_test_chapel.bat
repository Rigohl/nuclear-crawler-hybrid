@echo off
echo ===========================================
echo ⚡ CHAPEL HPC EDITOR - PRUEBA RÁPIDA
echo ===========================================
echo.

echo 🧪 Probando Chapel HPC Editor con imagen oficial...
echo.

REM Crear archivos de prueba
if not exist quick_test mkdir quick_test

echo // Archivo de prueba Chapel > quick_test\sample.chpl
echo writeln("Hello from Chapel HPC!"); >> quick_test\sample.chpl
echo oldFunction(); >> quick_test\sample.chpl
echo forall i in 1..10 do >> quick_test\sample.chpl
echo     compute(i); >> quick_test\sample.chpl

echo // Otro archivo > quick_test\another.chpl
echo oldFunction(); >> quick_test\another.chpl
echo coforall loc in Locales do >> quick_test\another.chpl
echo     on loc do >> quick_test\another.chpl
echo         processData(); >> quick_test\another.chpl

echo 📁 Archivos de prueba creados
echo.

echo 🐳 Ejecutando con imagen oficial de Chapel...
docker run --rm -v "%CD%:/workspace" -w /workspace chapel/chapel:latest bash -c "
    echo '🔍 Verificando Chapel...' &&
    chpl --version &&
    echo '📋 Listando archivos...' &&
    ls -la quick_test/ &&
    echo '⚡ Compilando herramienta...' &&
    chpl --fast --specialize --optimize-forall-unordered-ops chpl_parallel_editor.chpl -o chpl_parallel_editor &&
    echo '🚀 Ejecutando procesamiento paralelo...' &&
    ./chpl_parallel_editor quick_test 'oldFunction' 'newFunction' &&
    echo '📊 Resultados:' &&
    cat quick_test/sample.chpl &&
    echo '---' &&
    cat quick_test/another.chpl
"

if %errorlevel% equ 0 (
    echo.
    echo ✅ ¡Prueba exitosa con Chapel oficial!
    echo 🎯 El paralelismo HPC nativo está funcionando
) else (
    echo.
    echo ❌ Error en la prueba
    echo 💡 Verifica que Docker esté funcionando correctamente
)

echo.
echo 🧹 Limpiando archivos de prueba...
rmdir /s /q quick_test 2>nul

echo.
echo 🎯 Para usar permanentemente:
echo    1. docker_chapel.bat build    (construir imagen optimizada)
echo    2. docker_chapel.bat run      (usar la herramienta)
echo.
pause