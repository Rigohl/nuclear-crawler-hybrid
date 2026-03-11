//! Zig integration with explicit real-backend enforcement.

use anyhow::{Context, Result};
use libloading::{Library, Symbol};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::ffi::CString;
use std::os::raw::c_char;
use tokio::runtime::Builder;

use crate::ffi::wasm_ffi_bridge::{HashAlgorithm, WasmConfig, ZigWasmSimd};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZigSimdConfig {
    pub enable_simd: bool,
    pub hash_algorithm: String,
    pub buffer_size: usize,
    pub parallel_chunks: usize,
}

#[repr(C)]
pub struct ZigSimdConfigFFI {
    pub enable_simd: bool,
    pub hash_algorithm: *const c_char,
    pub buffer_size: usize,
    pub parallel_chunks: usize,
}

impl Default for ZigSimdConfig {
    fn default() -> Self {
        Self {
            enable_simd: true,
            hash_algorithm: "blake3".to_string(),
            buffer_size: 64 * 1024,
            parallel_chunks: 8,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZigHashResult {
    pub hash: String,
    pub algorithm: String,
    pub input_size: usize,
    pub processing_time_ns: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZigPatternResult {
    pub pattern: String,
    pub matches: Vec<usize>,
    pub match_count: usize,
    pub processing_time_ns: u64,
}

pub struct ZigSimdProcessor {
    library: Option<Library>,
    config: ZigSimdConfig,
    wasm_runtime: Option<Mutex<ZigWasmSimd>>,
}

impl ZigSimdProcessor {
    /// Initializes the processor and requires a real backend.
    pub fn new(config: ZigSimdConfig) -> Result<Self> {
        let library = Self::load_zig_library();
        let wasm_runtime = ZigWasmSimd::new(WasmConfig::default()).ok().map(Mutex::new);

        if library.is_none() && wasm_runtime.is_none() {
            return Err(anyhow::anyhow!(
                "Zig backend unavailable. Build native Zig FFI or ffi/wasm/zig/zig_simd.wasm"
            ));
        }

        Ok(Self {
            library,
            config,
            wasm_runtime,
        })
    }

    /// Hashes data through a real Zig backend.
    pub fn hash_data(&self, data: &[u8]) -> Result<ZigHashResult> {
        if data.is_empty() {
            return Ok(ZigHashResult {
                hash: String::new(),
                algorithm: self.config.hash_algorithm.clone(),
                input_size: 0,
                processing_time_ns: 0,
            });
        }

        if let Some(ref wasm_runtime) = self.wasm_runtime {
            let mut runtime = wasm_runtime.lock();
            let executor = Builder::new_current_thread().enable_all().build()?;
            let algorithm = Self::hash_algorithm_from_name(&self.config.hash_algorithm);
            let start = std::time::Instant::now();
            let hash_bytes = executor.block_on(runtime.hash_data(data, algorithm))?;
            let elapsed = start.elapsed();
            let hash_hex = hash_bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();
            return Ok(ZigHashResult {
                hash: hash_hex,
                algorithm: self.config.hash_algorithm.clone(),
                input_size: data.len(),
                processing_time_ns: elapsed.as_nanos() as u64,
            });
        }

        if let Some(ref lib) = self.library {
            return self.zig_hash(lib, data);
        }

        Err(anyhow::anyhow!("Zig hashing backend unavailable"))
    }

    /// Hashes data through the explicit backend contract.
    pub async fn hash_data_wasm(&mut self, data: &[u8]) -> Result<ZigHashResult> {
        self.hash_data(data)
    }

    /// Matches patterns through the real WASM backend.
    pub fn find_patterns(&self, text: &str, patterns: &[String]) -> Result<Vec<ZigPatternResult>> {
        if text.is_empty() || patterns.is_empty() {
            return Ok(Vec::new());
        }

        let wasm_runtime = self
            .wasm_runtime
            .as_ref()
            .context("Zig pattern matching requires ffi/wasm/zig/zig_simd.wasm")?;
        let mut runtime = wasm_runtime.lock();
        let executor = Builder::new_current_thread().enable_all().build()?;
        let mut results = Vec::with_capacity(patterns.len());

        for pattern in patterns {
            let start = std::time::Instant::now();
            let matches = executor.block_on(runtime.pattern_match(text.as_bytes(), pattern.as_bytes()))?;
            let elapsed = start.elapsed();
            let converted = matches.into_iter().map(|pos| pos as usize).collect::<Vec<_>>();
            results.push(ZigPatternResult {
                pattern: pattern.clone(),
                match_count: converted.len(),
                matches: converted,
                processing_time_ns: elapsed.as_nanos() as u64,
            });
        }

        Ok(results)
    }

    /// Matches patterns through the explicit backend contract.
    pub async fn find_patterns_wasm(
        &mut self,
        text: &str,
        patterns: &[String],
    ) -> Result<Vec<ZigPatternResult>> {
        self.find_patterns(text, patterns)
    }

    /// Batch processing requires a wired real export and otherwise fails.
    pub fn process_batch(&self, _texts: Vec<String>) -> Result<Vec<String>> {
        Err(anyhow::anyhow!(
            "Real Zig batch processing export is not wired in this repository; no fallback is allowed"
        ))
    }

    /// Reports whether at least one real backend is available.
    pub fn is_available(&self) -> bool {
        self.library.is_some() || self.wasm_runtime.is_some()
    }

    fn load_zig_library() -> Option<Library> {
        let lib_paths = [
            "zig/nuclear_zig.lib",
            "zig/lib.lib",
            "libs/nuclear_zig.lib",
            "zig/zig-out/lib/nuclear_zig.lib",
        ];

        for lib_path in &lib_paths {
            match unsafe { Library::new(*lib_path) } {
                Ok(lib) => return Some(lib),
                Err(_) => continue,
            }
        }

        None
    }

    fn zig_hash(&self, lib: &Library, data: &[u8]) -> Result<ZigHashResult> {
        type ZigHashFn = unsafe extern "C" fn(
            data: *const u8,
            data_len: usize,
            config: *const ZigSimdConfigFFI,
            out_hash: *mut u8,
            out_hash_len: usize,
            out_time: *mut u64,
        ) -> i32;

        let func: Symbol<ZigHashFn> = unsafe { lib.get(b"zig_simd_hash")? };
        let mut hash_buffer = [0u8; 64];
        let mut processing_time: u64 = 0;
        let algo_cstr = CString::new(self.config.hash_algorithm.as_str())?;
        let config_ffi = ZigSimdConfigFFI {
            enable_simd: self.config.enable_simd,
            hash_algorithm: algo_cstr.as_ptr(),
            buffer_size: self.config.buffer_size,
            parallel_chunks: self.config.parallel_chunks,
        };

        let result_code = unsafe {
            func(
                data.as_ptr(),
                data.len(),
                &config_ffi as *const ZigSimdConfigFFI,
                hash_buffer.as_mut_ptr(),
                hash_buffer.len(),
                &mut processing_time as *mut u64,
            )
        };

        if result_code != 0 {
            return Err(anyhow::anyhow!("Zig SIMD hash failed with code: {}", result_code));
        }

        let hash_hex = hash_buffer
            .iter()
            .take_while(|&&b| b != 0)
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        Ok(ZigHashResult {
            hash: hash_hex,
            algorithm: self.config.hash_algorithm.clone(),
            input_size: data.len(),
            processing_time_ns: processing_time,
        })
    }

    fn hash_algorithm_from_name(name: &str) -> HashAlgorithm {
        match name {
            "sha256" => HashAlgorithm::Sha256,
            "xxhash" => HashAlgorithm::XxHash,
            _ => HashAlgorithm::Blake3,
        }
    }
}

impl Default for ZigSimdProcessor {
    fn default() -> Self {
        Self::new(ZigSimdConfig::default())
            .expect("ZigSimdProcessor requires native Zig FFI or ffi/wasm/zig/zig_simd.wasm")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zig_backend_contract_is_explicit() {
        match ZigSimdProcessor::new(ZigSimdConfig::default()) {
            Ok(processor) => assert!(processor.is_available()),
            Err(err) => assert!(err.to_string().contains("Zig backend unavailable")),
        }
    }

    #[test]
    fn test_hash_algorithm_mapping() {
        assert!(matches!(ZigSimdProcessor::hash_algorithm_from_name("blake3"), HashAlgorithm::Blake3));
        assert!(matches!(ZigSimdProcessor::hash_algorithm_from_name("sha256"), HashAlgorithm::Sha256));
        assert!(matches!(ZigSimdProcessor::hash_algorithm_from_name("xxhash"), HashAlgorithm::XxHash));
    }
}