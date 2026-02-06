//! 🔥 DATA EXTRACTION ENGINE - Advanced Data Extraction
//!
//! Motor de extracción de datos avanzado con múltiples formatos
//! Soporta JSON, HTML, XML, CSV, y datos estructurados

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Resultado de extracción de datos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResult {
    pub source_url: String,
    pub data_type: DataType,
    pub extracted_fields: HashMap<String, ExtractedField>,
    pub raw_content: String,
    pub structured_data: Option<serde_json::Value>,
    pub extraction_time_ms: u64,
    pub confidence_score: f32,
}

/// Tipo de datos detectado
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    Html,
    Json,
    Xml,
    Csv,
    PlainText,
    Binary,
    Mixed,
    Unknown,
}

/// Campo extraído con metadatos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedField {
    pub name: String,
    pub value: String,
    pub field_type: FieldType,
    pub confidence: f32,
    pub source_location: Option<String>,
}

/// Tipo de campo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FieldType {
    Text,
    Number,
    Date,
    Email,
    Url,
    Phone,
    Currency,
    Percentage,
    Boolean,
    List,
    Object,
    Unknown,
}

/// Configuración de extracción
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionConfig {
    /// Extraer enlaces
    pub extract_links: bool,
    /// Extraer imágenes
    pub extract_images: bool,
    /// Extraer metadatos
    pub extract_metadata: bool,
    /// Extraer datos estructurados (JSON-LD, microdata)
    pub extract_structured_data: bool,
    /// Extraer tablas
    pub extract_tables: bool,
    /// Extraer formularios
    pub extract_forms: bool,
    /// Extraer emails
    pub extract_emails: bool,
    /// Extraer teléfonos
    pub extract_phones: bool,
    /// Extraer precios
    pub extract_prices: bool,
    /// Máxima profundidad de extracción
    pub max_depth: u32,
}

impl Default for ExtractionConfig {
    fn default() -> Self {
        Self {
            extract_links: true,
            extract_images: true,
            extract_metadata: true,
            extract_structured_data: true,
            extract_tables: true,
            extract_forms: true,
            extract_emails: true,
            extract_phones: true,
            extract_prices: true,
            max_depth: 3,
        }
    }
}

/// Motor de extracción de datos
pub struct DataExtractionEngine {
    config: ExtractionConfig,
}

impl DataExtractionEngine {
    /// Crear nuevo motor de extracción
    pub fn new(config: ExtractionConfig) -> Self {
        eprintln!("🔥 Initializing Data Extraction Engine");
        Self { config }
    }

