pub fn generate_summary(text: &str, query: &str) -> String {
    // Scoring sentences based on query keyword frequency, position, and length
    let query_terms: Vec<String> = query
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let sentences: Vec<&str> = text
        .split(|c| c == '.' || c == '!' || c == '?')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if sentences.is_empty() {
        // Fallback: simple truncation
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.len() > 50 {
            return words[..50].join(" ") + "...";
        }
        return text.to_string();
    }

    let mut scored_sentences: Vec<(usize, &str, f32)> = Vec::new();

    for (i, sentence) in sentences.iter().enumerate() {
        let mut score = 0.0;
        let sentence_lower = sentence.to_lowercase();
        let words: Vec<&str> = sentence_lower.split_whitespace().collect();
        let length = words.len();

        // 1. Length score (prefer sentences between 10 and 30 words)
        if length >= 10 && length <= 30 {
            score += 1.0;
        } else if length > 30 {
            score += 0.5;
        }

        // 2. Position score (early sentences often contain better summaries)
        if i < 3 {
            score += 2.0;
        } else if i < 10 {
            score += 1.0;
        }

        // 3. Keyword frequency score
        for term in &query_terms {
            if sentence_lower.contains(term) {
                score += 3.0; // High weight for query terms
            }
        }

        scored_sentences.push((i, sentence, score));
    }

    // Sort by score descending
    scored_sentences.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    // Take top 3 sentences
    let mut top_sentences: Vec<(usize, &str)> = scored_sentences
        .into_iter()
        .take(3)
        .map(|(i, s, _)| (i, s))
        .collect();

    // Sort by original position to maintain flow
    top_sentences.sort_by(|a, b| a.0.cmp(&b.0));

    let summary = top_sentences
        .into_iter()
        .map(|(_, s)| format!("{}.", s))
        .collect::<Vec<String>>()
        .join(" ");

    if summary.is_empty() {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.len() > 50 {
            return words[..50].join(" ") + "...";
        }
        return text.to_string();
    }

    summary
}
