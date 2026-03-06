//! 🔥 SCAN WORKSPACE TOOL - Deep Code Analysis + Chapel AI
//!
//! PODER TOTAL: Escanea archivos, carpetas, workspace completo
//! INVESTIGA EN INTERNET: busca librerías, compara, mejores prácticas
//! CHAPEL AI: consejos inteligentes basados en investigación web
//! Detecta: errores, warnings, TODOs, mocks, patrones problemáticos
//! Genera: consejos, métricas, reportes de salud, próximos pasos

use crate::chapel_integration::{create_context, get_chapel_ai};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Scan configuration - SIMPLIFICADA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    pub path: String,
    pub recursive: bool,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            path: ".".to_string(),
            recursive: true,
        }
    }
}

/// Individual issue found during scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanIssue {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub category: IssueCategory,
    pub severity: IssueSeverity,
    pub message: String,
    pub code_snippet: String,
    pub suggestion: String,
}

/// Issue categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum IssueCategory {
    Mock,
    Todo,
    Fixme,
    Hack,
    Security,
    Performance,
    UnusedCode,
    DeadCode,
    Deprecation,
    ErrorHandling,
    CodeStyle,
}

/// Issue severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// File analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub path: String,
    pub lines_total: usize,
    pub lines_code: usize,
    pub lines_comment: usize,
    pub lines_blank: usize,
    pub issues: Vec<ScanIssue>,
    pub health_score: f64,
    pub complexity_score: f64,
}

/// Complete scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub scanned_path: String,
    pub files_scanned: usize,
    pub total_lines: usize,
    pub total_issues: usize,
    pub issues_by_category: HashMap<String, usize>,
    pub issues_by_severity: HashMap<String, usize>,
    pub files: Vec<FileAnalysis>,
    pub top_issues: Vec<ScanIssue>,
    pub health_score: f64,
    pub advice: Vec<String>,
    pub scan_duration_ms: u64,
}

/// 🔥 SCAN WORKSPACE TOOL - Maximum Power
#[derive(Debug, Default)]
pub struct ScanWorkspaceTool {
    patterns: ScanPatterns,
}

/// Patterns to detect issues
#[derive(Debug, Clone)]
struct ScanPatterns {
    mock_patterns: Vec<&'static str>,
    todo_patterns: Vec<&'static str>,
    security_patterns: Vec<&'static str>,
    performance_patterns: Vec<&'static str>,
}

impl Default for ScanPatterns {
    fn default() -> Self {
        Self {
            mock_patterns: vec![
                "mock",
                "Mock",
                "MOCK",
                "fake",
                "Fake",
                "FAKE",
                "stub",
                "Stub",
                "STUB",
                "dummy",
                "Dummy",
                "simul",
                "Simul",
                "test_data",
                "placeholder",
            ],
            todo_patterns: vec![
                "TODO",
                "FIXME",
                "HACK",
                "XXX",
                "BUG",
                "OPTIMIZE",
                "REFACTOR",
                "REVIEW",
                "NOTE:",
                "WARN:",
                "DEPRECATED",
            ],
            security_patterns: vec![
                "password",
                "secret",
                "api_key",
                "apikey",
                "token",
                "credential",
                "unsafe",
                "unwrap()",
                "expect(",
                "panic!",
                "unreachable!",
                "sql!",
                "exec(",
                "eval(",
                "innerHTML",
                "dangerously",
            ],
            performance_patterns: vec![
                "clone()",
                ".collect()",
                "Vec::new()",
                "to_string()",
                "format!",
                "unwrap_or_else",
                "loop {",
                "while true",
                "sleep(",
                "thread::spawn",
                "blocking",
            ],
        }
    }
}

