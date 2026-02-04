/// 🧠 Chapel AI Parallel Orchestrator Demo
/// Ejecuta todos los MCP tools EN PARALELO
/// Chapel AI aprende de todas las operaciones simultáneamente
use nuclear_crawler_hybrid::chapel_parallel::ChapelAIOrchestrator;

#[tokio::main]
async fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║ 🧠 CHAPEL AI - PARALLEL ORCHESTRATOR                      ║");
    println!("║ Ejecutando 5 MCP Tools en paralelo con aprendizaje        ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let orchestrator = ChapelAIOrchestrator::new();

    // 🚀 EJECUTAR TODO EN PARALELO
    println!("🚀 Iniciando ejecución paralela de 5 MCP Tools...\n");
    let start = std::time::Instant::now();

    let results = orchestrator.run_tools_parallel().await;

    let elapsed = start.elapsed();

    // 📊 Mostrar resultados
    println!(
        "📊 RESULTADOS (Ejecutados en paralelo {:.2}s):\n",
        elapsed.as_secs_f64()
    );
    for result in &results {
        println!(
            "  ✓ {} - {}ms (Quality: {:.1}%) - {}",
            result.tool,
            result.duration_ms,
            result.quality * 100.0,
            result.output
        );
    }

    // 🧠 Mostrar aprendizaje
    println!("\n{}", orchestrator.learning_report().await);

    // 📈 Estadísticas
    let total_original = results.iter().map(|r| r.duration_ms).sum::<u64>();
    println!("\n📈 ANÁLISIS DE PARALELISMO:");
    println!("  Duración secuencial estimada: {}ms", total_original);
    println!("  Duración paralela real: {:.0}ms", elapsed.as_millis());
    println!(
        "  Ganancia de paralelismo: {:.1}x más rápido",
        total_original as f64 / elapsed.as_millis() as f64
    );

    println!("\n✅ Chapel AI está aprendiendo continuamente en paralelo");
}
