# 🔥 NUCLEAR CRAWLER HYBRID - MCP Server 2025

**Sistema de Búsqueda Web Masiva y Scraping Empresarial** con 18+ módulos integrados.

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![MCP Protocol](https://img.shields.io/badge/MCP-2025--06--18-blue.svg)](https://modelcontextprotocol.io/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

---

## 🚀 Inicio Rápido

### Compilación
```bash
cargo build --release --bin nuclear-mcp
```

### Ejecución
```bash
# Modo MCP Studio (stdio) - Para VS Code, Cursor, Claude Desktop
./target/release/nuclear-mcp.exe

# Modo HTTP (API REST)
./target/release/nuclear-mcp.exe --mode http --port 3000
```

---

## 🛠️ 5 Herramientas MCP

### 1. 🔥 `websearch` - Búsqueda Web MASIVA
Busca en **15+ motores de búsqueda** y **20+ sitios especializados** simultáneamente.

```json
{
  "name": "websearch",
  "arguments": {
    "query": "Rust async await tutorial",
    "max_results": 100,
    "deep_web": false
  }
}
```

**Motores de búsqueda integrados:**
- DuckDuckGo, Bing, Brave Search, Yandex
- Ecosia, Qwant, Startpage, SearX, Mojeek

**Sitios especializados:**
- GitHub, GitLab, StackOverflow, Reddit
- HuggingFace, arXiv, crates.io, npm, PyPI
- dev.to, Medium, Wikipedia, y más...

---

### 2. 🔍 `analyzer` - Analizador de Proyectos
Escanea proyectos en **10+ lenguajes** con análisis de seguridad.

```json
{
  "name": "analyzer",
  "arguments": {
    "path": "./mi-proyecto",
    "analyze_type": "full"
  }
}
```

**Lenguajes soportados:**
- Rust, Python, JavaScript, TypeScript
- Go, Java, C/C++, Zig, Chapel

**Tipos de análisis:**
- `full` - Análisis completo
- `errors` - Solo errores
- `security` - Vulnerabilidades (passwords, API keys, tokens)

---

### 3. 📂 `file_research` - Búsqueda en Archivos
Encuentra texto exacto con **número de línea preciso**.

```json
{
  "name": "file_research",
  "arguments": {
    "search_term": "TODO|FIXME|panic!",
    "path": ".",
    "use_regex": true
  }
}
```

---

### 4. 📊 `stats` - Estadísticas del Sistema
Métricas completas del MCP.

```json
{
  "name": "stats",
  "arguments": {
    "stat_type": "full"
  }
}
```

**Tipos:**
- `full` - Todo
- `performance` - Rendimiento
- `storage` - Almacenamiento
- `recent` - Actividad reciente

---

### 5. 🎭 `orchestrate` - Orquestador de Tareas Masivas
Ejecuta **múltiples tareas en paralelo**.

```json
{
  "name": "orchestrate",
  "arguments": {
    "tasks": [
      {"type": "search", "query": "Rust MCP"},
      {"type": "search", "query": "Python FastAPI"},
      {"type": "file_search", "term": "TODO"},
      {"type": "analyzer", "path": "."},
      {"type": "stats"}
    ],
    "parallel": true,
    "max_concurrent": 50
  }
}
```

**Tipos de tareas:**
- `search` - Búsqueda web (requiere `query`)
- `file_search` - Búsqueda en archivos (requiere `term`)
- `analyzer` - Análisis de proyecto (requiere `path`)
- `stats` - Estadísticas

---

## ⚡ Módulos Integrados

| Módulo | Función | Implementación |
|--------|---------|----------------|
| **WebSearch v5.0** | Orquestador principal de búsqueda | ✅ Rust nativo |
| **MassiveParallelSearch** | Búsqueda masiva con query real en motores | ✅ Rust + reqwest |
| **NuclearScraper** | Crawling masivo HTTP | ✅ Rust + reqwest |
| **GoIntegration** | Procesamiento paralelo + headers | ✅ FFI Go (goroutines reales) + fallback Rust |
| **ZigIntegration** | Parsing HTML optimizado | ✅ Rust + scraper (Zig FFI disponible) |
| **NimIntegration** | Extracción de texto | ✅ Rust + fallback (Nim FFI disponible) |
| **JaxAccelerator** | Vectorización paralela | ✅ Rust + rayon |
| **MojoJaxProcessor** | Procesamiento batch | ✅ Rust + rayon |
| **NuclearBypass** | Bypass de protecciones anti-bot | ✅ Rust nativo |
| **StealthSystem** | Anti-detección con headers rotativos | ✅ Rust nativo |
| **AISmart** | Estrategias inteligentes de búsqueda | ✅ Rust nativo |
| **DeepWebSearch** | Búsqueda en deep web | ✅ Rust nativo |
| **IntelligentStorage** | SQLite + Full-Text Search | ✅ Rust + rusqlite |
| **ParallelCrawler** | Crawl paralelo con workers | ✅ Rust + tokio |
| **FileSearch** | Búsqueda en archivos locales | ✅ Rust + walkdir |
| **ProjectScanner** | Análisis de proyectos multi-lenguaje | ✅ Rust nativo |
| **Improvements** | BloomFilter, CircuitBreaker, Cache | ✅ Rust nativo |

**Compilación con FFI:**
- `cargo build --release` - Compilación básica (Rust puro con fallbacks)
- `cargo build --release --features go` - Activa FFI de Go (requiere stealth_go.a)
- `cargo build --release --features zig` - Activa FFI de Zig (requiere nuclear_zig.lib)

---

## 🔧 Configuración VS Code / Cursor

### Global MCP Settings

**Windows:** `%APPDATA%\Code\User\settings.json`

```json
{
  "mcp": {
    "servers": {
      "nuclear-crawler": {
        "command": "C:\\Users\\TU_USUARIO\\Desktop\\hf_spaces\\NUCLEAR_CRAWLER_HYBRID\\target\\release\\nuclear-mcp.exe",
        "args": []
      }
    }
  }
}
```

### MCP Config File

**Windows:** `%APPDATA%\Code\User\globalStorage\rooveterinaryinc.roo-cline\settings\mcp_settings.json`

O crear `mcp.json` en el workspace:

```json
{
  "mcpServers": {
    "nuclear-crawler": {
      "command": "C:\\Users\\TU_USUARIO\\Desktop\\hf_spaces\\NUCLEAR_CRAWLER_HYBRID\\target\\release\\nuclear-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

---

## 📈 Rendimiento

| Métrica | Valor |
|---------|-------|
| **URLs simultáneas** | 200+ |
| **Requests/segundo** | 100,000 |
| **Timeout** | 15 segundos |
| **Cache** | 10,000 entradas |
| **Workers paralelos** | 2x CPU cores |
| **Motores de búsqueda** | 15+ |
| **Sitios especializados** | 20+ |

---

## 🔒 Características de Seguridad

- ✅ **Stealth Mode** - Headers anti-detección rotativos
- ✅ **Rate Limiting** - Control de velocidad por dominio
- ✅ **Circuit Breaker** - Tolerancia a fallos
- ✅ **Bloom Filter** - Deduplicación O(1)
- ✅ **Memory Cache** - Cache en memoria con TTL

---

## 📦 Estructura del Proyecto

```
NUCLEAR_CRAWLER_HYBRID/
├── src/
│   ├── bin/
│   │   └── nuclear_ultimate.rs    # 🔥 Binario MCP principal
│   ├── web_search.rs              # Búsqueda web v5.0
│   ├── massive_parallel_search.rs # Búsqueda masiva paralela
│   ├── nuclear_scraper.rs         # Scraper nuclear
│   ├── file_search.rs             # Búsqueda en archivos
│   ├── scan_project.rs            # Scanner de proyectos
│   ├── deep_web_search.rs         # Deep web
│   ├── stealth.rs                 # Sistema stealth
│   ├── ai_smart.rs                # IA inteligente
│   └── ...                        # 18+ módulos más
├── target/release/
│   └── nuclear-mcp.exe            # Binario compilado
├── resultados/                    # Resultados guardados
├── Cargo.toml
└── README.md
```

---

## 🧪 Pruebas

```bash
# Test básico MCP
$init = '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}'
$list = '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}'
($init, $list) -join "`n" | .\target\release\nuclear-mcp.exe

# Test búsqueda web
$search = '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"websearch","arguments":{"query":"Rust async await"}}}'
($init, $search) -join "`n" | .\target\release\nuclear-mcp.exe
```

---

## 📄 Licencia

MIT License - Ver [LICENSE](LICENSE)

---

## 🔥 Desarrollado por

**Nuclear Crawler Team** | Protocolo MCP 2025-06-18

```
██╗  ██╗██╗   ██╗ ██████╗██╗     ███████╗ █████╗ ██████╗ 
███╗ ██║██║   ██║██╔════╝██║     ██╔════╝██╔══██╗██╔══██╗
██╔██╗██║██║   ██║██║     ██║     █████╗  ███████║██████╔╝
██║╚████║██║   ██║██║     ██║     ██╔══╝  ██╔══██║██╔══██╗
██║ ╚███║╚██████╔╝╚██████╗███████╗███████╗██║  ██║██║  ██║
╚═╝  ╚══╝ ╚═════╝  ╚═════╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝
                CRAWLER HYBRID v0.5.0
```
