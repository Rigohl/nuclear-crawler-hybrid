use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct NuclearCrawlerWasm {
    base_url: String,
}

#[wasm_bindgen]
impl NuclearCrawlerWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(base_url: &str) -> NuclearCrawlerWasm {
        console_error_panic_hook::set_once();
        console::log_1(&"🚀 Nuclear Crawler WASM initialized!".into());
        
        NuclearCrawlerWasm {
            base_url: base_url.to_string(),
        }
    }

    /// Extrae URLs de HTML
    #[wasm_bindgen]
    pub fn extract_links(&self, html: &str) -> Vec<JsValue> {
        let mut links = Vec::new();
        
        // Simple regex para extraer href
        let re = regex::Regex::new(r#"href=["']([^"']+)["']"#).unwrap();
        
        for cap in re.captures_iter(html) {
            if let Some(url) = cap.get(1) {
                links.push(JsValue::from_str(url.as_str()));
            }
        }
        
        links
    }

    /// Extrae texto de HTML (simple)
    #[wasm_bindgen]
    pub fn extract_text(&self, html: &str) -> String {
        // Simple extracción removiendo tags
        let re = regex::Regex::new(r"<[^>]+>").unwrap();
        re.replace_all(html, " ").to_string()
    }

    /// Normaliza URLs
    #[wasm_bindgen]
    pub fn normalize_url(&self, url: &str) -> String {
        if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else if url.starts_with("/") {
            format!("{}{}", self.base_url, url)
        } else {
            format!("{}/{}", self.base_url, url)
        }
    }

    /// Obtiene estadísticas del HTML
    #[wasm_bindgen]
    pub fn get_stats(&self, html: &str) -> JsValue {
        let link_count = regex::Regex::new(r#"href=["']"#).unwrap().find_iter(html).count();
        let img_count = regex::Regex::new(r#"<img"#).unwrap().find_iter(html).count();
        let script_count = regex::Regex::new(r#"<script"#).unwrap().find_iter(html).count();
        let style_count = regex::Regex::new(r#"<style"#).unwrap().find_iter(html).count();

        serde_json::json!({
            "total_size": html.len(),
            "links": link_count,
            "images": img_count,
            "scripts": script_count,
            "styles": style_count,
        })
    }

    #[wasm_bindgen]
    pub fn version(&self) -> String {
        "🔥 NUCLEAR CRAWLER WASM v0.1.0 - Maximum Power Edition".to_string()
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("🚀 ¡Hola desde Nuclear Crawler WASM, {}!", name)
}
