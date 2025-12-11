//! Content Extractor - Extracción REAL de contenido de páginas web
//!
//! Este módulo mejora la extracción de datos para guardar información ÚTIL
//! no solo URLs vacías sino contenido real extraído.

use crate::utils::html_extraction::{
    extract_description, extract_headings, extract_images, extract_links, extract_tables,
    extract_title, TableData,
};
use ammonia::clean;
use anyhow::Result;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

/// Contenido extraído completo de una página
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedContent {
    /// URL original
    pub url: String,
    /// Título de la página
    pub title: String,
    /// Descripción/meta description
    pub description: String,
    /// Texto principal limpio (sin HTML)
    pub main_text: String,
    /// Texto completo (todo el contenido visible)
    pub full_text: String,
    /// Resumen automático (primeros 500 chars)
    pub summary: String,
    /// Número de palabras
    pub word_count: usize,
    /// Idioma detectado
    pub language: String,
    /// Lista de headings (h1-h6)
    pub headings: Vec<String>,
    /// Links encontrados
    pub links: Vec<ExtractedLink>,
    /// Imágenes encontradas
    pub images: Vec<ExtractedImage>,
    /// Tablas encontradas
    pub tables: Vec<TableData>,
    /// Código encontrado (snippets)
    pub code_snippets: Vec<CodeSnippet>,
    /// Fecha de extracción
    pub extracted_at: String,
    /// Tiempo de extracción (ms)
    pub extraction_time_ms: u64,
    /// Fuente/dominio
    pub source: String,
}

impl ExtractedContent {
    /// Crea un ExtractedContent vacío con solo la URL
    pub fn default_for_url(url: &str) -> Self {
        let source = url::Url::parse(url)
            .ok()
            .and_then(|u| u.host_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "unknown".to_string());
            
        Self {
            url: url.to_string(),
            title: String::new(),
            description: String::new(),
            main_text: String::new(),
            full_text: String::new(),
            summary: String::new(),
            word_count: 0,
            language: String::new(),
            headings: Vec::new(),
            links: Vec::new(),
            images: Vec::new(),
            tables: Vec::new(),
            code_snippets: Vec::new(),
            extracted_at: chrono::Utc::now().to_rfc3339(),
            extraction_time_ms: 0,
            source,
        }
    }
}

/// Link extraído simplificado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedLink {
    pub url: String,
    pub text: String,
}

/// Imagen extraída simplificada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedImage {
    pub url: String,
    pub alt: String,
}

/// Snippet de código encontrado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub language: String,
    pub code: String,
}

/// Extractor de contenido principal
pub struct ContentExtractor {
    /// Selectores para contenido principal
    main_content_selectors: Vec<&'static str>,
    /// Selectores a ignorar (ads, navigation, etc)
    ignore_selectors: Vec<&'static str>,
}

impl Default for ContentExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl ContentExtractor {
    pub fn new() -> Self {
        Self {
            main_content_selectors: vec![
                "article",
                "main",
                "[role='main']",
                ".post-content",
                ".article-content",
                ".entry-content",
                ".content",
                "#content",
                ".post",
                ".article",
                ".blog-post",
                // Para Stack Overflow / GitHub
                ".question",
                ".answer",
                ".readme",
                ".markdown-body",
                // Para documentación
                ".documentation",
                ".docs-content",
                "#readme",
            ],
            ignore_selectors: vec![
                "script",
                "style",
                "noscript",
                "iframe",
                "nav",
                "header",
                "footer",
                "aside",
                ".sidebar",
                ".advertisement",
                ".ad",
                ".ads",
                ".cookie-banner",
                ".popup",
                ".modal",
                "#comments",
                ".comments",
                ".social-share",
                ".related-posts",
            ],
        }
    }

