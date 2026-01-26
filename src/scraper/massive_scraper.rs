///! Massive Scraper - Genera datasets masivos para entrenar bots

use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use tokio::time::{sleep, Duration};

pub struct MassiveScraper {
    visited_urls: HashSet<String>,
    scraped_conversations: Vec<ScrapedConversation>,
    rate_limit_delay: Duration,
}

impl MassiveScraper {
    pub fn new() -> Self {
        Self {
            visited_urls: HashSet::new(),
            scraped_conversations: Vec::new(),
            rate_limit_delay: Duration::from_millis(500), // 2 requests/segundo
        }
    }
    
    /// Scrapea masivamente para crear dataset
    pub async fn scrape_massive_dataset(
        &mut self,
        seed_urls: Vec<String>,
        max_pages: usize,
    ) -> Result<Vec<ScrapedConversation>, Box<dyn std::error::Error>> {
        println!("🕷️  MASSIVE SCRAPER - Starting dataset generation");
        println!("   Target: {} pages", max_pages);
        println!();
        
        let mut queue = seed_urls;
        let mut scraped = 0;
        
        while !queue.is_empty() && scraped < max_pages {
            let url = queue.remove(0);
            
            if self.visited_urls.contains(&url) {
                continue;
            }
            
            println!("📡 Scraping: {} ({}/{})", url, scraped + 1, max_pages);
            
            // Scrapear página
            match self.scrape_page(&url).await {
                Ok(conversations) => {
                    self.scraped_conversations.extend(conversations.clone());
                    scraped += 1;
                    
                    println!("   ✅ Found {} conversations (Total: {})", 
                            conversations.len(), 
                            self.scraped_conversations.len());
                }
                Err(e) => {
                    println!("   ⚠️  Error: {}", e);
                }
            }
            
            self.visited_urls.insert(url);
            
            // Rate limiting
            sleep(self.rate_limit_delay).await;
        }
        
        println!();
        println!("✅ Dataset generation complete!");
        println!("   Total conversations: {}", self.scraped_conversations.len());
        println!("   Total pages scraped: {}", scraped);
        
        Ok(self.scraped_conversations.clone())
    }
    
    async fn scrape_page(&self, url: &str) -> Result<Vec<ScrapedConversation>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()?;
        
        let response = client.get(url).send().await?;
        let html = response.text().await?;
        
        // Extraer conversaciones del HTML
        let conversations = self.extract_conversations(&html, url);
        
        Ok(conversations)
    }
    
    fn extract_conversations(&self, html: &str, source: &str) -> Vec<ScrapedConversation> {
        let mut conversations = Vec::new();
        
        // Detectar diferentes formatos de conversación
        let patterns = [
            ("message", "class=\"message\""),
            ("comment", "class=\"comment\""),
            ("chat", "class=\"chat\""),
            ("conversation", "class=\"conversation\""),
        ];
        
        for (_, pattern) in &patterns {
            if html.contains(pattern) {
                // Extraer bloques de conversación
                let blocks: Vec<&str> = html.split(pattern).collect();
                
                for block in blocks.iter().skip(1).take(10) {
                    if let Some(conversation) = self.parse_conversation_block(block, source) {
                        conversations.push(conversation);
                    }
                }
            }
        }
        
        // Si no encuentra patrones específicos, buscar texto general
        if conversations.is_empty() {
            conversations.extend(self.extract_general_text(html, source));
        }
        
        conversations
    }
    
    fn parse_conversation_block(&self, block: &str, source: &str) -> Option<ScrapedConversation> {
        let mut messages = Vec::new();
        
        // Extraer mensajes del bloque
        let lines: Vec<&str> = block.lines().collect();
        
        for line in lines.iter().take(20) {
            let text = self.extract_text_from_line(line);
            if !text.is_empty() && text.len() > 5 {
                messages.push(ScrapedMessage {
                    text,
                    likes: self.extract_number(line, "like"),
                    replies: self.extract_number(line, "reply"),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                });
            }
        }
        
        if messages.len() >= 2 {
            Some(ScrapedConversation {
                messages,
                source: source.to_string(),
                engagement_score: self.calculate_engagement(&messages),
                context: self.detect_context(&messages),
            })
        } else {
            None
        }
    }
    
    fn extract_general_text(&self, html: &str, source: &str) -> Vec<ScrapedConversation> {
        let mut conversations = Vec::new();
        
        // Buscar párrafos que parezcan conversaciones
        let paragraphs: Vec<&str> = html.split("<p>").collect();
        let mut current_msgs = Vec::new();
        
        for para in paragraphs.iter().take(50) {
            let text = self.extract_text_from_line(para);
            if text.len() > 10 && text.len() < 200 {
                current_msgs.push(ScrapedMessage {
                    text,
                    likes: 0,
                    replies: 0,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                });
                
                if current_msgs.len() >= 3 {
                    conversations.push(ScrapedConversation {
                        messages: current_msgs.clone(),
                        source: source.to_string(),
                        engagement_score: 5.0,
                        context: "general".to_string(),
                    });
                    current_msgs.clear();
                }
            }
        }
        
        conversations
    }
    
    fn extract_text_from_line(&self, line: &str) -> String {
        // Extraer texto entre tags
        let mut text = String::new();
        let mut in_tag = false;
        
        for ch in line.chars() {
            if ch == '<' {
                in_tag = true;
            } else if ch == '>' {
                in_tag = false;
            } else if !in_tag {
                text.push(ch);
            }
        }
        
        text.trim().to_string()
    }
    
    fn extract_number(&self, text: &str, keyword: &str) -> u32 {
        // Buscar números cerca de keywords
        if let Some(pos) = text.to_lowercase().find(keyword) {
            let after = &text[pos..];
            for word in after.split_whitespace() {
                if let Ok(num) = word.parse::<u32>() {
                    return num;
                }
            }
        }
        0
    }
    
    fn calculate_engagement(&self, messages: &[ScrapedMessage]) -> f32 {
        let total_likes: u32 = messages.iter().map(|m| m.likes).sum();
        let total_replies: u32 = messages.iter().map(|m| m.replies).sum();
        
        (total_likes as f32 * 0.5 + total_replies as f32 * 2.0) / messages.len().max(1) as f32
    }
    
    fn detect_context(&self, messages: &[ScrapedMessage]) -> String {
        let combined = messages.iter()
            .map(|m| m.text.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ");
        
        if combined.contains("hey") || combined.contains("hi") {
            "greeting".to_string()
        } else if combined.contains("how") || combined.contains("what") {
            "question".to_string()
        } else {
            "general".to_string()
        }
    }
    
    /// Exporta dataset para entrenamiento
    pub fn export_dataset(&self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.scraped_conversations)?;
        std::fs::write(output_path, json)?;
        println!("💾 Dataset exported to: {}", output_path);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedConversation {
    pub messages: Vec<ScrapedMessage>,
    pub source: String,
    pub engagement_score: f32,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedMessage {
    pub text: String,
    pub likes: u32,
    pub replies: u32,
    pub timestamp: String,
}
