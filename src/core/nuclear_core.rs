use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::timeout;

// ===== BYPASS SYSTEM =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BypassResult {
    pub access_url: String,
    pub content: String,
    pub bypass_method: String,
    pub success_rate: f32,
    pub premium_accessed: bool,
}

#[derive(Clone)]
pub struct NuclearBypass {
    bypass_engines: Vec<String>,
    rate_limiter: Arc<crate::nuclear_core::RateLimiter>,
    concealment: Arc<ExtremeConcealment>,
}

impl NuclearBypass {
    pub fn new(_config: NuclearBypassConfig, concealment: Arc<ExtremeConcealment>) -> Result<Self> {
        let bypass_engines = vec![
            "quantum_bypass".to_string(),
            "neural_injection".to_string(),
            "cookie_forgery".to_string(),
            "header_spoofing".to_string(),
            "referer_manipulation".to_string(),
            "user_agent_rotation".to_string(),
        ];

        Ok(Self {
            bypass_engines,
            rate_limiter: Arc::new(crate::nuclear_core::RateLimiter::new(100, 1000)),
            concealment,
        })
    }

    pub async fn bypass(&self, url: &str) -> Result<BypassResult> {
        self.rate_limiter.wait().await;

        // Try all bypass methods
        for method in &self.bypass_engines {
            match self.try_bypass_method(url, method).await {
                Ok(result) if result.premium_accessed => return Ok(result),
                _ => continue,
            }
        }

        // Fallback: return original URL
        Ok(BypassResult {
            access_url: url.to_string(),
            content: "Bypass failed - using original URL".to_string(),
            bypass_method: "none".to_string(),
            success_rate: 0.0,
            premium_accessed: false,
        })
    }

    async fn try_bypass_method(&self, url: &str, method: &str) -> Result<BypassResult> {
        // Real bypass attempt using reqwest with concealment
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        // Get stealth headers from concealment
        let mut headers = self.concealment.get_headers(Some(url)).await;

        // Add method-specific headers
        match method {
            "quantum_bypass" => {
                headers.insert("User-Agent".to_string(), "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string());
                headers.insert("Referer".to_string(), "https://www.google.com/".to_string());
            }
            "neural_injection" => {
                headers.insert("X-Requested-With".to_string(), "XMLHttpRequest".to_string());
            }
            "cookie_forgery" => {
                headers.insert(
                    "Cookie".to_string(),
                    "premium_access=true; user_type=vip".to_string(),
                );
            }
            "header_spoofing" => {
                headers.insert("Accept-Language".to_string(), "en-US,en;q=0.9".to_string());
            }
            "referer_manipulation" => {
                if let Some(domain) = self.extract_domain(url) {
                    headers.insert("Referer".to_string(), format!("https://www.{}/", domain));
                }
            }
            "user_agent_rotation" => {
                headers.insert("User-Agent".to_string(), "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1".to_string());
            }
            _ => {}
        }

        let reqwest_headers = (&headers).try_into().unwrap_or_default();

        match client.get(url).headers(reqwest_headers).send().await {
            Ok(response) if response.status().is_success() => {
                let content = response.text().await.unwrap_or_default();
                let premium_accessed = content.contains("premium")
                    || content.contains("full access")
                    || content.len() > 10000;
                Ok(BypassResult {
                    access_url: url.to_string(),
                    content,
                    bypass_method: method.to_string(),
                    success_rate: if premium_accessed { 1.0 } else { 0.5 },
                    premium_accessed,
                })
            }
            _ => Ok(BypassResult {
                access_url: url.to_string(),
                content: String::new(),
                bypass_method: method.to_string(),
                success_rate: 0.0,
                premium_accessed: false,
            }),
        }
    }

    fn extract_domain(&self, url: &str) -> Option<String> {
        url::Url::parse(url)
            .ok()?
            .host_str()?
            .to_string()
            .split('.')
            .next_back()?
            .to_string()
            .into()
    }
}

#[derive(Debug, Clone)]
pub struct NuclearBypassConfig {
    pub max_attempts: usize,
    pub timeout_seconds: u64,
}

impl Default for NuclearBypassConfig {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            timeout_seconds: 30,
        }
    }
}

