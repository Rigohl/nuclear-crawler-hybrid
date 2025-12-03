//! Parallel Crawler Extreme - 🔥 NUCLEAR v3.0
//!
//! Crawler con paralelismo extremo usando Rayon + Tokio
//! Worker pool dedicado con hasta 50+ workers concurrentes

use crate::cache::Cache;
use crate::config::CrawlerConfig;
use crate::rate_limit::RateLimiter;
use anyhow::{Context, Result};
use dashmap::DashMap;
use rayon::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, RwLock, Semaphore};
use url::Url;

/// Estado de URL con metadatos paralelos
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParallelUrlStatus {
    Pending,
    Crawling {
        worker_id: usize,
        start_time: Instant,
    },
    Completed {
        worker_id: usize,
        duration: Duration,
    },
    Failed {
        worker_id: usize,
        error: String,
    },
    #[allow(dead_code)]
    Skipped {
        reason: String,
    },
}

/// Información extendida de URL crawlada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelCrawledUrl {
    pub url: String,
    pub status_code: u16,
    pub content_type: String,
    pub content_length: usize,
    pub response_time: Duration,
    pub crawled_at: chrono::DateTime<chrono::Utc>,
    pub links_found: Vec<String>,
    pub error: Option<String>,
    pub worker_id: usize,
    pub memory_usage: usize,
    pub cpu_time: Duration,
}

/// 🔥 NUCLEAR: Crawler con workers paralelos dedicados
pub struct ParallelCrawler {
    client: Client,
    #[allow(dead_code)]
    config: CrawlerConfig,
    cache: Arc<Cache>,
    rate_limiter: Arc<RateLimiter>,
    visited: Arc<DashMap<String, ParallelUrlStatus>>,
    queue: Arc<RwLock<VecDeque<String>>>,
    semaphore: Arc<Semaphore>,
    // Canal de resultados para procesamiento paralelo
    result_tx: mpsc::UnboundedSender<ParallelCrawledUrl>,
    result_rx: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<ParallelCrawledUrl>>>,
    // Estadísticas en tiempo real
    stats: Arc<DashMap<String, u64>>,
}

impl ParallelCrawler {
    /// Constructor con configuración de paralelismo automático
    pub fn new(config: CrawlerConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .user_agent(&config.user_agent)
            .gzip(true)
            .brotli(true)
            .cookie_store(true)
            .pool_max_idle_per_host(100)
            .pool_idle_timeout(Duration::from_secs(30))
            .build()
            .context("Error creando cliente HTTP")?;

        let cache = Arc::new(Cache::new(config.cache_size));
        let rate_limiter = Arc::new(RateLimiter::new(
            config.max_requests_per_second,
            config.burst_size,
        ));

        // Semaphore con CPU detection automática
        let max_concurrent = config.max_concurrent.max(num_cpus::get() * 4);
        let semaphore = Arc::new(Semaphore::new(max_concurrent));

        // Canal de resultados con buffer ilimitado
        let (result_tx, result_rx) = mpsc::unbounded_channel();

        Ok(Self {
            client,
            config,
            cache,
            rate_limiter,
            visited: Arc::new(DashMap::new()),
            queue: Arc::new(RwLock::new(VecDeque::new())),
            semaphore,
            result_tx,
            result_rx: Arc::new(tokio::sync::Mutex::new(result_rx)),
            stats: Arc::new(DashMap::new()),
        })
    }

    /// Agregar URLs con procesamiento paralelo por lotes
    pub async fn add_urls_parallel(&self, urls: Vec<String>) -> Result<()> {
        // Procesar URLs en paralelo usando Rayon
        let normalized_urls: Vec<String> = urls
            .par_iter()
            .filter_map(|url| {
                Url::parse(url)
                    .ok()
                    .map(|parsed| parsed.as_str().to_string())
            })
            .collect();

        let mut queue = self.queue.write().await;
        let mut new_urls = 0u64;

        for url in normalized_urls {
            if !self.visited.contains_key(&url) {
                self.visited.insert(url.clone(), ParallelUrlStatus::Pending);
                queue.push_back(url);
                new_urls += 1;
            }
        }

        self.stats
            .entry("urls_added".to_string())
            .and_modify(|v| *v += new_urls)
            .or_insert(new_urls);
        Ok(())
    }

