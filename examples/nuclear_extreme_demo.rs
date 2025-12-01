//! 🔥💥 Ejemplo de uso: Nuclear Extreme Crawler
//!
//! Demuestra el poder extremo del crawler:
//! - Crawlear 1,000,000+ URLs
//! - 10,000 requests simultáneos
//! - Go + Zig + Rust trabajando juntos
//! - Stealth total y anti-detección

use nuclear_scraper_web::nuclear_extreme_crawler::{
    NuclearExtremeCrawler, NuclearExtremeConfig, OutputFormat
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔥💥 NUCLEAR EXTREME CRAWLER - DEMO");
    println!("==================================\n");
    
    // EJEMPLO 1: Velocidad Máxima (sin stealth)
    println!("📊 EJEMPLO 1: VELOCIDAD MÁXIMA");
    ejemplo_velocidad_maxima().await?;
    
    println!("\n");
    
    // EJEMPLO 2: Stealth Máximo (más lento pero indetectable)
    println!("🥷 EJEMPLO 2: STEALTH MÁXIMO");
    ejemplo_stealth_maximo().await?;
    
    println!("\n");
    
    // EJEMPLO 3: Balanceado (velocidad + stealth)
    println!("⚖️ EJEMPLO 3: MODO BALANCEADO");
    ejemplo_balanceado().await?;
    
    println!("\n");
    
    // EJEMPLO 4: Crawl masivo (1 millón de URLs)
    println!("🌊 EJEMPLO 4: CRAWL MASIVO");
    ejemplo_crawl_masivo().await?;
    
    Ok(())
}

/// EJEMPLO 1: Velocidad Máxima
/// - 50,000 requests simultáneos
/// - Sin delays
/// - Sin stealth
/// - ¡MÁXIMA VELOCIDAD!
async fn ejemplo_velocidad_maxima() -> Result<()> {
    let config = NuclearExtremeConfig::maximum_speed();
    
    println!("⚙️ Configuración:");
    println!("   - Max concurrent: {}", config.max_concurrent_requests);
    println!("   - Go goroutines: {}", config.goroutines_per_batch);
    println!("   - Zig workers: {}", config.zig_simd_workers);
    
    let crawler = NuclearExtremeCrawler::new(config).await?;
    
    // URLs de ejemplo
    let seed_urls = vec![
        "https://example.com".to_string(),
        "https://news.ycombinator.com".to_string(),
        "https://github.com/trending".to_string(),
    ];
    
    println!("\n🚀 Iniciando crawl...");
    let results = crawler.nuclear_crawl(seed_urls).await?;
    
    println!("✅ Completado: {} URLs crawled", results.len());
    
    let stats = crawler.get_stats().await;
    println!("📊 Velocidad: {:.1} URLs/segundo", stats.urls_per_second);
    println!("📦 Datos: {:.2} MB descargados", 
        stats.total_bytes_downloaded as f64 / 1_048_576.0);
    
    Ok(())
}

/// EJEMPLO 2: Stealth Máximo
/// - 100 requests simultáneos
/// - Delays aleatorios
/// - Rotación de User Agents
/// - Rotación de Proxies
/// - ¡INDETECTABLE!
async fn ejemplo_stealth_maximo() -> Result<()> {
    let config = NuclearExtremeConfig::maximum_stealth();
    
    println!("⚙️ Configuración Stealth:");
    println!("   - Max concurrent: {}", config.max_concurrent_requests);
    println!("   - Rotate UA: {}", config.rotate_user_agents);
    println!("   - Rotate proxies: {}", config.rotate_proxies);
    println!("   - Random delays: {}-{}ms", config.min_delay_ms, config.max_delay_ms);
    println!("   - Bypass Cloudflare: {}", config.bypass_cloudflare);
    println!("   - Bypass CAPTCHA: {}", config.bypass_captcha);
    
    let crawler = NuclearExtremeCrawler::new(config).await?;
    
    let seed_urls = vec![
        "https://protected-site.com".to_string(),
        "https://cloudflare-protected.com".to_string(),
    ];
    
    println!("\n🥷 Iniciando crawl stealth...");
    let results = crawler.nuclear_crawl(seed_urls).await?;
    
    println!("✅ Completado sin detección: {} URLs", results.len());
    
    Ok(())
}