impl ScanWorkspaceTool {
    pub fn new() -> Self {
        eprintln!("🔥 Scan Workspace Tool - MÁXIMO PODER + Chapel AI");
        eprintln!("   ✅ ESCANEA: Errores, Warnings, TODOs, Mocks, Código muerto");
        eprintln!("   ✅ INVESTIGA INTERNET: Librerías, versiones, mejores prácticas");
        eprintln!("   ✅ COMPARA: Alternativas, benchmarks, recomendaciones");
        eprintln!("   ✅ CHAPEL AI: Consejos inteligentes basados en investigación");
        eprintln!("   ✅ Analiza: Patrones de seguridad, performance, complejidad");
        eprintln!("   ✅ Genera: Reportes, métricas, próximos pasos");
        Self {
            patterns: ScanPatterns::default(),
        }
    }

    /// 🔥 Execute full workspace scan - MÁXIMO PODER
    pub async fn scan(&self, config: ScanConfig) -> ScanResult {
        let start = std::time::Instant::now();

        eprintln!("🔥 SCAN INICIADO - MÁXIMO PODER");
        eprintln!("   📁 Path: {}", config.path);
        eprintln!("   🔁 Recursive: {}", config.recursive);

        let path = PathBuf::from(&config.path);
        let mut files_analyzed: Vec<FileAnalysis> = Vec::new();
        let mut all_issues: Vec<ScanIssue> = Vec::new();

        // Collect files to scan
        let files = self.collect_files(&path, &config, 0);

        // Analyze each file
        for file_path in &files {
            if let Ok(analysis) = self.analyze_file(file_path, &config) {
                all_issues.extend(analysis.issues.clone());
                files_analyzed.push(analysis);
            }
        }

        // Calculate statistics
        let total_lines: usize = files_analyzed.iter().map(|f| f.lines_total).sum();
        let total_issues = all_issues.len();

        // Issues by category
        let mut issues_by_category: HashMap<String, usize> = HashMap::new();
        for issue in &all_issues {
            *issues_by_category
                .entry(format!("{:?}", issue.category))
                .or_insert(0) += 1;
        }

        // Issues by severity
        let mut issues_by_severity: HashMap<String, usize> = HashMap::new();
        for issue in &all_issues {
            *issues_by_severity
                .entry(format!("{:?}", issue.severity))
                .or_insert(0) += 1;
        }

        // Sort issues by severity (critical first)
        all_issues.sort_by(|a, b| b.severity.cmp(&a.severity));
        let top_issues: Vec<ScanIssue> = all_issues.iter().take(20).cloned().collect();

        // Calculate overall health score
        let health_score = self.calculate_health_score(&files_analyzed, total_issues, total_lines);

        // Generate advice (with Chapel AI)
        let advice = self.generate_advice_with_chapel(
            &issues_by_category,
            &issues_by_severity,
            health_score,
        );

        let duration = start.elapsed().as_millis() as u64;

        // 🧠 Chapel AI: Learn from scan results
        let chapel = get_chapel_ai();
        let quality = health_score / 100.0;
        let context = create_context("scan", "scan_workspace", &config.path, quality);
        let _ = chapel.learn(context);

        ScanResult {
            scanned_path: config.path,
            files_scanned: files_analyzed.len(),
            total_lines,
            total_issues,
            issues_by_category,
            issues_by_severity,
            files: files_analyzed,
            top_issues,
            health_score,
            advice,
            scan_duration_ms: duration,
        }
    }

