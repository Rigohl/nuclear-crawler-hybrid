//! Módulo Intelligence - Análisis Inteligente de Contenido
//!
//! Consolida intelligence.rs y ai_smart.rs
//! Sistema de análisis inteligente para encontrar el mejor contenido,
//! detectar calidad, relevancia, y valor de la información

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[allow(unused_imports)]
use strsim;

// Re-exportar AntiBanAction desde ai_smart para compatibilidad
pub use crate::ai_smart::AntiBanAction;

/// Score de calidad de contenido
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentScore {
    /// Score total (0-100)
    pub total_score: f64,

    /// Relevancia del contenido
    pub relevance: f64,

    /// Calidad del contenido
    pub quality: f64,

    /// Completitud
    pub completeness: f64,

    /// Unicidad (no duplicado)
    pub uniqueness: f64,

    /// Riqueza de información
    pub information_density: f64,

    /// Factores que contribuyen al score
    pub factors: HashMap<String, f64>,
}

/// Analizador de contenido inteligente (consolida Intelligence + AISmart)
pub struct ContentIntelligence {
    matcher: SkimMatcherV2,
    keywords: Vec<String>,
    quality_indicators: Vec<String>,
    // Campos de AISmart integrados
    #[allow(dead_code)]
    patterns: dashmap::DashMap<String, Pattern>,
    #[allow(dead_code)]
    success_rate: dashmap::DashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub pattern_type: String,
    pub confidence: f32,
    pub action: String,
    pub frequency: usize,
}

