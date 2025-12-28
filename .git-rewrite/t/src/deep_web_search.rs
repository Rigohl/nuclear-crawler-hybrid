//! Módulo Deep Web Search - Búsqueda Profunda en Deep Web
//!
//! Sistema especializado para buscar:
//! - Código y software en repositorios profundos
//! - Inteligencia técnica y académica
//! - Contenido premium/pago/importante
//! - Métodos legales para acceder a contenido
//!
//! 🔥 INTEGRADO CON NUCLEAR BYPASS - Go, Zig, Stealth

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use url::Url;

use crate::content_extractor::ContentExtractor; // 🔥 EXTRACCIÓN REAL
use crate::nuclear_bypass::{NuclearBypass, NuclearBypassConfig};
use crate::nuclear_scraper::NuclearScraper;
use crate::web_search::WebSearch;

/// Configuración de búsqueda deep web
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepWebSearchConfig {
    /// Query de búsqueda
    pub query: String,

    /// Tipo de búsqueda
    pub search_type: DeepWebSearchType,

    /// Fuentes específicas a buscar
    pub sources: Vec<DeepWebSource>,

    /// Máximo de resultados
    pub max_results: usize,

    /// Buscar métodos de acceso
    pub find_access_methods: bool,

    /// Usar técnicas avanzadas
    pub use_advanced_techniques: bool,
}

/// Tipos de búsqueda deep web
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeepWebSearchType {
    /// Código y software
    Code,
    /// Inteligencia técnica
    Intelligence,
    /// Contenido premium/pago
    Premium,
    /// Todo
    All,
}

/// Fuentes deep web
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeepWebSource {
    /// Repositorios académicos
    Academic,
    /// Bases de datos técnicas
    TechnicalDB,
    /// Repositorios de código alternativos
    CodeRepos,
    /// Foros especializados
    SpecializedForums,
    /// Bibliotecas digitales
    DigitalLibraries,
    /// Archivos y documentos
    Archives,
    /// Tor/Onion (si está disponible)
    Tor,
    /// APIs públicas de contenido premium
    PremiumAPIs,
}

/// Resultado de búsqueda deep web con EXTRACCIÓN REAL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepWebSearchResult {
    /// URL encontrada
    pub url: String,

    /// Título
    pub title: String,

    /// Descripción
    pub description: String,

    /// 🔥 Texto principal extraído
    #[serde(default)]
    pub main_text: String,

    /// 🔥 Resumen del contenido
    #[serde(default)]
    pub summary: String,

    /// 🔥 Número de palabras
    #[serde(default)]
    pub word_count: usize,

    /// 🔥 Headings encontrados
    #[serde(default)]
    pub headings: Vec<String>,

    /// 🔥 Snippets de código
    #[serde(default)]
    pub code_snippets: Vec<String>,

    /// Tipo de contenido
    pub content_type: String,

    /// Es contenido premium
    pub is_premium: bool,

    /// Métodos de acceso disponibles
    pub access_methods: Vec<AccessMethod>,

    /// Relevancia (0-1)
    pub relevance: f32,

    /// Calidad (0-1)
    pub quality_score: f32,

    /// Fuente
    pub source: String,

    /// Metadata adicional
    pub metadata: serde_json::Value,
}

/// Método de acceso a contenido
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessMethod {
    /// Nombre del método
    pub name: String,

    /// Descripción
    pub description: String,

    /// Tipo de método
    pub method_type: AccessMethodType,

    /// URL o instrucciones
    pub instructions: String,

    /// Es legal
    pub is_legal: bool,

    /// Requiere autenticación
    pub requires_auth: bool,
}

/// Tipos de métodos de acceso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessMethodType {
    /// API pública
    PublicAPI,
    /// Biblioteca/acceso institucional
    LibraryAccess,
    /// Repositorio abierto
    OpenRepository,
    /// Método alternativo legal
    AlternativeLegal,
    /// Acceso directo
    DirectAccess,
    /// Tor/Onion
    TorAccess,
}

/// Sistema de búsqueda deep web con NUCLEAR BYPASS
pub struct DeepWebSearch {
    scraper: Arc<NuclearScraper>,
    #[allow(dead_code)]
    web_search: Arc<WebSearch>,
    /// 🔥 Sistema de bypass integrado
    bypass: Arc<NuclearBypass>,
}

