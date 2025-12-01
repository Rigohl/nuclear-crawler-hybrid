//! Módulo Massive Parallel Search
//!
//! Sistema que aprende a buscar en MÚLTIPLES lugares simultáneamente
//! Aprovecha que datos REALES son mejores y busca en paralelo masivo

use anyhow::Result;
use dashmap::DashMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[allow(unused_imports)]
use std::time::Duration;
use tokio::sync::Semaphore;

use crate::ai_smart::AISmart;
#[allow(unused_imports)]
use crate::ai_smart::MassiveSearchStrategy;
use crate::nuclear_scraper::NuclearScraper;

/// Resultado de búsqueda masiva
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassiveSearchResult {
    /// Fuente
    pub source: String,

    /// URLs encontradas
    pub urls_found: Vec<String>,

    /// Datos extraídos
    pub data_extracted: serde_json::Value,

    /// Calidad de datos (real = alta, sintético = baja)
    pub data_quality: f32,

    /// Es dato real
    pub is_real_data: bool,

    /// Tiempo de búsqueda
    pub search_time_ms: u64,

    /// Éxito
    pub success: bool,
}

/// Sistema de búsqueda masiva paralela
pub struct MassiveParallelSearch {
    scraper: Arc<NuclearScraper>,
    ai_smart: Arc<AISmart>,
    semaphore: Arc<Semaphore>,
    results: Arc<DashMap<String, MassiveSearchResult>>,
}

impl MassiveParallelSearch {
    /// Crea nuevo sistema de búsqueda masiva
    pub fn new(scraper: Arc<NuclearScraper>, ai_smart: Arc<AISmart>) -> Self {
        Self {
            scraper,
            ai_smart,
            semaphore: Arc::new(Semaphore::new(1000)), // Máximo paralelismo
            results: Arc::new(DashMap::new()),
        }
    }

    /// Busca en MÚLTIPLES fuentes simultáneamente
    pub async fn search_massive_parallel(
        &self,
        sources: Vec<String>,
    ) -> Result<Vec<MassiveSearchResult>> {
        println!("🚀 BÚSQUEDA MASIVA PARALELA");
        println!("   📚 Fuentes: {}", sources.len());
        println!("   ⚡ Paralelismo: Máximo");
        println!("   ✅ Priorizando datos REALES");

        // Obtener estrategia recomendada por AI
        let strategy = self
            .ai_smart
            .recommend_massive_parallel_search(sources.clone());

        println!("   🧠 Estrategia AI:");
        println!("      • Concurrent: {}", strategy.max_concurrent);
        println!("      • Delay: {}ms", strategy.delay_ms);
        println!("      • Batch: {}", strategy.batch_size);
        println!(
            "      • Real data priority: {}",
            strategy.prioritize_real_data
        );

        // Procesar en paralelo masivo
        let results: Vec<Result<MassiveSearchResult>> = sources
            .par_iter()
            .map(|source| {
                let source_clone = source.clone();
                let scraper = self.scraper.clone();
                let _ai_smart = self.ai_smart.clone();
                let semaphore = self.semaphore.clone();

                // Ejecutar en runtime async
                let rt = tokio::runtime::Handle::current();
                rt.block_on(async move {
                    let _permit = semaphore.acquire().await?;
                    let start = std::time::Instant::now();

                    // Buscar en esta fuente
                    match Self::search_single_source(&source_clone, &scraper).await {
                        Ok(data) => {
                            let search_time = start.elapsed();

                            // Determinar calidad (datos reales = mejor)
                            let is_real_data = Self::is_real_data_source(&source_clone);
                            let data_quality = if is_real_data { 0.95 } else { 0.60 };

                            Ok(MassiveSearchResult {
                                source: source_clone,
                                urls_found: data.urls,
                                data_extracted: data.content,
                                data_quality,
                                is_real_data,
                                search_time_ms: search_time.as_millis() as u64,
                                success: true,
                            })
                        }
                        Err(_e) => Ok(MassiveSearchResult {
                            source: source_clone,
                            urls_found: Vec::new(),
                            data_extracted: serde_json::json!({}),
                            data_quality: 0.0,
                            is_real_data: false,
                            search_time_ms: start.elapsed().as_millis() as u64,
                            success: false,
                        }),
                    }
                })
            })
            .collect();

        // Convertir resultados
        let mut final_results = Vec::new();
        for result in results {
            match result {
                Ok(r) => final_results.push(r),
                Err(_) => continue,
            }
        }

        // Filtrar por calidad (priorizar datos reales)
        if strategy.prioritize_real_data {
            final_results.sort_by(|a, b| b.data_quality.partial_cmp(&a.data_quality).unwrap());
        }

        // Guardar resultados
        for result in &final_results {
            self.results.insert(result.source.clone(), result.clone());
        }

        println!(
            "   ✅ Completado: {} fuentes procesadas",
            final_results.len()
        );
        println!(
            "   📊 Datos reales: {}",
            final_results.iter().filter(|r| r.is_real_data).count()
        );

        Ok(final_results)
    }

    /// Busca en una fuente individual
    async fn search_single_source(source: &str, _scraper: &NuclearScraper) -> Result<SourceData> {
        // Simular búsqueda (en producción sería scraping real)
        // Por ahora retornamos datos simulados
        Ok(SourceData {
            urls: vec![format!("{}/result1", source), format!("{}/result2", source)],
            content: serde_json::json!({
                "source": source,
                "type": "real_data",
                "quality": "high",
            }),
        })
    }

    /// Determina si es fuente de datos reales
    fn is_real_data_source(source: &str) -> bool {
        // Fuentes conocidas de datos reales
        let real_sources = [
            "github.com",
            "reddit.com",
            "medium.com",
            "dev.to",
            "huggingface.co",
            "stackoverflow.com",
            "arxiv.org",
            "towardsdatascience.com",
            "kaggle.com",
            "paperswithcode.com",
        ];

        real_sources.iter().any(|&s| source.contains(s))
    }

    /// Obtiene mejores resultados (datos reales primero)
    pub fn get_best_results(&self, limit: usize) -> Vec<MassiveSearchResult> {
        let mut results: Vec<_> = self.results.iter().map(|r| r.clone()).collect();

        // Ordenar por calidad (reales primero)
        results.sort_by(|a, b| b.data_quality.partial_cmp(&a.data_quality).unwrap());

        results.into_iter().take(limit).collect()
    }
}

#[derive(Debug)]
struct SourceData {
    urls: Vec<String>,
    content: serde_json::Value,
}
