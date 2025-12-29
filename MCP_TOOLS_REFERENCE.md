# 🔧 MCP Tools Reference - Nuclear Crawler Hybrid

Complete reference documentation for all Model Context Protocol (MCP) tools provided by Nuclear Crawler Hybrid.

---

## Table of Contents

- [Overview](#overview)
- [Tool 1: WebSearch](#tool-1-websearch)
- [Tool 2: DeepWeb Search](#tool-2-deepweb-search)
- [Tool 3: Premium Content Scraper](#tool-3-premium-content-scraper)
- [Tool 4: File Search](#tool-4-file-search)
- [Error Handling](#error-handling)
- [Best Practices](#best-practices)

---

## Overview

Nuclear Crawler Hybrid provides 4 specialized MCP tools for advanced web search, deep web exploration, premium content extraction, and intelligent file analysis.

### Quick Reference

| Tool | Purpose | Timeout | Max Input | Parallelism |
|------|---------|---------|-----------|-------------|
| **websearch** | Multi-engine web search | 5s | 50 queries | 100K goroutines |
| **deepweb_search** | Tor/deep web exploration | 10s | 20 queries | Tor-limited |
| **premium_content_scraper** | Paywall bypass & extraction | 15s | 20 URLs | Parallel |
| **file_search** | Local file pattern matching | 8s | 10 searches | SIMD-accelerated |

---

## Tool 1: WebSearch

### Description

Massively parallel web search across 55+ search engines, returning 2,100+ URLs per query in under 2 seconds. Integrates with general search engines, code repositories, developer communities, package registries, academic sources, and technical documentation.

### Specification

```json
{
  "name": "websearch",
  "description": "Multi-engine web search with 55+ sources",
  "timeout": 5,
  "max_queries": 50,
  "parallelism": "100K goroutines (Go FFI)"
}
```

### Input Schema

```json
{
  "type": "object",
  "properties": {
    "queries": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "maxItems": 50,
      "description": "Array of search queries"
    },
    "max_results": {
      "type": "integer",
      "default": 100,
      "description": "Maximum results per query"
    },
    "engines": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "description": "Optional: Specific engines to use"
    }
  },
  "required": ["queries"]
}
```

### Example Usage

#### Basic Search

```json
{
  "name": "websearch",
  "arguments": {
    "queries": ["rust async programming"]
  }
}
```

#### Multiple Queries

```json
{
  "name": "websearch",
  "arguments": {
    "queries": [
      "tokio runtime",
      "axum web framework",
      "rust mcp protocol"
    ],
    "max_results": 200
  }
}
```

#### Engine-Specific Search

```json
{
  "name": "websearch",
  "arguments": {
    "queries": ["machine learning"],
    "engines": ["github", "arxiv", "stackoverflow"]
  }
}
```

### Response Format

```json
{
  "content": [
    {
      "type": "text",
      "text": "Search completed: 2,147 URLs found from 55 engines\n\n=== Results for 'rust async programming' ===\n\n[DuckDuckGo]\nhttps://rust-lang.github.io/async-book/\nhttps://tokio.rs/tokio/tutorial\n...\n\n[GitHub]\nhttps://github.com/tokio-rs/tokio\nhttps://github.com/rust-lang/async-book\n...\n\n[Stack Overflow]\nhttps://stackoverflow.com/questions/tagged/rust+async\n..."
    }
  ],
  "isError": false
}
```

### Search Sources

#### General Search Engines (10+)
- DuckDuckGo (HTML version, no JS)
- Bing (web + news)
- Brave Search (+ Goggles)
- Yandex
- Ecosia
- Qwant
- Startpage
- Mojeek
- Swisscows
- SearX instances

#### Code Repositories (7)
- **GitHub**: Repos, code, issues, discussions, topics (100+ URLs per query)
  - 10 pages pagination
  - 20 language filters
  - Stars filters (100, 1K, 10K+)
- GitLab: Projects, snippets
- Codeberg
- Gitee (Chinese GitHub)
- BitBucket
- SourceForge
- SourceHut

#### Developer Communities (16+)
- Stack Overflow (20 pages + tags)
- Reddit (12 tech subreddits)
- Dev.to
- Medium
- Hashnode
- Hacker News (via Algolia)
- TechCrunch
- Ars Technica
- The Verge

#### Package Registries (6+)
- Rust: crates.io, docs.rs, rust-lang.org
- JavaScript: npm
- Python: PyPI
- ML: HuggingFace (models + datasets + spaces)

#### Academic Sources (4+)
- arXiv
- Papers with Code
- Google Scholar
- Semantic Scholar

### Performance Characteristics

- **Average Query Time**: <2 seconds
- **URLs per Query**: 2,100+
- **Parallel Requests**: 100K goroutines
- **Success Rate**: 95%+
- **Cache Hit Rate**: 60-70% (5-minute TTL)

### Error Scenarios

| Error | Cause | Solution |
|-------|-------|----------|
| `Rate limit exceeded` | Too many requests | Wait and retry |
| `Timeout` | Network latency | Reduce max_results |
| `No results` | Invalid query | Reformulate query |
| `Engine unavailable` | Source is down | Other engines used |

---

## Tool 2: DeepWeb Search

### Description

Anonymous deep web and Tor network exploration. Routes requests through Tor for .onion domains, implements stealth mode with randomized fingerprints, and accesses underground sources while maintaining user privacy.

### Specification

```json
{
  "name": "deepweb_search",
  "description": "Tor/deep web search with anonymity",
  "timeout": 10,
  "max_queries": 20,
  "anonymity": "Tor SOCKS5 proxy"
}
```

### Input Schema

```json
{
  "type": "object",
  "properties": {
    "queries": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "maxItems": 20,
      "description": "Array of deep web search queries"
    },
    "use_tor": {
      "type": "boolean",
      "default": true,
      "description": "Route through Tor network"
    },
    "stealth_mode": {
      "type": "boolean",
      "default": true,
      "description": "Randomize headers and fingerprints"
    }
  },
  "required": ["queries"]
}
```

### Example Usage

#### Basic Deep Web Search

```json
{
  "name": "deepweb_search",
  "arguments": {
    "queries": ["security research papers"]
  }
}
```

#### Tor-Enabled Search

```json
{
  "name": "deepweb_search",
  "arguments": {
    "queries": [
      "darknet research",
      "privacy tools"
    ],
    "use_tor": true,
    "stealth_mode": true
  }
}
```

### Response Format

```json
{
  "content": [
    {
      "type": "text",
      "text": "Deep web search completed: 487 results\n\n=== Results via Tor ===\n[Hidden Wiki]\nhttp://zqktlwiuavvvqqt4ybvgvi7tyo4hjl5xgfuvpdf6otjiycgwqbym2qad.onion\n\n[Security Forums]\nhttp://...\n\n=== Clearnet Results ===\nhttps://example.com/security-research\n..."
    }
  ],
  "isError": false
}
```

### Sources

- **Tor Hidden Services**: .onion domains
- **Hidden Wikis**: Directories and indexes
- **Underground Forums**: Legal content only
- **Research Networks**: Academic deep web
- **Clearnet Fallback**: Regular web as backup

### Security Features

1. **Tor Integration**:
   - SOCKS5 proxy on port 9050
   - Circuit rotation every 10 minutes
   - Exit node randomization

2. **Stealth Mode**:
   - Randomized User-Agent headers
   - Fingerprint obfuscation
   - Request timing randomization
   - No cookies or tracking

3. **Privacy**:
   - No query logging
   - No result persistence (optional)
   - Anonymous connection pooling

### Limitations

- Slower than clearnet (10s timeout)
- Limited concurrent queries (20 max)
- Tor circuit build time adds latency
- Some .onion sites may be unreachable

---

## Tool 3: Premium Content Scraper

### Description

Extracts premium content from paywalled sources using multiple bypass techniques. Supports academic papers (ArXiv, journals), media sites (Medium, tech blogs), and technical documentation while respecting copyright and fair use.

### Specification

```json
{
  "name": "premium_content_scraper",
  "description": "Extract content from paywalled sources",
  "timeout": 15,
  "max_urls": 20,
  "bypass_methods": ["archive", "cache", "direct"]
}
```

### Input Schema

```json
{
  "type": "object",
  "properties": {
    "urls": {
      "type": "array",
      "items": {
        "type": "string",
        "format": "uri"
      },
      "maxItems": 20,
      "description": "URLs to scrape"
    },
    "extract_metadata": {
      "type": "boolean",
      "default": true,
      "description": "Extract metadata (author, date, etc.)"
    },
    "extract_citations": {
      "type": "boolean",
      "default": false,
      "description": "Extract citations and references"
    },
    "format": {
      "type": "string",
      "enum": ["text", "markdown", "html"],
      "default": "markdown",
      "description": "Output format"
    }
  },
  "required": ["urls"]
}
```

### Example Usage

#### Single Article

```json
{
  "name": "premium_content_scraper",
  "arguments": {
    "urls": ["https://medium.com/@user/premium-article"],
    "format": "markdown"
  }
}
```

#### Academic Paper with Citations

```json
{
  "name": "premium_content_scraper",
  "arguments": {
    "urls": ["https://arxiv.org/abs/2301.12345"],
    "extract_metadata": true,
    "extract_citations": true,
    "format": "text"
  }
}
```

#### Batch Scraping

```json
{
  "name": "premium_content_scraper",
  "arguments": {
    "urls": [
      "https://medium.com/@user/article1",
      "https://arxiv.org/abs/2301.11111",
      "https://techblog.com/premium-post"
    ]
  }
}
```

### Response Format

```json
{
  "content": [
    {
      "type": "text",
      "text": "=== Extracted: Article Title ===\n\nAuthor: John Doe\nDate: 2024-01-15\nURL: https://medium.com/@user/article\n\n---\n\n[Article content in markdown format]\n\n### Summary\nExtracted 2,456 words from premium content.\n\n### Metadata\n- Publication: Medium\n- Reading Time: 12 minutes\n- Tags: technology, programming, rust\n\n### Citations\n1. Reference 1...\n2. Reference 2..."
    }
  ],
  "isError": false
}
```

### Supported Sources

#### Academic
- **ArXiv**: Open-access preprints
- **Research Papers**: Academic journals
- **IEEE Xplore**: Technical papers
- **ACM Digital Library**: Computer science papers

#### Media
- **Medium**: Articles and blog posts
- **Tech Blogs**: Industry blogs
- **News Sites**: Tech news articles

#### Documentation
- **API Docs**: Premium API documentation
- **Technical Whitepapers**: Industry whitepapers

### Bypass Techniques

1. **Archive.org**: Internet Archive snapshots
2. **Google Cache**: Cached versions of pages
3. **Direct Access**: JavaScript-free rendering
4. **RSS Feeds**: Full-text RSS when available
5. **API Access**: Official APIs when possible

### Extraction Pipeline

```
URL Input → Paywall Detection
              ↓
    ┌─────────┼─────────┐
    ↓         ↓         ↓
Archive   Google    Direct
  .org    Cache    Access
    ↓         ↓         ↓
    └─────────┼─────────┘
              ↓
      HTML Parsing (Nim FFI)
              ↓
    Content Extraction
              ↓
  Metadata + Text + Citations
```

### Legal & Ethical Considerations

- **Fair Use**: For research and education
- **Copyright**: Respects robots.txt and Terms of Service
- **Rate Limiting**: Prevents abuse
- **No Redistribution**: Content for personal use only
- **Academic Sources**: Prioritizes open-access content

---

## Tool 4: File Search

### Description

Ultra-fast local file pattern matching and code analysis with Zig SIMD acceleration. Provides exact line numbers, context snippets, code complexity metrics, circular import detection, and automatic edit suggestions.

### Specification

```json
{
  "name": "file_search",
  "description": "SIMD-accelerated file search and analysis",
  "timeout": 8,
  "max_searches": 10,
  "acceleration": "Zig SIMD"
}
```

### Input Schema

```json
{
  "type": "object",
  "properties": {
    "search_term": {
      "type": "string",
      "description": "Pattern to search for (regex supported)"
    },
    "path": {
      "type": "string",
      "default": "./",
      "description": "Root directory to search"
    },
    "file_types": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "description": "File extensions to search (.rs, .py, etc.)"
    },
    "search_mode": {
      "type": "string",
      "enum": ["exact", "regex", "fuzzy", "semantic"],
      "default": "exact",
      "description": "Search matching mode"
    },
    "context_lines": {
      "type": "integer",
      "default": 3,
      "description": "Lines of context before/after match"
    },
    "max_depth": {
      "type": "integer",
      "default": 10,
      "description": "Maximum directory depth"
    },
    "analyze": {
      "type": "boolean",
      "default": false,
      "description": "Perform code analysis"
    }
  },
  "required": ["search_term"]
}
```

### Example Usage

#### Basic Pattern Search

```json
{
  "name": "file_search",
  "arguments": {
    "search_term": "async fn",
    "path": "./src",
    "file_types": [".rs"]
  }
}
```

#### Regex Search with Context

```json
{
  "name": "file_search",
  "arguments": {
    "search_term": "pub fn \\w+\\(",
    "search_mode": "regex",
    "context_lines": 5,
    "file_types": [".rs", ".toml"]
  }
}
```

#### Fuzzy Search

```json
{
  "name": "file_search",
  "arguments": {
    "search_term": "handl_request",
    "search_mode": "fuzzy",
    "path": "./src"
  }
}
```

#### Code Analysis

```json
{
  "name": "file_search",
  "arguments": {
    "search_term": "complexity",
    "analyze": true,
    "path": "./src/bin"
  }
}
```

### Response Format

```json
{
  "content": [
    {
      "type": "text",
      "text": "=== File Search Results ===\n\nFound 47 matches in 12 files\n\n--- src/bin/nuclear_ultimate.rs:274 ---\n271: impl SearchEngine {\n272:     /// WebSearch tool implementation\n273:     #[instrument(skip(self))]\n274:     pub async fn tool_websearch(&self, args: &Value) -> Result<Value> {\n275:         let queries: Vec<String> = args[\"queries\"]\n276:             .as_array()\n277:             .ok_or_else(|| anyhow!(\"queries must be array\"))?;\n\n--- src/web_search.rs:142 ---\n...\n\n=== Code Analysis ===\nComplexity Metrics:\n- Average cyclomatic complexity: 7.2\n- Functions >20 complexity: 3\n- Duplicated code blocks: 2\n\nCircular Imports: None detected\n\nSuggestions:\n1. Refactor SearchEngine::tool_websearch (complexity: 23)\n2. Extract duplicate logic in modules cache.rs:45 and rate_limit.rs:67"
    }
  ],
  "isError": false
}
```

### Search Modes

#### 1. Exact Match
- Fast, case-sensitive exact string matching
- Best for known identifiers
- SIMD-accelerated

#### 2. Regex Match
- Full regex pattern support
- Case-sensitive or insensitive
- Slower than exact, faster than fuzzy

#### 3. Fuzzy Match
- Levenshtein distance-based
- Tolerates typos and variations
- Good for exploratory searches

#### 4. Semantic Search
- Context-aware matching
- Understands code structure
- Best for concept searches

### Analysis Features

#### Code Complexity
- **Cyclomatic Complexity**: Branch count per function
- **Cognitive Complexity**: Human readability metric
- **Function Length**: Lines per function
- **Nesting Depth**: Maximum indentation levels

#### Duplication Detection
- **Exact Duplicates**: Identical code blocks
- **Near Duplicates**: Similar with minor changes
- **Structural Clones**: Same logic, different syntax

#### Import Analysis
- **Circular Imports**: Detects import cycles
- **Unused Imports**: Identifies unused dependencies
- **Import Depth**: Module dependency depth

#### Auto Suggestions
- **Refactoring**: Complexity reduction suggestions
- **DRY**: Duplicate code elimination
- **Performance**: Optimization opportunities

### Performance

| Operation | Files | Time | Acceleration |
|-----------|-------|------|--------------|
| **Exact Search** | 10K | <100ms | Zig SIMD |
| **Regex Search** | 10K | <500ms | Parallel |
| **Fuzzy Search** | 10K | <1s | SIMD+Parallel |
| **Code Analysis** | 1K | <2s | Zig+Rust |

---

## Error Handling

### Common Errors

| Error Code | Description | Solution |
|------------|-------------|----------|
| `ERR_RATE_LIMIT` | Too many requests | Wait and retry |
| `ERR_TIMEOUT` | Operation exceeded timeout | Reduce input size |
| `ERR_INVALID_INPUT` | Malformed arguments | Check input schema |
| `ERR_NETWORK` | Network failure | Check connectivity |
| `ERR_PARSE` | Parsing error | Validate URL/path |
| `ERR_STORAGE` | Cannot save results | Check disk space |
| `ERR_FFI` | FFI library error | Verify libraries installed |

### Error Response Format

```json
{
  "content": [
    {
      "type": "text",
      "text": "Error: Rate limit exceeded\n\nDetails: You've exceeded the rate limit of 100 requests per second. Please wait 5 seconds and retry.\n\nRetry after: 5 seconds\nCurrent usage: 120/100 req/s"
    }
  ],
  "isError": true
}
```

---

## Best Practices

### 1. Query Optimization

**WebSearch**:
- Use specific, targeted queries
- Limit to 5-10 queries per call
- Leverage caching for repeated queries
- Specify engines when possible

**DeepWeb**:
- Be patient with Tor latency
- Use clearnet fallback
- Limit concurrent requests
- Respect rate limits

**Premium Scraper**:
- Batch URLs when possible
- Use markdown format for readability
- Extract metadata for context
- Check robots.txt compliance

**File Search**:
- Use exact match for known patterns
- Limit max_depth for large repos
- Enable analysis sparingly
- Filter by file_types

### 2. Performance Tips

- **Cache Hits**: Reuse queries within 5 minutes
- **Batch Operations**: Group related requests
- **Async Calls**: Don't wait for results sequentially
- **Resource Limits**: Stay within max query limits

### 3. Error Recovery

```javascript
// Example: Retry with exponential backoff
async function searchWithRetry(query, maxRetries = 3) {
  for (let i = 0; i < maxRetries; i++) {
    try {
      return await mcp.callTool('websearch', { queries: [query] });
    } catch (err) {
      if (err.code === 'ERR_RATE_LIMIT') {
        await sleep(Math.pow(2, i) * 1000);
        continue;
      }
      throw err;
    }
  }
}
```

### 4. Security Considerations

- **Input Validation**: Sanitize user inputs
- **URL Verification**: Validate URLs before scraping
- **Rate Limiting**: Respect server limits
- **Privacy**: No sensitive data in queries
- **Legal Compliance**: Follow Terms of Service

---

## Integration Examples

### VS Code Extension

```typescript
import { MCPClient } from '@modelcontextprotocol/sdk';

const client = new MCPClient({
  url: 'http://127.0.0.1:8079'
});

// WebSearch
const results = await client.callTool('websearch', {
  queries: ['rust async await'],
  max_results: 100
});

console.log(results.content[0].text);
```

### Python Script

```python
import requests

MCP_URL = "http://127.0.0.1:8079/tools/call"

def web_search(queries):
    response = requests.post(MCP_URL, json={
        "name": "websearch",
        "arguments": {
            "queries": queries
        }
    })
    return response.json()

results = web_search(["machine learning"])
print(results["content"][0]["text"])
```

### Bash/cURL

```bash
# WebSearch
curl -X POST http://127.0.0.1:8079/tools/call \
  -H "Content-Type: application/json" \
  -d '{
    "name": "websearch",
    "arguments": {
      "queries": ["rust programming"]
    }
  }'

# File Search
curl -X POST http://127.0.0.1:8079/tools/call \
  -H "Content-Type: application/json" \
  -d '{
    "name": "file_search",
    "arguments": {
      "search_term": "async fn",
      "path": "./src"
    }
  }'
```

---

## Monitoring & Metrics

### Tool Usage Metrics

Track tool performance:

- **Call Count**: Total invocations per tool
- **Success Rate**: Successful vs. failed calls
- **Average Duration**: Mean execution time
- **Cache Hit Rate**: Percentage of cached responses
- **Error Rate**: Failures per tool

### Example Metrics

```
websearch:
  - Calls: 1,234
  - Success: 98.5%
  - Avg Duration: 1.8s
  - Cache Hits: 68%
  
deepweb_search:
  - Calls: 87
  - Success: 94.2%
  - Avg Duration: 9.2s
  - Cache Hits: 12%
  
premium_content_scraper:
  - Calls: 456
  - Success: 91.7%
  - Avg Duration: 12.3s
  - Cache Hits: 45%
  
file_search:
  - Calls: 789
  - Success: 99.1%
  - Avg Duration: 0.6s
  - Cache Hits: 0% (no cache)
```

---

## Troubleshooting

### Tool Not Responding

**Check server status**:
```bash
curl http://localhost:8079/
```

**Restart server**:
```bash
docker restart nuclear-mcp-server
```

### Slow Performance

**Enable debug logging**:
```bash
RUST_LOG=debug ./nuclear-mcp --port 8079
```

**Check resource usage**:
```bash
docker stats nuclear-mcp-server
```

### FFI Errors

**Verify libraries**:
```bash
ldd ./target/release/nuclear-mcp
ls -la ./libs/
```

**Rebuild FFI**:
```bash
./scripts/build-ffi.sh
```

---

## Version History

### v0.1.0 (Current)

- Initial release with 4 tools
- 55+ search engines
- Tor integration
- Premium content extraction
- SIMD file search
- 100K goroutines parallelism

### Roadmap

- [ ] v0.2.0: GraphQL API
- [ ] v0.3.0: WebSocket transport
- [ ] v0.4.0: ML-powered result ranking
- [ ] v1.0.0: Production-ready stable release

---

**Last Updated**: 2025-12-29  
**Document Version**: 1.0.0  
**MCP Protocol**: 2025-01-01
