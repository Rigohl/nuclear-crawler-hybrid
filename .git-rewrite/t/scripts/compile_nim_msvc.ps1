# Build Nim FFI library for Nuclear Crawler Hybrid
# Compiles nuclear_nim.nim to static library for MSVC

Write-Host "🔥 Building Nim FFI Library..." -ForegroundColor Cyan

# Set working directory
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectDir = Split-Path -Parent $scriptDir
Set-Location $projectDir

# Nim compilation command for static library
$nimCmd = "nim c --app:staticlib --noMain --cpu:amd64 --os:windows --cc:vcc --verbosity:2 --hints:off --warnings:off nim\src\nuclear_nim.nim"

Write-Host "Executing: $nimCmd" -ForegroundColor Yellow

try {
    # Run Nim compilation
    $output = Invoke-Expression $nimCmd 2>&1

    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Nim library compiled successfully!" -ForegroundColor Green

        # Move the generated library to libs directory
        if (Test-Path "nim\src\nuclear_nim.lib") {
            Copy-Item "nim\src\nuclear_nim.lib" "libs\nuclear_nim.lib" -Force
            Write-Host "✅ Library moved to libs/ directory" -ForegroundColor Green
        } else {
            Write-Host "⚠️ nuclear_nim.lib not found in expected location" -ForegroundColor Yellow
        }

        # List generated files
        Write-Host "Generated files:" -ForegroundColor Cyan
        Get-ChildItem nim\src\ -Filter nuclear_nim* | ForEach-Object {
            Write-Host "  $($_.Name)" -ForegroundColor White
        }

    } else {
        Write-Host "❌ Nim compilation failed with exit code $LASTEXITCODE" -ForegroundColor Red
        Write-Host "Output:" -ForegroundColor Red
        $output | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    }

} catch {
    Write-Host "❌ Error during Nim compilation: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "Nim build process completed." -ForegroundColor Cyan
