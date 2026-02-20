//! 🔥 WASM FFI BRIDGE - Unified WebAssembly Integration
//!
//! This module provides a unified interface for all WASM-based FFI integrations:
//! - Go WASM (goroutines for parallel processing) - Source: ffi/wasm/go/main.go
//! - Nim WASM (HTML parsing and content extraction) - Source: ffi/wasm/nim/main.nim
//! - Zig WASM (SIMD-accelerated operations) - Source: ffi/wasm/zig/main.zig
//!
//! Uses wasmtime runtime for maximum performance and security.
//!
//! Build all WASM modules: ffi/wasm/build_wasm.sh
//! Powers ALL 7 MCP tools through portable multi-language acceleration.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use wasmtime::*;

/// WASM Runtime Configuration
#[derive(Debug, Clone)]
pub struct WasmConfig {
    pub enable_simd: bool,
    pub enable_bulk_memory: bool,
    pub enable_multi_memory: bool,
    pub max_memory_pages: usize, // Each page is 64KB
    pub enable_fuel: bool,
    pub initial_fuel: u64,
}

impl Default for WasmConfig {
    fn default() -> Self {
        Self {
            enable_simd: true,
            enable_bulk_memory: true,
            enable_multi_memory: false,
            max_memory_pages: 16384, // 1GB max
            enable_fuel: true,
            initial_fuel: 1_000_000_000, // 1 billion instructions
        }
    }
}

/// Go WASM Runtime - Parallel processing with goroutines
pub struct GoWasmRuntime {
    _engine: Engine,
    module: Module,
    store: Store<()>,
}

impl GoWasmRuntime {
    /// Initialize Go WASM runtime
    pub fn new(config: WasmConfig) -> Result<Self> {
        let engine_config = create_engine_config(&config)?;
        let engine = Engine::new(&engine_config)?;

        // Load Go WASM module
        let wasm_path = PathBuf::from("ffi/wasm/go/go_parallel.wasm");

        let module = if wasm_path.exists() {
            Module::from_file(&engine, &wasm_path).context("Failed to load Go WASM module")?
        } else {
            // Module not available, return error with helpful message
            return Err(anyhow::anyhow!(
                "Go WASM module not found at {:?}. Build with: cd ffi/wasm/go && tinygo build -o go_parallel.wasm -target wasm main.go",
                wasm_path
            ));
        };

        let store = Store::new(&engine, ());
        // Note: fuel metering is optional - would require FuelConsumer configuration

        Ok(Self {
            _engine: engine,
            module,
            store,
        })
    }

    /// Parallel fetch multiple URLs using goroutines
    pub async fn parallel_fetch_urls(
        &mut self,
        urls: Vec<String>,
        timeout_ms: u32,
    ) -> Result<Vec<HttpResult>> {
        let instance = Instance::new(&mut self.store, &self.module, &[])?;

        // Prepare input JSON
        let urls_json = serde_json::to_string(&urls)?;

        // Get memory export
        let memory = instance
            .get_memory(&mut self.store, "memory")
            .context("No memory export in Go WASM module")?;

        // Write URLs to WASM memory
        let urls_ptr = self.write_string_to_memory(&memory, &urls_json)?;

        // Get exported function
        let parallel_fetch = instance
            .get_typed_func::<(i32, i32, i32), i32>(&mut self.store, "parallel_fetch_urls")
            .context("Function parallel_fetch_urls not exported")?;

        // Call function
        let result_ptr = parallel_fetch.call(
            &mut self.store,
            (urls_ptr, urls.len() as i32, timeout_ms as i32),
        )?;

        // Read result from memory
        let result_json = self.read_string_from_memory(&memory, result_ptr as usize)?;

        // Parse results
        let results: Vec<HttpResult> =
            serde_json::from_str(&result_json).context("Failed to parse Go WASM results")?;

        Ok(results)
    }

