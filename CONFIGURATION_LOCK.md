# 🔒 NUCLEAR CRAWLER HYBRID - CONFIGURATION LOCK

## Estado del Proyecto

✅ **TIPO DE PROYECTO**: MCP Axum Server v0.5.0
✅ **VALIDACIÓN COMPLETADA**: Confirmado como MCP Axum

---

## 📋 Características Confirmadas

### Servidor MCP (JSON-RPC stdin/stdout)
- **Binario**: `nuclear-mcp`
- **Archivo**: `src/main_mcp.rs`
- **Función**: Servidor Model Context Protocol para Claude, VSCode, etc.

### Servidor HTTP REST (Axum)
- **Binario**: `nuclear-http`
- **Archivo**: `src/main_http.rs`
- **Framework**: Axum v0.7
- **Puerto**: 4000 (configurable)
- **Funcionalidad**: Expone herramientas MCP como endpoints REST

### Mejoras Integradas (10+)
1. CircuitBreaker
2. BloomFilter
3. SemaphoreLimiter
4. MetricsCollector
5. MemoryCache
6. SmartRetry
7. EventBus
8. Rate Limiting
9. Monitoring
10. Auto-dependency management

---

## 🔐 ARCHIVOS PROTEGIDOS (BLOQUEADOS)

### Configuración crítica (NO EDITAR)
```
✓ NUCLEAR_CRAWLER_HYBRID/Cargo.toml
✓ NUCLEAR_CRAWLER_HYBRID/build.rs
✓ NUCLEAR_CRAWLER_HYBRID/Makefile
✓ NUCLEAR_CRAWLER_HYBRID/src/main_mcp.rs
✓ NUCLEAR_CRAWLER_HYBRID/src/main_http.rs
✓ NUCLEAR_CRAWLER_HYBRID/src/lib.rs
✓ CONFIGS/**/*
```

### Método de Protección
- **Settings**: `.vscode/settings.json` - Configuración VS Code
- **EditorConfig**: `.editorconfig` - Reglas de estilo y protección
- **Atributos**: Archivos marcados como "read-only" cuando sea posible

---

## ✅ ARCHIVOS EDITABLES (BÚSQUEDA)

Solo se pueden modificar archivos relacionados con búsqueda:

```
✓ **/search*.rs
✓ **/web_search*.rs
✓ **/websearch*.rs
✓ **/crawler*.rs         (solo *query/params)
✓ **/query*.rs
✓ **/*params*.rs
```

### Razón de la Restricción
Mantener la **integridad del MCP Axum** mientras se permite optimización de búsquedas.

---

## 🚀 Binarios Disponibles

```bash
# MCP Server (stdin/stdout JSON-RPC)
cargo build --release --bin nuclear-mcp

# HTTP Server (REST API)
cargo build --release --bin nuclear-http

# Hybrid Principal
cargo build --release --bin nuclear_crawler_hybrid

# Chapel Analyzer
cargo build --release --bin chapel-analyzer
```

---

## 🔄 Próximos Pasos

Para modificar búsquedas:
1. Edita solo archivos en `src/search*.rs`, `src/web_search*.rs`
2. Las configs permanecen BLOQUEADAS
3. Construye con: `cargo build --release`
4. Ejecuta con: `./target/release/nuclear-http` (HTTP) o `./target/release/nuclear-mcp` (MCP)

---

## ⚠️ Advertencia

⛔ **NO INTENTES DESBLOQUEAR ARCHIVOS PROTEGIDOS SIN AUTORIZACIÓN**
- Puede romper la funcionalidad MCP
- Puede causar conflictos con la integración Axum
- Puede afectar compatibilidad con Claude/VSCode

---

**Última actualización**: 1 de diciembre de 2025
**Estado**: 🔒 PROTEGIDO Y VALIDADO
