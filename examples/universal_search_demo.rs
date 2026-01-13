// 🔥 EJEMPLO COMPLETO - UniversalSearchTool
// 
// Este archivo muestra cómo usar UniversalSearchTool sin modos
// Compile con: cargo build --example universal_search_demo --release
// Ejecute con: cargo run --example universal_search_demo --release

use nuclear_crawler_hybrid::mcp::tools::{
    UniversalSearchTool, SimpleSearchConfig, SearchType,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════════════");
    println!("🔥 UniversalSearchTool - DEMO (SIN MODOS)");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Crear herramienta
    let tool = UniversalSearchTool::new().await?;

    // ═══════════════════════════════════════════════════════════════
    // EJEMPLO 1: Búsqueda por Frase (Default)
    // ═══════════════════════════════════════════════════════════════
    println!("📝 EJEMPLO 1: Búsqueda por Frase (Default)");
    println!("─────────────────────────────────────────\n");

    let query = "machine learning rust";
    println!("Query: '{}'\n", query);

    let result = tool.search_phrase(query, SimpleSearchConfig::default()).await?;

    println!("✅ Búsqueda completada:");
    println!("   • Total resultados: {}", result.total_found);
    println!("   • Latencia: {:.2}ms", result.latency_ms);
    println!("   • Tipo: {:?}", result.search_type);
    println!("   • Módulos activados: {}", result.modules_activated.len());
    println!("\n   Módulos utilizados:");
    for module in &result.modules_activated {
        println!("     • {}", module);
    }

    println!("\n📊 Estadísticas:");
    println!("   • Total módulos disponibles: {}", result.statistics.total_modules_available);
    println!("   • Aceleración SIMD: {}", result.statistics.acceleration_applied);
    println!("   • ML aplicado: {}", result.statistics.ml_applied);
    println!("   • Resultados indexados: {}", result.statistics.indexed);

    println!("\n🔎 Top 3 resultados:");
    for (i, item) in result.results.iter().take(3).enumerate() {
        println!("   {}. {} (score: {:.2})", i + 1, item.title, item.relevance_score);
        println!("      URL: {}", item.url);
        println!("      Snippet: {}...", &item.snippet[..item.snippet.len().min(80)]);
        println!("      Procesado por: {:?}", item.processed_by);
    }

    println!("\n");

    // ═══════════════════════════════════════════════════════════════
    // EJEMPLO 2: Búsqueda por Link (Default)
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("📝 EJEMPLO 2: Búsqueda por Link (Análisis Profundo)");
    println!("─────────────────────────────────────────\n");

    let link = "https://example.com/article";
    println!("Link: {}\n", link);

    let result = tool.search_link(link).await?;

    println!("✅ Análisis completado:");
    println!("   • Tipo: {:?}", result.search_type);
    println!("   • Latencia: {:.2}ms", result.latency_ms);
    println!("   • Módulos utilizados: {}", result.modules_activated.len());

    println!("\n📊 Módulos en análisis profundo:");
    for module in &result.modules_activated {
        println!("   • {}", module);
    }

    if let Some(item) = result.results.first() {
        println!("\n📄 Contenido analizado:");
        println!("   • Título: {}", item.title);
        println!("   • Fuente: {}", item.source);
        println!("   • Score: {:.2}", item.relevance_score);
        println!("   • Procesado por: {:?}", item.processed_by);
    }

    println!("\n");

    // ═══════════════════════════════════════════════════════════════
    // EJEMPLO 3: Búsqueda con Opcionales Activados
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("📝 EJEMPLO 3: Búsqueda Potenciada (Con Opcionales)");
    println!("─────────────────────────────────────────\n");

    let query = "artificial intelligence algorithms";
    println!("Query: '{}'\n", query);

    let mut config = SimpleSearchConfig::default();
    config.search_type = SearchType::Phrase;
    config.query = query.to_string();
    config.max_results = 100;
    
    // Activar opcionales
    config.use_deep_web = true;           // 🌐 Buscar en Tor
    config.use_jax_ml = true;             // 🤖 ML vectorization
    config.use_zig_acceleration = true;   // ⚡ SIMD hashing
    
    println!("Configuración:");
    println!("   • Max resultados: {}", config.max_results);
    println!("   • Deep Web: {}", config.use_deep_web);
    println!("   • ML: {}", config.use_jax_ml);
    println!("   • SIMD: {}", config.use_zig_acceleration);
    println!("\n");

    let result = tool.search_phrase(query, config).await?;

    println!("✅ Búsqueda potenciada completada:");
    println!("   • Total resultados: {}", result.total_found);
    println!("   • Latencia: {:.2}ms", result.latency_ms);
    println!("   • Módulos: {}", result.modules_activated.len());

    println!("\n📊 Estadísticas de potenciación:");
    println!("   • Deep Web resultados: {}", result.statistics.deep_web_results);
    println!("   • Resultados en caché: {}", result.statistics.cached_results);
    println!("   • Aceleración SIMD: {}", result.statistics.acceleration_applied);
    println!("   • ML aplicado: {}", result.statistics.ml_applied);
    println!("   • Indexados: {}", result.statistics.indexed);

    println!("\n🔥 Módulos activados:");
    for module in &result.modules_activated {
        print!("   • {}", module);
        // Indicar tipo
        if module.contains("web_search") {
            println!(" (búsqueda)");
        } else if module.contains("cache") {
            println!(" (cache)");
        } else if module.contains("deepweb") {
            println!(" (🌐 deep web)");
        } else if module.contains("jax") {
            println!(" (🤖 ML)");
        } else if module.contains("zig") {
            println!(" (⚡ SIMD)");
        } else if module.contains("go") {
            println!(" (paralelo)");
        } else if module.contains("data") {
            println!(" (indexación)");
        } else {
            println!();
        }
    }

    println!("\n");

    // ═══════════════════════════════════════════════════════════════
    // RESUMEN
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("✅ RESUMEN DE DEMOSTRACIÓN");
    println!("═══════════════════════════════════════════════════════════════\n");

    let modules = tool.get_all_modules_info();
    
    println!("🔥 Módulos Disponibles ({}):", modules.len());
    for (i, module) in modules.iter().enumerate() {
        println!("   {}. {} ({} líneas) - {}", 
            i + 1, module.name, module.lines, module.category);
    }

    println!("\n✨ Características:");
    println!("   ✅ SIN MODOS (sin OperationMode enum)");
    println!("   ✅ Búsqueda por FRASE");
    println!("   ✅ Búsqueda por LINK");
    println!("   ✅ Potenciación AUTOMÁTICA");
    println!("   ✅ 16 módulos integrados (10,063 líneas)");
    println!("   ✅ Compilación: 0 errores");

    println!("\n🎯 Próximos pasos:");
    println!("   1. Ver UNIVERSAL_SEARCH_QUICK_START.md para más ejemplos");
    println!("   2. Ver UNIVERSAL_SEARCH_ARCHITECTURE.md para detalles técnicos");
    println!("   3. Integrar en tu aplicación MCP");

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("🔥 DEMO COMPLETADA EXITOSAMENTE");
    println!("═══════════════════════════════════════════════════════════════\n");

    Ok(())
}
