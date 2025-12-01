//! Módulo Scraper - Extracción y parsing de datos HTML
//!
//! Consolida scraper.rs y parser.rs - funciones avanzadas para extraer y procesar datos de páginas web

use anyhow::{anyhow, Context, Result};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

/// Datos extraídos de una página
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtractedData {
    pub title: Option<String>,
    pub meta: HashMap<String, String>,
    pub links: Vec<LinkInfo>,
    pub images: Vec<ImageInfo>,
    pub text_content: String,
    pub headings: Vec<HeadingInfo>,
    pub tables: Vec<TableData>,
    pub custom_data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkInfo {
    pub url: String,
    pub text: String,
    pub title: Option<String>,
    pub rel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub url: String,
    pub alt: Option<String>,
    pub title: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingInfo {
    pub level: u8,
    pub text: String,
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

/// Scraper principal
pub struct Scraper {
    link_selector: Selector,
    image_selector: Selector,
    heading_selector: Selector,
    table_selector: Selector,
    meta_selector: Selector,
}

impl Default for Scraper {
    fn default() -> Self {
        Self::new()
    }
}

impl Scraper {
    pub fn new() -> Self {
        Self {
            link_selector: Selector::parse("a[href]").unwrap(),
            image_selector: Selector::parse("img").unwrap(),
            heading_selector: Selector::parse("h1, h2, h3, h4, h5, h6").unwrap(),
            table_selector: Selector::parse("table").unwrap(),
            meta_selector: Selector::parse("meta").unwrap(),
        }
    }

    /// Extrae todos los links de un HTML
    pub fn extract_links(&self, html: &str, base_url: &str) -> Result<Vec<String>> {
        let document = Html::parse_document(html);
        let base = Url::parse(base_url).context("Error parseando URL base")?;

        let mut links = Vec::new();

        for element in document.select(&self.link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(absolute_url) = base.join(href) {
                    links.push(absolute_url.as_str().to_string());
                }
            }
        }

        Ok(links)
    }

    /// Extrae información detallada de links
    pub fn extract_link_info(&self, html: &str, base_url: &str) -> Result<Vec<LinkInfo>> {
        let document = Html::parse_document(html);
        let base = Url::parse(base_url).context("Error parseando URL base")?;

        let mut links = Vec::new();

        for element in document.select(&self.link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(absolute_url) = base.join(href) {
                    let text = element.text().collect::<String>().trim().to_string();
                    let title = element.value().attr("title").map(|s| s.to_string());
                    let rel = element.value().attr("rel").map(|s| s.to_string());

                    links.push(LinkInfo {
                        url: absolute_url.as_str().to_string(),
                        text,
                        title,
                        rel,
                    });
                }
            }
        }

        Ok(links)
    }

    /// Extrae todas las imágenes
    pub fn extract_images(&self, html: &str, base_url: &str) -> Result<Vec<ImageInfo>> {
        let document = Html::parse_document(html);
        let base = Url::parse(base_url).context("Error parseando URL base")?;

        let mut images = Vec::new();

        for element in document.select(&self.image_selector) {
            if let Some(src) = element.value().attr("src") {
                if let Ok(absolute_url) = base.join(src) {
                    let alt = element.value().attr("alt").map(|s| s.to_string());
                    let title = element.value().attr("title").map(|s| s.to_string());
                    let width = element
                        .value()
                        .attr("width")
                        .and_then(|w| w.parse::<u32>().ok());
                    let height = element
                        .value()
                        .attr("height")
                        .and_then(|h| h.parse::<u32>().ok());

                    images.push(ImageInfo {
                        url: absolute_url.as_str().to_string(),
                        alt,
                        title,
                        width,
                        height,
                    });
                }
            }
        }

        Ok(images)
    }

    /// Extrae todos los datos de una página
    pub fn extract_all(&self, html: &str, base_url: &str) -> Result<ExtractedData> {
        let document = Html::parse_document(html);
        let _base = Url::parse(base_url).context("Error parseando URL base")?;

        // Título
        let title_selector = Selector::parse("title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string());

        // Meta tags
        let mut meta = HashMap::new();
        for element in document.select(&self.meta_selector) {
            if let Some(name) = element.value().attr("name") {
                if let Some(content) = element.value().attr("content") {
                    meta.insert(name.to_string(), content.to_string());
                }
            } else if let Some(property) = element.value().attr("property") {
                if let Some(content) = element.value().attr("content") {
                    meta.insert(property.to_string(), content.to_string());
                }
            }
        }

        // Links
        let links = self.extract_link_info(html, base_url)?;

        // Imágenes
        let images = self.extract_images(html, base_url)?;

        // Texto contenido (sin HTML)
        let text_selector = Selector::parse("body").unwrap();
        let text_content = document
            .select(&text_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default();

        // Headings
        let mut headings = Vec::new();
        for element in document.select(&self.heading_selector) {
            let tag_name = element.value().name();
            let level = tag_name
                .chars()
                .nth(1)
                .and_then(|c| c.to_digit(10))
                .map(|n| n as u8)
                .unwrap_or(0);

            let text = element.text().collect::<String>().trim().to_string();
            let id = element.value().attr("id").map(|s| s.to_string());

            headings.push(HeadingInfo { level, text, id });
        }

        // Tablas
        let mut tables = Vec::new();
        for table_element in document.select(&self.table_selector) {
            let header_selector =
                Selector::parse("thead th, tr:first-child th, tr:first-child td").unwrap();
            let headers: Vec<String> = table_element
                .select(&header_selector)
                .map(|e| e.text().collect::<String>().trim().to_string())
                .collect();

            let row_selector = Selector::parse("tbody tr, tr:not(:first-child)").unwrap();
            let mut rows = Vec::new();

            for row in table_element.select(&row_selector) {
                let cell_selector = Selector::parse("td, th").unwrap();
                let cells: Vec<String> = row
                    .select(&cell_selector)
                    .map(|e| e.text().collect::<String>().trim().to_string())
                    .collect();

                if !cells.is_empty() {
                    rows.push(cells);
                }
            }

            if !headers.is_empty() || !rows.is_empty() {
                tables.push(TableData { headers, rows });
            }
        }

        Ok(ExtractedData {
            title,
            meta,
            links,
            images,
            text_content,
            headings,
            tables,
            custom_data: HashMap::new(),
        })
    }

    /// Extrae datos usando un selector CSS personalizado
    pub fn extract_by_selector(&self, html: &str, selector: &str) -> Result<Vec<String>> {
        let document = Html::parse_document(html);
        let css_selector = Selector::parse(selector)
            .map_err(|e| anyhow!("Error parseando selector CSS: {:?}", e))?;

        let mut results = Vec::new();
        for element in document.select(&css_selector) {
            let text = element.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                results.push(text);
            }
        }

        Ok(results)
    }

    // Funciones de parser integradas
    pub fn parse(&self, html: &str) -> Html {
        Html::parse_document(html)
    }

    pub fn extract_title(&self, html: &str) -> Option<String> {
        let doc = Html::parse_document(html);
        let selector = Selector::parse("title").ok()?;
        doc.select(&selector)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
    }

    pub fn extract_meta_tags(&self, html: &str) -> serde_json::Map<String, serde_json::Value> {
        let doc = Html::parse_document(html);
        let selector = Selector::parse("meta").unwrap();
        let mut meta = serde_json::Map::new();

        for element in doc.select(&selector) {
            if let Some(name) = element.value().attr("name") {
                if let Some(content) = element.value().attr("content") {
                    meta.insert(
                        name.to_string(),
                        serde_json::Value::String(content.to_string()),
                    );
                }
            } else if let Some(property) = element.value().attr("property") {
                if let Some(content) = element.value().attr("content") {
                    meta.insert(
                        property.to_string(),
                        serde_json::Value::String(content.to_string()),
                    );
                }
            }
        }
        meta
    }
}
