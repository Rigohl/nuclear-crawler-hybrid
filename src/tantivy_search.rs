//! Tantivy Search Engine Integration
//!
//! REAL full-text search implementation using Tantivy
//! NO MOCKS, NO FALLBACKS - Pure Rust search engine
//!
//! Provides high-performance indexing and searching for web content

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexWriter, ReloadPolicy, TantivyDocument};
use tokio::sync::RwLock;

/// Document to be indexed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocument {
    pub url: String,
    pub title: String,
    pub content: String,
    pub timestamp: i64,
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub snippet: String,
    pub score: f32,
}

/// Tantivy search engine
pub struct TantivySearchEngine {
    index: Index,
    schema: Schema,
    writer: Arc<RwLock<IndexWriter>>,
    url_field: Field,
    title_field: Field,
    content_field: Field,
    timestamp_field: Field,
}

impl TantivySearchEngine {
    /// Create a new search engine with in-memory index
    pub fn new_in_memory() -> Result<Self> {
        let mut schema_builder = Schema::builder();

        let url_field = schema_builder.add_text_field("url", STRING | STORED);
        let title_field = schema_builder.add_text_field("title", TEXT | STORED);
        let content_field = schema_builder.add_text_field("content", TEXT);
        let timestamp_field = schema_builder.add_i64_field("timestamp", STORED | INDEXED);

        let schema = schema_builder.build();
        let index = Index::create_in_ram(schema.clone());

        let writer = index
            .writer(50_000_000)
            .context("Failed to create index writer")?;

        Ok(Self {
            index,
            schema,
            writer: Arc::new(RwLock::new(writer)),
            url_field,
            title_field,
            content_field,
            timestamp_field,
        })
    }

    /// Create a new search engine with persistent index
    pub fn new_with_directory(index_path: PathBuf) -> Result<Self> {
        let mut schema_builder = Schema::builder();

        let url_field = schema_builder.add_text_field("url", STRING | STORED);
        let title_field = schema_builder.add_text_field("title", TEXT | STORED);
        let content_field = schema_builder.add_text_field("content", TEXT);
        let timestamp_field = schema_builder.add_i64_field("timestamp", STORED | INDEXED);

        let schema = schema_builder.build();

        // Create directory if it doesn't exist
        std::fs::create_dir_all(&index_path)?;

        let index = Index::create_in_dir(&index_path, schema.clone())
            .or_else(|_| Index::open_in_dir(&index_path))
            .context("Failed to create or open index")?;

        let writer = index
            .writer(50_000_000)
            .context("Failed to create index writer")?;

        Ok(Self {
            index,
            schema,
            writer: Arc::new(RwLock::new(writer)),
            url_field,
            title_field,
            content_field,
            timestamp_field,
        })
    }

    /// Add a document to the index
    pub async fn add_document(&self, doc: SearchDocument) -> Result<()> {
        let mut writer = self.writer.write().await;

        let tantivy_doc = doc!(
            self.url_field => doc.url,
            self.title_field => doc.title,
            self.content_field => doc.content,
            self.timestamp_field => doc.timestamp,
        );

        writer.add_document(tantivy_doc)?;
        Ok(())
    }

    /// Add multiple documents to the index
    pub async fn add_documents(&self, docs: Vec<SearchDocument>) -> Result<()> {
        let mut writer = self.writer.write().await;

        for doc in docs {
            let tantivy_doc = doc!(
                self.url_field => doc.url,
                self.title_field => doc.title,
                self.content_field => doc.content,
                self.timestamp_field => doc.timestamp,
            );

            writer.add_document(tantivy_doc)?;
        }

        Ok(())
    }

    /// Commit all pending documents
    pub async fn commit(&self) -> Result<()> {
        let mut writer = self.writer.write().await;
        writer.commit()?;
        Ok(())
    }

    /// Search the index
    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // Commit any pending documents first
        self.commit().await?;

        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .context("Failed to create index reader")?;

        let searcher = reader.searcher();

        // Parse query
        let query_parser =
            QueryParser::for_index(&self.index, vec![self.title_field, self.content_field]);

        let query = query_parser
            .parse_query(query)
            .context("Failed to parse query")?;

        // Search
        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(limit))
            .context("Failed to execute search")?;

        // Extract results
        let mut results = Vec::new();
        for (_score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher
                .doc(doc_address)
                .context("Failed to retrieve document")?;

            let url = retrieved_doc
                .get_first(self.url_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let title = retrieved_doc
                .get_first(self.title_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            // Extract snippet from content (first 200 chars)
            let content = retrieved_doc
                .get_first(self.content_field)
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let snippet = if content.len() > 200 {
                format!("{}...", &content[..200])
            } else {
                content.to_string()
            };

            results.push(SearchResult {
                url,
                title,
                snippet,
                score: _score,
            });
        }

        Ok(results)
    }

    /// Get document count
    pub async fn document_count(&self) -> Result<u64> {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .context("Failed to create index reader")?;

        let searcher = reader.searcher();
        Ok(searcher.num_docs())
    }

    /// Clear all documents from the index
    pub async fn clear(&self) -> Result<()> {
        let mut writer = self.writer.write().await;
        writer.delete_all_documents()?;
        writer.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_engine_creation() {
        let engine = TantivySearchEngine::new_in_memory();
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_add_and_search_document() {
        let engine = TantivySearchEngine::new_in_memory().unwrap();

        let doc = SearchDocument {
            url: "https://example.com".to_string(),
            title: "Example Title".to_string(),
            content: "This is example content about Rust programming".to_string(),
            timestamp: 1234567890,
        };

        engine.add_document(doc).await.unwrap();
        engine.commit().await.unwrap();

        let results = engine.search("Rust", 10).await.unwrap();
        assert!(!results.is_empty());
        assert!(results[0].title.contains("Example"));
    }

    #[tokio::test]
    async fn test_multiple_documents() {
        let engine = TantivySearchEngine::new_in_memory().unwrap();

        let docs = vec![
            SearchDocument {
                url: "https://example.com/1".to_string(),
                title: "Rust Programming".to_string(),
                content: "Learn Rust programming language".to_string(),
                timestamp: 1234567890,
            },
            SearchDocument {
                url: "https://example.com/2".to_string(),
                title: "Python Tutorial".to_string(),
                content: "Introduction to Python".to_string(),
                timestamp: 1234567891,
            },
            SearchDocument {
                url: "https://example.com/3".to_string(),
                title: "Advanced Rust".to_string(),
                content: "Advanced concepts in Rust".to_string(),
                timestamp: 1234567892,
            },
        ];

        engine.add_documents(docs).await.unwrap();
        engine.commit().await.unwrap();

        // Search for Rust documents
        let results = engine.search("Rust", 10).await.unwrap();
        assert_eq!(results.len(), 2);

        // Search for Python documents
        let results = engine.search("Python", 10).await.unwrap();
        assert_eq!(results.len(), 1);

        // Check document count
        let count = engine.document_count().await.unwrap();
        assert_eq!(count, 3);
    }

    #[tokio::test]
    async fn test_clear_index() {
        let engine = TantivySearchEngine::new_in_memory().unwrap();

        let doc = SearchDocument {
            url: "https://example.com".to_string(),
            title: "Test".to_string(),
            content: "Test content".to_string(),
            timestamp: 1234567890,
        };

        engine.add_document(doc).await.unwrap();
        engine.commit().await.unwrap();

        assert_eq!(engine.document_count().await.unwrap(), 1);

        engine.clear().await.unwrap();
        assert_eq!(engine.document_count().await.unwrap(), 0);
    }
}
