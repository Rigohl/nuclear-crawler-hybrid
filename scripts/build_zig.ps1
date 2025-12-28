# Build Zig static library for Nuclear Crawler Hybrid
# Run from project root

$zigDir = Join-Path $PSScriptRoot '..\zig'
if (-Not (Test-Path $zigDir)) {
    Write-Error "zig/ directory not found"
    exit 1
}

# Check if zig is installed
$zig = Get-Command zig -ErrorAction SilentlyContinue
if (-Not $zig) {
    Write-Error "Zig not found in PATH. Install Zig or add to PATH."
    exit 2
}

Write-Host "Building Zig lib in $zigDir"
Push-Location $zigDir
try {
    & zig build
    if ($LASTEXITCODE -ne 0) {
        Write-Error "zig build failed (exit $LASTEXITCODE)"
        exit $LASTEXITCODE
    }
    Write-Host "zig build completed"
} finally {
    Pop-Location
}
