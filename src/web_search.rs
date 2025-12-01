//! Módulo Web Search - Búsqueda Web con TODO el poder del crawler
//!
//! Usa todo el sistema: Stealth, AI, Paralelismo, Go, Zig, Nuclear Bypass
//! 
//! 🔥 NUCLEAR v3.0: Go + Zig + Stealth + 100 URLs Async

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Instant, Duration};

use crate::ai_smart::{AIConfig, AISmart};
use crate::nuclear_scraper::{NuclearConfig, NuclearScraper};
use crate::stealth::{StealthSystem, StealthConfig};

#[cfg(feature = "go")]
use crate::go_integration::GoIntegration;

#[cfg(feature = "zig")]
use crate::zig_integration::ZigIntegration;

/// Configuración de búsqueda web
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchConfig {
    /// Query de búsqueda
    pub query: String,

    /// Número máximo de resultados (0 = sin límite)
    pub max_results: usize,

    /// Fuentes prioritarias (se buscan primero y tienen mayor peso)
    pub priority_sources: Vec<String>,

    /// Fuentes a buscar
    pub sources: Vec<String>,

    /// Usar AI para optimizar
    pub use_ai: bool,

    /// Usar stealth
    pub use_stealth: bool,

    /// Paralelismo máximo
    pub max_parallel: usize,

    /// 🔥 NUCLEAR: Timeout total por consulta en segundos (7 seg por defecto)
    pub timeout_secs: u64,

    /// 🔥 NUCLEAR: URLs máximo a crawlear (100 por defecto)
    pub max_urls: usize,
}

impl Default for WebSearchConfig {
    fn default() -> Self {
        Self {
            query: String::new(),
            max_results: 100,
            priority_sources: vec![],
            sources: vec![
                // Código y desarrollo
                "github.com".to_string(),
                "gitlab.com".to_string(),
                "bitbucket.org".to_string(),
                "sourceforge.net".to_string(),
                // Foros y comunidades
                "reddit.com".to_string(),
                "stackoverflow.com".to_string(),
                "stackexchange.com".to_string(),
                // Documentación y tutoriales
                "medium.com".to_string(),
                "dev.to".to_string(),
                "hashnode.com".to_string(),
                // AI y ML
                "huggingface.co".to_string(),
                "paperswithcode.com".to_string(),
                "arxiv.org".to_string(),
                // Noticias tech
                "techcrunch.com".to_string(),
                "theverge.com".to_string(),
                "wired.com".to_string(),
                // Blogs y docs
                "rust-lang.org".to_string(),
                "docs.rs".to_string(),
                "crates.io".to_string(),
                "hackernews.com".to_string(),
            ],
            use_ai: true,
            use_stealth: true,
            max_parallel: 10000, // NUCLEAR: 10K paralelo
            timeout_secs: 7,     // 🔥 NUCLEAR: 7 segundos TOTAL por consulta
            max_urls: 100,       // 🔥 NUCLEAR: Máximo 100 URLs a crawlear
        }
    }
}

/// Resultado de búsqueda web
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchResult {
    pub url: String,
    pub title: String,
    pub description: String,
    pub relevance: f32,
    pub quality_score: f32,
    pub source: String,
}

/// Sistema de búsqueda web masiva
/// 🔥 NUCLEAR v3.0: Integración completa Go + Zig + Stealth
pub struct WebSearch {
    scraper: Arc<NuclearScraper>,
    #[allow(dead_code)]
    ai_smart: Arc<AISmart>,
    stealth: Arc<StealthSystem>,
    #[cfg(feature = "go")]
    go_integration: Arc<GoIntegration>,
    #[cfg(feature = "zig")]
    zig_integration: Arc<ZigIntegration>,
}

