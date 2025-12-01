//! Módulo Utils - Utilidades varias
//!
//! Funciones auxiliares y helpers

use regex::Regex;
use std::collections::HashSet;
use url::Url;

/// Normaliza una URL
pub fn normalize_url(url: &str) -> Option<String> {
    Url::parse(url).ok().map(|u| {
        let mut normalized = u.clone();
        normalized.set_fragment(None);
        normalized.set_query(None);
        normalized.as_str().to_string()
    })
}

/// Extrae el dominio de una URL
pub fn extract_domain(url: &str) -> Option<String> {
    Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(|h| h.to_string()))
}

/// Verifica si una URL es válida
pub fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}

/// Limpia y normaliza texto
pub fn clean_text(text: &str) -> String {
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

/// Extrae emails de un texto
pub fn extract_emails(text: &str) -> Vec<String> {
    let email_regex = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();

    email_regex
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

/// Extrae números de teléfono
pub fn extract_phones(text: &str) -> Vec<String> {
    let phone_regex =
        Regex::new(r"[\+]?[(]?[0-9]{3}[)]?[-\s\.]?[0-9]{3}[-\s\.]?[0-9]{4,6}").unwrap();

    phone_regex
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

/// Genera un hash simple de una URL
pub fn url_hash(url: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    hasher.finish()
}

/// Calcula similitud entre dos strings (Jaccard)
pub fn string_similarity(s1: &str, s2: &str) -> f64 {
    let set1: HashSet<char> = s1.chars().collect();
    let set2: HashSet<char> = s2.chars().collect();

    let intersection = set1.intersection(&set2).count();
    let union = set1.union(&set2).count();

    if union == 0 {
        return 0.0;
    }

    intersection as f64 / union as f64
}
