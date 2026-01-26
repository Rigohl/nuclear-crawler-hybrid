///! MCP Tool: Marketing Dataset Generator

use anyhow::Result;
use serde_json::Value;
use crate::OnlyFansChatbot;
use crate::core::marketing_crawler::MarketingCrawler;

pub struct MarketingTool;

impl MarketingTool {
    pub async fn execute(params: Value) -> Result<Value> {
        let action = params["action"].as_str().unwrap_or("crawl");
        
        match action {
            "crawl" => Self::crawl_social_media(params).await,
            "chat" => Self::chat_with_bot(params).await,
            "generate_dataset" => Self::generate_dataset(params).await,
            _ => Err(anyhow::anyhow!("Unknown action")),
        }
    }
    
    async fn crawl_social_media(params: Value) -> Result<Value> {
        let urls: Vec<String> = params["urls"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("urls required"))?
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();
        
        let crawler = MarketingCrawler::new();
        let data = crawler.crawl_social_media(urls).await?;
        
        Ok(serde_json::to_value(data)?)
    }
    
    async fn chat_with_bot(params: Value) -> Result<Value> {
        let message = params["message"].as_str().ok_or_else(|| anyhow::anyhow!("message required"))?;
        let name = params["name"].as_str().unwrap_or("Chatbot");
        
        let mut chatbot = OnlyFansChatbot::new(name.to_string());
        let response = chatbot.generate_response(message);
        
        Ok(serde_json::json!({
            "response": response,
            "bot_name": name,
        }))
    }
    
    async fn generate_dataset(params: Value) -> Result<Value> {
        let urls: Vec<String> = params["urls"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("urls required"))?
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();
        
        let crawler = MarketingCrawler::new();
        let campaign_data = crawler.crawl_social_media(urls).await?;
        
        // Generar dataset para training
        let dataset = serde_json::json!({
            "type": "marketing_training",
            "total_samples": campaign_data.len(),
            "data": campaign_data,
            "created_at": chrono::Utc::now().to_rfc3339(),
        });
        
        Ok(dataset)
    }
}