    /// Process data in parallel using worker pool
    pub async fn process_data_parallel(
        &mut self,
        data: serde_json::Value,
        workers: u32,
    ) -> Result<serde_json::Value> {
        let instance = Instance::new(&mut self.store, &self.module, &[])?;

        let data_json = serde_json::to_string(&data)?;
        let memory = instance
            .get_memory(&mut self.store, "memory")
            .context("No memory export")?;

        let data_ptr = self.write_string_to_memory(&memory, &data_json)?;

        let process_data = instance
            .get_typed_func::<(i32, i32, i32), i32>(&mut self.store, "process_data_parallel")?;

        let result_ptr = process_data.call(
            &mut self.store,
            (data_ptr, data_json.len() as i32, workers as i32),
        )?;

        let result_json = self.read_string_from_memory(&memory, result_ptr as usize)?;
        let result = serde_json::from_str(&result_json)?;

        Ok(result)
    }

    fn write_string_to_memory(&mut self, memory: &Memory, s: &str) -> Result<i32> {
        // Simplified - in production would manage memory properly
        let bytes = s.as_bytes();
        memory.write(&mut self.store, 1024, bytes)?; // Write at offset 1024
        Ok(1024)
    }

    fn read_string_from_memory(&self, memory: &Memory, ptr: usize) -> Result<String> {
        // Simplified - in production would read length first
        let mut buf = vec![0u8; 65536]; // 64KB buffer
        memory.read(&self.store, ptr, &mut buf)?;

        // Find null terminator
        let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        let s = String::from_utf8(buf[..len].to_vec())?;
        Ok(s)
    }
}

/// Nim WASM Runtime - HTML parsing and content extraction
pub struct NimWasmParser {
    _engine: Engine,
    module: Module,
    store: Store<()>,
}

impl NimWasmParser {
    pub fn new(config: WasmConfig) -> Result<Self> {
        let engine_config = create_engine_config(&config)?;
        let engine = Engine::new(&engine_config)?;

        let wasm_path = PathBuf::from("ffi/wasm/nim/nim_parser.wasm");

        let module = if wasm_path.exists() {
            Module::from_file(&engine, &wasm_path)?
        } else {
            return Err(anyhow::anyhow!(
                "Nim WASM module not found. Build with: cd ffi/wasm/nim && nim c -d:release -d:emscripten --os:linux --cpu:wasm32 --gc:arc -o:nim_parser.wasm main.nim"
            ));
        };

        let store = Store::new(&engine, ());

        Ok(Self {
            _engine: engine,
            module,
            store,
        })
    }

    /// Parse HTML and extract elements matching CSS selector
    pub async fn parse_html(&mut self, html: &str, selector: &str) -> Result<Vec<Element>> {
        let instance = Instance::new(&mut self.store, &self.module, &[])?;
        let memory = instance
            .get_memory(&mut self.store, "memory")
            .context("No memory export")?;

        // Write HTML and selector to memory
        let html_ptr = self.write_to_memory(&memory, html.as_bytes())?;
        let selector_ptr = self.write_to_memory(&memory, selector.as_bytes())?;

        // Call parse_html function
        let parse_html =
            instance.get_typed_func::<(i32, i32, i32, i32), i32>(&mut self.store, "parse_html")?;

        let result_ptr = parse_html.call(
            &mut self.store,
            (
                html_ptr,
                html.len() as i32,
                selector_ptr,
                selector.len() as i32,
            ),
        )?;

        // Read and parse result
        let result_json = self.read_from_memory(&memory, result_ptr as usize)?;
        let elements = serde_json::from_str(&result_json)?;

        Ok(elements)
    }

    /// Extract all links from HTML
    pub async fn extract_links(&mut self, html: &str) -> Result<Vec<Link>> {
        let instance = Instance::new(&mut self.store, &self.module, &[])?;
        let memory = instance
            .get_memory(&mut self.store, "memory")
            .ok_or_else(|| anyhow::anyhow!("No memory export"))?;

        let html_ptr = self.write_to_memory(&memory, html.as_bytes())?;

        let extract_links =
            instance.get_typed_func::<(i32, i32), i32>(&mut self.store, "extract_links")?;

        let result_ptr = extract_links.call(&mut self.store, (html_ptr, html.len() as i32))?;

        let result_json = self.read_from_memory(&memory, result_ptr as usize)?;
        let links = serde_json::from_str(&result_json)?;

        Ok(links)
    }

