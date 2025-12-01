use std::path::PathBuf;
use std::sync::Arc;
use nuclear_crawler_lib::scan_project::ProjectScanner;
use nuclear_crawler_lib::web_search::WebSearch;
use nuclear_crawler_lib::scan_project::ScanResult;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 NUCLEAR CRAWLER HYBRID - ANALIZADOR ULTRA-AVANZADO DE CHAPEL");
    println!("=================================================================");
    println!("🎯 Aprovechando el 200% del potencial de Chapel en HPC");
    println!();

    // Parsear argumentos de línea de comandos
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("📖 USO: {} <ruta_del_proyecto>", args[0]);
        println!();
        println!("Ejemplos:");
        println!("  {} .", args[0]);
        println!("  {} /ruta/a/proyecto/chapel", args[0]);
        println!("  {} C:\\Users\\miusuario\\proyectos\\chapel", args[0]);
        println!();
        println!("💡 El analizador detectará automáticamente archivos .chpl");
        return Ok(());
    }

    let project_path = PathBuf::from(&args[1]);

    if !project_path.exists() {
        println!("❌ Error: La ruta '{}' no existe", project_path.display());
        return Ok(());
    }

    if !project_path.is_dir() {
        println!("❌ Error: '{}' no es un directorio", project_path.display());
        return Ok(());
    }

    println!("🔍 Analizando proyecto: {}", project_path.display());
    println!();

    // Crear scanner
    let web_search = Arc::new(WebSearch::new_with_storage(None)?);
    let scanner = ProjectScanner::new(web_search);

    // Ejecutar análisis
    match scanner.scan_chapel_project(project_path).await {
        Ok(analysis) => {
            // Mostrar resultados
            display_analysis_results(&analysis);
        }
        Err(e) => {
            println!("❌ Error durante el análisis: {}", e);
            println!();
            println!("💡 Asegúrate de que:");
            println!("   • La ruta del proyecto es correcta");
            println!("   • Hay archivos .chpl en el proyecto");
            println!("   • El compilador de Chapel (chpl) está instalado");
        }
    }

    Ok(())
}

fn display_analysis_results(analysis: &ScanResult) {
    println!("🎯 RESULTADOS DEL ANÁLISIS ULTRA-AVANZADO");
    println!("========================================");

    // Estadísticas generales
    println!("📊 ESTADÍSTICAS GENERALES:");
    println!("   📁 Lenguaje: {:?}", analysis.language);
    println!("   📄 Archivos analizados: {}", analysis.security_analysis.files_analyzed);
    println!("   ❌ Errores encontrados: {}", analysis.errors.len());
    println!("   ⚠️  Warnings encontrados: {}", analysis.warnings.len());
    println!("   🛡️  Vulnerabilidades de seguridad: {}", analysis.security_analysis.vulnerabilities.len());
    println!("   💡 Recomendaciones: {}", analysis.recommendations.len());
    println!("   📈 Quality Score: {:.1}/100", analysis.quality_score);
    println!("   🔒 Security Score: {:.1}/100", analysis.security_analysis.security_score);
    println!();

    // Errores críticos
    if !analysis.errors.is_empty() {
        println!("🚨 ERRORES CRÍTICOS (DEBEN CORREGIRSE):");
        for error in &analysis.errors {
            println!("   ❌ {}:{}", error.file, error.line);
            println!("      {}", error.message);
            if let Some(code) = &error.code {
                println!("      Código: {}", code);
            }
            if !error.solutions.is_empty() {
                println!("      💡 Solución sugerida: {}", error.solutions[0].title);
            }
            println!();
        }
    }

    // Warnings importantes
    if !analysis.warnings.is_empty() {
        println!("⚠️  WARNINGS IMPORTANTES:");
        for warning in analysis.warnings.iter().take(10) {
            println!("   ⚠️  {}:{}", warning.file, warning.line);
            println!("      {}", warning.message);
            if !warning.solutions.is_empty() {
                println!("      💡 Sugerencia: {}", warning.solutions[0].title);
            }
            println!();
        }
        if analysis.warnings.len() > 10 {
            println!("   ... y {} warnings más", analysis.warnings.len() - 10);
        }
    }

    // Vulnerabilidades de seguridad
    if !analysis.security_analysis.vulnerabilities.is_empty() {
        println!("🔒 VULNERABILIDADES DE SEGURIDAD:");
        for vuln in &analysis.security_analysis.vulnerabilities {
            println!("   🚨 {} (Severidad: {})", vuln.vulnerability_type, vuln.severity);
            println!("      Archivo: {}:{}", vuln.file, vuln.line);
            println!("      {}", vuln.description);
            if !vuln.recommendations.is_empty() {
                println!("      🛡️  Recomendación: {}", vuln.recommendations[0]);
            }
            println!();
        }
    }

    // Recomendaciones
    if !analysis.recommendations.is_empty() {
        println!("💡 RECOMENDACIONES PARA MEJORAR EL CÓDIGO:");
        for rec in &analysis.recommendations {
            println!("   💡 {}", rec);
        }
        println!();
    }

    // Interpretación del Quality Score
    println!("📊 INTERPRETACIÓN DEL QUALITY SCORE:");
    match analysis.quality_score as u32 {
        90..=100 => println!("   🏆 ¡EXCELENTE! Código Chapel de calidad profesional"),
        75..=89 => println!("   ✅ BUENO: Código sólido con algunas áreas de mejora"),
        60..=74 => println!("   ⚠️  REGULAR: Requiere atención en varias áreas"),
        40..=59 => println!("   ❌ DEFICIENTE: Múltiples problemas que afectan el rendimiento"),
        _ => println!("   🚨 CRÍTICO: Revisión completa necesaria"),
    }
    println!();

    // Resumen final
    if analysis.errors.is_empty() && analysis.warnings.is_empty() &&
       analysis.security_analysis.vulnerabilities.is_empty() {
        println!("🎉 ¡FELICITACIONES! Tu código Chapel está optimizado al 200%");
        println!("   Has aprovechado completamente el potencial de Chapel en HPC");
    } else {
        println!("🔧 RESUMEN DE MEJORAS NECESARIAS:");
        if !analysis.errors.is_empty() {
            println!("   • Corregir {} errores críticos", analysis.errors.len());
        }
        if !analysis.warnings.is_empty() {
            println!("   • Revisar {} warnings de optimización", analysis.warnings.len());
        }
        if !analysis.security_analysis.vulnerabilities.is_empty() {
            println!("   • Abordar {} vulnerabilidades de seguridad", analysis.security_analysis.vulnerabilities.len());
        }
        println!("   • Implementar las recomendaciones proporcionadas");
    }

    println!();
    println!("📚 RECURSOS ADICIONALES:");
    println!("   🏛️  Documentación oficial: https://chapel-lang.org/docs/");
    println!("   🎯 Guía de HPC: https://chapel-lang.org/docs/language/spec/");
    println!("   🚀 Optimizaciones avanzadas: https://chapel-lang.org/docs/technotes/");
    println!();
    println!("✨ Análisis completado por Nuclear Crawler Hybrid - El futuro del análisis de código HPC");
}