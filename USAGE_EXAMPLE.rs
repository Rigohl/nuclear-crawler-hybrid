// 📖 EJEMPLO DE USO - WebSearchCompleteTool
// ==========================================

// En tu archivo Rust (por ejemplo, en src/mcp/server.rs o similar):

use nuclear_crawler_hybrid::mcp::tools::{
    WebSearchCompleteTool, 
    WebSearchCompleteConfig
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // PASO 1: Crear configuración
    // ────────────────────────────
    let config = WebSearchCompleteConfig {
        query: "rust async programming best practices".to_string(),
        max_results: 50,
        
        // OPCIONALES - Desactivados por defecto
        use_deep_web: false,           // ← Activar para búsqueda Tor
        vectorize_results: false,      // ← Activar para ML scoring
        use_parallel: true,            // ← Ya activado por defecto
        include_extracted_content: true, // ← Descargar contenido completo
        store_results: true,           // ← Guardar en disco
        enable_caching: true,          // ← Usar caché LRU
        rerank_results: false,         // ← Usar reordenamiento
        timeout_seconds: 15,           // ← Timeout global
    };

    println!("🔍 Iniciando búsqueda...");
    println!("Query: {}", config.query);

    // PASO 2: Inicializar herramienta
    // ────────────────────────────────
    let tool = WebSearchCompleteTool::new(config.clone()).await?;
    println!("✅ Herramienta inicializada correctamente");

    // PASO 3: Ejecutar búsqueda
    // ─────────────────────────
    let results = tool.search(config).await?;
    println!("\n📊 Resultados encontrados: {}", results.len());

    // PASO 4: Procesar resultados
    // ──────────────────────────
    for (i, result) in results.iter().enumerate().take(5) {
        println!("\n[{}] {}", i + 1, "━".repeat(60));
        println!("🌐 URL: {}", result.url);
        println!("📝 Título: {}", result.title);
        println!("📄 Snippet: {}", result.snippet);
        println!("🏢 Fuente: {}", result.source);
        println!("⭐ Relevancia: {:.2}%", result.relevance_score * 100.0);
        
        if result.is_premium {
            println!("💎 Contenido Premium");
        }
        
        if !result.extracted_content.is_empty() {
            println!("📋 Contenido: {} caracteres", result.extracted_content.len());
        }
        
        if result.vectorized_score > 0.0 {
            println!("🎯 Score ML: {:.4}", result.vectorized_score);
        }
    }

    // PASO 5: Obtener estadísticas
    // ───────────────────────────
    let stats = tool.get_stats().await;
    println!("\n📈 ESTADISTICAS:");
    println!("{}", serde_json::to_string_pretty(&stats)?);

    // RESULTADO ESPERADO:
    // ──────────────────
    // ✅ 50+ resultados de búsqueda web
    // ✅ Múltiples fuentes (Google, DuckDuckGo, Bing, etc.)
    // ✅ URLs extraídas y contenido descargado (si enabled)
    // ✅ Puntuaciones de relevancia ML (si enabled)
    // ✅ Resultados almacenados en disco (si enabled)
    // ✅ Índice actualizado para búsquedas futuras
    // ✅ Caché poblado para instant hits

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════

// EJEMPLO AVANZADO: Con opciones personalizadas
// ────────────────────────────────────────────

pub async fn advanced_search_example() -> Result<()> {
    // Búsqueda completa con todas las opciones
    let config = WebSearchCompleteConfig {
        query: "machine learning frameworks comparison".to_string(),
        max_results: 100,
        use_deep_web: true,              // ← BUSCAR EN TOR/I2P
        vectorize_results: true,         // ← APLICAR ML SCORING
        use_parallel: true,              // ← DESCARGAR EN PARALELO
        include_extracted_content: true, // ← OBTENER CONTENIDO COMPLETO
        store_results: true,             // ← GUARDAR RESULTADOS
        enable_caching: true,            // ← CACHEAR EN MEMORIA
        rerank_results: true,            // ← REORDENAR CON ML
        timeout_seconds: 30,             // ← TIMEOUT MÁS LARGO
    };

    let tool = WebSearchCompleteTool::new(config.clone()).await?;
    let results = tool.search(config).await?;

    // Filtrar resultados por relevancia
    let high_quality: Vec<_> = results
        .iter()
        .filter(|r| r.relevance_score >= 0.7)
        .collect();

    println!("🎯 Resultados de alta calidad: {}/{}", 
        high_quality.len(), 
        results.len()
    );

    // Procesar por fuente
    use std::collections::HashMap;
    let mut by_source: HashMap<String, usize> = HashMap::new();
    for result in &results {
        *by_source.entry(result.source.clone()).or_insert(0) += 1;
    }

    println!("\n📊 Distribución por fuente:");
    for (source, count) in by_source.iter() {
        println!("  {}: {} resultados", source, count);
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════

// EJEMPLO SIMPLE: Configuración mínima
// ──────────────────────────────────────

pub async fn simple_search() -> Result<()> {
    // Usar valores por defecto
    let config = WebSearchCompleteConfig {
        query: "python tutorials".to_string(),
        max_results: 20,
        ..Default::default()
    };

    let tool = WebSearchCompleteTool::new(config.clone()).await?;
    let results = tool.search(config).await?;

    for result in results {
        println!("{}: {}", result.source, result.title);
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════

// ESTRUCTURA DE RESULTADO: CompleteSearchResult
// ──────────────────────────────────────────────

/*
pub struct CompleteSearchResult {
    pub url: String,                    // URL del resultado
    pub title: String,                  // Título de la página
    pub snippet: String,                // Fragmento de búsqueda
    pub source: String,                 // Fuente (Google, DuckDuckGo, etc.)
    pub relevance_score: f64,           // Score 0.0-1.0
    pub is_premium: bool,               // ¿Es contenido premium?
    pub extracted_content: String,      // Contenido descargado (si enabled)
    pub vectorized_score: f64,          // Score ML (si enabled)
    pub deep_web: bool,                 // ¿Viene de deep web?
    pub timestamp: DateTime<Utc>,       // Cuándo se obtuvo
    pub metadata: serde_json::Value,    // Metadatos adicionales
}
*/

// ═══════════════════════════════════════════════════════════════════

// ESTADISTICAS: get_stats()
// ─────────────────────────

/*
{
  "total_queries": 42,
  "cache_hits": 10,
  "cache_misses": 32,
  "avg_response_time_ms": 1250.5,
  "total_results": 3500,
  "deep_web_enabled": true,
  "storage_size_mb": 45.2,
  "index_entries": 2850
}
*/

// ═══════════════════════════════════════════════════════════════════
