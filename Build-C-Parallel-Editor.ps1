# Build-C-Parallel-Editor.ps1
# ===========================
# Script de PowerShell para compilar y ejecutar el C Parallel Code Editor en Windows
# Requiere MinGW-w64 o MSYS2 con GCC instalado

param(
    [switch]$Debug,
    [switch]$Profile,
    [switch]$Static,
    [switch]$Test,
    [switch]$Benchmark,
    [switch]$Install,
    [switch]$Clean,
    [switch]$Chapel,
    [switch]$Help
)

# Configuración
$ProjectName = "c_parallel_editor"
$SourceFile = "c_parallel_editor.c"
$OutputFile = "$ProjectName.exe"

# Flags de compilación HPC
$BaseFlags = @(
    "-O3",
    "-march=native",
    "-flto",
    "-funroll-loops",
    "-fomit-frame-pointer",
    "-pthread",
    "-Wall",
    "-Wextra",
    "-Wpedantic",
    "-std=c99",
    "-D_GNU_SOURCE"
)

# Flags HPC adicionales
$HPCFlags = @(
    "-fopenmp",
    "-ftree-vectorize",
    "-floop-parallelize-all"
)

# Librerías
$Libraries = @(
    "-pthread",
    "-lm",
    "-lpcre2-8"
)

function Write-Header {
    Write-Host "🔬 C PARALLEL CODE EDITOR - HPC POWER" -ForegroundColor Cyan
    Write-Host "=====================================" -ForegroundColor Cyan
    Write-Host ""
}

function Get-SystemInfo {
    Write-Host "🖥️  Información del sistema:" -ForegroundColor Yellow
    try {
        $cpuCount = (Get-CimInstance Win32_ComputerSystem).NumberOfLogicalProcessors
        Write-Host "   CPU: $cpuCount cores" -ForegroundColor White
    } catch {
        Write-Host "   CPU: N/A" -ForegroundColor Red
    }

    try {
        $memory = Get-CimInstance Win32_ComputerSystem | Select-Object -ExpandProperty TotalPhysicalMemory
        $memoryGB = [math]::Round($memory / 1GB, 2)
        Write-Host "   Memoria: $memoryGB GB" -ForegroundColor White
    } catch {
        Write-Host "   Memoria: N/A" -ForegroundColor Red
    }

    Write-Host "   Compilador: GCC (MinGW-w64/MSYS2)" -ForegroundColor White
    Write-Host ""
}

function Test-GCC {
    try {
        $gccVersion = & gcc --version 2>$null | Select-Object -First 1
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ GCC encontrado: $gccVersion" -ForegroundColor Green
            return $true
        }
    } catch {
        # Continuar
    }

    Write-Host "❌ GCC no encontrado. Instale MinGW-w64 o MSYS2." -ForegroundColor Red
    Write-Host "   MinGW-w64: https://www.mingw-w64.org/" -ForegroundColor Yellow
    Write-Host "   MSYS2: https://www.msys2.org/" -ForegroundColor Yellow
    return $false
}

