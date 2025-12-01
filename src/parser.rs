//! Módulo Parser - Parsing avanzado de HTML/XML
//!
//! Procesamiento de documentos HTML y XML con múltiples estrategias

use scraper::{Html, Selector};
use serde_json::Value;
#[allow(unused_imports)]
use std::collections::HashMap;

/// Parser HTML principal
pub struct HtmlParser {}

impl HtmlParser {
    pub fn new() -> Self {
        Self {}
    }

    /// Parsea un documento HTML
    pub fn parse(&self, html: &str) -> Html {
        Html::parse_document(html)
    }

    /// Extrae el título de la página
    pub fn extract_title(&self, doc: &Html) -> Option<String> {
        let selector = Selector::parse("title").ok()?;
        doc.select(&selector)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
    }

    /// Extrae meta tags
    pub fn extract_meta_tags(&self, doc: &Html) -> serde_json::Map<String, Value> {
        let selector = Selector::parse("meta").unwrap();
        let mut meta = serde_json::Map::new();

        for element in doc.select(&selector) {
            if let Some(name) = element.value().attr("name") {
                if let Some(content) = element.value().attr("content") {
                    meta.insert(name.to_string(), Value::String(content.to_string()));
                }
            } else if let Some(property) = element.value().attr("property") {
                if let Some(content) = element.value().attr("content") {
                    meta.insert(property.to_string(), Value::String(content.to_string()));
                }
            }
        }

        meta
    }

    /// Extrae todos los links
    pub fn extract_links(&self, doc: &Html) -> Vec<Value> {
        let selector = Selector::parse("a[href]").unwrap();
        let mut links = Vec::new();

        for element in doc.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                let text = element.text().collect::<String>().trim().to_string();
                let mut link_obj = serde_json::Map::new();
                link_obj.insert("url".to_string(), Value::String(href.to_string()));
                link_obj.insert("text".to_string(), Value::String(text));

                if let Some(title) = element.value().attr("title") {
                    link_obj.insert("title".to_string(), Value::String(title.to_string()));
                }

                links.push(Value::Object(link_obj));
            }
        }

        links
    }

    /// Extrae todas las imágenes
    pub fn extract_images(&self, doc: &Html) -> Vec<Value> {
        let selector = Selector::parse("img").unwrap();
        let mut images = Vec::new();

        for element in doc.select(&selector) {
            if let Some(src) = element.value().attr("src") {
                let mut img_obj = serde_json::Map::new();
                img_obj.insert("url".to_string(), Value::String(src.to_string()));

                if let Some(alt) = element.value().attr("alt") {
                    img_obj.insert("alt".to_string(), Value::String(alt.to_string()));
                }

                if let Some(title) = element.value().attr("title") {
                    img_obj.insert("title".to_string(), Value::String(title.to_string()));
                }

                images.push(Value::Object(img_obj));
            }
        }

        images
    }
}

impl Default for HtmlParser {
    fn default() -> Self {
        Self::new()
    }
}
