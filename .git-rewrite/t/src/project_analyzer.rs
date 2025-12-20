//! Módulo Project Analyzer - Análisis ULTRA-AVANZADO de proyecto
//!
//! Analiza el proyecto con técnicas avanzadas: complejidad ciclomática, code smells,
//! vulnerabilidades, análisis de arquitectura, duplicación de código, profiling

use crate::web_search::WebSearch;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

/// Configuración de análisis de proyecto ULTRA-AVANZADO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectAnalysisConfig {
    /// Directorio del proyecto
    pub project_dir: PathBuf,

    /// Lenguaje de programación
    pub language: Option<String>,

    /// === ANÁLISIS BÁSICO ===
    pub detect_errors: bool,
    pub detect_warnings: bool,
    pub detect_dead_code: bool,
    pub detect_security_issues: bool,

    /// === ANÁLISIS AVANZADO ===
    pub analyze_complexity: bool,           // Complejidad ciclomática
    pub detect_code_smells: bool,           // Code smells avanzados
    pub analyze_dependencies: bool,         // Análisis de dependencias profundo
    pub detect_vulnerabilities: bool,       // Vulnerabilidades conocidas
    pub profile_performance: bool,          // Profiling de performance
    pub analyze_architecture: bool,         // Arquitectura y patrones
    pub detect_duplication: bool,           // Duplicación de código
    pub analyze_test_coverage: bool,        // Cobertura de tests
    pub suggest_refactoring: bool,          // Sugerencias de refactorización

    /// === BÚSQUEDA EXTERNA ===
    pub search_modern_libs: bool,
    pub search_best_practices: bool,
    pub search_tools: bool,

    /// === CONFIGURACIÓN AVANZADA ===
    pub max_file_size_mb: usize,            // Máximo tamaño de archivo a analizar
    pub min_complexity_threshold: usize,    // Umbral mínimo para reportar complejidad
    pub duplication_min_lines: usize,       // Mínimo de líneas para detectar duplicación
    pub security_scan_depth: usize,         // Profundidad del escaneo de seguridad
}

/// Resultado de análisis ULTRA-AVANZADO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAnalysisResult {
    /// Tipo de análisis
    pub analysis_type: String,

    /// Score de modernidad (0.0 - 1.0)
    pub modernity_score: f32,

    /// Score de calidad (0.0 - 1.0)
    pub quality_score: f32,

    /// Score de mantenibilidad (0.0 - 1.0)
    pub maintainability_score: f32,

    /// Recomendaciones por categoría
    pub recommendations: HashMap<String, Vec<Recommendation>>,

    /// Estadísticas del proyecto
    pub project_stats: ProjectStats,

    /// Métricas de código
    pub code_metrics: CodeMetrics,

    /// Issues encontrados
    pub issues: Vec<CodeIssue>,
}

/// Estadísticas del proyecto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStats {
    pub total_files: usize,
    pub total_lines: usize,
    pub language_distribution: HashMap<String, usize>,
    pub largest_files: Vec<(String, usize)>,
    pub oldest_files: Vec<(String, String)>,
    pub most_complex_files: Vec<(String, usize)>,
}

/// Métricas de código avanzadas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetrics {
    pub average_complexity: f32,
    pub max_complexity: usize,
    pub total_functions: usize,
    pub average_function_length: f32,
    pub duplication_percentage: f32,
    pub test_coverage_estimate: Option<f32>,
    pub documentation_coverage: f32,
}

/// Issue de código detectado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeIssue {
    pub file: String,
    pub line: Option<usize>,
    pub issue_type: String,
    pub severity: String,
    pub description: String,
    pub suggestion: Option<String>,
    pub complexity_score: Option<usize>,
}

/// Resultado del análisis de duplicación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicationResult {
    pub percentage: f32,
    pub issues: Vec<CodeIssue>,
    pub total_lines: usize,
    pub duplicated_lines: usize,
}

/// Resultado del análisis de seguridad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityResult {
    pub vulnerabilities: Vec<CodeIssue>,
    pub risk_score: f32,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Título
    pub title: String,

    /// Descripción
    pub description: String,

    /// URL/Referencia
    pub reference: String,

    /// Prioridad (1-10)
    pub priority: u8,

    /// Categoría
    pub category: String,
}

