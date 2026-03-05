with open('src/core/web_search.rs', 'r') as f:
    content = f.read()

# Update line 1983 equivalent (inside search_real)
content = content.replace("code_snippets: Vec::new(), // TODO: Extract code snippets", "code_snippets: extracted.code_snippets.clone(),")

# Update line ~845 (inside search loop)
content = content.replace("Vec::new(), // No code snippets in ExtractedData", "ext.code_snippets.clone(),")

with open('src/core/web_search.rs', 'w') as f:
    f.write(content)
