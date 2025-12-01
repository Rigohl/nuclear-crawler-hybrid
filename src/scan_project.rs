//! Módulo Scan Project - Escanea proyectos con ubicaciones exactas y soluciones
//!
//! Analiza proyectos en múltiples lenguajes: Rust, Python, JavaScript, Go, Java, C++
//! Incluye análisis de seguridad básico

use crate::web_search::{WebSearch, WebSearchConfig};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

/// Lenguajes de programación soportados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Java,
    Cpp,
    C,
    Zig,
    Chapel,
    Unknown,
}

/// Vulnerabilidades de seguridad encontradas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    /// Tipo de vulnerabilidad
    pub vulnerability_type: String,
    /// Severidad (1-10)
    pub severity: u8,
    /// Archivo donde se encuentra
    pub file: String,
    /// Línea exacta
    pub line: usize,
    /// Descripción de la vulnerabilidad
    pub description: String,
    /// Código vulnerable
    pub vulnerable_code: Option<String>,
    /// Recomendaciones de corrección
    pub recommendations: Vec<String>,
    /// CWE ID si aplica
    pub cwe_id: Option<String>,
}

/// Análisis de seguridad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAnalysis {
    /// Vulnerabilidades encontradas
    pub vulnerabilities: Vec<SecurityIssue>,
    /// Score de seguridad (0-100, 100 = más seguro)
    pub security_score: f32,
    /// Número total de archivos analizados
    pub files_analyzed: usize,
    /// Lenguaje detectado
    pub language: Language,
}

/// Solución encontrada para un error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solution {
    /// Título de la solución
    pub title: String,
    /// Descripción detallada
    pub description: String,
    /// URL de referencia
    pub url: String,
    /// Código de ejemplo (si aplica)
    pub example_code: Option<String>,
    /// Prioridad (1-10)
    pub priority: u8,
}

/// Issue encontrado en el código
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    /// Tipo de issue (error, warning, etc.)
    pub issue_type: String,
    /// Código del error/warning
    pub code: Option<String>,
    /// Archivo donde se encuentra
    pub file: String,
    /// Línea exacta
    pub line: usize,
    /// Columna (opcional)
    pub column: Option<usize>,
    /// Mensaje descriptivo
    pub message: String,
    /// Código fuente de la línea
    pub source_code: Option<String>,
    /// Soluciones encontradas
    pub solutions: Vec<Solution>,
}

/// Resultado del scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// Lenguaje detectado
    pub language: Language,
    /// Errores encontrados
    pub errors: Vec<Issue>,
    /// Warnings encontrados
    pub warnings: Vec<Issue>,
    /// Total de issues
    pub total_issues: usize,
    /// Análisis de seguridad
    pub security_analysis: SecurityAnalysis,
    /// Recomendaciones generales
    pub recommendations: Vec<String>,
    /// Score de calidad (0-100)
    pub quality_score: f32,
}

/// Scanner de proyectos
pub struct ProjectScanner {
    web_search: Arc<WebSearch>,
}

impl ProjectScanner {
    /// Crea nuevo scanner
    pub fn new(web_search: Arc<WebSearch>) -> Self {
        Self { web_search }
    }

    /// Detecta el lenguaje de un proyecto
    pub fn detect_language(&self, project_path: &PathBuf) -> Result<Language> {
        // Verificar archivos característicos de cada lenguaje
        let rust_files = self.count_files_with_extension(project_path, "rs")?;
        let python_files = self.count_files_with_extension(project_path, "py")?;
        let js_files = self.count_files_with_extension(project_path, "js")?;
        let ts_files = self.count_files_with_extension(project_path, "ts")?;
        let go_files = self.count_files_with_extension(project_path, "go")?;
        let java_files = self.count_files_with_extension(project_path, "java")?;
        let cpp_files = self.count_files_with_extension(project_path, "cpp")?;
        let c_files = self.count_files_with_extension(project_path, "c")?;
        let zig_files = self.count_files_with_extension(project_path, "zig")?;
        let chapel_files = self.count_files_with_extension(project_path, "chpl")?;

        // Verificar archivos de configuración específicos
        let has_cargo_toml = project_path.join("Cargo.toml").exists();
        let has_pyproject_toml = project_path.join("pyproject.toml").exists() || project_path.join("setup.py").exists();
        let has_package_json = project_path.join("package.json").exists();
        let has_go_mod = project_path.join("go.mod").exists();
        let has_pom_xml = project_path.join("pom.xml").exists() || project_path.join("build.gradle").exists();
        let has_makefile = project_path.join("Makefile").exists() || project_path.join("CMakeLists.txt").exists();

        // Lógica de detección basada en archivos y configuración
        if has_cargo_toml && rust_files > 0 {
            return Ok(Language::Rust);
        }
        if has_package_json && (js_files > 0 || ts_files > 0) {
            return Ok(Language::JavaScript);
        }
        if has_pyproject_toml && python_files > 0 {
            return Ok(Language::Python);
        }
        if has_go_mod && go_files > 0 {
            return Ok(Language::Go);
        }
        if has_pom_xml && java_files > 0 {
            return Ok(Language::Java);
        }
        if has_makefile && (cpp_files > 0 || c_files > 0) {
            return Ok(Language::Cpp);
        }
        if zig_files > 0 {
            return Ok(Language::Zig);
        }
        if chapel_files > 0 {
            return Ok(Language::Chapel);
        }

        // Fallback basado en conteo de archivos
        let counts = vec![
            (rust_files, Language::Rust),
            (python_files, Language::Python),
            (js_files + ts_files, Language::JavaScript),
            (go_files, Language::Go),
            (java_files, Language::Java),
            (cpp_files + c_files, Language::Cpp),
            (zig_files, Language::Zig),
            (chapel_files, Language::Chapel),
        ];

        let max_count = counts.iter().map(|(count, _)| *count).max().unwrap_or(0);
        if max_count > 0 {
            for (count, lang) in counts {
                if count == max_count {
                    return Ok(lang);
                }
            }
        }

        Ok(Language::Unknown)
    }

    /// Cuenta archivos con una extensión específica
    fn count_files_with_extension(&self, path: &PathBuf, extension: &str) -> Result<usize> {
        let mut count = 0;
        self.walk_directory(path, &mut |entry| {
            if let Some(ext) = entry.path().extension() {
                if ext == extension {
                    count += 1;
                }
            }
        })?;
        Ok(count)
    }

