# 🔥 ANÁLISIS ARQUITECTÓNICO - FLUJOS Y RELACIONES MCP

**Documento:** Relaciones internas, flujos de datos, decisiones arquitectónicas  
**Fecha:** 13 de enero de 2026

---

## 1. FLUJO COMPLETO: REQUEST → RESPONSE

### Ejemplo: Websearch Tool

```
┌─────────────────────────────────────────────────────────────────┐
│ CLIENT (Cursor, VS Code)                                         │
│                                                                   │
│ POST /mcp/tools/call                                            │
│ {                                                                │
│   "jsonrpc": "2.0",                                             │
│   "id": "req_123",                                              │
│   "method": "tools/call",                                       │
│   "params": {                                                   │
│     "name": "websearch",                                        │
│     "arguments": {"query": "machine learning"}                 │
│   }                                                              │
│ }                                                                │
└──────────────────────────┬──────────────────────────────────────┘
                           │ HTTP POST
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ MCP SERVER (Axum HTTP)                                           │
│                                                                   │
│ 1. Receive request on POST /mcp/tools/call                      │
│ 2. Parse JSON-RPC 2.0 request                                   │
│ 3. Validate format (jsonrpc="2.0", id present, method, params)  │
│ 4. Route to handle_tool_call()                                  │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ HANDLER: handle_tool_call()                                      │
│                                                                   │
│ let tool_name = "websearch"                                      │
│ let arguments = {"query": "machine learning"}                    │
│                                                                   │
│ Match tool_name:                                                │
│   "websearch" → execute_websearch()                             │
│   "premium" → execute_premium()                                 │
│   "file_search" → execute_file_search()                         │
│   "scan" → execute_scan()                                       │
│   "ai_dataset_trainer" → execute_ai_dataset_trainer()           │
│   _ → error (method not found)                                  │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ TOOL HANDLER: execute_websearch()                                │
│                                                                   │
│ Input: {"query": "machine learning"}                             │
│                                                                   │
│ 1. Extract query param from arguments                           │
│ 2. Validate query (non-empty)                                   │
│ 3. Apply rate limiting (wait())                                 │
│ 4. Check cache for "ws:machine learning:50:es"                 │
│    ├─ HIT: Return cached result                                │
│    └─ MISS: Continue to step 5                                 │
│ 5. Call WebSearchTool::search_real(query, max_results)          │
│ 6. Build response JSON                                          │
│ 7. Cache response                                               │
│ 8. Return MCPResponse::success(id, response)                    │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ TOOL IMPL: WebSearchTool::search_real()                          │
│                                                                   │
│ 1. Validate query ✓                                             │
│ 2. Check cache ✓                                                │
│ 3. Apply rate limiting ✓                                        │
│ 4. Initialize CoreWebSearch (if available)                      │
│ 5. Try Go processor for parallel (1000 goroutines)              │
│ 6. Fallback: HTTP requests a 55+ engines                        │
│    ├─ Google (en, es)                                          │
│    ├─ Bing                                                      │
│    ├─ DuckDuckGo                                                │
│    ├─ Ecosia                                                    │
│    ├─ StartPage                                                 │
│    ├─ Brave                                                     │
│    ├─ Yandex                                                    │
│    ├─ Google Scholar                                            │
│    └─ ArXiv                                                     │
│ 7. Try Zig SIMD for result processing (if available)            │
│ 8. Try DeepWeb (.onion sites) (if available)                   │
│ 9. Sort by relevance                                            │
│ 10. Truncate to max_results                                     │
│ 11. Cache results                                               │
│ 12. Return Vec<SearchResult>                                    │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ RESPONSE: SearchResult[]                                         │
│                                                                   │
│ MCPResponse {                                                    │
│   jsonrpc: "2.0",                                               │
│   id: "req_123",                                                │
│   result: {                                                     │
│     query: "machine learning",                                  │
│     results_count: 50,                                          │
│     results: [                                                  │
│       {                                                         │
│         url: "https://example.com",                            │
│         title: "Machine Learning Guide",                        │
│         snippet: "...",                                         │
│         source: "web",                                          │
│         relevance_score: 0.95                                   │
│       },                                                        │
│       ...                                                       │
│     ],                                                          │
│     source: "real_http_requests",                              │
│     cached: false                                               │
│   }                                                             │
│ }                                                               │
└──────────────────────────┬──────────────────────────────────────┘
                           │ JSON Serialized
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ CLIENT (Cursor, VS Code)                                         │
│                                                                   │
│ HTTP 200 OK                                                     │
│ Content-Type: application/json                                  │
│                                                                   │
│ {"jsonrpc":"2.0","id":"req_123","result":{...}}                │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. JERARQUÍA DE DECISIONES: TOOL ROUTING

```
START
  │
  ├─→ POST /mcp/tools/list
  │    └─→ get_tool_definitions()
  │         └─→ Return 5 tools (websearch, premium, file_search, scan, ai_dataset_trainer)
  │
  ├─→ POST /mcp/tools/call
  │    │
  │    └─→ Extract: name, arguments
  │         │
  │         ├─→ name == "websearch" ? execute_websearch()
  │         │    │
  │         │    ├─→ Rate limit (wait 1000/s)
  │         │    ├─→ Cache check
  │         │    ├─→ WebSearchTool::search()
  │         │    │    ├─→ Go FFI? (1000 goroutines)
  │         │    │    ├─→ HTTP fallback (55+ engines)
  │         │    │    └─→ Zig SIMD processing
  │         │    └─→ Return SearchResult[]
  │         │
  │         ├─→ name == "premium" ? execute_premium()
  │         │    │
  │         │    ├─→ Detect: URL or search?
  │         │    ├─→ URL: PremiumContentTool::fetch_url()
  │         │    │    ├─→ Nuclear Bypass
  │         │    │    ├─→ Nim FFI parsing
  │         │    │    └─→ Return PremiumContent
  │         │    │
  │         │    └─→ Search: PremiumContentTool::search()
  │         │         └─→ WebSearchTool + Premium extraction
  │         │
  │         ├─→ name == "file_search" ? execute_file_search()
  │         │    │
  │         │    ├─→ AdvancedFileSearchTool::analyze_file()
  │         │    ├─→ Detect: mocks, TODOs, errors, warnings
  │         │    ├─→ Calculate health_score
  │         │    └─→ Return FileAnalysisResult
  │         │
  │         ├─→ name == "scan" ? execute_scan()
  │         │    │
  │         │    ├─→ ScanWorkspaceTool::scan()
  │         │    ├─→ Collect files (recursive, exclude .git, target)
  │         │    ├─→ Analyze each file (mocks, TODOs, security, performance)
  │         │    ├─→ Calculate file health + overall health
  │         │    ├─→ Generate advice
  │         │    └─→ Return ScanResult
  │         │
  │         ├─→ name == "ai_dataset_trainer" ? execute_ai_dataset_trainer()
  │         │    │
  │         │    ├─→ AIDatasetTrainerTool::generate_dataset()
  │         │    ├─→ PHASE 1: Go parallel fetching (1000 goroutines)
  │         │    ├─→ PHASE 2: Zig SIMD deduplication
  │         │    ├─→ PHASE 3: Nim HTML parsing
  │         │    ├─→ PHASE 4: JAX GPU vectorization
  │         │    └─→ Return TrainingDataset
  │         │
  │         └─→ ELSE: METHOD NOT FOUND
  │              └─→ Error: -32601
  │
  └─→ POST /mcp/rpc (generic handler)
       │
       ├─→ method == "initialize" ? Return server info
       ├─→ method == "tools/list" ? Return tools
       ├─→ method == "tools/call" ? Delegate to handler
       └─→ ELSE: METHOD NOT FOUND

