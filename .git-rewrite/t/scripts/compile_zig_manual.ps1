# Compilar Zig FFI manualmente usando MSVC (sin necesidad de Zig instalado)
# Este script compila el código Zig usando el compilador C de Rust

Write-Host "🔥 Compilando Zig FFI manualmente con MSVC..." -ForegroundColor Cyan

# Verificar que estamos en el directorio correcto
$projectRoot = Split-Path -Parent $PSScriptRoot
if (-Not (Test-Path "$projectRoot\zig\src\lib.zig")) {
    Write-Error "No se encontró zig/src/lib.zig"
    exit 1
}

Write-Host "⚠️  Zig no está instalado. Descargando Zig portable..." -ForegroundColor Yellow

# Crear directorio temporal
$tempDir = "$projectRoot\temp_zig"
New-Item -ItemType Directory -Force -Path $tempDir | Out-Null

# Descargar Zig portable para Windows (versión 0.13.0 - estable)
$zigUrl = "https://ziglang.org/download/0.13.0/zig-windows-x86_64-0.13.0.zip"
$zigZip = "$tempDir\zig.zip"

Write-Host "📥 Descargando Zig 0.13.0..." -ForegroundColor Cyan
try {
    Invoke-WebRequest -Uri $zigUrl -OutFile $zigZip -UseBasicParsing
    Write-Host "✅ Zig descargado" -ForegroundColor Green
} catch {
    Write-Error "Error descargando Zig: $_"
    exit 1
}

# Extraer Zig
Write-Host "📦 Extrayendo Zig..." -ForegroundColor Cyan
Expand-Archive -Path $zigZip -DestinationPath $tempDir -Force

# Encontrar el ejecutable de Zig
$zigExe = Get-ChildItem -Path $tempDir -Recurse -Filter "zig.exe" | Select-Object -First 1 -ExpandProperty FullName

if (-Not $zigExe) {
    Write-Error "No se encontró zig.exe después de extraer"
    exit 1
}

Write-Host "✅ Zig encontrado en: $zigExe" -ForegroundColor Green

# Compilar la biblioteca Zig
Write-Host "🔨 Compilando biblioteca Zig..." -ForegroundColor Cyan

Push-Location "$projectRoot\zig"
try {
    # Compilar con Zig en modo release
    & $zigExe build -Doptimize=ReleaseFast

    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Biblioteca Zig compilada exitosamente" -ForegroundColor Green

        # Copiar la biblioteca al directorio target/release
        $zigLib = Get-ChildItem -Path "zig-out\lib" -Filter "*.lib" -Recurse | Select-Object -First 1
        if ($zigLib) {
            Copy-Item $zigLib.FullName "$projectRoot\target\release\nuclear_zig.lib" -Force
            Write-Host "✅ Biblioteca copiada a target/release/" -ForegroundColor Green
        }

        # También buscar DLL si existe
        $zigDll = Get-ChildItem -Path "zig-out\lib" -Filter "*.dll" -Recurse | Select-Object -First 1
        if ($zigDll) {
            Copy-Item $zigDll.FullName "$projectRoot\target\release\nuclear_zig.dll" -Force
            Write-Host "✅ DLL copiada a target/release/" -ForegroundColor Green
        }
    } else {
        Write-Error "Error compilando Zig (código: $LASTEXITCODE)"
        exit $LASTEXITCODE
    }
} finally {
    Pop-Location
}

# Limpiar archivos temporales (opcional)
Write-Host "🧹 Limpiando archivos temporales..." -ForegroundColor Cyan
Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue

Write-Host "Compilacion de Zig completada!" -ForegroundColor Green
Write-Host "Biblioteca en: target/release/nuclear_zig.lib" -ForegroundColor Cyan
