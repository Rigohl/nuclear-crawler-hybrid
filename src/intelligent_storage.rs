//! Almacenamiento Inteligente con SQLite - Guarda resultados en DB avanzada
//!
//! Sistema avanzado de almacenamiento que:
//! - Guarda búsquedas en SQLite (DB avanzada)
//! - Indexa por relevancia y fecha
//! - Permite búsqueda rápida en historial
//! - Optimiza espacio automáticamente
//! - Full-text search

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::task;

/// Entrada de búsqueda guardada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSearch {
    pub id: Option<i64>,
    pub query: String,
    pub timestamp: DateTime<Utc>,
    pub results_count: usize,
    pub results: Vec<SearchResultEntry>,
    pub metadata: HashMap<String, String>,
}

/// Entrada de resultado individual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultEntry {
    pub url: String,
    pub title: String,
    pub description: String,
    pub relevance: f32,
    pub quality_score: f32,
    pub source: String,
}

/// Sistema de almacenamiento inteligente con SQLite
pub struct IntelligentStorage {
    db: Arc<Mutex<Connection>>,
    results_dir: PathBuf,
    history_file: PathBuf,
}

impl IntelligentStorage {
    /// Crea nuevo sistema de almacenamiento con SQLite
    pub fn new(storage_dir: Option<PathBuf>) -> Result<Self> {
        // Detectar project root (donde está Cargo.toml)
        let project_root = std::env::current_dir()
            .ok()
            .and_then(|mut path| {
                loop {
                    if path.join("Cargo.toml").exists() {
                        return Some(path);
                    }
                    if !path.pop() {
                        break;
                    }
                }
                None
            })
            .unwrap_or_else(|| PathBuf::from("."));

        let results_dir = storage_dir.unwrap_or_else(|| project_root.join("resultados"));
        std::fs::create_dir_all(&results_dir).context("Error creando directorio resultados")?;

        let db_path = results_dir.join("nuclear_results.db");
        let conn = Connection::open(&db_path).context("Error abriendo conexión SQLite")?;

        // Crear tablas
        conn.execute(
            "CREATE TABLE IF NOT EXISTS searches (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                query TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                results_count INTEGER NOT NULL,
                metadata TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS search_results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                search_id INTEGER NOT NULL,
                url TEXT NOT NULL,
                title TEXT,
                description TEXT,
                relevance REAL,
                quality_score REAL,
                source TEXT,
                FOREIGN KEY(search_id) REFERENCES searches(id)
            )",
            [],
        )?;

