//! Módulo Massive Parallel Search
//!
//! Sistema que aprende a buscar en MÚLTIPLES lugares simultáneamente
//! Aprovecha que datos REALES son mejores y busca en paralelo masivo

use anyhow::Result;
use dashmap::DashMap;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[allow(unused_imports)]
use std::time::Duration;
use tokio::sync::Semaphore;

use crate::ai_smart::AISmart;
#[allow(unused_imports)]
use crate::ai_smart::MassiveSearchStrategy;
use crate::nuclear_scraper::NuclearScraper;

/// Resultado de búsqueda masiva
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassiveSearchResult {
    /// Fuente
    pub source: String,

    /// URLs encontradas
    pub urls_found: Vec<String>,

    /// Datos extraídos
    pub data_extracted: serde_json::Value,

    /// 🔥 NUEVO: Contenido de texto extraído (NO solo URLs)
    pub extracted_text: Vec<ExtractedContent>,

    /// Calidad de datos (real = alta, sintético = baja)
    pub data_quality: f32,

    /// Es dato real
    pub is_real_data: bool,

    /// Tiempo de búsqueda
    pub search_time_ms: u64,

    /// Éxito
    pub success: bool,
}

/// 🔥 Contenido extraído de una página
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedContent {
    /// URL de la página
    pub url: String,
    /// Título
    pub title: String,
    /// Descripción/resumen
    pub description: String,
    /// Texto del contenido principal (párrafos, artículos)
    pub main_content: String,
    /// Headings encontrados
    pub headings: Vec<String>,
    /// Snippets de código (si hay)
    pub code_snippets: Vec<String>,
    /// Tamaño del contenido
    pub content_length: usize,
}

/// Sistema de búsqueda masiva paralela
pub struct MassiveParallelSearch {
    scraper: Arc<NuclearScraper>,
    ai_smart: Arc<AISmart>,
    semaphore: Arc<Semaphore>,
    results: Arc<DashMap<String, MassiveSearchResult>>,
}

impl MassiveParallelSearch {
    /// Crea nuevo sistema de búsqueda masiva
    pub fn new(scraper: Arc<NuclearScraper>, ai_smart: Arc<AISmart>) -> Self {
        Self {
            scraper,
            ai_smart,
            semaphore: Arc::new(Semaphore::new(1000)), // Máximo paralelismo
            results: Arc::new(DashMap::new()),
        }
    }

    /// 🔥 BÚSQUEDA MASIVA CON QUERY - Busca de verdad en motores de búsqueda
    pub async fn search_with_query(
        &self,
        query: &str,
        sources: Vec<String>,
    ) -> Result<Vec<MassiveSearchResult>> {
        println!("🔥 BÚSQUEDA MASIVA CON QUERY: {}", query);
        
        // Generar URLs de búsqueda REALES para cada fuente
        let search_urls = self.generate_search_urls(query, &sources);
        println!("   📍 URLs de búsqueda generadas: {}", search_urls.len());
        
        // Buscar en todas las URLs
        self.search_massive_parallel(search_urls).await
    }

