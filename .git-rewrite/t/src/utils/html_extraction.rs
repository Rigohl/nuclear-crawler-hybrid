//! Utilidades centralizadas para extracción de HTML
//!
//! Este módulo unifica todas las funciones de extracción HTML duplicadas
//! en scraper, web_search, nuclear_scraper, etc.

use anyhow::{Context, Result};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Extrae el título de un HTML
/// Usado por: scraper, web_search, nuclear_scraper, parallel_crawler
pub fn extract_title(html: &str) -> String {
    let document = Html::parse_document(html);

    // Intenta múltiples selectores
    let selectors = [
        "title",
        "meta[property='og:title']",
        "meta[name='twitter:title']",
        "h1",
    ];

    for selector_str in &selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            if let Some(element) = document.select(&selector).next() {
                let text = if selector_str.starts_with("meta") {
                    element.value().attr("content").unwrap_or("").to_string()
                } else {
                    element.text().collect::<String>()
                };

                let text = text.trim();
                if !text.is_empty() {
                    return text.to_string();
                }
            }
        }
    }

    "Sin título".to_string()
}

/// Extrae la descripción de un HTML
/// Usado por: scraper, web_search, nuclear_scraper
pub fn extract_description(html: &str) -> String {
    let document = Html::parse_document(html);

    // Intenta múltiples selectores de descripción
    let selectors = [
        "meta[name='description']",
        "meta[property='og:description']",
        "meta[name='twitter:description']",
        "p",
    ];

    for selector_str in &selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            if let Some(element) = document.select(&selector).next() {
                let text = if selector_str.starts_with("meta") {
                    element.value().attr("content").unwrap_or("").to_string()
                } else {
                    element.text().collect::<String>()
                };

                let text = text.trim();
                if !text.is_empty() && text.len() > 20 {
                    // Limita a 500 caracteres
                    return text.chars().take(500).collect::<String>();
                }
            }
        }
    }

    "Sin descripción".to_string()
}

/// Extrae la fuente/dominio de una URL
/// Usado por: web_search, nuclear_scraper
pub fn extract_source(url: &str) -> String {
    if let Ok(parsed_url) = url::Url::parse(url) {
        if let Some(domain) = parsed_url.domain() {
            return domain.to_string();
        }
    }
    "Desconocido".to_string()
}

/// Extrae todos los meta tags de un HTML
/// Usado por: scraper
pub fn extract_meta_tags(html: &str) -> HashMap<String, String> {
    let mut meta_tags = HashMap::new();
    let document = Html::parse_document(html);

    if let Ok(selector) = Selector::parse("meta") {
        for element in document.select(&selector) {
            let name = element
                .value()
                .attr("name")
                .or_else(|| element.value().attr("property"))
                .unwrap_or("");

            let content = element.value().attr("content").unwrap_or("");

            if !name.is_empty() && !content.is_empty() {
                meta_tags.insert(name.to_string(), content.to_string());
            }
        }
    }

    meta_tags
}

/// Extrae todas las imágenes con metadatos
/// Usado por: scraper
pub fn extract_images(html: &str, base_url: &str) -> Result<Vec<ImageInfo>> {
    let document = Html::parse_document(html);
    let selector =
        Selector::parse("img").map_err(|e| anyhow::anyhow!("Invalid img selector: {:?}", e))?;
    let base = url::Url::parse(base_url).context("Invalid base URL")?;

    let mut images = Vec::new();

    for element in document.select(&selector) {
        if let Some(src) = element.value().attr("src") {
            if let Ok(absolute_url) = base.join(src) {
                images.push(ImageInfo {
                    url: absolute_url.to_string(),
                    alt: element.value().attr("alt").unwrap_or("").to_string(),
                    title: element.value().attr("title").unwrap_or("").to_string(),
                    width: element.value().attr("width").and_then(|w| w.parse().ok()),
                    height: element.value().attr("height").and_then(|h| h.parse().ok()),
                });
            }
        }
    }

    Ok(images)
}

/// Información de una imagen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub url: String,
    pub alt: String,
    pub title: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// Extrae todos los enlaces con metadatos
/// Usado por: scraper, crawler
pub fn extract_links(html: &str, base_url: &str) -> Result<Vec<LinkInfo>> {
    let document = Html::parse_document(html);
    let selector =
        Selector::parse("a").map_err(|e| anyhow::anyhow!("Invalid a selector: {:?}", e))?;
    let base = url::Url::parse(base_url).context("Invalid base URL")?;

    let mut links = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(absolute_url) = base.join(href) {
                links.push(LinkInfo {
                    url: absolute_url.to_string(),
                    text: element.text().collect::<String>().trim().to_string(),
                    title: element.value().attr("title").unwrap_or("").to_string(),
                    rel: element.value().attr("rel").unwrap_or("").to_string(),
                });
            }
        }
    }

    Ok(links)
}

