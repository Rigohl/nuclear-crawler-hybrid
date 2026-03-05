import re

with open('src/core/nuclear_core.rs', 'r') as f:
    content = f.read()

# Fix syntax error "let let main_text"
content = content.replace("let let main_text", "let main_text")

# Add code_snippets to ExtractedData struct
if "pub code_snippets: Vec<String>," not in content:
    content = content.replace("pub images: Vec<String>,", "pub images: Vec<String>,\n    pub code_snippets: Vec<String>,")

# Add code_snippets in extract_all
if "all_data.code_snippets.extend(data.code_snippets);" not in content:
    content = content.replace("all_data.images.extend(data.images);", "all_data.images.extend(data.images);\n                all_data.code_snippets.extend(data.code_snippets);")

# Update scrape_html
scrape_html_replacement = """    fn scrape_html(html: &str, _url: &str) -> Result<ExtractedData> {
        use scraper::{Html, Selector};

        let document = Html::parse_document(html);
        let link_selector = Selector::parse("a[href]").unwrap();
        let img_selector = Selector::parse("img[src]").unwrap();
        let code_selector = Selector::parse("pre, code").unwrap();

        let main_text = document.root_element().text().collect::<Vec<_>>().join(" ");

        let links = document
            .select(&link_selector)
            .filter_map(|el| el.value().attr("href"))
            .map(String::from)
            .collect();

        let images = document
            .select(&img_selector)
            .filter_map(|el| el.value().attr("src"))
            .map(String::from)
            .collect();

        let mut unique_snippets = Vec::new();
        for el in document.select(&code_selector) {
            let snippet = el.text().collect::<Vec<_>>().join("").trim().to_string();
            if !snippet.is_empty() && !unique_snippets.contains(&snippet) {
                unique_snippets.push(snippet);
            }
        }

        Ok(ExtractedData {
            main_text,
            word_count: 0, // Will be calculated later
            language: "unknown".to_string(),
            links,
            images,
            code_snippets: unique_snippets,
            metadata: HashMap::new(),
            structured_data: serde_json::Value::Null,
        })
    }"""

old_scrape_html_pattern = re.compile(r'fn scrape_html\(html: &str, _url: &str\) -> Result<ExtractedData> \{.*?\n    \}', re.DOTALL)
content = old_scrape_html_pattern.sub(scrape_html_replacement, content)

# Update scrape_json
if "code_snippets: Vec::new()," not in content.split("fn scrape_json")[1].split("}")[0]:
    content = content.replace("images: Vec::new(),\n                metadata: HashMap::new(),", "images: Vec::new(),\n                code_snippets: Vec::new(),\n                metadata: HashMap::new(),")

# Update scrape_api
if "code_snippets: Vec::new()," not in content.split("fn scrape_api")[1].split("}")[0]:
    content = content.replace("images: Vec::new(),\n                metadata: HashMap::new(),", "images: Vec::new(),\n                code_snippets: Vec::new(),\n                metadata: HashMap::new(),")

with open('src/core/nuclear_core.rs', 'w') as f:
    f.write(content)
