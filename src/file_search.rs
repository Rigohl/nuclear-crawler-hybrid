//! Módulo File Search - Búsqueda en archivos locales con detección de errores
//!
//! Busca palabras exactas, errores de compilación, warnings, y cualquier cosa en archivos

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Configuración de búsqueda en archivos ULTRA-AVANZADA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSearchConfig {
    /// Término a buscar
    pub search_term: String,

    /// Directorio raíz
    pub root_dir: PathBuf,

    /// Patrones de archivos a incluir
    pub include_patterns: Vec<String>,

    /// Patrones de archivos a excluir
    pub exclude_patterns: Vec<String>,

    /// Búsqueda exacta (case sensitive)
    pub exact_match: bool,

    /// Usar regex
    pub use_regex: bool,

    /// Buscar en contenido
    pub search_in_content: bool,

    /// Buscar en nombres de archivo
    pub search_in_filename: bool,

    /// Máximo de resultados
    pub max_results: usize,

    /// Detectar errores y warnings de compilación
    pub detect_errors: bool,

    /// Analizar con cargo check
    pub use_cargo_check: bool,

    /// === BÚSQUEDA AVANZADA ===
    /// Búsqueda semántica (buscar conceptos relacionados)
    pub semantic_search: bool,

    /// Análisis de contexto (líneas antes/después)
    pub context_analysis: bool,

    /// Detección de patrones avanzados
    pub pattern_detection: bool,

    /// Búsqueda fuzzy (aproximada)
    pub fuzzy_search: bool,

    /// Análisis de dependencias
    pub dependency_analysis: bool,

    /// Detectar imports circulares
    pub detect_circular_imports: bool,

    /// Análisis de complejidad de funciones
    pub analyze_function_complexity: bool,

    /// Buscar código duplicado
    pub detect_code_duplication: bool,

    /// Profundidad de contexto (líneas antes/después)
    pub context_depth: usize,

    /// Umbral de similitud para búsqueda fuzzy (0.0-1.0)
    pub fuzzy_threshold: f32,

    /// Mínimo de líneas para detectar duplicación
    pub duplication_min_lines: usize,
}

impl Default for FileSearchConfig {
    fn default() -> Self {
        Self {
            search_term: String::new(),
            root_dir: PathBuf::from("."),
            include_patterns: vec!["*".to_string()],
            exclude_patterns: vec![
                "target/".to_string(),
                "node_modules/".to_string(),
                ".git/".to_string(),
                "*.lock".to_string(),
            ],
            exact_match: false,
            use_regex: false,
            search_in_content: true,
            search_in_filename: true,
            max_results: 1000,
            detect_errors: true,
            use_cargo_check: true,
            // Búsqueda avanzada
            semantic_search: false,
            context_analysis: true,
            pattern_detection: false,
            fuzzy_search: false,
            dependency_analysis: false,
            detect_circular_imports: false,
            analyze_function_complexity: false,
            detect_code_duplication: false,
            context_depth: 3,
            fuzzy_threshold: 0.8,
            duplication_min_lines: 5,
        }
    }
}

/// Resultado de búsqueda en archivos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSearchResult {
    /// Ruta del archivo
    pub file_path: String,

    /// Línea donde se encontró (si aplica)
    pub line_number: Option<usize>,

    /// Contenido de la línea
    pub line_content: String,

    /// Número de coincidencias
    pub match_count: usize,

    /// Tipo de coincidencia
    pub match_type: String, // "filename", "content", "error", "warning", "dead_code"

    /// Severity (para errores/warnings): "error", "warning", "info"
    pub severity: Option<String>,
}

/// Sistema de búsqueda en archivos
pub struct FileSearch;

impl FileSearch {
    /// Crea nuevo buscador de archivos
    pub fn new() -> Self {
        Self
    }

    /// Busca en archivos (versión async)
    pub async fn search(&self, config: FileSearchConfig) -> Result<Vec<FileSearchResult>> {
        Self::search_sync(config)
    }

