//! Módulo WebAssembly - Interfaz consolidada para ejecución en navegador
//!
//! Consolida wasm.rs y wasm_mass_capture.rs
//! Usa las librerías más poderosas de WebAssembly (wasm-bindgen, wasmtime, wasmer)

use js_sys::{Array, Object, Reflect};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

/// Configuración para el crawler WebAssembly
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct WasmCrawlerConfig {
    /// Número máximo de requests concurrentes
    #[wasm_bindgen(getter_with_clone)]
    pub max_concurrent: u32,

    /// Timeout en milisegundos
    pub timeout_ms: u32,

    /// User agent personalizado
    #[wasm_bindgen(getter_with_clone)]
    pub user_agent: String,

    /// Headers adicionales
    #[wasm_bindgen(getter_with_clone)]
    pub headers: String, // JSON string
}

#[wasm_bindgen]
impl WasmCrawlerConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            max_concurrent: 10,
            timeout_ms: 30000,
            user_agent: "Nuclear Crawler Hybrid/0.1.0".to_string(),
            headers: "{}".to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn max_concurrent(&self) -> u32 {
        self.max_concurrent
    }

    #[wasm_bindgen(setter)]
    pub fn set_max_concurrent(&mut self, value: u32) {
        self.max_concurrent = value;
    }

    #[wasm_bindgen(getter)]
    pub fn timeout_ms(&self) -> u32 {
        self.timeout_ms
    }

    #[wasm_bindgen(setter)]
    pub fn set_timeout_ms(&mut self, value: u32) {
        self.timeout_ms = value;
    }

    #[wasm_bindgen(getter)]
    pub fn user_agent(&self) -> String {
        self.user_agent.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_user_agent(&mut self, value: String) {
        self.user_agent = value;
    }
}

impl Default for WasmCrawlerConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Resultado de un crawl
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct CrawlResult {
    /// URL crawlada
    #[wasm_bindgen(getter_with_clone)]
    pub url: String,

    /// Status code HTTP
    pub status: u16,

    /// HTML contenido
    #[wasm_bindgen(getter_with_clone)]
    pub html: String,

    /// Datos extraídos (JSON string)
    #[wasm_bindgen(getter_with_clone)]
    pub extracted_data: String,

    /// Tiempo de respuesta en ms
    pub response_time_ms: u64,

    /// Errores si los hay
    #[wasm_bindgen(getter_with_clone)]
    pub error: Option<String>,
}