/// EJEMPLO 3: Modo Balanceado
/// - 10,000 requests simultáneos
/// - Stealth moderado
/// - Velocidad alta
/// - ¡EL MEJOR DE AMBOS MUNDOS!
async fn ejemplo_balanceado() -> Result<()> {
    let config = NuclearExtremeConfig::balanced();
    
    println!("⚙️ Configuración Balanceada:");
    println!("   - Max concurrent: {}", config.max_concurrent_requests);
    println!("   - Stealth: MODERADO");
    println!("   - Velocidad: ALTA");
    
    let crawler = NuclearExtremeCrawler::new(config).await?;
    
    let seed_urls = vec![
        "https://wikipedia.org".to_string(),
        "https://github.com".to_string(),
        "https://stackoverflow.com".to_string(),
    ];
    
    println!("\n⚖️ Iniciando crawl balanceado...");
    let results = crawler.nuclear_crawl(seed_urls).await?;
    
    println!("✅ Completado: {} URLs crawled", results.len());
    
    let stats = crawler.get_stats().await;
    println!("📊 Stats:");
    println!("   - URLs/segundo: {:.1}", stats.urls_per_second);
    println!("   - Errores: {}", stats.total_errors);
    println!("   - Reintentos: {}", stats.total_retries);
    
    Ok(())
}

/// EJEMPLO 4: Crawl Masivo
/// - 1,000,000+ URLs objetivo
/// - Crawl recursivo profundo
/// - Almacenamiento comprimido
/// - ¡CRAWLING INDUSTRIAL!
async fn ejemplo_crawl_masivo() -> Result<()> {
    let mut config = NuclearExtremeConfig::balanced();
    config.max_urls_total = 1_000_000;
    config.max_depth = 50;
    config.save_to_disk = true;
    config.output_format = OutputFormat::JsonGz;
    config.output_directory = "./massive_crawl_results".to_string();
    config.compress_storage = true;
    config.deduplicate_content = true;
    
    println!("⚙️ Configuración Masiva:");
    println!("   - URLs objetivo: {}", config.max_urls_total);
    println!("   - Profundidad máxima: {}", config.max_depth);
    println!("   - Almacenamiento: {}", config.output_directory);
    println!("   - Formato: {:?}", config.output_format);
    println!("   - Compresión: {}", config.compress_storage);
    println!("   - Deduplicación: {}", config.deduplicate_content);
    
    let crawler = NuclearExtremeCrawler::new(config).await?;
    
    // Seeds iniciales
    let seed_urls = vec![
        "https://en.wikipedia.org/wiki/Web_crawler".to_string(),
    ];
    
    println!("\n🌊 Iniciando crawl MASIVO...");
    println!("⚠️ ADVERTENCIA: Esto puede tomar HORAS y descargar GIGABYTES");
    println!("📊 Progreso se mostrará cada 5 segundos...\n");
    
    let results = crawler.nuclear_crawl(seed_urls).await?;
    
    println!("\n✅ CRAWL MASIVO COMPLETADO!");
    let stats = crawler.get_stats().await;
    println!("📊 Estadísticas Finales:");
    println!("   - URLs totales: {}", stats.total_urls_crawled);
    println!("   - Tiempo total: {} segundos", stats.runtime_seconds);
    println!("   - Velocidad promedio: {:.1} URLs/segundo", stats.urls_per_second);
    println!("   - Datos descargados: {:.2} GB", 
        stats.total_bytes_downloaded as f64 / 1_073_741_824.0);
    println!("   - Velocidad de descarga: {:.2} Mbps", stats.average_speed_mbps);
    println!("   - Errores: {} ({:.2}%)", 
        stats.total_errors, 
        stats.total_errors as f64 / stats.total_urls_crawled as f64 * 100.0
    );
    
    Ok(())
}

/// EJEMPLO BONUS: Crawl personalizado
#[allow(dead_code)]
async fn ejemplo_personalizado() -> Result<()> {
    let mut config = NuclearExtremeConfig::default();
    
    // Personaliza cada aspecto
    config.max_concurrent_requests = 20_000;
    config.goroutines_per_batch = 8_000;
    config.zig_simd_workers = 16;
    config.rust_tokio_workers = 8;
    
    config.rotate_user_agents = true;
    config.rotate_proxies = true;
    config.random_delays = true;
    config.min_delay_ms = 50;
    config.max_delay_ms = 200;
    
    config.bypass_cloudflare = true;
    config.bypass_captcha = true;
    
    config.max_depth = 20;
    config.max_urls_total = 500_000;
    config.follow_external_links = true;
    
    // Filtra dominios
    config.allowed_domains = vec![
        "wikipedia.org".to_string(),
        "github.com".to_string(),
    ];
    
    config.blocked_domains = vec![
        "facebook.com".to_string(),
        "twitter.com".to_string(),
    ];
    
    config.extract_all_data = true;
    config.min_content_quality = 0.7;
    
    let crawler = NuclearExtremeCrawler::new(config).await?;
    
    let seed_urls = vec![
        "https://en.wikipedia.org".to_string(),
    ];
    
    let results = crawler.nuclear_crawl(seed_urls).await?;
    println!("Crawled {} high-quality URLs", results.len());
    
    Ok(())
}