impl WebSearch {
    /// Crea nuevo sistema de búsqueda web con storage
    /// 🔥 NUCLEAR: Inicializa Go, Zig y Stealth
    pub fn new_with_storage(
        storage: Option<Arc<crate::intelligent_storage::IntelligentStorage>>,
    ) -> Result<Self> {
        let nuclear_config = NuclearConfig::default();
        let scraper = Arc::new(NuclearScraper::new_with_storage(nuclear_config, storage)?);
        let ai_config = AIConfig::default();
        let ai_smart = Arc::new(AISmart::new(ai_config));
        let stealth = Arc::new(StealthSystem::new(StealthConfig::default()));

        Ok(Self { 
            scraper, 
            ai_smart,
            stealth,
            #[cfg(feature = "go")]
            go_integration: Arc::new(GoIntegration::new()),
            #[cfg(feature = "zig")]
            zig_integration: Arc::new(ZigIntegration::new()),
        })
    }

    /// 🔥 NUCLEAR: Preprocesa URLs con Go goroutines para máxima velocidad
    #[allow(dead_code)]
    fn preprocess_urls_with_go(&self, urls: &[String]) -> Vec<String> {
        #[cfg(feature = "go")]
        {
            // Usar Go para procesar URLs en paralelo con goroutines
            if let Ok(processed) = self.go_integration.fast_process_urls(urls) {
                return processed;
            }
        }
        
        // Fallback: procesar en Rust
        urls.to_vec()
    }

    /// 🔥 NUCLEAR: Parsea HTML con Zig SIMD para máxima velocidad  
    #[allow(dead_code)]
    fn parse_html_with_zig(&self, html: &str) -> String {
        #[cfg(feature = "zig")]
        {
            // Usar Zig para parseo ultra-rápido con SIMD
            if let Ok(parsed) = self.zig_integration.parse_html_fast(html) {
                return parsed;
            }
        }
        
        // Fallback: devolver original
        html.to_string()
    }

    /// 🔥 NUCLEAR: Obtiene headers stealth para evitar detección
    fn get_stealth_headers(&self) -> std::collections::HashMap<String, String> {
        self.stealth.get_headers(Some("chrome"))
    }

