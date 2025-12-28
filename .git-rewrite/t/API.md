# 🔧 API REFERENCE

## MCP Tools

### 1. websearch

**Descripción**: Búsqueda masiva en 55 fuentes web

**Input Schema**:
```json
{
  "name": "websearch",
  "arguments": {
    "queries": ["query1", "query2", ...]  // 1-5 queries
  }
}
```

**Output**:
```json
{
  "tool": "websearch",
  "queries_count": 2,
  "total_execution_time_ms": 2000,
  "results": [
    {
      "url": "https://example.com/article",
      "title": "Article Title",
      "description": "Brief description...",
      "main_text": "Extracted content...",
      "word_count": 1500,
      "headings": ["H1: Title", "H2: Section"],
      "code_snippets": ["fn main() { ... }"],
      "relevance": 0.95,
      "quality_score": 0.90,
      "source": "github.com"
    }
  ],
  "modules_used": {
    "go_ffi": true,
    "zig_simd": false,
    "real_search_engines": true,
    "deep_web_search": true,
    "...": "..."
  }
}
```

---

### 2. file_search

**Descripción**: Búsqueda en archivos locales

**Input Schema**:
```json
{
  "name": "file_search",
  "arguments": {
    "search_term": "async fn",       // Término a buscar
    "path": "./src"                   // Directorio (opcional)
  }
}
```

**Output**:
```json
{
  "tool": "file_search",
  "results": [
    {
      "file_path": "src/main.rs",
      "line_number": 42,
      "line_content": "async fn process() {",
      "match_count": 1,
      "match_type": "content"
    }
  ]
}
```

---

### 3. analyzer

**Descripción**: Análisis de código con búsqueda automática de soluciones

**Input Schema**:
```json
{
  "name": "analyzer",
  "arguments": {
    "path": "/path/to/project"
  }
}
```

**Output**:
```json
{
  "tool": "analyzer",
  "project_stats": {
    "total_files": 45,
    "rust_files": 32,
    "total_lines": 15000
  },
  "errors": [
    {
      "file": "src/main.rs",
      "line": 42,
      "column": 5,
      "message": "expected `;`",
      "suggested_fix": "Add semicolon",
      "search_results": [...]
    }
  ],
  "warnings": [...],
  "modules_status": {...}
}
```

---

### 4. stats

**Descripción**: Estadísticas de todos los módulos

**Input Schema**:
```json
{
  "name": "stats",
  "arguments": {}
}
```

**Output**:
```json
{
  "tool": "stats",
  "modules": {
    "go_ffi": {"active": true, "goroutines": 100000},
    "zig_simd": {"active": false, "reason": "disabled"},
    "cache": {"entries": 5000, "hit_rate": 0.85},
    "storage": {"searches": 150, "results": 25000},
    "...": "..."
  },
  "system": {
    "uptime_ms": 3600000,
    "memory_mb": 512,
    "cpu_cores": 8
  }
}
```

---

## HTTP Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Welcome message |
| `/health` | GET | Health check |
| `/tools` | GET | List available tools |
| `/mcp` | POST | Generic JSON-RPC handler |
| `/mcp/initialize` | POST | Initialize MCP session |
| `/mcp/tools/list` | POST | List tools (MCP) |
| `/mcp/tools/call` | POST | Call tool (MCP) |
| `/call` | POST | Direct tool call |

### JSON-RPC Format

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "websearch",
    "arguments": {"queries": ["rust"]}
  }
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{...JSON results...}"
      }
    ]
  }
}
```

---

## Error Codes

| Code | Description |
|------|-------------|
| -32600 | Invalid Request |
| -32601 | Method not found |
| -32602 | Invalid params |
| -32603 | Internal error |
| -32700 | Parse error |

---

## Rate Limits

- **Default**: 100 requests/second
- **Burst**: 200 requests
- **Per-domain**: 10 requests/second

---

## Examples

### cURL - Búsqueda Web
```bash
curl -X POST http://localhost:8080/call \
  -H "Content-Type: application/json" \
  -d '{"name": "websearch", "arguments": {"queries": ["rust async tokio"]}}'
```

### PowerShell - Estadísticas
```powershell
$body = @{name="stats"; arguments=@{}} | ConvertTo-Json
Invoke-RestMethod -Uri "http://localhost:8080/call" -Method POST -Body $body -ContentType "application/json"
```

### Python - Analyzer
```python
import requests
response = requests.post(
    "http://localhost:8080/call",
    json={"name": "analyzer", "arguments": {"path": "."}}
)
print(response.json())
```

---

**Protocolo**: MCP 2025-06-18  
**Transporte**: HTTP (Axum) / STDIO