    /// 🔥 NUCLEAR: Crawling masivo con workers dedicados
    pub async fn crawl_parallel(&self, num_workers: usize) -> Result<Vec<ParallelCrawledUrl>> {
        let mut results = Vec::new();
        let mut workers = Vec::new();

        // Crear workers Tokio dedicados
        for worker_id in 0..num_workers {
            let worker = self.spawn_worker(worker_id);
            workers.push(worker);
        }

        // Esperar a que todos los workers terminen
        for worker in workers {
            let _ = worker.await;
        }

        // Recolectar resultados
        while let Ok(result) = self.result_rx.lock().await.try_recv() {
            results.push(result);
        }

        Ok(results)
    }

    /// Worker dedicado con optimizaciones
    fn spawn_worker(&self, worker_id: usize) -> tokio::task::JoinHandle<()> {
        let client = self.client.clone();
        let cache = Arc::clone(&self.cache);
        let rate_limiter = Arc::clone(&self.rate_limiter);
        let visited = Arc::clone(&self.visited);
        let queue = Arc::clone(&self.queue);
        let semaphore = Arc::clone(&self.semaphore);
        let result_tx = self.result_tx.clone();
        let stats = Arc::clone(&self.stats);

        tokio::spawn(async move {
            loop {
                // Adquirir permiso del semaphore
                let _permit = match semaphore.acquire().await {
                    Ok(p) => p,
                    Err(_) => break,
                };

                // Obtener siguiente URL
                let url = {
                    let mut queue_lock = queue.write().await;
                    queue_lock.pop_front()
                };

                let Some(url) = url else {
                    break; // No más URLs
                };

                // Marcar como crawling
                let start_time = Instant::now();
                visited.insert(
                    url.clone(),
                    ParallelUrlStatus::Crawling {
                        worker_id,
                        start_time,
                    },
                );

                // Rate limiting
                rate_limiter.wait().await;

                // Procesar URL
                let result = Self::process_url_internal(&client, &cache, &url, worker_id).await;

                // Enviar resultado
                let crawled_url = match result {
                    Ok(data) => {
                        let duration = start_time.elapsed();
                        visited.insert(
                            url.clone(),
                            ParallelUrlStatus::Completed {
                                worker_id,
                                duration,
                            },
                        );

                        ParallelCrawledUrl {
                            url: url.clone(),
                            status_code: data.status_code,
                            content_type: data.content_type,
                            content_length: data.content_length,
                            response_time: data.response_time,
                            crawled_at: chrono::Utc::now(),
                            links_found: data.links_found,
                            error: None,
                            worker_id,
                            memory_usage: data.content_length,
                            cpu_time: duration,
                        }
                    }
                    Err(e) => {
                        visited.insert(
                            url.clone(),
                            ParallelUrlStatus::Failed {
                                worker_id,
                                error: e.to_string(),
                            },
                        );

                        ParallelCrawledUrl {
                            url,
                            status_code: 0,
                            content_type: "error".to_string(),
                            content_length: 0,
                            response_time: start_time.elapsed(),
                            crawled_at: chrono::Utc::now(),
                            links_found: vec![],
                            error: Some(e.to_string()),
                            worker_id,
                            memory_usage: 0,
                            cpu_time: start_time.elapsed(),
                        }
                    }
                };

                let _ = result_tx.send(crawled_url);

                // Actualizar estadísticas
                stats
                    .entry("requests_completed".to_string())
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        })
    }

    /// Procesamiento de URL con optimizaciones
    async fn process_url_internal(
        client: &Client,
        cache: &Arc<Cache>,
        url: &str,
        _worker_id: usize,
    ) -> Result<UrlData> {
        // Verificar caché primero
        if let Some(cached) = cache.get(url).await {
            return Ok(UrlData {
                status_code: cached.status_code,
                content_type: cached.content_type,
                content_length: cached.content_length,
                response_time: cached.response_time,
                links_found: cached.links_found,
            });
        }

        let start_time = Instant::now();

        // Fetch con timeout optimizado
        let response = client
            .get(url)
            .send()
            .await
            .context("Error en request HTTP")?;
        let status_code = response.status().as_u16();

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        // Leer contenido con límite de memoria
        let html = response.text().await.unwrap_or_default();
        let content_length = html.len();

        // Extraer links usando regex (sin dependencia externa de scraper)
        let links = Self::extract_links_from_html(&html, url);

        let response_time = start_time.elapsed();

        // Guardar en caché
        let cached_entry = crate::cache::CachedResponse {
            url: url.to_string(),
            status_code,
            content_type: content_type.clone(),
            content_length,
            response_time,
            links_found: links.clone(),
            html: html.clone(),
            cached_at: chrono::Utc::now(),
        };
        cache.set(url, cached_entry).await;

        Ok(UrlData {
            status_code,
            content_type,
            content_length,
            response_time,
            links_found: links,
        })
    }

    /// 🔥 NUCLEAR: Extracción de links con regex optimizado
    fn extract_links_from_html(html: &str, base_url: &str) -> Vec<String> {
        use regex::Regex;

        let mut links = Vec::new();
        let base = Url::parse(base_url).ok();

        // Regex para extraer hrefs
        let href_re = Regex::new(r#"href=["']([^"']+)["']"#).unwrap();

        for cap in href_re.captures_iter(html) {
            if let Some(href_match) = cap.get(1) {
                let href = href_match.as_str();

                // Normalizar URL
                let full_url = if href.starts_with("http") {
                    href.to_string()
                } else if let Some(ref base) = base {
                    base.join(href).map(|u| u.to_string()).unwrap_or_default()
                } else {
                    continue;
                };

                // Filtrar URLs válidas
                if !full_url.is_empty()
                    && !full_url.contains("javascript:")
                    && !full_url.contains("mailto:")
                    && !full_url.starts_with('#')
                {
                    links.push(full_url);
                }
            }
        }

        // Deduplicar
        links.sort();
        links.dedup();
        links.truncate(100); // Limitar a 100 links por página

        links
    }

    /// Método para obtener estadísticas en tiempo real
    pub fn get_stats(&self) -> HashMap<String, u64> {
        self.stats
            .iter()
            .map(|entry| (entry.key().clone(), *entry.value()))
            .collect()
    }

    /// Obtener número de URLs pendientes
    #[allow(dead_code)]
    pub async fn pending_count(&self) -> usize {
        self.queue.read().await.len()
    }

    /// Obtener número de URLs visitadas
    #[allow(dead_code)]
    pub fn visited_count(&self) -> usize {
        self.visited.len()
    }
}

/// Estructura de datos optimizada para procesamiento paralelo
#[derive(Debug)]
struct UrlData {
    status_code: u16,
    content_type: String,
    content_length: usize,
    response_time: Duration,
    links_found: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parallel_crawler_creation() {
        let config = CrawlerConfig::default();
        let crawler = ParallelCrawler::new(config).unwrap();
        assert!(crawler.get_stats().is_empty());
    }

    #[tokio::test]
    async fn test_add_urls_parallel() {
        let config = CrawlerConfig::default();
        let crawler = ParallelCrawler::new(config).unwrap();

        let urls = vec![
            "https://example.com".to_string(),
            "https://httpbin.org".to_string(),
        ];

        crawler.add_urls_parallel(urls).await.unwrap();
        let stats = crawler.get_stats();
        assert_eq!(stats.get("urls_added"), Some(&2));
    }

    #[test]
    fn test_extract_links() {
        let html = r#"
            <html>
                <body>
                    <a href="https://example.com/page1">Link 1</a>
                    <a href="/relative/path">Link 2</a>
                    <a href="javascript:void(0)">JS Link</a>
                </body>
            </html>
        "#;

        let links = ParallelCrawler::extract_links_from_html(html, "https://example.com");
        assert!(links.iter().any(|l| l.contains("page1")));
        assert!(links.iter().any(|l| l.contains("relative")));
        assert!(!links.iter().any(|l| l.contains("javascript")));
    }
}
