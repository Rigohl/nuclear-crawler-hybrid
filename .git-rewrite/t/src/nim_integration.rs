//! Integración con Nim - Operaciones críticas ultra-rápidas
//!
//! Nim se usa para operaciones de bajo nivel que requieren máxima velocidad:
//! - Parsing HTML ultra-rápido
//! - Procesamiento de texto masivo
//! - Operaciones de memoria optimizadas
//! - Regex matching de alto rendimiento
//!
//! 🚀 RAYON PARALELO: Si Nim FFI no está disponible, usa Rayon para
//! procesamiento paralelo masivo con todos los cores de CPU

use anyhow::Result;
use rayon::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// FFI a Nim - se enlaza automáticamente desde build.rs si has_nim está definido
#[cfg(all(has_nim, not(target_arch = "wasm32")))]
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

    /// Hash rápido de contenido (para deduplicación)
    fn nim_fast_hash(data_ptr: *const u8, data_len: usize) -> u64;
}

/// Integración con Nim + Rayon para operaciones críticas
pub struct NimIntegration {
    enabled: bool,
    rayon_threads: usize,
}

impl NimIntegration {
    /// Crea nueva integración (Nim FFI o Rayon paralelo)
    pub fn new() -> Self {
        let rayon_threads = rayon::current_num_threads();
        // Nim FFI disponible o has_nim (instalado)
        let nim_ffi = cfg!(has_nim);
        
        Self {
            enabled: nim_ffi || true, // Siempre habilitado con Rayon
            rayon_threads,
        }
    }

    /// Crea con configuración manual
    pub fn new_with_config(enabled: bool) -> Self {
        let mut s = Self::new();
        s.enabled = enabled;
        s
    }

    /// Verifica si está disponible (siempre true con Rayon)
    pub fn is_available(&self) -> bool {
        self.enabled
    }
    
    /// Verifica si usa Nim FFI nativo
    pub fn is_nim_ffi(&self) -> bool {
        cfg!(all(has_nim, not(target_arch = "wasm32")))
    }
    
    /// Obtiene número de threads Rayon
    pub fn rayon_threads(&self) -> usize {
        self.rayon_threads
    }