    /// 🔥 Genera URLs de búsqueda REALES para motores y sitios
    fn generate_search_urls(&self, query: &str, sources: &[String]) -> Vec<String> {
        let encoded_query = urlencoding::encode(query);
        let mut urls = Vec::new();
        
        for source in sources {
            let source_lower = source.to_lowercase();
            
            // 🔥 MOTORES DE BÚSQUEDA REALES
            if source_lower.contains("duckduckgo") {
                urls.push(format!("https://html.duckduckgo.com/html/?q={}", encoded_query));
                urls.push(format!("https://duckduckgo.com/?q={}&t=h_&ia=web", encoded_query));
            } else if source_lower.contains("bing") {
                urls.push(format!("https://www.bing.com/search?q={}", encoded_query));
                urls.push(format!("https://www.bing.com/search?q={}&first=10", encoded_query));
            } else if source_lower.contains("brave") {
                urls.push(format!("https://search.brave.com/search?q={}", encoded_query));
            } else if source_lower.contains("yandex") {
                urls.push(format!("https://yandex.com/search/?text={}", encoded_query));
            } else if source_lower.contains("ecosia") {
                urls.push(format!("https://www.ecosia.org/search?q={}", encoded_query));
            } else if source_lower.contains("qwant") {
                urls.push(format!("https://www.qwant.com/?q={}", encoded_query));
            } else if source_lower.contains("startpage") {
                urls.push(format!("https://www.startpage.com/sp/search?query={}", encoded_query));
            } else if source_lower.contains("searx") {
                urls.push(format!("https://searx.be/search?q={}", encoded_query));
                urls.push(format!("https://searx.tiekoetter.com/search?q={}", encoded_query));
            } else if source_lower.contains("mojeek") {
                urls.push(format!("https://www.mojeek.com/search?q={}", encoded_query));
            }
            // 🔥 SITIOS ESPECÍFICOS CON BÚSQUEDA
            else if source_lower.contains("github") {
                urls.push(format!("https://github.com/search?q={}&type=repositories", encoded_query));
                urls.push(format!("https://github.com/search?q={}&type=code", encoded_query));
            } else if source_lower.contains("stackoverflow") {
                urls.push(format!("https://stackoverflow.com/search?q={}", encoded_query));
            } else if source_lower.contains("reddit") {
                urls.push(format!("https://www.reddit.com/search/?q={}", encoded_query));
                urls.push(format!("https://old.reddit.com/search?q={}", encoded_query));
            } else if source_lower.contains("dev.to") {
                urls.push(format!("https://dev.to/search?q={}", encoded_query));
            } else if source_lower.contains("huggingface") {
                urls.push(format!("https://huggingface.co/models?search={}", encoded_query));
                urls.push(format!("https://huggingface.co/datasets?search={}", encoded_query));
            } else if source_lower.contains("crates.io") {
                urls.push(format!("https://crates.io/search?q={}", encoded_query));
            } else if source_lower.contains("pypi") {
                urls.push(format!("https://pypi.org/search/?q={}", encoded_query));
            } else if source_lower.contains("npm") {
                urls.push(format!("https://www.npmjs.com/search?q={}", encoded_query));
            } else if source_lower.contains("arxiv") {
                urls.push(format!("https://arxiv.org/search/?query={}&searchtype=all", encoded_query));
            } else if source_lower.contains("medium") {
                urls.push(format!("https://medium.com/search?q={}", encoded_query));
            } else if source_lower.contains("wikipedia") {
                urls.push(format!("https://en.wikipedia.org/w/index.php?search={}", encoded_query));
            }
            // Fuente genérica - intentar añadir /search
            else if source.starts_with("http") {
                urls.push(source.clone());
            } else {
                // Intentar múltiples formatos de búsqueda
                urls.push(format!("https://{}/search?q={}", source, encoded_query));
                urls.push(format!("https://www.{}/search?q={}", source, encoded_query));
            }
        }
        
        urls
    }

