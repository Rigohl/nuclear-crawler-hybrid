//! Helpers para formateo y parsing de URLs (usado por web_search y deep_web_search)

pub fn encode_component(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

pub fn build_search_url(base: &str, query: &str) -> String {
    format!("{}?q={}", base, encode_component(query))
}

// Agrega aquí más helpers de URL...
