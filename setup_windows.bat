@echo off
REM Nuclear Crawler Hybrid - Complete Setup Script for Windows
REM Run as Administrator

setlocal enabledelayedexpansion

set INSTALL_DIR=C:\Users\DELL\Desktop\hf_spaces\NUCLEAR_CRAWLER_HYBRID
set REPO_URL=https://github.com/Rigohl/nuclear-crawler-hybrid.git
set APPDATA_PATH=%APPDATA%

echo.
echo ============================================================
echo  🔥 NUCLEAR CRAWLER HYBRID - SETUP (Windows)
echo ============================================================
echo.

REM Check if directory exists
if exist "%INSTALL_DIR%" (
    echo 📂 Directorio ya existe: %INSTALL_DIR%
    echo Actualizando...
    cd /d "%INSTALL_DIR%"
    git pull origin main
) else (
    echo 📥 Clonando repositorio...
    cd /d "C:\Users\DELL\Desktop\hf_spaces"
    git clone %REPO_URL% NUCLEAR_CRAWLER_HYBRID
    cd NUCLEAR_CRAWLER_HYBRID
)

echo.
echo 🐳 Iniciando Docker...
docker-compose up -d

REM Wait for container to start
echo ⏳ Esperando que el container inicie...
timeout /t 5 /nobreak

REM Verify container
echo ✅ Verificando container...
docker ps | findstr "nuclear-mcp-server"

echo.
echo 🔧 Configurando VS Code...

REM Configure VS Code
set VS_CODE_CONFIG=%APPDATA%\Code\User\settings.json
if not exist "%APPDATA%\Code\User" mkdir "%APPDATA%\Code\User"

REM Create or update settings.json for VS Code
powershell -Command "^
$configPath = '%VS_CODE_CONFIG%'; ^
if (Test-Path $configPath) { ^
    $config = Get-Content $configPath -Raw | ConvertFrom-Json ^
} else { ^
    $config = @{} ^
}; ^
if (-not $config.'modelContextProtocol.servers') { ^
    $config | Add-Member -NotePropertyName 'modelContextProtocol.servers' -NotePropertyValue @{} ^
}; ^
$config.'modelContextProtocol.servers'.'nuclear-crawler-hybrid' = @{ ^
    type = 'http'; ^
    url = 'http://localhost:8079' ^
}; ^
$config | ConvertTo-Json -Depth 10 | Set-Content $configPath -Encoding UTF8; ^
Write-Host '✅ VS Code configurado'
"

echo.
echo 🔧 Configurando VS Code Insiders...

REM Configure VS Code Insiders
set VS_INSIDERS_CONFIG=%APPDATA%\Code - Insiders\User\settings.json
if not exist "%APPDATA%\Code - Insiders\User" mkdir "%APPDATA%\Code - Insiders\User"

powershell -Command "^
$configPath = '%VS_INSIDERS_CONFIG%'; ^
if (Test-Path $configPath) { ^
    $config = Get-Content $configPath -Raw | ConvertFrom-Json ^
} else { ^
    $config = @{} ^
}; ^
if (-not $config.'modelContextProtocol.servers') { ^
    $config | Add-Member -NotePropertyName 'modelContextProtocol.servers' -NotePropertyValue @{} ^
}; ^
$config.'modelContextProtocol.servers'.'nuclear-crawler-hybrid' = @{ ^
    type = 'http'; ^
    url = 'http://localhost:8079' ^
}; ^
$config | ConvertTo-Json -Depth 10 | Set-Content $configPath -Encoding UTF8; ^
Write-Host '✅ VS Code Insiders configurado'
"

echo.
echo ============================================================
echo  ✅ INSTALACIÓN COMPLETA
echo ============================================================
echo.
echo 📂 Directorio: %INSTALL_DIR%
echo 🐳 Docker: http://localhost:8079
echo 🔧 VS Code: Configurado
echo 🔧 VS Code Insiders: Configurado
echo.
echo 🚀 Próximos pasos:
echo    1. Reinicia VS Code / VS Code Insiders
echo    2. Abre la command palette (Ctrl+Shift+P)
echo    3. Busca "MCP" o usa los 4 tools directamente
echo.
echo 📚 Tools disponibles:
echo    • websearch (5s timeout)
echo    • deepweb_search (10s timeout)
echo    • premium_content_scraper (15s timeout)
echo    • file_search (8s timeout)
echo.
pause
