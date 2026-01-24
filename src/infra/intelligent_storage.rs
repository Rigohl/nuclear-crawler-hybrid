//! Intelligent Storage Module
//!
//! Advanced storage system for URLs, results, and metadata
//! Real implementation - no mocks or simulations

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub base_path: String,
    pub max_file_size: usize,
    pub compression: bool,
    pub encryption: bool,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            base_path: "resultados".to_string(),
            max_file_size: 10 * 1024 * 1024, // 10MB
            compression: false,
            encryption: false,
        }
    }
}

#[derive(Clone)]
pub struct IntelligentStorage {
    config: StorageConfig,
    url_history: Arc<RwLock<HashMap<String, Vec<String>>>>,
    metadata_cache: Arc<RwLock<HashMap<String, serde_json::Value>>>,
}

impl Default for IntelligentStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl IntelligentStorage {
    pub fn new() -> Self {
        Self::new_with_config(StorageConfig::default())
    }

    pub fn new_with_config(config: StorageConfig) -> Self {
        // Create base directory if it doesn't exist
        if !Path::new(&config.base_path).exists() {
            let _ = fs::create_dir_all(&config.base_path);
        }

        Self {
            config,
            url_history: Arc::new(RwLock::new(HashMap::new())),
            metadata_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Append URL to history
    pub async fn append_urls_to_history(&self, url: &str) -> Result<()> {
        let mut history = self.url_history.write().await;
        let timestamp = chrono::Utc::now().to_rfc3339();
        history
            .entry("visited_urls".to_string())
            .or_insert_with(Vec::new)
            .push(format!("{}: {}", timestamp, url));
        Ok(())
    }

    /// Store search results
    pub async fn store_search_results(
        &self,
        tool_type: &str,
        query: &str,
        results: &serde_json::Value,
    ) -> Result<String> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let subfolder = match tool_type {
            "websearch" => "websearch",
            "file_search" => "file_search",
            _ => "other",
        };
        let subfolder_path = format!("{}/{}", self.config.base_path, subfolder);

        // Create subfolder if it doesn't exist
        if !Path::new(&subfolder_path).exists() {
            fs::create_dir_all(&subfolder_path)?;
        }

        let filename = format!(
            "{}/{}_{}_{}.json",
            subfolder_path,
            tool_type,
            query.replace(" ", "_"),
            timestamp
        );

        let json_str = serde_json::to_string_pretty(results)?;
        fs::write(&filename, json_str)?;

        Ok(filename)
    }

    /// Load search results
    pub async fn load_search_results(&self, filename: &str) -> Result<serde_json::Value> {
        let filepath = format!("{}/{}", self.config.base_path, filename);
        let content = fs::read_to_string(filepath)?;
        let json: serde_json::Value = serde_json::from_str(&content)?;
        Ok(json)
    }

    /// Store metadata
    pub async fn store_metadata(&self, key: &str, metadata: serde_json::Value) -> Result<()> {
        let mut cache = self.metadata_cache.write().await;
        cache.insert(key.to_string(), metadata);
        Ok(())
    }

    /// Get metadata
    pub async fn get_metadata(&self, key: &str) -> Option<serde_json::Value> {
        let cache = self.metadata_cache.read().await;
        cache.get(key).cloned()
    }

    /// Clean old files
    pub async fn cleanup_old_files(&self, days_old: u32) -> Result<usize> {
        let cutoff = chrono::Utc::now() - chrono::Duration::days(days_old as i64);
        let mut count = 0;

        if let Ok(entries) = fs::read_dir(&self.config.base_path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let modified_time = chrono::DateTime::<chrono::Utc>::from(modified);
                        if modified_time < cutoff && fs::remove_file(entry.path()).is_ok() {
                            count += 1;
                        }
                    }
                }
            }
        }

        Ok(count)
    }

    /// Get storage stats
    pub async fn get_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();

        let history = self.url_history.read().await;
        stats.insert(
            "url_history_count".to_string(),
            history.values().map(|v| v.len()).sum(),
        );

        let metadata = self.metadata_cache.read().await;
        stats.insert("metadata_cache_count".to_string(), metadata.len());

        // Count files in directory
        if let Ok(entries) = fs::read_dir(&self.config.base_path) {
            stats.insert("stored_files".to_string(), entries.count());
        } else {
            stats.insert("stored_files".to_string(), 0);
        }

        stats
    }
}