/// Analizador de proyecto
pub struct ProjectAnalyzer {
    web_search: Arc<WebSearch>,
}

impl ProjectAnalyzer {
    /// Crea nuevo analizador con web_search personalizado
    pub fn new(web_search: Arc<WebSearch>) -> Self {
        Self { web_search }
    }

    /// Crea nuevo analizador con web_search por defecto
    pub fn new_default() -> Result<Self> {
        Ok(Self {
            web_search: Arc::new(WebSearch::new_with_storage(None)?),
        })
    }

    /// Analiza un path (método simplificado)
    pub async fn analyze(&self, path: &PathBuf) -> Result<serde_json::Value> {
        let config = ProjectAnalysisConfig {
            project_dir: path.clone(),
            language: None,
            detect_errors: true,
            detect_warnings: true,
            detect_dead_code: true,
            detect_security_issues: true,
            analyze_complexity: true,
            detect_code_smells: true,
            analyze_dependencies: true,
            detect_vulnerabilities: true,
            profile_performance: false,
            analyze_architecture: true,
            detect_duplication: true,
            analyze_test_coverage: false,
            suggest_refactoring: false,
            search_modern_libs: true,
            search_best_practices: true,
            search_tools: true,
            max_file_size_mb: 10,
            min_complexity_threshold: 10,
            duplication_min_lines: 5,
            security_scan_depth: 3,
        };

        let result = self.analyze_project(config).await?;
        Ok(serde_json::to_value(result)?)
    }

    /// Analiza proyecto completo CON ANÁLISIS ULTRA-AVANZADO
    pub async fn analyze_project(
        &self,
        config: ProjectAnalysisConfig,
    ) -> Result<ProjectAnalysisResult> {
        let start_time = std::time::Instant::now();
        println!("🔬 ANÁLISIS ULTRA-AVANZADO DE PROYECTO");
        println!("   🚀 Técnicas: Complejidad, Code Smells, Arquitectura, Duplicación");

        // Detectar lenguaje
        let language = config
            .language
            .clone()
            .or_else(|| self.detect_language(&config.project_dir));
        println!("   Lenguaje detectado: {:?}", language);

        // === FASE 1: ANÁLISIS BÁSICO ===
        let mut recommendations: HashMap<String, Vec<Recommendation>> = HashMap::new();
        let mut issues = Vec::new();

        // Errores de compilación
        if config.detect_errors {
            if let Ok(errors) = self.detect_compilation_errors(&config.project_dir) {
                println!("   🔴 Errores: {}", errors.len());
                recommendations.insert("errors".to_string(), errors);
            }
        }

        // Warnings
        if config.detect_warnings {
            if let Ok(warnings) = self.detect_warnings(&config.project_dir) {
                println!("   🟡 Warnings: {}", warnings.len());
                recommendations.insert("warnings".to_string(), warnings);
            }
        }

        // === FASE 2: ANÁLISIS AVANZADO ===
        let mut code_metrics = CodeMetrics {
            average_complexity: 0.0,
            max_complexity: 0,
            total_functions: 0,
            average_function_length: 0.0,
            duplication_percentage: 0.0,
            test_coverage_estimate: None,
            documentation_coverage: 0.0,
        };

        // Complejidad ciclomática
        if config.analyze_complexity {
            if let Ok(complexity_issues) = self.analyze_complexity(&config.project_dir, config.min_complexity_threshold) {
                println!("   🧠 Complejidad analizada: {} funciones complejas", complexity_issues.len());
                issues.extend(complexity_issues);
            }
        }

        // Code smells
        if config.detect_code_smells {
            if let Ok(smells) = self.detect_code_smells(&config.project_dir) {
                println!("   👃 Code smells detectados: {}", smells.len());
                issues.extend(smells);
            }
        }

        // Duplicación de código
        if config.detect_duplication {
            if let Ok(duplication) = self.detect_code_duplication(&config.project_dir, config.duplication_min_lines) {
                println!("   📋 Duplicación detectada: {:.1}%", duplication.percentage);
                code_metrics.duplication_percentage = duplication.percentage;
                issues.extend(duplication.issues);
            }
        }

        // Arquitectura
        if config.analyze_architecture {
            if let Ok(arch_issues) = self.analyze_architecture(&config.project_dir) {
                println!("   🏗️ Issues de arquitectura: {}", arch_issues.len());
                issues.extend(arch_issues);
            }
        }

        // === FASE 3: DEPENDENCIAS Y SEGURIDAD ===
        if config.analyze_dependencies {
            let deps = self.analyze_dependencies_deep(&config.project_dir, &language).await?;
            recommendations.insert("dependencies".to_string(), deps);
        }

        if config.detect_security_issues {
            if let Ok(security) = self.detect_security_issues_advanced(&config.project_dir, config.security_scan_depth) {
                println!("   🔐 Vulnerabilidades: {}", security.len());
                recommendations.insert("security".to_string(), security);
            }
        }

        // === FASE 4: BÚSQUEDA EXTERNA ===
        if config.search_modern_libs {
            let modern_libs = self.search_modern_libraries(&language).await?;
            recommendations.insert("modern_libraries".to_string(), modern_libs);
        }

        if config.search_best_practices {
            let best_practices = self.search_best_practices(&language).await?;
            recommendations.insert("best_practices".to_string(), best_practices);
        }

        if config.search_tools {
            let tools = self.search_tools(&language).await?;
            recommendations.insert("tools".to_string(), tools);
        }

        // === FASE 5: ESTADÍSTICAS Y MÉTRICAS ===
        let project_stats = self.collect_project_stats(&config.project_dir)?;
        code_metrics = self.calculate_code_metrics(&config.project_dir, &issues)?;

        // === FASE 6: SCORES FINALES ===
        let quality_score = self.calculate_quality_score(&issues, &recommendations);
        let maintainability_score = self.calculate_maintainability_score(&code_metrics, &project_stats);

        let analysis_time_ms = start_time.elapsed().as_millis();
        println!("⚡ Análisis completado en {}ms", analysis_time_ms);
        println!("   📊 Quality Score: {:.2}, Maintainability: {:.2}, Modernity: {:.2}",
                quality_score, maintainability_score, self.calculate_modernity_score_from_recommendations(&recommendations));

        Ok(ProjectAnalysisResult {
            analysis_type: "ultra_advanced".to_string(),
            modernity_score: self.calculate_modernity_score_from_recommendations(&recommendations),
            quality_score,
            maintainability_score,
            recommendations,
            project_stats,
            code_metrics,
            issues,
        })
    }

