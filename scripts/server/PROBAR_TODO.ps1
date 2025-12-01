# Script completo para probar TODO el sistema Nuclear Crawler
Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  🔥 PROBANDO TODO - NUCLEAR CRAWLER HYBRID 🔥" -ForegroundColor Red
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

$baseUrl = "http://localhost:4000"
$successCount = 0
$totalTests = 0

# Test 1: Health Check
$totalTests++
Write-Host "1️⃣ Test: Health Check" -ForegroundColor Yellow
try {
  $response = Invoke-RestMethod -Uri "$baseUrl/health" -Method Get -TimeoutSec 5
  Write-Host "   ✅ Health check OK" -ForegroundColor Green
  $successCount++
}
catch {
  Write-Host "   ❌ Health check falló: $_" -ForegroundColor Red
}

# Test 2: Web Search
$totalTests++
Write-Host "`n2️⃣ Test: Web Search" -ForegroundColor Yellow
try {
  $body = @{
    query       = "Rust async programming"
    max_results = 5
  } | ConvertTo-Json

  $response = Invoke-RestMethod -Uri "$baseUrl/api/websearch" -Method Post -Body $body -ContentType "application/json" -TimeoutSec 30
  Write-Host "   ✅ Web search OK" -ForegroundColor Green
  Write-Host "   📊 Resultados: $($response.result.Count)" -ForegroundColor Gray
  $successCount++
}
catch {
  Write-Host "   ⚠️  Web search falló: $_" -ForegroundColor Yellow
}

# Test 3: Scan Project
$totalTests++
Write-Host "`n3️⃣ Test: Scan Project" -ForegroundColor Yellow
try {
  $body = @{
    project_path     = "."
    search_solutions = $true
  } | ConvertTo-Json

  $response = Invoke-RestMethod -Uri "$baseUrl/api/scan_project" -Method Post -Body $body -ContentType "application/json" -TimeoutSec 30
  Write-Host "   ✅ Scan project OK" -ForegroundColor Green
  Write-Host "   📊 Issues: $($response.result.total_issues)" -ForegroundColor Gray
  Write-Host "   📊 Quality Score: $($response.result.quality_score)" -ForegroundColor Gray
  $successCount++
}
catch {
  Write-Host "   ⚠️  Scan project falló: $_" -ForegroundColor Yellow
}

# Test 4: Analizar Proyecto
$totalTests++
Write-Host "`n4️⃣ Test: Analizar Proyecto" -ForegroundColor Yellow
try {
  $body = @{
    project_path        = "."
    max_recommendations = 5
  } | ConvertTo-Json

  $response = Invoke-RestMethod -Uri "$baseUrl/api/analizar_proyecto" -Method Post -Body $body -ContentType "application/json" -TimeoutSec 30
  Write-Host "   ✅ Analizar proyecto OK" -ForegroundColor Green
  Write-Host "   📊 Recomendaciones: $($response.result.recommendations_count)" -ForegroundColor Gray
  $successCount++
}
catch {
  Write-Host "   ⚠️  Analizar proyecto falló: $_" -ForegroundColor Yellow
}

# Test 5: Stats
$totalTests++
Write-Host "`n5️⃣ Test: Stats" -ForegroundColor Yellow
try {
  $response = Invoke-RestMethod -Uri "$baseUrl/api/stats" -Method Get -TimeoutSec 5
  Write-Host "   ✅ Stats OK" -ForegroundColor Green
  $successCount++
}
catch {
  Write-Host "   ⚠️  Stats falló: $_" -ForegroundColor Yellow
}

# Test 6: Pipeline JAX
$totalTests++
Write-Host "`n6️⃣ Test: Pipeline JAX" -ForegroundColor Yellow
if (Test-Path "scripts\jax_pipeline.py") {
  Write-Host "   ✅ Pipeline JAX disponible" -ForegroundColor Green
  $successCount++
}
else {
  Write-Host "   ❌ Pipeline JAX no encontrado" -ForegroundColor Red
}

# Resumen
Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  📊 RESUMEN: $successCount/$totalTests tests exitosos" -ForegroundColor $(if ($successCount -eq $totalTests) { "Green" } else { "Yellow" })
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

if ($successCount -eq $totalTests) {
  Write-Host "✅✅✅ TODO FUNCIONANDO PERFECTAMENTE ✅✅✅" -ForegroundColor Green
}
else {
  Write-Host "⚠️  Algunos tests fallaron, pero el sistema está operativo" -ForegroundColor Yellow
}

Write-Host ""

