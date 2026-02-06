# Chapel Auto-Repair System
# Compila y repara automáticamente SIN PERDER CÓDIGO

param(
    [switch]$DryRun,
    [switch]$Backup = $true
)

$ErrorActionPreference = "Continue"

Write-Host "🔧 CHAPEL AUTO-REPAIR SYSTEM" -ForegroundColor Cyan
Write-Host "=" * 70
Write-Host ""

# Chapel files prioritarios
$chapelFiles = @(
    @{
        Name = "chapel_gpu_ultra"
        Path = "chapel\chapel_gpu_ultra.chpl"
        Output = "build\chapel_gpu_ultra.exe"
        Flags = "--fast"
    },
    @{
        Name = "search_engine"
        Path = "chapel\search_engine.chpl"
        Output = "build\search_engine.exe"
        Flags = "--fast"
        Dependencies = @("build\mcp_ffi_bridge.o")
    },
    @{
        Name = "mcp_direct_integration"
        Path = "chapel\mcp_direct_integration.chpl"
        Output = "build\mcp_integration.exe"
        Flags = "--fast"
        Dependencies = @("build\mcp_ffi_bridge.o")
    }
)

# Verificar Chapel
if (-not (Get-Command chpl -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Chapel no encontrado" -ForegroundColor Red
    Write-Host "   Instala desde: https://chapel-lang.org/download.html" -ForegroundColor Yellow
    exit 1
}

$chplVersion = (chpl --version 2>&1 | Select-Object -First 1)
Write-Host "✅ Chapel detectado: $chplVersion" -ForegroundColor Green
Write-Host ""

# Crear directorio build
if (-not (Test-Path "build")) {
    New-Item -ItemType Directory -Path "build" -Force | Out-Null
}

# Función de backup
function Backup-File {
    param($FilePath)
    
    if (-not $Backup) { return }
    
    $backupPath = "$FilePath.backup_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
    Copy-Item $FilePath $backupPath -Force
    Write-Host "   💾 Backup: $backupPath" -ForegroundColor Gray
}

# Función de repair
function Repair-ChapelCode {
    param(
        [string]$FilePath,
        [string]$ErrorOutput
    )
    
    Write-Host "   🔧 Auto-reparando..." -ForegroundColor Yellow
    
    $content = Get-Content $FilePath -Raw
    $originalContent = $content
    $repaired = $false
    
    # Pattern 1: Missing semicolons
    if ($ErrorOutput -match "expected ';'|expecting ';'") {
        Write-Host "      → Reparando semicolons..." -ForegroundColor Gray
        # No modificar, Chapel no usa ; al final de statements
    }
    
    # Pattern 2: Undefined symbols
    if ($ErrorOutput -match "error: '(\w+)' undeclared") {
        $symbol = $matches[1]
        Write-Host "      → Símbolo no declarado: $symbol" -ForegroundColor Gray
        
        # Agregar imports comunes si faltan
        if ($symbol -match "^(c_int|c_char|c_ptr|c_string|c_ptrConst)$") {
            if ($content -notmatch "use CTypes;") {
                $content = "use CTypes;`n" + $content
                Write-Host "         ✓ Agregado: use CTypes;" -ForegroundColor Green
                $repaired = $true
            }
        }
        
        if ($symbol -match "^(HTTP|URL|curl)") {
            if ($content -notmatch "use URL;") {
                $content = "use URL;`n" + $content
                Write-Host "         ✓ Agregado: use URL;" -ForegroundColor Green
                $repaired = $true
            }
        }
        
        if ($symbol -match "^(Time|now|sleep)") {
            if ($content -notmatch "use Time;") {
                $content = "use Time;`n" + $content
                Write-Host "         ✓ Agregado: use Time;" -ForegroundColor Green
                $repaired = $true
            }
        }
    }
    
    # Pattern 3: Type mismatches
    if ($ErrorOutput -match "type mismatch|cannot convert") {
        Write-Host "      → Type mismatch detectado" -ForegroundColor Gray
        # Esto requiere análisis más profundo, lo dejamos para manual
    }
    
    # Pattern 4: Missing extern declarations
    if ($ErrorOutput -match "undefined reference to '(\w+)'") {
        $funcName = $matches[1]
        Write-Host "      → Función externa faltante: $funcName" -ForegroundColor Gray
        
        # Verificar si hay declaración extern
        if ($content -notmatch "extern proc $funcName") {
            Write-Host "         ⚠️  Necesita declaración extern manual" -ForegroundColor Yellow
        }
    }
    
    # Pattern 5: Syntax errors comunes
    if ($ErrorOutput -match "syntax error") {
        Write-Host "      → Error de sintaxis detectado" -ForegroundColor Gray
        
        # Fix common issues
        # Corregir dobles puntos y comas
        $content = $content -replace ";;", ";"
        
        # Corregir espacios en declaraciones
        $content = $content -replace "proc\s+(\w+)\s*\(", "proc `$1("
        
        $repaired = $true
    }
    
    # Guardar si hubo cambios
    if ($repaired -and ($content -ne $originalContent)) {
        Backup-File $FilePath
        
        if (-not $DryRun) {
            Set-Content $FilePath $content -NoNewline
            Write-Host "      ✅ Reparaciones aplicadas" -ForegroundColor Green
        } else {
            Write-Host "      🔍 DRY RUN: Cambios NO aplicados" -ForegroundColor Yellow
        }
        
        return $true
    }
    
    return $false
}

# Compilar cada archivo
$results = @()

foreach ($file in $chapelFiles) {
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor DarkGray
    Write-Host "📄 Compilando: $($file.Name)" -ForegroundColor Cyan
    Write-Host ""
    
    $filePath = Join-Path $PSScriptRoot $file.Path
    
    if (-not (Test-Path $filePath)) {
        Write-Host "   ⚠️  Archivo no encontrado: $filePath" -ForegroundColor Yellow
        $results += @{
            Name = $file.Name
            Status = "NOT_FOUND"
            Repaired = $false
        }
        continue
    }
    
    # Intentar compilar (máximo 3 intentos con auto-repair)
    $maxAttempts = 3
    $attempt = 1
    $compiled = $false
    $wasRepaired = $false
    
    while ($attempt -le $maxAttempts -and -not $compiled) {
        Write-Host "   🔨 Intento $attempt/$maxAttempts..." -ForegroundColor Gray
        
        # Construir comando
        $chplArgs = @($filePath)
        
        if ($file.Dependencies) {
            foreach ($dep in $file.Dependencies) {
                if (Test-Path $dep) {
                    $chplArgs += $dep
                }
            }
        }
        
        $chplArgs += @("-o", $file.Output)
        
        if ($file.Flags) {
            $chplArgs += $file.Flags.Split(" ")
        }
        
        # Ejecutar compilación
        $output = & chpl $chplArgs 2>&1
        $exitCode = $LASTEXITCODE
        
        if ($exitCode -eq 0) {
            Write-Host "   ✅ COMPILADO OK" -ForegroundColor Green
            $compiled = $true
            break
        } else {
            Write-Host "   ❌ Error de compilación" -ForegroundColor Red
            
            # Mostrar errores
            $errorLines = $output | Where-Object { $_ -match "error:|warning:" }
            foreach ($err in $errorLines | Select-Object -First 5) {
                Write-Host "      $err" -ForegroundColor Red
            }
            
            # Intentar reparar
            if ($attempt -lt $maxAttempts) {
                $repairResult = Repair-ChapelCode -FilePath $filePath -ErrorOutput ($output -join "`n")
                
                if ($repairResult) {
                    $wasRepaired = $true
                    Write-Host "   🔄 Reintentando compilación..." -ForegroundColor Yellow
                } else {
                    Write-Host "   ⚠️  No se pudo auto-reparar este error" -ForegroundColor Yellow
                    break
                }
            }
        }
        
        $attempt++
    }
    
    $results += @{
        Name = $file.Name
        Status = if ($compiled) { "SUCCESS" } else { "FAILED" }
        Repaired = $wasRepaired
        Output = $file.Output
    }
    
    Write-Host ""
}

# Resumen
Write-Host "=" * 70 -ForegroundColor Cyan
Write-Host "📊 RESUMEN DE COMPILACIÓN" -ForegroundColor Cyan
Write-Host "=" * 70 -ForegroundColor Cyan
Write-Host ""

$successCount = ($results | Where-Object { $_.Status -eq "SUCCESS" }).Count
$failedCount = ($results | Where-Object { $_.Status -eq "FAILED" }).Count
$notFoundCount = ($results | Where-Object { $_.Status -eq "NOT_FOUND" }).Count
$repairedCount = ($results | Where-Object { $_.Repaired }).Count

foreach ($result in $results) {
    $icon = switch ($result.Status) {
        "SUCCESS" { "✅" }
        "FAILED" { "❌" }
        "NOT_FOUND" { "⚠️" }
    }
    
    $statusColor = switch ($result.Status) {
        "SUCCESS" { "Green" }
        "FAILED" { "Red" }
        "NOT_FOUND" { "Yellow" }
    }
    
    $repairedText = if ($result.Repaired) { " (auto-reparado)" } else { "" }
    
    Write-Host "$icon $($result.Name): " -NoNewline
    Write-Host "$($result.Status)$repairedText" -ForegroundColor $statusColor
    
    if ($result.Status -eq "SUCCESS" -and $result.Output) {
        Write-Host "   → $($result.Output)" -ForegroundColor Gray
    }
}

Write-Host ""
Write-Host "Total: $successCount exitosos | $failedCount fallidos | $repairedCount reparados" -ForegroundColor Cyan
Write-Host ""

if ($failedCount -gt 0) {
    Write-Host "⚠️  Archivos con errores que requieren revisión manual:" -ForegroundColor Yellow
    
    foreach ($result in $results | Where-Object { $_.Status -eq "FAILED" }) {
        $filePath = ($chapelFiles | Where-Object { $_.Name -eq $result.Name }).Path
        Write-Host "   • $filePath" -ForegroundColor Yellow
    }
    
    Write-Host ""
    Write-Host "💡 Tip: Revisa los errores específicos arriba y:" -ForegroundColor Cyan
    Write-Host "   1. Verifica imports (use CTypes, use Time, etc.)" -ForegroundColor Gray
    Write-Host "   2. Verifica declaraciones extern" -ForegroundColor Gray
    Write-Host "   3. Verifica tipos de datos" -ForegroundColor Gray
}

if ($DryRun) {
    Write-Host ""
    Write-Host "🔍 DRY RUN MODE: Ningún cambio fue aplicado" -ForegroundColor Yellow
    Write-Host "   Ejecuta sin -DryRun para aplicar reparaciones" -ForegroundColor Gray
}

Write-Host ""
Write-Host "✅ Auto-repair completado" -ForegroundColor Green

exit $(if ($failedCount -eq 0) { 0 } else { 1 })
