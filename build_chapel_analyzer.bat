@echo off
echo ========================================
echo 🔬 NUCLEAR CRAWLER HYBRID
echo 🎯 ANALIZADOR ULTRA-AVANZADO DE CHAPEL
echo ========================================
echo.

echo 🔨 Compilando ejecutable independiente...
echo.

cd /d "%~dp0"

if not exist "target\release\chapel-analyzer.exe" (
    echo 📦 Construyendo chapel-analyzer.exe...
    cargo build --release --bin chapel-analyzer
    if %errorlevel% neq 0 (
        echo ❌ Error durante la compilación
        pause
        exit /b 1
    )
) else (
    echo ✅ Ejecutable ya existe, omitiendo compilación
)

echo.
echo 🎯 Ejecutable listo: target\release\chapel-analyzer.exe
echo.
echo 💡 USO:
echo    chapel-analyzer.exe [ruta_del_proyecto]
echo.
echo 📚 EJEMPLOS:
echo    chapel-analyzer.exe .
echo    chapel-analyzer.exe C:\mi\proyecto\chapel
echo    chapel-analyzer.exe /home/user/chapel-project
echo.
echo 🔍 El analizador detectará automáticamente archivos .chpl
echo.

pause