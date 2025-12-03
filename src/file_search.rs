//! Módulo File Search - Búsqueda en archivos locales
//!
//! Busca palabras exactas, errores, o cualquier cosa en archivos

use anyhow::Result;
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
#[allow(unused_imports)]
use std::sync::Arc;

/// Configuración de búsqueda en archivos
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
    pub match_type: String, // "filename", "content", "error"
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

    /// Busca en archivos (versión sync)
    pub fn search_sync(config: FileSearchConfig) -> Result<Vec<FileSearchResult>> {
        println!("📁 BÚSQUEDA EN ARCHIVOS");
        println!("   Término: {}", config.search_term);
        println!("   Directorio: {}", config.root_dir.display());
        println!("   Exacto: {}", config.exact_match);
        println!("   Regex: {}", config.use_regex);

        let mut results = Vec::new();

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
            .par_iter()
            .filter_map(|file_path| Self::search_in_file(file_path, &config, pattern.as_ref()).ok())
            .collect();

        // Aplanar resultados
        for file_results in search_results {
            results.extend(file_results);
        }

        // Ordenar por número de coincidencias
        results.sort_by(|a, b| b.match_count.cmp(&a.match_count));

        // Limitar resultados
        results.truncate(config.max_results);

        println!("   ✅ Resultados encontrados: {}", results.len());

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

        // Simple glob matching
        if pattern.contains('*') {
            let regex_pattern = pattern.replace("*", ".*");
            let re = Regex::new(&format!("^{}$", regex_pattern))
                .unwrap_or_else(|_| Regex::new("$^").unwrap());
            re.is_match(path)
        } else {
            path.contains(pattern)
        }
    }

    /// Busca en un archivo
    fn search_in_file(
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
                });
            }
        }

        // Buscar en contenido
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
                            results.push(FileSearchResult {
                                file_path: file_path.to_string_lossy().to_string(),
                                line_number: Some(line_num + 1),
                                line_content: line.trim().to_string(),
                                match_count: matches,
                                match_type: "content".to_string(),
                            });
                        }
                    }
                }
                Err(_) => {
                    // Archivo binario o no legible, ignorar
                }
            }
        }

        Ok(results)
    }
}

impl Default for FileSearch {
    fn default() -> Self {
        Self::new()
    }
}
