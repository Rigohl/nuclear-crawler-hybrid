///! WASM Dataset Extractor - Extrae datos de web para training

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MarketingDataPoint {
    pub text: String,
    pub category: String,
    pub engagement_score: f32,
    pub keywords: Vec<String>,
    pub sentiment: f32,
}

#[wasm_bindgen]
pub struct DatasetExtractor {
    data_points: Vec<MarketingDataPoint>,
}

#[wasm_bindgen]
impl DatasetExtractor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { data_points: Vec::new() }
    }
    
    /// Extrae data de HTML para training
    #[wasm_bindgen]
    pub fn extract_marketing_data(&mut self, html: &str, category: &str) -> usize {
        let text_blocks = self.extract_text_blocks(html);
        
        for text in text_blocks {
            if text.len() > 20 {
                let keywords = self.extract_keywords(&text);
                let sentiment = self.calculate_sentiment(&text);
                let engagement = self.estimate_engagement(&text);
                
                self.data_points.push(MarketingDataPoint {
                    text,
                    category: category.to_string(),
                    engagement_score: engagement,
                    keywords,
                    sentiment,
                });
            }
        }
        
        self.data_points.len()
    }
    
    fn extract_text_blocks(&self, html: &str) -> Vec<String> {
        let mut blocks = Vec::new();
        let mut current_block = String::new();
        let mut in_tag = false;
        
        for ch in html.chars() {
            match ch {
                '<' => in_tag = true,
                '>' => {
                    in_tag = false;
                    if !current_block.trim().is_empty() {
                        blocks.push(current_block.trim().to_string());
                        current_block.clear();
                    }
                }
                _ if !in_tag => current_block.push(ch),
                _ => {}
            }
        }
        
        blocks
    }
    
    fn extract_keywords(&self, text: &str) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        words.iter()
            .filter(|w| w.len() > 4)
            .take(5)
            .map(|s| s.to_string())
            .collect()
    }
    
    fn calculate_sentiment(&self, text: &str) -> f32 {
        let positive = ["love", "amazing", "exclusive", "special", "hot", "beautiful"];
        let negative = ["spam", "fake", "bad"];
        
        let mut score = 0.0;
        let lower = text.to_lowercase();
        
        for word in positive {
            score += lower.matches(word).count() as f32 * 0.15;
        }
        for word in negative {
            score -= lower.matches(word).count() as f32 * 0.3;
        }
        
        score.clamp(-1.0, 1.0)
    }
    
    fn estimate_engagement(&self, text: &str) -> f32 {
        let engagement_words = ["click", "subscribe", "follow", "exclusive", "dm", "message"];
        let mut score = 0.5;
        
        let lower = text.to_lowercase();
        for word in engagement_words {
            if lower.contains(word) {
                score += 0.1;
            }
        }
        
        score.clamp(0.0, 1.0)
    }
    
    /// Genera dataset JSON completo
    #[wasm_bindgen]
    pub fn export_dataset(&self) -> String {
        serde_json::to_string(&self.data_points).unwrap_or_default()
    }
    
    /// Limpia el dataset actual
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.data_points.clear();
    }
    
    /// Cuenta total de data points
    #[wasm_bindgen]
    pub fn count(&self) -> usize {
        self.data_points.len()
    }
}
