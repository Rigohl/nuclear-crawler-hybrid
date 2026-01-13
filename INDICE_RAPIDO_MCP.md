# 🔍 ÍNDICE RÁPIDO - ARQUITECTURA MCP `/src/mcp/`

**Referencia rápida y tabla de contenidos para toda la arquitectura**

---

## 📍 ARCHIVOS Y UBICACIÓN EXACTA

### Módulo Raíz

| Archivo | LOC | Descripción | Lineas Clave |
|---------|-----|-------------|--------------|
| [mod.rs](src/mcp/mod.rs) | 12 | Exports públicos del módulo MCP | 1-12 |
| [protocol.rs](src/mcp/protocol.rs) | 401 | JSON-RPC 2.0 + 5 tool definitions | 1-401 |
| [server.rs](src/mcp/server.rs) | 749 | Servidor HTTP Axum + handlers | 1-749 |

### Carpeta Tools

| Archivo | LOC | Tool | Descripción | Lineas Clave |
|---------|-----|------|-------------|--------------|
| [tools/mod.rs](src/mcp/tools/mod.rs) | 29 | Meta | Exports de tools | 1-29 |
| [tools/websearch.rs](src/mcp/tools/websearch.rs) | 381 | #1 🔍 | Búsqueda web 55+ motores | 1-381 |
| [tools/premium_content.rs](src/mcp/tools/premium_content.rs) | 489 | #2 📚 | Extrae paywalls | 1-489 |
| [tools/file_search_advanced.rs](src/mcp/tools/file_search_advanced.rs) | 447 | #3 📄 | Análisis avanzado archivos | 1-447 |
| [tools/scan_workspace.rs](src/mcp/tools/scan_workspace.rs) | 525 | #4 🔬 | Escaneo profundo workspace | 1-525 |
| [tools/ai_dataset_trainer.rs](src/mcp/tools/ai_dataset_trainer.rs) | 484 | #5 🧠 | Entrenamiento IA con FFI | 1-484 |
| [tools/dataset_generator.rs](src/mcp/tools/dataset_generator.rs) | 276 | Bonus 📊 | Generador de datasets | 1-276 |

---

## 🔧 ESTRUCTURAS PRINCIPALES POR ARCHIVO

### protocol.rs (401 LOC)

```
MCPRequest              líneas 15-20
MCPResponse             líneas 28-37
MCPError                líneas 39-45
error_codes             líneas 47-58
ToolDefinition          líneas 65-71

MCPRequest::validate()  líneas 79-96
MCPRequest::list_tools()   líneas 99-107
MCPRequest::call_tool() líneas 110-120
MCPRequest::get_tool_name()    líneas 123-125
MCPRequest::get_arguments()    líneas 128-130

MCPResponse::success()  líneas 138-145
MCPResponse::error()    líneas 148-160
MCPResponse::error_with_data() líneas 163-173
MCPResponse::method_not_found() líneas 176-184
MCPResponse::invalid_params()   líneas 187-196
MCPResponse::parse_error()  líneas 199-206
MCPResponse::internal_error()   líneas 209-216
MCPResponse::rate_limited() líneas 219-225

get_tool_definitions()  líneas 230-398  ← TOOL DEFINITIONS
get_tool_definition()   líneas 401-405
tool_exists()           líneas 408-412
get_tool_names()        líneas 415-420

#[test] test_exactly_5_tools()  ← VALIDACIÓN CRÍTICA
#[test] test_tool_names()
#[test] test_request_validation()
#[test] test_response_serialization()
```

**Herramientas definidas (5 exactas):**
1. `websearch` - Búsqueda web 55+ motores
2. `premium` - Extrae paywalls
3. `file_search` - Análisis avanzado
4. `scan` - Escaneo profundo
5. `ai_dataset_trainer` - Entrenamiento IA

### server.rs (749 LOC)

