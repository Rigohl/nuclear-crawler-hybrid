with open('src/core/nuclear_core.rs', 'r') as f:
    content = f.read()

# Fix duplicate code_snippets
content = content.replace("pub code_snippets: Vec<String>,\n    pub code_snippets: Vec<String>,", "pub code_snippets: Vec<String>,")

with open('src/core/nuclear_core.rs', 'w') as f:
    f.write(content)