    fn write_to_memory(&mut self, memory: &Memory, data: &[u8]) -> Result<i32> {
        memory.write(&mut self.store, 2048, data)?;
        Ok(2048)
    }

    fn read_from_memory(&self, memory: &Memory, ptr: usize) -> Result<String> {
        let mut buf = vec![0u8; 131072]; // 128KB
        memory.read(&self.store, ptr, &mut buf)?;
        let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        Ok(String::from_utf8(buf[..len].to_vec())?)
    }
}

/// Zig WASM Runtime - SIMD-accelerated operations
pub struct ZigWasmSimd {
    _engine: Engine,
    module: Module,
    store: Store<()>,
}

impl ZigWasmSimd {
    pub fn new(config: WasmConfig) -> Result<Self> {
        let engine_config = create_engine_config(&config)?;
        let engine = Engine::new(&engine_config)?;

        let wasm_path = PathBuf::from("ffi/wasm/zig/zig_simd.wasm");

        let module = if wasm_path.exists() {
            Module::from_file(&engine, &wasm_path)?
        } else {
            return Err(anyhow::anyhow!(
                "Zig WASM module not found. Build with: cd ffi/wasm/zig && zig build-lib -target wasm32-wasi -O ReleaseFast -dynamic main.zig"
            ));
        };

        let store = Store::new(&engine, ());

        Ok(Self {
            _engine: engine,
            module,
            store,
        })
    }

    /// Hash data using SIMD-accelerated algorithms
    pub async fn hash_data(&mut self, data: &[u8], algorithm: HashAlgorithm) -> Result<[u8; 32]> {
        let instance = Instance::new(&mut self.store, &self.module, &[])?;
        let memory = instance
            .get_memory(&mut self.store, "memory")
            .ok_or_else(|| anyhow::anyhow!("No memory export"))?;

        // Write data to memory
        let data_ptr = 4096;
        memory.write(&mut self.store, data_ptr, data)?;

        // Call hash function
        let hash_data =
            instance.get_typed_func::<(i32, i32, i32), i32>(&mut self.store, "hash_data_simd")?;

        let hash_ptr = hash_data.call(
            &mut self.store,
            (data_ptr as i32, data.len() as i32, algorithm as i32),
        )?;

        // Read hash result
        let mut hash = [0u8; 32];
        memory.read(&self.store, hash_ptr as usize, &mut hash)?;

        Ok(hash)
    }

    /// SIMD pattern matching
    pub async fn pattern_match(&mut self, haystack: &[u8], needle: &[u8]) -> Result<Vec<u32>> {
        let instance = Instance::new(&mut self.store, &self.module, &[])?;
        let memory = instance
            .get_memory(&mut self.store, "memory")
            .ok_or_else(|| anyhow::anyhow!("No memory export"))?;

        let haystack_ptr = 8192;
        let needle_ptr = 8192 + haystack.len();
        let matches_ptr = needle_ptr + needle.len() + 1024;

        memory.write(&mut self.store, haystack_ptr, haystack)?;
        memory.write(&mut self.store, needle_ptr, needle)?;

        let pattern_match = instance.get_typed_func::<(i32, i32, i32, i32, i32, i32), i32>(
            &mut self.store,
            "pattern_match_simd",
        )?;

        let match_count = pattern_match.call(
            &mut self.store,
            (
                haystack_ptr as i32,
                haystack.len() as i32,
                needle_ptr as i32,
                needle.len() as i32,
                matches_ptr as i32,
                10000, // Max 10K matches
            ),
        )? as usize;

        // Securely clamp match_count to prevent unbounded memory allocation
        let match_count = match_count.min(10000);

        // Read match positions
        let mut matches = vec![0u32; match_count];
        let matches_bytes = unsafe {
            std::slice::from_raw_parts_mut(matches.as_mut_ptr() as *mut u8, match_count * 4)
        };
        memory.read(&self.store, matches_ptr, matches_bytes)?;

        Ok(matches)
    }
}