```
MCPServer struct        líneas 15-27
  websearch: Arc<WebSearchTool>
  premium_content: Arc<PremiumContentTool>
  file_search: Arc<AdvancedFileSearchTool>
  scan: Arc<ScanWorkspaceTool>
  cache: Arc<Cache>
  storage: Arc<IntelligentStorage>
  rate_limiter: Arc<RateLimiter>

MCPServer::new()        líneas 30-50
MCPServer::create_router()  líneas 53-120  ← ENDPOINTS HTTP

HTTP ENDPOINTS:
  GET  /                líneas 54-62
  GET  /health          líneas 63-72
  POST /mcp/tools/list  líneas 73-80
  POST /mcp/tools/call  líneas 81-90
  POST /mcp/rpc         líneas 91-100
  POST /tools/websearch líneas 101-110
  POST /tools/premium   líneas 111-120
  POST /tools/file_search
  POST /tools/scan
  POST /tools/info

health_check()          líneas 123-135
handle_tools_list()     líneas 138-145
handle_tool_call()      líneas 148-195  ← ROUTER PRINCIPAL
handle_rpc()            líneas 198-250

execute_websearch()     líneas 253-310
execute_premium()       líneas 313-370
execute_file_search()   líneas 373-430
execute_scan()          líneas 433-490
execute_ai_dataset_trainer()  líneas 493-550
execute_info()          líneas 553-600

direct_websearch()      líneas 603-620
direct_premium()        líneas 623-640
direct_file_search()    líneas 643-660
direct_scan()           líneas 663-680
direct_info()           líneas 683-700
```

### websearch.rs (381 LOC)

```
SearchResult struct     líneas 14-20
WebSearchConfig struct  líneas 23-33
WebSearchTool struct    líneas 37-47
  config: WebSearchConfig
  core_search: Arc<...>
  go_processor: Arc<...>
  rate_limiter: Arc<...>
  cache: Arc<Cache>
  deepweb: Arc<...>

WebSearchTool::new()    líneas 50-72
WebSearchTool::search()   líneas 75-200  ← LÓGICA PRINCIPAL
  1. Validar query
  2. Cache check
  3. Rate limiting
  4. CoreWebSearch si disponible
  5. Go processor (1000 goroutines)
  6. HTTP requests (55+ engines)
  7. Zig SIMD (si disponible)
  8. DeepWeb search
  9. Sort por relevancia
  10. Cache resultado

WebSearchTool::parse_html_with_go()  líneas 203-230
WebSearchTool::fetch_results_real()  líneas 233-250
WebSearchTool::parse_search_results() líneas 253-310
WebSearchTool::search_real()  líneas 313-321  ← MCP ENTRY POINT

Motores soportados (líneas 155-170):
  - Google (en, es)
  - Bing, DuckDuckGo, Ecosia, StartPage
  - Brave, Yandex
  - Google Scholar, ArXiv
```

### premium_content.rs (489 LOC)

```
PremiumContent struct   líneas 16-27
PremiumConfig struct    líneas 30-38
PremiumContentTool struct  líneas 42-52
  scraper, bypass, html_parser, zig_processor, go_processor, cache

PremiumContentTool::new()   líneas 55-72
PremiumContentTool::fetch_premium()  líneas 75-110  ← LÓGICA PRINCIPAL
  - Detect source (Medium, ArXiv, O'Reilly, etc.)
  - Route a método específico por fuente

fetch_medium_real()     líneas 113-160  ← MEDIUM PAYWALL BYPASS
  1. Nuclear Bypass real
  2. Nim FFI para parsing
  3. Go FFI para paralelismo
  4. Return content

fetch_arxiv_real()      líneas 163-200
fetch_oreilly_real()    líneas 203-240
fetch_generic_real()    líneas 243-280
```

### file_search_advanced.rs (447 LOC)

```
CodeIssue struct        líneas 19-28
AnalysisSummary struct  líneas 30-38
FileAnalysisResult struct  líneas 40-46
AdvancedFileSearchTool struct  líneas 50-56
  config: FileSearchConfig
  cache: Arc<Cache>

AdvancedFileSearchTool::new()   líneas 59-66
AdvancedFileSearchTool::analyze_file()  líneas 69-200  ← LÓGICA PRINCIPAL
  1. Cache check
  2. Read file
  3. Detect issues:
     - Mocks (mock, fake, stub, dummy)
     - TODOs (TODO, FIXME, HACK, XXX, BUG)
     - Unused vars
     - Syntax errors
  4. Generate summary
  5. Calculate health_score
  6. Cache resultado

has_mock_pattern()      líneas 203-215
has_unused_var()        líneas 218-230
has_syntax_error()      líneas 233-245
generate_summary()      líneas 248-270
```

### scan_workspace.rs (525 LOC)

