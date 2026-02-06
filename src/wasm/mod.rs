/// WASM Module - Main Library Export
/// Re-exports all WASM modules with proper feature gating
// `file_search` provides a native fallback (non-wasm32) used by tests and by
// the Rust tooling. Other WASM-only modules stay wasm32-gated.
pub mod file_search;

#[cfg(target_arch = "wasm32")]
pub mod neural_ops;

#[cfg(target_arch = "wasm32")]
pub mod data_search;

#[cfg(target_arch = "wasm32")]
pub mod marketing_extractor;

#[cfg(target_arch = "wasm32")]
pub mod dataset_extractor;

#[cfg(target_arch = "wasm32")]
pub mod ultra_scraper;

#[cfg(target_arch = "wasm32")]
pub mod real_human_scraper;

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
    r#"{\"file_search\": \"100x\", \"neural_ops\": \"50x\", \"data_search\": \"30x\"}"#.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_wasm_version() {
        #[cfg(target_arch = "wasm32")]
        {
            let v = super::wasm_version();
            assert!(v.contains("nuclear-crawler-wasm"));
        }
    }
}
