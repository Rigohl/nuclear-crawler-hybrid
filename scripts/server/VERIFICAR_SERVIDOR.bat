@echo off
REM Script para verificar estado de Nuclear Crawler
echo ============================================================
echo   NUCLEAR CRAWLER HYBRID - VERIFICACIÓN
echo ============================================================
echo.

cd /d "%~dp0"

REM Verificar puerto 4000
netstat -ano | findstr ":4000" >nul 2>&1
if %errorlevel% == 0 (
    echo ✅ Puerto 4000 (HTTP) está activo
    curl -s http://localhost:4000/health 2>nul | findstr "ok" >nul
    if %errorlevel% == 0 (
        echo ✅ Servidor HTTP respondiendo correctamente
    ) else (
        echo ⚠️  Puerto activo pero no responde
    )
) else (
    echo ❌ Puerto 4000 no está activo
)

echo.

REM Verificar puerto 3000
netstat -ano | findstr ":3000" >nul 2>&1
if %errorlevel% == 0 (
    echo ✅ Puerto 3000 (MCP) está activo
) else (
    echo ❌ Puerto 3000 no está activo
)

echo.

REM Verificar procesos
tasklist | findstr /i "nuclear" >nul 2>&1
if %errorlevel% == 0 (
    echo ✅ Procesos de Nuclear Crawler encontrados:
    tasklist | findstr /i "nuclear"
) else (
    echo ❌ No hay procesos de Nuclear Crawler corriendo
)

echo.
pause

