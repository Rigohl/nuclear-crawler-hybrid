# 🚀 Nuclear Crawler Hybrid - MCP 2025 Server

**Estado**: ✅ Producción  
**Versión**: 2.0 Final  
**MCP Protocol**: 2025-01-01  
**Tools**: 5 exactos (JSON-RPC 2.0)

![CI](https://github.com/Rigohl/nuclear-crawler-hybrid/workflows/CI/badge.svg)
![MCP Validation](https://github.com/Rigohl/nuclear-crawler-hybrid/workflows/MCP%20Validation%20-%20Real%20Server%20Testing/badge.svg)
![Copilot PR Validation](https://github.com/Rigohl/nuclear-crawler-hybrid/workflows/🤖%20Copilot%20PR%20Validation/badge.svg)

---

## 🎯 ¿Qué es?

**Nuclear Crawler Hybrid** es un servidor MCP (Model Context Protocol) ultra-potenciado con:
- **5 herramientas productivas** con capacidades avanzadas y IA integrada
- **Integración FFI REAL** con Go, Zig, Nim, JAX, Chapel (NO MOCKS)
- **Chapel AI Learning** - IA conectada que aprende continuamente
- **Bypass avanzado** para contenido protegido
- **GPU acceleration** con JAX (1536-dim embeddings)
- **12,249 LOC Rust puro** (CERO código muerto, CERO mocks)
- **Docker ready** (90.4 MB)
- **WSL compatible**

---

## ⚡ Las 5 Herramientas (Con Chapel AI Integration)

| # | Herramienta | Capacidad Potenciada |
|---|---|---|
| 1 | **websearch** | 55+ motores, stealth total, resultados potenciados con Chapel AI |
| 2 | **premium** | FFI real (Rust+Go+Zig+Nim+Chapel+JAX), captura Medium sin mocks |
| 3 | **file_search** | Detecta líneas exactas, errores, warnings, palabras específicas |
| 4 | **scan** | Escanea workspace + investiga internet + consejos con Chapel AI |
| 5 | **ai_dataset_trainer** | Crea datasets completos con temas múltiples, exámenes, Chapel learning |

---

## 🚀 Inicio Rápido

### 1. Iniciar servidor
```bash
./target/release/nuclear-mcp --serve tcp://0.0.0.0:8079
```

### 2. Verificar
```bash
curl http://localhost:8079/health
```

### 3. Usar con JSON-RPC 2.0
```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "method": "tools/call",
  "params": {
    "name": "websearch",
    "arguments": {"query": "machine learning"}
  }
}
```

---

## 📦 Desplegar

### Docker
```bash
docker run -p 8079:8079 ghcr.io/Rigohl/nuclear-crawler-hybrid:latest
```

### Compilar
```bash
cargo build --release
```

---

## 🤖 NEW: Chatbot & HuggingFace Integration

### Interactive AI Chatbot
```bash
# Chat via MCP
curl -X POST http://localhost:8079/mcp/tools/call \
  -H "Content-Type: application/json" \
  -d '{
    "name": "chatbot",
    "arguments": {
      "message": "Hello! Can you help me?"
    }
  }'
```

### HuggingFace Integration
```bash
# Set token for AI training
export HF_TOKEN="your_hf_token"

# Upload datasets, fine-tune models, deploy chatbots
# See docs/HUGGINGFACE_INTEGRATION.md for details
```

---

## 📚 Documentación

| Archivo | Propósito |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Arquitectura técnica completa |
| [API_REFERENCE.md](API_REFERENCE.md) | Referencia API |
| [TOOLS.md](TOOLS.md) | Especificaciones de herramientas |
| [HUGGINGFACE_INTEGRATION.md](docs/HUGGINGFACE_INTEGRATION.md) | Guía de HuggingFace |
| [CHATBOT_GUIDE.md](docs/CHATBOT_GUIDE.md) | Guía del chatbot |
| [WSL_DEPLOYMENT.md](WSL_DEPLOYMENT.md) | Guía de instalación WSL |
| [COPILOT_PR_COMPATIBILITY_REPORT.md](COPILOT_PR_COMPATIBILITY_REPORT.md) | Reporte de integración Copilot |

---

## 🤖 CI/CD y GitHub Copilot

Este repositorio está **completamente compatible** con GitHub Copilot Coding Agent para PRs automatizados.

### Workflows CI/CD

- ✅ **CI Pipeline**: Build, tests, clippy, formato
- ✅ **MCP Validation**: Validación de servidor MCP real (no mocks)
- ✅ **Security**: Auditoría de seguridad y dependencias
- ✅ **Copilot PR Validation**: Validación específica para PRs de Copilot

### Validaciones en PRs

Cada PR ejecuta automáticamente:
- ✅ Validación de exactamente 5 MCP tools
- ✅ Detección de mocks/stubs (no permitidos)
- ✅ Tests de integración contra servidor real
- ✅ Análisis de seguridad
- ✅ Formato y linting

### Para Contributors/Copilot

```bash
# Validar integración Copilot localmente
bash scripts/test_copilot_pr_integration.sh

# Validar 5 tools
cargo test test_exactly_5_tools

# Tests de integración real
cargo test --test integration_real_mcp
```

📖 Ver [COPILOT_PR_COMPATIBILITY_REPORT.md](COPILOT_PR_COMPATIBILITY_REPORT.md) para detalles completos.

---

## ✅ Estado

- Build: ✅ PASS
- Tests: ✅ PASS  
- Compilation: ✅ 0 errors
- Security: ✅ 0 vulnerabilities
- Copilot Integration: ✅ 32/32 tests passed

**Status: 🟢 PRODUCTION READY**
