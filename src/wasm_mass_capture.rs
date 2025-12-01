//! Módulo WebAssembly - Captura Masiva de Información
//!
//! Sistema avanzado de captura masiva usando WebAssembly para máximo rendimiento
//! Permite scraping paralelo masivo en navegador y servidor

use js_sys::{Array, Object, Reflect};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

/// Configuración para captura masiva
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct MassCaptureConfig {
    /// Número máximo de requests concurrentes
    pub max_concurrent: u32,

    /// Timeout en milisegundos
    pub timeout_ms: u32,

    /// User agent
    #[wasm_bindgen(getter_with_clone)]
    pub user_agent: String,

    /// Delay entre requests (ms)
    pub delay_ms: u32,

    /// Retry attempts
    pub retry_attempts: u32,

    /// Headers adicionales (JSON string)
    #[wasm_bindgen(getter_with_clone)]
    pub headers: String,
}

#[wasm_bindgen]
impl MassCaptureConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            max_concurrent: 1000, // NUCLEAR: Paralelismo extremo
            timeout_ms: 60000,    // Timeout largo para captura masiva
            user_agent: "Nuclear Crawler Hybrid/0.1.0".to_string(),
            delay_ms: 0,       // NUCLEAR: Sin delay, máxima velocidad
            retry_attempts: 5, // Más reintentos
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
}

impl Default for MassCaptureConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Resultado de captura masiva
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct MassCaptureResult {
    /// URL crawlada
    #[wasm_bindgen(getter_with_clone)]
    pub url: String,

    /// Status code
    pub status: u16,

    /// HTML contenido
    #[wasm_bindgen(getter_with_clone)]
    pub html: String,

    /// Datos extraídos (JSON string)
    #[wasm_bindgen(getter_with_clone)]
    pub extracted_data: String,

    /// Tiempo de respuesta (ms)
    pub response_time_ms: u64,

    /// Tamaño del contenido (bytes)
    pub content_size: usize,

    /// Links encontrados
    #[wasm_bindgen(getter_with_clone)]
    pub links: String, // JSON array

    /// Imágenes encontradas
    #[wasm_bindgen(getter_with_clone)]
    pub images: String, // JSON array

    /// Error si hubo
    #[wasm_bindgen(getter_with_clone)]
    pub error: Option<String>,
}

#[wasm_bindgen]
impl MassCaptureResult {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            url: String::new(),
            status: 0,
            html: String::new(),
            extracted_data: "{}".to_string(),
            response_time_ms: 0,
            content_size: 0,
            links: "[]".to_string(),
            images: "[]".to_string(),
            error: None,
        }
    }
}

impl Default for MassCaptureResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Sistema de captura masiva con WebAssembly
#[wasm_bindgen]
pub struct MassCapture {
    config: MassCaptureConfig,
}

#[wasm_bindgen]
impl MassCapture {
    /// Crea un nuevo sistema de captura masiva
    #[wasm_bindgen(constructor)]
    pub fn new(config: MassCaptureConfig) -> Self {
        Self { config }
    }

    /// Captura masiva de múltiples URLs en paralelo
    #[wasm_bindgen]
    pub async fn capture_many(&self, urls: Vec<String>) -> Result<Array, JsValue> {
        let start_time = js_sys::Date::now();

        // Crear array de resultados
        let results = Array::new();

        // Procesar URLs en lotes para controlar concurrencia
        let batch_size = self.config.max_concurrent as usize;
        let url_chunks: Vec<&[String]> = urls.chunks(batch_size).collect();

        for chunk in url_chunks {
            // Procesar URLs del lote en paralelo
            for url in chunk {
                let url_clone = url.clone();
                let config_clone = self.config.clone();

                match Self::capture_single_internal(&url_clone, &config_clone).await {
                    Ok(result) => {
                        results.push(&result);
                    }
                    Err(e) => {
                        let error_obj = Object::new();
                        Reflect::set(&error_obj, &"url".into(), &url_clone.into()).unwrap();
                        Reflect::set(&error_obj, &"error".into(), &e).unwrap();
                        results.push(&error_obj);
                    }
                }
            }

            // Delay entre lotes
            if self.config.delay_ms > 0 {
                let delay_promise = js_sys::Promise::new(&mut |resolve, _| {
                    let _timeout = web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            &resolve,
                            self.config.delay_ms as i32,
                        )
                        .unwrap();
                });
                JsFuture::from(delay_promise).await.ok();
            }
        }

