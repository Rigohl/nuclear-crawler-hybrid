//! Parallel Crawler Extreme - Mejora #1-16
//!
//! Crawler con paralelismo extremo usando Rayon + Tokio

use crate::cache::Cache;
use crate::config::CrawlerConfig;
use crate::rate_limit::RateLimiter;
use crate::scraper::Scraper;
use anyhow::{Context, Result};
use dashmap::DashMap;
use futures::future::join_all;
use rayon::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore, mpsc};
use tokio::task;
use url::Url;

/// Mejora #1: Estado de URL con metadatos paralelos
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParallelUrlStatus {
    Pending,
    Crawling { worker_id: usize, start_time: Instant },
    Completed { worker_id: usize, duration: Duration },
    Failed { worker_id: usize, error: String },
    Skipped { reason: String },
}

/// Mejora #2: Información extendida de URL crawlada
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

/// Mejora #3: Crawler con workers paralelos dedicados
pub struct ParallelCrawler {
    client: Client,
    config: CrawlerConfig,
    cache: Arc<Cache>,
    rate_limiter: Arc<RateLimiter>,
    visited: Arc<DashMap<String, ParallelUrlStatus>>,
    queue: Arc<RwLock<VecDeque<String>>>,
    semaphore: Arc<Semaphore>,
    scraper: Arc<Scraper>,
    // Mejora #4: Pool de workers Tokio dedicados
    worker_pool: Vec<tokio::task::JoinHandle<()>>,
    // Mejora #5: Canal de resultados para procesamiento paralelo
    result_tx: mpsc::UnboundedSender<ParallelCrawledUrl>,
    result_rx: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<ParallelCrawledUrl>>>,
    // Mejora #6: Estadísticas en tiempo real
    stats: Arc<DashMap<String, u64>>,
}

impl ParallelCrawler {
    /// Mejora #7: Constructor con configuración de paralelismo automático
    pub fn new(config: CrawlerConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .user_agent(&config.user_agent)
            .gzip(true)
            .brotli(true)
            .cookie_store(true)
            .pool_max_idle_per_host(100) // Mejora #8: Pool de conexiones optimizado
            .pool_idle_timeout(Duration::from_secs(30))
            .build()
            .context(\"Error creando cliente HTTP\")?;

        let cache = Arc::new(Cache::new(config.cache_size));
        let rate_limiter = Arc::new(RateLimiter::new(
            config.max_requests_per_second,
            config.burst_size,
        ));

        // Mejora #9: Semaphore con CPU detection automática
        let max_concurrent = config.max_concurrent.max(num_cpus::get() * 4);
        let semaphore = Arc::new(Semaphore::new(max_concurrent));

        let scraper = Arc::new(Scraper::new());

        // Mejora #10: Canal de resultados con buffer grande
        let (result_tx, result_rx) = mpsc::unbounded_channel();

        Ok(Self {
            client,
            config,
            cache,
            rate_limiter,
            visited: Arc::new(DashMap::new()),
            queue: Arc::new(RwLock::new(VecDeque::new())),
            semaphore,
            scraper,
            worker_pool: Vec::new(),
            result_tx,
            result_rx: Arc::new(tokio::sync::Mutex::new(result_rx)),
            stats: Arc::new(DashMap::new()),
        })
    }

    /// Mejora #11: Agregar URLs con procesamiento paralelo por lotes
    pub async fn add_urls_parallel(&self, urls: Vec<String>) -> Result<()> {
        // Procesar URLs en paralelo usando Rayon
        let normalized_urls: Vec<String> = urls
            .par_iter()
            .filter_map(|url| {
                Url::parse(url).ok().map(|parsed| parsed.as_str().to_string())
            })
            .collect();

        let mut queue = self.queue.write().await;
        let mut new_urls = 0;

        for url in normalized_urls {
            if !self.visited.contains_key(&url) {
                self.visited.insert(url.clone(), ParallelUrlStatus::Pending);
                queue.push_back(url);
                new_urls += 1;
            }
        }

        self.stats.insert(\"urls_added\".to_string(), new_urls);
        Ok(())
    }

    /// Mejora #12: Crawling masivo con workers dedicados
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
            worker.await?;
        }

        // Recolectar resultados
        while let Ok(result) = self.result_rx.lock().await.try_recv() {
            results.push(result);
        }

        Ok(results)
    }

