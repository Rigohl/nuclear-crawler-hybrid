# ============================================================================
# NUCLEAR CRAWLER - Compilación Zig para MSVC
# ============================================================================
# Este script compila la librería Zig para ser compatible con MSVC toolchain
# 
# Requisitos:
# - Zig instalado (zig version)
# ============================================================================

$ErrorActionPreference = "Stop"

Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║       NUCLEAR CRAWLER - Zig Library MSVC Compilation          ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan

# Directorio del proyecto
$ProjectRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$ZigDir = Join-Path $ProjectRoot "zig"

Write-Host "`n📦 Directorio Zig: $ZigDir" -ForegroundColor Yellow

# Verificar Zig instalado
try {
    $zigVersion = & zig version
    Write-Host "✅ Zig detectado: $zigVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Zig no está instalado. Instálalo desde https://ziglang.org/download/" -ForegroundColor Red
    Write-Host "   O con: winget install -e --id zig.zig" -ForegroundColor Yellow
    exit 1
}

# Verificar que existe build.zig
if (-not (Test-Path "$ZigDir\build.zig")) {
    Write-Host "❌ No se encontró $ZigDir\build.zig" -ForegroundColor Red
    exit 1
}

# Cambiar al directorio Zig
Push-Location $ZigDir

Write-Host "`n🔧 Compilando librería Zig con target MSVC..." -ForegroundColor Yellow

# ============================================================================
# Compilar con target x86_64-windows-msvc
# ============================================================================
Write-Host "`n📌 Compilando con target: x86_64-windows-msvc" -ForegroundColor Cyan

try {
    # Limpiar builds anteriores
    if (Test-Path "zig-out") {
        Remove-Item -Recurse -Force "zig-out"
    }
    if (Test-Path ".zig-cache") {
        Remove-Item -Recurse -Force ".zig-cache"
    }
    
    # Compilar con target MSVC
    & zig build -Dtarget=x86_64-windows-msvc -Doptimize=ReleaseFast
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Compilación exitosa con target MSVC" -ForegroundColor Green
    } else {
        throw "Error en compilación"
    }
} catch {
    Write-Host "⚠️ Error con target MSVC, intentando compilación directa..." -ForegroundColor Yellow
    
    # Intentar compilación directa del archivo .zig
    try {
        & zig build-lib -target x86_64-windows-msvc -O ReleaseFast src/lib.zig -femit-bin=nuclear_zig_msvc.lib
        Write-Host "✅ Librería compilada directamente" -ForegroundColor Green
    } catch {
        Write-Host "⚠️ Compilación directa falló: $_" -ForegroundColor Yellow
    }
}

# ============================================================================
# Verificar archivos generados
# ============================================================================
Write-Host "`n📦 Buscando archivos generados..." -ForegroundColor Cyan

$libFiles = @(
    "zig-out\lib\nuclear_zig.lib",
    "zig-out\lib\libnuclear_zig.a",
    "nuclear_zig_msvc.lib",
    "lib.lib"
)

foreach ($lib in $libFiles) {
    $fullPath = Join-Path $ZigDir $lib
    if (Test-Path $fullPath) {
        $size = [math]::Round((Get-Item $fullPath).Length / 1KB, 2)
        Write-Host "   ✅ $lib ($size KB)" -ForegroundColor Green
        
        # Copiar a raíz si está en subdirectorio
        if ($lib -like "zig-out\*") {
            $destFile = Join-Path $ZigDir (Split-Path -Leaf $lib)
            Copy-Item $fullPath $destFile -Force
            Write-Host "      📋 Copiado a: $destFile" -ForegroundColor White
        }
    }
}

# ============================================================================
# Crear archivo .def para compatibilidad MSVC
# ============================================================================
$defContent = @"
LIBRARY nuclear_zig
EXPORTS
    process_data_parallel
    parse_html_fast
    process_batch_parallel
    fast_memory_copy
    fast_hash
    search_patterns
    sum_array_parallel
    update_dependencies_auto
    repair_deployment_auto
    configure_deployment_auto
"@

$defPath = Join-Path $ZigDir "nuclear_zig.def"
$defContent | Out-File -FilePath $defPath -Encoding ASCII
Write-Host "`n📄 Archivo .def creado: nuclear_zig.def" -ForegroundColor Green

Pop-Location

# ============================================================================
# RESUMEN
# ============================================================================
Write-Host "`n╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║                    COMPILACIÓN COMPLETADA                      ║" -ForegroundColor Green
Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Green

Write-Host "`n📦 Archivos en $ZigDir`:" -ForegroundColor Cyan
Get-ChildItem $ZigDir -Filter "*.lib" -ErrorAction SilentlyContinue | ForEach-Object {
    $size = [math]::Round($_.Length / 1KB, 2)
    Write-Host "   • $($_.Name) ($size KB)" -ForegroundColor White
}

Write-Host "`n💡 Para usar con Rust MSVC:" -ForegroundColor Yellow
Write-Host "   1. La librería .lib ya es compatible con MSVC" -ForegroundColor White
Write-Host "   2. Modifica build.rs para linkear nuclear_zig.lib" -ForegroundColor White

Write-Host "`n✅ ¡Listo! Ahora ejecuta: cargo build --release" -ForegroundColor Green
