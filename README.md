# 🔥 NUCLEAR CRAWLER HYBRID - MCP SERVER 2025

> **Servidor MCP HTTP-Only de Alto Rendimiento con Protocolo 2025-06-18**

**Versión**: 1.0.0 | **Protocolo**: MCP 2025-06-18 | **Transporte**: HTTP Only | **Puerto**: 8079

**Características**: 100K goroutines, 55 motores de búsqueda, 2s completion, 2100+ URLs por query

---

## 🚀 INICIO RÁPIDO

```bash
# Compilar (optimizado)
cargo build --release

# Ejecutar servidor MCP
./target/release/nuclear-mcp --port 8079
```

---

## 📊 ESPECIFICACIONES TÉCNICAS

| Componente | Especificación |
|------------|----------------|
| Protocolo | MCP 2025-06-18 (HTTP-only) |
| Transporte | HTTP + SSE (Axum) |
| Herramientas | 3 (websearch, file_search, get_vscode_api) |
| Puerto | 8079 |
| JARVIXSERVER | 5050 |
| TRAE CLI API | 8080 |
| Rendimiento | 100K goroutines paralelas |
| Motores | 55 fuentes de búsqueda |
| Tiempo | <2s por búsqueda completa |
| Resultados | 2100+ URLs por query |

---

## 🔧 CONFIGURACIÓN DE SERVICIOS

### Puertos y Servicios
- **Nuclear MCP Server**: `http://localhost:8079` - Servidor principal MCP
- **JARVIXSERVER**: `http://localhost:5050` - Backend de análisis e IA
- **TRAE CLI API**: `http://localhost:8080` - API REST de TRAE CLI

### Variables de Entorno para TRAE CLI
```bash
export JARVIX_URL=http://localhost:5050
```

También puedes copiar `.env.example` a `.env` y ajustar las configuraciones.

---

## 🤖 AGENTES AVANZADOS Y WORKFLOWS AUTOMATIZADOS

### **Agentes Especializados Disponibles**

#### 1. **CodeAnalysisAgent** 🤖
- **Propósito**: Análisis inteligente de código Rust
- **Capacidades**: Detección de bugs, optimización automática, corrección de Clippy warnings
- **Herramientas**: `scan_project`, `websearch`, `deep_web_search`
- **Activación**: Automática en pushes a main/develop

#### 2. **ResearchAgent** 🔬
- **Propósito**: Investigación y desarrollo automatizado
- **Capacidades**: Búsqueda en múltiples fuentes, análisis de tendencias, generación de propuestas
- **Fuentes**: GitHub, arXiv, Stack Overflow, Reddit, Dev.to
- **Activación**: Semanal (lunes) o manual

#### 3. **AutomationAgent** ⚙️
- **Propósito**: Automatización de workflows complejos
- **Capacidades**: CI/CD, revisiones de código, actualizaciones de dependencias
- **Integraciones**: GitHub Actions, Docker, Kubernetes, Slack
- **Activación**: En PRs y releases

#### 4. **DevOpsAgent** 🏗️
- **Propósito**: Operaciones de desarrollo e infraestructura
- **Capacidades**: Orquestación de contenedores, IaC, monitoreo, optimización
- **Plataformas**: Docker, Kubernetes, AWS, Azure, GCP
- **Activación**: En deployments

### **Workflows Automatizados**

#### **Pipeline Principal: Multi-Agent CI/CD**
```yaml
# .github/workflows/nuclear-advanced-pipeline.yml
name: 🚀 Nuclear Multi-Agent CI/CD Pipeline

on: [push, pull_request, issues, schedule, workflow_dispatch]

jobs:
  multi_agent_analysis      # 🤖 Análisis paralelo con 4 agentes
  ai_research_development   # 🔬 Investigación automatizada
  auto_fix_optimization     # 🔧 Corrección automática
  monitoring_reporting      # 📊 Reportes avanzados
  deployment               # 🚀 Deploy automático
  security_scanning        # 🔒 Escaneo de seguridad
  performance_monitoring   # 📈 Monitoreo continuo
```

#### **Características del Pipeline**
- **Análisis Paralelo**: 4 agentes trabajando simultáneamente
- **Auto-Fix**: Corrección automática de issues críticos
- **Reportes Avanzados**: Generados con IA y métricas detalladas
- **Monitoreo Continuo**: Métricas en tiempo real
- **Deploy Seguro**: Solo con aprobación de todos los agentes

### **Scripts de Automatización**

#### **Generador de Reportes Avanzados**
```bash
python scripts/generate_advanced_report.py \
  --input analysis_results.json \
  --template nuclear_template.md \
  --output NUCLEAR_ANALYSIS_REPORT.md
```

#### **Sistema de Auto-Fix**
```bash
python scripts/auto_fix.py \
  --analysis-results analysis_results.json \
  --severity critical \
  --auto-commit true
```

#### **Benchmark de Rendimiento**
```bash
python scripts/benchmark.py \
  --output performance_results.json \
  --save-baseline
```

### **Integración con Herramientas Externas**

