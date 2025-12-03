//! 🔥 NUCLEAR BYPASS - Sistema de Evasión y Acceso Premium
//!
//! Módulo de bypass avanzado que usa:
//! - Go FFI para goroutines y procesamiento paralelo
//! - Zig FFI para operaciones de bajo nivel
//! - Stealth System para anti-detección
//! - Técnicas de bypass de paywalls
//! - Exploits legales para acceso a contenido

use anyhow::Result;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::go_integration::GoIntegration;
use crate::stealth::{StealthConfig, StealthSystem};
use crate::zig_integration::ZigIntegration;

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 CONFIGURACIÓN NUCLEAR BYPASS
// ═══════════════════════════════════════════════════════════════════════════

/// Configuración del sistema de bypass
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearBypassConfig {
    /// Usar integración Go
    pub use_go: bool,
    /// Usar integración Zig
    pub use_zig: bool,
    /// Timeout para requests
    pub timeout_secs: u64,
    /// Máximo de reintentos
    pub max_retries: u32,
    /// User Agent personalizado (None = rotativo)
    pub custom_user_agent: Option<String>,
    /// Bypass agresivo (más técnicas)
    pub aggressive_mode: bool,
    /// Usar cache de bypasses exitosos
    pub cache_successful_bypasses: bool,
}

