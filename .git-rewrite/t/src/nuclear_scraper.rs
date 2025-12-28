//! Módulo Nuclear Scraper - Scraping extremo sin límites
//!
//! Sistema de scraping masivo y agresivo para captura masiva de información
//! Sin limitaciones, máximo paralelismo, máxima velocidad

<<<<<<< HEAD:src/nuclear_scraper.rs
use crate::ai_smart::{AIConfig, AISmart};
=======
#[allow(unused_imports)]
use crate::ai_smart::{AIConfig, AISmart, TrainingData};
use crate::cache::Cache;
use crate::content_extractor::{ContentExtractor, ExtractedContent}; // 🔥 EXTRACCIÓN REAL
>>>>>>> origin/main:.git-rewrite/t/src/nuclear_scraper.rs
use crate::rate_limit::RateLimiter;
use crate::stealth::{StealthConfig, StealthSystem};
use anyhow::Context;
use anyhow::Result;
use dashmap::DashMap;
use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::fs;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

/// Configuración NUCLEAR - Sin límites
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearConfig {
    /// Paralelismo extremo
    pub max_concurrent: usize,

    /// Requests por segundo (sin límite práctico)
    pub max_requests_per_second: u32,

    /// Timeout largo
    pub timeout_seconds: u64,

    /// User agent
    pub user_agent: String,

    /// Seguir TODOS los links encontrados
    pub follow_all_links: bool,

    /// Profundidad ilimitada
    pub unlimited_depth: bool,

    /// Caché masivo
    pub cache_size: usize,

    /// Headers personalizados
    pub extra_headers: Option<std::collections::HashMap<String, String>>,
}

impl Default for NuclearConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 1000,           // NUCLEAR: Máximo poder - 1000 concurrent
            max_requests_per_second: 10000, // NUCLEAR: 10K requests/segundo
            timeout_seconds: 120,           // NUCLEAR: Timeout largo para captura masiva
            user_agent: "Nuclear Scraper Web/0.1.0".to_string(),
            follow_all_links: true,
            unlimited_depth: true, // NUCLEAR: Profundidad ilimitada
            cache_size: 1_000_000, // NUCLEAR: Caché masivo (1M entradas)
            extra_headers: None,
        }
    }
}

/// Resultado de scraping nuclear
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearResult {
    pub url: String,
    pub status_code: u16,
    pub html: String,
    pub content_length: usize,
    pub response_time: Duration,
    pub links_found: Vec<String>,
    pub images_found: Vec<String>,
    pub extracted_data: serde_json::Value,
    pub crawled_at: chrono::DateTime<chrono::Utc>,
    pub error: Option<String>,
}

/// Estadísticas de scraping nuclear
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearStats {
    pub total_urls_crawled: usize,
    pub successful: usize,
    pub failed: usize,
    pub total_data_captured: usize, // bytes
    pub total_links_found: usize,
    pub total_images_found: usize,
    pub total_time: Duration,
    pub urls_per_second: f64,
    pub avg_response_time: Duration,
}

/// Scraper NUCLEAR - Sin límites con Stealth e Inteligencia
pub struct NuclearScraper {
    client: Arc<Client>,
    config: NuclearConfig,
    rate_limiter: Arc<RateLimiter>,
    semaphore: Arc<Semaphore>,
    visited: Arc<DashMap<String, bool>>,
    results: Arc<DashMap<String, NuclearResult>>,
    stats: Arc<DashMap<String, NuclearStats>>,
    stealth: Arc<StealthSystem>,
    ai_smart: Arc<AISmart>,
    storage: Option<Arc<crate::intelligent_storage::IntelligentStorage>>,
}

impl NuclearScraper {
    /// Crea un nuevo scraper NUCLEAR
    pub fn new(config: NuclearConfig) -> Result<Self> {
        Self::new_with_storage(config, None)
    }