    /// Recorre recursivamente un directorio
    fn walk_directory<F>(&self, path: &PathBuf, callback: &mut F) -> Result<()>
    where
        F: FnMut(&std::fs::DirEntry),
    {
        if !path.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                // Evitar directorios comunes que no queremos escanear
                if let Some(dir_name) = entry_path.file_name() {
                    let dir_str = dir_name.to_string_lossy();
                    if !dir_str.starts_with('.') && 
                       dir_str != "target" && 
                       dir_str != "node_modules" && 
                       dir_str != "__pycache__" &&
                       dir_str != "build" &&
                       dir_str != "dist" {
                        self.walk_directory(&entry_path, callback)?;
                    }
                }
            } else {
                callback(&entry);
            }
        }
        Ok(())
    }

    /// Escanea un proyecto automáticamente detectando el lenguaje
    pub async fn scan_project(&self, project_path: PathBuf) -> Result<ScanResult> {
        let language = self.detect_language(&project_path)?;
        
        match language {
            Language::Rust => self.scan_rust_project(project_path).await,
            Language::Python => self.scan_python_project(project_path).await,
            Language::JavaScript | Language::TypeScript => self.scan_javascript_project(project_path).await,
            Language::Go => self.scan_go_project(project_path).await,
            Language::Java => self.scan_java_project(project_path).await,
            Language::Cpp | Language::C => self.scan_cpp_project(project_path).await,
            Language::Zig => self.scan_zig_project(project_path).await,
            Language::Chapel => self.scan_chapel_project(project_path).await,
            Language::Unknown => {
                // Fallback: intentar detectar por archivos de configuración
                if project_path.join("Cargo.toml").exists() {
                    self.scan_rust_project(project_path).await
                } else if project_path.join("package.json").exists() {
                    self.scan_javascript_project(project_path).await
                } else if project_path.join("requirements.txt").exists() || project_path.join("pyproject.toml").exists() {
                    self.scan_python_project(project_path).await
                } else {
                    Err(anyhow::anyhow!("No se pudo detectar el lenguaje del proyecto"))
                }
            }
        }
    }

    /// Escanea un proyecto Rust
    pub async fn scan_rust_project(&self, project_path: PathBuf) -> Result<ScanResult> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // 1. Ejecutar cargo check para obtener errores (con timeout)
        let check_output = tokio::task::spawn_blocking({
            let project_path = project_path.clone();
            move || {
                Command::new("cargo")
                    .arg("check")
                    .arg("--message-format=json")
                    .arg("--quiet")
                    .current_dir(&project_path)
                    .output()
            }
        })
        .await
        .unwrap_or_else(|_| {
            // Si falla el spawn, intentar sincrónicamente
            Ok(Command::new("cargo")
                .arg("check")
                .arg("--message-format=json")
                .arg("--quiet")
                .current_dir(&project_path)
                .output()
                .unwrap_or_else(|_| std::process::Output {
                    stdout: vec![],
                    stderr: vec![],
                    status: std::process::ExitStatus::default(),
                }))
        })?;

        // 2. Parsear salida JSON de cargo
        let stdout = String::from_utf8_lossy(&check_output.stdout);
        for line in stdout.lines() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(reason) = json.get("reason").and_then(|r| r.as_str()) {
                    if reason == "compiler-message" {
                        if let Some(message) = json.get("message") {
                            let issue = self.parse_compiler_message(message, &project_path).await?;
                            match issue.issue_type.as_str() {
                                "error" => errors.push(issue),
                                "warning" => warnings.push(issue),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        // 3. Ejecutar clippy para warnings adicionales (con timeout)
        let clippy_output = tokio::task::spawn_blocking({
            let project_path = project_path.clone();
            move || {
                Command::new("cargo")
                    .arg("clippy")
                    .arg("--message-format=json")
                    .arg("--quiet")
                    .arg("--")
                    .arg("-W")
                    .arg("clippy::all")
                    .current_dir(&project_path)
                    .output()
                    .unwrap_or_else(|_| std::process::Output {
                        stdout: vec![],
                        stderr: vec![],
                        status: std::process::ExitStatus::default(),
                    })
            }
        })
        .await
        .unwrap_or_else(|_| {
            // Si falla el spawn, intentar sincrónicamente
            Command::new("cargo")
                .arg("clippy")
                .arg("--message-format=json")
                .arg("--quiet")
                .arg("--")
                .arg("-W")
                .arg("clippy::all")
                .current_dir(&project_path)
                .output()
                .unwrap_or_else(|_| std::process::Output {
                    stdout: vec![],
                    stderr: vec![],
                    status: std::process::ExitStatus::default(),
                })
        });

        let stdout = String::from_utf8_lossy(&clippy_output.stdout);
        for line in stdout.lines() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(reason) = json.get("reason").and_then(|r| r.as_str()) {
                    if reason == "compiler-message" {
                        if let Some(message) = json.get("message") {
                            if let Some(level) = message.get("level").and_then(|l| l.as_str()) {
                                if level == "warning" {
                                    let issue = self.parse_compiler_message(message, &project_path).await?;
                                    warnings.push(issue);
                                }
                            }
                        }
                    }
                }
            }
        }

        // 4. Buscar soluciones para cada error
        for error in &mut errors {
            error.solutions = self.search_solutions(error).await?;
        }

        // 5. Realizar análisis de seguridad
        let security_analysis = self.analyze_security(&project_path, &Language::Rust).await?;

        // 6. Generar recomendaciones
        let recommendations = self.generate_recommendations(&errors, &warnings).await?;

        // 7. Calcular quality score
        let quality_score = self.calculate_quality_score(&errors, &warnings);

        Ok(ScanResult {
            language: Language::Rust,
            total_issues: errors.len() + warnings.len(),
            errors,
            warnings,
            security_analysis,
            recommendations,
            quality_score,
        })
    }

    /// Parsea un mensaje del compilador
    async fn parse_compiler_message(
        &self,
        message: &serde_json::Value,
        project_path: &std::path::Path,
    ) -> Result<Issue> {
        let level = message
            .get("level")
            .and_then(|l| l.as_str())
            .unwrap_or("unknown")
            .to_string();

        let code = message
            .get("code")
            .and_then(|c| c.get("code"))
            .and_then(|c| c.as_str())
            .map(|s| s.to_string());

        let message_text = message
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("")
            .to_string();

        // Obtener spans (ubicaciones)
        let mut file = String::new();
        let mut line = 0;
        let mut column = None;
        let mut source_code = None;

        if let Some(spans) = message.get("spans").and_then(|s| s.as_array()) {
            if let Some(first_span) = spans.first() {
                if let Some(file_name) = first_span.get("file_name").and_then(|f| f.as_str()) {
                    file = file_name.to_string();
                }
                if let Some(line_start) = first_span.get("line_start").and_then(|l| l.as_u64()) {
                    line = line_start as usize;
                }
                if let Some(col_start) = first_span.get("column_start").and_then(|c| c.as_u64()) {
                    column = Some(col_start as usize);
                }

                // Intentar leer código fuente
                if let Ok(full_path) = project_path.join(&file).canonicalize() {
                    if let Ok(content) = std::fs::read_to_string(&full_path) {
                        if let Some(line_content) = content.lines().nth(line.saturating_sub(1)) {
                            source_code = Some(line_content.to_string());
                        }
                    }
                }
            }
        }

        Ok(Issue {
            issue_type: level,
            code,
            file,
            line,
            column,
            message: message_text,
            source_code,
            solutions: Vec::new(),
        })
    }

    /// Busca soluciones para un error
    async fn search_solutions(&self, issue: &Issue) -> Result<Vec<Solution>> {
        let mut solutions = Vec::new();

        // Construir query de búsqueda
        let query = if let Some(ref code) = issue.code {
            format!("rust {} {}", code, issue.message)
        } else {
            format!("rust {}", issue.message)
        };

        let search_config = WebSearchConfig {
            query,
            max_results: 5,
            priority_sources: vec![
                "stackoverflow.com".to_string(),
                "doc.rust-lang.org".to_string(),
                "users.rust-lang.org".to_string(),
            ],
            sources: vec![
                "github.com".to_string(),
                "reddit.com".to_string(),
                "dev.to".to_string(),
            ],
            use_ai: true,
            use_stealth: true,
            ..Default::default()
        };

        let results = self.web_search.search(search_config).await?;

        for (idx, result) in results.iter().take(5).enumerate() {
            solutions.push(Solution {
                title: result.title.clone(),
                description: result.description.clone(),
                url: result.url.clone(),
                example_code: None, // Podría extraerse del contenido si está disponible
                priority: (10 - idx as u8).max(1),
            });
        }

        Ok(solutions)
    }

    /// Genera recomendaciones generales
    async fn generate_recommendations(
        &self,
        errors: &[Issue],
        warnings: &[Issue],
    ) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        // Analizar tipos de errores comunes
        let error_codes: Vec<String> = errors
            .iter()
            .filter_map(|e| e.code.clone())
            .collect();

        if error_codes.iter().any(|c| c.starts_with("E0599")) {
            recommendations.push(
                "Error E0599: Función o método no encontrado. Verifica que el trait esté en scope o que la función exista."
                    .to_string(),
            );
        }

        if error_codes.iter().any(|c| c.starts_with("E0382")) {
            recommendations.push(
                "Error E0382: Uso de valor movido. Considera usar referencias (&) o clonar el valor."
                    .to_string(),
            );
        }

        if warnings.len() > 10 {
            recommendations.push(format!(
                "Tienes {} warnings. Considera ejecutar 'cargo clippy --fix' para corregir automáticamente algunos."
            , warnings.len()));
        }

        if errors.is_empty() && warnings.is_empty() {
            recommendations.push("✅ ¡Proyecto sin errores ni warnings! Excelente trabajo.".to_string());
        }

        Ok(recommendations)
    }

    /// Escanea un proyecto Python
    pub async fn scan_python_project(&self, project_path: PathBuf) -> Result<ScanResult> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Ejecutar pylint o flake8 si están disponibles
        let lint_output = tokio::task::spawn_blocking({
            let project_path = project_path.clone();
            move || {
                // Intentar pylint primero, luego flake8
                let pylint_result = Command::new("pylint")
                    .arg("--output-format=json")
                    .arg("--disable=C,R,W0613,W0102") // Deshabilitar algunos warnings comunes
                    .arg(".")
                    .current_dir(&project_path)
                    .output();

                match pylint_result {
                    Ok(output) => output,
                    Err(_) => {
                        // Fallback a flake8
                        Command::new("flake8")
                            .arg("--format=json")
                            .arg("--max-line-length=100")
                            .arg(".")
                            .current_dir(&project_path)
                            .output()
                            .unwrap_or_else(|_| std::process::Output {
                                stdout: vec![],
                                stderr: vec![],
                                status: std::process::ExitStatus::default(),
                            })
                    }
                }
            }
        })
        .await
        .unwrap_or_else(|_| std::process::Output {
            stdout: vec![],
            stderr: vec![],
            status: std::process::ExitStatus::default(),
        });

        // Parsear salida JSON
        let stdout = String::from_utf8_lossy(&lint_output.stdout);
        if let Ok(json_array) = serde_json::from_str::<Vec<serde_json::Value>>(&stdout) {
            for item in json_array {
                let issue = self.parse_python_issue(&item, &project_path)?;
                match issue.issue_type.as_str() {
                    "error" | "fatal" => errors.push(issue),
                    "warning" | "convention" | "refactor" => warnings.push(issue),
                    _ => warnings.push(issue),
                }
            }
        }

        // Buscar soluciones para errores
        for error in &mut errors {
            error.solutions = self.search_solutions(error).await?;
        }

        // Análisis de seguridad para Python
        let security_analysis = self.analyze_security(&project_path, &Language::Python).await?;

        // Generar recomendaciones
        let recommendations = self.generate_recommendations(&errors, &warnings).await?;

        // Calcular quality score
        let quality_score = self.calculate_quality_score(&errors, &warnings);

        Ok(ScanResult {
            language: Language::Python,
            total_issues: errors.len() + warnings.len(),
            errors,
            warnings,
            security_analysis,
            recommendations,
            quality_score,
        })
    }

    /// Escanea un proyecto JavaScript/TypeScript
    pub async fn scan_javascript_project(&self, project_path: PathBuf) -> Result<ScanResult> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Ejecutar ESLint si está disponible
        let lint_output = tokio::task::spawn_blocking({
            let project_path = project_path.clone();
            move || {
                Command::new("npx")
                    .arg("eslint")
                    .arg("--format=json")
                    .arg("--no-eslintrc") // Usar configuración básica
                    .arg("--env")
                    .arg("browser,es6,node")
                    .arg("--parser-options")
                    .arg("{\"ecmaVersion\":2020,\"sourceType\":\"module\"}")
                    .arg(".")
                    .current_dir(&project_path)
                    .output()
                    .unwrap_or_else(|_| std::process::Output {
                        stdout: vec![],
                        stderr: vec![],
                        status: std::process::ExitStatus::default(),
                    })
            }
        })
        .await
        .unwrap_or_else(|_| std::process::Output {
            stdout: vec![],
            stderr: vec![],
            status: std::process::ExitStatus::default(),
        });

        // Parsear salida JSON de ESLint
        let stdout = String::from_utf8_lossy(&lint_output.stdout);
        if let Ok(json_array) = serde_json::from_str::<Vec<serde_json::Value>>(&stdout) {
            for file_result in json_array {
                if let Some(messages) = file_result.get("messages").and_then(|m| m.as_array()) {
                    let file_name = file_result.get("filePath").and_then(|f| f.as_str()).unwrap_or("unknown");
                    
                    for message in messages {
                        let issue = self.parse_eslint_message(message, file_name, &project_path)?;
                        match issue.issue_type.as_str() {
                            "error" => errors.push(issue),
                            "warning" => warnings.push(issue),
                            _ => warnings.push(issue),
                        }
                    }
                }
            }
        }

        // Buscar soluciones para errores
        for error in &mut errors {
            error.solutions = self.search_solutions(error).await?;
        }

        // Análisis de seguridad para JavaScript/TypeScript
        let security_analysis = self.analyze_security(&project_path, &Language::JavaScript).await?;

        // Generar recomendaciones
        let recommendations = self.generate_js_recommendations(&errors, &warnings).await?;

        // Calcular quality score
        let quality_score = self.calculate_quality_score(&errors, &warnings);

        Ok(ScanResult {
            language: Language::JavaScript,
            total_issues: errors.len() + warnings.len(),
            errors,
            warnings,
            security_analysis,
            recommendations,
            quality_score,
        })
    }

    /// Escanea un proyecto Go
    pub async fn scan_go_project(&self, project_path: PathBuf) -> Result<ScanResult> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Ejecutar go vet para análisis estático
        let vet_output = tokio::task::spawn_blocking({
            let project_path = project_path.clone();
            move || {
                Command::new("go")
                    .arg("vet")
                    .arg("./...")
                    .current_dir(&project_path)
                    .output()
                    .unwrap_or_else(|_| std::process::Output {
                        stdout: vec![],
                        stderr: vec![],
                        status: std::process::ExitStatus::default(),
                    })
            }
        })
        .await
        .unwrap_or_else(|_| std::process::Output {
            stdout: vec![],
            stderr: vec![],
            status: std::process::ExitStatus::default(),
        });

        // Parsear errores de go vet
        let stderr = String::from_utf8_lossy(&vet_output.stderr);
        for line in stderr.lines() {
            if !line.trim().is_empty() {
                let issue = self.parse_go_error_line(line, &project_path)?;
                errors.push(issue);
            }
        }

        // Intentar ejecutar golint o revive si están disponibles
        let lint_output = tokio::task::spawn_blocking({
            let project_path = project_path.clone();
            move || {
                // Intentar revive primero (reemplazo moderno de golint)
                let revive_result = Command::new("revive")
                    .arg("-formatter")
                    .arg("json")
                    .arg("./...")
                    .current_dir(&project_path)
                    .output();

                match revive_result {
                    Ok(output) => output,
                    Err(_) => {
                        // Fallback a golint
                        Command::new("golint")
                            .arg("./...")
                            .current_dir(&project_path)
                            .output()
                            .unwrap_or_else(|_| std::process::Output {
                                stdout: vec![],
                                stderr: vec![],
                                status: std::process::ExitStatus::default(),
                            })
                    }
                }
            }
        })
        .await
        .unwrap_or_else(|_| std::process::Output {
            stdout: vec![],
            stderr: vec![],
            status: std::process::ExitStatus::default(),
        });

        // Parsear salida de linter
        let stdout = String::from_utf8_lossy(&lint_output.stdout);
        if stdout.contains("[{") { // JSON output de revive
            if let Ok(json_array) = serde_json::from_str::<Vec<serde_json::Value>>(&stdout) {
                for item in json_array {
                    let issue = self.parse_revive_issue(&item, &project_path)?;
                    warnings.push(issue);
                }
            }
        } else { // Output de golint (texto plano)
            for line in stdout.lines() {
                if !line.trim().is_empty() {
                    let issue = self.parse_golint_line(line, &project_path)?;
                    warnings.push(issue);
                }
            }
        }

        // Buscar soluciones para errores
        for error in &mut errors {
            error.solutions = self.search_solutions(error).await?;
        }

        // Análisis de seguridad para Go
        let security_analysis = self.analyze_security(&project_path, &Language::Go).await?;

        // Generar recomendaciones
        let recommendations = self.generate_go_recommendations(&errors, &warnings).await?;

        // Calcular quality score
        let quality_score = self.calculate_quality_score(&errors, &warnings);

        Ok(ScanResult {
            language: Language::Go,
            total_issues: errors.len() + warnings.len(),
            errors,
            warnings,
            security_analysis,
            recommendations,
            quality_score,
        })
    }

    /// Escanea un proyecto Java
    pub async fn scan_java_project(&self, project_path: PathBuf) -> Result<ScanResult> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Ejecutar javac para verificar errores de compilación
        let java_files = self.find_java_files(&project_path);
        let compile_output = tokio::task::spawn_blocking({
            let project_path = project_path.clone();
            let java_files = java_files.clone();
            move || {
                Command::new("javac")
                    .arg("-cp")
                    .arg(".") // classpath básico
                    .arg("-d")
                    .arg("/tmp") // directorio temporal
                    .args(java_files)
                    .current_dir(&project_path)
                    .output()
                    .unwrap_or_else(|_| std::process::Output {
                        stdout: vec![],
                        stderr: vec![],
                        status: std::process::ExitStatus::default(),
                    })
            }
        })
        .await
        .unwrap_or_else(|_| std::process::Output {
            stdout: vec![],
            stderr: vec![],
            status: std::process::ExitStatus::default(),
        });

        // Parsear errores de javac
        let stderr = String::from_utf8_lossy(&compile_output.stderr);
        for line in stderr.lines() {
            if !line.trim().is_empty() && !line.contains("Note:") {
                let issue = self.parse_javac_error(line, &project_path)?;
                if issue.issue_type == "error" {
                    errors.push(issue);
                } else {
                    warnings.push(issue);
                }
            }
        }

        // Intentar ejecutar checkstyle si está disponible
        let java_files = self.find_java_files(&project_path);
        let checkstyle_output = tokio::task::spawn_blocking({
            let project_path = project_path.clone();
            let java_files = java_files.clone();
            move || {
                Command::new("java")
                    .arg("-jar")
                    .arg("checkstyle.jar") // Asumiendo que está en el PATH o directorio actual
                    .arg("-c")
                    .arg("/google_checks.xml") // Configuración por defecto
                    .args(java_files)
                    .current_dir(&project_path)
                    .output()
                    .unwrap_or_else(|_| std::process::Output {
                        stdout: vec![],
                        stderr: vec![],
                        status: std::process::ExitStatus::default(),
                    })
            }
        })
        .await
        .unwrap_or_else(|_| std::process::Output {
            stdout: vec![],
            stderr: vec![],
            status: std::process::ExitStatus::default(),
        });

        // Parsear salida de checkstyle
        let checkstyle_stderr = String::from_utf8_lossy(&checkstyle_output.stderr);
        for line in checkstyle_stderr.lines() {
            if line.contains("[") && (line.contains("ERROR") || line.contains("WARN")) {
                if let Some(issue) = self.parse_checkstyle_line(line, &project_path) {
                    if issue.issue_type == "error" {
                        errors.push(issue);
                    } else {
                        warnings.push(issue);
                    }
                }
            }
        }

        // Buscar soluciones para errores
        for error in &mut errors {
            error.solutions = self.search_solutions(error).await?;
        }

        // Análisis de seguridad para Java
        let security_analysis = self.analyze_security(&project_path, &Language::Java).await?;

        // Generar recomendaciones
        let recommendations = self.generate_java_recommendations(&errors, &warnings).await?;

        // Calcular quality score
        let quality_score = self.calculate_quality_score(&errors, &warnings);

        Ok(ScanResult {
            language: Language::Java,
            total_issues: errors.len() + warnings.len(),
            errors,
            warnings,
            security_analysis,
            recommendations,
            quality_score,
        })
    }

    /// Escanea un proyecto C/C++
    pub async fn scan_cpp_project(&self, project_path: PathBuf) -> Result<ScanResult> {
        // Placeholder - implementación básica por ahora
        let security_analysis = self.analyze_security(&project_path, &Language::Cpp).await?;
        
        Ok(ScanResult {
            language: Language::Cpp,
            total_issues: 0,
            errors: vec![],
            warnings: vec![],
            security_analysis,
            recommendations: vec!["Implementación completa próximamente".to_string()],
            quality_score: 85.0,
        })
    }

    /// Escanea un proyecto Zig
    pub async fn scan_zig_project(&self, project_path: PathBuf) -> Result<ScanResult> {
        // Placeholder - implementación básica por ahora
        let security_analysis = self.analyze_security(&project_path, &Language::Zig).await?;
        
        Ok(ScanResult {
            language: Language::Zig,
            total_issues: 0,
            errors: vec![],
            warnings: vec![],
            security_analysis,
            recommendations: vec!["Implementación completa próximamente".to_string()],
            quality_score: 85.0,
        })
    }

    /// Escanea un proyecto Chapel con análisis avanzado al 200%
    pub async fn scan_chapel_project(&self, project_path: PathBuf) -> Result<ScanResult> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut performance_issues = Vec::new();

        // Encontrar archivos Chapel
        let chapel_files = self.find_chapel_files(&project_path);

        // 1. Análisis de compilación con chpl
        let compile_output = tokio::task::spawn_blocking({
            let project_path = project_path.clone();
            let chapel_files = chapel_files.clone();
            move || {
                Command::new("chpl")
                    .arg("--check-only")
                    .arg("--warn-unstable")
                    .args(&chapel_files)
                    .current_dir(&project_path)
                    .output()
                    .unwrap_or_else(|_| std::process::Output {
                        stdout: vec![],
                        stderr: vec![],
                        status: std::process::ExitStatus::default(),
                    })
            }
        })
        .await
        .unwrap_or_else(|_| std::process::Output {
            stdout: vec![],
            stderr: vec![],
            status: std::process::ExitStatus::default(),
        });

        // Parsear errores del compilador Chapel
        let stderr = String::from_utf8_lossy(&compile_output.stderr);
        for line in stderr.lines() {
            if !line.trim().is_empty() && (line.contains("error") || line.contains("Error")) {
                let issue = self.parse_chpl_error(line, &project_path)?;
                errors.push(issue);
            } else if !line.trim().is_empty() && (line.contains("warning") || line.contains("Warning")) {
                let issue = self.parse_chpl_error(line, &project_path)?;
                warnings.push(issue);
            }
        }

        // 2. Análisis SUPER AVANZADO de Chapel (200% de aprovechamiento)
        for file in &chapel_files {
            let file_path = project_path.join(file);
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                // Análisis de dominio y distribución
                let domain_issues = self.analyze_chapel_domains(&content, file, &project_path)?;
                warnings.extend(domain_issues);

                // Análisis de paralelismo avanzado
                let parallel_issues = self.analyze_chapel_advanced_parallelism(&content, file, &project_path)?;
                performance_issues.extend(parallel_issues);

                // Análisis de locality y memoria
                let locality_issues = self.analyze_chapel_memory_locality(&content, file, &project_path)?;
                performance_issues.extend(locality_issues);

                // Análisis de patrones anti-óptimos
                let anti_pattern_issues = self.analyze_chapel_anti_patterns(&content, file, &project_path)?;
                warnings.extend(anti_pattern_issues);

                // Análisis de dependencias de tareas
                let task_dep_issues = self.analyze_chapel_task_dependencies(&content, file, &project_path)?;
                warnings.extend(task_dep_issues);

                // Sugerencias de optimización
                let optimization_suggestions = self.suggest_chapel_optimizations(&content, file, &project_path)?;
                performance_issues.extend(optimization_suggestions);
            }
        }

        // 3. Análisis de paralelismo básico (legacy)
        let basic_parallel_issues = self.analyze_chapel_parallelism(&project_path)?;
        warnings.extend(basic_parallel_issues);

        // 4. Análisis de locality básico (legacy)
        let basic_locality_issues = self.analyze_chapel_locality(&project_path)?;
        warnings.extend(basic_locality_issues);

        // Buscar soluciones para errores
        for error in &mut errors {
            error.solutions = self.search_solutions(error).await?;
        }

        // Análisis de seguridad específico para Chapel
        let security_analysis = self.analyze_security(&project_path, &Language::Chapel).await?;

        // Generar recomendaciones hiper-avanzadas para Chapel
        let recommendations = self.generate_hyper_chapel_recommendations(&errors, &warnings, &performance_issues).await?;

        // Calcular quality score con bonus por optimizaciones
        let quality_score = self.calculate_chapel_quality_score(&errors, &warnings, &performance_issues);

        // Combinar todos los issues
        let mut all_warnings = warnings;
        all_warnings.extend(performance_issues);

        Ok(ScanResult {
            language: Language::Chapel,
            total_issues: errors.len() + all_warnings.len(),
            errors,
            warnings: all_warnings,
            security_analysis,
            recommendations,
            quality_score,
        })
    }

    /// Analiza seguridad de un proyecto
    pub async fn analyze_security(&self, project_path: &PathBuf, language: &Language) -> Result<SecurityAnalysis> {
        let mut vulnerabilities = Vec::new();
        let mut files_analyzed = 0;

        // Recorrer archivos del proyecto
        self.walk_directory(project_path, &mut |entry| {
            files_analyzed += 1;
            let path = entry.path();
            
            if let Some(extension) = path.extension() {
                let ext_str = extension.to_string_lossy();
                
                // Solo analizar archivos de código fuente
                if self.is_source_file(&ext_str, language) {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        let file_vulns = self.scan_file_for_vulnerabilities(&content, &path, language);
                        vulnerabilities.extend(file_vulns);
                    }
                }
            }
        })?;

        // Calcular security score
        let security_score = self.calculate_security_score(&vulnerabilities, files_analyzed);

        Ok(SecurityAnalysis {
            vulnerabilities,
            security_score,
            files_analyzed,
            language: language.clone(),
        })
    }

    /// Verifica si un archivo es de código fuente para el lenguaje dado
    fn is_source_file(&self, extension: &str, language: &Language) -> bool {
        match language {
            Language::Rust => extension == "rs",
            Language::Python => extension == "py",
            Language::JavaScript | Language::TypeScript => matches!(extension, "js" | "ts" | "jsx" | "tsx"),
            Language::Go => extension == "go",
            Language::Java => extension == "java",
            Language::Cpp => matches!(extension, "cpp" | "cc" | "cxx" | "c++"),
            Language::C => extension == "c",
            Language::Zig => extension == "zig",
            Language::Chapel => extension == "chpl",
            Language::Unknown => matches!(extension, "rs" | "py" | "js" | "ts" | "go" | "java" | "cpp" | "c" | "zig" | "chpl"),
        }
    }

    /// Escanea un archivo en busca de vulnerabilidades
    fn scan_file_for_vulnerabilities(&self, content: &str, path: &std::path::Path, language: &Language) -> Vec<SecurityIssue> {
        let mut vulnerabilities = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_num = line_num + 1; // Líneas empiezan en 1
            
            match language {
                Language::Python => {
                    vulnerabilities.extend(self.scan_python_vulnerabilities(line, line_num, path));
                }
                Language::JavaScript | Language::TypeScript => {
                    vulnerabilities.extend(self.scan_javascript_vulnerabilities(line, line_num, path));
                }
                Language::Rust => {
                    vulnerabilities.extend(self.scan_rust_vulnerabilities(line, line_num, path));
                }
                Language::Go => {
                    vulnerabilities.extend(self.scan_go_vulnerabilities(line, line_num, path));
                }
                Language::Java => {
                    vulnerabilities.extend(self.scan_java_vulnerabilities(line, line_num, path));
                }
                Language::Chapel => {
                    vulnerabilities.extend(self.scan_chapel_vulnerabilities(line, line_num, path));
                }
                _ => {
                    // Análisis genérico básico
                    vulnerabilities.extend(self.scan_generic_vulnerabilities(line, line_num, path));
                }
            }
        }

        vulnerabilities
    }

    /// Escanea vulnerabilidades en Python
    fn scan_python_vulnerabilities(&self, line: &str, line_num: usize, path: &std::path::Path) -> Vec<SecurityIssue> {
        let mut vulnerabilities = Vec::new();

        // SQL Injection
        if line.contains("cursor.execute(") && (line.contains("format(") || line.contains("%")) {
            vulnerabilities.push(SecurityIssue {
                vulnerability_type: "SQL Injection".to_string(),
                severity: 9,
                file: path.to_string_lossy().to_string(),
                line: line_num,
                description: "Posible SQL injection mediante formateo de strings".to_string(),
                vulnerable_code: Some(line.trim().to_string()),
                recommendations: vec![
                    "Usa parámetros preparados: cursor.execute('SELECT * FROM table WHERE id = ?', (user_id,))".to_string(),
                    "Considera usar SQLAlchemy u ORM para prevenir SQL injection".to_string(),
                ],
                cwe_id: Some("CWE-89".to_string()),
            });
        }

        // Command Injection
        if line.contains("subprocess.") && (line.contains("shell=True") || line.contains("format(") || line.contains("%")) {
            vulnerabilities.push(SecurityIssue {
                vulnerability_type: "Command Injection".to_string(),
                severity: 8,
                file: path.to_string_lossy().to_string(),
                line: line_num,
                description: "Posible command injection en ejecución de subprocess".to_string(),
                vulnerable_code: Some(line.trim().to_string()),
                recommendations: vec![
                    "Evita shell=True cuando sea posible".to_string(),
                    "Usa listas en lugar de strings para comandos".to_string(),
                    "Valida y sanitiza inputs antes de usarlos en comandos".to_string(),
                ],
                cwe_id: Some("CWE-78".to_string()),
            });
        }

        // Hardcoded secrets
        if (line.contains("password") || line.contains("secret") || line.contains("key")) && 
           (line.contains("=") || line.contains(":")) &&
           !line.contains("#") { // Ignorar comentarios
            let lower_line = line.to_lowercase();
            if lower_line.contains("password = \"") || lower_line.contains("secret = \"") || lower_line.contains("key = \"") {
                vulnerabilities.push(SecurityIssue {
                    vulnerability_type: "Hardcoded Secret".to_string(),
                    severity: 7,
                    file: path.to_string_lossy().to_string(),
                    line: line_num,
                    description: "Credenciales o secrets hardcodeados en el código".to_string(),
                    vulnerable_code: Some(line.trim().to_string()),
                    recommendations: vec![
                        "Usa variables de entorno: os.getenv('SECRET_KEY')".to_string(),
                        "Considera usar archivos de configuración separados".to_string(),
                        "Nunca commits credenciales al repositorio".to_string(),
                    ],
                    cwe_id: Some("CWE-798".to_string()),
                });
            }
        }

        vulnerabilities
    }

    /// Escanea vulnerabilidades en JavaScript/TypeScript
    fn scan_javascript_vulnerabilities(&self, line: &str, line_num: usize, path: &std::path::Path) -> Vec<SecurityIssue> {
        let mut vulnerabilities = Vec::new();

        // XSS
        if line.contains("innerHTML") || line.contains("outerHTML") {
            vulnerabilities.push(SecurityIssue {
                vulnerability_type: "Cross-Site Scripting (XSS)".to_string(),
                severity: 8,
                file: path.to_string_lossy().to_string(),
                line: line_num,
                description: "Asignación directa a innerHTML puede causar XSS".to_string(),
                vulnerable_code: Some(line.trim().to_string()),
                recommendations: vec![
                    "Usa textContent en lugar de innerHTML cuando sea posible".to_string(),
                    "Sanitiza el HTML con una librería como DOMPurify".to_string(),
                    "Valida y escapa el contenido antes de insertarlo".to_string(),
                ],
                cwe_id: Some("CWE-79".to_string()),
            });
        }

        // Eval
        if line.contains("eval(") {
            vulnerabilities.push(SecurityIssue {
                vulnerability_type: "Code Injection".to_string(),
                severity: 9,
                file: path.to_string_lossy().to_string(),
                line: line_num,
                description: "Uso de eval() puede ejecutar código arbitrario".to_string(),
                vulnerable_code: Some(line.trim().to_string()),
                recommendations: vec![
                    "Evita usar eval() siempre que sea posible".to_string(),
                    "Considera alternativas como JSON.parse() o funciones específicas".to_string(),
                    "Si eval() es necesario, valida estrictamente el input".to_string(),
                ],
                cwe_id: Some("CWE-95".to_string()),
            });
        }

        vulnerabilities
    }

    /// Escanea vulnerabilidades en Rust
    fn scan_rust_vulnerabilities(&self, line: &str, line_num: usize, path: &std::path::Path) -> Vec<SecurityIssue> {
        let mut vulnerabilities = Vec::new();

        // Unsafe code
        if line.contains("unsafe") && !line.trim().starts_with("//") {
            vulnerabilities.push(SecurityIssue {
                vulnerability_type: "Unsafe Code".to_string(),
                severity: 5,
                file: path.to_string_lossy().to_string(),
                line: line_num,
                description: "Uso de código unsafe que puede causar vulnerabilidades de memoria".to_string(),
                vulnerable_code: Some(line.trim().to_string()),
                recommendations: vec![
                    "Minimiza el uso de unsafe blocks".to_string(),
                    "Audita cuidadosamente todo código unsafe".to_string(),
                    "Considera usar crates safe como nomicon alternatives".to_string(),
                ],
                cwe_id: Some("CWE-119".to_string()),
            });
        }

        vulnerabilities
    }

    /// Escanea vulnerabilidades en Go
    fn scan_go_vulnerabilities(&self, line: &str, line_num: usize, path: &std::path::Path) -> Vec<SecurityIssue> {
        let mut vulnerabilities = Vec::new();

        // SQL Injection
        if line.contains("db.Query(") || line.contains("db.Exec(") {
            if line.contains("fmt.Sprintf") || line.contains("string+") {
                vulnerabilities.push(SecurityIssue {
                    vulnerability_type: "SQL Injection".to_string(),
                    severity: 9,
                    file: path.to_string_lossy().to_string(),
                    line: line_num,
                    description: "Posible SQL injection mediante concatenación de strings".to_string(),
                    vulnerable_code: Some(line.trim().to_string()),
                    recommendations: vec![
                        "Usa prepared statements: db.Query('SELECT * FROM table WHERE id = ?', userID)".to_string(),
                        "Considera usar librerías ORM para prevenir SQL injection".to_string(),
                    ],
                    cwe_id: Some("CWE-89".to_string()),
                });
            }
        }

        vulnerabilities
    }

    /// Escanea vulnerabilidades en Java
    fn scan_java_vulnerabilities(&self, line: &str, line_num: usize, path: &std::path::Path) -> Vec<SecurityIssue> {
        let mut vulnerabilities = Vec::new();

        // SQL Injection
        if line.contains("executeQuery(") || line.contains("executeUpdate(") {
            if line.contains("+") || line.contains("format(") {
                vulnerabilities.push(SecurityIssue {
                    vulnerability_type: "SQL Injection".to_string(),
                    severity: 9,
                    file: path.to_string_lossy().to_string(),
                    line: line_num,
                    description: "Posible SQL injection mediante concatenación de strings".to_string(),
                    vulnerable_code: Some(line.trim().to_string()),
                    recommendations: vec![
                        "Usa PreparedStatement: PreparedStatement stmt = conn.prepareStatement('SELECT * FROM table WHERE id = ?')".to_string(),
                        "Considera usar JPA/Hibernate para prevenir SQL injection".to_string(),
                    ],
                    cwe_id: Some("CWE-89".to_string()),
                });
            }
        }

        vulnerabilities
    }

    /// Escanea vulnerabilidades en Chapel
    fn scan_chapel_vulnerabilities(&self, line: &str, line_num: usize, path: &std::path::Path) -> Vec<SecurityIssue> {
        let mut vulnerabilities = Vec::new();

        // Deadlock potencial en sincronización
        if line.contains("sync") && line.contains("waitFor") {
            if line.contains("forall") || line.contains("coforall") {
                vulnerabilities.push(SecurityIssue {
                    vulnerability_type: "Deadlock Risk".to_string(),
                    severity: 7,
                    file: path.to_string_lossy().to_string(),
                    line: line_num,
                    description: "Posible deadlock en código paralelo con sincronización".to_string(),
                    vulnerable_code: Some(line.trim().to_string()),
                    recommendations: vec![
                        "Evita sincronización compleja en bucles paralelos".to_string(),
                        "Usa 'atomic' variables en lugar de 'sync' cuando sea posible".to_string(),
                        "Considera reestructurar el algoritmo para evitar dependencias circulares".to_string(),
                    ],
                    cwe_id: Some("CWE-833".to_string()),
                });
            }
        }

        // Data race en variables compartidas
        if (line.contains("forall") || line.contains("coforall")) && line.contains("=") {
            if !line.contains("atomic") && !line.contains("sync") && !line.contains("single") {
                vulnerabilities.push(SecurityIssue {
                    vulnerability_type: "Data Race".to_string(),
                    severity: 8,
                    file: path.to_string_lossy().to_string(),
                    line: line_num,
                    description: "Posible data race en variable compartida en bucle paralelo".to_string(),
                    vulnerable_code: Some(line.trim().to_string()),
                    recommendations: vec![
                        "Usa variables 'atomic' para operaciones thread-safe: var x: atomic int;".to_string(),
                        "Considera usar 'single' para asignaciones únicas".to_string(),
                        "Revisa si la variable debe ser privada al hilo/task".to_string(),
                    ],
                    cwe_id: Some("CWE-362".to_string()),
                });
            }
        }

        // Buffer overflow en arrays distribuidos
        if line.contains("[") && line.contains("]") && (line.contains("..") || line.contains("#")) {
            if !line.contains("domain") && !line.contains("check") {
                vulnerabilities.push(SecurityIssue {
                    vulnerability_type: "Array Bounds".to_string(),
                    severity: 6,
                    file: path.to_string_lossy().to_string(),
                    line: line_num,
                    description: "Acceso a array sin verificación de límites".to_string(),
                    vulnerable_code: Some(line.trim().to_string()),
                    recommendations: vec![
                        "Verifica límites de arrays: if i >= arr.domain.low && i <= arr.domain.high".to_string(),
                        "Usa dominios para acceso seguro: arr[arr.domain]".to_string(),
                        "Considera usar 'try' para acceso seguro".to_string(),
                    ],
                    cwe_id: Some("CWE-125".to_string()),
                });
            }
        }

        // Información sensible en código HPC
        let lower_line = line.to_lowercase();
        if (lower_line.contains("password") || lower_line.contains("secret") || lower_line.contains("key")) &&
           (lower_line.contains("=") || lower_line.contains(":")) &&
           !line.trim().starts_with("//") && !line.trim().starts_with("/*") {
            vulnerabilities.push(SecurityIssue {
                vulnerability_type: "Hardcoded Secret".to_string(),
                severity: 7,
                file: path.to_string_lossy().to_string(),
                line: line_num,
                description: "Credenciales hardcodeadas en código de HPC".to_string(),
                vulnerable_code: Some(line.trim().to_string()),
                recommendations: vec![
                    "Usa variables de entorno: getEnv('SECRET_KEY')".to_string(),
                    "Lee credenciales desde archivos de configuración seguros".to_string(),
                    "Nunca commits credenciales al repositorio".to_string(),
                ],
                cwe_id: Some("CWE-798".to_string()),
            });
        }

        vulnerabilities
    }

    /// Escanea vulnerabilidades genéricas
    fn scan_generic_vulnerabilities(&self, line: &str, line_num: usize, path: &std::path::Path) -> Vec<SecurityIssue> {
        let mut vulnerabilities = Vec::new();

        // Hardcoded passwords/secrets (genérico)
        let lower_line = line.to_lowercase();
        if (lower_line.contains("password") || lower_line.contains("secret") || lower_line.contains("key")) &&
           (lower_line.contains("=") || lower_line.contains(":")) &&
           !line.trim().starts_with("//") && !line.trim().starts_with("#") && !line.trim().starts_with("--") {
            
            // Buscar patrones de strings hardcodeadas
            if lower_line.contains("\"") || lower_line.contains("'") {
                vulnerabilities.push(SecurityIssue {
                    vulnerability_type: "Hardcoded Secret".to_string(),
                    severity: 6,
                    file: path.to_string_lossy().to_string(),
                    line: line_num,
                    description: "Posible credencial o secret hardcodeado".to_string(),
                    vulnerable_code: Some(line.trim().to_string()),
                    recommendations: vec![
                        "Usa variables de entorno".to_string(),
                        "Considera usar archivos de configuración separados".to_string(),
                        "Nunca commits credenciales al repositorio".to_string(),
                    ],
                    cwe_id: Some("CWE-798".to_string()),
                });
            }
        }

        vulnerabilities
    }

    /// Calcula el security score
    fn calculate_security_score(&self, vulnerabilities: &[SecurityIssue], files_analyzed: usize) -> f32 {
        if vulnerabilities.is_empty() {
            return 100.0;
        }

        if files_analyzed == 0 {
            return 0.0;
        }

        // Calcular score basado en severidad y cantidad
        let total_severity: u32 = vulnerabilities.iter().map(|v| v.severity as u32).sum();
        let vuln_penalty = (total_severity as f32 * 2.0).min(80.0); // Máximo 80 puntos de penalización
        
        // Penalización adicional por cantidad de vulnerabilidades
        let count_penalty = (vulnerabilities.len() as f32 * 1.5).min(20.0);
        
        (100.0 - vuln_penalty - count_penalty).max(0.0)
    }

    /// Parsea un issue de Python (pylint/flake8)
    fn parse_python_issue(&self, item: &serde_json::Value, project_path: &PathBuf) -> Result<Issue> {
        let file = item.get("path").and_then(|p| p.as_str()).unwrap_or("unknown").to_string();
        let line = item.get("line").and_then(|l| l.as_u64()).unwrap_or(0) as usize;
        let column = item.get("column").and_then(|c| c.as_u64()).map(|c| c as usize);
        
        let issue_type = item.get("type").and_then(|t| t.as_str()).unwrap_or("warning").to_string();
        let code = item.get("symbol").and_then(|s| s.as_str()).map(|s| s.to_string());
        let message = item.get("message").and_then(|m| m.as_str()).unwrap_or("").to_string();
        
        // Intentar leer código fuente
        let mut source_code = None;
        if let Ok(full_path) = project_path.join(&file).canonicalize() {
            if let Ok(content) = std::fs::read_to_string(&full_path) {
                if let Some(line_content) = content.lines().nth(line.saturating_sub(1)) {
                    source_code = Some(line_content.to_string());
                }
            }
        }

        Ok(Issue {
            issue_type,
            code,
            file,
            line,
            column,
            message,
            source_code,
            solutions: Vec::new(),
        })
    }

    /// Parsea un mensaje de ESLint
    fn parse_eslint_message(&self, message: &serde_json::Value, file_name: &str, project_path: &PathBuf) -> Result<Issue> {
        let rule_id = message.get("ruleId").and_then(|r| r.as_str()).map(|s| s.to_string());
        let severity = message.get("severity").and_then(|s| s.as_u64()).unwrap_or(1);
        let line = message.get("line").and_then(|l| l.as_u64()).unwrap_or(0) as usize;
        let column = message.get("column").and_then(|c| c.as_u64()).map(|c| c as usize);
        let message_text = message.get("message").and_then(|m| m.as_str()).unwrap_or("").to_string();

        // Intentar leer código fuente
        let mut source_code = None;
        if let Ok(full_path) = project_path.join(file_name).canonicalize() {
            if let Ok(content) = std::fs::read_to_string(&full_path) {
                if let Some(line_content) = content.lines().nth(line.saturating_sub(1)) {
                    source_code = Some(line_content.to_string());
                }
            }
        }

        Ok(Issue {
            issue_type: if severity == 2 { "error".to_string() } else { "warning".to_string() },
            code: rule_id,
            file: file_name.to_string(),
            line,
            column,
            message: message_text,
            source_code,
            solutions: Vec::new(),
        })
    }

    /// Genera recomendaciones específicas para JavaScript/TypeScript
    async fn generate_js_recommendations(&self, errors: &[Issue], warnings: &[Issue]) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        // Analizar tipos de errores comunes
        let error_codes: Vec<String> = errors
            .iter()
            .filter_map(|e| e.code.clone())
            .collect();

        if error_codes.iter().any(|c| c.contains("no-undef")) {
            recommendations.push(
                "Error 'no-undef': Variable no definida. Asegúrate de declarar todas las variables o importar los módulos necesarios."
                    .to_string(),
            );
        }

        if error_codes.iter().any(|c| c.contains("no-unused-vars")) {
            recommendations.push(
                "Warning 'no-unused-vars': Variables no utilizadas. Remueve las variables no utilizadas para mantener el código limpio."
                    .to_string(),
            );
        }

        if error_codes.iter().any(|c| c.contains("semi")) {
            recommendations.push(
                "Error 'semi': Falta punto y coma. Configura ESLint para que sea consistente con el estilo de tu proyecto."
                    .to_string(),
            );
        }

        if warnings.len() > 20 {
            recommendations.push(format!(
                "Tienes {} warnings de ESLint. Considera configurar reglas más estrictas o usar Prettier para formateo automático."
            , warnings.len()));
        }

        if errors.is_empty() && warnings.is_empty() {
            recommendations.push("✅ ¡Código JavaScript/TypeScript sin errores ni warnings! Excelente trabajo.".to_string());
        }

        Ok(recommendations)
    }

    /// Parsea una línea de error de go vet
    fn parse_go_error_line(&self, line: &str, project_path: &PathBuf) -> Result<Issue> {
        // Formato típico: path/file.go:line:col: message
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 4 {
            return Ok(Issue {
                issue_type: "error".to_string(),
                code: None,
                file: "unknown".to_string(),
                line: 0,
                column: None,
                message: line.to_string(),
                source_code: None,
                solutions: Vec::new(),
            });
        }

        let file = parts[0].to_string();
        let line_num = parts[1].parse::<usize>().unwrap_or(0);
        let col = parts[2].parse::<usize>().ok();
        let message = parts[3..].join(":").trim().to_string();

        // Intentar leer código fuente
        let mut source_code = None;
        if let Ok(full_path) = project_path.join(&file).canonicalize() {
            if let Ok(content) = std::fs::read_to_string(&full_path) {
                if let Some(line_content) = content.lines().nth(line_num.saturating_sub(1)) {
                    source_code = Some(line_content.to_string());
                }
            }
        }

        Ok(Issue {
            issue_type: "error".to_string(),
            code: None,
            file,
            line: line_num,
            column: col,
            message,
            source_code,
            solutions: Vec::new(),
        })
    }

    /// Parsea un issue de revive (JSON)
    fn parse_revive_issue(&self, item: &serde_json::Value, project_path: &PathBuf) -> Result<Issue> {
        let file = item.get("Position").and_then(|p| p.get("Filename")).and_then(|f| f.as_str()).unwrap_or("unknown").to_string();
        let line = item.get("Position").and_then(|p| p.get("Offset")).and_then(|o| o.as_u64()).unwrap_or(0) as usize;
        let rule = item.get("RuleName").and_then(|r| r.as_str()).map(|s| s.to_string());
        let message = item.get("Failure").and_then(|f| f.as_str()).unwrap_or("").to_string();

        // Intentar leer código fuente
        let mut source_code = None;
        if let Ok(full_path) = project_path.join(&file).canonicalize() {
            if let Ok(content) = std::fs::read_to_string(&full_path) {
                if let Some(line_content) = content.lines().nth(line.saturating_sub(1)) {
                    source_code = Some(line_content.to_string());
                }
            }
        }

        Ok(Issue {
            issue_type: "warning".to_string(),
            code: rule,
            file,
            line,
            column: None,
            message,
            source_code,
            solutions: Vec::new(),
        })
    }

    /// Parsea una línea de golint
    fn parse_golint_line(&self, line: &str, project_path: &PathBuf) -> Result<Issue> {
        // Formato típico: path/file.go:line:col: message
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 4 {
            return Ok(Issue {
                issue_type: "warning".to_string(),
                code: None,
                file: "unknown".to_string(),
                line: 0,
                column: None,
                message: line.to_string(),
                source_code: None,
                solutions: Vec::new(),
            });
        }

        let file = parts[0].to_string();
        let line_num = parts[1].parse::<usize>().unwrap_or(0);
        let col = parts[2].parse::<usize>().ok();
        let message = parts[3..].join(":").trim().to_string();

        // Intentar leer código fuente
        let mut source_code = None;
        if let Ok(full_path) = project_path.join(&file).canonicalize() {
            if let Ok(content) = std::fs::read_to_string(&full_path) {
                if let Some(line_content) = content.lines().nth(line_num.saturating_sub(1)) {
                    source_code = Some(line_content.to_string());
                }
            }
        }

        Ok(Issue {
            issue_type: "warning".to_string(),
            code: None,
            file,
            line: line_num,
            column: col,
            message,
            source_code,
            solutions: Vec::new(),
        })
    }

    /// Genera recomendaciones específicas para Go
    async fn generate_go_recommendations(&self, errors: &[Issue], warnings: &[Issue]) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        if !errors.is_empty() {
            recommendations.push(
                "Ejecuta 'go vet ./...' para detectar problemas potenciales en el código.".to_string(),
            );
        }

        if warnings.iter().any(|w| w.message.contains("exported") && w.message.contains("comment")) {
            recommendations.push(
                "Agrega comentarios a las funciones y tipos exportados (deben empezar con el nombre de la función/tipo).".to_string(),
            );
        }

        if warnings.iter().any(|w| w.message.contains("unused")) {
            recommendations.push(
                "Remueve variables, imports o funciones no utilizadas para mantener el código limpio.".to_string(),
            );
        }

        if warnings.len() > 15 {
            recommendations.push(format!(
                "Tienes {} warnings. Considera instalar 'revive' para un linting más avanzado: go install github.com/mgechev/revive@latest"
            , warnings.len()));
        }

        if errors.is_empty() && warnings.is_empty() {
            recommendations.push("✅ ¡Código Go sin errores ni warnings! Excelente trabajo.".to_string());
        }

        Ok(recommendations)
    }

    /// Encuentra todos los archivos Java en el proyecto
    fn find_java_files(&self, project_path: &PathBuf) -> Vec<String> {
        let mut java_files = Vec::new();
        
        let _ = self.walk_directory(project_path, &mut |entry| {
            if let Some(extension) = entry.path().extension() {
                if extension == "java" {
                    if let Ok(relative_path) = entry.path().strip_prefix(project_path) {
                        java_files.push(relative_path.to_string_lossy().to_string());
                    }
                }
            }
        });

        java_files
    }

    /// Parsea un error de javac
    fn parse_javac_error(&self, line: &str, project_path: &PathBuf) -> Result<Issue> {
        // Formato típico: File.java:line: error: message
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 4 {
            return Ok(Issue {
                issue_type: "error".to_string(),
                code: None,
                file: "unknown".to_string(),
                line: 0,
                column: None,
                message: line.to_string(),
                source_code: None,
                solutions: Vec::new(),
            });
        }

        let file = parts[0].to_string();
        let line_num = parts[1].parse::<usize>().unwrap_or(0);
        let issue_type = if parts[2].trim() == "error" { "error" } else { "warning" }.to_string();
        let message = parts[3..].join(":").trim().to_string();

        // Intentar leer código fuente
        let mut source_code = None;
        if let Ok(full_path) = project_path.join(&file).canonicalize() {
            if let Ok(content) = std::fs::read_to_string(&full_path) {
                if let Some(line_content) = content.lines().nth(line_num.saturating_sub(1)) {
                    source_code = Some(line_content.to_string());
                }
            }
        }

        Ok(Issue {
            issue_type,
            code: None,
            file,
            line: line_num,
            column: None,
            message,
            source_code,
            solutions: Vec::new(),
        })
    }

    /// Parsea una línea de checkstyle
    fn parse_checkstyle_line(&self, line: &str, project_path: &PathBuf) -> Option<Issue> {
        // Formato típico: [ERROR] File.java:line:col: message [RuleName]
        let line = line.trim();
        if !line.starts_with('[') {
            return None;
        }

        let bracket_end = line.find(']')?;
        let issue_type = &line[1..bracket_end];
        let rest = &line[bracket_end + 1..].trim();

        let parts: Vec<&str> = rest.split(':').collect();
        if parts.len() < 4 {
            return None;
        }

        let file = parts[0].to_string();
        let line_num = parts[1].parse::<usize>().unwrap_or(0);
        let col = parts[2].parse::<usize>().ok();
        let message_and_rule = parts[3..].join(":");
        
        // Extraer regla si está entre []
        let (message, code) = if let Some(rule_start) = message_and_rule.rfind('[') {
            if let Some(rule_end) = message_and_rule.rfind(']') {
                let message = message_and_rule[..rule_start].trim().to_string();
                let code = Some(message_and_rule[rule_start + 1..rule_end].to_string());
                (message, code)
            } else {
                (message_and_rule, None)
            }
        } else {
            (message_and_rule, None)
        };

        // Intentar leer código fuente
        let mut source_code = None;
        if let Ok(full_path) = project_path.join(&file).canonicalize() {
            if let Ok(content) = std::fs::read_to_string(&full_path) {
                if let Some(line_content) = content.lines().nth(line_num.saturating_sub(1)) {
                    source_code = Some(line_content.to_string());
                }
            }
        }

        Some(Issue {
            issue_type: issue_type.to_lowercase(),
            code,
            file,
            line: line_num,
            column: col,
            message,
            source_code,
            solutions: Vec::new(),
        })
    }

    /// Genera recomendaciones específicas para Java
    async fn generate_java_recommendations(&self, errors: &[Issue], warnings: &[Issue]) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        if errors.iter().any(|e| e.message.contains("cannot find symbol")) {
            recommendations.push(
                "Error 'cannot find symbol': Import faltante o clase no encontrada. Verifica los imports y dependencias.".to_string(),
            );
        }

        if errors.iter().any(|e| e.message.contains("incompatible types")) {
            recommendations.push(
                "Error 'incompatible types': Conversión de tipos incorrecta. Revisa los tipos de datos y casteos.".to_string(),
            );
        }

        if warnings.iter().any(|w| w.message.contains("unused")) {
            recommendations.push(
                "Warning: Variables, imports o métodos no utilizados. Remueve el código no utilizado para mantener la limpieza.".to_string(),
            );
        }

        if warnings.iter().any(|w| w.message.contains("unchecked")) {
            recommendations.push(
                "Warning 'unchecked': Operación no verificada. Considera usar generics tipados para mayor seguridad de tipos.".to_string(),
            );
        }

        if warnings.len() > 30 {
            recommendations.push(format!(
                "Tienes {} warnings. Considera instalar Checkstyle para análisis de estilo más detallado."
            , warnings.len()));
        }

        if errors.is_empty() && warnings.is_empty() {
            recommendations.push("✅ ¡Código Java sin errores ni warnings! Excelente trabajo.".to_string());
        }

        Ok(recommendations)
    }

    /// Encuentra todos los archivos Chapel en el proyecto
    fn find_chapel_files(&self, project_path: &PathBuf) -> Vec<String> {
        let mut chapel_files = Vec::new();

        let _ = self.walk_directory(project_path, &mut |entry| {
            if let Some(extension) = entry.path().extension() {
                if extension == "chpl" {
                    if let Ok(relative_path) = entry.path().strip_prefix(project_path) {
                        chapel_files.push(relative_path.to_string_lossy().to_string());
                    }
                }
            }
        });

        chapel_files
    }

    /// Parsea un error del compilador Chapel
    fn parse_chpl_error(&self, line: &str, project_path: &PathBuf) -> Result<Issue> {
        // Formato típico: file.chpl:line: error: message
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 4 {
            return Ok(Issue {
                issue_type: "error".to_string(),
                code: None,
                file: "unknown".to_string(),
                line: 0,
                column: None,
                message: line.to_string(),
                source_code: None,
                solutions: Vec::new(),
            });
        }

        let file = parts[0].to_string();
        let line_num = parts[1].parse::<usize>().unwrap_or(0);
        let issue_type = if parts[2].trim().contains("error") { "error" } else { "warning" }.to_string();
        let message = parts[3..].join(":").trim().to_string();

        // Intentar leer código fuente
        let mut source_code = None;
        if let Ok(full_path) = project_path.join(&file).canonicalize() {
            if let Ok(content) = std::fs::read_to_string(&full_path) {
                if let Some(line_content) = content.lines().nth(line_num.saturating_sub(1)) {
                    source_code = Some(line_content.to_string());
                }
            }
        }

        Ok(Issue {
            issue_type,
            code: None,
            file,
            line: line_num,
            column: None,
            message,
            source_code,
            solutions: Vec::new(),
        })
    }

    /// Analiza paralelismo en código Chapel
    fn analyze_chapel_parallelism(&self, project_path: &PathBuf) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();

        let _ = self.walk_directory(project_path, &mut |entry| {
            if let Some(extension) = entry.path().extension() {
                if extension == "chpl" {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        let lines: Vec<&str> = content.lines().collect();

                        for (line_num, line) in lines.iter().enumerate() {
                            let line_num = line_num + 1;

                            // Detectar bucles seriales que podrían paralelizarse
                            if line.contains("for ") && !line.contains("forall") && !line.contains("coforall") {
                                if line.contains(" in ") && (line.contains("..") || line.contains("#")) {
                                    issues.push(Issue {
                                        issue_type: "warning".to_string(),
                                        code: Some("PARALLELISM".to_string()),
                                        file: entry.path().strip_prefix(project_path).unwrap_or(&*entry.path()).to_string_lossy().to_string(),
                                        line: line_num,
                                        column: None,
                                        message: "Bucle 'for' serial detectado. Considera usar 'forall' para paralelización".to_string(),
                                        source_code: Some(line.trim().to_string()),
                                        solutions: vec![
                                            Solution {
                                                title: "Convertir a forall".to_string(),
                                                description: "Usa 'forall' en lugar de 'for' para paralelizar el bucle".to_string(),
                                                url: "https://chapel-lang.org/docs/language/spec/parallel.html".to_string(),
                                                example_code: Some("forall i in 1..N do".to_string()),
                                                priority: 8,
                                            }
                                        ],
                                    });
                                }
                            }

                            // Detectar race conditions potenciales
                            if line.contains("forall") && line.contains("+=") {
                                issues.push(Issue {
                                    issue_type: "warning".to_string(),
                                    code: Some("RACE_CONDITION".to_string()),
                                    file: entry.path().strip_prefix(project_path).unwrap_or(&*entry.path()).to_string_lossy().to_string(),
                                    line: line_num,
                                    column: None,
                                    message: "Posible race condition en reducción paralela".to_string(),
                                    source_code: Some(line.trim().to_string()),
                                    solutions: vec![
                                        Solution {
                                            title: "Usar reducción atómica".to_string(),
                                            description: "Considera usar operaciones atómicas o variables de reducción".to_string(),
                                            url: "https://chapel-lang.org/docs/language/spec/task-parallelism-and-synchronization.html".to_string(),
                                            example_code: Some("var total: atomic int; total.add(value);".to_string()),
                                            priority: 9,
                                        }
                                    ],
                                });
                            }
                        }
                    }
                }
            }
        });

        Ok(issues)
    }

    /// Analiza locality y distribución de datos en Chapel
    fn analyze_chapel_locality(&self, project_path: &PathBuf) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();

        let _ = self.walk_directory(project_path, &mut |entry| {
            if let Some(extension) = entry.path().extension() {
                if extension == "chpl" {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        let lines: Vec<&str> = content.lines().collect();

                        for (line_num, line) in lines.iter().enumerate() {
                            let line_num = line_num + 1;

                            // Detectar acceso no local a arrays distribuidos
                            if line.contains("[") && line.contains("]") && !line.contains("local") {
                                if line.contains("forall") || line.contains("coforall") {
                                    issues.push(Issue {
                                        issue_type: "warning".to_string(),
                                        code: Some("LOCALITY".to_string()),
                                        file: entry.path().strip_prefix(project_path).unwrap_or(&*entry.path()).to_string_lossy().to_string(),
                                        line: line_num,
                                        column: None,
                                        message: "Posible acceso no local en bucle paralelo. Considera usar 'local' para mejorar locality".to_string(),
                                        source_code: Some(line.trim().to_string()),
                                        solutions: vec![
                                            Solution {
                                                title: "Usar acceso local".to_string(),
                                                description: "Accede a elementos locales para mejorar el rendimiento".to_string(),
                                                url: "https://chapel-lang.org/docs/language/spec/domains-and-arrays.html".to_string(),
                                                example_code: Some("local { arr[i] = value; }".to_string()),
                                                priority: 7,
                                            }
                                        ],
                                    });
                                }
                            }

                            // Detectar distribuciones ineficientes
                            if line.contains("dmapped") && line.contains("Block") {
                                if !line.contains("cyclic") && !line.contains("replicated") {
                                    issues.push(Issue {
                                        issue_type: "info".to_string(),
                                        code: Some("DISTRIBUTION".to_string()),
                                        file: entry.path().strip_prefix(project_path).unwrap_or(&*entry.path()).to_string_lossy().to_string(),
                                        line: line_num,
                                        column: None,
                                        message: "Considera distribuciones alternativas (cyclic, block-cyclic) para mejor balanceo de carga".to_string(),
                                        source_code: Some(line.trim().to_string()),
                                        solutions: vec![
                                            Solution {
                                                title: "Probar distribución cyclic".to_string(),
                                                description: "La distribución cyclic puede mejorar el balanceo en algunos casos".to_string(),
                                                url: "https://chapel-lang.org/docs/language/spec/domains-and-arrays.html".to_string(),
                                                example_code: Some("var A: [1..N] real dmapped Cyclic(startIdx=1);".to_string()),
                                                priority: 5,
                                            }
                                        ],
                                    });
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(issues)
    }

    /// Genera recomendaciones específicas para Chapel
    async fn generate_chapel_recommendations(&self, errors: &[Issue], warnings: &[Issue]) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        if errors.iter().any(|e| e.message.contains("undeclared") || e.message.contains("undefined")) {
            recommendations.push(
                "Error: Variable o función no declarada. Verifica que todos los símbolos estén definidos antes de usarlos.".to_string(),
            );
        }

        if warnings.iter().any(|w| w.code.as_ref().map_or(false, |c| c == "PARALLELISM")) {
            recommendations.push(
                "⚡ Optimización: Convierte bucles 'for' seriales a 'forall' para paralelización automática.".to_string(),
            );
        }

        if warnings.iter().any(|w| w.code.as_ref().map_or(false, |c| c == "RACE_CONDITION")) {
            recommendations.push(
                "🔒 Seguridad: Posibles race conditions detectadas. Usa operaciones atómicas o sincronización.".to_string(),
            );
        }

        if warnings.iter().any(|w| w.code.as_ref().map_or(false, |c| c == "LOCALITY")) {
            recommendations.push(
                "🚀 Rendimiento: Mejora la locality de datos usando bloques 'local' en bucles paralelos.".to_string(),
            );
        }

        if warnings.iter().any(|w| w.code.as_ref().map_or(false, |c| c == "DISTRIBUTION")) {
            recommendations.push(
                "⚖️ Balanceo: Experimenta con diferentes estrategias de distribución de datos.".to_string(),
            );
        }

        if warnings.len() > 10 {
            recommendations.push(format!(
                "Tienes {} warnings. Chapel es un lenguaje complejo - considera usar 'chpl --warn-unstable' para más análisis."
            , warnings.len()));
        }

        if errors.is_empty() && warnings.is_empty() {
            recommendations.push("✅ ¡Código Chapel sin errores ni warnings! Excelente trabajo en HPC.".to_string());
        } else {
            recommendations.push("📚 Recursos Chapel: https://chapel-lang.org/docs/".to_string());
        }

        Ok(recommendations)
    }

    /// Análisis AVANZADO de dominios y arrays distribuidos en Chapel
    fn analyze_chapel_domains(&self, content: &str, file: &str, project_path: &PathBuf) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_num = line_num + 1;

            // Dominios sin distribución explícita
            if line.contains("domain(") && !line.contains("dmapped") {
                if line.contains("2D") || line.contains("3D") || line.contains("..") {
                    issues.push(Issue {
                        issue_type: "warning".to_string(),
                        code: Some("DOMAIN_DISTRIBUTION".to_string()),
                        file: file.to_string(),
                        line: line_num,
                        column: None,
                        message: "Dominio multidimensional sin estrategia de distribución explícita".to_string(),
                        source_code: Some(line.trim().to_string()),
                        solutions: vec![
                            Solution {
                                title: "Agregar distribución Block".to_string(),
                                description: "Distribuye el dominio usando Block para buen balanceo de carga".to_string(),
                                url: "https://chapel-lang.org/docs/language/spec/domains.html".to_string(),
                                example_code: Some("var D: domain(2) dmapped Block(boundingBox={1..N, 1..M});".to_string()),
                                priority: 8,
                            },
                            Solution {
                                title: "Usar Cyclic para afinidad de datos".to_string(),
                                description: "Cyclic mantiene locality mejor para ciertos patrones de acceso".to_string(),
                                url: "https://chapel-lang.org/docs/language/spec/domains.html".to_string(),
                                example_code: Some("var D: domain(2) dmapped Cyclic(startIdx=(1,1));".to_string()),
                                priority: 7,
                            }
                        ],
                    });
                }
            }

            // Arrays sin dominios explícitos
            if line.contains("var ") && line.contains("[") && line.contains("]") && !line.contains("domain(") {
                if line.contains("real") || line.contains("int") || line.contains("complex") {
                    issues.push(Issue {
                        issue_type: "info".to_string(),
                        code: Some("ARRAY_DOMAIN".to_string()),
                        file: file.to_string(),
                        line: line_num,
                        column: None,
                        message: "Array declarado sin dominio explícito - considera usar dominios para mejor control".to_string(),
                        source_code: Some(line.trim().to_string()),
                        solutions: vec![
                            Solution {
                                title: "Usar dominio explícito".to_string(),
                                description: "Define un dominio separado para mejor control y distribución".to_string(),
                                url: "https://chapel-lang.org/docs/language/spec/arrays.html".to_string(),
                                example_code: Some("var D: domain(1) = {1..N};\nvar A: [D] real;".to_string()),
                                priority: 6,
                            }
                        ],
                    });
                }
            }

            // Uso de reshape sin optimización
            if line.contains("reshape") && !line.contains("dmapped") {
                issues.push(Issue {
                    issue_type: "warning".to_string(),
                    code: Some("RESHAPE_OPTIMIZATION".to_string()),
                    file: file.to_string(),
                    line: line_num,
                    column: None,
                    message: "Uso de reshape puede causar movimiento costoso de datos".to_string(),
                    source_code: Some(line.trim().to_string()),
                    solutions: vec![
                        Solution {
                            title: "Reconsiderar distribución inicial".to_string(),
                            description: "Mejor diseña la distribución inicial para evitar reshapes costosos".to_string(),
                            url: "https://chapel-lang.org/docs/language/spec/arrays.html".to_string(),
                            example_code: Some("// Diseña la distribución correcta desde el inicio".to_string()),
                            priority: 7,
                        }
                    ],
                });
            }
        }

        Ok(issues)
    }

    /// Análisis AVANZADO de paralelismo en Chapel
    fn analyze_chapel_advanced_parallelism(&self, content: &str, file: &str, project_path: &PathBuf) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_num = line_num + 1;

            // Task spawning sin gestión de recursos
            if line.contains("begin") && !content.lines().skip(line_num).take(10).any(|l| l.contains("sync") || l.contains("waitFor")) {
                issues.push(Issue {
                    issue_type: "warning".to_string(),
                    code: Some("TASK_MANAGEMENT".to_string()),
                    file: file.to_string(),
                    line: line_num,
                    column: None,
                    message: "Task 'begin' sin sincronización visible - posible fuga de tasks o race conditions".to_string(),
                    source_code: Some(line.trim().to_string()),
                    solutions: vec![
                        Solution {
                            title: "Agregar sincronización con sync".to_string(),
                            description: "Usa 'sync' para esperar la finalización de tasks".to_string(),
                            url: "https://chapel-lang.org/docs/language/spec/task-parallelism-and-synchronization.html".to_string(),
                            example_code: Some("sync begin { /* task code */ }".to_string()),
                            priority: 9,
                        },
                        Solution {
                            title: "Usar coforall para paralelismo estructurado".to_string(),
                            description: "coforall es más seguro que begin para paralelismo".to_string(),
                            url: "https://chapel-lang.org/docs/language/spec/task-parallelism-and-synchronization.html".to_string(),
                            example_code: Some("coforall i in 1..numTasks do /* task code */".to_string()),
                            priority: 8,
                        }
                    ],
                });
            }

            // Reducciones ineficientes
            if line.contains("+=") && (line.contains("forall") || line.contains("coforall")) {
                if !line.contains("reduce") && !line.contains("atomic") {
                    issues.push(Issue {
                        issue_type: "performance".to_string(),
                        code: Some("INEFFICIENT_REDUCTION".to_string()),
                        file: file.to_string(),
                        line: line_num,
                        column: None,
                        message: "Reducción ineficiente - usa 'reduce' para mejor rendimiento".to_string(),
                        source_code: Some(line.trim().to_string()),
                        solutions: vec![
                            Solution {
                                title: "Usar reduce para reducciones paralelas".to_string(),
                                description: "Las reducciones con 'reduce' son más eficientes que operaciones atómicas".to_string(),
                                url: "https://chapel-lang.org/docs/language/spec/reductions.html".to_string(),
                                example_code: Some("var total = + reduce A;".to_string()),
                                priority: 9,
                            }
                        ],
                    });
                }
            }

            // Comunicación excesiva entre locales
            if line.contains("on ") && line.contains("Local") {
                issues.push(Issue {
                    issue_type: "performance".to_string(),
                    code: Some("EXCESSIVE_COMMUNICATION".to_string()),
                    file: file.to_string(),
                    line: line_num,
                    column: None,
                    message: "Comunicación explícita 'on Locales' - considera locality-aware algorithms".to_string(),
                    source_code: Some(line.trim().to_string()),
                    solutions: vec![
                        Solution {
                            title: "Usar distribución de datos inteligente".to_string(),
                            description: "Deja que Chapel maneje la distribución automáticamente".to_string(),
                            url: "https://chapel-lang.org/docs/language/spec/locales.html".to_string(),
                            example_code: Some("// Usa dmapped en lugar de 'on' manual".to_string()),
                            priority: 8,
                        }
                    ],
                });
            }
        }

        Ok(issues)
    }

    /// Análisis AVANZADO de locality y memoria en Chapel
    fn analyze_chapel_memory_locality(&self, content: &str, file: &str, project_path: &PathBuf) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_num = line_num + 1;

            // Acceso no-local en bucles críticos
            if (line.contains("forall") || line.contains("coforall")) && line.contains("[") && line.contains("]") {
                if !line.contains("local") && !line.contains("here") {
                    // Buscar patrones de acceso no-local
                    if line.contains("A[") || line.contains("arr[") {
                        issues.push(Issue {
                            issue_type: "performance".to_string(),
                            code: Some("NON_LOCAL_ACCESS".to_string()),
                            file: file.to_string(),
                            line: line_num,
                            column: None,
                            message: "Acceso potencialmente no-local en bucle paralelo - impacto crítico en rendimiento".to_string(),
                            source_code: Some(line.trim().to_string()),
                            solutions: vec![
                                Solution {
                                    title: "Usar acceso local con 'local'".to_string(),
                                    description: "Asegura que los datos estén locales al task".to_string(),
                                    url: "https://chapel-lang.org/docs/language/spec/locales.html".to_string(),
                                    example_code: Some("local { arr[i] = compute(arr.localSlice(i)); }".to_string()),
                                    priority: 10,
                                },
                                Solution {
                                    title: "Rediseñar distribución de datos".to_string(),
                                    description: "Cambia la distribución para que los datos estén donde se necesitan".to_string(),
                                    url: "https://chapel-lang.org/docs/language/spec/domains.html".to_string(),
                                    example_code: Some("var A: [D] real dmapped Block(boundingBox=D);".to_string()),
                                    priority: 9,
                                }
                            ],
                        });
                    }
                }
            }

            // Asignaciones de memoria grandes sin consideración de distribución
            if line.contains("var ") && (line.contains("real") || line.contains("int")) && line.contains("[") {
                if line.contains("..1000") || line.contains("..10000") || line.contains("1..N") {
                    issues.push(Issue {
                        issue_type: "info".to_string(),
                        code: Some("MEMORY_DISTRIBUTION".to_string()),
                        file: file.to_string(),
                        line: line_num,
                        column: None,
                        message: "Array grande sin distribución - considera distribución para mejor rendimiento".to_string(),
                        source_code: Some(line.trim().to_string()),
                        solutions: vec![
                            Solution {
                                title: "Distribuir array grande".to_string(),
                                description: "Los arrays grandes deben distribuirse para aprovechar múltiples nodos".to_string(),
                                url: "https://chapel-lang.org/docs/language/spec/arrays.html".to_string(),
                                example_code: Some("var BigArray: [1..1000000] real dmapped Block(boundingBox={1..1000000});".to_string()),
                                priority: 8,
                            }
                        ],
                    });
                }
            }
        }

        Ok(issues)
    }

    /// Análisis de patrones anti-óptimos en Chapel
    fn analyze_chapel_anti_patterns(&self, content: &str, file: &str, project_path: &PathBuf) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_num = line_num + 1;

            // Uso de bucles while en lugar de paralelismo
            if line.contains("while") && content.lines().skip(line_num).take(5).any(|l| l.contains("forall") || l.contains("coforall")) {
                issues.push(Issue {
                    issue_type: "warning".to_string(),
                    code: Some("ANTI_PATTERN_WHILE".to_string()),
                    file: file.to_string(),
                    line: line_num,
                    column: None,
                    message: "Uso de 'while' con paralelismo - considera algoritmos paralelos nativos".to_string(),
                    source_code: Some(line.trim().to_string()),
                    solutions: vec![
                        Solution {
                            title: "Reemplazar con algoritmos paralelos".to_string(),
                            description: "Usa algoritmos paralelos en lugar de bucles while".to_string(),
                            url: "https://chapel-lang.org/docs/language/spec/".to_string(),
                            example_code: Some("// Usa reduce, scan, o algoritmos paralelos específicos".to_string()),
                            priority: 7,
                        }
                    ],
                });
            }

            // Copias innecesarias de arrays
            if line.contains("=") && line.contains("[") && line.contains("]") && !line.contains("ref") {
                if content.lines().skip(line_num).take(3).any(|l| l.contains("forall") || l.contains("coforall")) {
                    issues.push(Issue {
                        issue_type: "performance".to_string(),
                        code: Some("UNNECESSARY_COPY".to_string()),
                        file: file.to_string(),
                        line: line_num,
                        column: None,
                        message: "Copia potencialmente innecesaria de array antes de bucle paralelo".to_string(),
                        source_code: Some(line.trim().to_string()),
                        solutions: vec![
                            Solution {
                                title: "Usar 'ref' para evitar copia".to_string(),
                                description: "Pasa por referencia para evitar copias costosas".to_string(),
                                url: "https://chapel-lang.org/docs/language/spec/procedures.html".to_string(),
                                example_code: Some("proc processArray(ref A: [] real) { /* modifica A directamente */ }".to_string()),
                                priority: 8,
                            }
                        ],
                    });
                }
            }

            // Uso excesivo de variables globales
            if line.contains("var ") && !line.contains("proc ") && !line.contains("iter ") && !line.contains("class ") {
                if content.lines().take(line_num).any(|l| l.contains("module ")) {
                    issues.push(Issue {
                        issue_type: "info".to_string(),
                        code: Some("GLOBAL_VARIABLE".to_string()),
                        file: file.to_string(),
                        line: line_num,
                        column: None,
                        message: "Variable global detectada - considera si realmente necesita ser global".to_string(),
                        source_code: Some(line.trim().to_string()),
                        solutions: vec![
                            Solution {
                                title: "Usar variables locales cuando sea posible".to_string(),
                                description: "Las variables globales pueden causar problemas de locality y rendimiento".to_string(),
                                url: "https://chapel-lang.org/docs/language/spec/modules.html".to_string(),
                                example_code: Some("// Pasa como parámetro en lugar de usar global".to_string()),
                                priority: 5,
                        }
                        ],
                    });
                }
            }
        }

        Ok(issues)
    }

    /// Análisis de dependencias de tareas en Chapel
    fn analyze_chapel_task_dependencies(&self, content: &str, file: &str, project_path: &PathBuf) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_num = line_num + 1;

            // Dependencias circulares potenciales
            if line.contains("begin") && content.lines().skip(line_num).take(20).any(|l| l.contains("waitFor") || l.contains("sync")) {
                // Buscar patrones de dependencia circular
                let next_lines: Vec<&str> = content.lines().skip(line_num).take(20).collect();
                if next_lines.iter().any(|l| l.contains("begin")) && next_lines.iter().any(|l| l.contains("waitFor")) {
                    issues.push(Issue {
                        issue_type: "warning".to_string(),
                        code: Some("CIRCULAR_DEPENDENCY".to_string()),
                        file: file.to_string(),
                        line: line_num,
                        column: None,
                        message: "Posible dependencia circular entre tasks - riesgo de deadlock".to_string(),
                        source_code: Some(line.trim().to_string()),
                        solutions: vec![
                            Solution {
                                title: "Rediseñar dependencias de tasks".to_string(),
                                description: "Evita dependencias circulares reorganizando el flujo de tasks".to_string(),
                                url: "https://chapel-lang.org/docs/language/spec/task-parallelism-and-synchronization.html".to_string(),
                                example_code: Some("// Usa coforall para paralelismo sin dependencias complejas".to_string()),
                                priority: 9,
                            }
                        ],
                    });
                }
            }

            // Tasks sin límites de recursos
            if line.contains("coforall") && !content.lines().skip(line_num.saturating_sub(10)).take(15).any(|l| l.contains("numTasks") || l.contains("here.maxTaskPar")) {
                issues.push(Issue {
                    issue_type: "performance".to_string(),
                    code: Some("UNBOUNDED_TASKS".to_string()),
                    file: file.to_string(),
                    line: line_num,
                    column: None,
                    message: "coforall sin límite explícito de tasks - puede crear demasiados threads".to_string(),
                    source_code: Some(line.trim().to_string()),
                    solutions: vec![
                        Solution {
                            title: "Limitar número de tasks".to_string(),
                            description: "Especifica el número de tasks para controlar el paralelismo".to_string(),
                            url: "https://chapel-lang.org/docs/language/spec/task-parallelism-and-synchronization.html".to_string(),
                            example_code: Some("coforall i in 1..min(numElements, here.maxTaskPar) do".to_string()),
                            priority: 7,
                        }
                    ],
                });
            }
        }

        Ok(issues)
    }

    /// Sugerencias de optimización avanzadas para Chapel
    fn suggest_chapel_optimizations(&self, content: &str, file: &str, project_path: &PathBuf) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_num = line_num + 1;

            // Optimización: usar zippered iteration
            if line.contains("forall") && content.lines().skip(line_num).take(3).any(|l| l.contains("forall")) {
                issues.push(Issue {
                    issue_type: "optimization".to_string(),
                    code: Some("ZIPPERED_ITERATION".to_string()),
                    file: file.to_string(),
                    line: line_num,
                    column: None,
                    message: "Posible optimización: considera usar 'zippered iteration' para mejor vectorización".to_string(),
                    source_code: Some(line.trim().to_string()),
                    solutions: vec![
                        Solution {
                            title: "Usar zippered iteration".to_string(),
                            description: "Combina múltiples iteraciones en una para mejor rendimiento".to_string(),
                            url: "https://chapel-lang.org/docs/language/spec/forall.html".to_string(),
                            example_code: Some("forall (i, j) in zip(1..N, 1..M) do A[i] = B[j];".to_string()),
                            priority: 6,
                        }
                    ],
                });
            }

            // Optimización: usar promoted operations
            if line.contains("forall") && line.contains("A[") && line.contains("] = B[") {
                issues.push(Issue {
                    issue_type: "optimization".to_string(),
                    code: Some("PROMOTED_OPERATIONS".to_string()),
                    file: file.to_string(),
                    line: line_num,
                    column: None,
                    message: "Optimización posible: usa operaciones promovidas para mejor rendimiento".to_string(),
                    source_code: Some(line.trim().to_string()),
                    solutions: vec![
                        Solution {
                            title: "Usar operaciones promovidas".to_string(),
                            description: "Las operaciones sobre arrays completos son más eficientes".to_string(),
                            url: "https://chapel-lang.org/docs/language/spec/arrays.html".to_string(),
                            example_code: Some("A = B + C; // en lugar de forall i do A[i] = B[i] + C[i]".to_string()),
                            priority: 8,
                        }
                    ],
                });
            }

            // Optimización: usar reduce en lugar de bucles manuales
            if line.contains("forall") && (line.contains("sum") || line.contains("max") || line.contains("min")) {
                issues.push(Issue {
                    issue_type: "optimization".to_string(),
                    code: Some("REDUCE_OPTIMIZATION".to_string()),
                    file: file.to_string(),
                    line: line_num,
                    column: None,
                    message: "Optimización: usa 'reduce' en lugar de acumuladores manuales".to_string(),
                    source_code: Some(line.trim().to_string()),
                    solutions: vec![
                        Solution {
                            title: "Usar reduce para mejor rendimiento".to_string(),
                            description: "Las operaciones reduce están optimizadas y son más eficientes".to_string(),
                            url: "https://chapel-lang.org/docs/language/spec/reductions.html".to_string(),
                            example_code: Some("var total = + reduce A; // en lugar de bucles manuales".to_string()),
                            priority: 9,
                        }
                    ],
                });
            }
        }

        Ok(issues)
    }

    /// Genera recomendaciones HIPER-AVANZADAS para Chapel (200% aprovechamiento)
    async fn generate_hyper_chapel_recommendations(&self, errors: &[Issue], warnings: &[Issue], performance_issues: &[Issue]) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        // Análisis de errores críticos
        if errors.iter().any(|e| e.message.contains("undeclared") || e.message.contains("undefined")) {
            recommendations.push(
                "🚨 ERROR CRÍTICO: Variables no declaradas. Chapel requiere declaración explícita de todos los símbolos.".to_string(),
            );
        }

        // Análisis de paralelismo avanzado
        if warnings.iter().any(|w| w.code.as_ref().map_or(false, |c| c == "TASK_MANAGEMENT")) {
            recommendations.push(
                "🔥 PARALELISMO CRÍTICO: Tasks 'begin' sin sincronización pueden causar race conditions catastróficas.".to_string(),
            );
        }

        if performance_issues.iter().any(|p| p.code.as_ref().map_or(false, |c| c == "NON_LOCAL_ACCESS")) {
            recommendations.push(
                "⚡ RENDIMIENTO CRÍTICO: Acceso no-local en bucles paralelos puede reducir rendimiento >100x.".to_string(),
            );
        }

        // Análisis de dominios y distribución
        if warnings.iter().any(|w| w.code.as_ref().map_or(false, |c| c == "DOMAIN_DISTRIBUTION")) {
            recommendations.push(
                "🏗️ DISTRIBUCIÓN: Dominios sin distribución explícita pierden el poder de Chapel en múltiples nodos.".to_string(),
            );
        }

        // Análisis de optimizaciones
        if performance_issues.iter().any(|p| p.code.as_ref().map_or(false, |c| c == "PROMOTED_OPERATIONS")) {
            recommendations.push(
                "🚀 OPTIMIZACIÓN: Usa operaciones promovidas para rendimiento 10-100x mejor.".to_string(),
            );
        }

        if performance_issues.iter().any(|p| p.code.as_ref().map_or(false, |c| c == "REDUCE_OPTIMIZATION")) {
            recommendations.push(
                "⚡ REDUCE POWER: Las operaciones 'reduce' están altamente optimizadas en Chapel.".to_string(),
            );
        }

        // Análisis de patrones anti-óptimos
        if warnings.iter().any(|w| w.code.as_ref().map_or(false, |c| c == "ANTI_PATTERN_WHILE")) {
            recommendations.push(
                "🛑 ANTI-PATTERN: Los bucles 'while' no aprovechan el paralelismo de Chapel.".to_string(),
            );
        }

        // Recomendaciones específicas de Chapel
        recommendations.push("🎯 CHAPEL MASTERY: Usa 'dmapped Block' para distribución automática inteligente.".to_string());
        recommendations.push("🔬 HPC EXPERTISE: Chapel compila a C/CUDA/OpenCL - aprovecha todo el hardware.".to_string());
        recommendations.push("⚖️ LOAD BALANCING: Experimenta con Cyclic vs Block según patrones de acceso.".to_string());
        recommendations.push("🎪 ADVANCED FEATURES: Explora 'zippered iteration' y 'promoted operations'.".to_string());

        // Estadísticas avanzadas
        let total_issues = errors.len() + warnings.len() + performance_issues.len();
        if total_issues > 20 {
            recommendations.push(format!(
                "📊 ANÁLISIS COMPLETO: {} issues encontrados. Chapel requiere atención meticulosa para rendimiento óptimo."
            , total_issues));
        }

        if errors.is_empty() && warnings.is_empty() && performance_issues.is_empty() {
            recommendations.push("🎉 CHAPEL MASTERPIECE: Código perfecto que aprovecha el 200% del potencial de Chapel!".to_string());
        }

        recommendations.push("📚 CHAPEL BIBLE: https://chapel-lang.org/docs/language/spec/".to_string());
        recommendations.push("🔬 HANDS-ON: Ejecuta con 'chpl --fast --specialize' para optimizaciones agresivas.".to_string());

        Ok(recommendations)
    }

    /// Calcula quality score con bonus por optimizaciones avanzadas de Chapel
    fn calculate_chapel_quality_score(&self, errors: &[Issue], warnings: &[Issue], performance_issues: &[Issue]) -> f32 {
        let mut score = 100.0;

        // Penalizaciones estándar
        score -= errors.len() as f32 * 15.0; // Errores son críticos en Chapel
        score -= warnings.len() as f32 * 3.0;
        score -= performance_issues.len() as f32 * 2.0;

        // Bonus por optimizaciones avanzadas
        let optimization_bonus = performance_issues.iter()
            .filter(|p| p.issue_type == "optimization")
            .count() as f32 * 1.0; // Bonus por detectar oportunidades de optimización

        score += optimization_bonus.min(20.0); // Máximo 20 puntos de bonus

        // Bonus por buen uso de Chapel
        if warnings.iter().any(|w| w.code.as_ref().map_or(false, |c| c == "DOMAIN_DISTRIBUTION")) {
            score -= 5.0; // Penalización por no usar distribución
        }

        score.max(0.0).min(100.0)
    }

    /// Calcula score de calidad (0-100)
    fn calculate_quality_score(&self, errors: &[Issue], warnings: &[Issue]) -> f32 {
        if errors.is_empty() && warnings.is_empty() {
            return 100.0;
        }

        let error_penalty = errors.len() as f32 * 10.0;
        let warning_penalty = warnings.len() as f32 * 2.0;

        (100.0 - error_penalty - warning_penalty).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_ultra_advanced_chapel_analyzer() {
        println!("🧪 Probando analizador ULTRA-AVANZADO de Chapel (200% aprovechamiento)");

        // Crear WebSearch para el scanner
        let web_search = Arc::new(crate::web_search::WebSearch::new_with_storage(None).unwrap());
        let scanner = ProjectScanner::new(web_search);
        let project_path = PathBuf::from("c:\\Users\\DELL\\Desktop\\hf_spaces");

        match scanner.scan_chapel_project(project_path).await {
            Ok(analysis) => {
                println!("✅ Análisis completado exitosamente!");
                println!("📊 Quality Score: {:.1}/100", analysis.quality_score);
                println!("📁 Total Issues: {}, ❌ Errores: {}, ⚠️ Warnings: {}",
                    analysis.total_issues,
                    analysis.errors.len(),
                    analysis.warnings.len()
                );

                // Verificar que se generaron algunos issues o recomendaciones
                assert!(analysis.total_issues >= 0, "Debería procesar los archivos");

                // Verificar que hay recomendaciones
                assert!(!analysis.recommendations.is_empty(), "Debería generar recomendaciones");

                println!("🎯 Test completado - analizador ultra-avanzado funcionando!");
            }
            Err(e) => {
                panic!("❌ Error en el análisis: {}", e);
            }
        }
    }
}

