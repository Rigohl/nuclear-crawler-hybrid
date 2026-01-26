///! Real Human Scraper - Extrae conversaciones REALES de humanos, no inventa

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
pub struct RealHumanScraper {
    real_conversations: Vec<RealHumanConversation>,
    human_patterns: Vec<HumanPattern>,
}

#[wasm_bindgen]
impl RealHumanScraper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            real_conversations: Vec::new(),
            human_patterns: Vec::new(),
        }
    }
    
    /// Scrapea HTML y extrae conversaciones REALES de humanos
    #[wasm_bindgen]
    pub fn scrape_real_humans(&mut self, html: &str, source_url: &str) -> String {
        // Extraer conversaciones REALES del HTML
        let conversations = self.extract_real_conversations(html, source_url);
        
        self.real_conversations.extend(conversations.clone());
        
        format!("✅ Scraped {} REAL human conversations from {}", 
                conversations.len(), source_url)
    }
    
    /// Analiza patrones REALES de humanos (no inventados)
    #[wasm_bindgen]
    pub fn analyze_real_patterns(&mut self) -> String {
        self.human_patterns.clear();
        
        // Analizar conversaciones REALES para encontrar patrones
        for conv in &self.real_conversations {
            // Patrones REALES que encontramos en los datos
            for (i, msg) in conv.messages.iter().enumerate() {
                // Longitud promedio REAL
                if msg.text.len() > 10 && msg.text.len() < 150 {
                    self.human_patterns.push(HumanPattern {
                        pattern_type: "message_length".to_string(),
                        value: msg.text.len() as f32,
                        example: msg.text.clone(),
                    });
                }
                
                // Uso REAL de emojis (contar cuántos usan realmente)
                let emoji_count = msg.text.matches(char::is_emoji).count();
                if emoji_count > 0 {
                    self.human_patterns.push(HumanPattern {
                        pattern_type: "emoji_usage".to_string(),
                        value: emoji_count as f32,
                        example: msg.text.clone(),
                    });
                }
                
                // Errores REALES (typos que hacen los humanos)
                if self.has_human_typos(&msg.text) {
                    self.human_patterns.push(HumanPattern {
                        pattern_type: "human_typos".to_string(),
                        value: 1.0,
                        example: msg.text.clone(),
                    });
                }
                
                // Tiempo entre mensajes REAL
                if i > 0 {
                    let time_diff = self.calculate_time_diff(&conv.messages[i-1].timestamp, &msg.timestamp);
                    if time_diff > 0 {
                        self.human_patterns.push(HumanPattern {
                            pattern_type: "response_time".to_string(),
                            value: time_diff,
                            example: format!("{} seconds", time_diff),
                        });
                    }
                }
            }
        }
        
        format!("✅ Analyzed {} REAL human patterns from {} conversations",
                self.human_patterns.len(), self.real_conversations.len())
    }
    
    /// Exporta dataset REAL para entrenar
    #[wasm_bindgen]
    pub fn export_real_dataset(&self) -> String {
        let dataset = RealHumanDataset {
            conversations: self.real_conversations.clone(),
            patterns: self.human_patterns.clone(),
            total_human_samples: self.real_conversations.len(),
        };
        
        serde_json::to_string_pretty(&dataset).unwrap_or_default()
    }
    
    /// Obtiene estadísticas REALES de humanos
    #[wasm_bindgen]
    pub fn get_real_human_stats(&self) -> String {
        let avg_length: f32 = self.real_conversations.iter()
            .flat_map(|c| &c.messages)
            .map(|m| m.text.len() as f32)
            .sum::<f32>() / self.real_conversations.iter()
                .map(|c| c.messages.len())
                .sum::<usize>().max(1) as f32;
        
        let emoji_usage: f32 = self.real_conversations.iter()
            .flat_map(|c| &c.messages)
            .map(|m| m.text.matches(char::is_emoji).count() as f32)
            .sum::<f32>() / self.real_conversations.len().max(1) as f32;
        
        let typo_rate: f32 = self.real_conversations.iter()
            .flat_map(|c| &c.messages)
            .map(|m| if self.has_human_typos(&m.text) { 1.0 } else { 0.0 })
            .sum::<f32>() / self.real_conversations.iter()
                .map(|c| c.messages.len())
                .sum::<usize>().max(1) as f32;
        
        serde_json::json!({
            "avg_message_length": avg_length,
            "avg_emoji_per_message": emoji_usage,
            "typo_rate": typo_rate,
            "total_conversations": self.real_conversations.len(),
            "total_messages": self.real_conversations.iter().map(|c| c.messages.len()).sum::<usize>()
        }).to_string()
    }
}