// ===== EXTRACTION SYSTEM =====

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtractedData {
    pub main_text: String,
    pub word_count: usize,
    pub language: String,
    pub links: Vec<String>,
    pub images: Vec<String>,
    pub code_snippets: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub structured_data: serde_json::Value,
}

#[derive(Clone)]
pub enum WebScraperType {
    Html,
    Json,
    Api,
}

pub struct AdvancedExtractor {
    scrapers: Vec<WebScraperType>,
    content_analyzer: ContentAnalyzer,
}

impl Default for AdvancedExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl AdvancedExtractor {
    pub fn new() -> Self {
        Self {
            scrapers: vec![
                WebScraperType::Html,
                WebScraperType::Json,
                WebScraperType::Api,
            ],
            content_analyzer: ContentAnalyzer::new(),
        }
    }

    pub async fn extract_all(&self, html: &str, url: &str) -> Result<ExtractedData> {
        let mut all_data = ExtractedData::default();

        // Try all scrapers
        for scraper in &self.scrapers {
            if let Ok(data) = Self::scrape_with_type(scraper, html, url).await {
                all_data.main_text = data.main_text;
                all_data.links.extend(data.links);
                all_data.images.extend(data.images);
                all_data.code_snippets.extend(data.code_snippets);
                all_data.metadata.extend(data.metadata);
                all_data.structured_data = data.structured_data;
            }
        }

        // Analyze content
        all_data.word_count = all_data.main_text.split_whitespace().count();
        all_data.language = self.content_analyzer.detect_language(&all_data.main_text);
        if all_data.structured_data.is_null() {
            all_data.structured_data = self.content_analyzer.extract_structured_data(html);
        }

        Ok(all_data)
    }

    async fn scrape_with_type(
        scraper: &WebScraperType,
        html: &str,
        url: &str,
    ) -> Result<ExtractedData> {
        match scraper {
            WebScraperType::Html => Self::scrape_html(html, url),
            WebScraperType::Json => Self::scrape_json(html, url),
            WebScraperType::Api => Self::scrape_api(html, url),
        }
    }

        fn scrape_html(html: &str, _url: &str) -> Result<ExtractedData> {
        use scraper::{Html, Selector};

        let document = Html::parse_document(html);
        let link_selector = Selector::parse("a[href]").unwrap();
        let img_selector = Selector::parse("img[src]").unwrap();
        let code_selector = Selector::parse("pre, code").unwrap();

        let main_text = document.root_element().text().collect::<Vec<_>>().join(" ");

        let links = document
            .select(&link_selector)
            .filter_map(|el| el.value().attr("href"))
            .map(String::from)
            .collect();

        let images = document
            .select(&img_selector)
            .filter_map(|el| el.value().attr("src"))
            .map(String::from)
            .collect();

        let mut unique_snippets = Vec::new();
        for el in document.select(&code_selector) {
            let snippet = el.text().collect::<Vec<_>>().join("").trim().to_string();
            if !snippet.is_empty() && !unique_snippets.contains(&snippet) {
                unique_snippets.push(snippet);
            }
        }

        Ok(ExtractedData {
            main_text,
            word_count: 0, // Will be calculated later
            language: "unknown".to_string(),
            links,
            images,
            code_snippets: unique_snippets,
            metadata: HashMap::new(),
            structured_data: serde_json::Value::Null,
        })
    }

    fn scrape_json(html: &str, _url: &str) -> Result<ExtractedData> {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(html) {
            Ok(ExtractedData {
                main_text: format!(
                    "JSON Data: {} keys",
                    json.as_object().map(|o| o.len()).unwrap_or(0)
                ),
                word_count: 0,
                language: "json".to_string(),
                links: Vec::new(),
                images: Vec::new(),
                code_snippets: Vec::new(),
                metadata: HashMap::new(),
                structured_data: json,
            })
        } else {
            Err(anyhow::anyhow!("Not JSON content"))
        }
    }