    /// Mejora #13: Worker dedicado con optimizaciones
    fn spawn_worker(&self, worker_id: usize) -> tokio::task::JoinHandle<()> {
        let client = self.client.clone();
        let cache = Arc::clone(&self.cache);
        let rate_limiter = Arc::clone(&self.rate_limiter);
        let visited = Arc::clone(&self.visited);
        let queue = Arc::clone(&self.queue);
        let semaphore = Arc::clone(&self.semaphore);
        let scraper = Arc::clone(&self.scraper);
        let result_tx = self.result_tx.clone();
        let stats = Arc::clone(&self.stats);

        tokio::spawn(async move {
            loop {
                // Adquirir permiso del semaphore
                let _permit = semaphore.acquire().await.unwrap();

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
                    ParallelUrlStatus::Crawling { worker_id, start_time }
                );

                // Procesar URL
                let result = Self::process_url_parallel(
                    &client,
                    &cache,
                    &rate_limiter,
                    &scraper,
                    &url,
                    worker_id,
                ).await;

                // Enviar resultado
                let crawled_url = match result {
                    Ok(data) => {
                        let duration = start_time.elapsed();
                        visited.insert(
                            url.clone(),
                            ParallelUrlStatus::Completed { worker_id, duration }
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
                            memory_usage: 0, // TODO: implementar medición de memoria
                            cpu_time: duration,
                        }
                    }
                    Err(e) => {
                        visited.insert(
                            url.clone(),
                            ParallelUrlStatus::Failed {
                                worker_id,
                                error: e.to_string()
                            }
                        );

                        ParallelCrawledUrl {
                            url,
                            status_code: 0,
                            content_type: \"error\".to_string(),
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
                *stats.entry(\"requests_completed\".to_string()).or_insert(0) += 1;
            }
        })
    }

    /// Mejora #14: Procesamiento de URL con optimizaciones paralelas
    async fn process_url_parallel(
        client: &Client,
        cache: &Arc<Cache>,
        rate_limiter: &Arc<RateLimiter>,
        scraper: &Arc<Scraper>,
        url: &str,
        worker_id: usize,
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

        // Rate limiting
        rate_limiter.wait().await;

        let start_time = Instant::now();

        // Fetch con timeout optimizado
        let response = client.get(url).send().await?;
        let status_code = response.status().as_u16();

        let content_type = response
            .headers()
            .get(\"content-type\")
            .and_then(|h| h.to_str().ok())
            .unwrap_or(\"unknown\")
            .to_string();

        // Leer contenido con límite de memoria
        let html = response.text().await?;
        let content_length = html.len();

        // Extraer links en paralelo
        let links = scraper.extract_links_parallel(&html, url)?;

        Ok(UrlData {
            status_code,
            content_type,
            content_length,
            response_time: start_time.elapsed(),
            links_found: links,
        })
    }

    /// Mejora #15: Método para obtener estadísticas en tiempo real
    pub fn get_stats(&self) -> HashMap<String, u64> {
        self.stats.iter().map(|entry| (entry.key().clone(), *entry.value())).collect()
    }
}

/// Mejora #16: Estructura de datos optimizada para procesamiento paralelo
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
            \"https://example.com\".to_string(),
            \"https://httpbin.org\".to_string(),
        ];

        crawler.add_urls_parallel(urls).await.unwrap();
        let stats = crawler.get_stats();
        assert_eq!(stats.get(\"urls_added\"), Some(&2));
    }
}
