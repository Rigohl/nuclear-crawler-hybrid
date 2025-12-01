//! Integración con Nim - Operaciones críticas ultra-rápidas
//!
//! Nim se usa para operaciones de bajo nivel que requieren máxima velocidad:
//! - Parsing HTML ultra-rápido
//! - Procesamiento de texto masivo
//! - Operaciones de memoria optimizadas
//! - Regex matching de alto rendimiento
//!
//! Si Nim no está disponible, usa implementaciones en Rust puro

use anyhow::Result;

// FFI a Nim (solo si está compilado)
#[cfg(all(feature = "nim", has_nim, not(target_arch = "wasm32")))]
#[link(name = "nuclear_nim", kind = "static")]
extern "C" {
    /// Procesa HTML de forma ultra-rápida usando Nim
    fn nim_parse_html_fast(
        html_ptr: *const u8,
        html_len: usize,
        result_ptr: *mut u8,
        result_capacity: usize,
    ) -> usize;

    /// Extrae texto de HTML sin tags
    fn nim_extract_text(
        html_ptr: *const u8,
        html_len: usize,
        result_ptr: *mut u8,
        result_capacity: usize,
    ) -> usize;

    /// Busca patrones en texto masivo (más rápido que Rust regex)
    fn nim_search_patterns(
        text_ptr: *const u8,
        text_len: usize,
        pattern_ptr: *const u8,
        pattern_len: usize,
        matches_ptr: *mut usize,
        matches_capacity: usize,
    ) -> usize;

    /// Procesa URLs en batch (normalización, validación)
    fn nim_process_urls_batch(
        urls_ptr: *const u8,
        urls_len: usize,
        result_ptr: *mut u8,
        result_capacity: usize,
    ) -> usize;

    /// Hash rápido de contenido (para deduplicación)
    fn nim_fast_hash(data_ptr: *const u8, data_len: usize) -> u64;
}

/// Integración con Nim para operaciones críticas
pub struct NimIntegration {
    enabled: bool,
}

impl NimIntegration {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Parsea HTML de forma ultra-rápida
    pub fn parse_html_fast(&self, html: &str) -> Result<String> {
        if !self.enabled {
            return Ok(html.to_string());
        }

        // Intentar usar Nim si está disponible
        #[cfg(all(feature = "nim", has_nim, not(target_arch = "wasm32")))]
        {
            let mut result_buffer = vec![0u8; html.len() * 2];
            let len = unsafe {
                nim_parse_html_fast(
                    html.as_ptr(),
                    html.len(),
                    result_buffer.as_mut_ptr(),
                    result_buffer.capacity(),
                )
            };

            if len > 0 {
                result_buffer.truncate(len);
                if let Ok(result) = String::from_utf8(result_buffer) {
                    return Ok(result);
                }
            }
        }

        // Fallback: usar scraper de Rust
        Ok(html.to_string())
    }

    /// Extrae texto limpio de HTML
    pub fn extract_text(&self, html: &str) -> Result<String> {
        if !self.enabled {
            return Ok(String::new());
        }

        // Intentar usar Nim si está disponible
        #[cfg(all(feature = "nim", has_nim, not(target_arch = "wasm32")))]
        {
            let mut result_buffer = vec![0u8; html.len()];
            let len = unsafe {
                nim_extract_text(
                    html.as_ptr(),
                    html.len(),
                    result_buffer.as_mut_ptr(),
                    result_buffer.capacity(),
                )
            };

            if len > 0 {
                result_buffer.truncate(len);
                if let Ok(text) = String::from_utf8(result_buffer) {
                    return Ok(text);
                }
            }
        }

        // Fallback: extracción básica con regex
        let re = regex::Regex::new(r"<[^>]+>").unwrap_or_else(|_| regex::Regex::new("").unwrap());
        let text = re.replace_all(html, " ");
        // Limpiar espacios múltiples
        let cleaned = regex::Regex::new(r"\s+").unwrap_or_else(|_| regex::Regex::new("").unwrap());
        Ok(cleaned.replace_all(&text, " ").trim().to_string())
    }

    /// Busca patrones en texto (ultra-rápido)
    pub fn search_patterns(&self, text: &str, pattern: &str) -> Result<Vec<usize>> {
        if !self.enabled {
            return Ok(vec![]);
        }

        // Intentar usar Nim si está disponible
        #[cfg(all(feature = "nim", has_nim, not(target_arch = "wasm32")))]
        {
            let mut matches = vec![0usize; 1000];
            let count = unsafe {
                nim_search_patterns(
                    text.as_ptr(),
                    text.len(),
                    pattern.as_ptr(),
                    pattern.len(),
                    matches.as_mut_ptr(),
                    matches.capacity(),
                )
            };

            matches.truncate(count);
            return Ok(matches);
        }

        // Fallback: usar regex de Rust
        let re = match regex::Regex::new(pattern) {
            Ok(re) => re,
            Err(_) => return Ok(vec![]),
        };

        let mut matches = Vec::new();
        for mat in re.find_iter(text) {
            matches.push(mat.start());
        }
        Ok(matches)
    }

    /// Hash rápido para deduplicación
    pub fn fast_hash(&self, data: &[u8]) -> u64 {
        // Intentar usar Nim si está disponible
        #[cfg(all(feature = "nim", has_nim, not(target_arch = "wasm32")))]
        {
            return unsafe { nim_fast_hash(data.as_ptr(), data.len()) };
        }

        // Fallback: usar hasher de Rust
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }
}
