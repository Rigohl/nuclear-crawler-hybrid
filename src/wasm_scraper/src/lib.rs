use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::OnceLock;
use regex::Regex;
use scraper::{Html, Selector};
use anyhow::Result;

// ═══════════════════════════════════════════════════════════════════════════
// DATA STRUCTURES FOR AI TRAINING
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScrapedData {
    pub main_content: String,       // Clean text for LLM training
    pub title: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub author: Option<String>,
    pub date_published: Option<String>,
    pub language: Option<String>,
    pub metadata: HashMap<String, String>, // OG tags, Twitter cards, JSON-LD

    // Legacy fields
    pub phones: Vec<String>,
    pub emails: Vec<String>,
    pub social_profiles: Vec<SocialProfile>,
    pub links: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SocialProfile {
    pub platform: String,
    pub username: String,
    pub url: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// GLOBAL OPTIMIZED REGEXES
// ═══════════════════════════════════════════════════════════════════════════

static WHITESPACE_REGEX: OnceLock<Regex> = OnceLock::new();
static PHONE_REGEX: OnceLock<Regex> = OnceLock::new();
static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

// ═══════════════════════════════════════════════════════════════════════════
// WASM BINDGEN INTERFACE
// ═══════════════════════════════════════════════════════════════════════════

#[wasm_bindgen]
pub struct NuclearScraper {
}

#[wasm_bindgen]
impl NuclearScraper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
        }
    }

    /// High-quality scraping optimized for AI datasets
    #[wasm_bindgen]
    pub fn scrape_html(&self, html: &str, base_url: &str) -> Result<JsValue, JsValue> {
        let data = self.scrape_internal(html, base_url).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(serde_wasm_bindgen::to_value(&data)?)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// INTERNAL LOGIC (Rust)
// ═══════════════════════════════════════════════════════════════════════════

impl NuclearScraper {
    pub fn scrape_internal(&self, html_content: &str, _base_url: &str) -> Result<ScrapedData> {
        let document = Html::parse_document(html_content);
        
        // --- Metadata Extraction ---
        let mut title = String::new();
        if let Some(el) = document.select(&Selector::parse("title").unwrap()).next() {
            title = el.text().collect::<Vec<_>>().join("");
        }

        let mut description = String::new();
        let mut keywords = Vec::new();
        let mut author = None;
        let mut date_published = None;
        let mut language = None;
        let mut metadata = HashMap::new();

        // Parse meta tags
        let meta_selector = Selector::parse("meta").unwrap();
        for element in document.select(&meta_selector) {
            let name = element.value().attr("name").or_else(|| element.value().attr("property")).unwrap_or("");
            let content = element.value().attr("content").unwrap_or("");

            if !name.is_empty() && !content.is_empty() {
                metadata.insert(name.to_string(), content.to_string());

                match name {
                    "description" | "og:description" => description = content.to_string(),
                    "keywords" => keywords = content.split(',').map(|s| s.trim().to_string()).collect(),
                    "author" | "article:author" => author = Some(content.to_string()),
                    "article:published_time" => date_published = Some(content.to_string()),
                    _ => {}
                }
            }
        }
        
        // Language from HTML tag
        if let Some(html_tag) = document.select(&Selector::parse("html").unwrap()).next() {
            if let Some(lang) = html_tag.value().attr("lang") {
                language = Some(lang.to_string());
            }
        }

        // --- Content Extraction (Simulated Readability) ---
        // We prioritize article, main, or failing that, div with specific classes, or just paragraphs
        let mut main_content = String::new();
        
        // Strategy 1: <article> tag
        let article_selector = Selector::parse("article").unwrap();
        let mut found_content = false;
        
        for element in document.select(&article_selector) {
            let text = element.text().collect::<Vec<_>>().join(" ");
            if text.len() > 100 { // meaningful content check
                main_content.push_str(&text);
                main_content.push(' ');
                found_content = true;
            }
        }
        
        // Strategy 2: If no article, grab all <p> tags inside <body>
        if !found_content {
             let p_selector = Selector::parse("body p").unwrap();
             for element in document.select(&p_selector) {
                 let text = element.text().collect::<Vec<_>>().join(" ");
                 if !text.trim().is_empty() {
                     main_content.push_str(&text);
                     main_content.push(' ');
                 }
             }
        }
        
        main_content = self.clean_text(&main_content);

        // --- Links & Socials ---
        let mut links = Vec::new();
        let mut social_profiles = Vec::new();
        
        let a_selector = Selector::parse("a[href]").unwrap();
        for element in document.select(&a_selector) {
            if let Some(href) = element.value().attr("href") {
                if !href.starts_with("javascript:") && !href.starts_with("#") {
                    links.push(href.to_string());
                    if let Some(profile) = self.detect_social(href) {
                        social_profiles.push(profile);
                    }
                }
            }
        }

        // --- Regex Scan for Contact Info ---
        // We scan the raw HTML or cleaned content? Raw is safer for hidden emails
        let emails = self.extract_emails(html_content);
        let phones = self.extract_phones(html_content);

        // Dedup
        links.sort(); links.dedup();
        // social_profiles dedup logic omitted for brevity

        Ok(ScrapedData {
            main_content,
            title,
            description,
            keywords,
            author,
            date_published,
            language,
            metadata,
            phones,
            emails,
            social_profiles,
            links,
        })
    }

    fn clean_text(&self, text: &str) -> String {
        // Normalize whitespace: remove newlines, tabs, multiple spaces
        let re = WHITESPACE_REGEX.get_or_init(|| Regex::new(r"\s+").unwrap());
        re.replace_all(text.trim(), " ").to_string()
    }

    fn extract_emails(&self, text: &str) -> Vec<String> {
        let re = EMAIL_REGEX.get_or_init(|| Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap());
        let mut emails: Vec<String> = re.find_iter(text)
            .map(|m| m.as_str().to_string())
            .collect();
        emails.sort();
        emails.dedup();
        emails
    }

    fn extract_phones(&self, text: &str) -> Vec<String> {
        let re = PHONE_REGEX.get_or_init(|| Regex::new(r"(?:\+?\d{1,3}[-.\s]?)?(?:\(?\d{1,4}\)?[-.\s]?)?[\d\s.-]{7,15}").unwrap());
        let mut phones: Vec<String> = re.find_iter(text)
            .map(|m| m.as_str().trim().to_string())
            .filter(|p| p.len() >= 7)
            .collect();
        phones.sort();
        phones.dedup();
        phones
    }

    fn detect_social(&self, url: &str) -> Option<SocialProfile> {
        let url_lower = url.to_lowercase();
        let mut platform = String::new();
        
        if url_lower.contains("facebook.com") { platform = "Facebook".to_string(); }
        else if url_lower.contains("twitter.com") || url_lower.contains("x.com") { platform = "X/Twitter".to_string(); }
        else if url_lower.contains("linkedin.com/in/") { platform = "LinkedIn".to_string(); }
        else if url_lower.contains("instagram.com") { platform = "Instagram".to_string(); }
        else if url_lower.contains("t.me") { platform = "Telegram".to_string(); }
        else if url_lower.contains("github.com") { platform = "GitHub".to_string(); }
        else if url_lower.contains("medium.com") { platform = "Medium".to_string(); }
        
        if !platform.is_empty() {
             Some(SocialProfile { platform, username: "unknown".into(), url: url.into() })
        } else {
            None
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// NATIVE / WASMTIME INTERFACE (Extern "C")
// ═══════════════════════════════════════════════════════════════════════════

#[no_mangle]
pub extern "C" fn alloc_bytes(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn dealloc_bytes(ptr: *mut u8, len: usize) {
    let _ = Vec::from_raw_parts(ptr, len, len);
}

#[no_mangle]
pub unsafe extern "C" fn scrape_native(ptr: *mut u8, len: usize) -> *mut u8 {
    let html_slice = std::slice::from_raw_parts(ptr, len);
    let html = String::from_utf8_lossy(html_slice).into_owned();
    dealloc_bytes(ptr, len);

    let scraper = NuclearScraper::new();
    let result_data = scraper.scrape_internal(&html, "unknown")
        .unwrap_or_else(|_| ScrapedData {
            main_content: String::new(), title: String::new(), description: String::new(),
            keywords: vec![], author: None, date_published: None, language: None, metadata: HashMap::new(),
            phones: vec![], emails: vec![], social_profiles: vec![], links: vec![]
        });

    let json_bytes = serde_json::to_vec(&result_data).unwrap();
    let result_len = json_bytes.len() as u64;
    
    let mut out_vec = Vec::with_capacity(8 + json_bytes.len());
    out_vec.extend_from_slice(&result_len.to_le_bytes());
    out_vec.extend_from_slice(&json_bytes);

    let out_ptr = out_vec.as_mut_ptr();
    std::mem::forget(out_vec);
    out_ptr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrape_internal_basic() {
        let scraper = NuclearScraper::new();
        let html = r#"
            <html>
                <head>
                    <title>Test Page</title>
                    <meta name="description" content="A test page description">
                    <meta name="keywords" content="test, page, scraper">
                </head>
                <body>
                    <article>
                        <h1>Main Content</h1>
                        <p>This is the main content of the page. It has some text.</p>
                        <a href="https://example.com">Example Link</a>
                        <a href="mailto:test@example.com">Email Link</a>
                    </article>
                    <footer>Phone: +1-555-0199</footer>
                </body>
            </html>
        "#;

        let result = scraper.scrape_internal(html, "http://test.com").unwrap();

        assert_eq!(result.title, "Test Page");
        assert_eq!(result.description, "A test page description");
        assert!(result.keywords.contains(&"test".to_string()));
        assert!(result.main_content.contains("Main Content"));
        assert!(result.main_content.contains("This is the main content"));
        assert!(result.links.contains(&"https://example.com".to_string()));
        assert!(result.emails.contains(&"test@example.com".to_string()));
        assert!(result.phones.contains(&"+1-555-0199".to_string()));
    }
}
