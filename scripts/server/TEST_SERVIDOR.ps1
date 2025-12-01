# Script de prueba para verificar que Nuclear Crawler funciona
Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  TEST NUCLEAR CRAWLER HYBRID" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

$baseUrl = "http://localhost:4000"

# Test 1: Health check
Write-Host "1️⃣ Test: Health Check" -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/health" -Method Get -TimeoutSec 5
    Write-Host "   ✅ Health check OK" -ForegroundColor Green
    Write-Host "   Response: $($response | ConvertTo-Json -Compress)" -ForegroundColor Gray
} catch {
    Write-Host "   ❌ Health check falló: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""

# Test 2: Web Search
Write-Host "2️⃣ Test: Web Search" -ForegroundColor Yellow
try {
    $body = @{
        query = "Rust async"
        max_results = 5
    } | ConvertTo-Json

    $response = Invoke-RestMethod -Uri "$baseUrl/api/websearch" -Method Post -Body $body -ContentType "application/json" -TimeoutSec 30
    Write-Host "   ✅ Web search OK" -ForegroundColor Green
    Write-Host "   Resultados: $($response.result.Count)" -ForegroundColor Gray
} catch {
    Write-Host "   ⚠️  Web search falló: $_" -ForegroundColor Yellow
}

Write-Host ""

# Test 3: Scan Project
Write-Host "3️⃣ Test: Scan Project" -ForegroundColor Yellow
try {
    $body = @{
        project_path = "."
        search_solutions = $true
    } | ConvertTo-Json

    $response = Invoke-RestMethod -Uri "$baseUrl/api/scan_project" -Method Post -Body $body -ContentType "application/json" -TimeoutSec 30
    Write-Host "   ✅ Scan project OK" -ForegroundColor Green
    Write-Host "   Issues encontrados: $($response.result.total_issues)" -ForegroundColor Gray
} catch {
    Write-Host "   ⚠️  Scan project falló: $_" -ForegroundColor Yellow
}

Write-Host ""

# Test 4: Stats
Write-Host "4️⃣ Test: Stats" -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/stats" -Method Get -TimeoutSec 5
    Write-Host "   ✅ Stats OK" -ForegroundColor Green
    Write-Host "   Response: $($response | ConvertTo-Json -Compress)" -ForegroundColor Gray
} catch {
    Write-Host "   ⚠️  Stats falló: $_" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  ✅ TESTS COMPLETADOS" -ForegroundColor Green
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

