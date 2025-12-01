//! 🚀 NUCLEAR MAX POWER - Usa TODAS las mejoras al 100%
//!
//! Este módulo combina:
//! - WebSearch para búsqueda masiva
//! - Circuit Breaker para tolerancia
//! - Rate Limiter para control
//! - Metrics para estadísticas

use crate::improvements::*;
use crate::web_search::{WebSearch, WebSearchConfig};
use anyhow::Result;
use serde_json::json;
use std::sync::Arc;

/// Sistema NUCLEAR con TODA la potencia
pub struct NuclearMaxPower {
    web_search: Arc<WebSearch>,
    circuit_breaker: Arc<parking_lot::RwLock<CircuitBreaker>>,
    rate_limiter: Arc<SemaphoreLimiter>,
    metrics: Arc<MetricsCollector>,
}

impl NuclearMaxPower {
    /// Crea sistema NUCLEAR con TODO al máximo
    pub async fn new_max_power() -> Result<Self> {
        // Storage inteligente con caché de 5M
        let storage = Arc::new(crate::intelligent_storage::IntelligentStorage::new(None)?);

        let metrics = Arc::new(MetricsCollector::new());
        metrics.register("search_queries", MetricType::Counter);
        metrics.register("search_results", MetricType::Counter);
        
        Ok(Self {
            web_search: Arc::new(WebSearch::new_with_storage(Some(storage))?),
            circuit_breaker: Arc::new(parking_lot::RwLock::new(CircuitBreaker::new(5, 30))), // 5 fallos = abierto 30s
            rate_limiter: Arc::new(SemaphoreLimiter::new(2000)), // 2000 concurrent
            metrics,
        })
    }

    /// SEARCH NUCLEAR - Búsqueda masiva con TODAS las mejoras
    pub async fn search_nuclear(&self, query: &str, max_results: usize) -> Result<serde_json::Value> {
        let start = std::time::Instant::now();

        // Verificar circuit breaker
        if self.circuit_breaker.read().is_open() {
            return Err(anyhow::anyhow!("Circuit breaker OPEN - Sistema en recuperación"));
        }

        // Esperar rate limiter (controla concurrencia a 2000)
        let _permit = self.rate_limiter.acquire().await;

        // Smart search config - NUCLEAR TOTAL
        let search_config = WebSearchConfig {
            query: query.to_string(),
            max_results: if max_results == 0 { 1000 } else { max_results.min(2000) }, // Max 2000
            priority_sources: vec![
                "github.com".to_string(),
                "stackoverflow.com".to_string(),
                "dev.to".to_string(),
                "huggingface.co".to_string(),
            ],
            sources: vec![
                "github.com".to_string(),
                "gitlab.com".to_string(),
                "bitbucket.org".to_string(),
                "reddit.com".to_string(),
                "stackoverflow.com".to_string(),
                "medium.com".to_string(),
                "dev.to".to_string(),
                "hashnode.com".to_string(),
                "huggingface.co".to_string(),
                "paperswithcode.com".to_string(),
                "arxiv.org".to_string(),
                "techcrunch.com".to_string(),
                "hackernews.com".to_string(),
            ],
            use_ai: true,
            use_stealth: true,
            max_parallel: 2000, // NUCLEAR
        };

        // Ejecutar búsqueda
        let results = self.web_search.search(search_config).await?;

        // Registrar éxito
        self.circuit_breaker.write().record_success();
        self.metrics.increment("search_queries");
        self.metrics.set_gauge("search_results", results.len() as u64);

        // Convertir a JSON
        let results_json = json!({
            "query": query,
            "results_count": results.len(),
            "results": results.iter().map(|r| json!({
                "url": r.url,
                "title": r.title,
                "description": r.description,
                "relevance": r.relevance,
                "quality_score": r.quality_score,
                "source": r.source,
            })).collect::<Vec<_>>(),
            "search_time_ms": start.elapsed().as_millis(),
        });

        Ok(results_json)
    }

    /// Obtener estadísticas del sistema
    pub fn get_stats(&self) -> serde_json::Value {
        json!({
            "circuit_breaker": {
                "state": format!("{:?}", self.circuit_breaker.read().state()),
                "is_open": self.circuit_breaker.read().is_open(),
            },
            "rate_limiter": {
                "max_concurrent": 2000,
                "mode": "NUCLEAR"
            },
            "prometheus_metrics": self.metrics.export_prometheus(),
        })
    }

    /// Reset del sistema
    pub async fn reset(&self) {
        self.circuit_breaker.write().record_success();
    }
}