END
```

---

## 3. PIPELINE DETALLADO: AI_DATASET_TRAINER

```
┌──────────────────────────────────────────────────────────────────┐
│ INPUT: dataset_name, target_size                                 │
│        (e.g., "ml_papers_2024", 100000)                          │
└──────────────────────┬───────────────────────────────────────────┘
                       │
                       ▼
        ┌──────────────────────────────┐
        │ PHASE 1: Go Parallel         │
        │ Fetching (1000 goroutines)   │
        └──────────────┬───────────────┘
                       │
         ┌─────────────┴─────────────┐
         │                           │
         ▼                           ▼
    ✅ SUCCESS               ⚠️ TIMEOUT/ERROR
    Fetched:                Fallback to HTTP
    - 100K+ sources           sequential
    - 1000 concurrent
    - 60s timeout
    - 10 retries
         │                           │
         └─────────────┬─────────────┘
                       │
                       ▼ raw_content Vec<String>
        ┌──────────────────────────────┐
        │ PHASE 2: Zig SIMD            │
        │ Deduplication (Blake3 hashing)│
        └──────────────┬───────────────┘
                       │
         ┌─────────────┴──────────────────────┐
         │                                    │
         ▼                                    ▼
    ✅ SUCCESS                        ⚠️ NOT AVAILABLE
    Deduplicated:                     Skip deduplication
    - Hash each item (SIMD <1ms)       Use raw content
    - Remove duplicates                (fallback)
    - Keep 98K unique items
    - Removed: 2K duplicates
         │                                    │
         └─────────────┬──────────────────────┘
                       │
                       ▼ deduplicated Vec<(content, hash)>
        ┌──────────────────────────────┐
        │ PHASE 3: Nim HTML Parsing    │
        │ Text extraction + metadata   │
        └──────────────┬───────────────┘
                       │
         ┌─────────────┴──────────────────────┐
         │                                    │
         ▼                                    ▼
    ✅ SUCCESS                        ⚠️ NOT AVAILABLE
    Parsed:                           Use raw content
    - Extract text                     (fallback)
    - Extract metadata
    - JavaScript execution
    - 30s timeout per parse
    - Max 10MB content
    - 98K items parsed successfully
         │                                    │
         └─────────────┬──────────────────────┘
                       │
                       ▼ parsed Vec<(text, metadata, categories)>
        ┌──────────────────────────────┐
        │ PHASE 4: JAX GPU Vectorization│
        │ Embeddings + quality scoring │
        └──────────────┬───────────────┘
                       │
         ┌─────────────┴──────────────────────┐
         │                                    │
         ▼                                    ▼
    ✅ SUCCESS                        ⚠️ GPU NOT AVAILABLE
    Vectorized:                       Use fallback scoring
    - Generate 1536D embeddings        (CPU embeddings)
    - GPU acceleration (if available)
    - Batch processing (1024 items)
    - Quality scoring (0.0-1.0)
    - Processing metadata tracking
         │                                    │
         └─────────────┬──────────────────────┘
                       │
                       ▼
        ┌──────────────────────────────┐
        │ OUTPUT: TrainingDataset      │
        │                              │
        │ {                            │
        │   config: {...},             │
        │   datapoints: [              │
        │     {                        │
        │       id: "dp_001",          │
        │       text: "...",           │
        │       embedding: [...1536],  │
        │       quality_score: 0.85,   │
        │       processing_info: {...} │
        │     },                       │
        │     ...                      │
        │   ],                         │
        │   statistics: {              │
        │     total: 98000,            │
        │     avg_quality: 0.78,       │
        │     dedup_removed: 2000,     │
        │     processors_used: [       │
        │       "go_parallel",         │
        │       "zig_simd",            │
        │       "nim_parsing",         │
        │       "jax_embedding"        │
        │     ]                        │
        │   }                          │
        │ }                            │
        └──────────────────────────────┘