    /// Busca en archivos (versión sync) ULTRA-AVANZADA
    pub fn search_sync(config: FileSearchConfig) -> Result<Vec<FileSearchResult>> {
        println!("🔍 BÚSQUEDA ULTRA-AVANZADA EN ARCHIVOS");
        println!("   Término: {}", config.search_term);
        println!("   Directorio: {}", config.root_dir.display());
        println!("   Detectar errores: {}", config.detect_errors);
        println!("   Cargo check: {}", config.use_cargo_check);
        println!("   Búsqueda semántica: {}", config.semantic_search);
        println!("   Análisis de contexto: {}", config.context_analysis);
        println!("   Detección de patrones: {}", config.pattern_detection);
        println!("   Búsqueda fuzzy: {}", config.fuzzy_search);

        let mut results = Vec::new();

        // Si se activa detección de errores, ejecutar cargo check
        if config.detect_errors && config.use_cargo_check {
            if let Ok(cargo_results) = Self::detect_cargo_errors(&config.root_dir) {
                results.extend(cargo_results);
            }
        }

        // Detectar imports circulares si está activado
        if config.detect_circular_imports {
            if let Ok(circular_results) = Self::detect_circular_imports(&config.root_dir) {
                results.extend(circular_results);
            }
        }

        // Detectar duplicación de código si está activado
        if config.detect_code_duplication {
            if let Ok(duplication_results) =
                Self::detect_code_duplication(&config.root_dir, config.duplication_min_lines)
            {
                results.extend(duplication_results);
            }
        }

        // Compilar regex si es necesario
        let pattern = if config.use_regex {
            Some(Regex::new(&config.search_term)?)
        } else {
            None
        };

        // Recorrer archivos en paralelo
        let files = Self::collect_files(&config)?;
        println!("   📋 Archivos a buscar: {}", files.len());

        let search_results: Vec<Vec<FileSearchResult>> = files
            .iter()
            .filter_map(|file_path| {
                Self::search_in_file_advanced(file_path, &config, pattern.as_ref()).ok()
            })
            .collect();

        // Aplanar resultados
        for file_results in search_results {
            results.extend(file_results);
        }

        // Aplicar búsqueda semántica si está activada
        if config.semantic_search {
            let semantic_results = Self::perform_semantic_search(&config, &results);
            results.extend(semantic_results);
        }

        // Aplicar búsqueda fuzzy si está activada
        if config.fuzzy_search {
            let fuzzy_results = Self::perform_fuzzy_search(&config, &files);
            results.extend(fuzzy_results);
        }

        // Ordenar por severidad y número de coincidencias
        results.sort_by(|a, b| {
            let severity_order = |s: &Option<String>| match s.as_deref() {
                Some("error") => 0,
                Some("warning") => 1,
                _ => 2,
            };
            severity_order(&a.severity)
                .cmp(&severity_order(&b.severity))
                .then(b.match_count.cmp(&a.match_count))
        });

        // Limitar resultados
        results.truncate(config.max_results);

        println!("   ✅ Resultados encontrados: {}", results.len());

        Ok(results)
    }

