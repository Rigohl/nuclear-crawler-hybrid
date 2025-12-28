//! Utilidades centralizadas para cálculo de scores
//!
//! Este módulo unifica todas las funciones de scoring duplicadas
//! en web_search, nuclear_scraper, web_search_enhanced, etc.

use scraper::{Html, Selector};

/// Calcula la relevancia de un HTML con respecto a una query
/// Usado por: web_search, nuclear_scraper, web_search_enhanced
pub fn calculate_relevance(query: &str, html: &str) -> f32 {
    let query_lower = query.to_lowercase();
    let html_lower = html.to_lowercase();

    let query_terms: Vec<&str> = query_lower.split_whitespace().collect();
    if query_terms.is_empty() {
        return 0.0;
    }

    let mut score = 0.0;
    let total_terms = query_terms.len() as f32;

    // Cuenta cuántos términos aparecen
    for term in &query_terms {
        let count = html_lower.matches(term).count();
        if count > 0 {
            score += 1.0;
            // Bonus por múltiples apariciones (con saturación)
            score += (count as f32).ln() * 0.1;
        }
    }

    // Bonus si aparece en el título
    if let Some(title) = extract_title_simple(html) {
        let title_lower = title.to_lowercase();
        for term in &query_terms {
            if title_lower.contains(term) {
                score += 0.5;
            }
        }
    }

    // Normaliza por número de términos y a 0-1
    (score / total_terms / 2.0).min(1.0)
}

/// Calcula la relevancia semántica (versión mejorada con TF-IDF simplificado)
/// Usado por: web_search_enhanced
pub fn calculate_relevance_semantic(query: &str, html: &str) -> f32 {
    let base_score = calculate_relevance(query, html);

    // Extrae texto limpio
    let text = extract_text_from_html(html);
    let text_lower = text.to_lowercase();
    let query_lower = query.to_lowercase();

    // Calcula TF-IDF simplificado
    let query_terms: Vec<&str> = query_lower.split_whitespace().collect();
    let total_words = text_lower.split_whitespace().count() as f32;

    if total_words == 0.0 {
        return base_score;
    }

    let mut tf_idf_score = 0.0;
    for term in &query_terms {
        let term_count = text_lower.matches(term).count() as f32;
        if term_count > 0.0 {
            let tf = term_count / total_words;
            let idf = (1.0 / (1.0 + term_count)).ln();
            tf_idf_score += tf * idf;
        }
    }

    // Combina score base con TF-IDF
    let semantic_score = (base_score * 0.6) + (tf_idf_score * 0.4);
    semantic_score.min(1.0)
}

/// Calcula la calidad de un contenido HTML
/// Usado por: web_search, nuclear_scraper
pub fn calculate_quality(html: &str) -> f32 {
    let mut score = 0.0;
    let document = Html::parse_document(html);

    // Factor 1: Longitud del contenido (palabras)
    let text = document.root_element().text().collect::<String>();
    let word_count = text.split_whitespace().count();

    score += match word_count {
        0..=50 => 0.1,
        51..=200 => 0.3,
        201..=500 => 0.6,
        501..=1000 => 0.8,
        _ => 1.0,
    };

    // Factor 2: Presencia de estructura (headings)
    if let Ok(selector) = Selector::parse("h1, h2, h3") {
        let heading_count = document.select(&selector).count();
        score += (heading_count as f32 * 0.1).min(0.5);
    }

    // Factor 3: Presencia de párrafos
    if let Ok(selector) = Selector::parse("p") {
        let p_count = document.select(&selector).count();
        score += (p_count as f32 * 0.05).min(0.3);
    }

    // Factor 4: Presencia de enlaces
    if let Ok(selector) = Selector::parse("a") {
        let link_count = document.select(&selector).count();
        score += (link_count as f32 * 0.02).min(0.2);
    }

    // Penalización por contenido muy corto
    if word_count < 50 {
        score *= 0.5;
    }

    // Normaliza a 0-1
    (score / 3.0).min(1.0)
}

/// Calcula la calidad avanzada (con análisis de URL y estructura)
/// Usado por: web_search_enhanced
pub fn calculate_quality_advanced(html: &str, url: &str) -> f32 {
    let base_quality = calculate_quality(html);

    let mut advanced_score = base_quality;

    // Factor: Análisis de URL
    advanced_score += calculate_url_quality(url) * 0.2;

    // Factor: Presencia de imágenes relevantes
    if let Ok(selector) = Selector::parse("img[alt]") {
        let document = Html::parse_document(html);
        let img_count = document.select(&selector).count();
        advanced_score += (img_count as f32 * 0.05).min(0.3);
    }

    // Factor: Presencia de listas
    if let Ok(selector) = Selector::parse("ul, ol") {
        let document = Html::parse_document(html);
        let list_count = document.select(&selector).count();
        advanced_score += (list_count as f32 * 0.05).min(0.2);
    }

    // Factor: Metadatos
    if has_good_metadata(html) {
        advanced_score += 0.3;
    }

    // Normaliza
    (advanced_score / 2.0).min(1.0)
}