    /// Busca en MÚLTIPLES fuentes simultáneamente (sin query, URLs directas)
    pub async fn search_massive_parallel(
        &self,
        sources: Vec<String>,
    ) -> Result<Vec<MassiveSearchResult>> {
        println!("🚀 BÚSQUEDA MASIVA PARALELA");
        println!("   📚 Fuentes: {}", sources.len());
        println!("   ⚡ Paralelismo: Máximo");
        println!("   ✅ Priorizando datos REALES");

        // Obtener estrategia recomendada por AI
        let strategy = self
            .ai_smart
            .recommend_massive_parallel_search(sources.clone());

        println!("   🧠 Estrategia AI:");
        println!("      • Concurrent: {}", strategy.max_concurrent);
        println!("      • Delay: {}ms", strategy.delay_ms);
        println!("      • Batch: {}", strategy.batch_size);
        println!(
            "      • Real data priority: {}",
            strategy.prioritize_real_data
        );

        // Procesar en paralelo masivo usando async streams (NO rayon+block_on)
        let scraper = self.scraper.clone();
        let semaphore = self.semaphore.clone();
        
        let results: Vec<Result<MassiveSearchResult>> = stream::iter(sources)
            .map(|source| {
                let scraper = scraper.clone();
                let semaphore = semaphore.clone();
                async move {
                    let _permit = semaphore.acquire().await?;
                    let start = std::time::Instant::now();

                    // Buscar en esta fuente
                    match Self::search_single_source(&source, &scraper).await {
                        Ok(data) => {
                            let search_time = start.elapsed();

                            // Determinar calidad (datos reales = mejor)
                            let is_real_data = Self::is_real_data_source(&source);
                            let data_quality = if is_real_data { 0.95 } else { 0.60 };

                            Ok(MassiveSearchResult {
                                source,
                                urls_found: data.urls,
                                data_extracted: data.content,
                                extracted_text: data.extracted_content, // 🔥 NUEVO
                                data_quality,
                                is_real_data,
                                search_time_ms: search_time.as_millis() as u64,
                                success: true,
                            })
                        }
                        Err(_e) => Ok(MassiveSearchResult {
                            source,
                            urls_found: Vec::new(),
                            data_extracted: serde_json::json!({}),
                            extracted_text: Vec::new(), // 🔥 NUEVO
                            data_quality: 0.0,
                            is_real_data: false,
                            search_time_ms: start.elapsed().as_millis() as u64,
                            success: false,
                        }),
                    }
                }
            })
            .buffer_unordered(strategy.max_concurrent)
            .collect()
            .await;

        // Convertir resultados
        let mut final_results = Vec::new();
        for result in results {
            match result {
                Ok(r) => final_results.push(r),
                Err(_) => continue,
            }
        }

        // Filtrar por calidad (priorizar datos reales)
        if strategy.prioritize_real_data {
            final_results.sort_by(|a, b| b.data_quality.partial_cmp(&a.data_quality).unwrap());
        }

        // Guardar resultados
        for result in &final_results {
            self.results.insert(result.source.clone(), result.clone());
        }

        println!(
            "   ✅ Completado: {} fuentes procesadas",
            final_results.len()
        );
        println!(
            "   📊 Datos reales: {}",
            final_results.iter().filter(|r| r.is_real_data).count()
        );

        Ok(final_results)
    }

    /// Busca en una fuente individual - BÚSQUEDA REAL
    async fn search_single_source(source: &str, scraper: &NuclearScraper) -> Result<SourceData> {
        // 🔥 BÚSQUEDA REAL - NO SIMULADA
        // Construir URL de búsqueda real
        let search_url = if source.starts_with("http") {
            source.to_string()
        } else {
            format!("https://{}", source)
        };

        // Hacer crawl real
        let results = scraper.nuclear_crawl(vec![search_url.clone()]).await?;

        // Extraer datos reales
        let mut urls_found = Vec::new();
        let mut content_parts = Vec::new();
        let mut extracted_content = Vec::new(); // 🔥 NUEVO: Contenido extraído

        for result in results {
            if result.status_code == 200 && !result.html.is_empty() {
                // Extraer URLs del HTML
                let extracted_urls = Self::extract_urls_from_html(&result.html);
                urls_found.extend(extracted_urls);

                // 🔥 EXTRACCIÓN COMPLETA DE CONTENIDO
                let title = Self::extract_title(&result.html);
                let description = Self::extract_description(&result.html);
                let main_content = Self::extract_main_content(&result.html);
                let headings = Self::extract_headings(&result.html);
                let code_snippets = Self::extract_code_snippets(&result.html);

                content_parts.push(serde_json::json!({
                    "url": result.url,
                    "title": title,
                    "status": result.status_code,
                    "content_length": result.html.len(),
                }));

                // 🔥 Guardar contenido extraído
                extracted_content.push(ExtractedContent {
                    url: result.url.clone(),
                    title,
                    description,
                    main_content,
                    headings,
                    code_snippets,
                    content_length: result.html.len(),
                });
            }
        }

        // Deduplicar URLs
        urls_found.sort();
        urls_found.dedup();
        urls_found.truncate(50); // Máximo 50 URLs por fuente

        Ok(SourceData {
            urls: urls_found,
            content: serde_json::json!({
                "source": source,
                "type": "real_data",
                "quality": "high",
                "pages_crawled": content_parts.len(),
                "pages": content_parts,
            }),
            extracted_content, // 🔥 NUEVO
        })
    }

