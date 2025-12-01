use std::fs;
use std::sync::Arc;
use std::time::Instant;
use rayon::prelude::*;
use regex::Regex;
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use clap::{Arg, Command};

/// Detecta el lenguaje basándose en la extensión del archivo
fn detect_language_from_extension(extension: &str) -> String {
    match extension.to_lowercase().as_str() {
        "rs" => "rust".to_string(),
        "py" => "python".to_string(),
        "js" | "ts" | "jsx" | "tsx" => "javascript".to_string(),
        "c" | "h" => "c".to_string(),
        "cpp" | "cc" | "cxx" | "hpp" => "cpp".to_string(),
        "go" => "go".to_string(),
        "java" => "java".to_string(),
        "rb" => "ruby".to_string(),
        "php" => "php".to_string(),
        _ => "generic".to_string(),
    }
}

/// Aplica optimizaciones específicas del lenguaje al patrón de búsqueda/reemplazo
fn apply_language_optimizations(search: &str, replace: &str, _language: &str) -> (String, String) {
    // Por ahora, devolver los patrones sin modificar
    // Aquí se podrían añadir optimizaciones específicas por lenguaje
    (search.to_string(), replace.to_string())
}

#[derive(Clone)]
struct ProcessingStats {
    files_processed: usize,
    files_modified: usize,
    total_changes: usize,
    errors: usize,
}

impl ProcessingStats {
    fn new() -> Self {
        Self {
            files_processed: 0,
            files_modified: 0,
            total_changes: 0,
            errors: 0,
        }
    }

