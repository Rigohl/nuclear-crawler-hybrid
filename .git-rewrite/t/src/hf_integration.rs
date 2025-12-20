//! Módulo HuggingFace Integration - Integración con HuggingFace Hub
//!
//! Permite buscar y descargar modelos, datasets y spaces desde HuggingFace
//! para mejorar las capacidades de AI del crawler

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuración de HuggingFace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HfConfig {
    /// Token de API (opcional para repos públicos)
    pub api_token: Option<String>,

    /// Base URL de la API
    pub api_base: String,

    /// Timeout en segundos
    pub timeout_secs: u64,

    /// Cache de modelos local
    pub cache_dir: Option<String>,
}

impl Default for HfConfig {
    fn default() -> Self {
        Self {
            api_token: None,
            api_base: "https://huggingface.co/api".to_string(),
            timeout_secs: 30,
            cache_dir: None,
        }
    }
}

/// Tipo de recurso en HuggingFace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HfResourceType {
    Model,
    Dataset,
    Space,
}

/// Resultado de búsqueda en HuggingFace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HfSearchResult {
    /// ID del recurso (ej: "bert-base-uncased")
    pub id: String,

    /// Tipo de recurso
    pub resource_type: HfResourceType,

    /// Autor/Organización
    pub author: String,

    /// Descripción
    pub description: String,

    /// Downloads
    pub downloads: u64,

    /// Likes
    pub likes: u64,

    /// Tags
    pub tags: Vec<String>,

    /// URL
    pub url: String,

    /// Última actualización
    pub last_modified: String,
}

/// Integración con HuggingFace Hub
pub struct HfIntegration {
    config: HfConfig,
    client: reqwest::Client,
}

impl HfIntegration {
    /// Crea nueva integración con HuggingFace
    pub fn new(config: HfConfig) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();