    /// 🔥 Extrae descripción/meta del HTML
    fn extract_description(html: &str) -> String {
        use regex::Regex;
        
        // Intentar meta description
        if let Ok(re) = Regex::new(r#"(?i)<meta[^>]*name=["']description["'][^>]*content=["']([^"']+)["']"#) {
            if let Some(cap) = re.captures(html) {
                if let Some(desc) = cap.get(1) {
                    let text = desc.as_str().trim();
                    if !text.is_empty() {
                        return text.chars().take(500).collect();
                    }
                }
            }
        }
        
        // Intentar og:description
        if let Ok(re) = Regex::new(r#"(?i)<meta[^>]*property=["']og:description["'][^>]*content=["']([^"']+)["']"#) {
            if let Some(cap) = re.captures(html) {
                if let Some(desc) = cap.get(1) {
                    let text = desc.as_str().trim();
                    if !text.is_empty() {
                        return text.chars().take(500).collect();
                    }
                }
            }
        }
        
        // Intentar primer párrafo
        if let Ok(re) = Regex::new(r"(?i)<p[^>]*>([^<]{50,500})</p>") {
            if let Some(cap) = re.captures(html) {
                if let Some(p) = cap.get(1) {
                    return Self::clean_html_text(p.as_str());
                }
            }
        }
        
        String::new()
    }

    /// 🔥 Extrae contenido principal del HTML (artículos, posts, etc)
    fn extract_main_content(html: &str) -> String {
        use regex::Regex;
        let mut content = String::new();
        
        // Buscar contenedores de contenido principal
        let content_selectors = [
            r"(?is)<article[^>]*>(.*?)</article>",
            r"(?is)<main[^>]*>(.*?)</main>",
            r#"(?is)<div[^>]*class=["'][^"']*(?:content|post|article|entry|text)[^"']*["'][^>]*>(.*?)</div>"#,
            r"(?is)<section[^>]*>(.*?)</section>",
        ];
        
        for selector in content_selectors {
            if let Ok(re) = Regex::new(selector) {
                for cap in re.captures_iter(html) {
                    if let Some(inner) = cap.get(1) {
                        let text = Self::clean_html_text(inner.as_str());
                        if text.len() > 100 {
                            content.push_str(&text);
                            content.push_str("\n\n");
                            if content.len() > 5000 {
                                break;
                            }
                        }
                    }
                }
            }
            if content.len() > 5000 {
                break;
            }
        }
        
        // Si no encontramos contenido, extraer todos los párrafos
        if content.len() < 200 {
            if let Ok(re) = Regex::new(r"(?is)<p[^>]*>(.*?)</p>") {
                for cap in re.captures_iter(html) {
                    if let Some(p) = cap.get(1) {
                        let text = Self::clean_html_text(p.as_str());
                        if text.len() > 30 {
                            content.push_str(&text);
                            content.push_str("\n");
                            if content.len() > 5000 {
                                break;
                            }
                        }
                    }
                }
            }
        }
        
        content.chars().take(5000).collect()
    }

    /// 🔥 Extrae headings (h1-h3)
    fn extract_headings(html: &str) -> Vec<String> {
        use regex::Regex;
        let mut headings = Vec::new();
        
        if let Ok(re) = Regex::new(r"(?i)<h[123][^>]*>(.*?)</h[123]>") {
            for cap in re.captures_iter(html) {
                if let Some(h) = cap.get(1) {
                    let text = Self::clean_html_text(h.as_str());
                    if !text.is_empty() && text.len() < 200 {
                        headings.push(text);
                    }
                    if headings.len() >= 20 {
                        break;
                    }
                }
            }
        }
        
        headings
    }

