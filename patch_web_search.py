import sys

def patch_file():
    with open('src/core/web_search.rs', 'r') as f:
        content = f.read()

    target = """            if self.nim_integration.is_available() {
                eprintln!("🐉 Using Nim FFI for HTML parsing");
                // TODO: Implement Nim FFI processing
            }"""

    replacement = """            if self.nim_integration.is_available() {
                eprintln!("🐉 Using Nim FFI for HTML parsing");
                // Apply Nim FFI HTML parsing to enhance results
                for result in &mut fetched_results {
                    if let Ok(nim_parsed) = self.nim_integration.parse_html(&result.main_text, Some(&result.url)) {
                        if !nim_parsed.title.is_empty() && result.title == "No title" {
                            result.title = nim_parsed.title;
                        }

                        // Extract better code snippets if possible, or update word count
                        if result.word_count == 0 {
                            result.word_count = nim_parsed.word_count;
                        }

                        // Nim parsing might give better text
                        if nim_parsed.text_content.len() > result.main_text.len() {
                            result.main_text = nim_parsed.text_content;
                        }
                    }
                }
            }"""

    if target in content:
        new_content = content.replace(target, replacement)
        with open('src/core/web_search.rs', 'w') as f:
            f.write(new_content)
        print("Successfully patched src/core/web_search.rs")
    else:
        print("Target string not found in src/core/web_search.rs")

patch_file()
