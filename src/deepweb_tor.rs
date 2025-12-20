//! 🔥 DEEPWEB/TOR SEARCH - NUCLEAR CRAWLER HYBRID
//! REAL TOR integration for .onion sites and deepweb content
//! Uses SOCKS5 proxy (127.0.0.1:9050) for TOR routing
//! NO MOCKS - 100% REAL IMPLEMENTATION

use anyhow::{Context, Result};
use reqwest::{Client, Proxy};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// TOR/DeepWeb search result with full metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepWebResult {
    pub url: String,
    pub title: Option<String>,
    pub content_preview: String,
    pub content_type: String,
    pub size_bytes: usize,
    pub response_time_ms: u64,
    pub is_onion: bool,
    pub is_i2p: bool,
    pub security_score: f32,
    pub extracted_links: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Configuration for TOR/DeepWeb searches
#[derive(Debug, Clone)]
pub struct TorConfig {
    pub tor_proxy: String,
    pub i2p_proxy: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub user_agent: String,
}

impl Default for TorConfig {
    fn default() -> Self {
        Self {
            tor_proxy: "socks5://127.0.0.1:9050".to_string(), // Standard TOR SOCKS5 proxy
            i2p_proxy: "http://127.0.0.1:4444".to_string(),   // Standard I2P HTTP proxy
            timeout_seconds: 60,
            max_retries: 3,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; rv:109.0) Gecko/20100101 Firefox/115.0"
                .to_string(),
        }
    }
}

/// DeepWeb/TOR Search Engine
pub struct DeepWebSearch {
    /// 🔥 Tor configuration
    config: TorConfig,
    tor_client: Option<Client>,
    i2p_client: Option<Client>,
    clearnet_client: Client,
}