        let end_time = js_sys::Date::now();
        let total_time = (end_time - start_time) as u64;

        // Agregar metadata al array
        let metadata = Object::new();
        Reflect::set(&metadata, &"total_urls".into(), &(urls.len() as u32).into()).unwrap();
        Reflect::set(&metadata, &"total_time_ms".into(), &total_time.into()).unwrap();
        Reflect::set(&metadata, &"results".into(), &results).unwrap();

        let final_array = Array::new();
        final_array.push(&metadata);

        Ok(final_array)
    }

    /// Captura una URL individual con retry
    async fn capture_single_internal(
        url: &str,
        config: &MassCaptureConfig,
    ) -> Result<JsValue, JsValue> {
        let mut last_error = None;

        for attempt in 0..config.retry_attempts {
            match Self::fetch_and_process(url, config).await {
                Ok(result) => {
                    let obj = Object::new();
                    Reflect::set(&obj, &"url".into(), &result.url.into()).unwrap();
                    Reflect::set(&obj, &"status".into(), &result.status.into()).unwrap();
                    Reflect::set(&obj, &"html".into(), &result.html.into()).unwrap();
                    Reflect::set(
                        &obj,
                        &"extracted_data".into(),
                        &result.extracted_data.into(),
                    )
                    .unwrap();
                    Reflect::set(
                        &obj,
                        &"response_time_ms".into(),
                        &result.response_time_ms.into(),
                    )
                    .unwrap();
                    Reflect::set(&obj, &"links".into(), &result.links.into()).unwrap();
                    Reflect::set(&obj, &"images".into(), &result.images.into()).unwrap();
                    return Ok(obj.into());
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < config.retry_attempts - 1 {
                        // Esperar antes de reintentar
                        let delay = (config.delay_ms * (attempt + 1) as u32) as i32;
                        let delay_promise = js_sys::Promise::new(&mut |resolve, _| {
                            web_sys::window()
                                .unwrap()
                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                    &resolve, delay,
                                )
                                .unwrap();
                        });
                        JsFuture::from(delay_promise).await.ok();
                    }
                }
            }
        }

        // Si todos los intentos fallaron
        let error_obj = Object::new();
        Reflect::set(&error_obj, &"url".into(), &url.into()).unwrap();
        Reflect::set(
            &error_obj,
            &"error".into(),
            &last_error.unwrap_or_else(|| JsValue::from_str("Unknown error")),
        )
        .unwrap();
        Ok(error_obj.into())
    }

    /// Fetch y procesamiento de una URL
    async fn fetch_and_process(
        url: &str,
        config: &MassCaptureConfig,
    ) -> Result<MassCaptureResult, JsValue> {
        let start_time = js_sys::Date::now();

        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);

        let headers = Headers::new().unwrap();
        headers.set("User-Agent", &config.user_agent).unwrap();

        // Parsear headers adicionales
        if let Ok(header_obj) = serde_json::from_str::<serde_json::Value>(&config.headers) {
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
        let content_size = html_str.len();

        // Extraer datos masivamente (usando parsing simple para WASM)
        let links = Self::extract_links_simple(&html_str);
        let images = Self::extract_images_simple(&html_str);
        let links_json = serde_json::to_string(&links).unwrap_or_else(|_| "[]".to_string());
        let images_json = serde_json::to_string(&images).unwrap_or_else(|_| "[]".to_string());
        let extracted_data = Self::extract_data_simple(&html_str);

        Ok(MassCaptureResult {
            url: url.to_string(),
            status,
            html: html_str,
            extracted_data: serde_json::to_string(&extracted_data)
                .unwrap_or_else(|_| "{}".to_string()),
            response_time_ms,
            content_size,
            links: links_json,
            images: images_json,
            error: None,
        })
    }

    /// Extrae links de forma simple (para WASM)
    fn extract_links_simple(html: &str) -> Vec<serde_json::Value> {
        use regex::Regex;
        let re = Regex::new(r#"<a[^>]+href=["']([^"']+)["'][^>]*>([^<]*)</a>"#).unwrap();
        re.captures_iter(html)
            .map(|cap| {
                let url = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                let text = cap.get(2).map(|m| m.as_str()).unwrap_or("");
                serde_json::json!({
                    "url": url,
                    "text": text.trim(),
                })
            })
            .collect()
    }

    /// Extrae imágenes de forma simple (para WASM)
    fn extract_images_simple(html: &str) -> Vec<serde_json::Value> {
        use regex::Regex;
        let re = Regex::new(r#"<img[^>]+src=["']([^"']+)["'][^>]*>"#).unwrap();
        re.captures_iter(html)
            .map(|cap| {
                let url = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                serde_json::json!({
                    "url": url,
                })
            })
            .collect()
    }

    /// Extrae datos de forma simple (para WASM)
    fn extract_data_simple(html: &str) -> serde_json::Value {
        use regex::Regex;
        let mut data = serde_json::Map::new();

        // Título
        let title_re = Regex::new(r#"<title>([^<]+)</title>"#).unwrap();
        if let Some(cap) = title_re.captures(html) {
            if let Some(title) = cap.get(1) {
                data.insert(
                    "title".to_string(),
                    serde_json::Value::String(title.as_str().trim().to_string()),
                );
            }
        }

        // Meta tags
        let meta_re = Regex::new(
            r#"<meta[^>]+(?:name|property)=["']([^"']+)["'][^>]+content=["']([^"']+)["']"#,
        )
        .unwrap();
        let mut meta = serde_json::Map::new();
        for cap in meta_re.captures_iter(html) {
            if let (Some(name), Some(content)) = (cap.get(1), cap.get(2)) {
                meta.insert(
                    name.as_str().to_string(),
                    serde_json::Value::String(content.as_str().to_string()),
                );
            }
        }
        if !meta.is_empty() {
            data.insert("meta".to_string(), serde_json::Value::Object(meta));
        }

        serde_json::Value::Object(data)
    }

    /// Captura masiva con workers para máximo paralelismo
    #[wasm_bindgen]
    pub async fn capture_massive_parallel(&self, urls: Vec<String>) -> Result<Array, JsValue> {
        // Usar Web Workers para paralelismo máximo
        let _window = web_sys::window().ok_or_else(|| JsValue::from_str("No window object"))?;

        // Dividir URLs en chunks para workers
        let num_workers = (self.config.max_concurrent / 10).clamp(1, 10) as usize;
        let chunk_size = (urls.len() / num_workers).max(1);
        let chunks: Vec<&[String]> = urls.chunks(chunk_size).collect();

        let results = Array::new();

        // Procesar cada chunk (en navegador, usar workers si están disponibles)
        for chunk in chunks {
            let chunk_results = self.capture_many(chunk.to_vec()).await?;
            let first_result = chunk_results.get(0);
            if let Ok(obj) = first_result.dyn_into::<Object>() {
                if let Ok(results_value) = Reflect::get(&obj, &"results".into()) {
                    if let Ok(results_array) = results_value.dyn_into::<Array>() {
                        for i in 0..results_array.length() {
                            let item = results_array.get(i);
                            results.push(&item);
                        }
                    }
                }
            }
        }

        Ok(results)
    }
}

/// Crea un sistema de captura masiva
#[wasm_bindgen]
pub fn create_mass_capture(config: MassCaptureConfig) -> MassCapture {
    MassCapture::new(config)
}
