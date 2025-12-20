//! Módulo Go Integration - FFI Real a Go
//!
//! ✅ IMPLEMENTACIÓN REAL CON FFI:
//! - Usa biblioteca Go compilada (stealth_go.a) cuando feature "go" está activa
//! - Fallback a Rust nativo cuando Go no está disponible
//! - Headers stealth con rotación de User-Agents (desde Go)
//! - Procesamiento paralelo de URLs con goroutines reales

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

// FFI a Go - se enlaza desde build.rs si feature "go" está activada
#[cfg(feature = "go")]
#[link(name = "stealth_go", kind = "static")]
extern "C" {
    /// Obtiene headers stealth desde Go
    fn ExportStealthHeaders() -> *mut std::os::raw::c_char;
    
    /// Procesa URLs en paralelo con goroutines reales
    fn FastProcessURLs(urls_json: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    
    /// Libera memoria asignada por Go
    fn FreeString(s: *mut std::os::raw::c_char);
}

// User-Agents rotativos (fallback cuando Go no está disponible)
static USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_1) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Edge/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (iPad; CPU OS 17_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
];

static HEADER_INDEX: AtomicUsize = AtomicUsize::new(0);

/// Integración Go con FFI real o fallback Rust
pub struct GoIntegration {
    enabled: bool,
    use_ffi: bool,
}

impl GoIntegration {
    /// Crea nueva integración (auto-detecta si FFI está disponible)
    pub fn new() -> Self {
        Self { 
            enabled: true,
            use_ffi: cfg!(feature = "go"),
        }
    }
    
    /// Crea con configuración manual
    pub fn new_with_config(enabled: bool) -> Self {
        Self { 
            enabled,
            use_ffi: cfg!(feature = "go"),
        }
    }
    
    /// Verifica si Go FFI está disponible
    pub fn is_available(&self) -> bool {
        self.enabled && (cfg!(feature = "go") || true) // Siempre disponible con fallback
    }
    
    /// Verifica si está usando FFI real de Go
    pub fn is_using_ffi(&self) -> bool {
        self.use_ffi && cfg!(feature = "go")
    }

    /// Obtiene headers stealth (usa FFI de Go si está disponible)
    pub fn get_stealth_headers(&self) -> Result<StealthHeadersGo> {
        if !self.enabled {
            return Ok(StealthHeadersGo::default());
        }

        #[cfg(feature = "go")]
        if self.use_ffi {
            // ✅ USAR FFI REAL DE GO
            return self.get_stealth_headers_from_go();
        }

        // Fallback: implementación Rust
        self.get_stealth_headers_rust()
    }

    #[cfg(feature = "go")]
    fn get_stealth_headers_from_go(&self) -> Result<StealthHeadersGo> {
        unsafe {
            let json_ptr = ExportStealthHeaders();
            if json_ptr.is_null() {
                return Ok(StealthHeadersGo::default());
            }
            
            let c_str = std::ffi::CStr::from_ptr(json_ptr);
            let json_str = c_str.to_str().unwrap_or("{}");
            let result: StealthHeadersGo = serde_json::from_str(json_str).unwrap_or_default();
            
            FreeString(json_ptr);
            Ok(result)
        }
    }

    fn get_stealth_headers_rust(&self) -> Result<StealthHeadersGo> {
        // Rotar User-Agent de forma thread-safe
        let idx = HEADER_INDEX.fetch_add(1, Ordering::Relaxed) % USER_AGENTS.len();
        let user_agent = USER_AGENTS[idx].to_string();

        let mut headers = std::collections::HashMap::new();
        headers.insert("Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".to_string());
        headers.insert("Accept-Language".to_string(), "en-US,en;q=0.9,es;q=0.8".to_string());
        headers.insert("Accept-Encoding".to_string(), "gzip, deflate, br".to_string());
        headers.insert("Connection".to_string(), "keep-alive".to_string());
        headers.insert("Upgrade-Insecure-Requests".to_string(), "1".to_string());
        headers.insert("Sec-Fetch-Dest".to_string(), "document".to_string());
        headers.insert("Sec-Fetch-Mode".to_string(), "navigate".to_string());
        headers.insert("Sec-Fetch-Site".to_string(), "none".to_string());
        headers.insert("Sec-Fetch-User".to_string(), "?1".to_string());
        headers.insert("Cache-Control".to_string(), "max-age=0".to_string());
        headers.insert("DNT".to_string(), "1".to_string());

        Ok(StealthHeadersGo { user_agent, headers })
    }

    /// Procesa URLs en paralelo (usa goroutines reales de Go si está disponible)
    pub fn fast_process_urls(&self, urls: Vec<String>) -> Result<Vec<String>> {
        if !self.enabled {
            return Ok(urls);
        }

        #[cfg(feature = "go")]
        if self.use_ffi {
            // ✅ USAR FFI REAL DE GO CON GOROUTINES
            return self.fast_process_urls_go(urls);
        }

        // Fallback: implementación Rust con rayon
        self.fast_process_urls_rust(urls)
    }

    #[cfg(feature = "go")]
    fn fast_process_urls_go(&self, urls: Vec<String>) -> Result<Vec<String>> {
        let urls_json = serde_json::to_string(&urls)?;
        let urls_cstr = std::ffi::CString::new(urls_json)?;
        
        unsafe {
            let result_ptr = FastProcessURLs(urls_cstr.as_ptr());
            if result_ptr.is_null() {
                return Ok(Vec::new());
            }
            
            let c_str = std::ffi::CStr::from_ptr(result_ptr);
            let json_str = c_str.to_str().unwrap_or("[]");
            let results: Vec<String> = serde_json::from_str(json_str).unwrap_or_default();
            
            FreeString(result_ptr);
            Ok(results)
        }
    }

    fn fast_process_urls_rust(&self, urls: Vec<String>) -> Result<Vec<String>> {
        use rayon::prelude::*;
        use url::Url;
        use std::collections::HashSet;

        // Procesar en paralelo con rayon
        let mut results: Vec<String> = urls
            .par_iter()
            .filter_map(|url_str| {
                // Validar que sea una URL HTTP/HTTPS válida
                if let Ok(parsed) = Url::parse(url_str) {
                    if parsed.scheme() == "http" || parsed.scheme() == "https" {
                        // Normalizar URL (quitar fragmentos, ordenar query params)
                        let mut normalized = parsed.clone();
                        normalized.set_fragment(None);
                        return Some(normalized.to_string());
                    }
                }
                None
            })
            .collect();

        // Deduplicar manteniendo orden
        let mut seen = HashSet::new();
        results.retain(|url| seen.insert(url.clone()));

        Ok(results)
    }
}

impl Default for GoIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthHeadersGo {
    pub user_agent: String,
    pub headers: std::collections::HashMap<String, String>,
}

impl Default for StealthHeadersGo {
    fn default() -> Self {
        Self {
            user_agent: USER_AGENTS[0].to_string(),
            headers: std::collections::HashMap::new(),
        }
    }
}
