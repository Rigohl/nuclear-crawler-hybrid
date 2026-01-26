///! Google Custom Search Engine - Similar a JAX, bestia de búsqueda

use serde::{Deserialize, Serialize};
use anyhow::Result;

pub struct GoogleSearchEngine {
    api_key: String,
    search_engine_id: String,
    client: reqwest::Client,
}

impl GoogleSearchEngine {
    pub fn new(api_key: String, search_engine_id: String) -> Self {
        Self {
            api_key,
            search_engine_id,
            client: reqwest::Client::new(),
        }
    }
    
    /// Búsqueda potente tipo JAX
    pub async fn search(&self, query: &str, num_results: u32) -> Result<Vec<SearchResult>> {
        let url = format!(
            "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}&num={}",
            self.api_key,
            self.search_engine_id,
            urlencoding::encode(query),
            num_results.min(10) // Google limita a 10 por request
        );
        
        let response = self.client.get(&url).send().await?;
        let search_response: GoogleSearchResponse = response.json().await?;
        
        Ok(search_response.items.unwrap_or_default().into_iter()
            .map(|item| SearchResult {
                title: item.title,
                link: item.link,
                snippet: item.snippet,
                display_link: item.display_link,
            })
            .collect())
    }
    
    /// Búsqueda para entrenar bots (busca conversaciones reales)
    pub async fn search_conversations(&self, topic: &str) -> Result<Vec<String>> {
        let query = format!("{} conversation examples", topic);
        let results = self.search(&query, 10).await?;
        
        Ok(results.into_iter()
            .map(|r| format!("{} {}", r.title, r.snippet))
            .collect())
    }
    
    /// Búsqueda para mejorar respuestas del bot
    pub async fn enhance_response(&self, user_message: &str) -> Result<Vec<String>> {
        // Buscar respuestas similares en la web
        let query = format!("how to respond to '{}'", user_message);
        let results = self.search(&query, 5).await?;
        
        Ok(results.into_iter()
            .map(|r| r.snippet)
            .collect())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub link: String,
    pub snippet: String,
    pub display_link: String,
}

#[derive(Debug, Deserialize)]
struct GoogleSearchResponse {
    items: Option<Vec<GoogleSearchItem>>,
}

#[derive(Debug, Deserialize)]
struct GoogleSearchItem {
    title: String,
    link: String,
    snippet: String,
    #[serde(rename = "displayLink")]
    display_link: String,
}