    /// Detecta errores usando CARGO CHECK REAL + análisis de patrones
    fn detect_cargo_errors(root_dir: &Path) -> Result<Vec<FileSearchResult>> {
        let mut results = Vec::new();

        // 🔥 EJECUTAR CARGO CHECK REAL
        println!("   🔍 Ejecutando cargo check para detectar errores reales...");

        if root_dir.join("Cargo.toml").exists() {
            match std::process::Command::new("cargo")
                .arg("check")
                .arg("--message-format=json")
                .current_dir(root_dir)
                .output()
            {
                Ok(output) => {
                    let _stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);

                    // Parse JSON output from cargo check
                    for line in stdout.lines() {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                            if json["reason"] == "compiler-message" {
                                if let Some(message) = json["message"].as_object() {
                                    let severity = message["level"].as_str().unwrap_or("info");
                                    let msg_text =
                                        message["message"].as_str().unwrap_or("Unknown error");

                                    // Extract file path and line number
                                    if let Some(spans) = message["spans"].as_array() {
                                        for span in spans {
                                            if let Some(file_name) = span["file_name"].as_str() {
                                                let line_start =
                                                    span["line_start"].as_u64().unwrap_or(0)
                                                        as usize;
                                                let line_end =
                                                    span["line_end"].as_u64().unwrap_or(0) as usize;

                                                // Read the actual source code lines
                                                let mut context_lines = Vec::new();
                                                if let Ok(file_content) =
                                                    fs::read_to_string(root_dir.join(file_name))
                                                {
                                                    let lines: Vec<&str> =
                                                        file_content.lines().collect();
                                                    let start = line_start.saturating_sub(4);
                                                    let end = (line_end + 3).min(lines.len());

                                                    for (i, line) in
                                                        lines[start..end].iter().enumerate()
                                                    {
                                                        let actual_line_num = start + i + 1;
                                                        let marker = if actual_line_num
                                                            >= line_start
                                                            && actual_line_num <= line_end
                                                        {
                                                            ">>> "
                                                        } else {
                                                            "    "
                                                        };
                                                        context_lines.push(format!(
                                                            "{}{}: {}",
                                                            marker, actual_line_num, line
                                                        ));
                                                    }
                                                }

                                                let full_message = if !context_lines.is_empty() {
                                                    format!(
                                                        "{}\n\n{}",
                                                        msg_text,
                                                        context_lines.join("\n")
                                                    )
                                                } else {
                                                    msg_text.to_string()
                                                };

                                                results.push(FileSearchResult {
                                                    file_path: file_name.to_string(),
                                                    line_number: Some(line_start),
                                                    line_content: full_message,
                                                    match_count: 1,
                                                    match_type: format!("cargo_{}", severity),
                                                    severity: Some(severity.to_string()),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    println!(
                        "   ✅ Cargo check completado: {} errores/warnings encontrados",
                        results.len()
                    );
                }
                Err(e) => {
                    println!("   ⚠️  Cargo check no disponible: {}", e);
                }
            }
        }

        // Patrones de error comunes en Rust
        let error_patterns = vec![
            (
                r"\.unwrap\(\)",
                "error",
                "unwrap() usage - should handle Result properly",
            ),
            (
                r"\.expect\([^)]+\)",
                "error",
                "expect() usage - should handle Result properly",
            ),
            (
                r"panic!\([^)]+\)",
                "error",
                "panic! usage - should return Result instead",
            ),
            (
                r"todo!\([^)]*\)",
                "warning",
                "TODO comment - incomplete implementation",
            ),
            (
                r"unreachable!\([^)]*\)",
                "warning",
                "unreachable! usage - review logic",
            ),
            (
                r"#\[allow\(dead_code\)\]",
                "warning",
                "dead_code allow - remove unused code",
            ),
            (
                r"mock|Mock|simul|Simul",
                "warning",
                "possible mock/simulation code - verify real implementation",
            ),
            (
                r"unsafe\s*\{",
                "warning",
                "unsafe block - ensure safety guarantees",
            ),
            (
                r"refcell|RefCell",
                "info",
                "RefCell usage - consider if interior mutability is needed",
            ),
            (
                r"rc::Rc|Arc<",
                "info",
                "shared ownership - verify if needed",
            ),
        ];

        // Recolectar archivos Rust
        let files = Self::collect_files(&FileSearchConfig {
            search_term: String::new(),
            root_dir: root_dir.to_path_buf(),
            include_patterns: vec!["*.rs".to_string()],
            exclude_patterns: vec![
                "target/".to_string(),
                ".git/".to_string(),
                "node_modules/".to_string(),
            ],
            exact_match: false,
            use_regex: false,
            search_in_content: true,
            search_in_filename: false,
            max_results: usize::MAX,
            detect_errors: false,
            use_cargo_check: false,
            semantic_search: false,
            context_analysis: false,
            pattern_detection: false,
            fuzzy_search: false,
            dependency_analysis: false,
            detect_circular_imports: false,
            analyze_function_complexity: false,
            detect_code_duplication: false,
            context_depth: 0,
            fuzzy_threshold: 0.0,
            duplication_min_lines: 0,
        })?;

        for file_path in files {
            if let Ok(content) = fs::read_to_string(&file_path) {
                let lines: Vec<&str> = content.lines().collect();

                for (line_num, line) in lines.iter().enumerate() {
                    for (pattern, severity, description) in &error_patterns {
                        let re = Regex::new(pattern).unwrap();
                        if re.is_match(line) {
                            // Obtener contexto (3 líneas antes y después)
                            let start = line_num.saturating_sub(3);
                            let end = (line_num + 4).min(lines.len());
                            let context_lines: Vec<String> = lines[start..end]
                                .iter()
                                .enumerate()
                                .map(|(i, l)| {
                                    let marker = if start + i == line_num {
                                        ">>> "
                                    } else {
                                        "    "
                                    };
                                    format!("{}{}: {}", marker, start + i + 1, l)
                                })
                                .collect();

                            results.push(FileSearchResult {
                                file_path: file_path.to_string_lossy().to_string(),
                                line_number: Some(line_num + 1),
                                line_content: format!(
                                    "{} - {}\n{}",
                                    description,
                                    line.trim(),
                                    context_lines.join("\n")
                                ),
                                match_count: 1,
                                match_type: "error".to_string(),
                                severity: Some(severity.to_string()),
                            });
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Recolecta archivos a buscar
    fn collect_files(config: &FileSearchConfig) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        Self::walk_directory(&config.root_dir, config, &mut files)?;

        Ok(files)
    }

    /// Recorre directorio recursivamente
    fn walk_directory(
        dir: &Path,
        config: &FileSearchConfig,
        files: &mut Vec<PathBuf>,
    ) -> Result<()> {
        if !dir.exists() || !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // Verificar si está excluido
            let path_str = path.to_string_lossy();
            if config
                .exclude_patterns
                .iter()
                .any(|pattern| path_str.contains(pattern.trim_end_matches('/')))
            {
                continue;
            }

            if path.is_dir() {
                Self::walk_directory(&path, config, files)?;
            } else if path.is_file() {
                // Verificar patrones de inclusión
                if config
                    .include_patterns
                    .iter()
                    .any(|pattern| Self::matches_pattern(&path_str, pattern))
                {
                    files.push(path);
                }
            }
        }

        Ok(())
    }

    /// Verifica si path coincide con patrón
    fn matches_pattern(path: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }

        // Simple wildcard handling: '*' at start/end
        if pattern.starts_with('*') && pattern.ends_with('*') && pattern.len() >= 2 {
            let pat = &pattern[1..pattern.len() - 1];
            return path.contains(pat);
        }
        if let Some(pat) = pattern.strip_prefix('*') {
            return path.ends_with(pat);
        }
        if let Some(pat) = pattern.strip_suffix('*') {
            return path.starts_with(pat);
        }

        path.contains(pattern)
    }

    /// Busca en un archivo con capacidades avanzadas
    fn search_in_file_advanced(
        file_path: &Path,
        config: &FileSearchConfig,
        pattern: Option<&Regex>,
    ) -> Result<Vec<FileSearchResult>> {
        let mut results = Vec::new();
        let file_str = file_path.to_string_lossy();

        // Buscar en nombre de archivo
        if config.search_in_filename {
            let matches = if let Some(re) = pattern {
                re.find_iter(&file_str).count()
            } else {
                let search_lower = if config.exact_match {
                    config.search_term.clone()
                } else {
                    config.search_term.to_lowercase()
                };

                let file_lower = if config.exact_match {
                    file_str.to_string()
                } else {
                    file_str.to_lowercase()
                };

                if file_lower.contains(&search_lower) {
                    1
                } else {
                    0
                }
            };

            if matches > 0 {
                results.push(FileSearchResult {
                    file_path: file_path.to_string_lossy().to_string(),
                    line_number: None,
                    line_content: String::new(),
                    match_count: matches,
                    match_type: "filename".to_string(),
                    severity: None,
                });
            }
        }

        // Buscar en contenido con análisis de contexto
        if config.search_in_content {
            match fs::read_to_string(file_path) {
                Ok(content) => {
                    let lines: Vec<&str> = content.lines().collect();

                    for (line_num, line) in lines.iter().enumerate() {
                        let matches = if let Some(re) = pattern {
                            re.find_iter(line).count()
                        } else {
                            let search_lower = if config.exact_match {
                                config.search_term.clone()
                            } else {
                                config.search_term.to_lowercase()
                            };

                            let line_lower = if config.exact_match {
                                line.to_string()
                            } else {
                                line.to_lowercase()
                            };

                            if line_lower.contains(&search_lower) {
                                1
                            } else {
                                0
                            }
                        };

                        if matches > 0 {
                            let mut line_content = line.trim().to_string();

                            // Agregar contexto si está activado
                            if config.context_analysis && config.context_depth > 0 {
                                let start = line_num.saturating_sub(config.context_depth);
                                let end = (line_num + config.context_depth + 1).min(lines.len());

                                let context_lines: Vec<String> = lines[start..end]
                                    .iter()
                                    .enumerate()
                                    .map(|(i, l)| {
                                        let line_marker = if start + i == line_num {
                                            ">>> "
                                        } else {
                                            "    "
                                        };
                                        format!("{}{}: {}", line_marker, start + i + 1, l)
                                    })
                                    .collect();

                                line_content = context_lines.join("\n");
                            }

                            results.push(FileSearchResult {
                                file_path: file_path.to_string_lossy().to_string(),
                                line_number: Some(line_num + 1),
                                line_content,
                                match_count: matches,
                                match_type: "content".to_string(),
                                severity: None,
                            });
                            // Detect common risky patterns and provide actionable suggestions
                            let mut suggestions: Vec<String> = Vec::new();
                            let low = line.to_lowercase();
                            if line.contains(".unwrap(") || line.contains(".unwrap()") {
                                suggestions.push(
                                    "avoid .unwrap(): return Result or handle the error instead"
                                        .to_string(),
                                );
                            }
                            if line.contains(".expect(") {
                                suggestions.push("avoid .expect(): provide descriptive message or handle the error".to_string());
                            }
                            if line.contains("panic!") {
                                suggestions.push("avoid panic! in library code; return Result or propagate error".to_string());
                            }
                            if low.contains("mock")
                                || low.contains("simul")
                                || low.contains("simulate")
                            {
                                suggestions.push("possible mock/simulated code: verify it's real and remove mocks".to_string());
                            }
                            if line.contains("todo!") || line.contains("TODO") {
                                suggestions
                                    .push("address TODO comments before production".to_string());
                            }
                            if line.contains("allow(dead_code)")
                                || low.contains("dead_code")
                                || line.contains("unreachable!")
                            {
                                suggestions.push(
                                    "review dead code / unreachable! usage; remove or justify"
                                        .to_string(),
                                );
                            }

                            if !suggestions.is_empty() {
                                results.push(FileSearchResult {
                                    file_path: file_path.to_string_lossy().to_string(),
                                    line_number: Some(line_num + 1),
                                    line_content: format!(
                                        "SUGGESTION: {} -- original: {}",
                                        suggestions.join("; "),
                                        line.trim()
                                    ),
                                    match_count: 0,
                                    match_type: "suggestion".to_string(),
                                    severity: Some("warning".to_string()),
                                });
                            }
                        }
                    }

                    // Análisis de complejidad de funciones si está activado
                    if config.analyze_function_complexity {
                        let complexity_results =
                            Self::analyze_function_complexity(&content, file_path);
                        results.extend(complexity_results);
                    }
                }
                Err(_) => {
                    // Archivo binario o no legible, ignorar
                }
            }
        }

        Ok(results)
    }

    /// Realiza búsqueda semántica
    fn perform_semantic_search(
        config: &FileSearchConfig,
        existing_results: &[FileSearchResult],
    ) -> Vec<FileSearchResult> {
        let mut results = Vec::new();

        // Palabras clave relacionadas con el término de búsqueda
        let semantic_terms = Self::get_semantic_related_terms(&config.search_term);

        for term in semantic_terms {
            // Buscar archivos que contengan términos relacionados
            if let Ok(files) = Self::collect_files(config) {
                for file_path in files {
                    if let Ok(content) = fs::read_to_string(&file_path) {
                        let content_lower = content.to_lowercase();
                        if content_lower.contains(&term.to_lowercase()) {
                            // Verificar que no sea el mismo resultado ya encontrado
                            let already_found = existing_results.iter().any(|r| {
                                r.file_path == file_path.to_string_lossy()
                                    && r.line_content.contains(&config.search_term)
                            });

                            if !already_found {
                                results.push(FileSearchResult {
                                    file_path: file_path.to_string_lossy().to_string(),
                                    line_number: None,
                                    line_content: format!(
                                        "Semantic match: '{}' related to '{}'",
                                        term, config.search_term
                                    ),
                                    match_count: 1,
                                    match_type: "semantic".to_string(),
                                    severity: Some("info".to_string()),
                                });
                            }
                        }
                    }
                }
            }
        }

        results
    }

    /// Obtiene términos semánticamente relacionados
    fn get_semantic_related_terms(term: &str) -> Vec<String> {
        let term_lower = term.to_lowercase();

        // Mapa simple de términos relacionados (se puede expandir)
        let semantic_map = [
            (
                "async",
                vec!["await", "future", "tokio", "spawn", "concurrent"],
            ),
            (
                "error",
                vec!["anyhow", "thiserror", "result", "unwrap", "expect"],
            ),
            ("test", vec!["assert", "should_panic", "bench", "criterion"]),
            (
                "serde",
                vec!["serialize", "deserialize", "json", "toml", "yaml"],
            ),
            ("http", vec!["axum", "reqwest", "hyper", "tower", "client"]),
            (
                "database",
                vec!["sql", "postgres", "mysql", "sqlite", "diesel"],
            ),
            (
                "regex",
                vec!["pattern", "match", "find", "replace", "capture"],
            ),
        ];

        for (key, related) in &semantic_map {
            if term_lower.contains(key) {
                return related.iter().map(|s| s.to_string()).collect();
            }
        }

        // Si no hay términos relacionados específicos, devolver variaciones comunes
        vec![
            format!("{}ing", term),
            format!("{}ed", term),
            format!("{}able", term),
            format!("un{}", term),
        ]
    }

    /// Realiza búsqueda fuzzy (aproximada)
    fn perform_fuzzy_search(config: &FileSearchConfig, files: &[PathBuf]) -> Vec<FileSearchResult> {
        let mut results = Vec::new();

        for file_path in files {
            if let Ok(content) = fs::read_to_string(file_path) {
                let lines: Vec<&str> = content.lines().collect();

                for (line_num, line) in lines.iter().enumerate() {
                    let similarity = Self::calculate_string_similarity(&config.search_term, line);
                    if similarity >= config.fuzzy_threshold {
                        results.push(FileSearchResult {
                            file_path: file_path.to_string_lossy().to_string(),
                            line_number: Some(line_num + 1),
                            line_content: format!(
                                "Fuzzy match ({:.2}): {}",
                                similarity,
                                line.trim()
                            ),
                            match_count: 1,
                            match_type: "fuzzy".to_string(),
                            severity: Some("info".to_string()),
                        });
                    }
                }
            }
        }

        results
    }

    /// Calcula similitud entre dos strings (algoritmo simple de Levenshtein)
    fn calculate_string_similarity(a: &str, b: &str) -> f32 {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();

        if a_chars.is_empty() && b_chars.is_empty() {
            return 1.0;
        }

        let max_len = a_chars.len().max(b_chars.len());
        if max_len == 0 {
            return 1.0;
        }

        // Distancia de Levenshtein simple
        let distance = Self::levenshtein_distance(&a_chars, &b_chars);
        1.0 - (distance as f32 / max_len as f32)
    }

    /// Calcula distancia de Levenshtein
    fn levenshtein_distance(a: &[char], b: &[char]) -> usize {
        let mut matrix = vec![vec![0; b.len() + 1]; a.len() + 1];

        for (i, row) in matrix.iter_mut().enumerate() {
            row[0] = i;
        }
        for (j, _) in (0..=b.len()).enumerate() {
            matrix[0][j] = j;
        }

        for i in 1..=a.len() {
            for j in 1..=b.len() {
                let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[a.len()][b.len()]
    }

    /// Detecta imports circulares
    fn detect_circular_imports(root_dir: &Path) -> Result<Vec<FileSearchResult>> {
        let mut results = Vec::new();

        // Solo para proyectos Rust
        if !root_dir.join("Cargo.toml").exists() {
            return Ok(results);
        }

        // Recolectar módulos
        let mut modules = std::collections::HashMap::new();

        if let Ok(entries) = fs::read_dir(root_dir.join("src")) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".rs") && name != "lib.rs" && name != "main.rs" {
                        let module_name = name.trim_end_matches(".rs");
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            modules.insert(module_name.to_string(), content);
                        }
                    }
                }
            }
        }

        // Verificar imports circulares
        for (module_name, content) in &modules {
            for other_name in modules.keys() {
                if module_name != other_name {
                    let import_pattern = format!("use crate::{};", other_name);
                    if content.contains(&import_pattern) {
                        // Verificar si el otro módulo importa este
                        if let Some(other_content) = modules.get(other_name) {
                            let reverse_import = format!("use crate::{};", module_name);
                            if other_content.contains(&reverse_import) {
                                results.push(FileSearchResult {
                                    file_path: format!("src/{}.rs", module_name),
                                    line_number: None,
                                    line_content: format!(
                                        "Circular import detected between {} and {}",
                                        module_name, other_name
                                    ),
                                    match_count: 1,
                                    match_type: "circular_import".to_string(),
                                    severity: Some("error".to_string()),
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Detecta duplicación de código
    fn detect_code_duplication(root_dir: &Path, min_lines: usize) -> Result<Vec<FileSearchResult>> {
        let mut results = Vec::new();

        // Recolectar contenido de archivos Rust
        let mut file_contents = Vec::new();

        if let Ok(entries) = fs::read_dir(root_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        file_contents.push((path, content));
                    }
                }
            }
        }

        // Análisis simple de duplicación
        for (i, (path1, content1)) in file_contents.iter().enumerate() {
            let lines1: Vec<&str> = content1.lines().collect();

            for (j, (path2, content2)) in file_contents.iter().enumerate() {
                if i >= j {
                    continue;
                } // Evitar comparación consigo mismo y duplicados

                let lines2: Vec<&str> = content2.lines().collect();
                let mut common_lines = 0;

                // Comparación línea por línea
                for line1 in &lines1 {
                    let trimmed1 = line1.trim();
                    if trimmed1.is_empty() || trimmed1.starts_with("//") {
                        continue;
                    }

                    for line2 in &lines2 {
                        let trimmed2 = line2.trim();
                        if trimmed1 == trimmed2 {
                            common_lines += 1;
                            break;
                        }
                    }
                }

                if common_lines >= min_lines {
                    results.push(FileSearchResult {
                        file_path: format!("{} vs {}", path1.display(), path2.display()),
                        line_number: None,
                        line_content: format!(
                            "Code duplication detected: {} common lines",
                            common_lines
                        ),
                        match_count: common_lines,
                        match_type: "duplication".to_string(),
                        severity: Some("warning".to_string()),
                    });
                }
            }
        }

        Ok(results)
    }

    /// Analiza complejidad de funciones
    fn analyze_function_complexity(content: &str, file_path: &Path) -> Vec<FileSearchResult> {
        let mut results = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut current_function = None;
        let mut function_start = 0;
        let mut brace_count = 0;

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Detectar inicio de función
            if trimmed.starts_with("fn ") && current_function.is_none() {
                current_function = Some(trimmed.split('(').next().unwrap_or("unknown").to_string());
                function_start = line_num;
                brace_count = 0;
            }

            // Contar llaves
            brace_count += line.matches('{').count() as i32;
            brace_count -= line.matches('}').count() as i32;

            // Fin de función
            if brace_count == 0 && current_function.is_some() {
                let function_lines: Vec<&str> = lines[function_start..=line_num].to_vec();
                let complexity = Self::calculate_function_complexity(&function_lines);

                if complexity > 10 {
                    results.push(FileSearchResult {
                        file_path: file_path.to_string_lossy().to_string(),
                        line_number: Some(function_start + 1),
                        line_content: format!(
                            "High complexity function '{}': {} (lines: {})",
                            current_function
                                .as_ref()
                                .expect("Function name should be set"),
                            complexity,
                            function_lines.len()
                        ),
                        match_count: complexity,
                        match_type: "complexity".to_string(),
                        severity: Some("warning".to_string()),
                    });
                }

                current_function = None;
            }
        }

        results
    }

    /// Calcula complejidad de una función
    fn calculate_function_complexity(function_lines: &[&str]) -> usize {
        let mut complexity = 1; // Base complexity

        for line in function_lines {
            let trimmed = line.trim();

            // Estructuras de control
            if trimmed.contains("if ")
                || trimmed.contains("else if")
                || trimmed.contains("while ")
                || trimmed.contains("for ")
                || trimmed.contains("loop")
                || trimmed.contains("match ")
            {
                complexity += 1;
            }

            // Operadores lógicos
            complexity += trimmed.matches("&&").count();
            complexity += trimmed.matches("||").count();
        }

        complexity
    }
}

impl Default for FileSearch {
    fn default() -> Self {
        Self::new()
    }
}