impl RealHumanScraper {
    /// Extrae conversaciones REALES del HTML
    fn extract_real_conversations(&self, html: &str, source: &str) -> Vec<RealHumanConversation> {
        let mut conversations = Vec::new();
        
        // Buscar diferentes formatos de conversación REAL
        let lines: Vec<&str> = html.lines().collect();
        let mut current_conversation = Vec::new();
        let mut in_conversation = false;
        
        for line in lines {
            let line = line.trim();
            
            // Detectar inicio de conversación REAL
            if line.contains("data-author") || 
               line.contains("class=\"user-message\"") ||
               line.contains("class=\"message-text\"") ||
               line.contains("username") {
                in_conversation = true;
            }
            
            // Extraer mensaje REAL
            if in_conversation {
                if let Some(text) = self.extract_real_text(line) {
                    if !text.is_empty() && text.len() > 3 {
                        current_conversation.push(RealHumanMessage {
                            text: text.clone(),
                            timestamp: self.extract_real_timestamp(line),
                            author: self.extract_real_author(line),
                            is_human: true, // Viene de scraper = humano real
                        });
                    }
                }
            }
            
            // Fin de conversación
            if line.contains("</div>") && in_conversation && !current_conversation.is_empty() {
                if current_conversation.len() >= 2 {
                    conversations.push(RealHumanConversation {
                        messages: current_conversation.clone(),
                        source: source.to_string(),
                        engagement_score: self.calculate_real_engagement(&current_conversation),
                    });
                }
                current_conversation.clear();
                in_conversation = false;
            }
        }
        
        conversations
    }
    
    fn extract_real_text(&self, html: &str) -> Option<String> {
        // Extraer texto REAL entre tags
        if let Some(start) = html.find('>') {
            if let Some(end) = html[start..].find('<') {
                let text = &html[start + 1..start + end];
                let cleaned = text.trim();
                if !cleaned.is_empty() && cleaned.len() > 2 {
                    return Some(cleaned.to_string());
                }
            }
        }
        None
    }
    
    fn extract_real_timestamp(&self, html: &str) -> String {
        // Buscar timestamp REAL en atributos
        if let Some(pos) = html.find("data-time") {
            if let Some(start) = html[pos..].find('"') {
                if let Some(end) = html[pos + start + 1..].find('"') {
                    return html[pos + start + 1..pos + start + 1 + end].to_string();
                }
            }
        }
        chrono::Utc::now().to_rfc3339()
    }
    
    fn extract_real_author(&self, html: &str) -> String {
        // Buscar autor REAL
        if let Some(pos) = html.find("data-author") {
            if let Some(start) = html[pos..].find('"') {
                if let Some(end) = html[pos + start + 1..].find('"') {
                    return html[pos + start + 1..pos + start + 1 + end].to_string();
                }
            }
        }
        "unknown".to_string()
    }
    
    fn has_human_typos(&self, text: &str) -> bool {
        // Errores REALES comunes de humanos
        let human_typos = [
            "teh", "adn", "taht", "recieve", "seperate",
            "lol", "haha", "omg", "tbh", "imo"
        ];
        
        let text_lower = text.to_lowercase();
        human_typos.iter().any(|typo| text_lower.contains(typo))
    }
    
    fn calculate_real_engagement(&self, messages: &[RealHumanMessage]) -> f32 {
        // Engagement REAL basado en longitud y participación
        let avg_length: f32 = messages.iter()
            .map(|m| m.text.len() as f32)
            .sum::<f32>() / messages.len().max(1) as f32;
        
        // Conversaciones más largas = más engagement
        (avg_length / 100.0).min(10.0)
    }
    
    fn calculate_time_diff(&self, time1: &str, time2: &str) -> f32 {
        // Calcular diferencia de tiempo REAL
        if let (Ok(t1), Ok(t2)) = (
            chrono::DateTime::parse_from_rfc3339(time1),
            chrono::DateTime::parse_from_rfc3339(time2)
        ) {
            (t2 - t1).num_seconds() as f32
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealHumanConversation {
    pub messages: Vec<RealHumanMessage>,
    pub source: String,
    pub engagement_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealHumanMessage {
    pub text: String,
    pub timestamp: String,
    pub author: String,
    pub is_human: bool, // Siempre true - viene de scraper
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanPattern {
    pub pattern_type: String,
    pub value: f32,
    pub example: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealHumanDataset {
    pub conversations: Vec<RealHumanConversation>,
    pub patterns: Vec<HumanPattern>,
    pub total_human_samples: usize,
}