```

---

## 4. MATRIX DE INTEGRACIÓN FFI

| FFI Module | Tool | Función | Estado |
|-----------|------|---------|--------|
| **Go** | websearch | Parallel fetching (1000 goroutines) | ✅ Optional |
| **Go** | premium | Parallel content fetching | ✅ Optional |
| **Go** | ai_dataset_trainer | Phase 1: Parallel data fetching | ✅ Optional |
| **Zig** | websearch | SIMD result processing | ✅ Optional |
| **Zig** | premium | SIMD content hashing | ✅ Optional |
| **Zig** | ai_dataset_trainer | Phase 2: SIMD deduplication | ✅ Optional |
| **Zig** | dataset_generator | Integrity hashing | ✅ Optional |
| **Nim** | premium | HTML parsing + text extraction | ✅ Optional |
| **Nim** | ai_dataset_trainer | Phase 3: HTML parsing | ✅ Optional |
| **JAX** | ai_dataset_trainer | Phase 4: GPU vectorization | ✅ Optional |
| **JAX** | dataset_generator | Vectorization para datasets | ✅ Optional |

**Features:**
- Todos los FFI son **OPCIONALES**
- Si no disponible: **FALLBACK REAL A HTTP/CPU**
- No hay mocks, todas las alternativas son implementaciones reales
- Activadas/desactivadas por Cargo features: `go`, `zig`, `nim`, `jax`

---

## 5. CACHE STRATEGY

### Cache Locations & Keys

```
MCPServer
├── cache: Arc<Cache>  (5000 items)
│   │
│   ├─ websearch
│   │  └─ Key: "ws:{query}:{max_results}:{language}"
│   │     Value: JSON<SearchResult[]>
│   │     TTL: indefinido
│   │
│   ├─ premium
│   │  └─ Key: "{url}"
│   │     Value: JSON<PremiumContent>
│   │     TTL: indefinido
│   │
│   ├─ file_search
│   │  └─ Key: "{filepath}"
│   │     Value: JSON<FileAnalysisResult>
│   │     TTL: indefinido (file-based invalidation)
│   │
│   └─ ai_dataset_trainer
│      └─ Key: "{dataset_name}:{hash}"
│         Value: JSON<TrainingDatapoint[]>
│         TTL: indefinido
│
├── storage: Arc<IntelligentStorage>  (persistent)
│   │
│   └─ Use: Dataset export, search result archiving
│
└── rate_limiter: Arc<RateLimiter>
    │
    ├─ websearch: 1000 req/s, burst 2000
    ├─ premium: standard (100/s default)
    ├─ file_search: standard
    ├─ scan: standard
    └─ ai_dataset_trainer: standard
