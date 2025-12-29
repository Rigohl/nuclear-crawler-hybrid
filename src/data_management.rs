use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Data Management System - Gestión inteligente de información extraída
/// Sistema completo para organizar, indexar y analizar datos de búsquedas

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub tool: String,
    pub query: String,
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub tags: Vec<String>,
    pub category: String,
    pub relevance_score: f32,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataIndex {
    pub results: Vec<SearchResult>,
    pub categories: HashMap<String, Vec<String>>, // category -> [result_ids]
    pub tags: HashMap<String, Vec<String>>,       // tag -> [result_ids]
    pub sources: HashMap<String, Vec<String>>,    // source -> [result_ids]
    pub urls_seen: HashSet<String>,               // deduplication
    pub last_updated: DateTime<Utc>,
}

impl Default for DataIndex {
    fn default() -> Self {
        Self {
            results: Vec::new(),
            categories: HashMap::new(),
            tags: HashMap::new(),
            sources: HashMap::new(),
            urls_seen: HashSet::new(),
            last_updated: Utc::now(),
        }
    }
}

impl DataIndex {
    /// Crear nuevo índice
    pub fn new() -> Self {
        Self::default()
    }

    /// Agregar resultado con deduplicación
    pub fn add_result(&mut self, mut result: SearchResult) -> Result<bool> {
        if self.urls_seen.contains(&result.url) {
            return Ok(false); // Duplicado, no agregado
        }

        result.id = format!("{}-{}", result.tool, uuid::Uuid::new_v4());

        // Actualizar índices
        self.urls_seen.insert(result.url.clone());

        // Categorías
        self.categories
            .entry(result.category.clone())
            .or_insert_with(Vec::new)
            .push(result.id.clone());

        // Tags
        for tag in &result.tags {
            self.tags
                .entry(tag.clone())
                .or_insert_with(Vec::new)
                .push(result.id.clone());
        }

        // Fuentes
        self.sources
            .entry(result.source.clone())
            .or_insert_with(Vec::new)
            .push(result.id.clone());

        self.results.push(result);
        self.last_updated = Utc::now();

        Ok(true)
    }

    /// Buscar por palabra clave
    pub fn search(&self, keyword: &str) -> Vec<&SearchResult> {
        self.results
            .iter()
            .filter(|r| {
                r.title.to_lowercase().contains(&keyword.to_lowercase())
                    || r.snippet.to_lowercase().contains(&keyword.to_lowercase())
                    || r.tags.iter().any(|t| t.to_lowercase().contains(&keyword.to_lowercase()))
            })
            .collect()
    }

