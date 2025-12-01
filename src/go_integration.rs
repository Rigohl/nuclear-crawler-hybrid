//! Módulo Go Integration - Operaciones críticas en Go
//!
//! Integración con componentes Go para operaciones ultra-rápidas
//! usando FFI (Foreign Function Interface)

use anyhow::Result;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// FFI para Go (opcional - solo si Go está compilado)
#[cfg(feature = "go")]
#[link(name = "stealth_go", kind = "static")]
extern "C" {
    fn ExportStealthHeaders() -> *mut c_char;
    fn FastProcessURLs(urls_json: *const c_char) -> *mut c_char;
    fn FreeString(s: *mut c_char);
}

#[cfg(not(feature = "go"))]
#[allow(dead_code)]
extern "C" {
    fn ExportStealthHeaders() -> *mut c_char;
    fn FastProcessURLs(_: *const c_char) -> *mut c_char;
    fn FreeString(_: *mut c_char);
}

/// Integración con Go
pub struct GoIntegration {
    enabled: bool,
}

impl GoIntegration {
    /// Crea nueva integración Go
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Obtiene headers stealth desde Go
    pub fn get_stealth_headers(&self) -> Result<StealthHeadersGo> {
        if !self.enabled {
            return Ok(StealthHeadersGo::default());
        }

        #[cfg(feature = "go")]
        {
            unsafe {
                let c_str = ExportStealthHeaders();
                if c_str.is_null() {
                    return Ok(StealthHeadersGo::default());
                }

                let json_str = CStr::from_ptr(c_str).to_str()?;
                let headers: StealthHeadersGo = serde_json::from_str(json_str)?;

                FreeString(c_str);

                Ok(headers)
            }
        }

        #[cfg(not(feature = "go"))]
        {
            Ok(StealthHeadersGo::default())
        }
    }

    /// Procesa URLs rápido con Go goroutines
    pub fn fast_process_urls(&self, urls: Vec<String>) -> Result<Vec<String>> {
        if !self.enabled {
            return Ok(urls);
        }

        #[cfg(feature = "go")]
        {
            let urls_json = serde_json::to_string(&urls)?;
            let c_str = CString::new(urls_json)?;

            unsafe {
                let result_ptr = FastProcessURLs(c_str.as_ptr());
                if result_ptr.is_null() {
                    return Ok(urls);
                }

                let result_str = CStr::from_ptr(result_ptr).to_str()?;
                let results: Vec<String> = serde_json::from_str(result_str)?;

                FreeString(result_ptr);

                Ok(results)
            }
        }

        #[cfg(not(feature = "go"))]
        {
            Ok(urls)
        }
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
            user_agent: "Nuclear Crawler Hybrid/0.1.0".to_string(),
            headers: std::collections::HashMap::new(),
        }
    }
}