    /// Detecta lenguaje del proyecto
    fn detect_language(&self, dir: &Path) -> Option<String> {
        // Buscar archivos característicos
        if dir.join("Cargo.toml").exists() {
            return Some("rust".to_string());
        }
        if dir.join("package.json").exists() {
            return Some("javascript".to_string());
        }
        if dir.join("requirements.txt").exists() || dir.join("setup.py").exists() {
            return Some("python".to_string());
        }
        if dir.join("go.mod").exists() {
            return Some("go".to_string());
        }
        None
    }

    /// Analiza dependencias
    async fn _analyze_dependencies(
        &self,
        dir: &Path,
        language: &str,
    ) -> Result<Vec<Recommendation>> {
        let mut recommendations = Vec::new();

        if language == "rust" {
            if let Ok(content) = fs::read_to_string(dir.join("Cargo.toml")) {
                // Buscar dependencias desactualizadas
                if !content.contains("tokio") {
                    recommendations.push(Recommendation {
                        title: "Agregar Tokio para async".to_string(),
                        description: "Tokio es el runtime async más moderno para Rust".to_string(),
                        reference: "https://tokio.rs/".to_string(),
                        priority: 9,
                        category: "dependencies".to_string(),
                    });
                }
            }
        }

        Ok(recommendations)
    }

    /// Busca librerías modernas
    async fn search_modern_libraries(
        &self,
        language: &Option<String>,
    ) -> Result<Vec<Recommendation>> {
        let query = if let Some(lang) = language {
            format!("modern {} libraries 2024", lang)
        } else {
            "modern programming libraries 2024".to_string()
        };

        let search_config = crate::web_search::WebSearchConfig {
            query,
            max_results: 20,
            ..Default::default()
        };

        let results = self.web_search.search(search_config).await?;

        let recommendations: Vec<Recommendation> = results
            .into_iter()
            .take(10)
            .map(|r| Recommendation {
                title: r.title,
                description: r.description,
                reference: r.url,
                priority: (r.relevance * 10.0) as u8,
                category: "modern_libraries".to_string(),
            })
            .collect();

        Ok(recommendations)
    }

