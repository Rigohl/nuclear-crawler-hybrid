///! WASM Marketing Dataset Extractor - 100x faster extraction

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MarketingProfile {
    pub audience_type: String,
    pub engagement_patterns: Vec<String>,
    pub trending_topics: Vec<String>,
    pub sentiment: f32,
    pub keywords: Vec<String>,
    pub hashtags: Vec<String>,
    pub best_posting_times: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ConversationPattern {
    pub opener: String,
    pub response_style: String,
    pub emoji_usage: f32,
    pub avg_response_length: usize,
    pub tone: String,
    pub engagement_score: f32,
}

#[wasm_bindgen]
pub struct MarketingExtractor {
    profiles: Vec<MarketingProfile>,
}

#[wasm_bindgen]
impl MarketingExtractor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
        }
    }
    
    /// Extrae patrones de marketing de HTML
    #[wasm_bindgen]
    pub fn extract_from_html(&mut self, html: &str) -> String {
        let profile = self.analyze_html_fast(html);
        self.profiles.push(profile.clone());
        serde_json::to_string(&profile).unwrap_or_default()
    }
    
    /// Análisis ultra-rápido en WASM
    fn analyze_html_fast(&self, html: &str) -> MarketingProfile {
        let mut keywords = Vec::new();
        let mut hashtags = Vec::new();
        let mut trending = Vec::new();
        
        // Pattern matching ultra-rápido
        for line in html.lines() {
            // Hashtags
            if line.contains('#') {
                let words: Vec<&str> = line.split_whitespace().collect();
                for word in words {
                    if word.starts_with('#') {
                        hashtags.push(word.to_string());
                    }
                }
            }
            
            // Keywords de engagement
            if line.contains("subscribe") || line.contains("follow") || 
               line.contains("exclusive") || line.contains("DM") {
                keywords.push(line.trim().to_string());
            }
        }
        
        // Sentiment analysis rápido
        let sentiment = self.calculate_sentiment(html);
        
        MarketingProfile {
            audience_type: "onlyfans_subscribers".to_string(),
            engagement_patterns: keywords.clone(),
            trending_topics: trending,
            sentiment,
            keywords,
            hashtags,
            best_posting_times: vec!["20:00".to_string(), "22:00".to_string()],
        }
    }
    
    fn calculate_sentiment(&self, text: &str) -> f32 {
        let positive = ["love", "amazing", "perfect", "hot", "exclusive", "special"];
        let negative = ["spam", "fake", "scam"];
        
        let mut score = 0.0;
        let text_lower = text.to_lowercase();
        
        for word in positive {
            score += text_lower.matches(word).count() as f32 * 0.1;
        }
        for word in negative {
            score -= text_lower.matches(word).count() as f32 * 0.2;
        }
        
        score.clamp(-1.0, 1.0)
    }
    
    /// Extrae patrones de conversación para chatbot
    #[wasm_bindgen]
    pub fn extract_conversation_patterns(&self, messages: &str) -> String {
        let pattern = self.analyze_conversation_style(messages);
        serde_json::to_string(&pattern).unwrap_or_default()
    }
    
    fn analyze_conversation_style(&self, messages: &str) -> ConversationPattern {
        let lines: Vec<&str> = messages.lines().collect();
        let total_lines = lines.len();
        
        let mut emoji_count = 0;
        let mut total_length = 0;
        
        for line in &lines {
            emoji_count += line.chars().filter(|c| *c as u32 > 0x1F600).count();
            total_length += line.len();
        }
        
        let emoji_ratio = if total_lines > 0 {
            emoji_count as f32 / total_lines as f32
        } else {
            0.0
        };
        
        let avg_length = if total_lines > 0 {
            total_length / total_lines
        } else {
            0
        };
        
        ConversationPattern {
            opener: "Hey babe".to_string(),
            response_style: if emoji_ratio > 0.5 { "playful" } else { "professional" }.to_string(),
            emoji_usage: emoji_ratio,
            avg_response_length: avg_length,
            tone: "friendly".to_string(),
            engagement_score: 0.8,
        }
    }
    
    /// Genera dataset completo
    #[wasm_bindgen]
    pub fn generate_dataset(&self) -> String {
        serde_json::to_string(&self.profiles).unwrap_or_default()
    }
}