// Helper functions

fn create_engine_config(config: &WasmConfig) -> Result<Config> {
    let mut engine_config = Config::new();
    engine_config.wasm_simd(config.enable_simd);
    engine_config.wasm_bulk_memory(config.enable_bulk_memory);
    engine_config.wasm_multi_memory(config.enable_multi_memory);

    Ok(engine_config)
}

// Data structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResult {
    pub url: String,
    pub status_code: u16,
    pub content_length: usize,
    pub response_time_ms: u64,
    pub headers: std::collections::HashMap<String, String>,
    pub content: String,
    pub error: Option<String>,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    pub tag: String,
    pub text: String,
    pub attributes: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub url: String,
    pub text: String,
    pub title: String,
    pub rel: String,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum HashAlgorithm {
    Blake3 = 0,
    Sha256 = 1,
    XxHash = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_config_default() {
        let config = WasmConfig::default();
        assert!(config.enable_simd);
        assert!(config.enable_bulk_memory);
    }

    #[test]
    fn test_wasm_engine_creation() {
        let config = WasmConfig::default();
        let engine_config = create_engine_config(&config);
        assert!(engine_config.is_ok());
    }

    #[test]
    fn test_wasm_source_files_exist() {
        // Verify real WASM source code files exist and are non-empty
        let sources = [
            ("ffi/wasm/go/main.go", "Go"),
            ("ffi/wasm/zig/main.zig", "Zig"),
            ("ffi/wasm/nim/main.nim", "Nim"),
            ("ffi/wasm/build_wasm.sh", "Build script"),
        ];

        for (path, name) in &sources {
            let p = std::path::Path::new(path);
            assert!(p.exists(), "{} WASM source not found at {}", name, path);
            let metadata = std::fs::metadata(p).unwrap();
            assert!(
                metadata.len() > 100,
                "{} WASM source at {} is too small ({}B)",
                name,
                path,
                metadata.len()
            );
        }
    }

    #[test]
    fn test_wasm_source_contains_exports() {
        // Verify the WASM source files contain the expected exported functions
        let go_src = std::fs::read_to_string("ffi/wasm/go/main.go").unwrap();
        assert!(
            go_src.contains("//export parallel_fetch_urls"),
            "Go missing parallel_fetch_urls export"
        );
        assert!(
            go_src.contains("//export process_data_parallel"),
            "Go missing process_data_parallel export"
        );
        assert!(
            go_src.contains("//export hash_batch"),
            "Go missing hash_batch export"
        );
        assert!(go_src.contains("//export alloc"), "Go missing alloc export");

        let zig_src = std::fs::read_to_string("ffi/wasm/zig/main.zig").unwrap();
        assert!(
            zig_src.contains("export fn hash_data_simd"),
            "Zig missing hash_data_simd export"
        );
        assert!(
            zig_src.contains("export fn pattern_match_simd"),
            "Zig missing pattern_match_simd export"
        );
        assert!(
            zig_src.contains("export fn batch_transform"),
            "Zig missing batch_transform export"
        );
        assert!(
            zig_src.contains("export fn alloc"),
            "Zig missing alloc export"
        );

        let nim_src = std::fs::read_to_string("ffi/wasm/nim/main.nim").unwrap();
        assert!(
            nim_src.contains("exportc: \"parse_html\""),
            "Nim missing parse_html export"
        );
        assert!(
            nim_src.contains("exportc: \"extract_links\""),
            "Nim missing extract_links export"
        );
        assert!(
            nim_src.contains("exportc: \"extract_metadata\""),
            "Nim missing extract_metadata export"
        );
    }

    #[test]
    fn test_hash_algorithm_values() {
        assert_eq!(HashAlgorithm::Blake3 as u8, 0);
        assert_eq!(HashAlgorithm::Sha256 as u8, 1);
        assert_eq!(HashAlgorithm::XxHash as u8, 2);
    }
}
