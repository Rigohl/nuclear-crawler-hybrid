// 🔥 EJEMPLO COMPLETO - WebSearchTool
//
// Este archivo muestra cómo usar WebSearchTool
// Compile con: cargo build --example universal_search_demo --release
// Ejecute con: cargo run --example universal_search_demo --release

use nuclear_crawler_hybrid::mcp::tools::{WebSearchConfig, WebSearchTool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════════════");
    println!("🔥 WebSearchTool - DEMO");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Crear herramienta con configuración
    let config = WebSearchConfig {
        max_results: 10,
        timeout_seconds: 30,
        bypass: true,
    };
    let tool = WebSearchTool::new(config);

    // ═══════════════════════════════════════════════════════════════
    // EJEMPLO 1: Búsqueda básica
    // ═══════════════════════════════════════════════════════════════
    println!("📝 EJEMPLO 1: Búsqueda básica");
    println!("─────────────────────────────────────────\n");

    let query = "machine learning rust";
    println!("Query: '{}'\n", query);

    let results = tool.search(query).await?;

    println!("✅ Búsqueda completada:");
    println!("   • Total resultados: {}", results.len());

    println!("\n🔎 Top 3 resultados:");
    for (i, item) in results.iter().take(3).enumerate() {
        println!(
            "   {}. {} (score: {:.2})",
            i + 1,
            item.title,
            item.relevance_score
        );
        println!("      URL: {}", item.url);
        println!(
            "      Snippet: {}...",
            &item.snippet[..item.snippet.len().min(80)]
        );
    }

    println!("\n");

    // ═══════════════════════════════════════════════════════════════
    // EJEMPLO 2: Búsqueda con más resultados
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("📝 EJEMPLO 2: Búsqueda con más resultados");
    println!("─────────────────────────────────────────\n");

    let config2 = WebSearchConfig {
        max_results: 20,
        timeout_seconds: 60,
        bypass: true,
    };
    let tool2 = WebSearchTool::new(config2);

    let query = "artificial intelligence algorithms";
    println!("Query: '{}'\n", query);

    let results = tool2.search(query).await?;

    println!("✅ Búsqueda completada:");
    println!("   • Total resultados: {}", results.len());

    println!("\n");

    // ═══════════════════════════════════════════════════════════════
    // RESUMEN
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("✅ RESUMEN DE DEMOSTRACIÓN");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("\n✨ Características:");
    println!("   ✅ Búsqueda web real");
    println!("   ✅ Múltiples motores de búsqueda");
    println!("   ✅ Resultados con relevancia");
    println!("   ✅ Compilación: 0 errores");

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("🔥 DEMO COMPLETADA EXITOSAMENTE");
    println!("═══════════════════════════════════════════════════════════════\n");

    Ok(())
}