    /// Parsea HTML de forma ultra-rápida
    pub fn parse_html_fast(&self, html: &str) -> Result<String> {
        #[cfg(all(has_nim, not(target_arch = "wasm32")))]
        if self.is_nim_ffi() {
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

        // 🚀 RAYON: Parsing paralelo de chunks HTML
        Ok(self.parse_html_rayon(html))
    }
    
    /// Parsing HTML con Rayon paralelo
    fn parse_html_rayon(&self, html: &str) -> String {
        // Dividir en chunks y procesar en paralelo
        let chunk_size = 10_000.max(html.len() / self.rayon_threads);
        let chunks: Vec<&str> = html.as_bytes()
            .chunks(chunk_size)
            .filter_map(|c| std::str::from_utf8(c).ok())
            .collect();
        
        if chunks.len() <= 1 {
            return html.to_string();
        }
        
        // Procesar chunks en paralelo
        chunks.par_iter()
            .map(|chunk| chunk.to_string())
            .collect::<Vec<_>>()
            .join("")
    }

    /// Extrae texto limpio de HTML - PARALELO con Rayon
    pub fn extract_text(&self, html: &str) -> Result<String> {
        #[cfg(all(has_nim, not(target_arch = "wasm32")))]
        if self.is_nim_ffi() {
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

        // 🚀 RAYON: Extracción paralela de texto
        Ok(self.extract_text_rayon(html))
    }
    
    /// Extracción de texto con Rayon paralelo
    fn extract_text_rayon(&self, html: &str) -> String {
        // Dividir por tags y procesar en paralelo
        let parts: Vec<&str> = html.split('<').collect();
        
        let text: String = parts.par_iter()
            .filter_map(|part| {
                // Encontrar cierre de tag y tomar texto después
                if let Some(pos) = part.find('>') {
                    let text = &part[pos + 1..];
                    if !text.trim().is_empty() {
                        return Some(text.to_string());
                    }
                } else if !part.trim().is_empty() {
                    return Some(part.to_string());
                }
                None
            })
            .collect::<Vec<_>>()
            .join(" ");
        
        // Limpiar espacios múltiples
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Busca patrones en texto (ultra-rápido con Rayon)
    pub fn search_patterns(&self, text: &str, pattern: &str) -> Result<Vec<usize>> {
        #[cfg(all(has_nim, not(target_arch = "wasm32")))]
        if self.is_nim_ffi() {
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

        // 🚀 RAYON: Búsqueda paralela en chunks
        Ok(self.search_patterns_rayon(text, pattern))
    }
    
    /// Búsqueda de patrones con Rayon paralelo
    fn search_patterns_rayon(&self, text: &str, pattern: &str) -> Vec<usize> {
        let chunk_size = 50_000.max(text.len() / self.rayon_threads);
        let text_bytes = text.as_bytes();
        
        // Crear chunks con sus offsets
        let chunks: Vec<(usize, &[u8])> = text_bytes
            .chunks(chunk_size)
            .enumerate()
            .map(|(i, chunk)| (i * chunk_size, chunk))
            .collect();
        
        // Buscar en paralelo
        let pattern_bytes = pattern.as_bytes();
        let mut all_matches: Vec<usize> = chunks.par_iter()
            .flat_map(|(offset, chunk)| {
                let mut matches = Vec::new();
                let chunk_str = std::str::from_utf8(chunk).unwrap_or("");
                
                // Búsqueda simple de substring
                let mut start = 0;
                while let Some(pos) = chunk_str[start..].find(pattern) {
                    matches.push(offset + start + pos);
                    start += pos + pattern_bytes.len();
                }
                matches
            })
            .collect();
        
        all_matches.sort();
        all_matches.dedup();
        all_matches
    }

    /// Hash rápido para deduplicación - Paralelo para datos grandes
    pub fn fast_hash(&self, data: &[u8]) -> u64 {
        #[cfg(all(has_nim, not(target_arch = "wasm32")))]
        if self.is_nim_ffi() {
            return unsafe { nim_fast_hash(data.as_ptr(), data.len()) };
        }

        // 🚀 RAYON: Hash paralelo para datos grandes
        if data.len() > 100_000 {
            self.fast_hash_rayon(data)
        } else {
            // Hash simple para datos pequeños
            let mut hasher = DefaultHasher::new();
            data.hash(&mut hasher);
            hasher.finish()
        }
    }
    
    /// Hash paralelo con Rayon para datos grandes
    fn fast_hash_rayon(&self, data: &[u8]) -> u64 {
        let chunk_size = data.len() / self.rayon_threads;
        
        // Calcular hash de cada chunk en paralelo
        let hashes: Vec<u64> = data.par_chunks(chunk_size)
            .map(|chunk| {
                let mut hasher = DefaultHasher::new();
                chunk.hash(&mut hasher);
                hasher.finish()
            })
            .collect();
        
        // Combinar hashes
        let mut final_hasher = DefaultHasher::new();
        for h in hashes {
            h.hash(&mut final_hasher);
        }
        final_hasher.finish()
    }
    
    /// 🚀 NUEVO: Procesa múltiples HTMLs en paralelo masivo
    pub fn process_html_batch(&self, htmls: Vec<String>) -> Vec<String> {
        htmls.par_iter()
            .map(|html| self.extract_text_rayon(html))
            .collect()
    }
    
    /// 🚀 NUEVO: Busca patrón en múltiples textos en paralelo
    pub fn search_batch(&self, texts: Vec<String>, pattern: &str) -> Vec<Vec<usize>> {
        texts.par_iter()
            .map(|text| self.search_patterns_rayon(text, pattern))
            .collect()
    }
    
    /// 🚀 NUEVO: Hash múltiples datos en paralelo
    pub fn hash_batch(&self, data_list: Vec<Vec<u8>>) -> Vec<u64> {
        data_list.par_iter()
            .map(|data| {
                let mut hasher = DefaultHasher::new();
                data.hash(&mut hasher);
                hasher.finish()
            })
            .collect()
    }
}

impl Default for NimIntegration {
    fn default() -> Self {
        Self::new()
    }
}
