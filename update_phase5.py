import re

with open('src/core/web_search.rs', 'r') as f:
    content = f.read()

replacement = """        // Convertir resultados de MassiveParallelSearch a WebSearchResult
        for massive_result in &massive_results {
            if massive_result.status_code >= 200 && massive_result.status_code < 300 {
                for url in &massive_result.links_found {
                    if !processed_results.iter().any(|r| &r.url == url) {
                        processed_results.push(WebSearchResult {
                            url: url.clone(),
                            title: format!("Resultado de Spider en {}", massive_result.url),
                            description: format!(
                                "Encontrado via búsqueda masiva paralela en profundidad {}",
                                massive_result.depth
                            ),
                            main_text: String::new(), // Se llenará después si se crawlea
                            summary: String::new(),
                            word_count: 0,
                            headings: Vec::new(),
                            code_snippets: Vec::new(),
                            relevance: 0.6, // Relevancia para links encontrados
                            quality_score: 0.5,
                            source: "nuclear_spider".to_string(),
                        });
                    }
                }
            }
        }"""

old_code = """        // TODO: Fix massive_results processing - currently Vec<String> but code expects struct
        // Convertir resultados de MassiveParallelSearch a WebSearchResult
        /*
        for massive_result in &massive_results {
            if massive_result.success && massive_result.is_real_data {
                for url in &massive_result.urls_found {
                    if !processed_results.iter().any(|r| &r.url == url) {
                        processed_results.push(WebSearchResult {
                            url: url.clone(),
                            title: format!("Resultado de {}", massive_result.source),
                            description: format!(
                                "Encontrado via búsqueda masiva paralela en {}",
                                massive_result.source
                            ),
                            main_text: String::new(), // Se llenará después si se crawlea
                            summary: String::new(),
                            word_count: 0,
                            headings: Vec::new(),
                            code_snippets: Vec::new(),
                            relevance: massive_result.data_quality,
                            quality_score: massive_result.data_quality,
                            source: massive_result.source.clone(),
                        });
                    }
                }
            }
        }
        */"""

new_content = content.replace(old_code, replacement)

with open('src/core/web_search.rs', 'w') as f:
    f.write(new_content)