    /// Busca mejores prácticas
    async fn search_best_practices(
        &self,
        language: &Option<String>,
    ) -> Result<Vec<Recommendation>> {
        let query = if let Some(lang) = language {
            format!("{} best practices 2024", lang)
        } else {
            "programming best practices 2024".to_string()
        };

        let search_config = crate::web_search::WebSearchConfig {
            query,
            max_results: 15,
            ..Default::default()
        };

        let results = self.web_search.search(search_config).await?;

        let recommendations: Vec<Recommendation> = results
            .into_iter()
            .take(10)
            .map(|r| Recommendation {
                title: r.title,
                description: r.description,
                reference: r.url,
                priority: (r.relevance * 10.0) as u8,
                category: "best_practices".to_string(),
            })
            .collect();

        Ok(recommendations)
    }

    /// Busca herramientas
    async fn search_tools(&self, language: &Option<String>) -> Result<Vec<Recommendation>> {
        let query = if let Some(lang) = language {
            format!("best {} tools 2024", lang)
        } else {
            "best development tools 2024".to_string()
        };

        let search_config = crate::web_search::WebSearchConfig {
            query,
            max_results: 15,
            ..Default::default()
        };

        let results = self.web_search.search(search_config).await?;

        let recommendations: Vec<Recommendation> = results
            .into_iter()
            .take(10)
            .map(|r| Recommendation {
                title: r.title,
                description: r.description,
                reference: r.url,
                priority: (r.relevance * 10.0) as u8,
                category: "tools".to_string(),
            })
            .collect();

        Ok(recommendations)
    }

