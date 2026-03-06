import re
import sys

with open("src/core/web_search.rs", "r") as f:
    content = f.read()

# Replace `summary: "".to_string(), // TODO: Generate summary` with the actual call
old_line = 'summary: "".to_string(), // TODO: Generate summary'
new_line = 'summary: self.generate_summary(&extracted.main_text, &query),'

if old_line not in content:
    print("Could not find the line to replace")
    sys.exit(1)

new_content = content.replace(old_line, new_line)

with open("src/core/web_search.rs", "w") as f:
    f.write(new_content)

print("Updated search logic to call generate_summary")
