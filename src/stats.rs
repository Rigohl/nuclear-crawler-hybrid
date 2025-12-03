//! Módulo Stats - Estadísticas y últimas búsquedas
//!
//! Muestra estadísticas completas del sistema

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::ai_smart::AISmart;
use crate::nuclear_scraper::NuclearScraper;
use crate::web_search::WebSearch;

/// Estadísticas completas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    /// Estadísticas de búsquedas web
    pub web_search_stats: WebSearchStats,

    /// Estadísticas de scraping
    pub scraping_stats: ScrapingStats,

    /// Estadísticas de AI
    pub ai_stats: AIStats,

    /// Últimas búsquedas
    pub recent_searches: Vec<RecentSearch>,

    /// Tiempo total de operación
    pub total_uptime: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchStats {
    pub total_searches: usize,
    pub successful_searches: usize,
    pub failed_searches: usize,
    pub avg_results_per_search: f32,
    pub total_results_found: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapingStats {
    pub total_urls_crawled: usize,
    pub successful: usize,
    pub failed: usize,
    pub total_data_captured_mb: f32,
    pub avg_response_time_ms: f32,
    pub urls_per_second: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIStats {
    pub patterns_learned: usize,
    pub domains_analyzed: usize,
    pub predictions_made: usize,
    pub accuracy: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentSearch {
    pub query: String,
    pub timestamp: SystemTime,
    pub results_count: usize,
    pub duration_ms: u64,
    pub success: bool,
}

/// Sistema de estadísticas
#[allow(dead_code)]
pub struct StatsSystem {
    web_search: Arc<WebSearch>,
    scraper: Arc<NuclearScraper>,
    ai_smart: Arc<AISmart>,
    start_time: SystemTime,
    search_history: Arc<DashMap<String, Vec<RecentSearch>>>,
}

impl StatsSystem {
    /// Crea nuevo sistema de stats
    pub fn new(
        web_search: Arc<WebSearch>,
        scraper: Arc<NuclearScraper>,
        ai_smart: Arc<AISmart>,
    ) -> Self {
        Self {
            web_search,
            scraper,
            ai_smart,
            start_time: SystemTime::now(),
            search_history: Arc::new(DashMap::new()),
        }
    }

    /// Obtiene estadísticas completas
    pub fn get_full_stats(&self) -> SystemStats {
        // Web search stats
        let total_searches = self.search_history.len();
        let successful_searches = self
            .search_history
            .iter()
            .filter(|entry| !entry.value().is_empty())
            .count();

        let total_results = self
            .search_history
            .iter()
            .map(|entry| entry.value().len())
            .sum::<usize>();

        let web_search_stats = WebSearchStats {
            total_searches,
            successful_searches,
            failed_searches: total_searches - successful_searches,
            avg_results_per_search: if total_searches > 0 {
                total_results as f32 / total_searches as f32
            } else {
                0.0
            },
            total_results_found: total_results,
        };

        // Scraping stats (simplificado)
        let scraping_stats = ScrapingStats {
            total_urls_crawled: 0,
            successful: 0,
            failed: 0,
            total_data_captured_mb: 0.0,
            avg_response_time_ms: 0.0,
            urls_per_second: 0.0,
        };

        // AI stats
        let ai_stats = AIStats {
            patterns_learned: 0,
            domains_analyzed: 0,
            predictions_made: 0,
            accuracy: 0.0,
        };

        // Recent searches
        let recent_searches: Vec<RecentSearch> = self
            .search_history
            .iter()
            .flat_map(|entry| entry.value().clone())
            .collect();

        SystemStats {
            web_search_stats,
            scraping_stats,
            ai_stats,
            recent_searches,
            total_uptime: self.start_time.elapsed().unwrap_or(Duration::ZERO),
        }
    }

    /// Obtiene últimas búsquedas
    pub fn get_recent_searches(&self, limit: usize) -> Vec<RecentSearch> {
        let mut searches: Vec<RecentSearch> = self
            .search_history
            .iter()
            .flat_map(|entry| entry.value().clone())
            .collect();

        searches.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        searches.truncate(limit);

        searches
    }

    /// Obtiene todas las estadísticas como JSON
    pub fn get_all_stats(&self) -> serde_json::Value {
        let full = self.get_full_stats();
        serde_json::json!({
            "web_search": {
                "total_searches": full.web_search_stats.total_searches,
                "successful": full.web_search_stats.successful_searches,
                "failed": full.web_search_stats.failed_searches,
                "avg_results": full.web_search_stats.avg_results_per_search,
                "total_results": full.web_search_stats.total_results_found
            },
            "scraping": {
                "urls_crawled": full.scraping_stats.total_urls_crawled,
                "successful": full.scraping_stats.successful,
                "failed": full.scraping_stats.failed,
                "data_mb": full.scraping_stats.total_data_captured_mb,
                "avg_response_ms": full.scraping_stats.avg_response_time_ms,
                "urls_per_sec": full.scraping_stats.urls_per_second
            },
            "ai": {
                "patterns_learned": full.ai_stats.patterns_learned,
                "domains_analyzed": full.ai_stats.domains_analyzed,
                "predictions_made": full.ai_stats.predictions_made,
                "accuracy": full.ai_stats.accuracy
            },
            "uptime_secs": full.total_uptime.as_secs(),
            "recent_searches_count": full.recent_searches.len()
        })
    }
}