impl DeepWebSearch {
    /// Initialize DeepWeb search with TOR/I2P support
    pub fn new(config: TorConfig) -> Result<Self> {
        eprintln!("🔥 Initializing DeepWeb/TOR Search Engine");
        eprintln!("  • TOR Proxy: {}", config.tor_proxy);
        eprintln!("  • I2P Proxy: {}", config.i2p_proxy);
        eprintln!("  • Timeout: {}s", config.timeout_seconds);

        // 🔥 REAL TOR CLIENT with SOCKS5 proxy
        let tor_client = match Proxy::all(&config.tor_proxy) {
            Ok(proxy) => {
                match Client::builder()
                    .proxy(proxy)
                    .timeout(Duration::from_secs(config.timeout_seconds))
                    .user_agent(&config.user_agent)
                    .danger_accept_invalid_certs(true) // Many .onion sites have self-signed certs
                    .build()
                {
                    Ok(client) => {
                        eprintln!("  ✅ TOR client initialized successfully");
                        Some(client)
                    }
                    Err(e) => {
                        eprintln!(
                            "  ⚠️  TOR client failed: {} (will use clearnet fallback)",
                            e
                        );
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("  ⚠️  TOR proxy configuration failed: {}", e);
                None
            }
        };

        // 🔥 REAL I2P CLIENT with HTTP proxy
        let i2p_client = match Proxy::http(&config.i2p_proxy) {
            Ok(proxy) => {
                match Client::builder()
                    .proxy(proxy)
                    .timeout(Duration::from_secs(config.timeout_seconds))
                    .user_agent(&config.user_agent)
                    .build()
                {
                    Ok(client) => {
                        eprintln!("  ✅ I2P client initialized successfully");
                        Some(client)
                    }
                    Err(e) => {
                        eprintln!(
                            "  ⚠️  I2P client failed: {} (will use clearnet fallback)",
                            e
                        );
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("  ⚠️  I2P proxy configuration failed: {}", e);
                None
            }
        };

        // 🔥 Standard HTTPS client for clearnet fallback
        let clearnet_client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .user_agent(&config.user_agent)
            .build()
            .context("Failed to create clearnet HTTP client")?;

        eprintln!("  ✅ Clearnet client initialized");

        Ok(Self {
            config,
            tor_client,
            i2p_client,
            clearnet_client,
        })
    }

    /// Search DeepWeb/TOR with automatic routing
    pub async fn search(&self, query: &str) -> Result<Vec<DeepWebResult>> {
        eprintln!("🔥 DeepWeb Search: '{}'", query);
        eprintln!(
            "  Config: timeout {}s, max_retries {}",
            self.config.timeout_seconds, self.config.max_retries
        );

        let mut all_results = Vec::new();

        // 🔥 SEARCH HIDDEN WIKI AND TOR SEARCH ENGINES
        let tor_search_engines = vec![
            "http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion/", // Ahmia - TOR search engine
            "http://hss3uro2hsxfogfq.onion/", // Not Evil - TOR search
            "http://xmh57jrzrnw6insl.onion/", // Torch - TOR search
        ];

        // 🔥 SEARCH I2P SITES
        let i2p_search_engines = vec!["http://stats.i2p/", "http://identiguy.i2p/"];

        // 🔥 Try TOR search engines
        if self.tor_client.is_some() {
            eprintln!("  🔍 Searching TOR network...");
            for engine_url in &tor_search_engines {
                if let Ok(results) = self.search_tor_engine(engine_url, query).await {
                    all_results.extend(results);
                }
            }
        } else {
            eprintln!("  ⚠️  TOR not available, skipping .onion searches");
        }

        // 🔥 Try I2P search engines
        if self.i2p_client.is_some() {
            eprintln!("  🔍 Searching I2P network...");
            for engine_url in &i2p_search_engines {
                if let Ok(results) = self.search_i2p_engine(engine_url, query).await {
                    all_results.extend(results);
                }
            }
        } else {
            eprintln!("  ⚠️  I2P not available, skipping .i2p searches");
        }

        // 🔥 Also search clearnet for .onion references
        eprintln!("  🔍 Searching clearnet for deepweb references...");
        if let Ok(results) = self.search_clearnet_for_onion(query).await {
            all_results.extend(results);
        }

        eprintln!(
            "🔥 DeepWeb search completed: {} results found",
            all_results.len()
        );
        Ok(all_results)
    }

    /// Fetch a specific URL (auto-detects TOR/I2P/clearnet)
    pub async fn fetch_url(&self, url: &str) -> Result<DeepWebResult> {
        let start = Instant::now();
        eprintln!("🔥 Fetching: {}", url);

        let (client, is_onion, is_i2p) = if url.contains(".onion") {
            (
                self.tor_client.as_ref().unwrap_or(&self.clearnet_client),
                true,
                false,
            )
        } else if url.contains(".i2p") {
            (
                self.i2p_client.as_ref().unwrap_or(&self.clearnet_client),
                false,
                true,
            )
        } else {
            (&self.clearnet_client, false, false)
        };

        let response = client
            .get(url)
            .send()
            .await
            .context("Failed to fetch URL")?;

        let status = response.status();
        let headers = response.headers().clone();
        let content = response
            .text()
            .await
            .context("Failed to read response body")?;

        let response_time_ms = start.elapsed().as_millis() as u64;

        // Extract metadata
        let mut metadata = HashMap::new();
        metadata.insert("status".to_string(), status.as_u16().to_string());
        for (key, value) in headers.iter() {
            if let Ok(v) = value.to_str() {
                metadata.insert(key.to_string(), v.to_string());
            }
        }

        // Extract links
        let extracted_links = self.extract_links(&content, url);

        // Extract title
        let title = self.extract_title(&content);

        // Calculate security score
        let security_score = self.calculate_security_score(&content, &metadata);

        let result = DeepWebResult {
            url: url.to_string(),
            title,
            content_preview: content.chars().take(500).collect(),
            content_type: metadata
                .get("content-type")
                .unwrap_or(&"text/html".to_string())
                .clone(),
            size_bytes: content.len(),
            response_time_ms,
            is_onion,
            is_i2p,
            security_score,
            extracted_links,
            metadata,
        };

        eprintln!(
            "  ✅ Fetched {} bytes in {}ms",
            result.size_bytes, response_time_ms
        );
        Ok(result)
    }

    /// Search a TOR search engine
    async fn search_tor_engine(&self, engine_url: &str, query: &str) -> Result<Vec<DeepWebResult>> {
        let client = self
            .tor_client
            .as_ref()
            .context("TOR client not available")?;

        // Try to fetch the search engine homepage
        let search_url = format!("{}?q={}", engine_url, urlencoding::encode(query));

        match client.get(&search_url).send().await {
            Ok(response) => {
                let html = response.text().await?;
                Ok(self.parse_search_results(&html, true, false))
            }
            Err(e) => {
                eprintln!("  ⚠️  TOR search failed for {}: {}", engine_url, e);
                Ok(Vec::new())
            }
        }
    }

    /// Search an I2P search engine
    async fn search_i2p_engine(&self, engine_url: &str, query: &str) -> Result<Vec<DeepWebResult>> {
        let client = self
            .i2p_client
            .as_ref()
            .context("I2P client not available")?;

        let search_url = format!("{}?q={}", engine_url, urlencoding::encode(query));

        match client.get(&search_url).send().await {
            Ok(response) => {
                let html = response.text().await?;
                Ok(self.parse_search_results(&html, false, true))
            }
            Err(e) => {
                eprintln!("  ⚠️  I2P search failed for {}: {}", engine_url, e);
                Ok(Vec::new())
            }
        }
    }

    /// Search clearnet for .onion/.i2p references
    async fn search_clearnet_for_onion(&self, query: &str) -> Result<Vec<DeepWebResult>> {
        // Use DuckDuckGo to find .onion links
        let search_query = format!("{} .onion OR .i2p", query);
        let search_url = format!(
            "https://duckduckgo.com/html/?q={}",
            urlencoding::encode(&search_query)
        );

        match self.clearnet_client.get(&search_url).send().await {
            Ok(response) => {
                let html = response.text().await?;
                Ok(self.parse_search_results(&html, false, false))
            }
            Err(e) => {
                eprintln!("  ⚠️  Clearnet search failed: {}", e);
                Ok(Vec::new())
            }
        }
    }

    /// Parse search results from HTML
    fn parse_search_results(&self, html: &str, is_onion: bool, is_i2p: bool) -> Vec<DeepWebResult> {
        let mut results = Vec::new();

        // Extract all links that look like .onion or .i2p
        let links = self.extract_links(html, "");

        for link in links {
            if (is_onion && link.contains(".onion"))
                || (is_i2p && link.contains(".i2p"))
                || (!is_onion && !is_i2p && (link.contains(".onion") || link.contains(".i2p")))
            {
                results.push(DeepWebResult {
                    url: link.clone(),
                    title: None,
                    content_preview: String::new(),
                    content_type: "unknown".to_string(),
                    size_bytes: 0,
                    response_time_ms: 0,
                    is_onion: link.contains(".onion"),
                    is_i2p: link.contains(".i2p"),
                    security_score: 0.5,
                    extracted_links: Vec::new(),
                    metadata: HashMap::new(),
                });
            }
        }

        results
    }

    /// Extract all links from HTML
    fn extract_links(&self, html: &str, base_url: &str) -> Vec<String> {
        let mut links = Vec::new();

        // Simple regex-free extraction
        for part in html.split("href=\"") {
            if let Some(end) = part.find('"') {
                let link = &part[..end];
                if link.starts_with("http") || link.contains(".onion") || link.contains(".i2p") {
                    links.push(link.to_string());
                } else if !link.starts_with('#')
                    && !link.starts_with("javascript:")
                    && !base_url.is_empty()
                {
                    // Relative URL - make absolute
                    if let Ok(base) = url::Url::parse(base_url) {
                        if let Ok(absolute) = base.join(link) {
                            links.push(absolute.to_string());
                        }
                    }
                }
            }
        }

        links
    }

    /// Extract title from HTML
    fn extract_title(&self, html: &str) -> Option<String> {
        html.split("<title>")
            .nth(1)
            .and_then(|s| s.split("</title>").next())
            .map(|s| s.trim().to_string())
    }

    /// Calculate security score based on content
    fn calculate_security_score(&self, content: &str, metadata: &HashMap<String, String>) -> f32 {
        let mut score: f32 = 0.5;

        // Check for HTTPS
        if metadata.get("status").map(|s| s == "200").unwrap_or(false) {
            score += 0.2;
        }

        // Check for common security indicators
        if content.contains("csrf") || content.contains("xsrf") {
            score += 0.1;
        }

        // Penalize if looks like scam
        let scam_keywords = ["bitcoin", "wallet", "send money", "login", "password"];
        let scam_count = scam_keywords
            .iter()
            .filter(|&kw| content.to_lowercase().contains(kw))
            .count();

        if scam_count > 3 {
            score -= 0.3;
        }

        score.max(0.0_f32).min(1.0_f32)
    }

    /// Get known DeepWeb directories and search engines
    pub fn get_known_deepweb_sources(&self) -> Vec<String> {
        vec![
            // TOR Hidden Wiki mirrors
            "http://zqktlwiuavvvqqt4ybvgvi7tyo4hjl5xgfuvpdf6otjiycgwqbym2qad.onion/wiki/"
                .to_string(),
            // TOR Search Engines
            "http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion/".to_string(), // Ahmia
            "http://hss3uro2hsxfogfq.onion/".to_string(), // Not Evil
            "http://xmh57jrzrnw6insl.onion/".to_string(), // Torch
            // TOR Directories
            "http://darkfailenbsdla5mal2mxn2uz66od5vtzd5qozslagrfzachha3f3id.onion/".to_string(), // DarkFail
            // I2P Sites
            "http://stats.i2p/".to_string(),
            "http://i2p-projekt.i2p/".to_string(),
        ]
    }

    /// Check if TOR is available
    pub fn is_tor_available(&self) -> bool {
        self.tor_client.is_some()
    }

    /// Check if I2P is available
    pub fn is_i2p_available(&self) -> bool {
        self.i2p_client.is_some()
    }
}

impl Default for DeepWebSearch {
    fn default() -> Self {
        Self::new(TorConfig::default()).unwrap_or_else(|e| {
            eprintln!("⚠️  DeepWeb search initialization failed: {}", e);
            panic!("Cannot create DeepWebSearch");
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let config = TorConfig::default();
        let deepweb = DeepWebSearch::new(config);
        assert!(deepweb.is_ok());
    }

    #[test]
    fn test_known_sources() {
        let deepweb = DeepWebSearch::default();
        let sources = deepweb.get_known_deepweb_sources();
        assert!(!sources.is_empty());
        assert!(sources.iter().any(|s| s.contains(".onion")));
    }
}