```

### Cache Hit/Miss Flow

```
execute_websearch("rust async")
  │
  ├─→ cache_key = "ws:rust async:50:es"
  │
  ├─→ cache.get_simple(cache_key)?
  │    │
  │    ├─→ HIT: Return cached SearchResult[]
  │    │         eprintln!("📦 Cache hit")
  │    │
  │    └─→ MISS: Continue to fetch
  │         │
  │         ├─→ rate_limiter.wait()
  │         ├─→ websearch.search_real()
  │         ├─→ json_result = serde_json::to_string(&results)
  │         ├─→ cache.set_simple(cache_key, json_result)
  │         └─→ Return fresh results
  │
  └─→ End
```

---

## 6. RATE LIMITING STRATEGY

### Configuración por Tool

```
MCPServer initialization:
  rate_limiter = RateLimiter::new(
    rate: 100,        // 100 requests/second base
    burst: 200        // 200 burst capacity
  )

Per-request application:
  In each execute_*() handler:
    server.rate_limiter.wait().await

Behavior:
  - Si requests < 100/s: No espera
  - Si 100/s < requests < 200/s: Espera corta
  - Si requests > 200/s: Espera larga + rate limited error posible

Response si rate limited:
  {
    "jsonrpc": "2.0",
    "id": "req_123",
    "error": {
      "code": -32001,
      "message": "Rate limited: Too many requests"
    }
  }
```

---

## 7. ERROR HANDLING CODES (JSON-RPC 2.0)

```
-32700: PARSE_ERROR
        Descripción: JSON inválido en request
        Causa: Cliente envía JSON malformado
        Solución: Validar JSON antes de enviar

-32600: INVALID_REQUEST
        Descripción: jsonrpc != "2.0" o structure inválida
        Causa: jsonrpc field missing o incorrecto
        Solución: Incluir "jsonrpc": "2.0"

-32601: METHOD_NOT_FOUND
        Descripción: method no existe
        Causa: method != "initialize" | "tools/list" | "tools/call"
        Solución: Usar método válido

-32602: INVALID_PARAMS
        Descripción: parámetros ausentes o tipos incorrectos
        Causa: "name" o "arguments" missing en tools/call
        Solución: Incluir params requeridos

-32603: INTERNAL_ERROR
        Descripción: Error interno del servidor
        Causa: Panic, database error, etc.
        Solución: Revisar logs del servidor

-32000: TOOL_EXECUTION_ERROR
        Descripción: Tool execution falló
        Causa: websearch timeout, network error, etc.
        Solución: Reintentar o usar fallback

-32001: RATE_LIMITED
        Descripción: Demasiadas requests
        Causa: > 200 requests/segundo
        Solución: Esperar antes de reintentar

-32002: UNAUTHORIZED
        Descripción: Sin autorización
        Causa: (Futuro: API key validation)
        Solución: Proporcionar credenciales válidas
```

---

## 8. REQUEST/RESPONSE TIMING

### Timeouts Configurados

```
Global:
  Tool execution timeout: 30 segundos (websearch default)

Per Tool:
  websearch:         30s
  premium:           15s
  file_search:       5s (local)
  scan:              60s (workspace)
  ai_dataset_trainer: 1200s (20 minutos para 100K items)

Per Sub-operation:
  Go parallel:       60s (per request)
  Zig SIMD:          <1s (per item)
  Nim parsing:       30s (per item)
  JAX vectorization: variable (batch dependent)
  HTTP requests:     30s (per request)
```

### Response Time Targets

```
websearch:         < 2s (cached)
               < 5s (1st request)

premium:           < 3s (cached)
               < 8s (1st request)

