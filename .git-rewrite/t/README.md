# 🔥 NUCLEAR CRAWLER HYBRID

> **El Motor de Búsqueda Libre Más Potente del Mundo**

**Versión**: 0.1.0 | **Protocolo**: MCP 2025-06-18 | **Puerto**: 8080

---

## 🚀 INICIO RÁPIDO

```bash
# Compilar
cargo build --release

# Ejecutar (HTTP)
./target/release/nuclear-mcp.exe --mode http --port 8080

# Ejecutar (STDIO)
./target/release/nuclear-mcp.exe --mode studio
```

---

## 📊 ESPECIFICACIONES

| Métrica | Valor |
|---------|-------|
| Tiempo de búsqueda | **2 segundos** |
| URLs por búsqueda | **2,100+** |
| Fuentes | **55** |
| Goroutines | **100,000** |
| Módulos activos | **23** |

---

## 🛠️ HERRAMIENTAS MCP

### 1. `websearch` - Búsqueda masiva
```json
{"name": "websearch", "arguments": {"queries": ["rust async", "tokio"]}}
```
- Hasta 5 queries simultáneos
- 55 fuentes (GitHub, StackOverflow, Reddit, etc.)
- Bypass de paywalls incluido

### 2. `file_search` - Búsqueda en archivos
```json
{"name": "file_search", "arguments": {"search_term": "async fn", "path": "./src"}}
```

### 3. `analyzer` - Análisis de código
```json
{"name": "analyzer", "arguments": {"path": "."}}
```
- Detecta errores con línea exacta
- Busca soluciones automáticamente
- Sugiere código para corregir

### 4. `stats` - Estadísticas del sistema
```json
{"name": "stats", "arguments": {}}
```

---

## 🌐 ENDPOINTS HTTP

| Endpoint | Método | Descripción |
|----------|--------|-------------|
| `/` | GET | Bienvenida |
| `/health` | GET | Health check |
| `/tools` | GET | Lista de herramientas |
| `/mcp` | POST | JSON-RPC genérico |
| `/mcp/initialize` | POST | Inicializar sesión |
| `/mcp/tools/list` | POST | Listar herramientas |
| `/mcp/tools/call` | POST | Ejecutar herramienta |
| `/call` | POST | Llamada directa |

### Ejemplo cURL
```bash
curl -X POST http://localhost:8080/call \
  -H "Content-Type: application/json" \
  -d '{"name": "websearch", "arguments": {"queries": ["rust async"]}}'
```

---

## 🔥 23 MÓDULOS ACTIVOS

| # | Módulo | Función |
|---|--------|---------|
| 1 | WebSearch | Búsqueda masiva 55 fuentes |
| 2 | RealSearchEngines | DuckDuckGo, Bing, Brave, SearX |
| 3 | DeepWebSearch | Papers, archives, databases |
| 4 | MassiveParallelSearch | 10K+ URLs paralelas |
| 5 | ParallelCrawler | Crawling recursivo |
| 6 | NuclearScraper | Extracción profunda |
| 7 | HuggingFace | Datasets y modelos |
| 8 | JAXPipeline | Vectorización masiva |
| 9 | JAXAccelerator | Procesamiento paralelo |
| 10 | MojoProcessor | Aceleración GPU/CPU |
| 11 | StealthSystem | Anti-detección |
| 12 | NuclearBypass | Bypass de paywalls |
| 13 | GoFFI | 100K goroutines |
| 14 | ZigSIMD | Hash ultra-rápido |
| 15 | NimParser | HTML parsing |
| 16 | BloomFilter | Deduplicación O(1) |
| 17 | CircuitBreaker | Tolerancia a fallos |
| 18 | IntelligentStorage | SQLite + FTS5 |
| 19 | HtmlParser | Parsing avanzado |
| 20 | AISmart | Ranking inteligente |
| 21 | MemoryCache | Caché en memoria |
| 22 | RateLimiter | Control de velocidad |
| 23 | Orchestrator | Coordinación final |

---

## 🎯 TECNOLOGÍAS

- **Rust** 🦀 - Core del sistema
- **Go** 🐹 - 100K goroutines concurrentes
- **JAX** 🧮 - Aceleración GPU/CPU
- **Zig** ⚡ - SIMD ultra-rápido
- **Nim** 👑 - Parsing HTML
- **Mojo** 🔥 - ML avanzado

---

## 📈 COMPARACIÓN

| Motor | Tiempo | URLs | Fuentes | Libre |
|-------|--------|------|---------|-------|
| Google | 0.5s | 10 | 1 | ❌ |
| DuckDuckGo | 1s | 20 | 1 | ✅ |
| SearX | 3s | 50 | 10 | ✅ |
| **NUCLEAR** | **2s** | **2100+** | **55** | **✅** |

---

## 📝 CONFIGURACIÓN

### Cargo.toml (Release)
```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"
```

### Performance
- `max_parallel`: 100,000
- `max_results`: 1,000
- `timeout`: 1s por módulo
- `total_time`: 2s

---

**Powered by Rust 🦀 + Go 🐹 + JAX 🧮 + Zig ⚡ + Nim 👑 + Mojo 🔥**
