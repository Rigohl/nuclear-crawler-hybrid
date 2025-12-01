//! Módulo Crawler - Motor principal de crawling
//!
//! Maneja la lógica de crawling, rate limiting, y gestión de colas

use crate::cache::Cache;
use crate::config::CrawlerConfig;
use crate::rate_limit::RateLimiter;
use crate::scraper::Scraper;
use anyhow::{Context, Result};
use dashmap::DashMap;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashSet;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};
use url::Url;

/// Estado de una URL en el crawler
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UrlStatus {
    Pending,
    Crawling,
    Completed,
    Failed,
    Skipped,
}

/// Información sobre una URL crawlada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawledUrl {
    pub url: String,
    pub status_code: u16,
    pub content_type: String,
    pub content_length: usize,
    pub response_time: Duration,
    pub crawled_at: chrono::DateTime<chrono::Utc>,
    pub links_found: Vec<String>,
    pub error: Option<String>,
}

/// Crawler principal
pub struct Crawler {
    client: Client,
    config: CrawlerConfig,
    cache: Arc<Cache>,
    rate_limiter: Arc<RateLimiter>,
    visited: Arc<DashMap<String, UrlStatus>>,
    queue: Arc<RwLock<VecDeque<String>>>,
    semaphore: Arc<Semaphore>,
    scraper: Arc<Scraper>,
}

impl Crawler {
    /// Crea un nuevo crawler
    pub fn new(config: CrawlerConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .user_agent(&config.user_agent)
            .gzip(true)
            .brotli(true)
            .cookie_store(true)
            .build()
            .context("Error creando cliente HTTP")?;

        let cache = Arc::new(Cache::new(config.cache_size));
        let rate_limiter = Arc::new(RateLimiter::new(
            config.max_requests_per_second,
            config.burst_size,
        ));

        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));

        let scraper = Arc::new(Scraper::new());

        Ok(Self {
            client,
            config,
            cache,
            rate_limiter,
            visited: Arc::new(DashMap::new()),
            queue: Arc::new(RwLock::new(VecDeque::new())),
            semaphore,
            scraper,
        })
    }

    /// Agrega URLs a la cola
    pub async fn add_urls(&self, urls: Vec<String>) -> Result<()> {
        let mut queue = self.queue.write().await;
        for url in urls {
            // Normalizar URL
            if let Ok(parsed) = Url::parse(&url) {
                let normalized = parsed.as_str().to_string();

                // Verificar si ya fue visitada
                if !self.visited.contains_key(&normalized) {
                    queue.push_back(normalized);
                }
            }
        }
        Ok(())
    }

    /// Crawlea una URL individual
    pub async fn crawl_url(&self, url: &str) -> Result<CrawledUrl> {
        // Verificar caché
        if let Some(cached) = self.cache.get(url).await {
            return Ok(cached);
        }

        // Rate limiting
        self.rate_limiter.wait().await;

        // Marcar como crawling
        self.visited.insert(url.to_string(), UrlStatus::Crawling);

        let start_time = Instant::now();

        let result = self.fetch_url(url).await;

        let response_time = start_time.elapsed();

        match result {
            Ok(response) => {
                let status_code = response.status().as_u16();
                let content_type = response
                    .headers()
                    .get("content-type")
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or("unknown")
                    .to_string();

                let html = response.text().await.context("Error leyendo respuesta")?;

                let content_length = html.len();

                // Extraer links
                let links = self.scraper.extract_links(&html, url)?;

                // Filtrar y agregar links válidos a la cola
                if self.config.follow_links {
                    let valid_links: Vec<String> = links
                        .iter()
                        .filter(|link| self.is_valid_url(link))
                        .filter(|link| !self.visited.contains_key(*link))
                        .cloned()
                        .collect();

                    if !valid_links.is_empty() {
                        self.add_urls(valid_links).await?;
                    }
                }

                let crawled = CrawledUrl {
                    url: url.to_string(),
                    status_code,
                    content_type,
                    content_length,
                    response_time,
                    crawled_at: chrono::Utc::now(),
                    links_found: links,
                    error: None,
                };

                // Guardar en caché
                self.cache.set(url, &crawled).await;

                self.visited.insert(url.to_string(), UrlStatus::Completed);

                Ok(crawled)
            }
            Err(e) => {
                let error_msg = format!("{}", e);
                self.visited.insert(url.to_string(), UrlStatus::Failed);

                Err(anyhow::anyhow!("Error crawling {}: {}", url, error_msg))
            }
        }
    }

    /// Fetch de una URL
    async fn fetch_url(&self, url: &str) -> Result<Response> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Error en request HTTP")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("HTTP error: {}", response.status()));
        }

        Ok(response)
    }

    /// Verifica si una URL es válida para crawling
    fn is_valid_url(&self, url: &str) -> bool {
        // Verificar dominio permitido
        if let Some(allowed_domains) = &self.config.allowed_domains {
            if let Ok(parsed) = Url::parse(url) {
                if let Some(domain) = parsed.host_str() {
                    if !allowed_domains.iter().any(|d| domain.ends_with(d)) {
                        return false;
                    }
                }
            }
        }

        // Verificar patrones excluidos
        if let Some(excluded_patterns) = &self.config.excluded_patterns {
            for pattern in excluded_patterns {
                if url.contains(pattern) {
                    return false;
                }
            }
        }

        // Verificar profundidad máxima
        if let Some(_max_depth) = self.config.max_depth {
            // Implementar lógica de profundidad si es necesario
        }

        true
    }

    /// Procesa la cola de URLs
    pub async fn process_queue(&self) -> Result<Vec<CrawledUrl>> {
        let mut results = Vec::new();

        loop {
            // Obtener siguiente URL de la cola
            let url = {
                let mut queue = self.queue.write().await;
                queue.pop_front()
            };

            if let Some(url) = url {
                // Adquirir permiso de semáforo
                let _permit = self
                    .semaphore
                    .acquire()
                    .await
                    .context("Error adquiriendo semáforo")?;

                // Crawlear URL
                match self.crawl_url(&url).await {
                    Ok(crawled) => {
                        results.push(crawled);
                    }
                    Err(e) => {
                        eprintln!("Error crawling {}: {}", url, e);
                    }
                }
            } else {
                // Cola vacía
                break;
            }

            // Verificar límite de resultados
            if let Some(max_results) = self.config.max_results {
                if results.len() >= max_results {
                    break;
                }
            }
        }

        Ok(results)
    }

    /// Crawlea múltiples URLs en paralelo
    pub async fn crawl_many(&self, urls: Vec<String>) -> Result<Vec<CrawledUrl>> {
        self.add_urls(urls).await?;
        self.process_queue().await
    }
}