#### **Hugging Face Jobs**
- **CodeAnalysisAgent**: Usa modelos de código como `microsoft/codebert-base`
- **GPU Acceleration**: Procesamiento paralelo en infraestructura HF
- **Modelos Especializados**: Fine-tuned para análisis Rust

#### **N8N Workflows**
- **Workflow Automation**: Conexión directa con MCP server
- **Multi-Agent Orchestration**: Coordinación entre agentes
- **External Integrations**: Slack, Discord, email, APIs

#### **GitHub Copilot CLI**
- **Agent Integration**: Copilot CLI como agente adicional
- **Prompt Engineering**: Instrucciones contextuales por proyecto
- **Workflow Triggers**: Activación basada en eventos

### **Métricas y Monitoreo**

#### **Dashboard de Rendimiento**
- **Response Times**: <2s por operación
- **Success Rate**: >95% en todas las operaciones
- **Resource Usage**: Monitoreo de CPU/memoria
- **Agent Performance**: Métricas por agente especializado

#### **Alertas Automáticas**
- **Slack Integration**: Notificaciones en tiempo real
- **Performance Thresholds**: Alertas cuando se exceden límites
- **Security Alerts**: Detección automática de vulnerabilidades
- **Deployment Status**: Notificaciones de deploy

---

## 🛠️ HERRAMIENTAS MCP

### 1. `websearch` - Búsqueda Web Masiva con FFI
```json
{"name": "websearch", "arguments": {"queries": ["rust async", "tokio patterns"]}}
```
- **Hasta 5 queries simultáneos**
- **55 motores de búsqueda integrados**
- **Scraping premium**: Medium.com, ArXiv, papers académicos
- **FFI Acelerado**: Go (100K goroutines), Nim (HTML parsing), JAX (GPU)
- **Resultados reales** guardados en `resultados/`
- **Tiempo**: <2 segundos para 2100+ URLs

### 2. `file_search` - Búsqueda de Archivos con SIMD
```json
{"name": "file_search", "arguments": {"search_term": "async fn", "path": "./src"}}
```
- **SIMD Zig FFI** para búsqueda ultra-rápida
- **Análisis avanzado**: detección de errores, warnings, complejidad
- **Resultados precisos** con números de línea exactos
- **Funciones**: búsqueda semántica, duplicados, imports circulares

### 3. `get_vscode_api` - Documentación VS Code API
```json
{"name": "get_vscode_api", "arguments": {"query": "commands"}}
```
- **Documentación completa** de APIs de VS Code
- **Ejemplos de código** TypeScript funcionales
- **Consultas específicas**: workspace, commands, window, languages
- **Interfaces y métodos** detallados

---

## 🌐 ENDPOINTS MCP

| Endpoint | Método | Descripción |
|----------|--------|-------------|
| `/` | GET | Health check + SSE connections |
| `/` | POST | JSON-RPC MCP messages |
| `/call` | POST | JSON-RPC tool calls |

### Ejemplo de Uso
```bash
# Listar herramientas
curl -X POST http://localhost:8079/call \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'

# Ejecutar búsqueda web
curl -X POST http://localhost:8079/call \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "websearch", "arguments": {"queries": ["rust async"]}}, "id": 2}'
```

---

## 🎯 ARQUITECTURA Y MÓDULOS

### Core Modules (11 integrados)
- **Web Search**: Motor de búsqueda masiva con FFI
- **File Search**: Análisis de archivos con SIMD
- **Nuclear Core**: Núcleo de procesamiento
- **Premium Scraper**: Scraping de contenido premium
- **Rate Limiter**: Control de tasa de requests
- **Intelligent Storage**: Almacenamiento inteligente
- **Cache**: Sistema de cache avanzado

### FFI Integrations
- **Go FFI**: 100K goroutines paralelas para stealth requests
- **Zig SIMD**: Procesamiento de hashing y parsing ultra-rápido
- **Nim HTML**: Parsing HTML alternativo de alto rendimiento
- **JAX GPU**: Aceleración vectorizada para procesamiento batch

### Tecnologías
- **Rust** 🦀 - Core system
- **Axum** 🌐 - HTTP MCP server
- **Tokio** ⚡ - Async runtime completo
- **Serde** 📦 - JSON serialization
- **libloading** 🔗 - FFI dynamic loading
- **DashMap** 🗺️ - Concurrent hash maps

---

<<<<<<< HEAD
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
=======
## 📁 ESTRUCTURA DEL PROYECTO
>>>>>>> origin/main

```
NUCLEAR_CRAWLER_HYBRID/
├── src/                    # Código fuente Rust - Lógica core del servidor MCP
├── go/                     # Implementación FFI en Go - 100K goroutines paralelas
├── zig/                    # Librería SIMD en Zig - Procesamiento ultra-rápido
├── nim/                    # Parser HTML en Nim - Alto rendimiento
├── libs/                   # Librerías compiladas FFI dinámicas
├── scripts/                # Scripts de build, compilación y automatización
├── memories/               # Sistema de memoria persistente y cache
├── REPORTS/                # Análisis y reportes de hallazgos del crawler
├── resultados/             # Resultados de búsquedas y análisis guardados
├── target/                 # Artefactos de compilación Cargo
├── recomendaciones.md      # Recomendaciones de GitHub para mejoras
├── MAXIMO_PODER_ACTIVADO.md      # Guía de máxima potencia
├── RESUMEN_CONFIGURACION.md      # Configuración resumida
├── MCP_TOOLKIT_2025_EXAMPLES.md  # Ejemplos MCP 2025
├── MCP_SERVER_GUIDE.md           # Guía del servidor
└── README.md              # Este archivo
```

