//! Módulo Stealth - Anti-Detección y Ocultamiento
//!
//! Sistema avanzado de evasión de detección, rotación de fingerprints,
//! y técnicas de ocultamiento para evitar bans

use dashmap::DashMap;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// User agents reales y actualizados
const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
];

/// Headers realistas por navegador
const CHROME_HEADERS: &[(&str, &str)] = &[
    ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8"),
    ("Accept-Language", "en-US,en;q=0.9"),
    ("Accept-Encoding", "gzip, deflate, br"),
    ("DNT", "1"),
    ("Connection", "keep-alive"),
    ("Upgrade-Insecure-Requests", "1"),
    ("Sec-Fetch-Dest", "document"),
    ("Sec-Fetch-Mode", "navigate"),
    ("Sec-Fetch-Site", "none"),
    ("Sec-Fetch-User", "?1"),
    ("Cache-Control", "max-age=0"),
];

const FIREFOX_HEADERS: &[(&str, &str)] = &[
    (
        "Accept",
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    ),
    ("Accept-Language", "en-US,en;q=0.5"),
    ("Accept-Encoding", "gzip, deflate, br"),
    ("DNT", "1"),
    ("Connection", "keep-alive"),
    ("Upgrade-Insecure-Requests", "1"),
    ("Sec-Fetch-Dest", "document"),
    ("Sec-Fetch-Mode", "navigate"),
    ("Sec-Fetch-Site", "none"),
    ("Sec-Fetch-User", "?1"),
];

/// Configuración de Stealth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthConfig {
    /// Rotar user agents
    pub rotate_user_agents: bool,

    /// Rotar headers
    pub rotate_headers: bool,

    /// Delay aleatorio entre requests (ms)
    pub random_delay_min: u64,
    pub random_delay_max: u64,

    /// Simular comportamiento humano
    pub human_behavior: bool,

    /// Usar proxies
    pub use_proxies: bool,

    /// Lista de proxies
    pub proxies: Vec<String>,

    /// Evitar detección de headless
    pub avoid_headless_detection: bool,

    /// TLS fingerprinting evasion
    pub tls_fingerprint_evasion: bool,
}

impl Default for StealthConfig {
    fn default() -> Self {
        Self {
            rotate_user_agents: true,
            rotate_headers: true,
            random_delay_min: 1000,
            random_delay_max: 5000,
            human_behavior: true,
            use_proxies: false,
            proxies: Vec::new(),
            avoid_headless_detection: true,
            tls_fingerprint_evasion: true,
        }
    }
}

/// Sistema de Stealth
pub struct StealthSystem {
    config: StealthConfig,
    user_agent_pool: Vec<String>,
    header_pools: HashMap<String, Vec<HashMap<String, String>>>,
    proxy_pool: Vec<String>,
    current_proxy_index: Arc<DashMap<String, usize>>, // Por dominio
    request_count: Arc<DashMap<String, usize>>,       // Por dominio
}

impl StealthSystem {
    /// Crea un nuevo sistema de stealth
    pub fn new(config: StealthConfig) -> Self {
        let user_agent_pool = USER_AGENTS.iter().map(|s| s.to_string()).collect();

        // Crear pools de headers
        let mut header_pools = HashMap::new();

        // Chrome headers
        let chrome_headers: Vec<HashMap<String, String>> = (0..5)
            .map(|_| {
                let mut headers = HashMap::new();
                for (k, v) in CHROME_HEADERS {
                    headers.insert(k.to_string(), v.to_string());
                }
                headers
            })
            .collect();
        header_pools.insert("chrome".to_string(), chrome_headers);

        // Firefox headers
        let firefox_headers: Vec<HashMap<String, String>> = (0..5)
            .map(|_| {
                let mut headers = HashMap::new();
                for (k, v) in FIREFOX_HEADERS {
                    headers.insert(k.to_string(), v.to_string());
                }
                headers
            })
            .collect();
        header_pools.insert("firefox".to_string(), firefox_headers);

        Self {
            config,
            user_agent_pool,
            header_pools,
            proxy_pool: Vec::new(),
            current_proxy_index: Arc::new(DashMap::new()),
            request_count: Arc::new(DashMap::new()),
        }
    }

    /// Obtiene un user agent aleatorio
    pub fn get_user_agent(&self) -> String {
        if self.config.rotate_user_agents && !self.user_agent_pool.is_empty() {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..self.user_agent_pool.len());
            self.user_agent_pool[index].clone()
        } else {
            USER_AGENTS[0].to_string()
        }
    }

    /// Obtiene headers realistas
    pub fn get_headers(&self, browser_type: Option<&str>) -> HashMap<String, String> {
        let browser = browser_type.unwrap_or("chrome");

        if self.config.rotate_headers {
            if let Some(pool) = self.header_pools.get(browser) {
                if !pool.is_empty() {
                    let mut rng = rand::thread_rng();
                    let index = rng.gen_range(0..pool.len());
                    return pool[index].clone();
                }
            }
        }

        // Headers por defecto
        let mut headers = HashMap::new();
        let header_set = if browser == "firefox" {
            FIREFOX_HEADERS
        } else {
            CHROME_HEADERS
        };

        for (k, v) in header_set {
            headers.insert(k.to_string(), v.to_string());
        }

        headers
    }

    /// Obtiene un proxy para un dominio
    pub fn get_proxy(&self, domain: &str) -> Option<String> {
        if !self.config.use_proxies || self.proxy_pool.is_empty() {
            return None;
        }

        let mut current_index = self
            .current_proxy_index
            .entry(domain.to_string())
            .or_insert(0);

        let proxy = self.proxy_pool[*current_index % self.proxy_pool.len()].clone();

        // Rotar proxy
        *current_index = (*current_index + 1) % self.proxy_pool.len();

        Some(proxy)
    }

    /// Calcula delay humano aleatorio
    pub fn get_human_delay(&self) -> u64 {
        if self.config.human_behavior {
            let mut rng = rand::thread_rng();
            rng.gen_range(self.config.random_delay_min..=self.config.random_delay_max)
        } else {
            0
        }
    }

    /// Incrementa contador de requests para un dominio
    pub fn increment_request_count(&self, domain: &str) {
        let mut count = self.request_count.entry(domain.to_string()).or_insert(0);
        *count += 1;
    }

    /// Obtiene contador de requests
    pub fn get_request_count(&self, domain: &str) -> usize {
        *self.request_count.entry(domain.to_string()).or_insert(0)
    }

    /// Verifica si necesita pausa (anti-ban)
    pub fn should_pause(&self, domain: &str) -> bool {
        let count = self.get_request_count(domain);
        // Pausa cada 50 requests
        count.is_multiple_of(50)
    }

    /// Agrega proxies
    pub fn add_proxies(&mut self, proxies: Vec<String>) {
        self.proxy_pool.extend(proxies);
    }

    /// Genera headers anti-detección adicionales
    pub fn get_anti_detection_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();

        // Headers para evitar detección de headless
        if self.config.avoid_headless_detection {
            headers.insert(
                "sec-ch-ua".to_string(),
                "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\""
                    .to_string(),
            );
            headers.insert("sec-ch-ua-mobile".to_string(), "?0".to_string());
            headers.insert("sec-ch-ua-platform".to_string(), "\"Windows\"".to_string());
        }

        headers
    }
}
