///! WASM Ultra-Fast Scraper - Scrapea y extrae info de web para entrenar bots

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
pub struct WasmScraper {
    scraped_data: Vec<ScrapedConversation>,
    patterns: Vec<ConversationPattern>,
}

#[wasm_bindgen]
impl WasmScraper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            scraped_data: Vec::new(),
            patterns: Vec::new(),
        }
    }
    
    /// Scrapea HTML ultra-rápido y extrae conversaciones
    #[wasm_bindgen]
    pub fn scrape_html(&mut self, html: &str, source_url: &str) -> String {
        let conversations = self.extract_conversations_fast(html, source_url);
        
        self.scraped_data.extend(conversations.clone());
        
        format!("✅ Scraped {} conversations from {}", conversations.len(), source_url)
    }
    
    /// Detecta patrones de conversación para entrenar el bot
    #[wasm_bindgen]
    pub fn detect_patterns(&mut self) -> String {
        self.patterns.clear();
        
        // Analizar todos los datos scrapeados
        for conversation in &self.scraped_data {
            // Detectar opening lines efectivos
            if conversation.engagement_score > 7.0 {
                self.patterns.push(ConversationPattern {
                    pattern_type: "opening_line".to_string(),
                    text: conversation.messages.first()
                        .map(|m| m.text.clone())
                        .unwrap_or_default(),
                    effectiveness: conversation.engagement_score,
                    context: conversation.context.clone(),
                });
            }
            
            // Detectar respuestas que generan engagement
            for (i, msg) in conversation.messages.iter().enumerate() {
                if msg.likes > 5 || msg.replies > 3 {
                    self.patterns.push(ConversationPattern {
                        pattern_type: "high_engagement".to_string(),
                        text: msg.text.clone(),
                        effectiveness: (msg.likes + msg.replies * 2) as f32,
                        context: format!("Message {} in conversation", i + 1),
                    });
                }
            }
            
            // Detectar tone y estilo efectivo
            let tone = self.detect_tone(&conversation.messages);
            if !tone.is_empty() {
                self.patterns.push(ConversationPattern {
                    pattern_type: "effective_tone".to_string(),
                    text: tone.clone(),
                    effectiveness: conversation.engagement_score,
                    context: conversation.context.clone(),
                });
            }
        }
        
        format!("✅ Detected {} patterns from {} conversations", 
                self.patterns.len(), self.scraped_data.len())
    }
    
    /// Exporta datos para entrenar el chatbot
    #[wasm_bindgen]
    pub fn export_training_data(&self) -> String {
        let training_data = TrainingDataset {
            conversations: self.scraped_data.clone(),
            patterns: self.patterns.clone(),
            total_samples: self.scraped_data.len(),
            total_patterns: self.patterns.len(),
        };
        
        serde_json::to_string_pretty(&training_data).unwrap_or_default()
    }
    
    /// Obtiene top patterns para el bot
    #[wasm_bindgen]
    pub fn get_top_patterns(&self, limit: usize) -> String {
        let mut sorted = self.patterns.clone();
        sorted.sort_by(|a, b| b.effectiveness.partial_cmp(&a.effectiveness).unwrap());
        
        let top = sorted.iter().take(limit).collect::<Vec<_>>();
        serde_json::to_string_pretty(&top).unwrap_or_default()
    }
}

