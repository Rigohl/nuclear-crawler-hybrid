// WebAssembly Ultra-Fast Scraper
// Scraping de alto rendimiento compilado a WASM
// Para uso en: OSINT, investigación de seguridad, pentesting

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use regex::Regex;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct ScrapedData {
    pub phones: Vec<String>,
    pub emails: Vec<String>,
    pub social_profiles: Vec<SocialProfile>,
    pub chat_data: Vec<ChatMessage>,
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SocialProfile {
    pub platform: String,
    pub username: String,
    pub url: String,
    pub followers: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub platform: String,
    pub author: String,
    pub message: String,
    pub timestamp: String,
    pub phone: Option<String>,
}

#[wasm_bindgen]
pub struct NuclearScraper {
    phone_regex: Regex,
    email_regex: Regex,
    stealth_mode: bool,
}

#[wasm_bindgen]
impl NuclearScraper {
    #[wasm_bindgen(constructor)]
    pub fn new(stealth_mode: bool) -> Self {
        // Regex para teléfonos (internacional)
        let phone_regex = Regex::new(
            r"(?:\+?\d{1,3}[-.\s]?)?(?:\(?\d{1,4}\)?[-.\s]?)?[\d\s.-]{7,15}"
        ).unwrap();
        
        // Regex para emails
        let email_regex = Regex::new(
            r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"
        ).unwrap();
        
        Self {
            phone_regex,
            email_regex,
            stealth_mode,
        }
    }
    
    /// Extrae números de teléfono de HTML/texto
    #[wasm_bindgen]
    pub fn extract_phones(&self, html: &str) -> JsValue {
        let phones: Vec<String> = self.phone_regex
            .find_iter(html)
            .map(|m| self.normalize_phone(m.as_str()))
            .filter(|p| self.is_valid_phone(p))
            .collect();
        
        serde_wasm_bindgen::to_value(&phones).unwrap()
    }
    
    /// Extrae emails
    #[wasm_bindgen]
    pub fn extract_emails(&self, html: &str) -> JsValue {
        let emails: Vec<String> = self.email_regex
            .find_iter(html)
            .map(|m| m.as_str().to_lowercase())
            .collect();
        
        serde_wasm_bindgen::to_value(&emails).unwrap()
    }
    
    /// Extrae perfiles sociales (Facebook, Instagram, Twitter, LinkedIn, etc.)
    #[wasm_bindgen]
    pub fn extract_social_profiles(&self, html: &str) -> JsValue {
        let mut profiles: Vec<SocialProfile> = Vec::new();
        
        // Facebook
        let fb_regex = Regex::new(r"(?:https?://)?(?:www\.)?facebook\.com/([a-zA-Z0-9._-]+)").unwrap();
        for cap in fb_regex.captures_iter(html) {
            if let Some(username) = cap.get(1) {
                profiles.push(SocialProfile {
                    platform: "Facebook".to_string(),
                    username: username.as_str().to_string(),
                    url: format!("https://facebook.com/{}", username.as_str()),
                    followers: None,
                });
            }
        }
        
        // Instagram
        let ig_regex = Regex::new(r"(?:https?://)?(?:www\.)?instagram\.com/([a-zA-Z0-9._-]+)").unwrap();
        for cap in ig_regex.captures_iter(html) {
            if let Some(username) = cap.get(1) {
                profiles.push(SocialProfile {
                    platform: "Instagram".to_string(),
                    username: username.as_str().to_string(),
                    url: format!("https://instagram.com/{}", username.as_str()),
                    followers: None,
                });
            }
        }
        
        // Twitter/X
        let tw_regex = Regex::new(r"(?:https?://)?(?:www\.)?(?:twitter|x)\.com/([a-zA-Z0-9_]+)").unwrap();
        for cap in tw_regex.captures_iter(html) {
            if let Some(username) = cap.get(1) {
                profiles.push(SocialProfile {
                    platform: "Twitter".to_string(),
                    username: username.as_str().to_string(),
                    url: format!("https://x.com/{}", username.as_str()),
                    followers: None,
                });
            }
        }
        
        // LinkedIn
        let li_regex = Regex::new(r"(?:https?://)?(?:www\.)?linkedin\.com/in/([a-zA-Z0-9_-]+)").unwrap();
        for cap in li_regex.captures_iter(html) {
            if let Some(username) = cap.get(1) {
                profiles.push(SocialProfile {
                    platform: "LinkedIn".to_string(),
                    username: username.as_str().to_string(),
                    url: format!("https://linkedin.com/in/{}", username.as_str()),
                    followers: None,
                });
            }
        }
        
        // Telegram
        let tg_regex = Regex::new(r"(?:https?://)?(?:www\.)?t\.me/([a-zA-Z0-9_]+)").unwrap();
        for cap in tg_regex.captures_iter(html) {
            if let Some(username) = cap.get(1) {
                profiles.push(SocialProfile {
                    platform: "Telegram".to_string(),
                    username: username.as_str().to_string(),
                    url: format!("https://t.me/{}", username.as_str()),
                    followers: None,
                });
            }
        }
        
        // WhatsApp (enlaces)
        let wa_regex = Regex::new(r"(?:https?://)?(?:wa\.me|api\.whatsapp\.com)/(\d+)").unwrap();
        for cap in wa_regex.captures_iter(html) {
            if let Some(phone) = cap.get(1) {
                profiles.push(SocialProfile {
                    platform: "WhatsApp".to_string(),
                    username: phone.as_str().to_string(),
                    url: format!("https://wa.me/{}", phone.as_str()),
                    followers: None,
                });
            }
        }
        
        serde_wasm_bindgen::to_value(&profiles).unwrap()
    }
    
    /// Scraping completo (todo de una vez)
    #[wasm_bindgen]
    pub fn scrape_all(&self, html: &str, url: &str) -> JsValue {
        let phones = self.extract_phones_vec(html);
        let emails = self.extract_emails_vec(html);
        let social_profiles = self.extract_social_profiles_vec(html);
        
        let mut metadata = HashMap::new();
        metadata.insert("url".to_string(), url.to_string());
        metadata.insert("total_phones".to_string(), phones.len().to_string());
        metadata.insert("total_emails".to_string(), emails.len().to_string());
        metadata.insert("total_profiles".to_string(), social_profiles.len().to_string());
        
        let data = ScrapedData {
            phones,
            emails,
            social_profiles,
            chat_data: Vec::new(),
            metadata,
        };
        
        serde_wasm_bindgen::to_value(&data).unwrap()
    }
    
    // Helpers internos
    fn normalize_phone(&self, phone: &str) -> String {
        phone.chars()
            .filter(|c| c.is_numeric() || *c == '+')
            .collect()
    }
    
    fn is_valid_phone(&self, phone: &str) -> bool {
        let digits: String = phone.chars().filter(|c| c.is_numeric()).collect();
        digits.len() >= 7 && digits.len() <= 15
    }
    
    fn extract_phones_vec(&self, html: &str) -> Vec<String> {
        self.phone_regex
            .find_iter(html)
            .map(|m| self.normalize_phone(m.as_str()))
            .filter(|p| self.is_valid_phone(p))
            .collect()
    }
    
    fn extract_emails_vec(&self, html: &str) -> Vec<String> {
        self.email_regex
            .find_iter(html)
            .map(|m| m.as_str().to_lowercase())
            .collect()
    }
    
    fn extract_social_profiles_vec(&self, html: &str) -> Vec<SocialProfile> {
        // Usar la función extract_social_profiles y deserializar
        let js_value = self.extract_social_profiles(html);
        serde_wasm_bindgen::from_value(js_value).unwrap_or_default()
    }
}

/// Parser de chats (WhatsApp export, Telegram export, Discord export)
#[wasm_bindgen]
pub struct ChatParser;

#[wasm_bindgen]
impl ChatParser {
    #[wasm_bindgen]
    pub fn parse_whatsapp_export(text: &str) -> JsValue {
        let mut messages: Vec<ChatMessage> = Vec::new();
        
        // Formato WhatsApp: [DD/MM/YY, HH:MM:SS] Author: Message
        let wa_regex = Regex::new(
            r"\[(\d{1,2}/\d{1,2}/\d{2,4}),?\s*(\d{1,2}:\d{2}:\d{2}(?:\s*[AP]M)?)\]\s*([^:]+):\s*(.+)"
        ).unwrap();
        
        for cap in wa_regex.captures_iter(text) {
            let timestamp = format!("{} {}", &cap[1], &cap[2]);
            let author = cap[3].trim().to_string();
            let message = cap[4].trim().to_string();
            
            // Extraer teléfono del mensaje si existe
            let phone_regex = Regex::new(r"\+?\d[\d\s.-]{7,15}").unwrap();
            let phone = phone_regex.find(&message).map(|m| m.as_str().to_string());
            
            messages.push(ChatMessage {
                platform: "WhatsApp".to_string(),
                author,
                message,
                timestamp,
                phone,
            });
        }
        
        serde_wasm_bindgen::to_value(&messages).unwrap()
    }
    
    #[wasm_bindgen]
    pub fn parse_telegram_export(json: &str) -> JsValue {
        // Telegram exports en JSON
        let messages: Vec<ChatMessage> = Vec::new();
        // TODO: Implementar parser JSON de Telegram
        serde_wasm_bindgen::to_value(&messages).unwrap()
    }
    
    #[wasm_bindgen]
    pub fn parse_discord_export(json: &str) -> JsValue {
        // Discord exports en JSON
        let messages: Vec<ChatMessage> = Vec::new();
        // TODO: Implementar parser JSON de Discord
        serde_wasm_bindgen::to_value(&messages).unwrap()
    }
}

/// Fingerprint spoofer (anti-detección)
#[wasm_bindgen]
pub struct StealthConfig {
    user_agents: Vec<String>,
    current_ua_index: usize,
}

#[wasm_bindgen]
impl StealthConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let user_agents = vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36".to_string(),
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36".to_string(),
            "Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X)".to_string(),
        ];
        
        Self {
            user_agents,
            current_ua_index: 0,
        }
    }
    
    #[wasm_bindgen]
    pub fn get_random_user_agent(&mut self) -> String {
        let ua = self.user_agents[self.current_ua_index].clone();
        self.current_ua_index = (self.current_ua_index + 1) % self.user_agents.len();
        ua
    }
}
