# ARCHITECTURE - Nuclear Crawler Hybrid

## System Overview

```
┌─────────────────────────────────────────────────────────┐
│         HTTP JSON-RPC 2.0 Server (Port 8079)            │
└──────────────────┬──────────────────────────────────────┘
                   │
        ┌──────────┼──────────┐
        │          │          │
    ┌───▼──┐  ┌───▼──┐  ┌───▼──┐
    │ 7 MCP Tools (Composable, Independent)          │
    └───┬──┴──┴──┬──┴──┴──┬──┘
        │        │        │
    ┌───▼────────▼────────▼─────────────────────┐
    │   37 Internal Modules                     │
    │  ┌─ Core (8)      ┌─ OSINT (5)           │
    │  ├─ FFI (5)       ├─ Infra (8)           │
    │  ├─ WASM (6)      └─ AI (2)              │
    └───┬────────────────────────────────────────┘
        │
    ┌───▼────────────────────────────────────┐
    │   5 FFI Backends                       │
    │  Go | Zig | Nim | JAX | Chapel        │
    └────────────────────────────────────────┘
```

## 7 MCP Tools (Root Level)

### 1. WEBSEARCH
**Massive web search across 55+ engines with 50K goroutine parallelism**

Dependencies:
- Go FFI (50,000 goroutines)
- Nim FFI (100-200x parsing speedup)
- Tantivy (full-text indexing)
- WASM (additional acceleration)
- Proxy rotation (anonymity)

### 2. PREMIUM_CONTENT
**Access paywall-protected content using 10 bypass methods + integrated lateral movement (21 techniques)**

Dependencies:
- Lateral movement intelligence (21 exploitation techniques)
- Credential theft & reuse (LSASS dumping)
- SMB/WMI/RDP/SSH exploitation
- CloudFlare bypass (Chrome Headless)
- Network proxies
- Go FFI (50,000 goroutines for parallel probes)
- PowerShell (real command execution)

### 3. SCAN_WORKSPACE
**Analyze files/folders: detect errors, warnings, vulnerabilities**

Dependencies:
- Static code analysis
- CVE database matching
- ML vulnerability detection
- Library scanning
- Internet-based fix suggestions

### 4. AI_DATASET_TRAINER
**Generate 5K-10K diversified datasets for ML testing**

Dependencies:
- Chapel AI (intelligent generation)
- WASM dataset extraction (50x speedup)
- Multi-language support
- Edge case synthesis

### 5. FILE_SEARCH
**Precision keyword search with exact line:column location**

Dependencies:
- Zig SIMD (256-bit vectorization)
- WASM acceleration (80x speedup)
- Regex pattern matching
- Error detection heuristics
- Go parallel processing

### 6. CODE_INTELLIGENCE
**ML-powered code analysis: vulnerability detection, fix suggestions**

Dependencies:
- ML model (Chapel AI)
- Static analysis
- Pattern detection
- Vulnerability matching
- Remediation database
- Zig SIMD (code vectorization)
- JAX GPU (ML inference)

### 7. INTELLIGENCE_OSINT
**Open Source Intelligence: public information gathering, risk analysis**

Dependencies:
- WebSearch (55+ engines)
- WHOIS queries
- DNS enumeration
- Website scraping
- IP databases
- Bayesian networks
- Game theory analysis
- Neural networks

## Module Organization

### Core (8 modules)
- lateral_movement_advanced.rs (21 techniques)
- exploit_engine.rs (CVE database, payloads)
- premium_content_scraper.rs
- web_search.rs
- dataset_generator.rs
- nuclear_core.rs
- data_management.rs
- url_helpers.rs

### FFI (5 backends)
- go_integration.rs (50K goroutines)
- zig_integration.rs (SIMD 256-bit)
- nim_integration.rs (100-200x parsing)
- jax_integration.rs (GPU acceleration)
- chapel_integration.rs (neural networks)

### WASM (6 modules)
- dataset_extractor.rs (50x speedup)
- data_search.rs (100x speedup)
- file_search.rs (80x speedup)
- neural_ops.rs (60x speedup)
- real_human_scraper.rs
- ultra_power.rs

### OSINT (5 modules)
- bayesian_networks_osint.rs
- game_theory_osint.rs
- neural_networks_osint.rs
- case_resolver_osint.rs
- nuclear_integration_osint.rs

### Infra (8 modules)
- cache.rs
- rate_limit.rs
- proxy_rotation.rs
- chromium_rendering.rs
- deepweb_tor.rs
- advanced_bypass.rs
- data_extraction.rs
- intelligent_storage.rs

### AI (2 modules)
- chatbot.rs
- huggingface_integration.rs

### MCP (7 tools)
- websearch.rs
- premium_content.rs
- file_search_advanced.rs
- scan_workspace.rs
- ai_dataset_trainer.rs
- lateral_movement_tool.rs
- code_intelligence_tool.rs

## Data Flow Example: WEBSEARCH

```
User Query
    ↓
MCP Server (websearch endpoint)
    ↓
Chapel AI (pre-optimization strategy)
    ↓
Go FFI (spawn 50,000 goroutines)
    ├─ Query engine 1 (Google)
    ├─ Query engine 2 (Bing)
    ├─ Query engine 3 (DuckDuckGo)
    └─ ... (50+ more in parallel)
    ↓
Nim FFI (parse HTML results - 100-200x faster)
    ↓
Tantivy (index results)
    ↓
WASM (additional ranking/filtering - 50x faster)
    ↓
Chapel AI (post-optimization: ranking, deduplication)
    ↓
JSON Response (results with metadata)
```

## Key Architecture Decisions

1. **Each Tool is Independent**
   - No tool depends on another tool
   - Tools can be invoked solo or in sequence
   - Master engine orchestrates but doesn't absorb

2. **Code Reuse via Dependencies**
   - Tools share Core, FFI, WASM modules
   - No code duplication
   - Single source of truth per capability

3. **Chapel AI Integration**
   - Pre-execution: strategy optimization
   - Post-execution: result enhancement
   - Per-tool optional (graceful degradation)

4. **Parallelism by Default**
   - Go FFI: 50,000 goroutines available
   - Zig SIMD: 256-bit vector operations
   - WASM: 50-100x speedup modules
   - No sequential operations where parallel possible

5. **Real Execution, No Mocks**
   - All tools hit real servers/data
   - FFI backends are actual compiled libraries
   - Fallbacks are real implementations, not stubs

## Performance Metrics

| Tool | Backend | Speedup | Parallelism |
|------|---------|---------|-------------|
| WEBSEARCH | Go + Nim | 100-200x | 50K goroutines |
| PREMIUM_CONTENT | Native + Go + Lateral | 50x | 50K goroutines |
| SCAN_WORKSPACE | Zig SIMD | 4x | CPU cores |
| AI_DATASET_TRAINER | Chapel + WASM | 50x | 50K goroutines |
| FILE_SEARCH | Zig + WASM | 80x | 50K goroutines |
| CODE_INTELLIGENCE | ML + Zig | 60x | GPU + CPU |
| INTELLIGENCE_OSINT | Go + Web | 10x | 50K goroutines |

---

**Last Updated**: 9 de febrero de 2026
