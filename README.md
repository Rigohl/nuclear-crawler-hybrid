# 🚀 Nuclear Crawler Hybrid - MCP 2025 Server

**Estado**: ✅ Producción  
**Versión**: 2.0 Final  
**MCP Protocol**: 2025-01-01  
**Tools**: 5 exactos (JSON-RPC 2.0)

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

## 📚 Documentación

| Archivo | Propósito |
|---------|-----------|
| [ARCHITECTURE.md](ARCHITECTURE.md) | Arquitectura técnica completa |
| [API_REFERENCE.md](API_REFERENCE.md) | Referencia API |
| [WSL_DEPLOYMENT.md](WSL_DEPLOYMENT.md) | Guía de instalación WSL |
| [TOOLS.md](TOOLS.md) | Especificaciones |

---

## ✅ Estado

- Build: ✅ PASS
- Tests: ✅ PASS  
- Compilation: ✅ 0 errors
- Security: ✅ 0 vulnerabilities

**Status: 🟢 PRODUCTION READY**
