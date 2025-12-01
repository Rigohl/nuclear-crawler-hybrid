@echo off
REM Script rápido para iniciar en puerto 4000
echo ============================================================
echo   NUCLEAR CRAWLER - PUERTO 4000
echo ============================================================
echo.

cd /d "%~dp0"

REM Verificar si ya está corriendo
netstat -ano | findstr ":4000" >nul 2>&1
if %errorlevel% == 0 (
    echo ✅ Servidor ya está corriendo en puerto 4000
    echo.
    echo Para verificar: curl http://localhost:4000/health
    pause
    exit /b
)

REM Buscar binario
if exist "target\release\nuclear-http.exe" (
    set BINARY=target\release\nuclear-http.exe
) else if exist "target\debug\nuclear-http.exe" (
    set BINARY=target\debug\nuclear-http.exe
) else (
    echo ❌ No se encontró el binario
    echo.
    echo Compilando...
    call cargo build --bin nuclear-http
    if %errorlevel% neq 0 (
        echo ❌ Error al compilar
        pause
        exit /b 1
    )
    set BINARY=target\debug\nuclear-http.exe
)

echo ✅ Iniciando servidor en puerto 4000...
echo.
echo 📡 URL: http://localhost:4000
echo.

REM Iniciar con puerto 4000
set PORT=4000
start "Nuclear Crawler - Puerto 4000" /MIN %BINARY%

timeout /t 3 /nobreak >nul

REM Verificar
netstat -ano | findstr ":4000" >nul 2>&1
if %errorlevel% == 0 (
    echo ✅ Servidor iniciado en http://localhost:4000
    echo.
    echo Para probar: curl http://localhost:4000/health
) else (
    echo ⚠️  Servidor iniciando... espera 5 segundos
)

pause

