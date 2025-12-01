//! Módulo Config - Configuración del crawler
//!
//! Maneja la configuración y opciones del crawler

use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::time::Duration;

/// Configuración del crawler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlerConfig {
    /// User agent a usar
    pub user_agent: String,

    /// Timeout en segundos
    pub timeout_seconds: u64,

    /// Número máximo de requests concurrentes
    pub max_concurrent: usize,

    /// Requests por segundo máximo
    pub max_requests_per_second: u32,

    /// Tamaño de burst para rate limiting
    pub burst_size: u32,

    /// Tamaño máximo del caché
    pub cache_size: usize,

    /// Seguir links encontrados
    pub follow_links: bool,

    /// Profundidad máxima de crawling
    pub max_depth: Option<usize>,

    /// Dominios permitidos (None = todos)
    pub allowed_domains: Option<Vec<String>>,

    /// Patrones excluidos de URLs
    pub excluded_patterns: Option<Vec<String>>,

    /// Número máximo de resultados
    pub max_results: Option<usize>,

    /// Headers adicionales
    pub extra_headers: Option<std::collections::HashMap<String, String>>,
}

impl Default for CrawlerConfig {
    fn default() -> Self {
        Self {
            user_agent: "Nuclear Crawler Hybrid/0.1.0".to_string(),
            timeout_seconds: 60,
            max_concurrent: 200,           // NUCLEAR: Paralelismo extremo
            max_requests_per_second: 1000, // NUCLEAR: Velocidad masiva
            burst_size: 500,               // NUCLEAR: Burst masivo
            cache_size: 100000,            // NUCLEAR: Caché enorme
            follow_links: true,
            max_depth: None,       // NUCLEAR: Sin límite de profundidad
            allowed_domains: None, // NUCLEAR: Sin restricciones
            excluded_patterns: None,
            max_results: None, // NUCLEAR: Sin límite de resultados
            extra_headers: None,
        }
    }
}

impl CrawlerConfig {
    /// Crea una configuración con valores personalizados
    pub fn custom() -> Self {
        Self::default()
    }

    /// Configuración NUCLEAR - Máximo poder TOTAL 🚀
    pub fn nuclear() -> Self {
        Self {
            user_agent: "Nuclear Crawler Hybrid/0.1.0".to_string(),
            timeout_seconds: 180,         // 3 MINUTOS - Deep crawl profundo
            max_concurrent: 2000,          // 2000 PARALELO - Máximo sistema
            max_requests_per_second: 10000, // 10K/s - Velocidad extrema
            burst_size: 5000,              // 5000 BURST - Ráfagas monstruo
            cache_size: 5000000,           // 5M CACHÉ - Memoria masiva
            follow_links: true,
            max_depth: None, // Sin límite
            allowed_domains: None,
            excluded_patterns: None,
            max_results: None,
            extra_headers: None,
        }
    }

    /// Configuración para scraping rápido
    pub fn fast() -> Self {
        Self {
            max_concurrent: 200,
            max_requests_per_second: 500,
            burst_size: 1000,
            ..Default::default()
        }
    }

    /// Configuración para scraping cuidadoso (respetuoso)
    pub fn respectful() -> Self {
        Self {
            max_concurrent: 10,
            max_requests_per_second: 10,
            burst_size: 20,
            ..Default::default()
        }
    }
}
