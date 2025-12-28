# 🔥 CONSOLIDAR Y LIMPIAR RESULTADOS
# Este script:
# 1. Ejecuta el consolidador para extraer contenido REAL de todas las URLs
# 2. Mueve los JSON viejos a una carpeta de backup
# 3. O los elimina si prefieres (descomenta la línea)

$resultadosDir = "C:\Users\DELL\Desktop\hf_spaces\NUCLEAR_CRAWLER_HYBRID\resultados"
$backupDir = "$resultadosDir\backup_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
$consolidator = "C:\Users\DELL\Desktop\hf_spaces\NUCLEAR_CRAWLER_HYBRID\target\release\consolidate_results.exe"

Write-Host "🔥 CONSOLIDACIÓN Y LIMPIEZA DE RESULTADOS" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan

# Contar archivos antes
$jsonFiles = Get-ChildItem -Path $resultadosDir -Filter "*.json" | Where-Object { $_.Name -notlike "CONSOLIDATED_*" -and $_.Name -notlike "urls_*" }
$fileCount = $jsonFiles.Count
Write-Host "`n📊 Archivos JSON encontrados: $fileCount" -ForegroundColor Yellow

if ($fileCount -eq 0) {
    Write-Host "❌ No hay archivos JSON para procesar" -ForegroundColor Red
    exit
}

# Ejecutar consolidador
Write-Host "`n🔥 Ejecutando consolidador..." -ForegroundColor Green
& $consolidator

# Verificar que se creó el archivo CONSOLIDATED
$consolidated = Get-ChildItem -Path $resultadosDir -Filter "CONSOLIDATED_*.json" | Sort-Object LastWriteTime -Descending | Select-Object -First 1

if ($consolidated) {
    Write-Host "`n✅ Archivo consolidado creado: $($consolidated.Name)" -ForegroundColor Green
    Write-Host "   Tamaño: $([math]::Round($consolidated.Length / 1KB, 2)) KB" -ForegroundColor Gray
    
    # Crear backup y mover archivos viejos
    Write-Host "`n📦 Moviendo archivos viejos a backup..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Path $backupDir -Force | Out-Null
    
    foreach ($file in $jsonFiles) {
        Move-Item -Path $file.FullName -Destination $backupDir -Force
        Write-Host "   → $($file.Name)" -ForegroundColor Gray
    }
    
    Write-Host "`n✅ $fileCount archivos movidos a: $backupDir" -ForegroundColor Green
    
    # OPCIONAL: Eliminar en lugar de mover (descomenta si prefieres)
    # Remove-Item -Path $backupDir -Recurse -Force
    # Write-Host "🗑️ Backup eliminado" -ForegroundColor Red
    
} else {
    Write-Host "`n❌ Error: No se creó el archivo consolidado" -ForegroundColor Red
}

# Mostrar estado final
Write-Host "`n📂 Estado final de ./resultados:" -ForegroundColor Cyan
Get-ChildItem -Path $resultadosDir | Format-Table Name, Length, LastWriteTime -AutoSize

Write-Host "`n🎉 PROCESO COMPLETADO!" -ForegroundColor Green