        // Agregar token si existe
        if let Some(ref token) = config.api_token {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))?,
            );
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()?;

        Ok(Self { config, client })
    }

    /// Busca modelos en HuggingFace
    pub async fn search_models(&self, query: &str, limit: usize) -> Result<Vec<HfSearchResult>> {
        let url = format!(
            "{}/models?search={}&limit={}",
            self.config.api_base,
            urlencoding::encode(query),
            limit
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            // Fallback: devolver resultados vacíos si la API no responde
            return Ok(Vec::new());
        }

        let models: Vec<serde_json::Value> = response.json().await.unwrap_or_default();

        let results = models
            .into_iter()
            .map(|m| HfSearchResult {
                id: m
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                resource_type: HfResourceType::Model,
                author: m
                    .get("author")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                description: m
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                downloads: m.get("downloads").and_then(|v| v.as_u64()).unwrap_or(0),
                likes: m.get("likes").and_then(|v| v.as_u64()).unwrap_or(0),
                tags: m
                    .get("tags")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|t| t.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default(),
                url: format!(
                    "https://huggingface.co/{}",
                    m.get("id").and_then(|v| v.as_str()).unwrap_or("")
                ),
                last_modified: m
                    .get("lastModified")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            })
            .collect();

        Ok(results)
    }

    /// Busca datasets en HuggingFace
    pub async fn search_datasets(&self, query: &str, limit: usize) -> Result<Vec<HfSearchResult>> {
        let url = format!(
            "{}/datasets?search={}&limit={}",
            self.config.api_base,
            urlencoding::encode(query),
            limit
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        let datasets: Vec<serde_json::Value> = response.json().await.unwrap_or_default();

        let results = datasets
            .into_iter()
            .map(|d| HfSearchResult {
                id: d
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                resource_type: HfResourceType::Dataset,
                author: d
                    .get("author")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                description: d
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                downloads: d.get("downloads").and_then(|v| v.as_u64()).unwrap_or(0),
                likes: d.get("likes").and_then(|v| v.as_u64()).unwrap_or(0),
                tags: d
                    .get("tags")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|t| t.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default(),
                url: format!(
                    "https://huggingface.co/datasets/{}",
                    d.get("id").and_then(|v| v.as_str()).unwrap_or("")
                ),
                last_modified: d
                    .get("lastModified")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            })
            .collect();

        Ok(results)
    }

    /// Busca spaces en HuggingFace
    pub async fn search_spaces(&self, query: &str, limit: usize) -> Result<Vec<HfSearchResult>> {
        let url = format!(
            "{}/spaces?search={}&limit={}",
            self.config.api_base,
            urlencoding::encode(query),
            limit
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        let spaces: Vec<serde_json::Value> = response.json().await.unwrap_or_default();

        let results = spaces
            .into_iter()
            .map(|s| {
                HfSearchResult {
                    id: s
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    resource_type: HfResourceType::Space,
                    author: s
                        .get("author")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    description: s
                        .get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    downloads: 0, // Spaces no tienen downloads
                    likes: s.get("likes").and_then(|v| v.as_u64()).unwrap_or(0),
                    tags: s
                        .get("tags")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|t| t.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default(),
                    url: format!(
                        "https://huggingface.co/spaces/{}",
                        s.get("id").and_then(|v| v.as_str()).unwrap_or("")
                    ),
                    last_modified: s
                        .get("lastModified")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                }
            })
            .collect();

        Ok(results)
    }

    /// Búsqueda unificada en todos los recursos
    pub async fn search_all(
        &self,
        query: &str,
        limit_per_type: usize,
    ) -> Result<HashMap<String, Vec<HfSearchResult>>> {
        let mut results = HashMap::new();

        // Buscar en paralelo
        let (models, datasets, spaces) = tokio::join!(
            self.search_models(query, limit_per_type),
            self.search_datasets(query, limit_per_type),
            self.search_spaces(query, limit_per_type)
        );

        results.insert("models".to_string(), models.unwrap_or_default());
        results.insert("datasets".to_string(), datasets.unwrap_or_default());
        results.insert("spaces".to_string(), spaces.unwrap_or_default());

        Ok(results)
    }

    /// Obtiene información detallada de un modelo
    pub async fn get_model_info(&self, model_id: &str) -> Result<Option<HfSearchResult>> {
        let url = format!("{}/models/{}", self.config.api_base, model_id);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Ok(None);
        }

        let m: serde_json::Value = response.json().await?;

        Ok(Some(HfSearchResult {
            id: m
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            resource_type: HfResourceType::Model,
            author: m
                .get("author")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            description: m
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            downloads: m.get("downloads").and_then(|v| v.as_u64()).unwrap_or(0),
            likes: m.get("likes").and_then(|v| v.as_u64()).unwrap_or(0),
            tags: m
                .get("tags")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|t| t.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            url: format!("https://huggingface.co/{}", model_id),
            last_modified: m
                .get("lastModified")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        }))
    }

    /// Genera URLs de búsqueda para HuggingFace (para usar con el crawler)
    pub fn generate_search_urls(&self, query: &str) -> Vec<String> {
        let query_encoded = urlencoding::encode(query);

        vec![
            // Búsqueda principal
            format!("https://huggingface.co/models?search={}", query_encoded),
            format!("https://huggingface.co/datasets?search={}", query_encoded),
            format!("https://huggingface.co/spaces?search={}", query_encoded),
            // Por tareas específicas
            format!(
                "https://huggingface.co/models?pipeline_tag=text-generation&search={}",
                query_encoded
            ),
            format!(
                "https://huggingface.co/models?pipeline_tag=text-classification&search={}",
                query_encoded
            ),
            format!(
                "https://huggingface.co/models?pipeline_tag=token-classification&search={}",
                query_encoded
            ),
            format!(
                "https://huggingface.co/models?pipeline_tag=question-answering&search={}",
                query_encoded
            ),
            format!(
                "https://huggingface.co/models?pipeline_tag=summarization&search={}",
                query_encoded
            ),
            format!(
                "https://huggingface.co/models?pipeline_tag=translation&search={}",
                query_encoded
            ),
            format!(
                "https://huggingface.co/models?pipeline_tag=image-classification&search={}",
                query_encoded
            ),
            format!(
                "https://huggingface.co/models?pipeline_tag=object-detection&search={}",
                query_encoded
            ),
            // Por librería
            format!(
                "https://huggingface.co/models?library=transformers&search={}",
                query_encoded
            ),
            format!(
                "https://huggingface.co/models?library=pytorch&search={}",
                query_encoded
            ),
            format!(
                "https://huggingface.co/models?library=tensorflow&search={}",
                query_encoded
            ),
            format!(
                "https://huggingface.co/models?library=jax&search={}",
                query_encoded
            ),
            format!(
                "https://huggingface.co/models?library=rust&search={}",
                query_encoded
            ),
            // Trending y populares
            format!(
                "https://huggingface.co/models?sort=trending&search={}",
                query_encoded
            ),
            format!(
                "https://huggingface.co/models?sort=downloads&search={}",
                query_encoded
            ),
            format!(
                "https://huggingface.co/models?sort=likes&search={}",
                query_encoded
            ),
        ]
    }
}

impl Default for HfIntegration {
    fn default() -> Self {
        Self::new(HfConfig::default()).unwrap_or_else(|_| Self {
            config: HfConfig::default(),
            client: reqwest::Client::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hf_config_default() {
        let config = HfConfig::default();
        assert!(config.api_token.is_none());
        assert!(!config.api_base.is_empty());
    }

    #[test]
    fn test_generate_search_urls() {
        let hf = HfIntegration::default();
        let urls = hf.generate_search_urls("bert");

        assert!(!urls.is_empty());
        assert!(urls.iter().any(|u| u.contains("models")));
        assert!(urls.iter().any(|u| u.contains("datasets")));
    }
}
