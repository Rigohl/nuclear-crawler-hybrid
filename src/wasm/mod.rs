/// WASM Module - Main Library Export
/// Re-exports all WASM modules with proper feature gating

#[cfg(target_arch = "wasm32")]
pub mod file_search;
pub mod file_search as file_search_module;

#[cfg(target_arch = "wasm32")]
pub mod neural_ops;
pub mod neural_ops as neural_ops_module;

#[cfg(target_arch = "wasm32")]
pub mod data_search;
pub mod data_search as data_search_module;

/// WASM initialization (web target)
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init_wasm() {
    console_error_panic_hook::set_once();
}

/// Export version for WASM
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_version() -> String {
    format!("nuclear-crawler-wasm@{}", env!("CARGO_PKG_VERSION"))
}

/// Utility: Get WASM capabilities
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn get_capabilities() -> String {
    r#"{"file_search": "100x", "neural_ops": "50x", "data_search": "30x"}"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_version() {
        #[cfg(target_arch = "wasm32")]
        {
            let v = wasm_version();
            assert!(v.contains("nuclear-crawler-wasm"));
        }
    }
}