    /// Crea un nuevo scraper NUCLEAR con almacenamiento
    pub fn new_with_storage(
        config: NuclearConfig,
        storage: Option<Arc<crate::intelligent_storage::IntelligentStorage>>,
    ) -> Result<Self> {
        let client = Arc::new(
            Client::builder()
                .timeout(Duration::from_secs(config.timeout_seconds))
                .user_agent(&config.user_agent)
                .cookie_store(true)
                .pool_max_idle_per_host(10) // REDUCIDO: De 100 a 10
                .pool_idle_timeout(Duration::from_secs(30)) // REDUCIDO: De 90 a 30
                .tcp_keepalive(Duration::from_secs(30)) // REDUCIDO: De 60 a 30
                .tcp_nodelay(true)
                .build()
                .context("Error creando cliente HTTP")?,
        );

        let rate_limiter = Arc::new(RateLimiter::new(
            config.max_requests_per_second,
            config.max_requests_per_second * 2, // Burst doble
        ));

        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));

        // Sistema de stealth
        let stealth_config = StealthConfig {
            rotate_user_agents: true,
            rotate_headers: true,
            random_delay_min: 500,
            random_delay_max: 3000,
            human_behavior: true,
            use_proxies: false,
            proxies: Vec::new(),
            avoid_headless_detection: true,
            tls_fingerprint_evasion: true,
        };
        let stealth = Arc::new(StealthSystem::new(stealth_config));

        // Sistema de inteligencia - integrado inline
        // let intelligence = Arc::new(ContentIntelligence::new(Vec::new()));

        // Acelerador JAX
        // Usar rayon directamente para paralelismo

        // AI Smart
        let ai_config = AIConfig::default();
        let ai_smart = Arc::new(AISmart::new(ai_config));

        Ok(Self {
            client,
            config,
            rate_limiter,
            semaphore,
            visited: Arc::new(DashMap::new()),
            results: Arc::new(DashMap::new()),
            stats: Arc::new(DashMap::new()),
            stealth,
            // intelligence, // Eliminado - funcionalidad inline
            ai_smart,
            storage,
        })
    }

    /// Scraping NUCLEAR masivo - Sin límites
    pub async fn nuclear_crawl(&self, urls: Vec<String>) -> Result<Vec<NuclearResult>> {
        let start_time = Instant::now();
        let total_urls = urls.len();

        // Procesar concurrentemente con tokio
        let mut tasks: Vec<tokio::task::JoinHandle<Result<NuclearResult>>> = Vec::new();

        for url in urls {
            let url_clone = url.clone();
            let client = self.client.clone();
            // let scraper = self.scraper.clone(); // Eliminado
            // Parser integrado en scraper
            let semaphore = self.semaphore.clone();
            let rate_limiter = self.rate_limiter.clone();
            let visited = self.visited.clone();
            let results_map = self.results.clone();
            let stealth_system = self.stealth.clone();
            // let intelligence = self.intelligence.clone(); // Eliminado
            let ai_smart = self.ai_smart.clone();
            let storage_opt = self.storage.clone();

            let task = tokio::spawn(async move {
                // Verificar si ya fue visitada
                if visited.contains_key(&url_clone) {
                    return Ok(results_map
                        .get(&url_clone)
                        .map(|r| r.clone())
                        .unwrap_or_else(|| NuclearResult {
                            url: url_clone.clone(),
                            status_code: 0,
                            html: String::new(),
                            content_length: 0,
                            response_time: Duration::ZERO,
                            links_found: Vec::new(),
                            images_found: Vec::new(),
                            extracted_data: serde_json::json!({}),
                            crawled_at: chrono::Utc::now(),
                            error: Some("Already visited".to_string()),
                        }));
                }

                // Adquirir permiso
                let _permit = semaphore.acquire().await?;

                // Rate limiting
                rate_limiter.wait().await;

                // Stealth: Delay humano
                let delay = stealth_system.get_human_delay();
                if delay > 0 {
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }

                // Stealth: Verificar si necesita pausa
                let domain = url::Url::parse(&url_clone)
                    .ok()
                    .and_then(|u| u.host_str().map(|s| s.to_string()))
                    .unwrap_or_default();

                stealth_system.increment_request_count(&domain);

                // AI Smart: Detectar riesgo de ban
                let recent_results = vec!["success"]; // Simplificado
                let ban_risk = ai_smart.detect_ban_risk(&domain, &recent_results);
                let anti_ban = ai_smart.recommend_anti_ban_action(&domain, ban_risk);

                if anti_ban.action != "continue" {
                    tokio::time::sleep(Duration::from_millis(anti_ban.duration_ms)).await;
                } else if stealth_system.should_pause(&domain) {
                    tokio::time::sleep(Duration::from_secs(30)).await;
                }

                // Stealth: Headers y User Agent
                let user_agent = stealth_system.get_user_agent();
                let headers = stealth_system.get_headers(None);
                let anti_detection = stealth_system.get_anti_detection_headers();

                let start = Instant::now();

                // Fetch or Read File
                let (html, status_code, fetch_error) =
                    if url_clone.starts_with("http://") || url_clone.starts_with("https://") {
                        // HTTP fetch
                        let mut request = client.get(&url_clone);
                        request = request.header("User-Agent", &user_agent);

                        for (k, v) in headers {
                            request = request.header(&k, &v);
                        }

                        for (k, v) in anti_detection {
                            request = request.header(&k, &v);
                        }

                        // Agregar delay aleatorio entre requests
                        let delay = stealth_system.get_human_delay();
                        if delay > 0 {
                            tokio::time::sleep(Duration::from_millis(delay)).await;
                        }

                        match request.send().await {
                            Ok(response) => {
                                let status = response.status().as_u16();

                                // Verificar si es un error de rate limit o ban
                                if status == 429 || status == 403 || status == 503 {
                                    // Marcar como error de rate limit
                                    let error_msg =
                                        format!("Rate limit or ban detected (status: {})", status);
                                    (String::new(), status, Some(error_msg))
                                } else {
                                    match response.text().await {
                                        Ok(html) => (html, status, None),
                                        Err(e) => (
                                            String::new(),
                                            status,
                                            Some(format!("Error reading response: {}", e)),
                                        ),
                                    }
                                }
                            }
                            Err(e) => {
                                let error_msg = format!("Request error: {}", e);
                                // Devolver error para cualquier tipo
                                (String::new(), 0, Some(error_msg))
                            }
                        }
                    } else {
                        // File read
                        match std::fs::read_to_string(&url_clone) {
                            Ok(content) => (content, 200, None),
                            Err(e) => (String::new(), 0, Some(format!("File read error: {}", e))),
                        }
                    };

                if fetch_error.is_none() {
                    let content_length = html.len();

                    // 🔥 EXTRACCIÓN REAL de contenido usando ContentExtractor
                    let extractor = ContentExtractor::new();
                    let extracted: ExtractedContent = extractor.extract_all(&html, &url_clone)
                        .unwrap_or_else(|_| ExtractedContent::default_for_url(&url_clone));
                    
                    // Extraer links y imágenes del contenido extraído
                    let links_found: Vec<String> = extracted.links.iter().map(|l| l.url.clone()).collect();
                    let images_found: Vec<String> = extracted.images.iter().map(|i| i.url.clone()).collect();

                    let result = NuclearResult {
                        url: url_clone.clone(),
                        status_code,
                        html: html.clone(),
                        content_length,
                        response_time: start.elapsed(),
                        links_found, // 🔥 Links REALES extraídos
                        images_found, // 🔥 Imágenes REALES extraídas
                        extracted_data: serde_json::json!({
                            // 🔥 DATOS COMPLETOS EXTRAÍDOS
                            "title": extracted.title,
                            "description": extracted.description,
                            "main_text": extracted.main_text,
                            "summary": extracted.summary,
                            "word_count": extracted.word_count,
                            "language": extracted.language,
                            "headings": extracted.headings,
                            "code_snippets": extracted.code_snippets.iter().map(|c| {
                                serde_json::json!({
                                    "language": c.language,
                                    "code": c.code
                                })
                            }).collect::<Vec<_>>(),
                            "tables_count": extracted.tables.len(),
                            "links_count": extracted.links.len(),
                            "images_count": extracted.images.len(),
                            "content_length": content_length,
                            "status_code": status_code,
                        }),
                        crawled_at: chrono::Utc::now(),
                        error: None,
                    };

                    // Marcar como visitada
                    visited.insert(url_clone.clone(), true);
                    results_map.insert(url_clone.clone(), result.clone());

                    // Guardar URL en historial si hay storage (sin bloquear)
                    if let Some(storage) = storage_opt {
                        let url_for_history = url_clone.clone();
                        // Spawn task para no bloquear el flujo principal
                        let _handle = tokio::spawn(async move {
                            if let Err(e) = storage.append_urls_to_history(&url_for_history).await {
                                eprintln!("Warning: Error guardando URL en historial: {}", e);
                            }
                        });
                    }

                    Ok(result)
                } else {
                    Ok(NuclearResult {
                        url: url_clone,
                        status_code,
                        html: String::new(),
                        content_length: 0,
                        response_time: start.elapsed(),
                        links_found: Vec::new(),
                        images_found: Vec::new(),
                        extracted_data: serde_json::json!({}),
                        crawled_at: chrono::Utc::now(),
                        error: fetch_error,
                    })
                }
            });

            tasks.push(task);
        }

        // Esperar a que todas las tareas terminen
        let results = join_all(tasks).await;

        // Convertir resultados
        let mut final_results = Vec::new();
        let mut successful = 0;
        let mut failed = 0;
        let mut total_data = 0;
        let mut total_links = 0;
        let mut total_images = 0;
        let mut total_response_time = Duration::ZERO;

        for result in results {
            match result {
                Ok(task_result) => {
                    // task_result es Result<NuclearResult>
                    match task_result {
                        Ok(res) => {
                            if res.error.is_none() {
                                successful += 1;
                            } else {
                                failed += 1;
                            }
                            total_data += res.content_length;
                            total_links += res.links_found.len();
                            total_images += res.images_found.len();
                            total_response_time += res.response_time;
                            final_results.push(res);
                        }
                        Err(e) => {
                            failed += 1;
                            final_results.push(NuclearResult {
                                url: String::new(),
                                status_code: 0,
                                html: String::new(),
                                content_length: 0,
                                response_time: Duration::ZERO,
                                links_found: Vec::new(),
                                images_found: Vec::new(),
                                extracted_data: serde_json::json!({}),
                                crawled_at: chrono::Utc::now(),
                                error: Some(format!("Task error: {}", e)),
                            });
                        }
                    }
                }
                Err(join_error) => {
                    failed += 1;
                    final_results.push(NuclearResult {
                        url: String::new(),
                        status_code: 0,
                        html: String::new(),
                        content_length: 0,
                        response_time: Duration::ZERO,
                        links_found: Vec::new(),
                        images_found: Vec::new(),
                        extracted_data: serde_json::json!({}),
                        crawled_at: chrono::Utc::now(),
                        error: Some(format!("Join error: {}", join_error)),
                    });
                }
            }
        }

        let total_time = start_time.elapsed();
        let urls_per_second = if total_time.as_secs() > 0 {
            total_urls as f64 / total_time.as_secs() as f64
        } else {
            total_urls as f64
        };
        let avg_response_time = if successful > 0 {
            total_response_time / successful as u32
        } else {
            Duration::ZERO
        };

        // Guardar estadísticas
        let stats = NuclearStats {
            total_urls_crawled: total_urls,
            successful,
            failed,
            total_data_captured: total_data,
            total_links_found: total_links,
            total_images_found: total_images,
            total_time,
            urls_per_second,
            avg_response_time,
        };

        self.stats.insert("last_nuclear_crawl".to_string(), stats);

        Ok(final_results)
    }

    /// Scraping NUCLEAR recursivo - Sigue TODOS los links
    pub async fn nuclear_crawl_recursive(
        &self,
        start_urls: Vec<String>,
    ) -> Result<Vec<NuclearResult>> {
        let mut all_results = Vec::new();
        let mut urls_to_crawl = start_urls;
        let mut depth = 0;
        let max_depth = if self.config.unlimited_depth {
            usize::MAX
        } else {
            10
        };

        while !urls_to_crawl.is_empty() && depth < max_depth {
            // Crawlear este nivel
            let results = self.nuclear_crawl(urls_to_crawl.clone()).await?;

            // Agregar resultados
            all_results.extend(results.clone());

            // Si seguimos links, agregar nuevos URLs
            if self.config.follow_all_links {
                let mut new_urls = Vec::new();
                for result in results {
                    if result.error.is_none() {
                        for link in result.links_found {
                            if !self.visited.contains_key(&link) {
                                new_urls.push(link);
                            }
                        }
                    }
                }
                urls_to_crawl = new_urls;
            } else {
                break;
            }

            depth += 1;
        }

        Ok(all_results)
    }

    /// Obtiene estadísticas
    pub fn get_stats(&self) -> Option<NuclearStats> {
        self.stats.get("last_nuclear_crawl").map(|s| s.clone())
    }

    /// Limpia el estado
    pub fn clear(&self) {
        self.visited.clear();
        self.results.clear();
        self.stats.clear();
    }
}