    /// Extrae todo el contenido útil de un HTML
    pub fn extract_all(&self, html: &str, url: &str) -> Result<ExtractedContent> {
        let start = std::time::Instant::now();
        let document = Html::parse_document(html);

        // Extraer título y descripción
        let title = extract_title(html);
        let description = extract_description(html);

        // Extraer texto principal (contenido limpio)
        let main_text = self.extract_main_text(&document);
        
        // Extraer todo el texto visible
        let full_text = self.extract_full_text(html);
        
        // Generar resumen
        let summary = self.generate_summary(&main_text, &description);
        
        // Contar palabras
        let word_count = main_text.split_whitespace().count();
        
        // Detectar idioma
        let language = self.detect_language(&document);
        
        // Extraer headings
        let headings = extract_headings(html)
            .unwrap_or_default()
            .into_iter()
            .map(|h| format!("H{}: {}", h.level, h.text))
            .collect();
        
        // Extraer links
        let links = extract_links(html, url)
            .unwrap_or_default()
            .into_iter()
            .filter(|l| !l.url.is_empty() && l.url.starts_with("http"))
            .map(|l| ExtractedLink {
                url: l.url,
                text: if l.text.is_empty() { l.title } else { l.text },
            })
            .take(100) // Máximo 100 links
            .collect();
        
        // Extraer imágenes
        let images = extract_images(html, url)
            .unwrap_or_default()
            .into_iter()
            .filter(|i| !i.url.is_empty() && i.url.starts_with("http"))
            .map(|i| ExtractedImage {
                url: i.url,
                alt: i.alt,
            })
            .take(50) // Máximo 50 imágenes
            .collect();
        
        // Extraer tablas
        let tables = extract_tables(html).unwrap_or_default();
        
        // Extraer código
        let code_snippets = self.extract_code_snippets(&document);
        
        // Extraer dominio
        let source = url::Url::parse(url)
            .ok()
            .and_then(|u| u.host_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "unknown".to_string());

        Ok(ExtractedContent {
            url: url.to_string(),
            title,
            description,
            main_text,
            full_text,
            summary,
            word_count,
            language,
            headings,
            links,
            images,
            tables,
            code_snippets,
            extracted_at: chrono::Utc::now().to_rfc3339(),
            extraction_time_ms: start.elapsed().as_millis() as u64,
            source,
        })
    }

    /// Extrae el texto principal del contenido (filtrando elementos no deseados)
    fn extract_main_text(&self, document: &Html) -> String {
        // Intentar selectores de contenido principal
        for selector_str in &self.main_content_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let text: String = element.text().collect();
                    let cleaned = self.clean_text(&text);
                    // Filtrar contenido de elementos ignorados
                    let filtered = self.filter_ignored_content(&cleaned, document);
                    if filtered.len() > 100 {
                        return filtered;
                    }
                }
            }
        }

        // Fallback: extraer de body filtrando elementos no deseados
        if let Ok(selector) = Selector::parse("body") {
            if let Some(element) = document.select(&selector).next() {
                let text: String = element.text().collect();
                let cleaned = self.clean_text(&text);
                return self.filter_ignored_content(&cleaned, document);
            }
        }

        String::new()
    }

    /// Filtra contenido de elementos que deben ignorarse (ads, nav, footer, etc.)
    fn filter_ignored_content(&self, text: &str, document: &Html) -> String {
        let mut ignored_texts: Vec<String> = Vec::new();
        
        // Recolectar texto de elementos a ignorar
        for selector_str in &self.ignore_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    let elem_text: String = element.text().collect();
                    let cleaned = self.clean_text(&elem_text);
                    if cleaned.len() > 20 {
                        ignored_texts.push(cleaned);
                    }
                }
            }
        }
        
        // Filtrar texto ignorado del contenido principal
        let mut result = text.to_string();
        for ignored in &ignored_texts {
            // Solo remover si es una coincidencia sustancial
            if ignored.len() > 50 {
                result = result.replace(ignored, " ");
            }
        }
        
        self.clean_text(&result)
    }

    /// Extrae todo el texto visible
    fn extract_full_text(&self, html: &str) -> String {
        // Usar ammonia para limpiar HTML y obtener texto
        let cleaned = clean(html);
        self.clean_text(&cleaned)
    }

    /// Limpia texto de espacios extra y caracteres no deseados
    fn clean_text(&self, text: &str) -> String {
        text.split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string()
    }

    /// Genera un resumen del contenido
    fn generate_summary(&self, main_text: &str, description: &str) -> String {
        // Preferir descripción si es buena
        if description.len() > 50 && description != "Sin descripción" {
            return description.chars().take(500).collect();
        }

        // Sino, usar primeros 500 chars del texto principal
        main_text.chars().take(500).collect()
    }

    /// Detecta el idioma del documento
    fn detect_language(&self, document: &Html) -> String {
        if let Ok(selector) = Selector::parse("html") {
            if let Some(element) = document.select(&selector).next() {
                if let Some(lang) = element.value().attr("lang") {
                    return lang.to_string();
                }
            }
        }
        "es".to_string()
    }

    /// Extrae snippets de código
    fn extract_code_snippets(&self, document: &Html) -> Vec<CodeSnippet> {
        let mut snippets = Vec::new();

        // Buscar <pre><code>
        if let Ok(selector) = Selector::parse("pre code, pre, code") {
            for element in document.select(&selector).take(20) {
                let code: String = element.text().collect();
                if code.len() > 10 {
                    // Detectar lenguaje de la clase
                    let class = element.value().attr("class").unwrap_or("");
                    let language = self.detect_code_language(class);
                    
                    snippets.push(CodeSnippet {
                        language,
                        code: code.chars().take(2000).collect(), // Máximo 2000 chars
                    });
                }
            }
        }

        snippets
    }

    /// Detecta el lenguaje de programación de una clase CSS
    fn detect_code_language(&self, class: &str) -> String {
        let languages = [
            "rust", "python", "javascript", "typescript", "go", "java",
            "c", "cpp", "csharp", "ruby", "php", "swift", "kotlin",
            "scala", "zig", "nim", "julia", "r", "sql", "bash", "shell",
            "html", "css", "json", "yaml", "toml", "xml", "markdown",
        ];

        for lang in languages {
            if class.to_lowercase().contains(lang) {
                return lang.to_string();
            }
        }

        // Buscar patrones como "language-xxx" o "lang-xxx"
        if let Some(start) = class.find("language-").or_else(|| class.find("lang-")) {
            let rest = &class[start..];
            let lang: String = rest
                .chars()
                .skip_while(|c| *c == '-' || c.is_alphabetic())
                .take_while(|c| c.is_alphanumeric())
                .collect();
            if !lang.is_empty() {
                return lang;
            }
        }

        "unknown".to_string()
    }
}