/// Calcula la calidad de una URL
fn calculate_url_quality(url: &str) -> f32 {
    let mut score: f32 = 0.5; // Base score

    // Prefiere HTTPS
    if url.starts_with("https://") {
        score += 0.2;
    }

    // Penaliza URLs muy largas
    if url.len() > 200 {
        score -= 0.2;
    }

    // Penaliza parámetros excesivos
    let param_count = url.matches('&').count();
    if param_count > 5 {
        score -= 0.1;
    }

    // Prefiere dominios conocidos
    if url.contains("wikipedia.org")
        || url.contains("github.com")
        || url.contains("stackoverflow.com")
    {
        score += 0.3;
    }

    score.max(0.0_f32).min(1.0_f32)
}

/// Verifica si el HTML tiene buenos metadatos
fn has_good_metadata(html: &str) -> bool {
    let metadata_indicators = ["og:title", "og:description", "twitter:card", "description"];

    let html_lower = html.to_lowercase();
    metadata_indicators
        .iter()
        .filter(|&indicator| html_lower.contains(indicator))
        .count()
        >= 2
}

/// Calcula el score de autoridad de una fuente
/// Usado por: web_search_enhanced
pub fn calculate_authority_score(url: &str) -> f32 {
    let mut score: f32 = 0.5; // Base score

    // Fuentes de alta autoridad
    let high_authority = [
        "wikipedia.org",
        "github.com",
        "stackoverflow.com",
        "docs.rs",
        "developer.mozilla.org",
        "rust-lang.org",
        "python.org",
        "arxiv.org",
        "ieee.org",
        "acm.org",
    ];

    for domain in &high_authority {
        if url.contains(domain) {
            return 1.0;
        }
    }

    // Fuentes de media autoridad
    let medium_authority = [".edu", ".gov", "medium.com", "dev.to", "hackernoon.com"];

    for domain in &medium_authority {
        if url.contains(domain) {
            score += 0.3;
        }
    }

    // Prefiere HTTPS
    if url.starts_with("https://") {
        score += 0.1;
    }

    score.min(1.0_f32)
}

/// Calcula el score de frescura (freshness) del contenido
/// Usado por: web_search_enhanced
pub fn calculate_freshness_score(_html: &str) -> f32 {
    // Busca indicadores de fecha
    let _date_patterns = [
        r"\d{4}-\d{2}-\d{2}", // 2024-01-01
        r"\d{2}/\d{2}/\d{4}", // 01/01/2024
        r"(?i)(enero|febrero|marzo|abril|mayo|junio|julio|agosto|septiembre|octubre|noviembre|diciembre)\s+\d{1,2},?\s+\d{4}",
    ];

    // Por ahora retornamos un score neutral
    // En una implementación completa, se extraería y analizaría la fecha
    0.5
}

/// Calcula score de mantenibilidad (para código)
/// Usado por: scan_project_enhanced
pub fn calculate_maintainability_score(
    complexity: f32,
    lines_of_code: usize,
    comment_ratio: f32,
) -> f32 {
    let mut score = 1.0;

    // Penaliza alta complejidad
    if complexity > 10.0 {
        score -= (complexity - 10.0) / 50.0;
    }

    // Penaliza archivos muy grandes
    if lines_of_code > 500 {
        score -= (lines_of_code as f32 - 500.0) / 5000.0;
    }

    // Bonus por buenos comentarios
    if comment_ratio > 0.1 && comment_ratio < 0.5 {
        score += 0.2;
    }

    score.max(0.0).min(1.0)
}

/// Calcula score de seguridad
/// Usado por: scan_project_enhanced
pub fn calculate_security_score(critical_issues: usize, warnings: usize) -> f32 {
    let mut score: f32 = 1.0;

    // Penaliza fuertemente problemas críticos
    score -= critical_issues as f32 * 0.3;

    // Penaliza warnings
    score -= warnings as f32 * 0.1;

    score.max(0.0).min(1.0)
}

/// Extrae texto simple del HTML (helper interno)
fn extract_text_from_html(html: &str) -> String {
    let document = Html::parse_document(html);
    document.root_element().text().collect::<String>()
}

/// Extrae título simple (helper interno)
fn extract_title_simple(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    if let Ok(selector) = Selector::parse("title") {
        if let Some(element) = document.select(&selector).next() {
            let text = element.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_relevance() {
        let html = "<p>Rust programming language tutorial</p>";
        let score = calculate_relevance("rust programming", html);
        assert!(score > 0.5);
    }

    #[test]
    fn test_calculate_quality() {
        let good_html = r##"
            <html>
                <body>
                    <h1>Title</h1>
                    <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit.</p>
                    <p>More content here with lots of words to make it quality.</p>
                    <a href="#">Link</a>
                </body>
            </html>
        "##;
        let score = calculate_quality(good_html);
        assert!(score > 0.3);
    }

    #[test]
    fn test_calculate_authority_score() {
        assert_eq!(
            calculate_authority_score("https://wikipedia.org/article"),
            1.0
        );
        assert!(calculate_authority_score("https://example.edu/page") > 0.5);
    }

    #[test]
    fn test_calculate_security_score() {
        assert_eq!(calculate_security_score(0, 0), 1.0);
        assert!(calculate_security_score(1, 2) < 0.7);
    }
}