    /// Extraer datos de contenido HTML
    pub fn extract_from_html(&self, html: &str, source_url: &str) -> Result<ExtractionResult> {
        use scraper::{Html, Selector};
        use std::time::Instant;

        let start = Instant::now();
        let document = Html::parse_document(html);
        let mut fields = HashMap::new();

        // Extraer título
        if let Ok(selector) = Selector::parse("title") {
            if let Some(title) = document.select(&selector).next() {
                fields.insert(
                    "title".to_string(),
                    ExtractedField {
                        name: "title".to_string(),
                        value: title.text().collect::<String>().trim().to_string(),
                        field_type: FieldType::Text,
                        confidence: 1.0,
                        source_location: Some("head > title".to_string()),
                    },
                );
            }
        }

        // Extraer enlaces
        if self.config.extract_links {
            if let Ok(selector) = Selector::parse("a[href]") {
                let links: Vec<String> = document
                    .select(&selector)
                    .filter_map(|el| el.value().attr("href"))
                    .map(String::from)
                    .collect();

                fields.insert(
                    "links".to_string(),
                    ExtractedField {
                        name: "links".to_string(),
                        value: links.join(", "),
                        field_type: FieldType::List,
                        confidence: 1.0,
                        source_location: Some("a[href]".to_string()),
                    },
                );
            }
        }

        // Extraer imágenes
        if self.config.extract_images {
            if let Ok(selector) = Selector::parse("img[src]") {
                let images: Vec<String> = document
                    .select(&selector)
                    .filter_map(|el| el.value().attr("src"))
                    .map(String::from)
                    .collect();

                fields.insert(
                    "images".to_string(),
                    ExtractedField {
                        name: "images".to_string(),
                        value: images.join(", "),
                        field_type: FieldType::List,
                        confidence: 1.0,
                        source_location: Some("img[src]".to_string()),
                    },
                );
            }
        }

        // Extraer metadatos
        if self.config.extract_metadata {
            if let Ok(selector) = Selector::parse("meta") {
                for meta in document.select(&selector) {
                    let name = meta
                        .value()
                        .attr("name")
                        .or_else(|| meta.value().attr("property"))
                        .unwrap_or("");
                    let content = meta.value().attr("content").unwrap_or("");

                    if !name.is_empty() && !content.is_empty() {
                        fields.insert(
                            format!("meta_{}", name),
                            ExtractedField {
                                name: name.to_string(),
                                value: content.to_string(),
                                field_type: FieldType::Text,
                                confidence: 0.9,
                                source_location: Some(format!("meta[name='{}']", name)),
                            },
                        );
                    }
                }
            }
        }

        // Extraer emails con regex
        if self.config.extract_emails {
            let email_pattern =
                regex::Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
            let emails: Vec<String> = email_pattern
                .find_iter(html)
                .map(|m| m.as_str().to_string())
                .collect();

            if !emails.is_empty() {
                fields.insert(
                    "emails".to_string(),
                    ExtractedField {
                        name: "emails".to_string(),
                        value: emails.join(", "),
                        field_type: FieldType::Email,
                        confidence: 0.95,
                        source_location: None,
                    },
                );
            }
        }

        // Extraer teléfonos
        if self.config.extract_phones {
            let phone_pattern = regex::Regex::new(r"[\+]?[(]?[0-9]{1,3}[)]?[-\s\.]?[(]?[0-9]{1,4}[)]?[-\s\.]?[0-9]{1,4}[-\s\.]?[0-9]{1,9}").unwrap();
            let phones: Vec<String> = phone_pattern
                .find_iter(html)
                .map(|m| m.as_str().to_string())
                .filter(|p| p.len() >= 7)
                .collect();

            if !phones.is_empty() {
                fields.insert(
                    "phones".to_string(),
                    ExtractedField {
                        name: "phones".to_string(),
                        value: phones.join(", "),
                        field_type: FieldType::Phone,
                        confidence: 0.8,
                        source_location: None,
                    },
                );
            }
        }

        // Extraer precios
        if self.config.extract_prices {
            let price_pattern = regex::Regex::new(r"[\$€£]\s*[0-9]+[,\.]?[0-9]*").unwrap();
            let prices: Vec<String> = price_pattern
                .find_iter(html)
                .map(|m| m.as_str().to_string())
                .collect();

            if !prices.is_empty() {
                fields.insert(
                    "prices".to_string(),
                    ExtractedField {
                        name: "prices".to_string(),
                        value: prices.join(", "),
                        field_type: FieldType::Currency,
                        confidence: 0.85,
                        source_location: None,
                    },
                );
            }
        }

        // Extraer datos estructurados (JSON-LD)
        let structured_data = if self.config.extract_structured_data {
            self.extract_json_ld(html)
        } else {
            None
        };

        let extraction_time_ms = start.elapsed().as_millis() as u64;
        let confidence_score =
            fields.values().map(|f| f.confidence).sum::<f32>() / fields.len().max(1) as f32;

        Ok(ExtractionResult {
            source_url: source_url.to_string(),
            data_type: DataType::Html,
            extracted_fields: fields,
            raw_content: html.to_string(),
            structured_data,
            extraction_time_ms,
            confidence_score,
        })
    }

    /// Extraer datos JSON-LD de HTML
    fn extract_json_ld(&self, html: &str) -> Option<serde_json::Value> {
        let start_marker = r#"<script type="application/ld+json">"#;
        let end_marker = "</script>";

        if let Some(start_idx) = html.find(start_marker) {
            let json_start = start_idx + start_marker.len();
            if let Some(end_idx) = html[json_start..].find(end_marker) {
                let json_str = &html[json_start..json_start + end_idx];
                if let Ok(json) = serde_json::from_str(json_str) {
                    return Some(json);
                }
            }
        }
        None
    }

