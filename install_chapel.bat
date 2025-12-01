@echo off
echo ===========================================
echo 🟡 INSTALANDO CHAPEL COMPILER
echo ===========================================
echo.

echo 🔍 Verificando si Chapel ya está instalado...
chpl --version >nul 2>&1
if %errorlevel% equ 0 (
    echo ✅ Chapel ya está instalado
    chpl --version
    goto :end
)

echo ❌ Chapel no está instalado
echo.
echo 📦 Descargando Chapel compiler...
echo.

REM Crear directorio temporal
if not exist temp_chapel mkdir temp_chapel
cd temp_chapel

REM Descargar Chapel para Windows
echo ⬇️  Descargando chapel-1.31.0-win64.zip...
powershell -Command "& {Invoke-WebRequest -Uri 'https://github.com/chapel-lang/chapel/releases/download/1.31.0/chapel-1.31.0-win64.zip' -OutFile 'chapel-1.31.0-win64.zip'}"

if not exist chapel-1.31.0-win64.zip (
    echo ❌ Error: No se pudo descargar Chapel
    cd ..
    rmdir /s /q temp_chapel
    pause
    exit /b 1
)

echo 📦 Extrayendo archivos...
powershell -Command "& {Expand-Archive -Path 'chapel-1.31.0-win64.zip' -DestinationPath '.'}"

if not exist chapel-1.31.0 (
    echo ❌ Error: No se pudo extraer Chapel
    cd ..
    rmdir /s /q temp_chapel
    pause
    exit /b 1
)

echo ⚙️  Configurando variables de entorno...
set "CHPL_HOME=%CD%\chapel-1.31.0"
set "PATH=%CHPL_HOME%\bin\%CHPL_TARGET_PLATFORM%;%PATH%"

echo 📝 Creando script de configuración...
echo @echo off > chapel_env.bat
echo set "CHPL_HOME=%CHPL_HOME%">> chapel_env.bat
echo set "PATH=%%CHPL_HOME%%\bin\%%CHPL_TARGET_PLATFORM%%;%%PATH%%">> chapel_env.bat
echo echo Chapel environment configured>> chapel_env.bat

echo 🧪 Probando instalación...
chpl --version
if %errorlevel% neq 0 (
    echo ❌ Error: Chapel no se instaló correctamente
    cd ..
    pause
    exit /b 1
)

echo.
echo ✅ ¡Chapel instalado exitosamente!
echo.
echo 📚 PARA USAR CHAPEL:
echo    • Ejecuta: chapel_env.bat
echo    • O agrega manualmente a PATH: %CHPL_HOME%\bin
echo.
echo 🎯 AHORA PUEDES USAR:
echo    • chpl --version (ver versión)
echo    • ./build_chpl_editor.bat (compilar editor)
echo.

cd ..
pause

:end