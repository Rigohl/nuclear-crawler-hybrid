//! Nim integration with explicit real-backend enforcement.

use anyhow::{Context, Result};
use libloading::Library;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::runtime::Builder;

use crate::ffi::wasm_ffi_bridge::{Element, Link, NimWasmParser, WasmConfig};

#[cfg(has_nim)]
extern "C" {}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct NimHtmlResult {
    pub title: *const i8,
    pub content: *const i8,
    pub links: *const i8,
    pub images: *const i8,
    pub error: *const i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NimParserConfig {
    pub enable_javascript_extraction: bool,
    pub extract_metadata: bool,
    pub follow_redirects: bool,
    pub timeout_ms: u32,
    pub max_content_length: usize,
}

impl Default for NimParserConfig {
    fn default() -> Self {
        Self {
            enable_javascript_extraction: true,
            extract_metadata: true,
            follow_redirects: true,
            timeout_ms: 10000,
            max_content_length: 10 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NimParsedContent {
    pub title: String,
    pub text_content: String,
    pub links: Vec<String>,
    pub images: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub structured_data: serde_json::Value,
    pub word_count: usize,
    pub language: String,
    pub has_javascript: bool,
}

pub struct NimHtmlParser {
    config: NimParserConfig,
    library: Option<Library>,
    wasm_runtime: Option<Mutex<NimWasmParser>>,
}

impl NimHtmlParser {
    /// Initializes the parser and requires a real backend.
    pub fn new(config: NimParserConfig) -> Result<Self> {
        let library = Self::load_nim_library();
        let wasm_runtime = NimWasmParser::new(WasmConfig::default()).ok().map(Mutex::new);

        if library.is_none() && wasm_runtime.is_none() {
            return Err(anyhow::anyhow!(
                "Nim backend unavailable. Build native Nim FFI or ffi/wasm/nim/nim_parser.wasm"
            ));
        }

        Ok(Self {
            config,
            library,
            wasm_runtime,
        })
    }

    /// Parses HTML through native Nim FFI or the Nim WASM runtime.
    pub fn parse_html(&self, html: &str, url: Option<&str>) -> Result<NimParsedContent> {
        self.ensure_content_length(html)?;

        if let Some(ref lib) = self.library {
            if let Ok(result) = self.nim_parse_html_ffi(lib, html, url) {
                return Ok(result);
            }
        }

        if self.wasm_runtime.is_some() {
            return self.parse_html_via_wasm(html, url);
        }

        Err(anyhow::anyhow!("Nim parsing backend unavailable"))
    }

    /// Parses several pages through the same explicit backend contract.
    pub fn parse_html_batch(
        &self,
        html_pages: Vec<(String, Option<String>)>,
    ) -> Result<Vec<NimParsedContent>> {
        let mut results = Vec::with_capacity(html_pages.len());
        for (html, url) in html_pages {
            results.push(self.parse_html(&html, url.as_deref())?);
        }
        Ok(results)
    }

    /// Extracts visible body text through the real parsing backend.
    pub fn extract_clean_text(&self, html: &str) -> Result<String> {
        Ok(self.parse_html(html, None)?.text_content)
    }

    /// Checks whether at least one real backend is available.
    pub fn is_available(&self) -> bool {
        self.library.is_some() || self.wasm_runtime.is_some()
    }

    fn load_nim_library() -> Option<Library> {
        let lib_paths = [
            "nim/nuclear_nim.dll",
            "nim/nuclear_nim.lib",
            "libs/nuclear_nim.lib",
        ];

        for lib_path in &lib_paths {
            match unsafe { Library::new(*lib_path) } {
                Ok(lib) => return Some(lib),
                Err(_) => continue,
            }
        }

        None
    }

    fn nim_parse_html_ffi(
        &self,
        _lib: &Library,
        _html: &str,
        _url: Option<&str>,
    ) -> Result<NimParsedContent> {
        Err(anyhow::anyhow!(
            "Native Nim FFI result conversion is not implemented; use the real WASM backend"
        ))
    }

    fn parse_html_via_wasm(&self, html: &str, _url: Option<&str>) -> Result<NimParsedContent> {
        let wasm_runtime = self
            .wasm_runtime
            .as_ref()
            .context("Nim WASM runtime not initialized")?;
        let mut runtime = wasm_runtime.lock();
        let executor = Builder::new_current_thread().enable_all().build()?;

        let title_elements = executor.block_on(runtime.parse_html(html, "title"))?;
        let body_elements = executor.block_on(runtime.parse_html(html, "body"))?;
        let link_elements = executor.block_on(runtime.extract_links(html))?;
        let image_elements = executor.block_on(runtime.parse_html(html, "img"))?;

        Ok(self.assemble_parsed_content(
            html,
            title_elements,
            body_elements,
            link_elements,
            image_elements,
        ))
    }

    fn ensure_content_length(&self, html: &str) -> Result<()> {
        if html.len() > self.config.max_content_length {
            return Err(anyhow::anyhow!(
                "HTML content exceeds maximum length of {} bytes",
                self.config.max_content_length
            ));
        }
        Ok(())
    }

    fn assemble_parsed_content(
        &self,
        html: &str,
        title_elements: Vec<Element>,
        body_elements: Vec<Element>,
        link_elements: Vec<Link>,
        image_elements: Vec<Element>,
    ) -> NimParsedContent {
        let title = title_elements
            .first()
            .map(|element| element.text.clone())
            .unwrap_or_default();
        let text_content = body_elements
            .iter()
            .map(|element| element.text.trim())
            .filter(|text| !text.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        let links = link_elements.into_iter().map(|link| link.url).collect::<Vec<_>>();
        let images = image_elements
            .into_iter()
            .filter_map(|element| element.attributes.get("src").cloned())
            .collect::<Vec<_>>();

        NimParsedContent {
            title,
            text_content: text_content.clone(),
            links,
            images,
            metadata: HashMap::new(),
            structured_data: serde_json::Value::Null,
            word_count: text_content.split_whitespace().count(),
            language: Self::detect_language(&text_content),
            has_javascript: html.contains("<script") || html.contains("javascript:"),
        }
    }

    fn detect_language(text_content: &str) -> String {
        if text_content.contains("the ") || text_content.contains(" and ") {
            "english".to_string()
        } else if text_content.contains(" el ") || text_content.contains(" la ") {
            "spanish".to_string()
        } else {
            "unknown".to_string()
        }
    }
}

impl Default for NimHtmlParser {
    fn default() -> Self {
        Self::new(NimParserConfig::default())
            .expect("NimHtmlParser requires native Nim FFI or ffi/wasm/nim/nim_parser.wasm")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nim_backend_contract_is_explicit() {
        match NimHtmlParser::new(NimParserConfig::default()) {
            Ok(parser) => assert!(parser.is_available()),
            Err(err) => assert!(err.to_string().contains("Nim backend unavailable")),
        }
    }

    #[test]
    fn test_language_detection() {
        assert_eq!(NimHtmlParser::detect_language("the system and the parser"), "english");
        assert_eq!(NimHtmlParser::detect_language(" el parser y la pagina"), "spanish");
    }
}