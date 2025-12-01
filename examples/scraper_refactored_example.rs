//! Módulo Scraper - Ejemplo de cómo quedaría después de la refactorización
//!
//! Este archivo muestra cómo eliminar duplicación usando el nuevo módulo utils
//! PRESERVANDO todas las funcionalidades existentes

use anyhow::{anyhow, Context, Result};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ✨ NUEVO: Importamos las funciones centralizadas
use crate::utils::{
    extract_description, extract_headings, extract_images, extract_links, extract_meta_tags,
    extract_source, extract_tables, extract_title, HeadingInfo, ImageInfo, LinkInfo, TableData,
};

/// Datos extraídos de una página
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtractedData {
    pub title: String,
    pub description: String,
    pub url: String,
    pub links: Vec<LinkInfo>,
    pub images: Vec<ImageInfo>,
    pub headings: Vec<HeadingInfo>,
    pub tables: Vec<TableData>,
    pub meta_tags: HashMap<String, String>,
    pub text_content: String,
    pub word_count: usize,
}

/// Scraper principal - VERSIÓN REFACTORIZADA
pub struct Scraper {
    #[allow(dead_code)]
    config: ScraperConfig,
}

#[derive(Debug, Clone, Default)]
pub struct ScraperConfig {
    pub extract_images: bool,
    pub extract_tables: bool,
    pub extract_links: bool,
}

impl Default for Scraper {
    fn default() -> Self {
        Self::new()
    }
}

impl Scraper {
    pub fn new() -> Self {
        Self {
            config: ScraperConfig::default(),
        }
    }

    /// ✨ REFACTORIZADO: Usa utils::extract_links
    /// ANTES: 30 líneas de código duplicado
    /// AHORA: 1 línea llamando a la función centralizada
    pub fn extract_links(&self, html: &str, base_url: &str) -> Result<Vec<String>> {
        extract_links(html, base_url).map(|links| links.iter().map(|l| l.url.clone()).collect())
    }

    /// ✨ REFACTORIZADO: Usa utils::extract_links para información detallada
    pub fn extract_link_info(&self, html: &str, base_url: &str) -> Result<Vec<LinkInfo>> {
        extract_links(html, base_url)
    }

    /// ✨ REFACTORIZADO: Usa utils::extract_images
    /// ANTES: 35 líneas de código duplicado
    /// AHORA: 1 línea
    pub fn extract_images(&self, html: &str, base_url: &str) -> Result<Vec<ImageInfo>> {
        extract_images(html, base_url)
    }

    /// ✨ REFACTORIZADO: Usa múltiples funciones de utils
    /// ANTES: 100+ líneas con código duplicado
    /// AHORA: ~20 líneas llamando a funciones centralizadas
    pub fn extract_all(&self, html: &str, base_url: &str) -> Result<ExtractedData> {
        let document = Html::parse_document(html);
        let text_content: String = document.root_element().text().collect();
        let word_count = text_content.split_whitespace().count();

        Ok(ExtractedData {
            title: extract_title(html),
            description: extract_description(html),
            url: base_url.to_string(),
            links: extract_links(html, base_url).unwrap_or_default(),
            images: extract_images(html, base_url).unwrap_or_default(),
            headings: extract_headings(html).unwrap_or_default(),
            tables: extract_tables(html).unwrap_or_default(),
            meta_tags: extract_meta_tags(html),
            text_content,
            word_count,
        })
    }

    /// Extrae datos usando un selector CSS personalizado
    /// (Esta función es única de scraper.rs, se mantiene)
    pub fn extract_by_selector(&self, html: &str, selector: &str) -> Result<Vec<String>> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(selector).context("Invalid CSS selector")?;

        let results: Vec<String> = document
            .select(&selector)
            .map(|element| element.text().collect::<String>().trim().to_string())
            .filter(|text| !text.is_empty())
            .collect();

        if results.is_empty() {
            return Err(anyhow!("No elements found for selector: {}", selector));
        }

        Ok(results)
    }

    /// ✨ REFACTORIZADO: Funciones de parser integradas
    pub fn parse(&self, html: &str) -> Html {
        Html::parse_document(html)
    }

    /// ✨ REFACTORIZADO: Usa utils::extract_title
    pub fn extract_title(&self, html: &str) -> Option<String> {
        let title = extract_title(html);
        if title == "Sin título" {
            None
        } else {
            Some(title)
        }
    }

    /// ✨ REFACTORIZADO: Usa utils::extract_meta_tags
    pub fn extract_meta_tags(&self, html: &str) -> serde_json::Map<String, serde_json::Value> {
        let meta_tags = extract_meta_tags(html);
        let mut json_map = serde_json::Map::new();

        for (key, value) in meta_tags {
            json_map.insert(key, serde_json::Value::String(value));
        }

        json_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scraper_creation() {
        let scraper = Scraper::new();
        assert!(scraper.config.extract_images == false);
    }

    #[test]
    fn test_extract_title() {
        let scraper = Scraper::new();
        let html = "<html><head><title>Test Page</title></head><body></body></html>";
        let title = scraper.extract_title(html);
        assert_eq!(title, Some("Test Page".to_string()));
    }

    #[test]
    fn test_extract_by_selector() {
        let scraper = Scraper::new();
        let html = r#"
            <html>
                <body>
                    <h1>Heading 1</h1>
                    <h1>Heading 2</h1>
                </body>
            </html>
        "#;
        let results = scraper.extract_by_selector(html, "h1").unwrap();
        assert_eq!(results.len(), 2);
    }
}