Cada carpeta raíz contiene un `README.md` con documentación específica sobre su contenido y propósito en el proyecto híbrido.

---

## ⚙️ CONFIGURACIÓN

### Cargo.toml (Extracto)
```toml
[package]
name = "nuclear-crawler-hybrid"
version = "1.0.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
clap = { version = "4.0", features = ["derive"] }
libloading = "0.8"
dashmap = "5.5"
chrono = "0.4"
```

### Build Optimizado
```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
```

---

## 🚀 DEPLOYMENT

```bash
# Build optimizado
cargo build --release

# Ejecutar servidor
./target/release/nuclear-mcp --port 8079

# Verificar funcionamiento
curl -X POST http://localhost:8079/call \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

---

## 📚 DOCUMENTACIÓN

- **[MAXIMO_PODER_ACTIVADO.md](MAXIMO_PODER_ACTIVADO.md)** - Guía completa de máxima potencia
- **[RESUMEN_CONFIGURACION.md](RESUMEN_CONFIGURACION.md)** - Configuración resumida
- **[MCP_TOOLKIT_2025_EXAMPLES.md](MCP_TOOLKIT_2025_EXAMPLES.md)** - Ejemplos MCP 2025
- **[MCP_SERVER_GUIDE.md](MCP_SERVER_GUIDE.md)** - Guía detallada del servidor
- **[.github/AGENT.MD](.github/AGENT.MD)** - Documentación completa de automatización GitHub con TRAE CLI

## 📋 RECOMENDACIONES

- **[recomendaciones.md](recomendaciones.md)** - Recomendaciones de repositorios similares, librerías y código de ejemplo obtenidos de búsquedas en GitHub. Incluye crawlers en Rust como spider-rs/spider, librerías como reqwest y scraper, y ejemplos de integración.

---

## 🤖 CONTEXTO PARA IA

### Arquitectura del Sistema
**Nuclear Crawler Hybrid** es un servidor MCP HTTP avanzado que combina múltiples lenguajes y tecnologías para lograr rendimiento extremo:

- **Rust Core**: Sistema base memory-safe con async Tokio
- **Multi-FFI**: Integraciones con Go (goroutines), Zig (SIMD), Nim (parsing)
- **MCP Protocol**: Implementación HTTP-only del protocolo 2025-06-18
- **Hybrid Search**: 55 motores de búsqueda con scraping inteligente

### Patrones de Diseño
- **Async-First**: Todo el sistema usa Tokio para concurrencia
- **FFI Orchestration**: Rust coordina, lenguajes nativos ejecutan kernels intensivos
- **Memory Management**: Zig para control manual, Rust para safety
- **Modular Architecture**: Componentes desacoplados con interfaces claras

### Consideraciones para Desarrollo
- **Performance Critical**: Optimizaciones LTO, SIMD, parallel processing
- **Multi-Language**: Coordenación compleja entre Rust + FFI languages
- **Real-time Processing**: <2s completion para búsquedas masivas
- **Resource Intensive**: 100K goroutines, GPU acceleration opcional

### Estructura de Carpetas (AI Context)
Cada carpeta raíz tiene documentación específica en `README.md`:
- `src/` - Código Rust principal con módulos MCP
- `go/` - FFI Go para requests paralelos masivos
- `zig/` - SIMD Zig para procesamiento de datos
- `nim/` - HTML parsing Nim de alto rendimiento
- `libs/` - Librerías compiladas FFI dinámicas
- `scripts/` - Automatización de build y mantenimiento
- `resultados/` - Outputs de búsqueda y análisis
- `memories/` - Sistema de memoria persistente
- `REPORTS/` - Análisis y reportes de diagnóstico
- `target/` - Artefactos de compilación Cargo
- `.github/` - Automatización GitHub con TRAE CLI por defecto

### GitHub Automation con TRAE CLI por Defecto
El proyecto utiliza TRAE CLI por defecto para todos los procesos de CI/CD y desarrollo:
- **CI/CD Automático**: `trae fmt`, `trae clippy --strict`, `trae test --release`
- **Builds**: `trae build --release`
- **Seguridad**: `trae clippy --strict --pedantic` + `cargo audit`
- **Desarrollo Local**: `trae repair`, `trae clippy --strict`, `trae security --audit`
- **Code Quality**: Zero warnings policy con clippy estricto
- **Templates**: Issues y PRs estandarizados para desarrollo híbrido

**🔥 Powered by Nuclear Technology - Maximum Power Activated 🦀**