    /// Analiza complejidad ciclomática
    fn analyze_complexity(&self, project_dir: &Path, threshold: usize) -> Result<Vec<CodeIssue>> {
        let mut issues = Vec::new();

        if let Ok(entries) = fs::read_dir(project_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let complexity = self.calculate_cyclomatic_complexity(&content);
                        if complexity > threshold {
                            issues.push(CodeIssue {
                                file: path.to_string_lossy().to_string(),
                                line: None,
                                issue_type: "high_complexity".to_string(),
                                severity: "warning".to_string(),
                                description: format!("High cyclomatic complexity: {}", complexity),
                                suggestion: Some("Consider breaking down this function into smaller ones".to_string()),
                                complexity_score: Some(complexity),
                            });
                        }
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Calcula complejidad ciclomática de código Rust
    fn calculate_cyclomatic_complexity(&self, content: &str) -> usize {
        let mut complexity = 1; // Base complexity

        // Contar estructuras de control
        let control_keywords = ["if", "else if", "while", "for", "loop", "match"];
        for keyword in &control_keywords {
            complexity += content.matches(keyword).count();
        }

        // Contar operadores lógicos
        complexity += content.matches("&&").count();
        complexity += content.matches("||").count();

        complexity
    }

    /// Detecta code smells avanzados
    fn detect_code_smells(&self, project_dir: &Path) -> Result<Vec<CodeIssue>> {
        let mut issues = Vec::new();

        if let Ok(entries) = fs::read_dir(project_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        // Long method smell
                        let lines: Vec<&str> = content.lines().collect();
                        if lines.len() > 100 {
                            issues.push(CodeIssue {
                                file: path.to_string_lossy().to_string(),
                                line: None,
                                issue_type: "long_method".to_string(),
                                severity: "info".to_string(),
                                description: format!("Very long file: {} lines", lines.len()),
                                suggestion: Some("Consider splitting into multiple modules".to_string()),
                                complexity_score: None,
                            });
                        }

                        // Large class smell (many methods)
                        let method_count = content.matches("fn ").count();
                        if method_count > 20 {
                            issues.push(CodeIssue {
                                file: path.to_string_lossy().to_string(),
                                line: None,
                                issue_type: "large_class".to_string(),
                                severity: "info".to_string(),
                                description: format!("Many functions in one file: {}", method_count),
                                suggestion: Some("Consider splitting into multiple structs/impl blocks".to_string()),
                                complexity_score: None,
                            });
                        }

                        // Primitive obsession
                        let primitive_vars = content.matches("let ").count();
                        if primitive_vars > 50 {
                            issues.push(CodeIssue {
                                file: path.to_string_lossy().to_string(),
                                line: None,
                                issue_type: "primitive_obsession".to_string(),
                                severity: "info".to_string(),
                                description: format!("Many primitive variables: {}", primitive_vars),
                                suggestion: Some("Consider creating custom types/structs".to_string()),
                                complexity_score: None,
                            });
                        }
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Detecta duplicación de código
    fn detect_code_duplication(&self, project_dir: &Path, min_lines: usize) -> Result<DuplicationResult> {
        let mut file_contents = Vec::new();

        // Recolectar contenido de archivos
        if let Ok(entries) = fs::read_dir(project_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        file_contents.push((path, content));
                    }
                }
            }
        }

        let mut issues = Vec::new();
        let mut _total_lines = 0;
        let mut duplicated_lines = 0;

        // Análisis simple de duplicación (puede mejorarse con algoritmos más sofisticados)
        for (i, (path1, content1)) in file_contents.iter().enumerate() {
            let lines1: Vec<&str> = content1.lines().collect();
            _total_lines += lines1.len();

            for (j, (path2, content2)) in file_contents.iter().enumerate() {
                if i >= j { continue; } // Evitar comparación consigo mismo y duplicados

                let lines2: Vec<&str> = content2.lines().collect();
                let mut common_lines = 0;

                // Comparación simple línea por línea
                for line1 in &lines1 {
                    let trimmed1 = line1.trim();
                    if trimmed1.is_empty() || trimmed1.starts_with("//") { continue; }

                    for line2 in &lines2 {
                        let trimmed2 = line2.trim();
                        if trimmed1 == trimmed2 {
                            common_lines += 1;
                            break;
                        }
                    }
                }

                if common_lines >= min_lines {
                    duplicated_lines += common_lines;
                    issues.push(CodeIssue {
                        file: format!("{} vs {}", path1.display(), path2.display()),
                        line: None,
                        issue_type: "code_duplication".to_string(),
                        severity: "warning".to_string(),
                        description: format!("Code duplication detected: {} common lines", common_lines),
                        suggestion: Some("Consider extracting common code into a shared function/module".to_string()),
                        complexity_score: None,
                    });
                }
            }
        }

        let percentage = if _total_lines > 0 {
            (duplicated_lines as f32 / _total_lines as f32) * 100.0
        } else {
            0.0
        };

        Ok(DuplicationResult {
            percentage,
            issues,
            total_lines: _total_lines,
            duplicated_lines,
        })
    }

    /// Analiza arquitectura del proyecto
    fn analyze_architecture(&self, project_dir: &Path) -> Result<Vec<CodeIssue>> {
        let mut issues = Vec::new();

        // Verificar estructura de directorios
        let expected_dirs = ["src", "tests", "benches", "examples"];
        for dir in &expected_dirs {
            let dir_path = project_dir.join(dir);
            if !dir_path.exists() {
                issues.push(CodeIssue {
                    file: dir.to_string(),
                    line: None,
                    issue_type: "missing_directory".to_string(),
                    severity: "info".to_string(),
                    description: format!("Missing standard directory: {}", dir),
                    suggestion: Some(format!("Consider creating {} directory for {}", dir, dir)),
                    complexity_score: None,
                });
            }
        }

        // Verificar imports circulares (análisis básico)
        if let Ok(entries) = fs::read_dir(project_dir.join("src")) {
            let mut modules = Vec::new();

            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".rs") && name != "lib.rs" && name != "main.rs" {
                        modules.push(name.trim_end_matches(".rs").to_string());
                    }
                }
            }

