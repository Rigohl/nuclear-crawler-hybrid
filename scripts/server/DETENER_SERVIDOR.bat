@echo off
REM Script para detener Nuclear Crawler Hybrid
echo ============================================================
echo   NUCLEAR CRAWLER HYBRID - DETENIENDO SERVIDOR
echo ============================================================
echo.

REM Buscar procesos
for /f "tokens=2" %%a in ('netstat -ano ^| findstr ":4000" ^| findstr "LISTENING"') do (
    echo Deteniendo proceso PID: %%a
    taskkill /F /PID %%a >nul 2>&1
)

for /f "tokens=2" %%a in ('netstat -ano ^| findstr ":3000" ^| findstr "LISTENING"') do (
    echo Deteniendo proceso PID: %%a
    taskkill /F /PID %%a >nul 2>&1
)

REM Buscar por nombre
taskkill /F /IM nuclear-http.exe >nul 2>&1
taskkill /F /IM nuclear-mcp.exe >nul 2>&1
taskkill /F /IM nuclear_crawler_hybrid.exe >nul 2>&1

echo ✅ Servidor detenido
pause

