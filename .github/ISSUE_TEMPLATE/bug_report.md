---
name: Bug report
about: Create a report to help us improve Nuclear Crawler Hybrid
title: "[BUG] "
labels: bug
assignees: ''

---

**Describe the bug**
A clear and concise description of what the bug is.

**MCP Tool Context**
Which MCP tool is affected?
- [ ] websearch
- [ ] deepweb_search
- [ ] premium_content_scraper
- [ ] file_search
- [ ] Other (server/infrastructure)

**To Reproduce**
Steps to reproduce the behavior:
1. Configure MCP client with '...'
2. Call tool with arguments '....'
3. Observe error '....'

**Expected behavior**
A clear and concise description of what you expected to happen.

**Tool invocation**
If applicable, provide the exact MCP tool call that triggered the bug:
```json
{
  "name": "websearch",
  "arguments": {
    "queries": ["example query"]
  }
}
```

**Error output**
```
Paste error message or logs here
```

**Environment:**
 - OS: [e.g. Windows 11, Ubuntu 22.04, macOS 14]
 - Rust version: [e.g. 1.75.0]
 - Nuclear Crawler version: [e.g. 0.1.0]
 - MCP Client: [e.g. VS Code, Cursor, Claude Desktop]
 - FFI Libraries: [e.g. Go 1.21, Zig 0.11, Nim 2.0]
 - Deployment: [e.g. Docker, native binary]

**Docker context (if applicable)**
- Docker version: [e.g. 24.0.0]
- Container logs: 
```
Paste relevant Docker logs here
```

**Additional context**
Add any other context about the problem here, such as:
- Network conditions
- Rate limiting encountered
- Specific search engines failing
- FFI library issues