impl DeepWebSearch {
    /// Crea nuevo sistema de búsqueda deep web con bypass
    pub fn new(scraper: Arc<NuclearScraper>, web_search: Arc<WebSearch>) -> Self {
        // Configurar bypass con Go y Zig
        let bypass_config = NuclearBypassConfig {
            use_go: true,
            use_zig: true,
            timeout_secs: 30,
            max_retries: 3,
            custom_user_agent: None,
            aggressive_mode: true,
            cache_successful_bypasses: true,
        };

        let bypass = Arc::new(
            NuclearBypass::new(bypass_config)
                .unwrap_or_else(|_| NuclearBypass::new(NuclearBypassConfig::default()).unwrap()),
        );

        Self {
            scraper,
            web_search,
            bypass,
        }
    }

    /// Realiza búsqueda deep web con BYPASS NUCLEAR
    pub async fn search(&self, config: DeepWebSearchConfig) -> Result<Vec<DeepWebSearchResult>> {
        let start = Instant::now();

        println!("🔍 BÚSQUEDA DEEP WEB - CÓDIGO, SOFTWARE, INTELIGENCIA");
        println!("   Query: {}", config.query);
        println!("   Tipo: {:?}", config.search_type);
        println!("   Fuentes: {}", config.sources.len());

        // Preparar URLs de búsqueda según tipo y fuentes
        let search_urls = self.prepare_deep_web_urls(&config).await?;

        println!("   📡 URLs a buscar: {}", search_urls.len());

        // Realizar búsqueda masiva
        let crawl_results = self.scraper.nuclear_crawl(search_urls).await?;

        // Procesar resultados
        let mut results = Vec::new();

        for crawl_result in crawl_results {
            if crawl_result.status_code == 200 && !crawl_result.html.is_empty() {
                let result = self.process_deep_web_result(&crawl_result, &config).await?;

                if let Some(deep_result) = result {
                    results.push(deep_result);
                }
            }
        }

        // Buscar métodos de acceso si está habilitado
        if config.find_access_methods {
            for result in &mut results {
                result.access_methods = self.find_access_methods(result, &config).await?;
            }
        }

        // Ordenar por relevancia y calidad
        results.sort_by(|a, b| {
            let score_a = a.relevance * 0.7 + a.quality_score * 0.3;
            let score_b = b.relevance * 0.7 + b.quality_score * 0.3;
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Limitar resultados
        if config.max_results > 0 {
            results.truncate(config.max_results);
        }

        let duration = start.elapsed();
        println!("   ✅ Búsqueda completada en {:?}", duration);
        println!("   📊 Resultados encontrados: {}", results.len());

        Ok(results)
    }

    /// Prepara URLs de búsqueda deep web - NUCLEAR PAYWALL BYPASS
    async fn prepare_deep_web_urls(&self, config: &DeepWebSearchConfig) -> Result<Vec<String>> {
        let mut urls = Vec::new();

        // Si no hay fuentes específicas, usar todas
        let sources = if config.sources.is_empty() {
            vec![
                DeepWebSource::Academic,
                DeepWebSource::TechnicalDB,
                DeepWebSource::CodeRepos,
                DeepWebSource::DigitalLibraries,
                DeepWebSource::Archives,
                DeepWebSource::SpecializedForums,
                DeepWebSource::PremiumAPIs,
            ]
        } else {
            config.sources.clone()
        };

        let query_encoded = urlencoding::encode(&config.query);

        for source in &sources {
            match source {
                DeepWebSource::Academic => {
                    // ═══════════════════════════════════════════════════════════════
                    // 🔥 REPOSITORIOS ACADÉMICOS - BYPASS PAYWALLS
                    // ═══════════════════════════════════════════════════════════════

                    // Google Scholar (gratis)
                    urls.push(format!(
                        "https://scholar.google.com/scholar?q={}",
                        query_encoded
                    ));
                    for start in (10..=100).step_by(10) {
                        urls.push(format!(
                            "https://scholar.google.com/scholar?q={}&start={}",
                            query_encoded, start
                        ));
                    }

                    // arXiv (gratis, preprints)
                    urls.push(format!(
                        "https://arxiv.org/search/?query={}&searchtype=all",
                        query_encoded
                    ));
                    urls.push(format!(
                        "https://arxiv.org/search/?query={}&searchtype=title",
                        query_encoded
                    ));
                    urls.push(format!(
                        "https://arxiv.org/search/?query={}&searchtype=abstract",
                        query_encoded
                    ));
                    urls.push(format!("https://export.arxiv.org/api/query?search_query=all:{}&start=0&max_results=100", query_encoded));

                    // Semantic Scholar (gratis)
                    urls.push(format!(
                        "https://www.semanticscholar.org/search?q={}",
                        query_encoded
                    ));
                    urls.push(format!(
                        "https://api.semanticscholar.org/graph/v1/paper/search?query={}&limit=100",
                        query_encoded
                    ));

                    // ResearchGate (parcialmente gratis)
                    urls.push(format!(
                        "https://www.researchgate.net/search?q={}",
                        query_encoded
                    ));

                    // CORE (acceso abierto a papers)
                    urls.push(format!("https://core.ac.uk/search?q={}", query_encoded));

                    // BASE (Bielefeld Academic Search Engine)
                    urls.push(format!(
                        "https://www.base-search.net/Search/Results?lookfor={}",
                        query_encoded
                    ));

                    // DOAJ (Directory of Open Access Journals)
                    urls.push(format!("https://doaj.org/search/articles?ref=homepage-box&source={{\"query\":{{\"query_string\":{{\"query\":\"{}\"}}}}}}", config.query));

                    // OpenAlex (gratis, metadatos académicos)
                    urls.push(format!(
                        "https://api.openalex.org/works?search={}&per-page=100",
                        query_encoded
                    ));

                    // Unpaywall alternativas
                    urls.push(format!(
                        "https://unpaywall.org/api/search?query={}",
                        query_encoded
                    ));
                }
                DeepWebSource::TechnicalDB => {
                    // ═══════════════════════════════════════════════════════════════
                    // 🔥 BASES DE DATOS TÉCNICAS - BYPASS Y ALTERNATIVAS
                    // ═══════════════════════════════════════════════════════════════

                    // IEEE (buscar versiones abiertas)
                    urls.push(format!(
                        "https://ieeexplore.ieee.org/search/searchresult.jsp?queryText={}",
                        query_encoded
                    ));

                    // ACM (buscar en DL abierta)
                    urls.push(format!(
                        "https://dl.acm.org/action/doSearch?AllField={}",
                        query_encoded
                    ));

                    // Springer (algunas secciones abiertas)
                    urls.push(format!(
                        "https://link.springer.com/search?query={}",
                        query_encoded
                    ));

                    // PubMed (gratis)
                    urls.push(format!(
                        "https://pubmed.ncbi.nlm.nih.gov/?term={}",
                        query_encoded
                    ));

                    // Europe PMC (gratis)
                    urls.push(format!(
                        "https://europepmc.org/search?query={}",
                        query_encoded
                    ));

                    // CiteSeerX (gratis)
                    urls.push(format!(
                        "http://citeseerx.ist.psu.edu/search?q={}",
                        query_encoded
                    ));

                    // DBLP (gratis, CS papers)
                    urls.push(format!("https://dblp.org/search?q={}", query_encoded));
                }
                DeepWebSource::CodeRepos => {
                    // ═══════════════════════════════════════════════════════════════
                    // 🔥 REPOSITORIOS DE CÓDIGO ALTERNATIVOS
                    // ═══════════════════════════════════════════════════════════════

                    // SourceForge
                    urls.push(format!(
                        "https://sourceforge.net/directory/?q={}",
                        query_encoded
                    ));

                    // GitLab explore
                    urls.push(format!(
                        "https://gitlab.com/explore/projects?search={}",
                        query_encoded
                    ));
                    for page in 2..=10 {
                        urls.push(format!(
                            "https://gitlab.com/explore/projects?search={}&page={}",
                            query_encoded, page
                        ));
                    }

                    // Bitbucket
                    urls.push(format!(
                        "https://bitbucket.org/repo/all?name={}",
                        query_encoded
                    ));

                    // Codeberg (alternativa ética a GitHub)
                    urls.push(format!(
                        "https://codeberg.org/explore/repos?q={}",
                        query_encoded
                    ));

                    // Gitee (GitHub chino, mucho código)
                    urls.push(format!(
                        "https://gitee.com/search?type=repository&q={}",
                        query_encoded
                    ));

                    // Sourcehut
                    urls.push(format!("https://sr.ht/projects?search={}", query_encoded));

                    // Launchpad
                    urls.push(format!(
                        "https://launchpad.net/+search?field.text={}",
                        query_encoded
                    ));

                    // GNU Savannah
                    urls.push(format!(
                        "https://savannah.gnu.org/search/?type_of_search=soft&words={}",
                        query_encoded
                    ));
                }
                DeepWebSource::SpecializedForums => {
                    // ═══════════════════════════════════════════════════════════════
                    // 🔥 FOROS ESPECIALIZADOS
                    // ═══════════════════════════════════════════════════════════════

                    // Reddit técnicos
                    for sub in &[
                        "programming",
                        "rust",
                        "python",
                        "javascript",
                        "reverseengineering",
                        "netsec",
                        "hacking",
                        "cybersecurity",
                        "machinelearning",
                        "datascience",
                        "devops",
                        "sysadmin",
                        "linux",
                        "opensource",
                        "privacy",
                    ] {
                        urls.push(format!(
                            "https://www.reddit.com/r/{}/search/?q={}&restrict_sr=1&sort=relevance",
                            sub, query_encoded
                        ));
                    }

                    // Hacker News
                    urls.push(format!("https://hn.algolia.com/?q={}", query_encoded));

                    // Lobsters
                    urls.push(format!("https://lobste.rs/search?q={}", query_encoded));

                    // Stack Exchange network
                    for site in &[
                        "security",
                        "softwareengineering",
                        "unix",
                        "serverfault",
                        "superuser",
                    ] {
                        urls.push(format!(
                            "https://{}.stackexchange.com/search?q={}",
                            site, query_encoded
                        ));
                    }
                }
                DeepWebSource::DigitalLibraries => {
                    // ═══════════════════════════════════════════════════════════════
                    // 🔥 BIBLIOTECAS DIGITALES - CONTENIDO LIBRE
                    // ═══════════════════════════════════════════════════════════════

                    // Internet Archive (gratis)
                    urls.push(format!(
                        "https://archive.org/search.php?query={}",
                        query_encoded
                    ));
                    urls.push(format!(
                        "https://archive.org/advancedsearch.php?q={}&output=json",
                        query_encoded
                    ));

                    // Project Gutenberg (ebooks gratis)
                    urls.push(format!(
                        "https://www.gutenberg.org/ebooks/search/?query={}",
                        query_encoded
                    ));

                    // Open Library
                    urls.push(format!(
                        "https://openlibrary.org/search?q={}",
                        query_encoded
                    ));

                    // Library Genesis mirrors (papers/libros, legal en muchos países)
                    urls.push(format!(
                        "https://libgen.is/search.php?req={}",
                        query_encoded
                    ));
                    urls.push(format!(
                        "https://libgen.rs/search.php?req={}",
                        query_encoded
                    ));

                    // Z-Library mirrors
                    urls.push(format!("https://z-lib.org/s/{}", query_encoded));

                    // Anna's Archive (metabúsqueda)
                    urls.push(format!(
                        "https://annas-archive.org/search?q={}",
                        query_encoded
                    ));

                    // Sci-Hub (papers, acceso abierto)
                    // Note: Verificar legalidad en tu jurisdicción
                    urls.push(format!("https://sci-hub.se/{}", query_encoded));
                }
                DeepWebSource::Archives => {
                    // ═══════════════════════════════════════════════════════════════
                    // 🔥 ARCHIVOS Y DOCUMENTOS
                    // ═══════════════════════════════════════════════════════════════

                    // Scribd (vista previa)
                    urls.push(format!(
                        "https://www.scribd.com/search?query={}",
                        query_encoded
                    ));

                    // SlideShare
                    urls.push(format!(
                        "https://www.slideshare.net/search/slideshow?searchfrom=header&q={}",
                        query_encoded
                    ));

                    // Issuu
                    urls.push(format!("https://issuu.com/search?q={}", query_encoded));

                    // DocDroid
                    urls.push(format!(
                        "https://www.docdroid.net/search?q={}",
                        query_encoded
                    ));

                    // Wayback Machine (versiones archivadas)
                    urls.push(format!(
                        "https://web.archive.org/cdx/search/cdx?url=*{}*&output=json&limit=100",
                        query_encoded
                    ));

                    // Common Crawl (datos web masivos)
                    urls.push(format!(
                        "https://index.commoncrawl.org/CC-MAIN-2023-50-index?url=*{}*&output=json",
                        query_encoded
                    ));
                }
                DeepWebSource::Tor => {
                    // ═══════════════════════════════════════════════════════════════
                    // 🔥 TOR/ONION - SOLO SI ADVANCED TECHNIQUES
                    // ═══════════════════════════════════════════════════════════════
                    if config.use_advanced_techniques {
                        // Ahmia (buscador .onion)
                        urls.push(format!("https://ahmia.fi/search/?q={}", query_encoded));

                        // Torch (si Tor está configurado)
                        urls.push(format!("http://xmh57jrknzkhv6y3ls3ubitzfqnkrwxhopf5aygthi7d6rplyvk3noyd.onion/cgi-bin/omega/omega?P={}", query_encoded));

                        // Haystak
                        urls.push(format!("http://haystak5njsmn2hqkewecpaxetahtwhsbsa64jom2k22z5afxhnpxfid.onion/?q={}", query_encoded));
                    }
                }
                DeepWebSource::PremiumAPIs => {
                    // ═══════════════════════════════════════════════════════════════
                    // 🔥 APIs PÚBLICAS QUE DAN ACCESO A PREMIUM
                    // ═══════════════════════════════════════════════════════════════

                    // GitHub API (sin límites estrictos para búsqueda básica)
                    urls.push(format!(
                        "https://api.github.com/search/repositories?q={}&per_page=100",
                        query_encoded
                    ));
                    urls.push(format!(
                        "https://api.github.com/search/code?q={}&per_page=100",
                        query_encoded
                    ));

                    // NPM Registry
                    urls.push(format!(
                        "https://registry.npmjs.org/-/v1/search?text={}&size=250",
                        query_encoded
                    ));

                    // Crates.io API
                    urls.push(format!(
                        "https://crates.io/api/v1/crates?q={}&per_page=100",
                        query_encoded
                    ));

                    // PyPI JSON API
                    urls.push(format!("https://pypi.org/pypi/{}/json", query_encoded));

                    // RubyGems API
                    urls.push(format!(
                        "https://rubygems.org/api/v1/search.json?query={}",
                        query_encoded
                    ));

                    // Docker Hub
                    urls.push(format!(
                        "https://hub.docker.com/v2/search/repositories/?query={}&page_size=100",
                        query_encoded
                    ));

                    // Hugging Face API
                    urls.push(format!(
                        "https://huggingface.co/api/models?search={}&limit=100",
                        query_encoded
                    ));
                    urls.push(format!(
                        "https://huggingface.co/api/datasets?search={}&limit=100",
                        query_encoded
                    ));
                }
            }
        }

        Ok(urls)
    }

    /// Procesa resultado de deep web
    async fn process_deep_web_result(
        &self,
        crawl_result: &crate::nuclear_scraper::NuclearResult,
        config: &DeepWebSearchConfig,
    ) -> Result<Option<DeepWebSearchResult>> {
        let html = &crawl_result.html;
        let url = &crawl_result.url;

        // Extraer título
        let title = self.extract_title(html);
        let description = self.extract_description(html);

        // Determinar tipo de contenido
        let content_type = self.detect_content_type(html, url);
        let is_premium = self.detect_premium_content(html, url);

        // Calcular relevancia
        let relevance = self.calculate_relevance(&config.query, html);

        // Calcular calidad
        let quality_score = self.calculate_quality(html, url);

        // Extraer metadata
        let metadata = self.extract_metadata(html, url);

        // 🔥 EXTRACCIÓN REAL de contenido usando ContentExtractor
        let extractor = ContentExtractor::new();
        let extracted = extractor.extract_all(html, url).ok();
        
        let (main_text, summary, word_count, headings, code_snippets) = if let Some(ext) = extracted {
            (
                ext.main_text.chars().take(3000).collect(), // Deep web: más texto
                ext.summary,
                ext.word_count,
                ext.headings,
                ext.code_snippets.into_iter().map(|c| c.code).take(10).collect(), // Más código
            )
        } else {
            (String::new(), String::new(), 0, Vec::new(), Vec::new())
        };

        // Filtrar por tipo de búsqueda
        if !self.matches_search_type(&config.search_type, &content_type, is_premium) {
            return Ok(None);
        }

        Ok(Some(DeepWebSearchResult {
            url: url.clone(),
            title,
            description,
            main_text,
            summary,
            word_count,
            headings,
            code_snippets,
            content_type,
            is_premium,
            access_methods: Vec::new(),
            relevance,
            quality_score,
            source: self.extract_source(url),
            metadata,
        }))
    }

    /// Detecta tipo de contenido
    fn detect_content_type(&self, html: &str, url: &str) -> String {
        let url_lower = url.to_lowercase();
        let html_lower = html.to_lowercase();

        if url_lower.contains("github")
            || url_lower.contains("gitlab")
            || url_lower.contains("code")
        {
            "code".to_string()
        } else if url_lower.contains("arxiv")
            || url_lower.contains("research")
            || url_lower.contains("paper")
        {
            "research".to_string()
        } else if url_lower.contains("pdf") || html_lower.contains("application/pdf") {
            "document".to_string()
        } else if html_lower.contains("software") || html_lower.contains("download") {
            "software".to_string()
        } else if url_lower.contains("api") || html_lower.contains("api") {
            "api".to_string()
        } else {
            "general".to_string()
        }
    }

    /// Detecta contenido premium
    fn detect_premium_content(&self, html: &str, url: &str) -> bool {
        let html_lower = html.to_lowercase();
        let url_lower = url.to_lowercase();

        html_lower.contains("premium")
            || html_lower.contains("subscription")
            || html_lower.contains("paywall")
            || html_lower.contains("purchase")
            || html_lower.contains("buy now")
            || url_lower.contains("premium")
            || html_lower.contains("members only")
            || html_lower.contains("requires payment")
    }

    /// Calcula relevancia
    fn calculate_relevance(&self, query: &str, html: &str) -> f32 {
        let query_lower = query.to_lowercase();
        let html_lower = html.to_lowercase();

        let query_words: Vec<&str> = query_lower.split_whitespace().collect();
        let mut matches = 0;

        for word in &query_words {
            if html_lower.contains(word) {
                matches += 1;
            }
        }

        if query_words.is_empty() {
            0.0
        } else {
            (matches as f32) / (query_words.len() as f32)
        }
    }

    /// Calcula calidad
    fn calculate_quality(&self, html: &str, url: &str) -> f32 {
        let mut score: f32 = 0.5;

        // Bonus por tamaño razonable
        if html.len() > 1000 && html.len() < 100000 {
            score += 0.2;
        }

        // Bonus por estructura HTML válida
        if html.contains("<html") && html.contains("<body") {
            score += 0.1;
        }

        // Bonus por tener código/software
        if html.contains("code") || html.contains("software") || html.contains("download") {
            score += 0.1;
        }

        // Bonus por URL de fuente confiable
        let url_lower = url.to_lowercase();
        if url_lower.contains("github")
            || url_lower.contains("gitlab")
            || url_lower.contains("arxiv")
        {
            score += 0.1;
        }

        score.min(1.0f32)
    }

    /// Extrae metadata
    fn extract_metadata(&self, html: &str, url: &str) -> serde_json::Value {
        let mut metadata = serde_json::json!({});

        // Extraer keywords
        let keywords = self.extract_keywords(html);
        if !keywords.is_empty() {
            metadata["keywords"] = serde_json::json!(keywords);
        }

        // Extraer autor si está disponible
        if let Some(author) = self.extract_author(html) {
            metadata["author"] = serde_json::json!(author);
        }

        // Extraer fecha si está disponible
        if let Some(date) = self.extract_date(html) {
            metadata["date"] = serde_json::json!(date);
        }

        metadata["url"] = serde_json::json!(url);
        metadata
    }

    /// Extrae keywords
    fn extract_keywords(&self, html: &str) -> Vec<String> {
        use regex::Regex;
        let re = Regex::new(r#"(?i)<meta[^>]*name=["']keywords["'][^>]*content=["']([^"']*)["']"#)
            .unwrap_or_else(|_| Regex::new("").unwrap());

        if let Some(cap) = re.captures(html) {
            if let Some(keywords_str) = cap.get(1) {
                return keywords_str
                    .as_str()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
            }
        }

        Vec::new()
    }

    /// Extrae autor
    fn extract_author(&self, html: &str) -> Option<String> {
        use regex::Regex;
        let re = Regex::new(r#"(?i)<meta[^>]*name=["']author["'][^>]*content=["']([^"']*)["']"#)
            .unwrap_or_else(|_| Regex::new("").unwrap());

        if let Some(cap) = re.captures(html) {
            return cap.get(1).map(|m| m.as_str().to_string());
        }

        None
    }

    /// Extrae fecha
    fn extract_date(&self, html: &str) -> Option<String> {
        use regex::Regex;
        let re = Regex::new(
            r#"(?i)<meta[^>]*property=["']article:published_time["'][^>]*content=["']([^"']*)["']"#,
        )
        .unwrap_or_else(|_| Regex::new("").unwrap());

        if let Some(cap) = re.captures(html) {
            return cap.get(1).map(|m| m.as_str().to_string());
        }

        None
    }

    /// Verifica si coincide con tipo de búsqueda
    fn matches_search_type(
        &self,
        search_type: &DeepWebSearchType,
        content_type: &str,
        is_premium: bool,
    ) -> bool {
        match search_type {
            DeepWebSearchType::All => true,
            DeepWebSearchType::Code => content_type == "code" || content_type == "software",
            DeepWebSearchType::Intelligence => {
                content_type == "research" || content_type == "document"
            }
            DeepWebSearchType::Premium => is_premium,
        }
    }

    /// 🔥 Encuentra métodos de acceso CON BYPASS NUCLEAR
    async fn find_access_methods(
        &self,
        result: &DeepWebSearchResult,
        _config: &DeepWebSearchConfig,
    ) -> Result<Vec<AccessMethod>> {
        let mut methods = Vec::new();

        let url_lower = result.url.to_lowercase();

        // ═══════════════════════════════════════════════════════════════
        // 🔥 MÉTODO 0: BYPASS NUCLEAR DIRECTO (Go + Zig + Stealth)
        // ═══════════════════════════════════════════════════════════════
        if result.is_premium {
            // Intentar bypass real
            let bypass_result = self.bypass.bypass(&result.url).await;
            if let Ok(br) = bypass_result {
                if br.success {
                    methods.push(AccessMethod {
                        name: "🔥 NUCLEAR BYPASS".to_string(),
                        description: format!("Bypass automático exitoso: {}", br.message),
                        method_type: AccessMethodType::DirectAccess,
                        instructions: format!(
                            "Contenido extraído ({} bytes). Usar API de Nuclear.",
                            br.content.len()
                        ),
                        is_legal: true,
                        requires_auth: false,
                    });
                }
            }
        }

        // ═══════════════════════════════════════════════════════════════
        // 🔥 MÉTODO 1: SCI-HUB para papers académicos
        // ═══════════════════════════════════════════════════════════════
        if url_lower.contains("ieee")
            || url_lower.contains("acm")
            || url_lower.contains("springer")
            || url_lower.contains("nature.com")
            || url_lower.contains("sciencedirect")
            || url_lower.contains("wiley")
            || url_lower.contains("tandfonline")
        {
            methods.push(AccessMethod {
                name: "🔓 Sci-Hub Direct".to_string(),
                description: "Acceso directo vía Sci-Hub (bypass de paywall académico)".to_string(),
                method_type: AccessMethodType::DirectAccess,
                instructions: format!("https://sci-hub.se/{}", result.url),
                is_legal: true, // Legal en muchos países
                requires_auth: false,
            });
            methods.push(AccessMethod {
                name: "📚 Sci-Hub Mirror".to_string(),
                description: "Mirror alternativo de Sci-Hub".to_string(),
                method_type: AccessMethodType::DirectAccess,
                instructions: format!("https://sci-hub.st/{}", result.url),
                is_legal: true,
                requires_auth: false,
            });
        }

        // ═══════════════════════════════════════════════════════════════
        // 🔥 MÉTODO 2: LIBGEN para libros y papers
        // ═══════════════════════════════════════════════════════════════
        if result.is_premium || url_lower.contains("book") || url_lower.contains("ebook") {
            methods.push(AccessMethod {
                name: "📖 Library Genesis".to_string(),
                description: "Buscar en LibGen (biblioteca libre más grande del mundo)".to_string(),
                method_type: AccessMethodType::OpenRepository,
                instructions: format!(
                    "https://libgen.is/search.php?req={}",
                    urlencoding::encode(&result.title)
                ),
                is_legal: true,
                requires_auth: false,
            });
            methods.push(AccessMethod {
                name: "📚 Anna's Archive".to_string(),
                description: "Meta-buscador de libros y papers".to_string(),
                method_type: AccessMethodType::OpenRepository,
                instructions: format!(
                    "https://annas-archive.org/search?q={}",
                    urlencoding::encode(&result.title)
                ),
                is_legal: true,
                requires_auth: false,
            });
        }

        // ═══════════════════════════════════════════════════════════════
        // 🔥 MÉTODO 3: 12FT.IO para noticias premium
        // ═══════════════════════════════════════════════════════════════
        if url_lower.contains("nytimes")
            || url_lower.contains("washingtonpost")
            || url_lower.contains("wsj.com")
            || url_lower.contains("economist")
            || url_lower.contains("medium.com")
            || url_lower.contains("bloomberg")
        {
            methods.push(AccessMethod {
                name: "📰 12ft.io Bypass".to_string(),
                description: "Bypass de paywall de noticias".to_string(),
                method_type: AccessMethodType::DirectAccess,
                instructions: format!("https://12ft.io/{}", result.url),
                is_legal: true,
                requires_auth: false,
            });
            methods.push(AccessMethod {
                name: "📄 Outline.com".to_string(),
                description: "Vista limpia sin paywall".to_string(),
                method_type: AccessMethodType::DirectAccess,
                instructions: format!("https://outline.com/{}", result.url),
                is_legal: true,
                requires_auth: false,
            });
        }

        // ═══════════════════════════════════════════════════════════════
        // 🔥 MÉTODO 4: UNPAYWALL para papers con acceso abierto
        // ═══════════════════════════════════════════════════════════════
        methods.push(AccessMethod {
            name: "🔓 Unpaywall".to_string(),
            description: "Busca versión de acceso abierto automáticamente".to_string(),
            method_type: AccessMethodType::OpenRepository,
            instructions: "Instalar extensión Unpaywall en navegador o usar API".to_string(),
            is_legal: true,
            requires_auth: false,
        });

        // Método: API pública si es GitHub/GitLab
        if url_lower.contains("github.com") {
            methods.push(AccessMethod {
                name: "GitHub API".to_string(),
                description: "Acceso vía API pública de GitHub".to_string(),
                method_type: AccessMethodType::PublicAPI,
                instructions: format!(
                    "Usar: curl https://api.github.com/repos/{}/{}",
                    result.url.split('/').nth(3).unwrap_or(""),
                    result.url.split('/').nth(4).unwrap_or("")
                ),
                is_legal: true,
                requires_auth: false,
            });
        }

        // Método: Archive.org para contenido histórico
        if !url_lower.contains("archive.org") {
            methods.push(AccessMethod {
                name: "Wayback Machine".to_string(),
                description: "Ver versión archivada en Archive.org".to_string(),
                method_type: AccessMethodType::AlternativeLegal,
                instructions: format!("Visitar: https://web.archive.org/web/*/{}", result.url),
                is_legal: true,
                requires_auth: false,
            });
        }

        // ═══════════════════════════════════════════════════════════════
        // 🔥 MÉTODO FINAL: Google Cache
        // ═══════════════════════════════════════════════════════════════
        methods.push(AccessMethod {
            name: "🔍 Google Cache".to_string(),
            description: "Versión en caché de Google".to_string(),
            method_type: AccessMethodType::AlternativeLegal,
            instructions: format!(
                "https://webcache.googleusercontent.com/search?q=cache:{}",
                urlencoding::encode(&result.url)
            ),
            is_legal: true,
            requires_auth: false,
        });

        Ok(methods)
    }

    /// Extrae título
    fn extract_title(&self, html: &str) -> String {
        use regex::Regex;
        let re = Regex::new(r"(?i)<title[^>]*>(.*?)</title>")
            .unwrap_or_else(|_| Regex::new("").unwrap());
        if let Some(cap) = re.captures(html) {
            let title = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            return title
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&#39;", "'")
                .to_string();
        }
        "Sin título".to_string()
    }

    /// Extrae descripción
    fn extract_description(&self, html: &str) -> String {
        use regex::Regex;
        let re =
            Regex::new(r#"(?i)<meta[^>]*name=["']description["'][^>]*content=["']([^"']*)["']"#)
                .unwrap_or_else(|_| Regex::new("").unwrap());
        if let Some(cap) = re.captures(html) {
            return cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
        }
        "Sin descripción".to_string()
    }

    /// Extrae fuente
    fn extract_source(&self, url: &str) -> String {
        if let Ok(parsed) = Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                return host.to_string();
            }
        }
        "unknown".to_string()
    }
}
