@echo off
echo ===========================================
echo ⚡ CHAPEL HPC - PRUEBA DIRECTA
echo ===========================================
echo.

echo 🐳 Usando imagen oficial: chapel/chapel:latest
echo 📊 ID de imagen: 6f118f6655ab
echo.

if "%1"=="" (
    echo 💡 USO:
    echo    test_direct.bat [comando]
    echo.
    echo 📚 EJEMPLOS:
    echo    test_direct.bat "chpl --version"
    echo    test_direct.bat "chpl --help"
    echo    test_direct.bat "echo 'Hello Chapel' | chpl"
    echo.
    pause
    exit /b 1
)

echo 🚀 Ejecutando: %1
echo.

docker run --rm chapel/chapel:latest bash -c "%1"

echo.
echo ✅ Comando ejecutado
pause