file_search:       < 500ms (small file)
               < 3s (large file)

scan:              < 10s (small workspace)
               < 60s (entire workspace)

ai_dataset_trainer: 5-20 min (100K items with GPU)
```

---

## 9. SEGURIDAD Y AISLAMIENTO

### Input Validation

```
websearch:
  - query: Non-empty string, max 500 chars
  - max_results: 1-1000
  - language: ISO 639-1 code

premium:
  - input: Valid URL or search phrase
  - type: auto | article | book | course
  - bypass_paywall: boolean

file_search:
  - path: Valid filesystem path, no .. traversal
  - query: Non-empty string

scan:
  - path: Valid filesystem path
  - recursive: boolean

ai_dataset_trainer:
  - dataset_name: Alphanumeric + underscore
  - target_size: 1000-10000000
  - sources: Valid URLs
```

### Output Sanitization

```
All SearchResult:
  - URLs: Validated as http(s)
  - Text: Truncated to reasonable length (50KB max)
  - HTML tags: Stripped

PremiumContent:
  - Content: Sanitized (no scripts)
  - Metadata: Type-checked

FileAnalysisResult:
  - Code snippets: Truncated (1000 chars)
  - File paths: Relative paths only (no /etc, /root)

ScanResult:
  - Issue suggestions: Safe text only
  - File paths: Relative paths only
```

---

## 10. MONITOREO Y LOGGING

### Log Levels (via eprintln!)

```
🔥 CRITICAL (always)
   - Server startup
   - Tool initialization
   - Fatal errors

✅ SUCCESS
   - Cache hit/miss
   - Successful execution
   - Results count

⚠️ WARNING
   - FFI unavailable (fallback used)
   - Timeout approaching
   - Partial failures

🔍 DEBUG (if enabled)
   - Request details
   - Parameter values
   - Processing steps
   - Performance metrics

Ejemplos:
  eprintln!("🔍 WebSearch: Searching for '{}'", query);
  eprintln!("✅ Cache hit para '{}'", query);
  eprintln!("⚠️ Go FFI error (fallback): {}", e);
  eprintln!("📊 Final results: {} items", results.len());
```

---

## 11. DATOS TÍPICOS POR TOOL

### WebSearch Response
```json
{
  "query": "rust async",
  "results_count": 50,
  "results": [
    {
      "url": "https://rust-lang.org/...",
      "title": "Async Rust Book",
      "snippet": "Learn async programming in Rust...",
      "source": "web",
      "relevance_score": 0.98
    },
    ...
  ],
  "source": "real_http_requests",
  "cached": false,
  "deep_search": false,
  "language": "es"
}
```

### Premium Response
```json
{
  "input": "https://medium.com/article",
  "input_type": "url",
  "content_type": "article",
  "title": "Article Title",
  "content": "Full article text...",
  "source": "https://medium.com/article",
  "access_method": "paywall_bypass",
  "full_content": true
}
```

### FileAnalysis Response
```json
{
  "file_path": "src/main.rs",
  "total_lines": 500,
  "issues": [
    {
      "file": "src/main.rs",
      "line_number": 45,
      "column": 10,
      "severity": "warning",
      "issue_type": "mock",
      "message": "Mock pattern detected: 'fake'",
      "code_snippet": "let data = fake_data();",
      "suggestion": "Replace with real implementation"
    },
    ...
  ],
  "summary": {
    "total_errors": 2,
    "total_warnings": 5,
    "mocks_found": 3,
    "todos_found": 2,
    "health_score": 0.82,
    "recommendations": ["Fix TODOs", "Remove mocks"]
  }
}
```

### Scan Response
```json
{
  "scanned_path": ".",
  "files_scanned": 45,
  "total_lines": 50000,
  "total_issues": 127,
  "issues_by_category": {
    "Todo": 15,
    "Mock": 8,
    "Security": 3,
    "Performance": 40
  },
  "issues_by_severity": {
    "Critical": 1,
    "Error": 5,
    "Warning": 50,
    "Info": 71
  },
  "top_issues": [...],
  "health_score": 0.76,
  "advice": [
    "⚡ ADVERTENCIA: Código necesita mejoras",
    "🎭 8 patrones mock/fake encontrados",
    "📝 15 TODOs pendientes"
  ],
  "scan_duration_ms": 2840
}
```

---

**Documento completo de relaciones y flujos**  
**Status:** ✅ DOCUMENTADO  
**Última actualización:** 13 de enero de 2026
