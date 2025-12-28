// 🔥 ANALYZER ULTRA-MEJORADO - Líneas exactas + Búsqueda automática + Código para editar

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::cargo_parser::CompilerMessage;

/// Resultado de análisis ULTRA-DETALLADO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UltraAnalysisResult {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub error_code: String,
    pub error_message: String,
    pub source_code_snippet: String,
    pub exact_line: String,
    
    // 🔥 NUEVO: Búsqueda automática en internet
    pub internet_solutions: Vec<InternetSolution>,
    
    // 🔥 NUEVO: Código exacto para editar
    pub fix_suggestions: Vec<CodeFix>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetSolution {
    pub source: String,  // "stackoverflow", "github", "reddit", etc.
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub relevance_score: f64,
    pub votes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFix {
    pub description: String,
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
    pub original_code: String,
    pub fixed_code: String,
    pub confidence: f64,  // 0.0 - 1.0
}

impl UltraAnalysisResult {
    /// Crear desde mensaje del compilador
    pub fn from_compiler_message(msg: &CompilerMessage) -> Option<Self> {
        if msg.message.level != "error" && msg.message.level != "warning" {
            return None;
        }
        
        // Obtener el span principal
        let primary_span = msg.message.spans.iter()
            .find(|s| s.is_primary)?;
        
        let error_code = msg.message.code.as_ref()
            .map(|c| c.code.clone())
            .unwrap_or_else(|| "unknown".to_string());
        
        Some(Self {
            file: primary_span.file_name.clone(),
            line: primary_span.line_start,
            column: primary_span.column_start,
            error_code,
            error_message: msg.message.message.clone(),
            source_code_snippet: String::new(),
            exact_line: primary_span.text.first()
                .map(|t| t.text.clone())
                .unwrap_or_default(),
            internet_solutions: Vec::new(),
            fix_suggestions: Vec::new(),
        })
    }
    /// Busca soluciones en internet para este error
    pub async fn search_internet_solutions(
        &mut self,
        web_search: &crate::web_search::WebSearch,
    ) -> Result<()> {
        // Construir query de búsqueda inteligente
        let search_query = format!(
            "{} {} site:stackoverflow.com OR site:github.com OR site:reddit.com/r/rust",
            self.error_code,
            self.error_message.split_whitespace().take(10).collect::<Vec<_>>().join(" ")
        );
        
        eprintln!("🔍 Buscando soluciones en internet para: {}", self.error_code);
        
        let config = crate::web_search::WebSearchConfig {
            query: search_query,
            max_results: 20,
            max_parallel: 100,
            timeout_secs: 10,
            max_urls: 20,
            use_ai: true,
            use_stealth: false,
            priority_sources: vec![
                "stackoverflow.com".into(),
                "github.com".into(),
                "reddit.com".into(),
                "users.rust-lang.org".into(),
            ],
            sources: vec![],
            ..Default::default()
        };
        
        match web_search.search(config).await {
            Ok(results) => {
                self.internet_solutions = results.into_iter()
                    .map(|r| InternetSolution {
                        source: if r.url.contains("stackoverflow") {
                            "StackOverflow".to_string()
                        } else if r.url.contains("github") {
                            "GitHub".to_string()
                        } else if r.url.contains("reddit") {
                            "Reddit".to_string()
                        } else {
                            "Web".to_string()
                        },
                        title: r.title,
                        url: r.url,
                        snippet: r.description,
                        relevance_score: r.relevance as f64,
                        votes: None,
                    })
                    .collect();
                
                eprintln!("✅ Encontradas {} soluciones en internet", self.internet_solutions.len());
            }
            Err(e) => {
                eprintln!("⚠️ Error buscando soluciones: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// Genera sugerencias de código EXACTO para editar
    pub fn generate_code_fixes(&mut self, file_content: &str) -> Result<()> {
        let lines: Vec<&str> = file_content.lines().collect();
        
        // Obtener contexto (5 líneas antes y después)
        let start_ctx = self.line.saturating_sub(5);
        let end_ctx = (self.line + 5).min(lines.len());
        
        let context_lines = &lines[start_ctx..end_ctx];
        let exact_line = lines.get(self.line.saturating_sub(1))
            .unwrap_or(&"")
            .to_string();
        
        self.exact_line = exact_line.clone();
        self.source_code_snippet = context_lines.join("\n");
        
        // 🔥 ANÁLISIS INTELIGENTE DE ERRORES COMUNES
        self.fix_suggestions = self.analyze_and_suggest_fixes(&exact_line, &lines);
        
        Ok(())
    }
    
    fn analyze_and_suggest_fixes(&self, exact_line: &str, _all_lines: &[&str]) -> Vec<CodeFix> {
        let mut fixes = Vec::new();
        
        // ERROR: unused variable
        if self.error_code.contains("E0unused") || self.error_message.contains("unused") {
            if let Some(var_name) = self.extract_variable_name(&self.error_message) {
                fixes.push(CodeFix {
                    description: format!("Agregar prefijo _ a variable no usada: {}", var_name),
                    file: self.file.clone(),
                    start_line: self.line,
                    end_line: self.line,
                    original_code: exact_line.to_string(),
                    fixed_code: exact_line.replace(&var_name, &format!("_{}", var_name)),
                    confidence: 0.95,
                });
            }
        }
        
        // ERROR: cannot find type/value
        if self.error_code.contains("E0412") || self.error_code.contains("E0425") {
            if let Some(missing_item) = self.extract_missing_item(&self.error_message) {
                fixes.push(CodeFix {
                    description: format!("Agregar import para: {}", missing_item),
                    file: self.file.clone(),
                    start_line: 1,
                    end_line: 1,
                    original_code: "".to_string(),
                    fixed_code: self.suggest_import(&missing_item),
                    confidence: 0.80,
                });
            }
        }
        
        // ERROR: mismatched types
        if self.error_code.contains("E0308") {
            fixes.push(CodeFix {
                description: "Tipos no coinciden - revisar conversión".to_string(),
                file: self.file.clone(),
                start_line: self.line,
                end_line: self.line,
                original_code: exact_line.to_string(),
                fixed_code: format!("{} // TODO: Convertir tipo aquí", exact_line),
                confidence: 0.60,
            });
        }
        
        // ERROR: method not found
        if self.error_code.contains("E0599") {
            if let Some(method_name) = self.extract_method_name(&self.error_message) {
                fixes.push(CodeFix {
                    description: format!("Método '{}' no encontrado - verificar trait o tipo", method_name),
                    file: self.file.clone(),
                    start_line: self.line,
                    end_line: self.line,
                    original_code: exact_line.to_string(),
                    fixed_code: format!("{} // TODO: Implementar trait o cambiar método", exact_line),
                    confidence: 0.70,
                });
            }
        }
        
        fixes
    }
    
    fn extract_variable_name(&self, message: &str) -> Option<String> {
        // Extraer nombre de variable de mensajes como "unused variable: `foo`"
        if let Some(start) = message.find('`') {
            if let Some(end) = message[start+1..].find('`') {
                return Some(message[start+1..start+1+end].to_string());
            }
        }
        None
    }
    
    fn extract_missing_item(&self, message: &str) -> Option<String> {
        // Extraer item faltante de mensajes como "cannot find type `Foo`"
        if let Some(start) = message.find('`') {
            if let Some(end) = message[start+1..].find('`') {
                return Some(message[start+1..start+1+end].to_string());
            }
        }
        None
    }
    
    fn extract_method_name(&self, message: &str) -> Option<String> {
        // Extraer nombre de método de mensajes como "no method named `foo`"
        if let Some(start) = message.find('`') {
            if let Some(end) = message[start+1..].find('`') {
                return Some(message[start+1..start+1+end].to_string());
            }
        }
        None
    }
    
    fn suggest_import(&self, item: &str) -> String {
        // Sugerencias inteligentes de imports comunes
        let common_imports = [
            ("Result", "use anyhow::Result;"),
            ("HashMap", "use std::collections::HashMap;"),
            ("Vec", "use std::vec::Vec;"),
            ("String", "use std::string::String;"),
            ("Arc", "use std::sync::Arc;"),
            ("Mutex", "use std::sync::Mutex;"),
            ("tokio", "use tokio;"),
            ("serde", "use serde::{Serialize, Deserialize};"),
        ];
        
        for (name, import) in &common_imports {
            if item.contains(name) {
                return import.to_string();
            }
        }
        
        format!("use {}; // TODO: Verificar crate correcto", item)
    }
}