/// Resultado consolidado de múltiples extracciones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedResults {
    /// Fecha de consolidación
    pub consolidated_at: String,
    /// Query original (si aplica)
    pub query: Option<String>,
    /// Total de URLs procesadas
    pub total_urls: usize,
    /// URLs exitosas
    pub successful: usize,
    /// URLs fallidas
    pub failed: usize,
    /// Contenido extraído
    pub contents: Vec<ExtractedContent>,
    /// Estadísticas
    pub stats: ExtractionStats,
}

/// Estadísticas de extracción
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionStats {
    pub total_words: usize,
    pub total_links: usize,
    pub total_images: usize,
    pub total_code_snippets: usize,
    pub avg_extraction_time_ms: u64,
    pub languages_found: Vec<String>,
    pub top_domains: Vec<(String, usize)>,
}

impl ConsolidatedResults {
    pub fn new(query: Option<String>) -> Self {
        Self {
            consolidated_at: chrono::Utc::now().to_rfc3339(),
            query,
            total_urls: 0,
            successful: 0,
            failed: 0,
            contents: Vec::new(),
            stats: ExtractionStats {
                total_words: 0,
                total_links: 0,
                total_images: 0,
                total_code_snippets: 0,
                avg_extraction_time_ms: 0,
                languages_found: Vec::new(),
                top_domains: Vec::new(),
            },
        }
    }

    pub fn add_content(&mut self, content: ExtractedContent) {
        self.stats.total_words += content.word_count;
        self.stats.total_links += content.links.len();
        self.stats.total_images += content.images.len();
        self.stats.total_code_snippets += content.code_snippets.len();
        
        if !self.stats.languages_found.contains(&content.language) {
            self.stats.languages_found.push(content.language.clone());
        }
        
        self.successful += 1;
        self.total_urls += 1;
        self.contents.push(content);
    }

    pub fn add_failure(&mut self) {
        self.failed += 1;
        self.total_urls += 1;
    }

    pub fn finalize(&mut self) {
        // Calcular promedio de tiempo
        if !self.contents.is_empty() {
            let total_time: u64 = self.contents.iter().map(|c| c.extraction_time_ms).sum();
            self.stats.avg_extraction_time_ms = total_time / self.contents.len() as u64;
        }

        // Calcular top dominios
        let mut domain_count: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for content in &self.contents {
            *domain_count.entry(content.source.clone()).or_insert(0) += 1;
        }
        
        let mut domains: Vec<_> = domain_count.into_iter().collect();
        domains.sort_by(|a, b| b.1.cmp(&a.1));
        self.stats.top_domains = domains.into_iter().take(10).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_extraction() {
        let html = r#"
            <html lang="en">
            <head>
                <title>Test Page</title>
                <meta name="description" content="This is a test page">
            </head>
            <body>
                <article>
                    <h1>Main Title</h1>
                    <p>This is the main content of the page with some text.</p>
                    <pre><code class="language-rust">fn main() { println!("Hello"); }</code></pre>
                    <a href="https://example.com">Link</a>
                </article>
            </body>
            </html>
        "#;

        let extractor = ContentExtractor::new();
        let result = extractor.extract_all(html, "https://test.com").unwrap();

        assert_eq!(result.title, "Test Page");
        assert!(!result.main_text.is_empty());
        assert!(!result.code_snippets.is_empty());
        assert_eq!(result.language, "en");
    }
}