/// Información de un enlace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkInfo {
    pub url: String,
    pub text: String,
    pub title: String,
    pub rel: String,
}

/// Cuenta palabras en un HTML (sin tags)
/// Usado por: web_search_enhanced
pub fn count_words(html: &str) -> usize {
    let document = Html::parse_document(html);
    let text = document.root_element().text().collect::<String>();
    text.split_whitespace().count()
}

/// Detecta si el HTML contiene código
/// Usado por: web_search_enhanced
pub fn detect_code_content(html: &str) -> bool {
    let code_indicators = ["<code>", "<pre>", "```", "class=\"highlight\""];
    code_indicators
        .iter()
        .any(|indicator| html.contains(indicator))
}

/// Detecta el idioma del contenido HTML
/// Usado por: web_search_enhanced
pub fn detect_language(html: &str) -> String {
    let document = Html::parse_document(html);

    // Intenta obtener del atributo lang
    if let Ok(selector) = Selector::parse("html") {
        if let Some(element) = document.select(&selector).next() {
            if let Some(lang) = element.value().attr("lang") {
                return lang.to_string();
            }
        }
    }

    // Intenta obtener de meta tags
    if let Ok(selector) = Selector::parse("meta[http-equiv='content-language']") {
        if let Some(element) = document.select(&selector).next() {
            if let Some(content) = element.value().attr("content") {
                return content.to_string();
            }
        }
    }

    "es".to_string() // Default español
}

/// Extrae headings (h1-h6) del HTML
/// Usado por: scraper
pub fn extract_headings(html: &str) -> Result<Vec<HeadingInfo>> {
    let document = Html::parse_document(html);
    let mut headings = Vec::new();

    // Pre-parse selectores estáticos
    let selectors: Vec<(&str, u8)> = vec![
        ("h1", 1),
        ("h2", 2),
        ("h3", 3),
        ("h4", 4),
        ("h5", 5),
        ("h6", 6),
    ];

    for (selector_str, level) in selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector) {
                headings.push(HeadingInfo {
                    level,
                    text: element.text().collect::<String>().trim().to_string(),
                });
            }
        }
    }

    Ok(headings)
}

/// Información de un heading
#[derive(Debug, Clone)]
pub struct HeadingInfo {
    pub level: u8,
    pub text: String,
}

/// Extrae tablas del HTML
/// Usado por: scraper
pub fn extract_tables(html: &str) -> Result<Vec<TableData>> {
    let document = Html::parse_document(html);
    let table_selector =
        Selector::parse("table").map_err(|e| anyhow::anyhow!("Invalid table selector: {:?}", e))?;
    let mut tables = Vec::new();

    for table in document.select(&table_selector) {
        let mut headers = Vec::new();
        let mut rows = Vec::new();

        // Extrae headers
        if let Ok(th_selector) = Selector::parse("th") {
            for th in table.select(&th_selector) {
                headers.push(th.text().collect::<String>().trim().to_string());
            }
        }

        // Extrae filas
        if let Ok(tr_selector) = Selector::parse("tr") {
            for tr in table.select(&tr_selector) {
                let mut row = Vec::new();
                if let Ok(td_selector) = Selector::parse("td") {
                    for td in tr.select(&td_selector) {
                        row.push(td.text().collect::<String>().trim().to_string());
                    }
                }
                if !row.is_empty() {
                    rows.push(row);
                }
            }
        }

        tables.push(TableData { headers, rows });
    }

    Ok(tables)
}

/// Datos de una tabla
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_title() {
        let html = r#"
            <html>
                <head><title>Test Title</title></head>
                <body></body>
            </html>
        "#;
        assert_eq!(extract_title(html), "Test Title");
    }

    #[test]
    fn test_extract_source() {
        assert_eq!(extract_source("https://example.com/path"), "example.com");
        assert_eq!(extract_source("invalid"), "Desconocido");
    }

    #[test]
    fn test_count_words() {
        let html = "<p>Hello world this is a test</p>";
        assert_eq!(count_words(html), 6);
    }

    #[test]
    fn test_detect_code_content() {
        assert!(detect_code_content("<pre>code here</pre>"));
        assert!(!detect_code_content("<p>normal text</p>"));
    }
}