        // Crear índice FTS5 para búsqueda full-text
        conn.execute(
            "CREATE VIRTUAL TABLE IF NOT EXISTS searches_fts USING fts5(
                query,
                content='searches',
                content_rowid='id'
            )",
            [],
        )?;

        // Crear índices para optimización
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_search_results_search_id ON search_results(search_id)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_search_results_relevance ON search_results(relevance DESC)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_searches_timestamp ON searches(timestamp DESC)",
            [],
        )?;

        let history_file = results_dir.join("urls_visited.txt");

        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
            results_dir,
            history_file,
        })
    }

    /// Guarda una búsqueda con sus resultados
    pub async fn save_search(
        &self,
        query: &str,
        results: Vec<SearchResultEntry>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<i64> {
        let db = self.db.clone();
        let query_str = query.to_string();
        let timestamp = Utc::now().to_rfc3339();
        let results_count = results.len();
        let metadata_json = metadata
            .map(|m| serde_json::to_string(&m).unwrap_or_default())
            .unwrap_or_default();
        let results_clone = results.clone();

        let search_id = task::spawn_blocking(move || {
            let conn = db.lock().unwrap();
            conn.execute(
                "INSERT INTO searches (query, timestamp, results_count, metadata) VALUES (?1, ?2, ?3, ?4)",
                params![query_str, timestamp, results_count, metadata_json],
            )?;

            let search_id = conn.last_insert_rowid();

            // Insertar en FTS5
            conn.execute(
                "INSERT INTO searches_fts(rowid, query) VALUES (?1, ?2)",
                params![search_id, query_str],
            )?;

            // Insertar resultados
            for result in results_clone {
                conn.execute(
                    "INSERT INTO search_results (search_id, url, title, description, relevance, quality_score, source) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![
                        search_id,
                        result.url,
                        result.title,
                        result.description,
                        result.relevance,
                        result.quality_score,
                        result.source
                    ],
                )?;
            }

            Ok::<i64, rusqlite::Error>(search_id)
        })
        .await??;

        // Guardar URLs en historial
        for result in results {
            self.append_urls_to_history(&result.url).await?;
        }

        Ok(search_id)
    }

    /// Obtiene las últimas búsquedas
    pub fn get_recent_searches(&self, limit: usize) -> Result<Vec<SavedSearch>> {
        let conn = self.db.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, query, timestamp, results_count, metadata FROM searches ORDER BY timestamp DESC LIMIT ?1",
        )?;

        let searches_iter = stmt.query_map([limit], |row| {
            let id: i64 = row.get(0)?;
            let query: String = row.get(1)?;
            let timestamp_str: String = row.get(2)?;
            let results_count: usize = row.get(3)?;
            let metadata_json: String = row.get(4)?;

            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            let metadata: HashMap<String, String> = serde_json::from_str(&metadata_json)
                .unwrap_or_default();

            // Obtener resultados
            let mut results_stmt = conn.prepare(
                "SELECT url, title, description, relevance, quality_score, source FROM search_results WHERE search_id = ?1",
            )?;

            let results: Result<Vec<SearchResultEntry>, rusqlite::Error> = results_stmt
                .query_map([id], |row| {
                    Ok(SearchResultEntry {
                        url: row.get(0)?,
                        title: row.get(1)?,
                        description: row.get(2)?,
                        relevance: row.get(3)?,
                        quality_score: row.get(4)?,
                        source: row.get(5)?,
                    })
                })
                .and_then(|iter| iter.collect());

            Ok(SavedSearch {
                id: Some(id),
                query,
                timestamp,
                results_count,
                results: results.unwrap_or_default(),
                metadata,
            })
        })?;

        let mut searches = Vec::new();
        for search in searches_iter {
            searches.push(search?);
        }

        Ok(searches)
    }

    /// Busca en el historial
    pub fn search_history(&self, query: &str, limit: usize) -> Result<Vec<SavedSearch>> {
        let conn = self.db.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT s.id, s.query, s.timestamp, s.results_count, s.metadata 
             FROM searches s
             JOIN searches_fts fts ON s.id = fts.rowid
             WHERE searches_fts MATCH ?1
             ORDER BY s.timestamp DESC
             LIMIT ?2",
        )?;

        let search_query = format!("\"{}\"", query.replace("\"", "\"\""));

        let searches_iter = stmt.query_map(params![search_query, limit], |row| {
            let id: i64 = row.get(0)?;
            let query: String = row.get(1)?;
            let timestamp_str: String = row.get(2)?;
            let results_count: usize = row.get(3)?;
            let metadata_json: String = row.get(4)?;

            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            let metadata: HashMap<String, String> = serde_json::from_str(&metadata_json)
                .unwrap_or_default();

            // Obtener resultados
            let mut results_stmt = conn.prepare(
                "SELECT url, title, description, relevance, quality_score, source FROM search_results WHERE search_id = ?1",
            )?;

            let results: Result<Vec<SearchResultEntry>, rusqlite::Error> = results_stmt
                .query_map([id], |row| {
                    Ok(SearchResultEntry {
                        url: row.get(0)?,
                        title: row.get(1)?,
                        description: row.get(2)?,
                        relevance: row.get(3)?,
                        quality_score: row.get(4)?,
                        source: row.get(5)?,
                    })
                })
                .and_then(|iter| iter.collect());

            Ok(SavedSearch {
                id: Some(id),
                query,
                timestamp,
                results_count,
                results: results.unwrap_or_default(),
                metadata,
            })
        })?;

        let mut searches = Vec::new();
        for search in searches_iter {
            searches.push(search?);
        }

        Ok(searches)
    }

    /// Cuenta el número total de entradas (búsquedas + resultados)
    pub fn count_entries(&self) -> Result<usize> {
        let stats = self.get_stats()?;
        Ok(stats.total_searches + stats.total_results)
    }

    /// Obtiene estadísticas de almacenamiento
    pub fn get_stats(&self) -> Result<StorageStats> {
        let conn = self.db.lock().unwrap();

        let total_searches: i64 =
            conn.query_row("SELECT COUNT(*) FROM searches", [], |row| row.get(0))?;

        let total_results: i64 =
            conn.query_row("SELECT COUNT(*) FROM search_results", [], |row| row.get(0))?;

        let db_size = self
            .results_dir
            .join("nuclear_results.db")
            .metadata()
            .map(|m| m.len())
            .unwrap_or(0);

        let history_size = self.history_file.metadata().map(|m| m.len()).unwrap_or(0);

        Ok(StorageStats {
            total_searches: total_searches as usize,
            total_results: total_results as usize,
            db_size_bytes: db_size,
            history_size_bytes: history_size,
        })
    }

    /// Obtiene historial de URLs visitadas
    pub fn get_url_history(&self, limit: Option<usize>) -> Result<Vec<String>> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(&self.history_file).context("Error abriendo archivo de historial")?;

        let reader = BufReader::new(file);
        let mut urls = Vec::new();

        for line in reader.lines() {
            let line = line?;
            // Ignorar comentarios y líneas vacías
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            // Formato: [timestamp] url
            if let Some(start) = trimmed.find(']') {
                let url = trimmed[start + 1..].trim();
                if !url.is_empty() {
                    urls.push(url.to_string());
                }
            }
        }

        // Revertir para tener las más recientes primero
        urls.reverse();

        if let Some(limit) = limit {
            urls.truncate(limit);
        }

        Ok(urls)
    }

    /// Agrega URL al historial de texto
    pub async fn append_urls_to_history(&self, url: &str) -> Result<()> {
        let history_file = self.history_file.clone();
        let url_str = url.to_string();
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");

        task::spawn_blocking(move || {
            use std::fs::OpenOptions;
            use std::io::Write;

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&history_file)
                .context("Error abriendo archivo de historial")?;

            writeln!(file, "[{}] {}", timestamp, url_str)
                .context("Error escribiendo en historial")?;

            Ok::<(), anyhow::Error>(())
        })
        .await??;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct StorageStats {
    pub total_searches: usize,
    pub total_results: usize,
    pub db_size_bytes: u64,
    pub history_size_bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    async fn create_test_storage() -> (IntelligentStorage, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage = IntelligentStorage::new(Some(temp_dir.path().to_path_buf()))
            .expect("Error creando storage de test");
        (storage, temp_dir)
    }

    #[tokio::test]
    async fn test_storage_creation() {
        let (storage, _temp_dir) = create_test_storage().await;
        let stats = storage.get_stats().unwrap();
        assert_eq!(stats.total_searches, 0);
        assert_eq!(stats.total_results, 0);
    }

    #[tokio::test]
    async fn test_save_and_retrieve_search() {
        let (storage, _temp_dir) = create_test_storage().await;

        let results = vec![SearchResultEntry {
            url: "https://example.com".to_string(),
            title: "Test".to_string(),
            description: "Test description".to_string(),
            relevance: 0.9,
            quality_score: 0.8,
            source: "test".to_string(),
        }];

        let search_id = storage
            .save_search("test query", results.clone(), None)
            .await
            .unwrap();

        assert!(search_id > 0);

        let searches = storage.get_recent_searches(10).unwrap();
        assert_eq!(searches.len(), 1);
        assert_eq!(searches[0].query, "test query");
        assert_eq!(searches[0].results.len(), 1);
    }

    #[tokio::test]
    async fn test_search_history() {
        let (storage, _temp_dir) = create_test_storage().await;

        let results = vec![SearchResultEntry {
            url: "https://example.com".to_string(),
            title: "Test".to_string(),
            description: "Test description".to_string(),
            relevance: 0.9,
            quality_score: 0.8,
            source: "test".to_string(),
        }];

        storage
            .save_search("rust programming", results, None)
            .await
            .unwrap();

        let searches = storage.search_history("rust", 10).unwrap();
        assert!(!searches.is_empty());
    }

    #[tokio::test]
    async fn test_url_history() {
        let (storage, _temp_dir) = create_test_storage().await;

        storage
            .append_urls_to_history("https://example.com")
            .await
            .unwrap();

        let urls = storage.get_url_history(None).unwrap();
        assert!(!urls.is_empty());
        assert!(urls[0].contains("example.com"));
    }

    #[tokio::test]
    async fn test_stats() {
        let (storage, _temp_dir) = create_test_storage().await;

        let results = vec![
            SearchResultEntry {
                url: "https://example1.com".to_string(),
                title: "Test 1".to_string(),
                description: "Desc 1".to_string(),
                relevance: 0.9,
                quality_score: 0.8,
                source: "test".to_string(),
            },
            SearchResultEntry {
                url: "https://example2.com".to_string(),
                title: "Test 2".to_string(),
                description: "Desc 2".to_string(),
                relevance: 0.8,
                quality_score: 0.7,
                source: "test".to_string(),
            },
        ];

        storage
            .save_search("test query", results, None)
            .await
            .unwrap();

        let stats = storage.get_stats().unwrap();
        assert_eq!(stats.total_searches, 1);
        assert_eq!(stats.total_results, 2);
        assert!(stats.db_size_bytes > 0);
    }
}
