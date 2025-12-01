@echo off
REM Script para iniciar Nuclear Crawler Hybrid - Servidor HTTP
echo ============================================================
echo   NUCLEAR CRAWLER HYBRID - INICIANDO SERVIDOR HTTP
echo ============================================================
echo.

cd /d "%~dp0"

REM Verificar si ya está corriendo
netstat -ano | findstr ":4000" >nul 2>&1
if %errorlevel% == 0 (
    echo ⚠️  El servidor ya está corriendo en el puerto 4000
    echo.
    echo Para detenerlo, ejecuta: DETENER_SERVIDOR.bat
    pause
    exit /b
)

REM Verificar si existe el binario
if not exist "target\release\nuclear-http.exe" (
    echo ❌ Error: No se encontró target\release\nuclear-http.exe
    echo.
    echo Compilando...
    call cargo build --release --bin nuclear-http
    if %errorlevel% neq 0 (
        echo ❌ Error al compilar
        pause
        exit /b 1
    )
)

echo ✅ Iniciando servidor HTTP en puerto 4000...
echo.
echo 📡 Endpoints disponibles:
echo    - GET  http://localhost:4000/health
echo    - POST http://localhost:4000/api/websearch
echo    - POST http://localhost:4000/api/analizar_proyecto
echo    - POST http://localhost:4000/api/scan_project
echo    - GET  http://localhost:4000/api/stats
echo.
echo Presiona Ctrl+C para detener el servidor
echo.

REM Iniciar servidor
start "Nuclear Crawler HTTP Server" /MIN target\release\nuclear-http.exe

timeout /t 3 /nobreak >nul

REM Verificar que esté corriendo
netstat -ano | findstr ":4000" >nul 2>&1
if %errorlevel% == 0 (
    echo ✅ Servidor iniciado correctamente en http://localhost:4000
    echo.
    echo Para verificar: curl http://localhost:4000/health
) else (
    echo ❌ Error: El servidor no se inició correctamente
    pause
    exit /b 1
)

pause

