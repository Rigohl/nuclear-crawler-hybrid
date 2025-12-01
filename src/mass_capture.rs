//! Módulo Captura Masiva - Sistema de scraping masivo
//!
//! Implementación de alto rendimiento para captura masiva de información
//! usando paralelismo extremo y procesamiento optimizado

use crate::config::CrawlerConfig;
use crate::crawler::Crawler;
use crate::scraper::Scraper;
use anyhow::Result;
use dashmap::DashMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

/// Estadísticas de captura masiva
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassCaptureStats {
    pub total_urls: usize,
    pub successful: usize,
    pub failed: usize,
    pub total_time: Duration,
    pub avg_response_time: Duration,
    pub total_data_captured: usize, // bytes
    pub links_found: usize,
    pub images_found: usize,
}

/// Resultado de captura masiva
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassCaptureResult {
    pub url: String,
    pub status_code: u16,
    pub content_length: usize,
    pub response_time: Duration,
    pub links_found: Vec<String>,
    pub images_found: Vec<String>,
    pub extracted_data: serde_json::Value,
    pub error: Option<String>,
}

/// Sistema de captura masiva
pub struct MassCapture {
    crawler: Arc<Crawler>,
    scraper: Arc<Scraper>,
    semaphore: Arc<Semaphore>,
    stats: Arc<DashMap<String, MassCaptureStats>>,
}

impl MassCapture {
    /// Crea un nuevo sistema de captura masiva
    pub fn new(config: CrawlerConfig) -> Result<Self> {
        let crawler = Arc::new(Crawler::new(config.clone())?);
        let scraper = Arc::new(Scraper::new());
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));

        Ok(Self {
            crawler,
            scraper,
            semaphore,
            stats: Arc::new(DashMap::new()),
        })
    }

    /// Captura masiva de URLs con paralelismo extremo
    pub async fn capture_massive(&self, urls: Vec<String>) -> Result<Vec<MassCaptureResult>> {
        let start_time = Instant::now();
        let total_urls = urls.len();

        // Procesar en paralelo con rayon para máximo rendimiento
        let results: Vec<Result<MassCaptureResult>> = urls
            .par_iter()
            .map(|url| {
                // Usar semáforo para controlar concurrencia
                let permit = self.semaphore.clone();
                let url_clone = url.clone();
                let crawler = self.crawler.clone();
                let scraper = self.scraper.clone();

                // Ejecutar en runtime async
                let rt = tokio::runtime::Handle::current();
                rt.block_on(async move {
                    let _permit = permit.acquire().await?;

                    let start = Instant::now();
                    match crawler.crawl_url(&url_clone).await {
                        Ok(crawled) => {
                            // Extraer datos adicionales
                            let extracted = scraper
                                .extract_all(&crawled.url, &url_clone)
                                .unwrap_or_default();

                            Ok(MassCaptureResult {
                                url: url_clone,
                                status_code: crawled.status_code,
                                content_length: crawled.content_length,
                                response_time: start.elapsed(),
                                links_found: crawled.links_found,
                                images_found: extracted
                                    .images
                                    .iter()
                                    .map(|i| i.url.clone())
                                    .collect(),
                                extracted_data: serde_json::json!({
                                    "title": extracted.title,
                                    "meta": extracted.meta,
                                    "headings": extracted.headings,
                                    "tables": extracted.tables,
                                }),
                                error: None,
                            })
                        }
                        Err(e) => Ok(MassCaptureResult {
                            url: url_clone,
                            status_code: 0,
                            content_length: 0,
                            response_time: start.elapsed(),
                            links_found: Vec::new(),
                            images_found: Vec::new(),
                            extracted_data: serde_json::json!({}),
                            error: Some(format!("{}", e)),
                        }),
                    }
                })
            })
            .collect();

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
                    final_results.push(MassCaptureResult {
                        url: String::new(),
                        status_code: 0,
                        content_length: 0,
                        response_time: Duration::ZERO,
                        links_found: Vec::new(),
                        images_found: Vec::new(),
                        extracted_data: serde_json::json!({}),
                        error: Some(format!("{}", e)),
                    });
                }
            }
        }

        let total_time = start_time.elapsed();
        let avg_response_time = if successful > 0 {
            total_response_time / successful as u32
        } else {
            Duration::ZERO
        };

        // Guardar estadísticas
        let stats = MassCaptureStats {
            total_urls,
            successful,
            failed,
            total_time,
            avg_response_time,
            total_data_captured: total_data,
            links_found: total_links,
            images_found: total_images,
        };

        self.stats.insert("last_capture".to_string(), stats);

        Ok(final_results)
    }

    /// Captura masiva con procesamiento en lotes
    pub async fn capture_in_batches(
        &self,
        urls: Vec<String>,
        batch_size: usize,
    ) -> Result<Vec<MassCaptureResult>> {
        let mut all_results = Vec::new();

        for chunk in urls.chunks(batch_size) {
            let batch_results = self.capture_massive(chunk.to_vec()).await?;
            all_results.extend(batch_results);
        }

        Ok(all_results)
    }

    /// Obtiene estadísticas de la última captura
    pub fn get_stats(&self) -> Option<MassCaptureStats> {
        self.stats.get("last_capture").map(|s| s.clone())
    }
}