    /// 🔥 Extrae snippets de código
    fn extract_code_snippets(html: &str) -> Vec<String> {
        use regex::Regex;
        let mut snippets = Vec::new();
        
        // Buscar <pre><code>
        if let Ok(re) = Regex::new(r"(?is)<pre[^>]*><code[^>]*>(.*?)</code></pre>") {
            for cap in re.captures_iter(html) {
                if let Some(code) = cap.get(1) {
                    let text = Self::decode_html_entities(code.as_str());
                    if text.len() > 20 && text.len() < 2000 {
                        snippets.push(text);
                    }
                    if snippets.len() >= 10 {
                        break;
                    }
                }
            }
        }
        
        // Buscar <code> standalone
        if snippets.is_empty() {
            if let Ok(re) = Regex::new(r"(?is)<code[^>]*>(.*?)</code>") {
                for cap in re.captures_iter(html) {
                    if let Some(code) = cap.get(1) {
                        let text = Self::decode_html_entities(code.as_str());
                        if text.len() > 20 && text.len() < 500 {
                            snippets.push(text);
                        }
                        if snippets.len() >= 5 {
                            break;
                        }
                    }
                }
            }
        }
        
        snippets
    }

    /// Limpia texto HTML
    fn clean_html_text(html: &str) -> String {
        use regex::Regex;
        
        // Remover tags HTML
        let text = if let Ok(re) = Regex::new(r"<[^>]+>") {
            re.replace_all(html, " ").to_string()
        } else {
            html.to_string()
        };
        
        // Decodificar entidades HTML
        let text = Self::decode_html_entities(&text);
        
        // Limpiar espacios múltiples
        let text = if let Ok(re) = Regex::new(r"\s+") {
            re.replace_all(&text, " ").to_string()
        } else {
            text
        };
        
        text.trim().to_string()
    }

    /// Decodifica entidades HTML
    fn decode_html_entities(text: &str) -> String {
        text.replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'")
            .replace("&nbsp;", " ")
            .replace("&#x27;", "'")
            .replace("&#x2F;", "/")
            .replace("&apos;", "'")
    }

    /// Extrae URLs de HTML
    fn extract_urls_from_html(html: &str) -> Vec<String> {
        use regex::Regex;
        let mut urls = Vec::new();

        // Extraer hrefs
        if let Ok(re) = Regex::new(r#"href=["']([^"']+)["']"#) {
            for cap in re.captures_iter(html) {
                if let Some(url) = cap.get(1) {
                    let url_str = url.as_str();
                    if url_str.starts_with("http") && !url_str.contains("login") && !url_str.contains("signup") {
                        urls.push(url_str.to_string());
                    }
                }
            }
        }

        urls
    }

    /// Extrae título de HTML
    fn extract_title(html: &str) -> String {
        use regex::Regex;
        if let Ok(re) = Regex::new(r"(?i)<title[^>]*>(.*?)</title>") {
            if let Some(cap) = re.captures(html) {
                return cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
            }
        }
        "Sin título".to_string()
    }

    /// Determina si es fuente de datos reales
    fn is_real_data_source(source: &str) -> bool {
        // Fuentes conocidas de datos reales
        let real_sources = [
            "github.com",
            "reddit.com",
            "medium.com",
            "dev.to",
            "huggingface.co",
            "stackoverflow.com",
            "arxiv.org",
            "towardsdatascience.com",
            "kaggle.com",
            "paperswithcode.com",
        ];

        real_sources.iter().any(|&s| source.contains(s))
    }

    /// Obtiene mejores resultados (datos reales primero)
    pub fn get_best_results(&self, limit: usize) -> Vec<MassiveSearchResult> {
        let mut results: Vec<_> = self.results.iter().map(|r| r.clone()).collect();

        // Ordenar por calidad (reales primero)
        results.sort_by(|a, b| b.data_quality.partial_cmp(&a.data_quality).unwrap());

        results.into_iter().take(limit).collect()
    }
}

#[derive(Debug)]
struct SourceData {
    urls: Vec<String>,
    content: serde_json::Value,
    extracted_content: Vec<ExtractedContent>, // 🔥 NUEVO
}