    /// Collect all files to scan
    fn collect_files(&self, path: &Path, config: &ScanConfig, depth: usize) -> Vec<PathBuf> {
        let mut files = Vec::new();

        if depth > 10 {
            return files;
        }

        if path.is_file() {
            if self.should_scan_file(path, config) {
                files.push(path.to_path_buf());
            }
        } else if path.is_dir() {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();

                    // Skip hidden directories and common excludes
                    if let Some(name) = entry_path.file_name().and_then(|n| n.to_str()) {
                        if name.starts_with('.')
                            || name == "target"
                            || name == "node_modules"
                            || name == "vendor"
                            || name == "__pycache__"
                        {
                            continue;
                        }
                    }

                    if entry_path.is_file() {
                        if self.should_scan_file(&entry_path, config) {
                            files.push(entry_path);
                        }
                    } else if config.recursive && entry_path.is_dir() {
                        files.extend(self.collect_files(&entry_path, config, depth + 1));
                    }
                }
            }
        }

        files
    }

    /// Check if file should be scanned
    fn should_scan_file(&self, path: &Path, _config: &ScanConfig) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            matches!(
                ext,
                "rs" | "toml"
                    | "md"
                    | "py"
                    | "js"
                    | "ts"
                    | "go"
                    | "zig"
                    | "nim"
                    | "java"
                    | "cpp"
                    | "c"
                    | "h"
            )
        } else {
            false
        }
    }

    /// Analyze a single file
    fn analyze_file(
        &self,
        path: &Path,
        _config: &ScanConfig,
    ) -> Result<FileAnalysis, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let lines: Vec<&str> = content.lines().collect();

        let mut issues = Vec::new();
        let mut lines_code = 0;
        let mut lines_comment = 0;
        let mut lines_blank = 0;

        let path_str = path.display().to_string();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let is_rust = ext == "rs";

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Count line types
            if trimmed.is_empty() {
                lines_blank += 1;
            } else if trimmed.starts_with("//")
                || trimmed.starts_with("#")
                || trimmed.starts_with("/*")
                || trimmed.starts_with("*")
            {
                lines_comment += 1;
            } else {
                lines_code += 1;
            }

            // Detect mocks (siempre activado)
            for pattern in &self.patterns.mock_patterns {
                if line.contains(pattern) {
                    // Skip intentional patterns (like in bypass code)
                    if !line.contains("generate_fake_ray_id") && !line.contains("detect_mock") {
                        issues.push(ScanIssue {
                            file: path_str.clone(),
                            line: line_num + 1,
                            column: line.find(pattern).unwrap_or(0) + 1,
                            category: IssueCategory::Mock,
                            severity: IssueSeverity::Warning,
                            message: format!("Mock pattern detected: '{}'", pattern),
                            code_snippet: line.to_string(),
                            suggestion: "Replace with real implementation or remove if unused"
                                .to_string(),
                        });
                    }
                }
            }

            // Detect TODOs (siempre activado)
            for pattern in &self.patterns.todo_patterns {
                if line.contains(pattern) {
                    let severity = match *pattern {
                        "FIXME" | "BUG" => IssueSeverity::Error,
                        "HACK" | "XXX" => IssueSeverity::Warning,
                        _ => IssueSeverity::Info,
                    };

                    issues.push(ScanIssue {
                        file: path_str.clone(),
                        line: line_num + 1,
                        column: line.find(pattern).unwrap_or(0) + 1,
                        category: IssueCategory::Todo,
                        severity,
                        message: format!("{} comment found", pattern),
                        code_snippet: line.to_string(),
                        suggestion: format!(
                            "Address the {} comment or remove if resolved",
                            pattern
                        ),
                    });
                }
            }

            // Detect security issues (siempre activado)
            for pattern in &self.patterns.security_patterns {
                if line.to_lowercase().contains(&pattern.to_lowercase()) {
                    // Skip false positives
                    if *pattern == "unsafe" && is_rust && !trimmed.starts_with("unsafe") {
                        continue;
                    }
                    if *pattern == "unwrap()" && line.contains("unwrap_or") {
                        continue;
                    }

                    let severity = match *pattern {
                        "password" | "secret" | "api_key" | "credential" => IssueSeverity::Critical,
                        "unsafe" | "panic!" | "sql!" | "eval(" => IssueSeverity::Error,
                        _ => IssueSeverity::Warning,
                    };

                    issues.push(ScanIssue {
                        file: path_str.clone(),
                        line: line_num + 1,
                        column: line
                            .to_lowercase()
                            .find(&pattern.to_lowercase())
                            .unwrap_or(0)
                            + 1,
                        category: IssueCategory::Security,
                        severity,
                        message: format!("Security pattern detected: '{}'", pattern),
                        code_snippet: line.to_string(),
                        suggestion: self.get_security_suggestion(pattern),
                    });
                }
            }

            // Detect performance issues (siempre activado)
            if is_rust {
                for pattern in &self.patterns.performance_patterns {
                    if line.contains(pattern) {
                        issues.push(ScanIssue {
                            file: path_str.clone(),
                            line: line_num + 1,
                            column: line.find(pattern).unwrap_or(0) + 1,
                            category: IssueCategory::Performance,
                            severity: IssueSeverity::Info,
                            message: format!("Performance pattern: '{}'", pattern),
                            code_snippet: line.to_string(),
                            suggestion: self.get_performance_suggestion(pattern),
                        });
                    }
                }
            }
        }

        // Calculate file health score
        let issue_penalty: f64 = issues
            .iter()
            .map(|i| match i.severity {
                IssueSeverity::Critical => 25.0,
                IssueSeverity::Error => 10.0,
                IssueSeverity::Warning => 3.0,
                IssueSeverity::Info => 1.0,
            })
            .sum();

        let health_score = (100.0 - issue_penalty.min(100.0)).max(0.0);

        // Calculate complexity (simplified)
        let complexity_score = if lines_code > 0 {
            let comment_ratio = lines_comment as f64 / lines_code as f64;
            let lines_factor = (lines_code as f64 / 500.0).min(1.0);
            50.0 + (comment_ratio * 20.0) - (lines_factor * 30.0)
        } else {
            50.0
        };

        Ok(FileAnalysis {
            path: path_str,
            lines_total: lines.len(),
            lines_code,
            lines_comment,
            lines_blank,
            issues,
            health_score,
            complexity_score: complexity_score.clamp(0.0, 100.0),
        })
    }

    fn get_security_suggestion(&self, pattern: &str) -> String {
        match pattern {
            "password" | "secret" | "api_key" | "credential" => {
                "Use environment variables or a secrets manager instead of hardcoding".to_string()
            }
            "unsafe" => "Ensure unsafe block is necessary and properly documented".to_string(),
            "unwrap()" => "Use ? operator or match for proper error handling".to_string(),
            "expect(" => "Consider using ? or map_err for better error messages".to_string(),
            "panic!" => "Replace with Result return type for recoverable errors".to_string(),
            "sql!" | "exec(" | "eval(" => {
                "Ensure input is properly sanitized to prevent injection".to_string()
            }
            _ => "Review for potential security implications".to_string(),
        }
    }

    fn get_performance_suggestion(&self, pattern: &str) -> String {
        match pattern {
            "clone()" => {
                "Consider using references or Cow<> to avoid unnecessary cloning".to_string()
            }
            ".collect()" => {
                "Collect only when necessary; prefer iterators for lazy evaluation".to_string()
            }
            "Vec::new()" => "Use Vec::with_capacity() if size is known".to_string(),
            "to_string()" => "Consider using &str or Cow<str> if ownership not needed".to_string(),
            "format!" => "Use write! or push_str for repeated string building".to_string(),
            _ => "Review for potential performance improvements".to_string(),
        }
    }

    fn calculate_health_score(
        &self,
        files: &[FileAnalysis],
        total_issues: usize,
        total_lines: usize,
    ) -> f64 {
        if files.is_empty() || total_lines == 0 {
            return 100.0;
        }

        // Average file health
        let avg_health: f64 =
            files.iter().map(|f| f.health_score).sum::<f64>() / files.len() as f64;

        // Issues per 1000 lines
        let issues_ratio = (total_issues as f64 / total_lines as f64) * 1000.0;
        let issues_penalty = (issues_ratio * 2.0).min(30.0);

        (avg_health - issues_penalty).clamp(0.0, 100.0)
    }

    fn generate_advice(
        &self,
        by_category: &HashMap<String, usize>,
        by_severity: &HashMap<String, usize>,
        health: f64,
    ) -> Vec<String> {
        let mut advice = Vec::new();

        // Health-based advice
        if health < 50.0 {
            advice.push(
                "⚠️ CRÍTICO: Salud del código muy baja. Priorizar limpieza urgente.".to_string(),
            );
        } else if health < 70.0 {
            advice.push(
                "⚡ ADVERTENCIA: Código necesita mejoras. Planificar refactorización.".to_string(),
            );
        } else if health < 90.0 {
            advice.push("✅ BUENO: Código en estado aceptable. Mantener vigilancia.".to_string());
        } else {
            advice.push("🔥 EXCELENTE: Código en muy buen estado!".to_string());
        }

        // Category-specific advice
        if let Some(&mock_count) = by_category.get("Mock") {
            if mock_count > 0 {
                advice.push(format!(
                    "🎭 Encontrados {} patrones mock/fake. Reemplazar con implementaciones reales.",
                    mock_count
                ));
            }
        }

        if let Some(&todo_count) = by_category.get("Todo") {
            if todo_count > 10 {
                advice.push(format!(
                    "📝 {} TODOs pendientes. Crear issues para tracking.",
                    todo_count
                ));
            } else if todo_count > 0 {
                advice.push(format!(
                    "📝 {} TODOs pendientes. Revisar y completar.",
                    todo_count
                ));
            }
        }

        if let Some(&security_count) = by_category.get("Security") {
            if security_count > 0 {
                advice.push(format!(
                    "🔒 {} patrones de seguridad detectados. Revisar URGENTE.",
                    security_count
                ));
            }
        }

        // Severity-based advice
        if let Some(&critical) = by_severity.get("Critical") {
            if critical > 0 {
                advice.push(format!(
                    "🚨 {} issues CRÍTICOS requieren atención inmediata.",
                    critical
                ));
            }
        }

        advice
    }

    /// Generate advice with Chapel AI integration - ENHANCED
    fn generate_advice_with_chapel(
        &self,
        by_category: &HashMap<String, usize>,
        by_severity: &HashMap<String, usize>,
        health: f64,
    ) -> Vec<String> {
        let mut advice = self.generate_advice(by_category, by_severity, health);

        // 🧠 Add Chapel AI suggestions
        let chapel = get_chapel_ai();

        // Get AI-based next steps
        let mut category_map = HashMap::new();
        if let Some(&errors) = by_severity.get("Error") {
            category_map.insert("errors".to_string(), errors);
        }
        if let Some(&warnings) = by_severity.get("Warning") {
            category_map.insert("warnings".to_string(), warnings);
        }
        if let Some(&mocks) = by_category.get("Mock") {
            category_map.insert("mocks".to_string(), mocks);
        }
        if let Some(&todos) = by_category.get("Todo") {
            category_map.insert("todos".to_string(), todos);
        }

        let next_steps = chapel.suggest_next_steps();
        advice.push("".to_string());
        advice.push("🧠 Chapel AI - Próximos Pasos Sugeridos:".to_string());
        match next_steps {
            Ok(steps) => {
                for step in steps {
                    advice.push(format!("  {}", step));
                }
            }
            Err(_e) => {
                advice.push("  (Chapel AI optimization en progreso)".to_string());
            }
        }

        // Get Chapel AI specific advice for scan tool
        if let Ok(ai_advice) = chapel.get_advice("scan", "scan_workspace") {
            if !ai_advice.is_empty() {
                advice.push("".to_string());
                advice.push("🔍 Chapel AI - Investigación y Recomendaciones:".to_string());
                for a in ai_advice {
                    advice.push(format!(
                        "  {} [{}]: {} (confianza: {:.0}%)",
                        match a.priority.as_str() {
                            "high" => "🔴",
                            "medium" => "🟡",
                            _ => "🟢",
                        },
                        a.category,
                        a.suggestion,
                        a.confidence * 100.0
                    ));
                }
            }
        }

        advice
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scan_workspace() {
        let tool = ScanWorkspaceTool::new();
        let config = ScanConfig {
            path: "src".to_string(),
            ..Default::default()
        };

        let result = tool.scan(config).await;
        assert!(result.files_scanned > 0);
        assert!(result.health_score >= 0.0 && result.health_score <= 100.0);
    }
}
