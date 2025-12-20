# ============================================================================
# NUCLEAR CRAWLER - Compilación Go para MSVC
# ============================================================================
# Este script compila la librería Go para ser compatible con MSVC toolchain
# 
# Requisitos:
# - Go instalado (go version)
# - MinGW-w64 o MSVC Build Tools
# ============================================================================

$ErrorActionPreference = "Stop"

Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║       NUCLEAR CRAWLER - Go Library MSVC Compilation           ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan

# Directorio del proyecto
$ProjectRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$GoDir = Join-Path $ProjectRoot "go"
$GoSrc = Join-Path $GoDir "src\stealth_go.go"

Write-Host "`n📦 Directorio Go: $GoDir" -ForegroundColor Yellow

# Verificar Go instalado
try {
    $goVersion = & go version
    Write-Host "✅ Go detectado: $goVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Go no está instalado. Instálalo desde https://go.dev/dl/" -ForegroundColor Red
    exit 1
}

# Verificar que existe el archivo fuente
if (-not (Test-Path $GoSrc)) {
    Write-Host "❌ No se encontró $GoSrc" -ForegroundColor Red
    exit 1
}

# Cambiar al directorio Go
Push-Location $GoDir\src

Write-Host "`n🔧 Compilando librería Go..." -ForegroundColor Yellow

# ============================================================================
# MÉTODO 1: Compilar como DLL (c-shared) - Más compatible con MSVC
# ============================================================================
Write-Host "`n📌 Método 1: Compilando como DLL (c-shared)..." -ForegroundColor Cyan

$env:CGO_ENABLED = "1"
$env:GOOS = "windows"
$env:GOARCH = "amd64"

try {
    # Compilar como DLL
    & go build -buildmode=c-shared -o "$GoDir\stealth_go.dll" stealth_go.go
    
    if (Test-Path "$GoDir\stealth_go.dll") {
        Write-Host "✅ DLL creada: stealth_go.dll" -ForegroundColor Green
        
        # Generar .lib para MSVC desde la DLL
        $defFile = "$GoDir\stealth_go.def"
        $libFile = "$GoDir\stealth_go_msvc.lib"
        
        # Crear archivo .def manualmente
        @"
LIBRARY stealth_go
EXPORTS
    ExportStealthHeaders
    FastProcessURLs
    FreeString
    BatchProcessData
    GetVersion
"@ | Out-File -FilePath $defFile -Encoding ASCII
        
        Write-Host "📄 Archivo .def creado" -ForegroundColor Green
        
        # Intentar usar lib.exe de MSVC si está disponible
        $vcVars = @(
            "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat",
            "C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvars64.bat",
            "C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Auxiliary\Build\vcvars64.bat",
            "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat"
        )
        
        $vcFound = $false
        foreach ($vc in $vcVars) {
            if (Test-Path $vc) {
                Write-Host "🔧 Usando MSVC de: $vc" -ForegroundColor Yellow
                # Crear .lib desde .def
                cmd /c "`"$vc`" && lib /DEF:$defFile /OUT:$libFile /MACHINE:X64"
                $vcFound = $true
                break
            }
        }
        
        if (-not $vcFound) {
            Write-Host "⚠️ MSVC no encontrado, usando .dll directamente" -ForegroundColor Yellow
        }
    }
} catch {
    Write-Host "⚠️ Error compilando DLL: $_" -ForegroundColor Yellow
}

# ============================================================================
# MÉTODO 2: Compilar como archivo estático con CGO
# ============================================================================
Write-Host "`n📌 Método 2: Compilando archivo estático (.a)..." -ForegroundColor Cyan

try {
    # Compilar como archivo estático
    & go build -buildmode=c-archive -o "$GoDir\stealth_go.a" stealth_go.go
    
    if (Test-Path "$GoDir\stealth_go.a") {
        Write-Host "✅ Archivo estático creado: stealth_go.a" -ForegroundColor Green
        
        # El header se genera automáticamente
        if (Test-Path "$GoDir\stealth_go.h") {
            Write-Host "✅ Header generado: stealth_go.h" -ForegroundColor Green
        }
    }
} catch {
    Write-Host "⚠️ Error compilando archivo estático: $_" -ForegroundColor Yellow
}

Pop-Location

# ============================================================================
# RESUMEN
# ============================================================================
Write-Host "`n╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║                    COMPILACIÓN COMPLETADA                      ║" -ForegroundColor Green
Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Green

Write-Host "`n📦 Archivos generados en $GoDir`:" -ForegroundColor Cyan
Get-ChildItem $GoDir -Filter "stealth_go*" | ForEach-Object {
    $size = [math]::Round($_.Length / 1KB, 2)
    Write-Host "   • $($_.Name) ($size KB)" -ForegroundColor White
}

Write-Host "`n💡 Para usar con Rust MSVC:" -ForegroundColor Yellow
Write-Host "   1. Si tienes .dll: Copia stealth_go.dll junto al ejecutable" -ForegroundColor White
Write-Host "   2. Si tienes .lib MSVC: Modifica build.rs para linkear" -ForegroundColor White
Write-Host "   3. Alternativa: rustup target add x86_64-pc-windows-gnu" -ForegroundColor White

Write-Host "`n✅ ¡Listo! Ahora ejecuta: cargo build --release" -ForegroundColor Green
