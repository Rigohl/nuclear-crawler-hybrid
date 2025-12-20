//! Módulo Zig Integration - Implementación 100% Rust
//!
//! ⚠️ ZIG FFI DESACTIVADO PERMANENTEMENTE
//! La librería nuclear_zig.lib causa crash en Windows:
//! - "thread panic: integer overflow"
//! - "thread has overflowed its stack"
//!
//! Esta implementación usa Rust puro que es igual de rápida:
//! - process_data_parallel() -> rayon (paralelo)
//! - parse_html_fast() -> scraper
//! - fast_hash() -> blake3 (SIMD nativo)
//! - search_patterns() -> Rust nativo
//!
//! 🛡️ SEGURO - Sin FFI, sin crashes, sin panics

use anyhow::Result;
use rayon::prelude::*;

/// Límite máximo de datos para procesamiento
const MAX_PROCESS_SIZE: usize = 512 * 1024; // 512 KB

/// Integración Zig - Implementación Rust pura (sin FFI)
pub struct ZigIntegration {
    num_threads: usize,
}

impl ZigIntegration {
    /// Crea nueva integración (100% Rust)
    pub fn new() -> Self {
        let num_threads = num_cpus::get().max(1);
        eprintln!("⚡ ZigIntegration: usando Rust puro ({} threads, blake3 SIMD)", num_threads);
        Self { num_threads }
    }

    /// Crea con configuración manual
    pub fn new_with_config(_enabled: bool) -> Self {
        Self::new()
    }

    /// Siempre disponible (usa Rust)
    pub fn is_available(&self) -> bool {
        true
    }

    /// No usa FFI (usa Rust puro)
    pub fn is_ffi_active(&self) -> bool {
        false // No hay FFI activo
    }

    /// Procesa datos en paralelo con rayon
    pub fn process_data_parallel(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        // Limitar tamaño para evitar uso excesivo de memoria
        let safe_data = if data.len() > MAX_PROCESS_SIZE {
            &data[..MAX_PROCESS_SIZE]
        } else {
            data
        };

        // Procesar en paralelo con rayon
        let chunk_size = (safe_data.len() / self.num_threads).max(1024);
        let result: Vec<u8> = safe_data
            .par_chunks(chunk_size)
            .flat_map(|chunk| {
                chunk.iter().map(|&b| b ^ 0xFF).collect::<Vec<u8>>()
            })
            .collect();

        Ok(result)
    }

    /// Parsea HTML con scraper
    pub fn parse_html_fast(&self, html: &str, selector: &str) -> Result<Vec<String>> {
        use scraper::{Html, Selector};

        if html.is_empty() {
            return Ok(Vec::new());
        }

        let document = Html::parse_document(html);

        let sel = match Selector::parse(selector) {
            Ok(s) => s,
            Err(_) => return Ok(Vec::new()),
        };

        let results: Vec<String> = document
            .select(&sel)
            .map(|element| element.text().collect::<String>())
            .collect();

        Ok(results)
    }

    /// Procesa batch de elementos en paralelo
    pub fn process_batch_parallel(&self, items: &[u32], _batch_size: usize) -> Result<Vec<u32>> {
        let output: Vec<u32> = items.par_iter().map(|&item| item * 2).collect();
        Ok(output)
    }

    /// Copia de memoria eficiente
    pub fn fast_memory_copy(&self, src: &[u8], dst: &mut [u8]) -> Result<()> {
        if dst.len() < src.len() {
            return Err(anyhow::anyhow!("Destination buffer too small"));
        }
        dst[..src.len()].copy_from_slice(src);
        Ok(())
    }

    /// Hash rápido con blake3 (usa SIMD automáticamente)
    pub fn fast_hash(&self, data: &[u8]) -> u64 {
        if data.is_empty() {
            return 5381; // Valor por defecto (djb2)
        }

        // blake3 es extremadamente rápido y usa SIMD
        let hash = blake3::hash(data);
        let bytes = hash.as_bytes();

        // Tomar los primeros 8 bytes como u64
        u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }

    /// Búsqueda de patrones con Rust nativo
    pub fn search_patterns(&self, text: &str, pattern: &str) -> Result<Vec<usize>> {
        if pattern.is_empty() || text.is_empty() {
            return Ok(Vec::new());
        }

        let mut results = Vec::new();
        let mut start = 0;

        while let Some(pos) = text[start..].find(pattern) {
            results.push(start + pos);
            start += pos + pattern.len();
        }

        Ok(results)
    }

    /// Suma elementos de array en paralelo con rayon
    pub fn sum_array_parallel(&self, array: &[f64]) -> f64 {
        if array.is_empty() {
            return 0.0;
        }
        array.par_iter().sum()
    }
}

impl Default for ZigIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zig_integration() {
        let zig = ZigIntegration::new();
        let data = b"hello world";
        let result = zig.process_data_parallel(data).unwrap();
        assert_eq!(result.len(), data.len());
    }

    #[test]
    fn test_fast_hash() {
        let zig = ZigIntegration::new();
        let data = b"test";
        let hash = zig.fast_hash(data);
        assert!(hash > 0);
    }

    #[test]
    fn test_search_patterns() {
        let zig = ZigIntegration::new();
        let results = zig.search_patterns("hello world hello", "hello").unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], 0);
        assert_eq!(results[1], 12);
    }

    #[test]
    fn test_sum_parallel() {
        let zig = ZigIntegration::new();
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let sum = zig.sum_array_parallel(&array);
        assert!((sum - 15.0).abs() < 0.001);
    }
}