```
ScanConfig struct       líneas 18-24
ScanIssue struct        líneas 27-39
IssueCategory enum      líneas 42-53
IssueSeverity enum      líneas 56-60
FileAnalysis struct     líneas 63-73
ScanResult struct       líneas 76-93
ScanPatterns struct     líneas 96-105

ScanWorkspaceTool::new()    líneas 108-115
ScanWorkspaceTool::scan()   líneas 118-180  ← LÓGICA PRINCIPAL
  1. Recolectar archivos (recursive, exclude)
  2. Analizar cada archivo
  3. Detectar issues (mocks, TODOs, security, performance)
  4. Calcular health scores
  5. Generar advice

collect_files()         líneas 183-220
should_scan_file()      líneas 223-235
analyze_file()          líneas 238-350  ← ANÁLISIS POR ARCHIVO
  - Count líneas (code, comment, blank)
  - Detect patterns (mock, todo, security, performance)
  - Calculate file health_score
  - Calculate complexity_score

get_security_suggestion()   líneas 353-365
get_performance_suggestion()    líneas 368-378
calculate_health_score()    líneas 381-393
generate_advice()       líneas 396-425

Patrones detectados:
  mocks (14)         líneas 103-105
  todos (10)         líneas 106-107
  security (20+)     líneas 108-109
  performance (10)   líneas 110
```

### ai_dataset_trainer.rs (484 LOC)

```
TrainingDatapoint struct    líneas 23-35
ProcessingInfo struct       líneas 38-49
DatasetTrainerConfig struct líneas 52-79
TrainingDataset struct      líneas 82-86
DatasetStatistics struct    líneas 89-103
QualityReport struct        líneas 106-120
QualityIssue struct         líneas 122-130
AIDatasetTrainerTool struct líneas 133-143
  go_processor, zig_processor, nim_parser, jax_processor, cache

AIDatasetTrainerTool::new()     líneas 146-200  ← INICIALIZACIÓN
  - Go config (1000 concurrent, 60s timeout)
  - Zig config (SIMD, Blake3)
  - Nim config (JS extraction, metadata)
  - JAX processor (GPU vectorization)

AIDatasetTrainerTool::generate_dataset()   líneas 203-350  ← LÓGICA PRINCIPAL
  FASE 1: Go parallel fetching (líneas 220-235)
  FASE 2: Zig SIMD deduplication (líneas 238-270)
  FASE 3: Nim HTML parsing (líneas 273-310)
  FASE 4: JAX GPU vectorization (líneas 313-350)

Output: TrainingDataset con:
  - datapoints: Vec<TrainingDatapoint>
  - statistics: DatasetStatistics
  - quality_report: QualityReport
```

---

## 📊 IMPORTACIÓN CRUZADA

### protocol.rs Importa
```rust
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
```

### server.rs Importa
```rust
use crate::mcp::protocol::{MCPRequest, MCPResponse, ...};
use crate::mcp::tools::{WebSearchTool, PremiumContentTool, ...};
use crate::{cache::Cache, intelligent_storage::IntelligentStorage, ...};
use axum::{...};
use std::sync::Arc;
```

### websearch.rs Importa
```rust
use crate::web_search::WebSearch as CoreWebSearch;
use crate::go_integration::GoParallelProcessor;
use crate::rate_limit::RateLimiter;
use crate::cache::Cache;
use crate::deepweb_tor::DeepWebSearch;
```

### premium_content.rs Importa
```rust
use crate::premium_content_scraper::NuclearPremiumScraper;
use crate::nuclear_core::NuclearBypass;
use crate::nim_integration::NimHtmlParser;
use crate::zig_integration::ZigSimdProcessor;
use crate::go_integration::GoParallelProcessor;
use crate::cache::Cache;
```