    fn merge(&mut self, other: &ProcessingStats) {
        self.files_processed += other.files_processed;
        self.files_modified += other.files_modified;
        self.total_changes += other.total_changes;
        self.errors += other.errors;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 RUST PARALLEL CODE EDITOR - HPC SIMULATION");
    println!("============================================");
    println!("🎯 Simulando el poder de Chapel con Rust + Rayon");
    println!("🚀 Aprovechando paralelismo masivo con Rayon");
    println!();

    let matches = Command::new("Rust Parallel Code Editor")
        .version("1.0")
        .author("Nuclear Crawler Hybrid")
        .about("Edición masiva de código con paralelismo HPC usando Rust")
        .arg(Arg::new("directory")
            .help("Directorio raíz para procesar")
            .required(true)
            .index(1))
        .arg(Arg::new("search")
            .help("Patrón de búsqueda (regex soportado)")
            .required(true)
            .index(2))
        .arg(Arg::new("replace")
            .help("Texto de reemplazo")
            .required(true)
            .index(3))
        .arg(Arg::new("extension")
            .short('e')
            .long("ext")
            .help("Extensión de archivo (ej: rs, py, js, c, h, cpp)")
            .default_value("*"))
        .arg(Arg::new("language")
            .short('l')
            .long("lang")
            .help("Lenguaje específico con optimizaciones (c, cpp, rs, py, js)")
            .default_value("auto"))
        .arg(Arg::new("dry-run")
            .short('d')
            .long("dry-run")
            .help("Solo mostrar cambios, no aplicar")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    let root_dir = matches.get_one::<String>("directory").unwrap();
    let search_pattern = matches.get_one::<String>("search").unwrap();
    let replace_pattern = matches.get_one::<String>("replace").unwrap();
    let extension = matches.get_one::<String>("extension").unwrap();
    let language = matches.get_one::<String>("language").unwrap();
    let dry_run = matches.get_flag("dry-run");

    // Detectar lenguaje automáticamente si es "auto"
    let detected_language = if language == "auto" {
        detect_language_from_extension(extension)
    } else {
        language.clone()
    };

    // Aplicar optimizaciones específicas del lenguaje
    let (final_search, _final_replace) = apply_language_optimizations(
        search_pattern,
        replace_pattern,
        &detected_language
    );

    // Compilar regex y envolver en Arc para uso paralelo
    let regex = Regex::new(&final_search)?;
    let regex_arc = Arc::new(regex);

    println!("📁 Directorio: {}", root_dir);
    println!("🔍 Patrón: {}", search_pattern);
    println!("✏️  Reemplazo: {}", replace_pattern);
    println!("📄 Extensión: {}", extension);
    if dry_run {
        println!("🔍 Modo dry-run: Solo mostrando cambios");
    }
    println!();

    // Fase 1: Recolección de archivos
    println!("📁 Fase 1: Escaneando archivos...");
    let files: Vec<_> = WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            if extension == "*" {
                true
            } else {
                e.path().extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == extension)
                    .unwrap_or(false)
            }
        })
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();

    println!("📊 Encontrados {} archivos para procesar", files.len());
    println!();

    if files.is_empty() {
        println!("ℹ️  No se encontraron archivos para procesar");
        return Ok(());
    }

    // Fase 2: Procesamiento paralelo masivo
    println!("⚡ Fase 2: Procesamiento paralelo con Rayon...");
    let start_time = Instant::now();

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("#>-")
    );

    let pb_arc = Arc::new(pb);

    // Procesamiento paralelo usando Rayon
    let results: Vec<ProcessingStats> = files
        .par_iter()
        .map(|file_path| {
            let result = process_file(file_path, &regex_arc, replace_pattern, dry_run);
            pb_arc.inc(1);
            result
        })
        .collect();

    pb_arc.finish_with_message("Procesamiento completado");

    let end_time = Instant::now();
    let total_time = end_time.duration_since(start_time);

    // Agregar estadísticas
    let mut total_stats = ProcessingStats::new();
    for stat in results {
        total_stats.merge(&stat);
    }

    // Fase 3: Reporte final
    println!();
    println!("🎯 RESULTADOS DEL PROCESAMIENTO PARALELO");
    println!("=======================================");
    println!("⏱️  Tiempo total: {:.2} segundos", total_time.as_secs_f64());
    println!("📁 Archivos procesados: {}", total_stats.files_processed);
    println!("✏️  Archivos modificados: {}", total_stats.files_modified);
    println!("🔄 Cambios totales realizados: {}", total_stats.total_changes);
    println!("❌ Archivos con errores: {}", total_stats.errors);

    let files_per_sec = total_stats.files_processed as f64 / total_time.as_secs_f64();
    println!("⚡ Rendimiento: {:.0} archivos/segundo", files_per_sec);

    let num_cores = num_cpus::get();
    let efficiency = files_per_sec / num_cores as f64;
    println!("🚀 Eficiencia paralela: {:.1} archivos/segundo por core", efficiency);

    println!();
    if total_stats.total_changes > 0 && !dry_run {
        println!("🎉 ¡Procesamiento completado exitosamente!");
        println!("💡 Los cambios han sido aplicados usando paralelismo Rust HPC");
    } else if dry_run {
        println!("🔍 Dry-run completado - Usa sin --dry-run para aplicar cambios");
    } else {
        println!("ℹ️  No se encontraron coincidencias para el patrón especificado");
    }

    Ok(())
}

fn process_file(file_path: &str, regex: &Arc<Regex>, replace_pattern: &str, dry_run: bool) -> ProcessingStats {
    let mut stats = ProcessingStats::new();
    stats.files_processed = 1;

    match fs::read_to_string(file_path) {
        Ok(content) => {
            let new_content = regex.replace_all(&content, replace_pattern);
            let changes = regex.find_iter(&content).count();

            if changes > 0 {
                stats.files_modified = 1;
                stats.total_changes = changes;

                if !dry_run {
                    if let Err(_) = fs::write(file_path, new_content.as_ref()) {
                        stats.errors = 1;
                        stats.files_modified = 0;
                        stats.total_changes = 0;
                    }
                }
            }
        }
        Err(_) => {
            stats.errors = 1;
        }
    }

    stats
}