    /// Extraer datos de JSON
    pub fn extract_from_json(&self, json_str: &str, source_url: &str) -> Result<ExtractionResult> {
        use std::time::Instant;

        let start = Instant::now();
        let json: serde_json::Value = serde_json::from_str(json_str)?;
        let mut fields = HashMap::new();

        // Extraer campos del JSON
        if let Some(obj) = json.as_object() {
            for (key, value) in obj {
                let (field_type, value_str) = match value {
                    serde_json::Value::String(s) => (FieldType::Text, s.clone()),
                    serde_json::Value::Number(n) => (FieldType::Number, n.to_string()),
                    serde_json::Value::Bool(b) => (FieldType::Boolean, b.to_string()),
                    serde_json::Value::Array(_) => (FieldType::List, value.to_string()),
                    serde_json::Value::Object(_) => (FieldType::Object, value.to_string()),
                    serde_json::Value::Null => (FieldType::Unknown, "null".to_string()),
                };

                fields.insert(
                    key.clone(),
                    ExtractedField {
                        name: key.clone(),
                        value: value_str,
                        field_type,
                        confidence: 1.0,
                        source_location: Some(format!("$.{}", key)),
                    },
                );
            }
        }

        let extraction_time_ms = start.elapsed().as_millis() as u64;

        Ok(ExtractionResult {
            source_url: source_url.to_string(),
            data_type: DataType::Json,
            extracted_fields: fields,
            raw_content: json_str.to_string(),
            structured_data: Some(json),
            extraction_time_ms,
            confidence_score: 1.0,
        })
    }

    /// Detectar tipo de datos automáticamente
    pub fn detect_data_type(&self, content: &str) -> DataType {
        let trimmed = content.trim();

        // JSON
        if (trimmed.starts_with('{') && trimmed.ends_with('}'))
            || (trimmed.starts_with('[') && trimmed.ends_with(']'))
        {
            if serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
                return DataType::Json;
            }
        }

        // HTML
        if trimmed.contains("<!DOCTYPE") || trimmed.contains("<html") || trimmed.contains("<body") {
            return DataType::Html;
        }

        // XML
        if trimmed.starts_with("<?xml") || (trimmed.starts_with('<') && trimmed.contains("</")) {
            return DataType::Xml;
        }

        // CSV (simple detection)
        if trimmed.lines().count() > 1 {
            let first_line = trimmed.lines().next().unwrap_or("");
            let comma_count = first_line.matches(',').count();
            if comma_count > 0 {
                let consistent = trimmed
                    .lines()
                    .all(|line| line.matches(',').count() == comma_count || line.is_empty());
                if consistent {
                    return DataType::Csv;
                }
            }
        }

        // Plain text
        if trimmed.chars().all(|c| c.is_ascii() || c.is_whitespace()) {
            return DataType::PlainText;
        }

        DataType::Unknown
    }

    /// Extraer automáticamente basado en tipo detectado
    pub fn auto_extract(&self, content: &str, source_url: &str) -> Result<ExtractionResult> {
        match self.detect_data_type(content) {
            DataType::Html => self.extract_from_html(content, source_url),
            DataType::Json => self.extract_from_json(content, source_url),
            _ => {
                // Fallback: tratar como HTML
                self.extract_from_html(content, source_url)
            }
        }
    }
}

impl Default for DataExtractionEngine {
    fn default() -> Self {
        Self::new(ExtractionConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_type_detection() {
        let engine = DataExtractionEngine::default();

        assert_eq!(
            engine.detect_data_type(r#"{"key": "value"}"#),
            DataType::Json
        );
        assert_eq!(
            engine.detect_data_type("<html><body>test</body></html>"),
            DataType::Html
        );
        assert_eq!(
            engine.detect_data_type("a,b,c\n1,2,3\n4,5,6"),
            DataType::Csv
        );
    }

    #[test]
    fn test_html_extraction() {
        let engine = DataExtractionEngine::default();
        let html = r#"<html><head><title>Test Page</title></head><body><a href="https://example.com">Link</a></body></html>"#;

        let result = engine.extract_from_html(html, "https://test.com").unwrap();
        assert_eq!(result.data_type, DataType::Html);
        assert!(result.extracted_fields.contains_key("title"));
        assert!(result.extracted_fields.contains_key("links"));
    }

    #[test]
    fn test_json_extraction() {
        let engine = DataExtractionEngine::default();
        let json = r#"{"name": "John", "age": 30, "active": true}"#;

        let result = engine
            .extract_from_json(json, "https://api.test.com")
            .unwrap();
        assert_eq!(result.data_type, DataType::Json);
        assert!(result.extracted_fields.contains_key("name"));
        assert!(result.extracted_fields.contains_key("age"));
    }
}