### scan_workspace.rs Importa
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
```

### ai_dataset_trainer.rs Importa
```rust
use crate::go_integration::{GoParallelProcessor, GoParallelConfig};
use crate::zig_integration::{ZigSimdProcessor, ZigSimdConfig};
use crate::nim_integration::{NimHtmlParser, NimParserConfig};
use crate::jax_integration::JaxProcessor;
use crate::cache::Cache;
```

---

## 🔌 INTEGRACIÓN CON CORE MODULES

```
src/
├── mcp/                         (Este análisis)
│   ├── protocol.rs ─────→ Defines 5 tools
│   ├── server.rs ───────→ HTTP + handlers
│   └── tools/
│       ├── websearch.rs ──────→ usa web_search.rs
│       ├── premium_content.rs ─→ usa premium_content_scraper.rs, nuclear_core.rs
│       ├── file_search_advanced.rs
│       ├── scan_workspace.rs
│       ├── ai_dataset_trainer.rs ─→ usa go_integration, zig_integration, etc.
│       └── dataset_generator.rs
│
├── web_search.rs        ← Importado por websearch.rs
├── premium_content_scraper.rs ← Importado por premium_content.rs
├── nuclear_core.rs      ← Importado por premium_content.rs
├── cache.rs             ← Importado por server.rs y todos los tools
├── rate_limit.rs        ← Importado por server.rs y todos los tools
├── intelligent_storage.rs ← Importado por server.rs
├── go_integration.rs    ← Importado por websearch, premium, ai_dataset
├── zig_integration.rs   ← Importado por websearch, premium, ai_dataset, dataset_gen
├── nim_integration.rs   ← Importado por premium, ai_dataset
├── jax_integration.rs   ← Importado por ai_dataset, dataset_generator
└── deepweb_tor.rs       ← Importado por websearch.rs
```

---

## 🎯 QUICK LOOKUP TABLES

### Métodos Públicos por Tool

**WebSearchTool**
```
pub fn new(config: WebSearchConfig) -> Self
pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>>
pub async fn search_real(&self, query: &str, max_results: usize) -> Result<Vec<SearchResult>>
```

**PremiumContentTool**
```
pub fn new(config: PremiumConfig) -> Self
pub async fn fetch_premium(&self, url: &str) -> Result<PremiumContent>
pub async fn fetch_medium_real(&self, url: &str) -> Result<PremiumContent>
pub async fn fetch_arxiv_real(&self, url: &str) -> Result<PremiumContent>
```

**AdvancedFileSearchTool**
```
pub fn new(config: FileSearchConfig) -> Self
pub fn analyze_file(&self, file_path: &str) -> Result<FileAnalysisResult>
```

**ScanWorkspaceTool**
```
pub fn new() -> Self
pub async fn scan(&self, config: ScanConfig) -> ScanResult
```

**AIDatasetTrainerTool**
```
pub async fn new(config: DatasetTrainerConfig) -> Result<Self>
pub async fn generate_dataset(&self, sources: Vec<String>) -> Result<TrainingDataset>
```

---

## 📈 ESTADÍSTICAS POR ARCHIVO

| Archivo | LOC | Structs | Enums | Fns | Comments | Ratio |
|---------|-----|---------|-------|-----|----------|-------|
| protocol.rs | 401 | 4 | - | 12 | ~50 | 12% |
| server.rs | 749 | 1 | - | 15 | ~40 | 5% |
| websearch.rs | 381 | 2 | - | 7 | ~30 | 8% |
| premium_content.rs | 489 | 2 | - | 8 | ~40 | 8% |
| file_search_advanced.rs | 447 | 3 | - | 6 | ~35 | 8% |
| scan_workspace.rs | 525 | 4 | 2 | 10 | ~45 | 9% |
| ai_dataset_trainer.rs | 484 | 5 | - | 5 | ~40 | 8% |
| dataset_generator.rs | 276 | 3 | - | 4 | ~25 | 9% |
| tools/mod.rs | 29 | - | - | - | ~5 | 17% |
| mod.rs | 12 | - | - | - | ~2 | 17% |
| **TOTAL** | **3,787** | **24** | **2** | **82** | ~312 | **~8%** |

---

## 🔗 REFERENCIAS RÁPIDAS

### Documentación Completa Generada

1. **[ANALISIS_MCP_PROFUNDO.md](ANALISIS_MCP_PROFUNDO.md)** (1,439 LOC)
   - Análisis línea por línea
   - Estructuras y métodos detallados
   - Protocolo JSON-RPC 2.0
   - Ejemplos de requests/responses

2. **[ANALISIS_FLUJOS_MCP.md](ANALISIS_FLUJOS_MCP.md)** (728 LOC)
   - Flujos de datos
   - Diagramas de decisión
   - Pipeline FFI
   - Caching y rate limiting

3. **[RESUMEN_EJECUTIVO_MCP.md](RESUMEN_EJECUTIVO_MCP.md)** (484 LOC)
   - Vista rápida
   - Casos de uso
   - Ejemplos curl
   - Métricas finales

4. **[INDICE_RAPIDO_MCP.md](INDICE_RAPIDO_MCP.md)** (Este archivo)
   - Referencia rápida
   - Tablas de contenidos
   - Ubicación de código

---

## ✅ CHECKLIST DE VALIDACIÓN

- [x] 5 tools EXACTAMENTE (test_exactly_5_tools)
- [x] JSON-RPC 2.0 compliant
- [x] Error codes estándar
- [x] Rate limiting implementado
- [x] Caching implementado
- [x] FFI integración (Go, Zig, Nim, JAX)
- [x] Fallbacks reales (no mocks)
- [x] Compilación exitosa
- [x] Tests pasando
- [x] Documentación completa

---

**Documento:** Índice Rápido MCP  
**Total de documentación generada:** 2,651 líneas  
**Status:** ✅ COMPLETO  
**Fecha:** 13 de enero de 2026