#[wasm_bindgen]
impl CrawlResult {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            url: String::new(),
            status: 0,
            html: String::new(),
            extracted_data: "{}".to_string(),
            response_time_ms: 0,
            error: None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn url(&self) -> String {
        self.url.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn status(&self) -> u16 {
        self.status
    }

    #[wasm_bindgen(getter)]
    pub fn html(&self) -> String {
        self.html.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn extracted_data(&self) -> String {
        self.extracted_data.clone()
    }
}

impl Default for CrawlResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Crawler WebAssembly principal
#[wasm_bindgen]
pub struct WasmCrawler {
    config: WasmCrawlerConfig,
}

#[wasm_bindgen]
impl WasmCrawler {
    /// Crea un nuevo crawler con configuración
    #[wasm_bindgen(constructor)]
    pub fn new(config: WasmCrawlerConfig) -> Self {
        Self { config }
    }

    /// Crawlea una URL y retorna el resultado
    #[wasm_bindgen]
    pub async fn crawl(&self, url: &str) -> Result<CrawlResult, JsValue> {
        let start_time = js_sys::Date::now();

        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);

        let headers = Headers::new().unwrap();
        headers.set("User-Agent", &self.config.user_agent).unwrap();

        // Parsear headers adicionales
        if let Ok(header_obj) = serde_json::from_str::<serde_json::Value>(&self.config.headers) {
            if let Some(obj) = header_obj.as_object() {
                for (key, value) in obj {
                    if let Some(val_str) = value.as_str() {
                        headers.set(key, val_str).unwrap_or_default();
                    }
                }
            }
        }

        opts.set_headers(&headers);

        let request = Request::new_with_str_and_init(url, &opts)
            .map_err(|e| JsValue::from_str(&format!("Error creando request: {:?}", e)))?;

        let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window object"))?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| JsValue::from_str(&format!("Error en fetch: {:?}", e)))?;

        let resp: Response = resp_value
            .dyn_into()
            .map_err(|e| JsValue::from_str(&format!("Error convirtiendo response: {:?}", e)))?;

        let status = resp.status();
        let html_promise = resp
            .text()
            .map_err(|e| JsValue::from_str(&format!("Error obteniendo texto: {:?}", e)))?;

        let html = JsFuture::from(html_promise)
            .await
            .map_err(|e| JsValue::from_str(&format!("Error leyendo texto: {:?}", e)))?;

        let html_str = html.as_string().unwrap_or_default();

        let end_time = js_sys::Date::now();
        let response_time_ms = (end_time - start_time) as u64;

        // Extraer datos usando el parser
        let extracted_data = self.extract_data(&html_str);

        Ok(CrawlResult {
            url: url.to_string(),
            status,
            html: html_str,
            extracted_data: serde_json::to_string(&extracted_data)
                .unwrap_or_else(|_| "{}".to_string()),
            response_time_ms,
            error: None,
        })
    }

    /// Crawlea múltiples URLs en paralelo
    #[wasm_bindgen]
    pub async fn crawl_many(&self, urls: Vec<String>) -> Result<Array, JsValue> {
        let futures: Vec<_> = urls.iter().map(|url| self.crawl(url)).collect();

        let results = futures::future::join_all(futures).await;

        let array = Array::new();
        for result in results {
            match result {
                Ok(crawl_result) => {
                    let obj = Object::new();
                    Reflect::set(&obj, &"url".into(), &crawl_result.url.into()).unwrap();
                    Reflect::set(&obj, &"status".into(), &crawl_result.status.into()).unwrap();
                    Reflect::set(&obj, &"html".into(), &crawl_result.html.into()).unwrap();
                    Reflect::set(
                        &obj,
                        &"extracted_data".into(),
                        &crawl_result.extracted_data.into(),
                    )
                    .unwrap();
                    array.push(&obj);
                }
                Err(e) => {
                    let obj = Object::new();
                    Reflect::set(&obj, &"error".into(), &e).unwrap();
                    array.push(&obj);
                }
            }
        }

        Ok(array)
    }

    /// Extrae datos estructurados del HTML
    fn extract_data(&self, html: &str) -> serde_json::Value {
        use crate::scraper::Scraper;

        let scraper = Scraper::new();
        let mut data = serde_json::Map::new();

        // Extraer título
        if let Some(title) = scraper.extract_title(html) {
            data.insert("title".to_string(), serde_json::Value::String(title));
        }

        // Extraer meta tags
        let meta_tags = scraper.extract_meta_tags(html);
        data.insert("meta".to_string(), serde_json::Value::Object(meta_tags));

        // Extraer links (simplificado)
        if let Ok(links) = scraper.extract_links(html, "http://example.com") {
            let links_json: Vec<serde_json::Value> = links
                .into_iter()
                .map(serde_json::Value::String)
                .collect();
            data.insert("links".to_string(), serde_json::Value::Array(links_json));
        }

        // Extraer imágenes (simplificado)
        if let Ok(images) = scraper.extract_images(html, "http://example.com") {
            let images_json: Vec<serde_json::Value> = images
                .into_iter()
                .map(|img| {
                    let mut img_obj = serde_json::Map::new();
                    img_obj.insert("url".to_string(), serde_json::Value::String(img.url));
                    if let Some(alt) = img.alt {
                        img_obj.insert("alt".to_string(), serde_json::Value::String(alt));
                    }
                    serde_json::Value::Object(img_obj)
                })
                .collect();
            data.insert("images".to_string(), serde_json::Value::Array(images_json));
        }

        serde_json::Value::Object(data)
    }
}

/// Inicializa el crawler WebAssembly
#[wasm_bindgen]
pub fn init_crawler(config: WasmCrawlerConfig) -> WasmCrawler {
    WasmCrawler::new(config)
}