impl WasmScraper {
    /// Extrae conversaciones del HTML (ultra-rápido en WASM)
    fn extract_conversations_fast(&self, html: &str, source: &str) -> Vec<ScrapedConversation> {
        let mut conversations = Vec::new();
        
        // Detectar diferentes tipos de conversaciones
        let lines: Vec<&str> = html.lines().collect();
        
        let mut current_conversation = Vec::new();
        let mut in_conversation = false;
        
        for line in lines {
            let line = line.trim();
            
            // Detectar inicio de conversación
            if line.contains("class=\"message\"") || 
               line.contains("class=\"comment\"") ||
               line.contains("class=\"chat\"") {
                in_conversation = true;
            }
            
            // Extraer mensaje
            if in_conversation && !line.is_empty() {
                if let Some(text) = self.extract_text(line) {
                    current_conversation.push(Message {
                        text,
                        likes: self.extract_number(line, "likes"),
                        replies: self.extract_number(line, "replies"),
                        timestamp: self.extract_timestamp(line),
                    });
                }
            }
            
            // Fin de conversación
            if line.contains("</div>") && in_conversation {
                if !current_conversation.is_empty() {
                    conversations.push(ScrapedConversation {
                        messages: current_conversation.clone(),
                        source: source.to_string(),
                        engagement_score: self.calculate_engagement(&current_conversation),
                        context: self.extract_context(&current_conversation),
                    });
                    current_conversation.clear();
                }
                in_conversation = false;
            }
        }
        
        conversations
    }
    
    fn extract_text(&self, html: &str) -> Option<String> {
        // Extrae texto entre > y <
        if let Some(start) = html.find('>') {
            if let Some(end) = html[start..].find('<') {
                let text = &html[start + 1..start + end];
                if !text.trim().is_empty() {
                    return Some(text.trim().to_string());
                }
            }
        }
        None
    }
    
    fn extract_number(&self, html: &str, attribute: &str) -> u32 {
        if let Some(pos) = html.find(attribute) {
            let rest = &html[pos..];
            if let Some(num_start) = rest.find('"') {
                if let Some(num_end) = rest[num_start + 1..].find('"') {
                    let num_str = &rest[num_start + 1..num_start + 1 + num_end];
                    return num_str.parse().unwrap_or(0);
                }
            }
        }
        0
    }
    
    fn extract_timestamp(&self, html: &str) -> String {
        // Buscar timestamp común
        if let Some(pos) = html.find("data-time") {
            if let Some(start) = html[pos..].find('"') {
                if let Some(end) = html[pos + start + 1..].find('"') {
                    return html[pos + start + 1..pos + start + 1 + end].to_string();
                }
            }
        }
        chrono::Utc::now().to_rfc3339()
    }
    
    fn calculate_engagement(&self, messages: &[Message]) -> f32 {
        let total_likes: u32 = messages.iter().map(|m| m.likes).sum();
        let total_replies: u32 = messages.iter().map(|m| m.replies).sum();
        
        (total_likes as f32 * 0.5 + total_replies as f32 * 2.0) / messages.len().max(1) as f32
    }
    
    fn extract_context(&self, messages: &[Message]) -> String {
        // Detectar tema de conversación
        let combined_text = messages.iter()
            .map(|m| m.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        
        if combined_text.contains("hey") || combined_text.contains("hi") {
            "greeting".to_string()
        } else if combined_text.contains("how") || combined_text.contains("what") {
            "question".to_string()
        } else if combined_text.contains("thanks") || combined_text.contains("appreciate") {
            "gratitude".to_string()
        } else {
            "general".to_string()
        }
    }
    
    fn detect_tone(&self, messages: &[Message]) -> String {
        let text = messages.iter()
            .map(|m| m.text.as_str())
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        
        if text.contains("!") || text.contains("amazing") || text.contains("love") {
            "enthusiastic".to_string()
        } else if text.contains("?") {
            "curious".to_string()
        } else if text.contains("haha") || text.contains("lol") {
            "playful".to_string()
        } else {
            "neutral".to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedConversation {
    pub messages: Vec<Message>,
    pub source: String,
    pub engagement_score: f32,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub likes: u32,
    pub replies: u32,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationPattern {
    pub pattern_type: String,
    pub text: String,
    pub effectiveness: f32,
    pub context: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingDataset {
    pub conversations: Vec<ScrapedConversation>,
    pub patterns: Vec<ConversationPattern>,
    pub total_samples: usize,
    pub total_patterns: usize,
}