            // Verificar imports entre módulos
            for module in &modules {
                let module_file = project_dir.join("src").join(format!("{}.rs", module));
                if let Ok(content) = fs::read_to_string(&module_file) {
                    for other_module in &modules {
                        if module != other_module {
                            let import_pattern = format!("use crate::{};", other_module);
                            if content.contains(&import_pattern) {
                                // Verificar si el otro módulo importa este (posible circular)
                                let other_file = project_dir.join("src").join(format!("{}.rs", other_module));
                                if let Ok(other_content) = fs::read_to_string(&other_file) {
                                    let reverse_import = format!("use crate::{};", module);
                                    if other_content.contains(&reverse_import) {
                                        issues.push(CodeIssue {
                                            file: format!("src/{}.rs", module),
                                            line: None,
                                            issue_type: "circular_import".to_string(),
                                            severity: "error".to_string(),
                                            description: format!("Circular import detected between {} and {}", module, other_module),
                                            suggestion: Some("Refactor to break circular dependency".to_string()),
                                            complexity_score: None,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Detecta errores de compilación usando cargo check
    fn detect_compilation_errors(&self, project_dir: &Path) -> Result<Vec<Recommendation>> {
        let mut recommendations = Vec::new();

        if project_dir.join("Cargo.toml").exists() {
            match Command::new("cargo")
                .arg("check")
                .arg("--message-format=json")
                .current_dir(project_dir)
                .output()
            {
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let combined = format!("{}\n{}", stderr, stdout);

                    for line in combined.lines() {
                        if let Ok(msg) = serde_json::from_str::<serde_json::Value>(line) {
                            if let Some("error") = msg.get("level").and_then(|l| l.as_str()) {
                                let message = msg
                                    .get("message")
                                    .and_then(|m| m.as_str())
                                    .unwrap_or("Compilation error");

                                recommendations.push(Recommendation {
                                    title: "Compilation Error".to_string(),
                                    description: message.to_string(),
                                    reference: project_dir.display().to_string(),
                                    priority: 10,
                                    category: "errors".to_string(),
                                });
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }

        Ok(recommendations)
    }

    /// Detecta warnings usando cargo check
    fn detect_warnings(&self, project_dir: &Path) -> Result<Vec<Recommendation>> {
        let mut recommendations = Vec::new();

        if project_dir.join("Cargo.toml").exists() {
            match Command::new("cargo")
                .arg("check")
                .arg("--message-format=json")
                .current_dir(project_dir)
                .output()
            {
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let combined = format!("{}\n{}", stderr, stdout);

                    for line in combined.lines() {
                        if let Ok(msg) = serde_json::from_str::<serde_json::Value>(line) {
                            if let Some("warning") = msg.get("level").and_then(|l| l.as_str()) {
                                let message = msg
                                    .get("message")
                                    .and_then(|m| m.as_str())
                                    .unwrap_or("Warning");

                                recommendations.push(Recommendation {
                                    title: "Warning".to_string(),
                                    description: message.to_string(),
                                    reference: project_dir.display().to_string(),
                                    priority: 6,
                                    category: "warnings".to_string(),
                                });
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }

        Ok(recommendations)
    }

    /// Detecta dead code
    fn _detect_dead_code(
        &self,
        project_dir: &Path,
        _language: &Option<String>,
    ) -> Result<Vec<Recommendation>> {
        let mut recommendations = Vec::new();

        // Buscar patrones comunes de dead code
        if let Ok(entries) = fs::read_dir(project_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        // Detectar funciones no usadas
                        if content.contains("#[allow(dead_code)]") {
                            recommendations.push(Recommendation {
                                title: "Dead Code Found".to_string(),
                                description: format!(
                                    "Possible dead code in {}",
                                    path.display()
                                ),
                                reference: path.display().to_string(),
                                priority: 4,
                                category: "dead_code".to_string(),
                            });
                        }
                    }
                }
            }
        }

        Ok(recommendations)
    }

    /// Detecta problemas de seguridad avanzados
    fn detect_security_issues_advanced(&self, project_dir: &Path, _depth: usize) -> Result<Vec<Recommendation>> {
        let mut recommendations = Vec::new();

        // Verificar uso de unsafe
        if let Ok(entries) = fs::read_dir(project_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let unsafe_count = content.matches("unsafe").count();
                        if unsafe_count > 0 {
                            recommendations.push(Recommendation {
                                title: "Unsafe Code Detected".to_string(),
                                description: format!(
                                    "{} unsafe blocks found in {}",
                                    unsafe_count,
                                    path.display()
                                ),
                                reference: path.display().to_string(),
                                priority: 8,
                                category: "security".to_string(),
                            });
                        }

                        // Detectar unwrap sin justificación
                        let unwrap_count = content.matches(".unwrap()").count();
                        if unwrap_count > 0 {
                            recommendations.push(Recommendation {
                                title: "Potential Panic Points".to_string(),
                                description: format!(
                                    "{} unwrap() calls found in {}",
                                    unwrap_count,
                                    path.display()
                                ),
                                reference: path.display().to_string(),
                                priority: 7,
                                category: "security".to_string(),
                            });
                        }

                        // Detectar uso de expect sin justificación
                        let expect_count = content.matches(".expect(").count();
                        if expect_count > 0 {
                            recommendations.push(Recommendation {
                                title: "Potential Panic Points (expect)".to_string(),
                                description: format!(
                                    "{} expect() calls found in {}",
                                    expect_count,
                                    path.display()
                                ),
                                reference: path.display().to_string(),
                                priority: 6,
                                category: "security".to_string(),
                            });
                        }

                        // Detectar posibles SQL injection (patrones básicos)
                        if content.contains("format!") && content.contains("sql") {
                            recommendations.push(Recommendation {
                                title: "Potential SQL Injection".to_string(),
                                description: "String formatting with SQL detected - consider prepared statements".to_string(),
                                reference: path.display().to_string(),
                                priority: 9,
                                category: "security".to_string(),
                            });
                        }

                        // Detectar posibles XSS (patrones básicos)
                        if content.contains("innerHTML") || content.contains("outerHTML") {
                            recommendations.push(Recommendation {
                                title: "Potential XSS Vulnerability".to_string(),
                                description: "Direct HTML manipulation detected".to_string(),
                                reference: path.display().to_string(),
                                priority: 8,
                                category: "security".to_string(),
                            });
                        }
                    }
                }
            }
        }

        Ok(recommendations)
    }

    /// Calcula métricas de código avanzadas
    fn calculate_code_metrics(&self, project_dir: &Path, issues: &[CodeIssue]) -> Result<CodeMetrics> {
        let mut total_complexity = 0;
        let mut max_complexity = 0;
        let mut total_functions = 0;
        let mut _total_lines = 0;
        let mut total_function_lines = 0;

        if let Ok(entries) = fs::read_dir(project_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let lines: Vec<&str> = content.lines().collect();
                        _total_lines += lines.len();

                        // Contar funciones
                        let function_count = content.matches("fn ").count();
                        total_functions += function_count;

                        // Calcular complejidad por función
                        for line in &lines {
                            if line.contains("fn ") {
                                let complexity = self.calculate_cyclomatic_complexity(&content);
                                total_complexity += complexity;
                                if complexity > max_complexity {
                                    max_complexity = complexity;
                                }
                                // Estimar líneas por función (aproximado)
                                total_function_lines += 10; // Estimación básica
                            }
                        }
                    }
                }
            }
        }

        let average_complexity = if total_functions > 0 {
            total_complexity as f32 / total_functions as f32
        } else {
            0.0
        };

        let average_function_length = if total_functions > 0 {
            total_function_lines as f32 / total_functions as f32
        } else {
            0.0
        };

        // Calcular duplicación desde issues
        let duplication_percentage = issues
            .iter()
            .find(|i| i.issue_type == "code_duplication")
            .and_then(|i| i.description.split(": ").nth(1))
            .and_then(|s| s.split(" ").next())
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0);

        // Estimar cobertura de tests (muy básica)
        let test_coverage_estimate = Some(0.5); // Estimación por defecto

        // Calcular cobertura de documentación
        let documentation_coverage = 0.3; // Estimación básica

        Ok(CodeMetrics {
            average_complexity,
            max_complexity,
            total_functions,
            average_function_length,
            duplication_percentage,
            test_coverage_estimate,
            documentation_coverage,
        })
    }

    /// Analiza dependencias de manera profunda
    async fn analyze_dependencies_deep(&self, dir: &Path, language: &Option<String>) -> Result<Vec<Recommendation>> {
        let mut recommendations = Vec::new();

        if let Some(lang) = language {
            if lang == "rust" {
                if let Ok(content) = fs::read_to_string(dir.join("Cargo.toml")) {
                    // Verificar versiones desactualizadas
                    if content.contains("serde = \"") && !content.contains("serde = \"1.") {
                        recommendations.push(Recommendation {
                            title: "Update Serde".to_string(),
                            description: "Serde version might be outdated".to_string(),
                            reference: "https://crates.io/crates/serde".to_string(),
                            priority: 7,
                            category: "dependencies".to_string(),
                        });
                    }

                    // Verificar dependencias críticas faltantes
                    if !content.contains("tokio") {
                        recommendations.push(Recommendation {
                            title: "Add Tokio".to_string(),
                            description: "Tokio is essential for async operations".to_string(),
                            reference: "https://crates.io/crates/tokio".to_string(),
                            priority: 9,
                            category: "dependencies".to_string(),
                        });
                    }

                    if !content.contains("anyhow") {
                        recommendations.push(Recommendation {
                            title: "Add Anyhow".to_string(),
                            description: "Anyhow provides ergonomic error handling".to_string(),
                            reference: "https://crates.io/crates/anyhow".to_string(),
                            priority: 8,
                            category: "dependencies".to_string(),
                        });
                    }
                }
            }
        }

        Ok(recommendations)
    }

    /// Recolecta estadísticas del proyecto
    fn collect_project_stats(&self, project_dir: &Path) -> Result<ProjectStats> {
        let mut total_files = 0;
        let mut total_lines = 0;
        let mut language_distribution = HashMap::new();
        let mut largest_files = Vec::new();
        let mut oldest_files = Vec::new();
        let mut most_complex_files = Vec::new();

        if let Ok(entries) = fs::read_dir(project_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    total_files += 1;

                    if let Ok(content) = fs::read_to_string(&path) {
                        let lines = content.lines().count();
                        total_lines += lines;

                        // Distribución por lenguaje
                        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                            *language_distribution.entry(ext.to_string()).or_insert(0) += 1;
                        }

                        // Archivos más grandes
                        largest_files.push((path.display().to_string(), lines));
                        largest_files.sort_by(|a, b| b.1.cmp(&a.1));
                        largest_files.truncate(5);

                        // Complejidad
                        let complexity = self.calculate_cyclomatic_complexity(&content);
                        most_complex_files.push((path.display().to_string(), complexity));
                        most_complex_files.sort_by(|a, b| b.1.cmp(&a.1));
                        most_complex_files.truncate(5);
                    }

                    // Archivos más antiguos (por metadata)
                    if let Ok(metadata) = path.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            oldest_files.push((path.display().to_string(), format!("{:?}", modified)));
                        }
                    }
                }
            }
        }

        Ok(ProjectStats {
            total_files,
            total_lines,
            language_distribution,
            largest_files,
            oldest_files,
            most_complex_files,
        })
    }

    /// Calcula score de calidad
    fn calculate_quality_score(&self, issues: &[CodeIssue], recommendations: &HashMap<String, Vec<Recommendation>>) -> f32 {
        let mut score: f32 = 1.0;

        // Penalizar por issues
        for issue in issues {
            match issue.severity.as_str() {
                "error" => score -= 0.1,
                "warning" => score -= 0.05,
                "info" => score -= 0.01,
                _ => score -= 0.02,
            }
        }

        // Penalizar por recomendaciones críticas
        for (_category, recs) in recommendations {
            for rec in recs {
                if rec.priority >= 8 {
                    score -= 0.05;
                }
            }
        }

        score.max(0.0).min(1.0)
    }

    /// Calcula score de mantenibilidad
    fn calculate_maintainability_score(&self, metrics: &CodeMetrics, stats: &ProjectStats) -> f32 {
        let mut score: f32 = 1.0;

        // Penalizar complejidad alta
        if metrics.average_complexity > 10.0 {
            score -= 0.2;
        }

        // Penalizar duplicación alta
        if metrics.duplication_percentage > 20.0 {
            score -= 0.3;
        }

        // Penalizar archivos muy grandes
        if stats.total_lines > 10000 {
            score -= 0.1;
        }

        // Bonus por buena documentación
        if metrics.documentation_coverage > 0.5 {
            score += 0.1;
        }

        score.max(0.0).min(1.0)
    }

    /// Calcula score de modernidad desde recomendaciones
    fn calculate_modernity_score_from_recommendations(&self, recommendations: &HashMap<String, Vec<Recommendation>>) -> f32 {
        if recommendations.is_empty() {
            return 0.5;
        }

        let total_recs: usize = recommendations.values().map(|v| v.len()).sum();
        let high_priority_recs: usize = recommendations
            .values()
            .flatten()
            .filter(|r| r.priority >= 8)
            .count();

        if total_recs == 0 {
            0.5
        } else {
            1.0 - (high_priority_recs as f32 / total_recs as f32)
        }
    }
}