impl Default for NuclearBypassConfig {
    fn default() -> Self {
        Self {
            use_go: true,
            use_zig: true,
            timeout_secs: 30,
            max_retries: 3,
            custom_user_agent: None,
            aggressive_mode: true,
            cache_successful_bypasses: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 TIPOS DE BYPASS
// ═══════════════════════════════════════════════════════════════════════════

/// Tipos de bypass disponibles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BypassType {
    /// Bypass de paywall de noticias
    NewsPaywall,
    /// Bypass de contenido académico
    AcademicPaywall,
    /// Bypass de JavaScript/AJAX
    JavaScriptBypass,
    /// Bypass de cookies/sesión
    CookieBypass,
    /// Bypass de Cloudflare
    CloudflareBypass,
    /// Bypass de CAPTCHA
    CaptchaBypass,
    /// Bypass de geolocalización
    GeoBypass,
    /// Bypass de rate limiting
    RateLimitBypass,
    /// Bypass genérico
    Generic,
}

/// Resultado de bypass
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BypassResult {
    /// URL original
    pub original_url: String,
    /// URL de acceso (puede ser diferente)
    pub access_url: String,
    /// Contenido extraído
    pub content: String,
    /// Tipo de bypass usado
    pub bypass_type: BypassType,
    /// Éxito
    pub success: bool,
    /// Mensaje/error
    pub message: String,
    /// Metadata adicional
    pub metadata: HashMap<String, String>,
    /// Tiempo de ejecución (ms)
    pub execution_time_ms: u64,
}

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 SISTEMA NUCLEAR BYPASS
// ═══════════════════════════════════════════════════════════════════════════

/// Sistema Nuclear de Bypass
pub struct NuclearBypass {
    config: NuclearBypassConfig,
    client: Client,
    stealth: StealthSystem,
    go_integration: GoIntegration,
    zig_integration: ZigIntegration,
    // Cache de URLs con bypass exitoso
    successful_bypasses: Arc<dashmap::DashMap<String, BypassType>>,
    // Patrones de paywall conocidos
    paywall_patterns: Vec<PaywallPattern>,
}

/// Patrón de paywall
struct PaywallPattern {
    domain_pattern: Regex,
    bypass_type: BypassType,
    bypass_methods: Vec<BypassMethod>,
}

/// Método de bypass
#[derive(Clone)]
#[allow(dead_code)]
enum BypassMethod {
    /// Agregar parámetro a URL
    AddUrlParam { key: String, value: String },
    /// Modificar Referer
    ModifyReferer(String),
    /// Usar Google Cache
    GoogleCache,
    /// Usar Archive.org
    ArchiveOrg,
    /// Usar 12ft.io
    TwelveFt,
    /// Usar Outline.com
    Outline,
    /// Modificar cookies
    ModifyCookies(HashMap<String, String>),
    /// Usar API alternativa
    AlternativeApi(String),
    /// Inyectar JavaScript (para scraping)
    InjectJs(String),
    /// Bypass específico de sitio
    SiteSpecific(String),
}

impl NuclearBypass {
    /// Crea nuevo sistema de bypass
    pub fn new(config: NuclearBypassConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .danger_accept_invalid_certs(true)
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()?;

        let stealth_config = StealthConfig {
            rotate_user_agents: true,
            rotate_headers: true,
            random_delay_min: 500,
            random_delay_max: 2000,
            human_behavior: true,
            use_proxies: false,
            proxies: vec![],
            avoid_headless_detection: true,
            tls_fingerprint_evasion: true,
        };

        let stealth = StealthSystem::new(stealth_config);
        let go_integration = GoIntegration::new_with_config(config.use_go);
        let zig_integration = ZigIntegration::new_with_config(config.use_zig);

        // Patrones de paywall conocidos
        let paywall_patterns = Self::init_paywall_patterns();

        Ok(Self {
            config,
            client,
            stealth,
            go_integration,
            zig_integration,
            successful_bypasses: Arc::new(dashmap::DashMap::new()),
            paywall_patterns,
        })
    }

    /// Inicializa patrones de paywall
    fn init_paywall_patterns() -> Vec<PaywallPattern> {
        vec![
            // ═══════════════════════════════════════════════════════════════
            // 🔥 NOTICIAS / MEDIOS
            // ═══════════════════════════════════════════════════════════════
            PaywallPattern {
                domain_pattern: Regex::new(r"(nytimes\.com|washingtonpost\.com|wsj\.com|ft\.com|economist\.com|bloomberg\.com|medium\.com)").unwrap(),
                bypass_type: BypassType::NewsPaywall,
                bypass_methods: vec![
                    BypassMethod::TwelveFt,
                    BypassMethod::Outline,
                    BypassMethod::GoogleCache,
                    BypassMethod::ArchiveOrg,
                    BypassMethod::ModifyReferer("https://www.google.com/".to_string()),
                    BypassMethod::ModifyReferer("https://t.co/".to_string()),
                    BypassMethod::ModifyReferer("https://facebook.com/".to_string()),
                ],
            },
            // ═══════════════════════════════════════════════════════════════
            // 🔥 ACADÉMICO / RESEARCH
            // ═══════════════════════════════════════════════════════════════
            PaywallPattern {
                domain_pattern: Regex::new(r"(ieee\.org|acm\.org|springer\.com|sciencedirect\.com|nature\.com|wiley\.com|jstor\.org|tandfonline\.com)").unwrap(),
                bypass_type: BypassType::AcademicPaywall,
                bypass_methods: vec![
                    // Sci-Hub mirrors
                    BypassMethod::AlternativeApi("https://sci-hub.se/".to_string()),
                    BypassMethod::AlternativeApi("https://sci-hub.st/".to_string()),
                    BypassMethod::AlternativeApi("https://sci-hub.ru/".to_string()),
                    // LibGen
                    BypassMethod::AlternativeApi("https://libgen.is/scimag/?q=".to_string()),
                    // Unpaywall
                    BypassMethod::AlternativeApi("https://api.unpaywall.org/v2/".to_string()),
                    // Core.ac.uk
                    BypassMethod::AlternativeApi("https://core.ac.uk/search?q=".to_string()),
                    // Semantic Scholar
                    BypassMethod::AlternativeApi("https://api.semanticscholar.org/".to_string()),
                    BypassMethod::ArchiveOrg,
                ],
            },
            // ═══════════════════════════════════════════════════════════════
            // 🔥 LIBROS / DOCUMENTOS
            // ═══════════════════════════════════════════════════════════════
            PaywallPattern {
                domain_pattern: Regex::new(r"(scribd\.com|issuu\.com|slideshare\.net|academia\.edu)").unwrap(),
                bypass_type: BypassType::Generic,
                bypass_methods: vec![
                    // Document downloaders
                    BypassMethod::AlternativeApi("https://docdownloader.com/".to_string()),
                    BypassMethod::GoogleCache,
                    BypassMethod::ArchiveOrg,
                    // Print mode
                    BypassMethod::AddUrlParam { key: "view".to_string(), value: "print".to_string() },
                ],
            },
            // ═══════════════════════════════════════════════════════════════
            // 🔥 CLOUDFLARE PROTECTION
            // ═══════════════════════════════════════════════════════════════
            PaywallPattern {
                domain_pattern: Regex::new(r".*").unwrap(), // Cualquier dominio puede tener CF
                bypass_type: BypassType::CloudflareBypass,
                bypass_methods: vec![
                    BypassMethod::ModifyCookies({
                        let mut cookies = HashMap::new();
                        cookies.insert("cf_clearance".to_string(), "bypass".to_string());
                        cookies
                    }),
                    // Usar navegador real headers
                    BypassMethod::SiteSpecific("cloudflare_scraper".to_string()),
                ],
            },
        ]
    }

    // ═══════════════════════════════════════════════════════════════════════
    // 🔥 BYPASS PRINCIPAL
    // ═══════════════════════════════════════════════════════════════════════

    /// Intenta bypass de una URL
    pub async fn bypass(&self, url: &str) -> Result<BypassResult> {
        let start = std::time::Instant::now();

        // Verificar cache
        if self.config.cache_successful_bypasses {
            if let Some(cached_type) = self.successful_bypasses.get(url) {
                // Usar método que funcionó antes
                return self.bypass_with_type(url, cached_type.clone()).await;
            }
        }

        // Detectar tipo de paywall
        let bypass_type = self.detect_paywall_type(url);

        // Intentar bypass
        let result = self.bypass_with_type(url, bypass_type).await?;

        // Cachear si exitoso
        if result.success && self.config.cache_successful_bypasses {
            self.successful_bypasses
                .insert(url.to_string(), result.bypass_type.clone());
        }

        Ok(BypassResult {
            execution_time_ms: start.elapsed().as_millis() as u64,
            ..result
        })
    }

    /// Bypass con tipo específico
    async fn bypass_with_type(&self, url: &str, bypass_type: BypassType) -> Result<BypassResult> {
        let mut last_error = String::new();

        // Encontrar patrones aplicables
        for pattern in &self.paywall_patterns {
            if pattern.domain_pattern.is_match(url) {
                // Intentar cada método
                for method in &pattern.bypass_methods {
                    match self.try_bypass_method(url, method).await {
                        Ok(content) if !content.is_empty() => {
                            return Ok(BypassResult {
                                original_url: url.to_string(),
                                access_url: url.to_string(),
                                content,
                                bypass_type: bypass_type.clone(),
                                success: true,
                                message: format!("Bypass exitoso con {:?}", method_name(method)),
                                metadata: HashMap::new(),
                                execution_time_ms: 0,
                            });
                        }
                        Err(e) => {
                            last_error = e.to_string();
                            continue;
                        }
                        _ => continue,
                    }
                }
            }
        }

        // Fallback: intento directo con stealth
        match self.direct_fetch_stealth(url).await {
            Ok(content) => Ok(BypassResult {
                original_url: url.to_string(),
                access_url: url.to_string(),
                content,
                bypass_type,
                success: true,
                message: "Acceso directo con stealth".to_string(),
                metadata: HashMap::new(),
                execution_time_ms: 0,
            }),
            Err(e) => Ok(BypassResult {
                original_url: url.to_string(),
                access_url: url.to_string(),
                content: String::new(),
                bypass_type,
                success: false,
                message: format!("Bypass fallido: {} | {}", last_error, e),
                metadata: HashMap::new(),
                execution_time_ms: 0,
            }),
        }
    }

    /// Detecta tipo de paywall
    fn detect_paywall_type(&self, url: &str) -> BypassType {
        for pattern in &self.paywall_patterns {
            if pattern.domain_pattern.is_match(url) {
                return pattern.bypass_type.clone();
            }
        }
        BypassType::Generic
    }

    /// Intenta un método de bypass específico
    async fn try_bypass_method(&self, url: &str, method: &BypassMethod) -> Result<String> {
        match method {
            BypassMethod::TwelveFt => {
                let bypass_url = format!("https://12ft.io/{}", url);
                self.fetch_with_stealth(&bypass_url).await
            }
            BypassMethod::Outline => {
                let bypass_url = format!("https://outline.com/{}", url);
                self.fetch_with_stealth(&bypass_url).await
            }
            BypassMethod::GoogleCache => {
                let bypass_url = format!(
                    "https://webcache.googleusercontent.com/search?q=cache:{}",
                    urlencoding::encode(url)
                );
                self.fetch_with_stealth(&bypass_url).await
            }
            BypassMethod::ArchiveOrg => {
                // Buscar versión más reciente en Archive.org
                let api_url = format!(
                    "https://archive.org/wayback/available?url={}",
                    urlencoding::encode(url)
                );
                let response = self.fetch_with_stealth(&api_url).await?;

                // Parsear respuesta para obtener URL del archivo
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&response) {
                    if let Some(snapshot) = json["archived_snapshots"]["closest"]["url"].as_str() {
                        return self.fetch_with_stealth(snapshot).await;
                    }
                }
                Err(anyhow::anyhow!("No archive found"))
            }
            BypassMethod::AlternativeApi(base_url) => {
                // Para Sci-Hub y similares
                let bypass_url = if base_url.contains("sci-hub") {
                    format!("{}{}", base_url, url)
                } else if base_url.contains("unpaywall") {
                    // Unpaywall necesita DOI
                    if let Some(doi) = self.extract_doi(url) {
                        format!("{}{}?email=bypass@nuclear.dev", base_url, doi)
                    } else {
                        return Err(anyhow::anyhow!("No DOI found"));
                    }
                } else {
                    format!("{}{}", base_url, urlencoding::encode(url))
                };
                self.fetch_with_stealth(&bypass_url).await
            }
            BypassMethod::ModifyReferer(referer) => self.fetch_with_referer(url, referer).await,
            BypassMethod::AddUrlParam { key, value } => {
                let separator = if url.contains('?') { "&" } else { "?" };
                let bypass_url = format!("{}{}{}={}", url, separator, key, value);
                self.fetch_with_stealth(&bypass_url).await
            }
            BypassMethod::ModifyCookies(cookies) => self.fetch_with_cookies(url, cookies).await,
            BypassMethod::SiteSpecific(handler) => self.site_specific_bypass(url, handler).await,
            BypassMethod::InjectJs(_js) => {
                // JavaScript injection requiere headless browser
                // Por ahora, fallback a fetch normal
                self.fetch_with_stealth(url).await
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // 🔥 MÉTODOS DE FETCH CON STEALTH
    // ═══════════════════════════════════════════════════════════════════════

    /// Fetch con sistema stealth completo
    async fn fetch_with_stealth(&self, url: &str) -> Result<String> {
        // Usar Go si disponible para headers stealth
        let stealth_headers = if self.config.use_go {
            self.go_integration.get_stealth_headers()?
        } else {
            crate::go_integration::StealthHeadersGo::default()
        };

        let user_agent = self
            .config
            .custom_user_agent
            .clone()
            .unwrap_or_else(|| self.stealth.get_user_agent());

        let headers = self.stealth.get_headers(None);
        let anti_detection = self.stealth.get_anti_detection_headers();

        let mut request = self.client.get(url).header("User-Agent", &user_agent);

        // Agregar headers stealth
        for (k, v) in headers {
            request = request.header(&k, &v);
        }
        for (k, v) in anti_detection {
            request = request.header(&k, &v);
        }
        for (k, v) in stealth_headers.headers {
            request = request.header(&k, &v);
        }

        // Delay humano
        if self.stealth.get_human_delay() > 0 {
            tokio::time::sleep(Duration::from_millis(self.stealth.get_human_delay())).await;
        }

        let response = request.send().await?;
        let body = response.text().await?;

        // Usar Zig para procesar si está disponible
        if self.config.use_zig {
            let processed = self
                .zig_integration
                .process_data_parallel(body.as_bytes())?;
            return Ok(String::from_utf8_lossy(&processed).to_string());
        }

        Ok(body)
    }

    /// Fetch con referer específico
    async fn fetch_with_referer(&self, url: &str, referer: &str) -> Result<String> {
        let user_agent = self.stealth.get_user_agent();
        let headers = self.stealth.get_headers(None);

        let mut request = self
            .client
            .get(url)
            .header("User-Agent", &user_agent)
            .header("Referer", referer);

        for (k, v) in headers {
            request = request.header(&k, &v);
        }

        let response = request.send().await?;
        Ok(response.text().await?)
    }

    /// Fetch con cookies específicas
    async fn fetch_with_cookies(
        &self,
        url: &str,
        cookies: &HashMap<String, String>,
    ) -> Result<String> {
        let user_agent = self.stealth.get_user_agent();

        let cookie_string: String = cookies
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("; ");

        let response = self
            .client
            .get(url)
            .header("User-Agent", &user_agent)
            .header("Cookie", cookie_string)
            .send()
            .await?;

        Ok(response.text().await?)
    }

    /// Fetch directo con stealth máximo
    async fn direct_fetch_stealth(&self, url: &str) -> Result<String> {
        for retry in 0..self.config.max_retries {
            match self.fetch_with_stealth(url).await {
                Ok(content) => return Ok(content),
                Err(e) => {
                    if retry < self.config.max_retries - 1 {
                        // Esperar antes de reintentar
                        tokio::time::sleep(Duration::from_millis(1000 * (retry as u64 + 1))).await;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        Err(anyhow::anyhow!("Max retries exceeded"))
    }

    /// Bypass específico por sitio
    async fn site_specific_bypass(&self, url: &str, handler: &str) -> Result<String> {
        match handler {
            "cloudflare_scraper" => {
                // Técnicas específicas para Cloudflare
                // 1. Intentar con headers de navegador real
                let headers = self.stealth.get_anti_detection_headers();
                let mut request = self.client.get(url);

                for (k, v) in headers {
                    request = request.header(&k, &v);
                }

                // 2. Simular comportamiento humano
                tokio::time::sleep(Duration::from_millis(2000)).await;

                let response = request.send().await?;

                if response.status().is_success() {
                    return Ok(response.text().await?);
                }

                Err(anyhow::anyhow!("Cloudflare bypass failed"))
            }
            _ => Err(anyhow::anyhow!("Unknown handler: {}", handler)),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // 🔥 UTILIDADES
    // ═══════════════════════════════════════════════════════════════════════

    /// Extrae DOI de URL
    fn extract_doi(&self, url: &str) -> Option<String> {
        let doi_re = Regex::new(r"10\.\d{4,}/[^\s]+").ok()?;
        doi_re.find(url).map(|m| m.as_str().to_string())
    }

    /// Bypass masivo de múltiples URLs
    pub async fn bypass_batch(&self, urls: Vec<String>) -> Vec<BypassResult> {
        use futures::future::join_all;

        let futures: Vec<_> = urls.iter().map(|url| self.bypass(url)).collect();

        let results = join_all(futures).await;

        results
            .into_iter()
            .map(|r| {
                r.unwrap_or_else(|e| BypassResult {
                    original_url: String::new(),
                    access_url: String::new(),
                    content: String::new(),
                    bypass_type: BypassType::Generic,
                    success: false,
                    message: e.to_string(),
                    metadata: HashMap::new(),
                    execution_time_ms: 0,
                })
            })
            .collect()
    }

    /// Estadísticas de bypass
    pub fn get_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert(
            "cached_bypasses".to_string(),
            self.successful_bypasses.len(),
        );
        stats.insert("paywall_patterns".to_string(), self.paywall_patterns.len());
        stats
    }
}

/// Helper para nombre de método
fn method_name(method: &BypassMethod) -> &'static str {
    match method {
        BypassMethod::TwelveFt => "12ft.io",
        BypassMethod::Outline => "Outline",
        BypassMethod::GoogleCache => "Google Cache",
        BypassMethod::ArchiveOrg => "Archive.org",
        BypassMethod::AlternativeApi(_) => "Alternative API",
        BypassMethod::ModifyReferer(_) => "Referer Bypass",
        BypassMethod::AddUrlParam { .. } => "URL Param",
        BypassMethod::ModifyCookies(_) => "Cookie Bypass",
        BypassMethod::InjectJs(_) => "JS Injection",
        BypassMethod::SiteSpecific(_) => "Site Specific",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nuclear_bypass() {
        let config = NuclearBypassConfig::default();
        let bypass = NuclearBypass::new(config).unwrap();

        let stats = bypass.get_stats();
        assert!(stats["paywall_patterns"] > 0);
    }
}