impl ContentIntelligence {
    /// Crea un nuevo analizador
    pub fn new(keywords: Vec<String>) -> Self {
        let quality_indicators = vec![
            "article",
            "content",
            "main",
            "post",
            "entry",
            "description",
            "summary",
            "abstract",
            "body",
            "text",
            "paragraph",
            "section",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        Self {
            matcher: SkimMatcherV2::default(),
            keywords,
            quality_indicators,
            patterns: dashmap::DashMap::new(),
            success_rate: dashmap::DashMap::new(),
        }
    }

    // Funciones de AISmart integradas - usar tipo de ai_smart
    pub fn recommend_anti_ban_action(&self, _domain: &str, ban_risk: f32) -> AntiBanAction {
        if ban_risk > 0.8 {
            AntiBanAction {
                action: "pause".to_string(),
                duration_ms: 60000,
                change_strategy: true,
            }
        } else if ban_risk > 0.5 {
            AntiBanAction {
                action: "slow".to_string(),
                duration_ms: 10000,
                change_strategy: false,
            }
        } else {
            AntiBanAction {
                action: "continue".to_string(),
                duration_ms: 0,
                change_strategy: false,
            }
        }
    }

    /// Analiza y scorea contenido HTML
    pub fn analyze_content(&self, html: &str, _url: &str) -> ContentScore {
        let mut factors = HashMap::new();

        // 1. Relevancia basada en keywords
        let relevance = self.calculate_relevance(html, &mut factors);

        // 2. Calidad del contenido
        let quality = self.calculate_quality(html, &mut factors);

        // 3. Completitud
        let completeness = self.calculate_completeness(html, &mut factors);

        // 4. Densidad de información
        let information_density = self.calculate_information_density(html, &mut factors);

        // 5. Unicidad (simplificado - en producción usar hash de contenido)
        let uniqueness = 1.0; // Por ahora asumimos único

        // Score total (promedio ponderado)
        let total_score = relevance * 0.3
            + quality * 0.25
            + completeness * 0.2
            + information_density * 0.15
            + uniqueness * 0.1;

        ContentScore {
            total_score,
            relevance,
            quality,
            completeness,
            uniqueness,
            information_density,
            factors,
        }
    }

    /// Calcula relevancia basada en keywords
    fn calculate_relevance(&self, html: &str, factors: &mut HashMap<String, f64>) -> f64 {
        if self.keywords.is_empty() {
            factors.insert("relevance_no_keywords".to_string(), 0.5);
            return 0.5;
        }

        let html_lower = html.to_lowercase();
        let mut matches = 0;
        let mut total_score = 0.0;

        for keyword in &self.keywords {
            let keyword_lower = keyword.to_lowercase();

            // Búsqueda exacta
            if html_lower.contains(&keyword_lower) {
                matches += 1;
                total_score += 1.0;
            } else {
                // Búsqueda fuzzy
                if let Some(score) = self.matcher.fuzzy_match(&html_lower, &keyword_lower) {
                    let normalized = (score as f64 / 100.0).min(1.0);
                    total_score += normalized * 0.5;
                }
            }
        }

        let relevance = (total_score / self.keywords.len() as f64).min(1.0);
        factors.insert("relevance_matches".to_string(), matches as f64);
        factors.insert("relevance_score".to_string(), relevance);

        relevance
    }

    /// Calcula calidad del contenido
    fn calculate_quality(&self, html: &str, factors: &mut HashMap<String, f64>) -> f64 {
        let mut indicators_found = 0;

        let html_lower = html.to_lowercase();

        // Buscar indicadores de calidad
        for indicator in &self.quality_indicators {
            if html_lower.contains(indicator) {
                indicators_found += 1;
            }
        }

        // Score basado en indicadores encontrados
        let indicator_score =
            (indicators_found as f64 / self.quality_indicators.len() as f64).min(1.0);

        // Longitud del contenido (más largo generalmente = mejor)
        let text_length = html.len();
        let length_score = if text_length > 5000 {
            1.0
        } else if text_length > 2000 {
            0.8
        } else if text_length > 1000 {
            0.6
        } else if text_length > 500 {
            0.4
        } else {
            0.2
        };

        // Presencia de estructura (headings, lists, etc.)
        let structure_score = self.calculate_structure_score(html);

        let quality_score =
            (indicator_score * 0.4 + length_score * 0.4 + structure_score * 0.2).min(1.0);

        factors.insert("quality_indicators".to_string(), indicators_found as f64);
        factors.insert("quality_length".to_string(), length_score);
        factors.insert("quality_structure".to_string(), structure_score);

        quality_score
    }

    /// Calcula score de estructura
    fn calculate_structure_score(&self, html: &str) -> f64 {
        let mut score = 0.0;

        // Headings
        let h_count = html.matches("<h").count();
        if h_count > 0 {
            score += 0.3;
        }

        // Lists
        let list_count = html.matches("<li>").count();
        if list_count > 5 {
            score += 0.3;
        }

        // Paragraphs
        let p_count = html.matches("<p>").count();
        if p_count > 3 {
            score += 0.2;
        }

        // Links (contenido enlazado)
        let link_count = html.matches("<a").count();
        if link_count > 5 {
            score += 0.2;
        }

        f64::min(score, 1.0)
    }

    /// Calcula completitud
    fn calculate_completeness(&self, html: &str, factors: &mut HashMap<String, f64>) -> f64 {
        let mut completeness = 0.0;

        // Título presente
        if html.contains("<title>") || html.contains("<h1>") {
            completeness += 0.2;
        }

        // Meta description
        if html.contains("meta name=\"description\"") {
            completeness += 0.2;
        }

        // Contenido principal
        if html.contains("<main>") || html.contains("<article>") || html.contains("<body>") {
            completeness += 0.3;
        }

        // Imágenes
        if html.matches("<img").count() > 0 {
            completeness += 0.15;
        }

        // Links
        if html.matches("<a").count() > 0 {
            completeness += 0.15;
        }

        factors.insert("completeness_score".to_string(), completeness);
        completeness
    }

    /// Calcula densidad de información
    fn calculate_information_density(&self, html: &str, factors: &mut HashMap<String, f64>) -> f64 {
        // Extraer solo texto (simplificado)
        let text = self.extract_text(html);
        let text_length = text.len();

        // Contar palabras únicas
        let words: Vec<&str> = text.split_whitespace().collect();
        let unique_words: std::collections::HashSet<&str> = words.iter().cloned().collect();
        let unique_ratio = if words.is_empty() {
            0.0
        } else {
            unique_words.len() as f64 / words.len() as f64
        };

        // Densidad = ratio de palabras únicas * factor de longitud
        let length_factor = if text_length > 1000 {
            1.0
        } else {
            text_length as f64 / 1000.0
        };

        let density = f64::min(unique_ratio * 0.7 + length_factor * 0.3, 1.0);

        factors.insert(
            "density_unique_words".to_string(),
            unique_words.len() as f64,
        );
        factors.insert("density_total_words".to_string(), words.len() as f64);
        factors.insert("density_ratio".to_string(), unique_ratio);

        density
    }

    /// Extrae texto del HTML (simplificado)
    fn extract_text(&self, html: &str) -> String {
        // Remover tags HTML básicas
        let mut text = html.to_string();

        // Remover scripts y styles (sin backreferences ya que no están soportadas en Rust regex)
        let script_re = regex::Regex::new(r"(?i)<script[^>]*>.*?</script>").unwrap();
        text = script_re.replace_all(&text, "").to_string();
        let style_re = regex::Regex::new(r"(?i)<style[^>]*>.*?</style>").unwrap();
        text = style_re.replace_all(&text, "").to_string();

        // Remover tags HTML
        let re = regex::Regex::new(r"<[^>]+>").unwrap();
        text = re.replace_all(&text, " ").to_string();

        // Limpiar espacios
        let re = regex::Regex::new(r"\s+").unwrap();
        text = re.replace_all(&text, " ").to_string();

        text.trim().to_string()
    }

    /// Filtra contenido por score mínimo
    pub fn filter_by_score(&self, content: &str, url: &str, min_score: f64) -> bool {
        let score = self.analyze_content(content, url);
        score.total_score >= min_score
    }
}