    /// Buscar por categoría
    pub fn search_by_category(&self, category: &str) -> Vec<&SearchResult> {
        if let Some(ids) = self.categories.get(category) {
            self.results
                .iter()
                .filter(|r| ids.contains(&r.id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Buscar por tag
    pub fn search_by_tag(&self, tag: &str) -> Vec<&SearchResult> {
        if let Some(ids) = self.tags.get(tag) {
            self.results
                .iter()
                .filter(|r| ids.contains(&r.id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Buscar por fuente
    pub fn search_by_source(&self, source: &str) -> Vec<&SearchResult> {
        if let Some(ids) = self.sources.get(source) {
            self.results
                .iter()
                .filter(|r| ids.contains(&r.id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Obtener estadísticas
    pub fn get_stats(&self) -> DataStats {
        DataStats {
            total_results: self.results.len(),
            total_categories: self.categories.len(),
            total_tags: self.tags.len(),
            total_sources: self.sources.len(),
            total_unique_urls: self.urls_seen.len(),
            tools_used: self.results
                .iter()
                .map(|r| r.tool.clone())
                .collect::<HashSet<_>>()
                .len(),
            avg_relevance: if self.results.is_empty() {
                0.0
            } else {
                self.results.iter().map(|r| r.relevance_score).sum::<f32>() / self.results.len() as f32
            },
        }
    }

    /// Obtener resultados de mayor relevancia
    pub fn get_top_results(&self, limit: usize) -> Vec<&SearchResult> {
        let mut results = self.results.iter().collect::<Vec<_>>();
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        results.into_iter().take(limit).collect()
    }

    /// Guardar índice a archivo
    pub fn save(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Cargar índice desde archivo
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let index = serde_json::from_str(&content)?;
        Ok(index)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataStats {
    pub total_results: usize,
    pub total_categories: usize,
    pub total_tags: usize,
    pub total_sources: usize,
    pub total_unique_urls: usize,
    pub tools_used: usize,
    pub avg_relevance: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataReport {
    pub title: String,
    pub generated_at: DateTime<Utc>,
    pub stats: DataStats,
    pub top_results: Vec<SearchResult>,
    pub category_breakdown: HashMap<String, usize>,
    pub source_breakdown: HashMap<String, usize>,
    pub tag_distribution: HashMap<String, usize>,
}

impl DataReport {
    /// Generar reporte a partir del índice
    pub fn from_index(index: &DataIndex, title: &str) -> Self {
        let mut category_breakdown = HashMap::new();
        for (cat, ids) in &index.categories {
            category_breakdown.insert(cat.clone(), ids.len());
        }

        let mut source_breakdown = HashMap::new();
        for (src, ids) in &index.sources {
            source_breakdown.insert(src.clone(), ids.len());
        }

        let mut tag_distribution = HashMap::new();
        for (tag, ids) in &index.tags {
            tag_distribution.insert(tag.clone(), ids.len());
        }

        let top_results = index
            .get_top_results(10)
            .iter()
            .map(|r| (*r).clone())
            .collect();

        Self {
            title: title.to_string(),
            generated_at: Utc::now(),
            stats: index.get_stats(),
            top_results,
            category_breakdown,
            source_breakdown,
            tag_distribution,
        }
    }

    /// Guardar reporte
    pub fn save(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Generar reporte en formato texto
    pub fn to_text(&self) -> String {
        let mut text = format!("═══════════════════════════════════════════\n");
        text.push_str(&format!("📊 {}\n", self.title));
        text.push_str(&format!("Generated: {}\n", self.generated_at.format("%Y-%m-%d %H:%M:%S")));
        text.push_str("═══════════════════════════════════════════\n\n");

        // Estadísticas
        text.push_str("📈 ESTADÍSTICAS\n");
        text.push_str(&format!("  Total resultados: {}\n", self.stats.total_results));
        text.push_str(&format!("  URLs únicas: {}\n", self.stats.total_unique_urls));
        text.push_str(&format!("  Categorías: {}\n", self.stats.total_categories));
        text.push_str(&format!("  Tags: {}\n", self.stats.total_tags));
        text.push_str(&format!("  Fuentes: {}\n", self.stats.total_sources));
        text.push_str(&format!("  Tools usadas: {}\n", self.stats.tools_used));
        text.push_str(&format!("  Relevancia promedio: {:.2}\n\n", self.stats.avg_relevance));

        // Top resultados
        text.push_str("🏆 TOP RESULTADOS\n");
        for (i, result) in self.top_results.iter().enumerate() {
            text.push_str(&format!("  {}. {} ({})\n", i + 1, result.title, result.relevance_score));
        }
        text.push_str("\n");

        // Categorías
        text.push_str("📁 DISTRIBUCIÓN POR CATEGORÍA\n");
        for (cat, count) in &self.category_breakdown {
            text.push_str(&format!("  {}: {}\n", cat, count));
        }
        text.push_str("\n");

        // Fuentes
        text.push_str("📌 DISTRIBUCIÓN POR FUENTE\n");
        for (src, count) in &self.source_breakdown {
            text.push_str(&format!("  {}: {}\n", src, count));
        }

        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_search() {
        let mut index = DataIndex::new();

        let result = SearchResult {
            id: String::new(),
            tool: "websearch".to_string(),
            query: "AI".to_string(),
            title: "Artificial Intelligence".to_string(),
            url: "https://example.com".to_string(),
            snippet: "AI is amazing".to_string(),
            source: "example.com".to_string(),
            timestamp: Utc::now(),
            tags: vec!["AI".to_string(), "tech".to_string()],
            category: "Technology".to_string(),
            relevance_score: 0.95,
            metadata: HashMap::new(),
        };

        assert!(index.add_result(result).unwrap());
        assert_eq!(index.results.len(), 1);
    }
}