    fn scrape_api(_html: &str, url: &str) -> Result<ExtractedData> {
        // Real API detection - no simulation
        if url.contains("/api/") || url.contains(".json") || url.contains("api.") {
            Ok(ExtractedData {
                main_text: "API Endpoint detected".to_string(),
                word_count: 3,
                language: "api".to_string(),
                links: Vec::new(),
                images: Vec::new(),
                code_snippets: Vec::new(),
                metadata: HashMap::new(),
                structured_data: serde_json::json!({"type": "api_endpoint", "url": url}),
            })
        } else {
            Err(anyhow::anyhow!("Not API content"))
        }
    }
}

struct ContentAnalyzer;

impl ContentAnalyzer {
    fn new() -> Self {
        Self
    }

    fn detect_language(&self, text: &str) -> String {
        // Simple language detection
        if text.contains("the ") || text.contains(" and ") || text.contains(" is ") {
            "english".to_string()
        } else if text.contains(" el ") || text.contains(" la ") || text.contains(" es ") {
            "spanish".to_string()
        } else {
            "unknown".to_string()
        }
    }

    fn extract_structured_data(&self, html: &str) -> serde_json::Value {
        // Extract JSON-LD structured data
        if let Some(start) = html.find(r#"<script type="application/ld+json">"#) {
            if let Some(end) = html[start..].find("</script>") {
                let json_str = &html[start + 35..start + end];
                if let Ok(json) = serde_json::from_str(json_str) {
                    return json;
                }
            }
        }
        serde_json::Value::Null
    }
}

// ===== CONCEALMENT SYSTEM =====

#[derive(Debug, Clone)]
pub struct StealthConfig {
    pub user_agents: Vec<String>,
    pub headers: HashMap<String, String>,
    pub proxy_rotation: bool,
    pub cookie_jar: bool,
    pub referer_spoofing: bool,
}

impl Default for StealthConfig {
    fn default() -> Self {
        Self {
            user_agents: vec![
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            ],
            headers: HashMap::new(),
            proxy_rotation: true,
            cookie_jar: true,
            referer_spoofing: true,
        }
    }
}

#[derive(Clone)]
pub struct ExtremeConcealment {
    config: StealthConfig,
    active_headers: Arc<RwLock<HashMap<String, String>>>,
}