function Build-Project {
    param([string[]]$ExtraFlags = @())

    Write-Host "🔨 Compilando con optimizaciones HPC..." -ForegroundColor Blue

    $allFlags = $BaseFlags + $HPCFlags + $ExtraFlags
    $command = "gcc $($allFlags -join ' ') $SourceFile -o $OutputFile $($Libraries -join ' ')"

    Write-Host "Comando: $command" -ForegroundColor Gray

    try {
        Invoke-Expression $command
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ Compilación exitosa!" -ForegroundColor Green
            $fileInfo = Get-Item $OutputFile
            Write-Host "📁 Tamaño del binario: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor White
            return $true
        } else {
            Write-Host "❌ Error de compilación (código: $LASTEXITCODE)" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "❌ Error ejecutando GCC: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
}

function Run-Tests {
    Write-Host "🧪 Ejecutando pruebas..." -ForegroundColor Blue

    # Crear archivos de prueba
    Write-Host "📝 Creando archivos de prueba..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Force -Path "test_files" | Out-Null
    "void oldFunction() { printf(`"old`"); }" | Out-File -FilePath "test_files\test1.c" -Encoding UTF8
    "oldFunction(); oldFunction();" | Out-File -FilePath "test_files\test2.c" -Encoding UTF8

    # Dry-run
    Write-Host "🔍 Ejecutando dry-run..." -ForegroundColor Yellow
    & .\$OutputFile "test_files" "oldFunction" "newFunction" "--dry-run"

    # Aplicar cambios
    Write-Host "✏️  Aplicando cambios..." -ForegroundColor Yellow
    & .\$OutputFile "test_files" "oldFunction" "newFunction"

    # Verificar
    Write-Host "🔍 Verificando resultados..." -ForegroundColor Yellow
    Get-Content "test_files\test1.c"
    Get-Content "test_files\test2.c"

    # Limpiar
    Write-Host "🧹 Limpiando..." -ForegroundColor Yellow
    Remove-Item -Recurse -Force "test_files"
}

function Run-Benchmark {
    Write-Host "📊 Ejecutando benchmark..." -ForegroundColor Blue

    # Crear archivos de prueba
    Write-Host "📝 Preparando archivos de prueba..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Force -Path "benchmark_data" | Out-Null

    1..1000 | ForEach-Object {
        "void function$_.c() { oldFunction(); }" | Out-File -FilePath "benchmark_data\file$_.c" -Encoding UTF8
    }

    # Ejecutar benchmark
    Write-Host "⚡ Ejecutando benchmark..." -ForegroundColor Yellow
    $startTime = Get-Date
    & .\$OutputFile "benchmark_data" "oldFunction" "newFunction"
    $endTime = Get-Date
    $duration = $endTime - $startTime

    Write-Host "⏱️  Tiempo total: $($duration.TotalSeconds) segundos" -ForegroundColor Green

    # Limpiar
    Write-Host "🧹 Limpiando..." -ForegroundColor Yellow
    Remove-Item -Recurse -Force "benchmark_data"
}

function Show-Help {
    Write-Header
    Write-Host "🎯 Parámetros disponibles:" -ForegroundColor Yellow
    Write-Host "  -Debug      Compilar con debugging (-g -O0)" -ForegroundColor White
    Write-Host "  -Profile    Compilar con profiling (-pg)" -ForegroundColor White
    Write-Host "  -Static     Compilar binario estático" -ForegroundColor White
    Write-Host "  -Test       Ejecutar pruebas después de compilar" -ForegroundColor White
    Write-Host "  -Benchmark  Ejecutar benchmark después de compilar" -ForegroundColor White
    Write-Host "  -Install    Instalar en PATH del sistema" -ForegroundColor White
    Write-Host "  -Clean      Limpiar archivos generados" -ForegroundColor White
    Write-Host "  -Help       Mostrar esta ayuda" -ForegroundColor White
    Write-Host ""
    Write-Host "⚡ Optimizaciones HPC aplicadas:" -ForegroundColor Cyan
    Write-Host "  • -O3 -march=native -flto" -ForegroundColor White
    Write-Host "  • -fopenmp -ftree-vectorize" -ForegroundColor White
    Write-Host "  • -funroll-loops -fomit-frame-pointer" -ForegroundColor White
    Write-Host "  • Paralelización automática de bucles" -ForegroundColor White
    Write-Host ""
    Write-Host "💡 Ejemplos de uso:" -ForegroundColor Yellow
    Write-Host "  .\Build-C-Parallel-Editor.ps1" -ForegroundColor White
    Write-Host "  .\Build-C-Parallel-Editor.ps1 -Test" -ForegroundColor White
    Write-Host "  .\Build-C-Parallel-Editor.ps1 -Debug -Test" -ForegroundColor White
}

# Función principal
function Main {
    Write-Header

    if ($Help) {
        Show-Help
        return
    }

    if ($Clean) {
        Write-Host "🧹 Limpiando archivos generados..." -ForegroundColor Yellow
        Remove-Item -Force -ErrorAction SilentlyContinue $OutputFile, "*.o", "*.gcda", "*.gcno", "gmon.out", "chpl_parallel_editor*"
        Remove-Item -Recurse -Force -ErrorAction SilentlyContinue "test_files", "benchmark_data"
        Write-Host "✅ Limpieza completada" -ForegroundColor Green
        return
    }

    Get-SystemInfo

    if (!(Test-GCC)) {
        return
    }

    # Compilar Chapel primero si se solicita
    if ($Chapel) {
        Write-Host "🌟 Compilando integración Chapel..." -ForegroundColor Cyan
        if (!(Compile-Chapel)) {
            Write-Host "⚠️  Chapel no disponible, continuando solo con C" -ForegroundColor Yellow
        }
    }

    # Determinar flags adicionales
    $extraFlags = @()
    if ($Debug) {
        $extraFlags = @("-g", "-O0", "-DDEBUG")
        Write-Host "🐛 Modo debug activado" -ForegroundColor Yellow
    } elseif ($Profile) {
        $extraFlags = @("-pg", "-g")
        Write-Host "📊 Modo profiling activado" -ForegroundColor Yellow
    }

    if ($Static) {
        $Libraries += "-static"
        Write-Host "📦 Modo estático activado" -ForegroundColor Yellow
    }

    # Compilar
    if (!(Build-Project -ExtraFlags $extraFlags)) {
        return
    }

    # Ejecutar pruebas si solicitado
    if ($Test) {
        Run-Tests
    }

    # Ejecutar benchmark si solicitado
    if ($Benchmark) {
        Run-Benchmark
    }

    # Instalar si solicitado
    if ($Install) {
        Write-Host "📥 Instalando en PATH..." -ForegroundColor Yellow
        # En Windows, copiar a un directorio en PATH
        $installPath = "$env:USERPROFILE\bin"
        New-Item -ItemType Directory -Force -Path $installPath | Out-Null
        Copy-Item $OutputFile -Destination $installPath
        Write-Host "✅ Instalado en: $installPath\$OutputFile" -ForegroundColor Green
        Write-Host "💡 Agregue '$installPath' a su PATH si no está incluido" -ForegroundColor Cyan
    }

    Write-Host ""
    Write-Host "🎉 ¡Listo! El editor paralelo C está disponible como '$OutputFile'" -ForegroundColor Green
    if ($Chapel -and (Test-Path "chpl_parallel_editor.exe")) {
        Write-Host "🌟 Chapel integrado y listo para procesamiento HPC masivo" -ForegroundColor Cyan
    }
}

function Compile-Chapel {
    # Verificar si Chapel está disponible
    try {
        $chplVersion = & chpl --version 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ Chapel compiler encontrado: $chplVersion" -ForegroundColor Green
        } else {
            throw "Chapel no encontrado"
        }
    } catch {
        Write-Host "⚠️  Chapel compiler no encontrado localmente" -ForegroundColor Yellow
        Write-Host "🐳 Intentando con Docker..." -ForegroundColor Cyan

        # Intentar con Docker
        try {
            $dockerVersion = & docker --version 2>$null
            if ($LASTEXITCODE -eq 0) {
                Write-Host "✅ Docker encontrado, compilando Chapel via Docker..." -ForegroundColor Green

                $dockerCmd = "docker run --rm -v ${PWD}:/workspace -w /workspace chapel/chapel:latest chpl --fast --specialize --optimize-forall-unordered-ops --optimize-loop-iterators --optimize-on-clauses --optimize-on --inline --vectorize --cache-remote --report-optimized-loop-iterators chpl_parallel_editor.chpl -o chpl_parallel_editor"

                $result = Invoke-Expression $dockerCmd
                if ($LASTEXITCODE -eq 0) {
                    Write-Host "✅ Chapel compilado exitosamente via Docker" -ForegroundColor Green
                    return $true
                }
            }
        } catch {
            Write-Host "❌ Docker no disponible" -ForegroundColor Red
        }

        Write-Host "❌ No se pudo compilar Chapel" -ForegroundColor Red
        return $false
    }

    # Compilar Chapel localmente
    Write-Host "🔨 Compilando Chapel con optimizaciones HPC..." -ForegroundColor Blue

    $compileCmd = "chpl --fast --specialize --optimize-forall-unordered-ops --optimize-loop-iterators --optimize-on-clauses --optimize-on --inline --vectorize --cache-remote --report-optimized-loop-iterators chpl_parallel_editor.chpl -o chpl_parallel_editor"

    try {
        Invoke-Expression $compileCmd
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ Chapel compilado exitosamente" -ForegroundColor Green
            return $true
        } else {
            Write-Host "❌ Error compilando Chapel" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "❌ Error ejecutando Chapel compiler: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
}

# Ejecutar función principal
Main