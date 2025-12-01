# ===========================================
# 🐳 BUILD CHAPEL IN DOCKER - PowerShell Script
# ===========================================
# Compila el editor paralelo Chapel usando Docker
# Requiere: Docker Desktop instalado y corriendo

param(
    [switch]$Clean,
    [switch]$NoCache
)

Write-Host "🐳 BUILDING CHAPEL HPC EDITOR IN DOCKER" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Verificar Docker
try {
    $dockerVersion = docker --version 2>&1
    Write-Host "✅ Docker encontrado: $dockerVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Docker no está instalado o no está corriendo" -ForegroundColor Red
    Write-Host "💡 Instala Docker Desktop desde: https://www.docker.com/products/docker-desktop" -ForegroundColor Yellow
    exit 1
}

# Limpiar si se solicita
if ($Clean) {
    Write-Host "🧹 Limpiando imágenes y contenedores anteriores..." -ForegroundColor Yellow
    docker system prune -f
    docker rmi chapel/chapel:latest 2>$null
}

# Construir la imagen
Write-Host "🏗️ Construyendo imagen Docker con Chapel..." -ForegroundColor Yellow

$buildArgs = @("build", "-f", "Dockerfile.chapel", "-t", "nuclear-chapel-editor")

if ($NoCache) {
    $buildArgs += "--no-cache"
}

$buildArgs += "."

try {
    & docker $buildArgs
    if ($LASTEXITCODE -ne 0) {
        throw "Error en construcción Docker"
    }
} catch {
    Write-Host "❌ Error construyendo la imagen Docker" -ForegroundColor Red
    exit 1
}

# Crear directorio build si no existe
if (!(Test-Path "build")) {
    New-Item -ItemType Directory -Path "build" | Out-Null
}

# Ejecutar contenedor y copiar binario
Write-Host "📦 Ejecutando contenedor y extrayendo binario..." -ForegroundColor Yellow

try {
    # Crear contenedor temporal
    docker create --name temp-chapel nuclear-chapel-editor

    # Copiar binario compilado
    docker cp temp-chapel:/workspace/chpl_parallel_editor ./build/chpl_parallel_editor.exe

    # Limpiar contenedor temporal
    docker rm temp-chapel

} catch {
    Write-Host "❌ Error extrayendo el binario" -ForegroundColor Red
    exit 1
}

# Verificar que se copió correctamente
if (Test-Path "./build/chpl_parallel_editor.exe") {
    $fileSize = (Get-Item "./build/chpl_parallel_editor.exe").Length
    $fileSizeMB = [math]::Round($fileSize / 1MB, 2)

    Write-Host "✅ ¡Binario Chapel compilado exitosamente!" -ForegroundColor Green
    Write-Host "📁 Ubicación: ./build/chpl_parallel_editor.exe" -ForegroundColor Green
    Write-Host "📊 Tamaño: $fileSizeMB MB" -ForegroundColor Green

    # Probar que funciona
    Write-Host "🧪 Probando binario..." -ForegroundColor Yellow
    try {
        $testOutput = & "./build/chpl_parallel_editor.exe" --help 2>&1
        Write-Host "✅ Binario funcional:" -ForegroundColor Green
        Write-Host $testOutput -ForegroundColor Gray
    } catch {
        Write-Host "⚠️ Binario creado pero no se puede ejecutar directamente en Windows" -ForegroundColor Yellow
        Write-Host "💡 Ejecuta en WSL o Linux: ./chpl_parallel_editor" -ForegroundColor Yellow
    }

} else {
    Write-Host "❌ Error: No se pudo copiar el binario" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "🎉 ¡COMPILACIÓN COMPLETADA!" -ForegroundColor Green
Write-Host "🚀 El editor paralelo Chapel está listo en: ./build/chpl_parallel_editor.exe" -ForegroundColor Green
Write-Host ""
Write-Host "💡 Para usar en Windows con WSL:" -ForegroundColor Cyan
Write-Host "   wsl ./build/chpl_parallel_editor.exe [argumentos]" -ForegroundColor White
Write-Host ""
Write-Host "💡 Para usar en Linux/Mac:" -ForegroundColor Cyan
Write-Host "   ./build/chpl_parallel_editor [argumentos]" -ForegroundColor White