impl ExtremeConcealment {
    pub fn new(config: StealthConfig) -> Self {
        Self {
            config,
            active_headers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_headers(&self, target_url: Option<&str>) -> HashMap<String, String> {
        let mut headers = HashMap::new();

        // 🔥 NUCLEAR STEALTH: Dynamic User Agent Rotation with Device Fingerprinting
        let ua_index =
            (Instant::now().elapsed().as_millis() / 10000) as usize % self.config.user_agents.len();
        headers.insert(
            "User-Agent".to_string(),
            self.config.user_agents[ua_index].clone(),
        );

        // 🔥 QUANTUM BYPASS: Advanced Referer Spoofing with Context Awareness
        if self.config.referer_spoofing {
            if let Some(url) = target_url {
                if let Some(domain) = self.extract_domain(url) {
                    // Spoof referer from search engines or social media
                    let referers = [
                        format!(
                            "https://www.google.com/search?q={}",
                            urlencoding::encode(url)
                        ),
                        format!("https://www.{}/", domain),
                        "https://t.co/".to_string(),
                        "https://www.facebook.com/".to_string(),
                        "https://www.linkedin.com/".to_string(),
                    ];
                    let referer_index =
                        (Instant::now().elapsed().as_millis() / 15000) as usize % referers.len();
                    headers.insert("Referer".to_string(), referers[referer_index].clone());
                }
            }
        }

        // 🔥 MAXIMUM STEALTH: Comprehensive Browser Fingerprint Evasion
        headers.insert("Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".to_string());
        headers.insert(
            "Accept-Language".to_string(),
            "en-US,en;q=0.9,es;q=0.8,fr;q=0.7".to_string(),
        );
        headers.insert(
            "Accept-Encoding".to_string(),
            "gzip, deflate, br".to_string(),
        );
        headers.insert("DNT".to_string(), "1".to_string());
        headers.insert("Connection".to_string(), "keep-alive".to_string());
        headers.insert("Upgrade-Insecure-Requests".to_string(), "1".to_string());

        // 🔥 NUCLEAR CONCEALMENT: Advanced Anti-Detection Headers
        headers.insert("Sec-Fetch-Dest".to_string(), "document".to_string());
        headers.insert("Sec-Fetch-Mode".to_string(), "navigate".to_string());
        headers.insert("Sec-Fetch-Site".to_string(), "none".to_string());
        headers.insert("Sec-Fetch-User".to_string(), "?1".to_string());
        headers.insert(
            "Sec-Ch-Ua".to_string(),
            "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\""
                .to_string(),
        );
        headers.insert("Sec-Ch-Ua-Mobile".to_string(), "?0".to_string());
        headers.insert("Sec-Ch-Ua-Platform".to_string(), "\"Windows\"".to_string());

        // 🔥 QUANTUM STEALTH: Cache Control and Privacy Headers
        headers.insert("Cache-Control".to_string(), "max-age=0".to_string());
        headers.insert("Pragma".to_string(), "no-cache".to_string());

        // 🔥 MAXIMUM BYPASS: Premium Content Access Headers
        headers.insert("X-Requested-With".to_string(), "XMLHttpRequest".to_string());
        headers.insert("X-Forwarded-For".to_string(), self.generate_random_ip());
        headers.insert("CF-RAY".to_string(), self.generate_fake_ray_id());

        headers
    }

    fn extract_domain(&self, url: &str) -> Option<String> {
        url::Url::parse(url)
            .ok()?
            .host_str()?
            .to_string()
            .split('.')
            .next_back()?
            .to_string()
            .into()
    }

    pub async fn rotate_identity(&self) {
        // Real identity rotation - cycle through user agents
        let mut headers = self.active_headers.write().await;
        headers.clear();

        let ua_index = (std::time::Instant::now().elapsed().as_millis() / 30000) as usize
            % self.config.user_agents.len();
        headers.insert(
            "User-Agent".to_string(),
            self.config.user_agents[ua_index].clone(),
        );
    }

    /// 🔥 NUCLEAR STEALTH: Generate random IP for X-Forwarded-For
    fn generate_random_ip(&self) -> String {
        use std::time::SystemTime;
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u32;
        let mut rng = oorandom::Rand32::new(seed.into());

        format!(
            "{}.{}.{}.{}",
            rng.rand_range(1..255),
            rng.rand_range(0..255),
            rng.rand_range(0..255),
            rng.rand_range(1..255)
        )
    }

    /// 🔥 QUANTUM BYPASS: Generate fake Cloudflare Ray ID
    fn generate_fake_ray_id(&self) -> String {
        use std::time::SystemTime;
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let random_part = (timestamp % 1000000) as u32;
        format!("{:x}-{}", timestamp / 1000, random_part)
    }
}

// ===== SCRAPING & SPIDER SYSTEM =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiderConfig {
    pub max_depth: usize,
    pub max_pages: usize,
    pub respect_robots: bool,
    pub crawl_delay_ms: u64,
    pub allowed_domains: Vec<String>,
    pub follow_external: bool,
}

impl Default for SpiderConfig {
    fn default() -> Self {
        Self {
            max_depth: 3,
            max_pages: 1000,
            respect_robots: false, // Nuclear mode - ignore robots.txt
            crawl_delay_ms: 100,
            allowed_domains: Vec::new(),
            follow_external: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiderResult {
    pub url: String,
    pub depth: usize,
    pub status_code: u16,
    pub content_length: usize,
    pub links_found: Vec<String>,
    pub crawled_at: chrono::DateTime<chrono::Utc>,
}

pub struct NuclearSpider {
    config: SpiderConfig,
    visited: Arc<RwLock<HashSet<String>>>,
    results: Arc<RwLock<Vec<SpiderResult>>>,
    extractor: AdvancedExtractor,
    concealment: ExtremeConcealment,
    rate_limiter: Arc<RateLimiter>,
}

impl NuclearSpider {
    pub fn new(config: SpiderConfig) -> Self {
        Self {
            config,
            visited: Arc::new(RwLock::new(HashSet::new())),
            results: Arc::new(RwLock::new(Vec::new())),
            extractor: AdvancedExtractor::new(),
            concealment: ExtremeConcealment::new(Default::default()),
            rate_limiter: Arc::new(RateLimiter::new(10, 1000)), // 10 req/sec
        }
    }

    pub async fn crawl(&self, start_url: &str) -> Result<Vec<SpiderResult>> {
        let start = Instant::now();
        eprintln!("🕷️ Nuclear Spider starting crawl from: {}", start_url);

        use std::collections::VecDeque;

        let mut queue: VecDeque<(String, usize)> = VecDeque::new();
        queue.push_back((start_url.to_string(), 0));

        while let Some((url, depth)) = queue.pop_front() {
            if depth > self.config.max_depth {
                continue;
            }

            // Respect page cap
            {
                let results = self.results.read().await;
                if results.len() >= self.config.max_pages {
                    break;
                }
            }

            // Skip already visited
            {
                let visited = self.visited.read().await;
                if visited.contains(&url) {
                    continue;
                }
            }

            // Mark visited
            {
                let mut visited = self.visited.write().await;
                visited.insert(url.clone());
            }

            // Crawl one page (stores its own result)
            let _ = self.crawl_recursive(&url, depth).await;

            // Enqueue new links from latest stored result (best-effort)
            let maybe_links = {
                let results = self.results.read().await;
                results
                    .iter()
                    .rev()
                    .find(|r| r.url == url)
                    .map(|r| r.links_found.clone())
                    .unwrap_or_default()
            };

            for link in maybe_links {
                queue.push_back((link, depth + 1));
            }
        }

        let results = self.results.read().await.clone();
        eprintln!(
            "🕷️ Nuclear Spider completed: {} pages in {:.2}s",
            results.len(),
            start.elapsed().as_secs_f32()
        );

        Ok(results)
    }

    async fn crawl_recursive(&self, url: &str, depth: usize) -> Result<()> {
        if depth > self.config.max_depth {
            return Ok(());
        }

        // Check if already visited
        {
            let visited = self.visited.read().await;
            if visited.contains(url) {
                return Ok(());
            }
        }

        // Check limits
        {
            let results = self.results.read().await;
            if results.len() >= self.config.max_pages {
                return Ok(());
            }
        }

        // Mark as visited
        {
            let mut visited = self.visited.write().await;
            visited.insert(url.to_string());
        }

        // Rate limiting and concealment
        self.rate_limiter.wait().await;
        let headers = self.concealment.get_headers(Some(url)).await;

        // Fetch page
        let client = reqwest::Client::new();
        let response = timeout(
            Duration::from_secs(30),
            client
                .get(url)
                .headers(
                    headers
                        .into_iter()
                        .map(|(k, v)| (k.parse().unwrap(), v.parse().unwrap()))
                        .collect::<reqwest::header::HeaderMap>(),
                )
                .send(),
        )
        .await??;

        let status_code = response.status().as_u16();
        let html = response.text().await.unwrap_or_default();
        let content_length = html.len();

        // Extract links
        let extracted = self
            .extractor
            .extract_all(&html, url)
            .await
            .unwrap_or_default();
        let links_found = extracted
            .links
            .into_iter()
            .filter(|link| self.is_allowed_link(link))
            .take(50) // Limit links per page
            .collect::<Vec<_>>();

        // Store result
        let result = SpiderResult {
            url: url.to_string(),
            depth,
            status_code,
            content_length,
            links_found: links_found.clone(),
            crawled_at: chrono::Utc::now(),
        };

        {
            let mut results = self.results.write().await;
            results.push(result);
        }

        // Recursion is handled by the queue in `crawl()`.

        Ok(())
    }

    fn is_allowed_link(&self, link: &str) -> bool {
        if self.config.allowed_domains.is_empty() {
            return true;
        }

        if let Some(domain) = self.extract_domain(link) {
            self.config.allowed_domains.contains(&domain)
        } else {
            false
        }
    }

    fn extract_domain(&self, url: &str) -> Option<String> {
        Some(url::Url::parse(url).ok()?.host_str()?.to_string())
    }
}

// ===== UTILITY COMPONENTS =====

pub struct RateLimiter {
    permits: usize,
    refill_rate: usize,
    available: Arc<RwLock<usize>>,
    last_refill: Arc<RwLock<Instant>>,
}

impl RateLimiter {
    pub fn new(permits: usize, refill_rate_per_second: usize) -> Self {
        Self {
            permits,
            refill_rate: refill_rate_per_second,
            available: Arc::new(RwLock::new(permits)),
            last_refill: Arc::new(RwLock::new(Instant::now())),
        }
    }

    pub async fn wait(&self) {
        loop {
            let mut available = self.available.write().await;
            let mut last_refill = self.last_refill.write().await;

            // Refill tokens
            let now = Instant::now();
            let elapsed = now.duration_since(*last_refill).as_secs_f64();
            let tokens_to_add = (elapsed * self.refill_rate as f64) as usize;

            if tokens_to_add > 0 {
                *available = (*available + tokens_to_add).min(self.permits);
                *last_refill = now;
            }

            if *available > 0 {
                *available -= 1;
                break;
            }

            // Wait a bit before checking again
            drop(available);
            drop(last_refill);
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
}

// ===== NUCLEAR CORE UNIFIED INTERFACE =====

pub struct NuclearCore {
    pub bypass: NuclearBypass,
    pub extractor: AdvancedExtractor,
    pub concealment: ExtremeConcealment,
    pub spider: NuclearSpider,
}

impl NuclearCore {
    pub fn new() -> Result<Self> {
        let concealment = ExtremeConcealment::new(Default::default());
        Ok(Self {
            bypass: NuclearBypass::new(Default::default(), Arc::new(concealment.clone()))?,
            extractor: AdvancedExtractor::new(),
            concealment,
            spider: NuclearSpider::new(Default::default()),
        })
    }

    /// 🔥 MAXIMUM POWER: Extract with bypass and concealment
    pub async fn extract_with_maximum_power(&self, url: &str) -> Result<ExtractedData> {
        // Apply concealment
        let headers = self.concealment.get_headers(Some(url)).await;

        // Try bypass first
        let access_url = if self.is_premium_site(url) {
            self.bypass.bypass(url).await?.access_url
        } else {
            url.to_string()
        };

        // Fetch with stealth
        let client_builder = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36");

        // Add additional stealth headers
        let mut header_map = reqwest::header::HeaderMap::new();
        header_map.insert(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"
                .parse()
                .unwrap(),
        );
        header_map.insert("Accept-Language", "en-US,en;q=0.5".parse().unwrap());
        header_map.insert("Accept-Encoding", "gzip, deflate".parse().unwrap());
        header_map.insert("DNT", "1".parse().unwrap());
        header_map.insert("Connection", "keep-alive".parse().unwrap());
        header_map.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());

        let client = client_builder.default_headers(header_map).build()?;
        let response = client
            .get(&access_url)
            .headers(
                headers
                    .into_iter()
                    .map(|(k, v)| (k.parse().unwrap(), v.parse().unwrap()))
                    .collect::<reqwest::header::HeaderMap>(),
            )
            .send()
            .await?;

        let html = response.text().await?;

        // Extract with all advanced methods
        self.extractor.extract_all(&html, &access_url).await
    }

    /// Check if site is premium/paywalled
    fn is_premium_site(&self, url: &str) -> bool {
        let premium_domains = [
            "wsj.com",
            "nytimes.com",
            "ft.com",
            "bloomberg.com",
            "washingtonpost.com",
            "economist.com",
            "forbes.com",
        ];

        premium_domains.iter().any(|domain| url.contains(domain))
    }

    /// 🔥 SPIDER CRAWL: Maximum power web crawling
    pub async fn spider_crawl_maximum_power(
        &self,
        start_url: &str,
        max_pages: usize,
    ) -> Result<Vec<SpiderResult>> {
        let config = SpiderConfig {
            max_pages,
            ..Default::default()
        };

        let spider = NuclearSpider::new(config);
        spider.crawl(start_url).await
    }
}

impl Default for NuclearCore {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
