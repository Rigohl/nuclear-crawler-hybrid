@echo off
echo ===========================================
echo 🐳 CHAPEL HPC EDITOR - DOCKER BUILD
echo ===========================================
echo.

REM Verificar Docker
docker --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Docker no está instalado
    echo.
    echo 💡 INSTALACIÓN DE DOCKER:
    echo.
    echo 🪟 Windows:
    echo    1. Descarga Docker Desktop: https://www.docker.com/products/docker-desktop
    echo    2. Instala y ejecuta como administrador
    echo    3. Reinicia PowerShell y ejecuta: docker --version
    echo.
    echo 🐧 WSL/Linux:
    echo    sudo apt update ^&^& sudo apt install docker.io
    echo.
    echo 🍎 macOS:
    echo    Instala Docker Desktop desde la web
    echo.
    echo 🔄 Después ejecuta: docker_chapel.bat build
    echo.
    pause
    exit /b 1
)

if "%1"=="run" goto :run
if "%1"=="build" goto :build
if "%1"=="shell" goto :shell
if "%1"=="test" goto :test

echo 💡 USO:
echo    docker_chapel.bat build    - Construir imagen
echo    docker_chapel.bat run      - Ejecutar herramienta
echo    docker_chapel.bat shell    - Abrir shell en contenedor
echo    docker_chapel.bat test     - Probar funcionalidad
echo.
pause
exit /b 1

:build
echo 🔨 Construyendo imagen Docker con Chapel oficial...
echo ⚡ Usando imagen base: chapel/chapel:latest
echo ⏱️  Esto será mucho más rápido que compilar Chapel...
echo.

docker build -f Dockerfile.chapel -t chapel-hpc-editor .

if %errorlevel% neq 0 (
    echo ❌ Error al construir la imagen
    echo.
    echo 💡 Posibles soluciones:
    echo    • Verifica conexión a internet
    echo    • La imagen oficial puede estar caída temporalmente
    echo    • Ejecuta: docker system prune -a
    echo    • Usa imagen alternativa: docker pull chapel/chapel:1.31
    echo.
    pause
    exit /b 1
)

echo ✅ Imagen construida exitosamente
echo.
echo 🎯 Información de la imagen:
docker images chapel-hpc-editor
echo.
echo 🚀 Listo para usar: docker_chapel.bat run
goto :end

:run
echo 🚀 Ejecutando Chapel HPC Editor...
echo.

REM Verificar si la imagen existe
docker images chapel-hpc-editor | findstr chapel-hpc-editor >nul
if %errorlevel% neq 0 (
    echo ❌ Imagen no encontrada
    echo 💡 Ejecuta primero: docker_chapel.bat build
    pause
    exit /b 1
)

REM Verificar parámetros
if "%2"=="" (
    echo 💡 Uso: docker_chapel.bat run [directorio] [patrón_búsqueda] [patrón_reemplazo]
    echo 📚 Ejemplo: docker_chapel.bat run . "old_text" "new_text"
    echo 📚 Ejemplo: docker_chapel.bat run /src "println!" "writeln!"
    pause
    exit /b 1
)

echo 📁 Montando directorio actual y ejecutando...
echo 🔍 Patrón: %3
echo ✏️  Reemplazo: %4
echo.

docker run --rm -v "%CD%:/workspace" -w /workspace chapel-hpc-editor ./chpl_parallel_editor %2 %3 %4 %5 %6 %7 %8 %9

goto :end

:shell
echo 🐚 Abriendo shell interactivo en contenedor Chapel...
echo 💡 Dentro del contenedor puedes:
echo    • chpl --version (ver Chapel)
echo    • ./chpl_parallel_editor --help (ayuda)
echo    • vim chpl_parallel_editor.chpl (editar código)
echo.

docker run --rm -it -v "%CD%:/workspace" -w /workspace chapel-hpc-editor /bin/bash
goto :end

:test
echo 🧪 Probando funcionalidad de Chapel HPC Editor...
echo.

REM Verificar imagen
docker images chapel-hpc-editor | findstr chapel-hpc-editor >nul
if %errorlevel% neq 0 (
    echo ❌ Imagen no encontrada - ejecuta build primero
    goto :end
)

echo 📝 Creando archivos de prueba...
if not exist test_chapel mkdir test_chapel
echo // Archivo de prueba 1 > test_chapel\test1.chpl
echo writeln("Hello from Chapel!"); >> test_chapel\test1.chpl
echo oldFunction(); >> test_chapel\test1.chpl

echo // Archivo de prueba 2 > test_chapel\test2.chpl
echo oldFunction(); >> test_chapel\test2.chpl
echo writeln("Another test"); >> test_chapel\test2.chpl

echo ✅ Archivos de prueba creados
echo.

echo 🧪 Ejecutando prueba...
docker run --rm -v "%CD%:/workspace" -w /workspace chapel-hpc-editor ./chpl_parallel_editor test_chapel "oldFunction" "newFunction"

echo.
echo 🔍 Verificando resultados...
if exist test_chapel\test1.chpl (
    echo 📄 Contenido de test1.chpl:
    type test_chapel\test1.chpl
    echo.
)
if exist test_chapel\test2.chpl (
    echo 📄 Contenido de test2.chpl:
    type test_chapel\test2.chpl
    echo.
)

echo 🧹 Limpiando archivos de prueba...
rmdir /s /q test_chapel 2>nul

echo ✅ Prueba completada
goto :end

:end
echo.
echo 🎉 ¡Operación completada!
pause