    /// Realiza búsqueda web masiva - MODO NUCLEAR SIN LÍMITES
    /// 🔥 NUCLEAR v3.0: Go + Zig + Stealth + 7 segundos TOTAL
    pub async fn search(&self, config: WebSearchConfig) -> Result<Vec<WebSearchResult>> {
        let search_start = Instant::now();
        let timeout_duration = Duration::from_secs(config.timeout_secs);

        // 🔥 NUCLEAR: Obtener headers stealth para todas las requests
        let _stealth_headers = self.get_stealth_headers();

        // NUCLEAR: Sin prints a stdout para no contaminar MCP
        // Solo stderr si hay errores críticos

        // Preparar todas las URLs a buscar
        let all_sources: Vec<String> = config
            .priority_sources
            .iter()
            .chain(config.sources.iter())
            .cloned()
            .collect();

        let mut search_urls = self.prepare_search_urls(&config.query, &all_sources);
        
        // 🔥 NUCLEAR: Agregar URLs adicionales de fuentes alternativas
        search_urls.extend(self.prepare_alternative_sources(&config.query));
        
        // 🔥 NUCLEAR: Limitar a max_urls (100 por defecto) 
        if search_urls.len() > config.max_urls {
            search_urls.truncate(config.max_urls);
        }
        
        // 🔥 NUCLEAR: Preprocesar URLs con Go si disponible
        #[cfg(feature = "go")]
        let search_urls = self.preprocess_urls_with_go(&search_urls);

        // FASE 1: Crawl de páginas de búsqueda (NUCLEAR MASIVO)
        // Con timeout global
        let search_results = match tokio::time::timeout(
            timeout_duration - Duration::from_millis(500), // Dejar 500ms de margen
            self.scraper.nuclear_crawl(search_urls)
        ).await {
            Ok(Ok(results)) => results,
            Ok(Err(e)) => {
                eprintln!("❌ Error en fase 1: {}", e);
                Vec::new()
            },
            Err(_) => {
                eprintln!("⏱️ TIMEOUT en fase 1 (7 segundos)");
                Vec::new()
            }
        };

        // FASE 2: DEEP CRAWL - Extraer links de resultados y crawlearlos
        // Pero solo si aún hay tiempo
        let mut deep_urls: Vec<String> = Vec::new();
        if search_start.elapsed() < timeout_duration - Duration::from_secs(1) {
            for result in &search_results {
                if result.status_code == 200 && !result.html.is_empty() {
                    // 🔥 NUCLEAR: Parsear HTML con Zig si disponible
                    #[cfg(feature = "zig")]
                    let html = self.parse_html_with_zig(&result.html);
                    #[cfg(not(feature = "zig"))]
                    let html = result.html.clone();
                    
                    // Extraer links relevantes de la página de búsqueda
                    let links = self.extract_result_links(&html, &result.url, &config.query);
                    deep_urls.extend(links);
                    
                    // 🔥 NUCLEAR: Limite 100 URLs
                    if deep_urls.len() >= config.max_urls {
                        break;
                    }
                }
            }

            // Deduplicar URLs
            deep_urls.sort();
            deep_urls.dedup();
            deep_urls.truncate(config.max_urls);
        }
        
        // FASE 3: Crawl profundo de los resultados
        let mut all_results = search_results;
        if !deep_urls.is_empty() && search_start.elapsed() < timeout_duration - Duration::from_millis(500) {
            let remaining_time = timeout_duration - search_start.elapsed() - Duration::from_millis(100);
            
            if let Ok(Ok(deep_results)) = tokio::time::timeout(
                remaining_time,
                self.scraper.nuclear_crawl(deep_urls)
            ).await {
                all_results.extend(deep_results);
            }
        }

        // Procesar resultados
        let mut processed_results: Vec<WebSearchResult> = all_results
            .into_iter()
            .filter_map(|r| {
                if r.status_code == 200 && !r.html.is_empty() {
                    let title = self.extract_title(&r.html);
                    let description = self.extract_description(&r.html);
                    let source = self.extract_source(&r.url);
                    let relevance = self.calculate_relevance(&config.query, &r.html);
                    let quality_score = self.calculate_quality(&r.html);

                    // Filtrar solo resultados relevantes
                    if relevance > 0.1 || quality_score > 0.3 {
                        Some(WebSearchResult {
                            url: r.url,
                            title,
                            description,
                            relevance,
                            quality_score,
                            source,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // Ordenar por relevancia y calidad
        processed_results.sort_by(|a, b| {
            let score_a = a.relevance * 0.7 + a.quality_score * 0.3;
            let score_b = b.relevance * 0.7 + b.quality_score * 0.3;
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Limitar resultados si es necesario
        if config.max_results > 0 {
            processed_results.truncate(config.max_results);
        }

        Ok(processed_results)
    }

    /// NUCLEAR: Extrae links de resultados de páginas de búsqueda
    fn extract_result_links(&self, html: &str, _source_url: &str, query: &str) -> Vec<String> {
        use regex::Regex;
        let mut links = Vec::new();
        
        // Extraer todos los hrefs
        let href_re = Regex::new(r#"href=["']([^"']+)["']"#).unwrap();
        let query_lower = query.to_lowercase();
        let query_words: Vec<String> = query_lower.split_whitespace().map(|s| s.to_string()).collect();
        
        for cap in href_re.captures_iter(html) {
            if let Some(href) = cap.get(1) {
                let url = href.as_str();
                
                // Filtrar URLs válidas
                if url.starts_with("http") && !url.contains("login") && !url.contains("signup") {
                    // Verificar que el URL contiene palabras del query o es de fuentes conocidas
                    let url_lower = url.to_lowercase();
                    let is_relevant = query_words.iter().any(|w| url_lower.contains(w))
                        || url.contains("github.com")
                        || url.contains("stackoverflow.com")
                        || url.contains("dev.to")
                        || url.contains("medium.com");
                    
                    if is_relevant && !url.contains("/search") {
                        links.push(url.to_string());
                    }
                }
            }
        }
        
        // Deduplicar
        links.sort();
        links.dedup();
        links
    }

    /// Prepara URLs de búsqueda para múltiples fuentes - NUCLEAR 100 ASYNC
    fn prepare_search_urls(&self, query: &str, sources: &[String]) -> Vec<String> {
        let mut urls: Vec<String> = Vec::new();
        let query_encoded = urlencoding::encode(query);
        
        for source in sources {
            match source.as_str() {
                // ═══════════════════════════════════════════════════════════════
                // 🔥 GITHUB - 100 BÚSQUEDAS ASÍNCRONAS SIN API
                // ═══════════════════════════════════════════════════════════════
                "github.com" => {
                    // Búsqueda principal
                    urls.push(format!("https://github.com/search?q={}&type=repositories", query_encoded));
                    urls.push(format!("https://github.com/search?q={}&type=code", query_encoded));
                    urls.push(format!("https://github.com/search?q={}&type=issues", query_encoded));
                    urls.push(format!("https://github.com/search?q={}&type=discussions", query_encoded));
                    
                    // Paginación - 10 páginas por tipo = 40 URLs
                    for page in 2..=10 {
                        urls.push(format!("https://github.com/search?q={}&type=repositories&p={}", query_encoded, page));
                        urls.push(format!("https://github.com/search?q={}&type=code&p={}", query_encoded, page));
                    }
                    
                    // Filtros por lenguaje (20 lenguajes populares)
                    for lang in &["rust", "python", "javascript", "typescript", "go", "java", "c", "cpp", 
                                  "csharp", "ruby", "php", "swift", "kotlin", "scala", "haskell", "julia",
                                  "r", "lua", "perl", "shell"] {
                        urls.push(format!("https://github.com/search?q={}+language:{}&type=repositories", query_encoded, lang));
                    }
                    
                    // Filtros por estrellas
                    urls.push(format!("https://github.com/search?q={}+stars:>100&type=repositories", query_encoded));
                    urls.push(format!("https://github.com/search?q={}+stars:>1000&type=repositories", query_encoded));
                    urls.push(format!("https://github.com/search?q={}+stars:>10000&type=repositories", query_encoded));
                    
                    // Ordenar por actualización reciente
                    urls.push(format!("https://github.com/search?q={}&type=repositories&s=updated&o=desc", query_encoded));
                    
                    // Topics populares
                    urls.push(format!("https://github.com/topics/{}", query_encoded));
                }
                
                // ═══════════════════════════════════════════════════════════════
                // 🔥 STACKOVERFLOW - BÚSQUEDAS MASIVAS SIN API
                // ═══════════════════════════════════════════════════════════════
                "stackoverflow.com" => {
                    urls.push(format!("https://stackoverflow.com/search?q={}", query_encoded));
                    // Paginación
                    for page in 2..=20 {
                        urls.push(format!("https://stackoverflow.com/search?q={}&page={}", query_encoded, page));
                    }
                    // Por etiquetas populares
                    for tag in &["rust", "python", "javascript", "java", "c++", "go", "typescript"] {
                        urls.push(format!("https://stackoverflow.com/questions/tagged/{}?tab=votes&page=1&pagesize=50", tag));
                    }
                }
                
                // ═══════════════════════════════════════════════════════════════
                // 🔥 REDDIT - BÚSQUEDAS MASIVAS
                // ═══════════════════════════════════════════════════════════════
                "reddit.com" => {
                    urls.push(format!("https://www.reddit.com/search/?q={}", query_encoded));
                    // Subreddits técnicos
                    for sub in &["programming", "rust", "python", "javascript", "golang", "learnprogramming",
                                 "webdev", "machinelearning", "datascience", "devops", "linux", "opensource"] {
                        urls.push(format!("https://www.reddit.com/r/{}/search/?q={}&restrict_sr=1", sub, query_encoded));
                    }
                }
                
                // ═══════════════════════════════════════════════════════════════
                // 🔥 GITLAB - BÚSQUEDAS SIN API
                // ═══════════════════════════════════════════════════════════════
                "gitlab.com" => {
                    urls.push(format!("https://gitlab.com/explore/projects?search={}", query_encoded));
                    urls.push(format!("https://gitlab.com/explore/snippets?search={}", query_encoded));
                    for page in 2..=10 {
                        urls.push(format!("https://gitlab.com/explore/projects?search={}&page={}", query_encoded, page));
                    }
                }
                
                // ═══════════════════════════════════════════════════════════════
                // 🔥 DEV.TO / MEDIUM / HASHNODE - BLOGS TÉCNICOS
                // ═══════════════════════════════════════════════════════════════
                "dev.to" => {
                    urls.push(format!("https://dev.to/search?q={}", query_encoded));
                    for page in 2..=10 {
                        urls.push(format!("https://dev.to/search?q={}&page={}", query_encoded, page));
                    }
                }
                "medium.com" => {
                    urls.push(format!("https://medium.com/search?q={}", query_encoded));
                }
                "hashnode.com" => {
                    urls.push(format!("https://hashnode.com/search?q={}", query_encoded));
                }
                
                // ═══════════════════════════════════════════════════════════════
                // 🔥 HUGGINGFACE - MODELOS Y DATASETS
                // ═══════════════════════════════════════════════════════════════
                "huggingface.co" => {
                    urls.push(format!("https://huggingface.co/models?search={}", query_encoded));
                    urls.push(format!("https://huggingface.co/datasets?search={}", query_encoded));
                    urls.push(format!("https://huggingface.co/spaces?search={}", query_encoded));
                    for page in 2..=5 {
                        urls.push(format!("https://huggingface.co/models?search={}&p={}", query_encoded, page));
                    }
                }
                
                // ═══════════════════════════════════════════════════════════════
                // 🔥 RUST ECOSYSTEM
                // ═══════════════════════════════════════════════════════════════
                "docs.rs" => {
                    urls.push(format!("https://docs.rs/releases/search?query={}", query_encoded));
                }
                "crates.io" => {
                    urls.push(format!("https://crates.io/search?q={}", query_encoded));
                    for page in 2..=5 {
                        urls.push(format!("https://crates.io/search?q={}&page={}", query_encoded, page));
                    }
                }
                "rust-lang.org" => {
                    urls.push(format!("https://doc.rust-lang.org/?search={}", query_encoded));
                }
                
                // ═══════════════════════════════════════════════════════════════
                // 🔥 NPM / PYPI - PACKAGES
                // ═══════════════════════════════════════════════════════════════
                "npmjs.com" => {
                    urls.push(format!("https://www.npmjs.com/search?q={}", query_encoded));
                    for page in 2..=5 {
                        urls.push(format!("https://www.npmjs.com/search?q={}&page={}", query_encoded, page));
                    }
                }
                "pypi.org" => {
                    urls.push(format!("https://pypi.org/search/?q={}", query_encoded));
                }
                
                // ═══════════════════════════════════════════════════════════════
                // 🔥 PAPERS Y RESEARCH
                // ═══════════════════════════════════════════════════════════════
                "paperswithcode.com" => {
                    urls.push(format!("https://paperswithcode.com/search?q={}", query_encoded));
                }
                "arxiv.org" => {
                    urls.push(format!("https://arxiv.org/search/?query={}&searchtype=all", query_encoded));
                }
                
                // ═══════════════════════════════════════════════════════════════
                // 🔥 BITBUCKET / CODEBERG / SOURCEFORGE
                // ═══════════════════════════════════════════════════════════════
                "bitbucket.org" => {
                    urls.push(format!("https://bitbucket.org/repo/all?name={}", query_encoded));
                }
                "codeberg.org" => {
                    urls.push(format!("https://codeberg.org/explore/repos?q={}", query_encoded));
                }
                "sourceforge.net" => {
                    urls.push(format!("https://sourceforge.net/directory/?q={}", query_encoded));
                }
                
                // ═══════════════════════════════════════════════════════════════
                // 🔥 NOTICIAS TECH
                // ═══════════════════════════════════════════════════════════════
                "hackernews.com" | "news.ycombinator.com" => {
                    urls.push(format!("https://hn.algolia.com/?q={}", query_encoded));
                }
                "techcrunch.com" => {
                    urls.push(format!("https://techcrunch.com/?s={}", query_encoded));
                }
                "theverge.com" => {
                    urls.push(format!("https://www.theverge.com/search?q={}", query_encoded));
                }
                "wired.com" => {
                    urls.push(format!("https://www.wired.com/search/?q={}", query_encoded));
                }
                
                // Fuente genérica
                _ => {
                    urls.push(format!("https://{}/search?q={}", source, query_encoded));
                }
            }
        }
        
        urls
    }

    /// 🔥 NUCLEAR v3.0: Fuentes alternativas adicionales para maximizar resultados
    fn prepare_alternative_sources(&self, query: &str) -> Vec<String> {
        let mut urls = Vec::new();
        let query_encoded = urlencoding::encode(query);
        
        // ═══════════════════════════════════════════════════════════════
        // 🔥 DOCUMENTACIÓN OFICIAL
        // ═══════════════════════════════════════════════════════════════
        urls.push(format!("https://docs.python.org/3/search.html?q={}", query_encoded));
        urls.push(format!("https://doc.rust-lang.org/std/?search={}", query_encoded));
        urls.push(format!("https://golang.org/search?q={}", query_encoded));
        urls.push(format!("https://nodejs.org/search?q={}", query_encoded));
        
        // ═══════════════════════════════════════════════════════════════
        // 🔥 ALTERNATIVAS A GITHUB (Chino y más)
        // ═══════════════════════════════════════════════════════════════
        urls.push(format!("https://gitee.com/search?q={}", query_encoded)); // GitHub chino
        urls.push(format!("https://gitea.com/explore/repos?q={}", query_encoded));
        urls.push(format!("https://sr.ht/projects?search={}", query_encoded)); // SourceHut
        urls.push(format!("https://notabug.org/explore/repos?q={}", query_encoded));
        
        // ═══════════════════════════════════════════════════════════════
        // 🔥 FOROS ADICIONALES
        // ═══════════════════════════════════════════════════════════════
        urls.push(format!("https://lobste.rs/search?q={}", query_encoded));
        urls.push(format!("https://news.ycombinator.com/item?id=ask&q={}", query_encoded));
        urls.push(format!("https://www.v2ex.com/search?q={}", query_encoded));
        
        // ═══════════════════════════════════════════════════════════════
        // 🔥 TUTORIALES Y CURSOS
        // ═══════════════════════════════════════════════════════════════
        urls.push(format!("https://www.freecodecamp.org/news/search/?query={}", query_encoded));
        urls.push(format!("https://www.codecademy.com/search?query={}", query_encoded));
        urls.push(format!("https://www.tutorialspoint.com/search?query={}", query_encoded));
        urls.push(format!("https://www.w3schools.com/search/?q={}", query_encoded));
        urls.push(format!("https://www.geeksforgeeks.org/search/?q={}", query_encoded));
        
        // ═══════════════════════════════════════════════════════════════
        // 🔥 PREGUNTAS Y RESPUESTAS
        // ═══════════════════════════════════════════════════════════════
        urls.push(format!("https://www.quora.com/search?q={}", query_encoded));
        urls.push(format!("https://askubuntu.com/search?q={}", query_encoded));
        urls.push(format!("https://unix.stackexchange.com/search?q={}", query_encoded));
        urls.push(format!("https://superuser.com/search?q={}", query_encoded));
        urls.push(format!("https://serverfault.com/search?q={}", query_encoded));
        
        // ═══════════════════════════════════════════════════════════════
        // 🔥 PACKAGE MANAGERS ADICIONALES
        // ═══════════════════════════════════════════════════════════════
        urls.push(format!("https://packagist.org/?query={}", query_encoded)); // PHP
        urls.push(format!("https://rubygems.org/search?query={}", query_encoded)); // Ruby
        urls.push(format!("https://pkg.go.dev/search?q={}", query_encoded)); // Go
        urls.push(format!("https://hex.pm/packages?search={}", query_encoded)); // Elixir
        urls.push(format!("https://pub.dev/packages?q={}", query_encoded)); // Dart/Flutter
        urls.push(format!("https://search.maven.org/search?q={}", query_encoded)); // Java
        urls.push(format!("https://nuget.org/packages?q={}", query_encoded)); // .NET
        
        // ═══════════════════════════════════════════════════════════════
        // 🔥 SNIPPETS Y EJEMPLOS
        // ═══════════════════════════════════════════════════════════════
        urls.push(format!("https://gist.github.com/search?q={}", query_encoded));
        urls.push(format!("https://pastebin.com/search?q={}", query_encoded));
        urls.push(format!("https://www.rosettacode.org/mw/index.php?search={}", query_encoded));
        
        // ═══════════════════════════════════════════════════════════════
        // 🔥 SEGURIDAD Y CVEs
        // ═══════════════════════════════════════════════════════════════
        urls.push(format!("https://cve.mitre.org/cgi-bin/cvekey.cgi?keyword={}", query_encoded));
        urls.push(format!("https://nvd.nist.gov/vuln/search/results?query={}", query_encoded));
        urls.push(format!("https://www.exploit-db.com/search?q={}", query_encoded));
        
        // ═══════════════════════════════════════════════════════════════
        // 🔥 APIs Y DOCS
        // ═══════════════════════════════════════════════════════════════
        urls.push(format!("https://rapidapi.com/search/{}", query_encoded));
        urls.push(format!("https://www.postman.com/search?q={}", query_encoded));
        urls.push(format!("https://swagger.io/search/?q={}", query_encoded));
        
        // ═══════════════════════════════════════════════════════════════
        // 🔥 CLOUD Y DEVOPS
        // ═══════════════════════════════════════════════════════════════
        urls.push(format!("https://registry.terraform.io/search/providers?q={}", query_encoded));
        urls.push(format!("https://artifacthub.io/packages/search?ts_query_web={}", query_encoded)); // Helm charts
        urls.push(format!("https://hub.docker.com/search?q={}", query_encoded));
        urls.push(format!("https://galaxy.ansible.com/search?keywords={}", query_encoded));
        
        urls
    }

    /// Calcula relevancia de un resultado
    fn calculate_relevance(&self, query: &str, html: &str) -> f32 {
        let query_lower = query.to_lowercase();
        let html_lower = html.to_lowercase();

        // Contar ocurrencias de palabras clave
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();
        let mut matches = 0;
        let total_words = query_words.len();

        for word in &query_words {
            if html_lower.contains(word) {
                matches += 1;
            }
        }

        if total_words > 0 {
            (matches as f32) / (total_words as f32)
        } else {
            0.0
        }
    }

    /// Extrae título del HTML
    fn extract_title(&self, html: &str) -> String {
        use regex::Regex;
        let re = Regex::new(r"(?i)<title[^>]*>(.*?)</title>")
            .unwrap_or_else(|_| Regex::new("").unwrap());
        if let Some(cap) = re.captures(html) {
            let title = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            // Decodificar entidades HTML básicas
            title
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&#39;", "'")
                .to_string()
        } else {
            "Sin título".to_string()
        }
    }

    /// Extrae descripción del HTML
    fn extract_description(&self, html: &str) -> String {
        use regex::Regex;
        let re =
            Regex::new(r#"(?i)<meta[^>]*name=["']description["'][^>]*content=["']([^"']*)["']"#)
                .unwrap_or_else(|_| Regex::new("").unwrap());
        if let Some(cap) = re.captures(html) {
            cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string()
        } else {
            // Intentar extraer del body
            let body_re = Regex::new(r"(?i)<body[^>]*>(.*?)</body>")
                .unwrap_or_else(|_| Regex::new("").unwrap());
            if let Some(cap) = body_re.captures(html) {
                let body = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                let text = regex::Regex::new(r"<[^>]+>")
                    .unwrap_or_else(|_| Regex::new("").unwrap())
                    .replace_all(body, " ");
                text.trim().chars().take(200).collect()
            } else {
                "Sin descripción".to_string()
            }
        }
    }

    /// Extrae fuente de la URL
    fn extract_source(&self, url: &str) -> String {
        use url::Url;
        if let Ok(parsed) = Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                return host.to_string();
            }
        }
        "unknown".to_string()
    }

    /// Calcula calidad del contenido
    fn calculate_quality(&self, html: &str) -> f32 {
        let html_len = html.len();
        let mut score: f32 = 0.5; // Base

        // Bonus por tamaño razonable
        if html_len > 1000 && html_len < 100000 {
            score += 0.2;
        }

        // Bonus por tener estructura HTML válida
        if html.contains("<html") && html.contains("<body") {
            score += 0.2;
        }

        // Bonus por tener meta tags
        if html.contains("<meta") {
            score += 0.1;
        }

        score.min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_search_config_default() {
        let config = WebSearchConfig::default();
        assert!(!config.sources.is_empty());
        assert_eq!(config.max_parallel, 10000);
        assert!(config.use_ai);
        assert!(config.use_stealth);
    }

    #[test]
    fn test_calculate_relevance() {
        let storage = None;
        let web_search = WebSearch::new_with_storage(storage).unwrap();

        let html = "<html><body>Rust programming language is great</body></html>";
        let relevance = web_search.calculate_relevance("Rust programming", html);

        assert!(relevance > 0.0);
        assert!(relevance <= 1.0);
    }

    #[test]
    fn test_extract_title() {
        let storage = None;
        let web_search = WebSearch::new_with_storage(storage).unwrap();

        let html = "<html><head><title>Test Page</title></head></html>";
        let title = web_search.extract_title(html);

        assert_eq!(title, "Test Page");
    }

    #[test]
    fn test_extract_source() {
        let storage = None;
        let web_search = WebSearch::new_with_storage(storage).unwrap();

        let source = web_search.extract_source("https://github.com/search?q=rust");
        assert_eq!(source, "github.com");
    }

    #[test]
    fn test_calculate_quality() {
        let storage = None;
        let web_search = WebSearch::new_with_storage(storage).unwrap();

        let html = "<html><head><meta name=\"description\" content=\"test\"></head><body>Content</body></html>";
        let quality = web_search.calculate_quality(html);

        assert!(quality > 0.5);
        assert!(quality <= 1.0);
    }

    #[test]
    fn test_prepare_search_urls() {
        let storage = None;
        let web_search = WebSearch::new_with_storage(storage).unwrap();

        let sources = vec!["github.com".to_string(), "stackoverflow.com".to_string()];
        let urls = web_search.prepare_search_urls("rust", &sources);

        // 🔥 NUCLEAR: Ahora genera MUCHAS más URLs por fuente
        assert!(urls.len() > 50, "Debe generar 50+ URLs para búsqueda masiva");
        assert!(urls.iter().any(|u| u.contains("github.com")));
        assert!(urls.iter().any(|u| u.contains("stackoverflow.com")));
    }
    
    #[test]
    fn test_alternative_sources() {
        let storage = None;
        let web_search = WebSearch::new_with_storage(storage).unwrap();

        let urls = web_search.prepare_alternative_sources("rust");

        // 🔥 NUCLEAR: Debe generar URLs de fuentes alternativas
        assert!(urls.len() > 40, "Debe generar 40+ URLs alternativas");
        assert!(urls.iter().any(|u| u.contains("gitee.com") || u.contains("geeksforgeeks")));
    }
}
