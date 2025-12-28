# 📁 Carpeta `src/` - Código Fuente Principal en Rust

## 🎯 Propósito
Código fuente principal del Nuclear Crawler Hybrid MCP Server en Rust.

## 🏗️ Arquitectura
- **Tokio async runtime** para concurrencia
- **Modular design** con crates separados
- **FFI integrations** con lenguajes nativos
- **MCP protocol** implementation

## 📂 Contenido
- `main.rs` - Punto de entrada del servidor
- `nuclear_core.rs` - Núcleo del sistema
- `web_search.rs` - Motor de búsqueda web
- `cache.rs` - Sistema de cache
- Módulos específicos por funcionalidad

## 🔧 Funciones
- `NuclearServer` - Servidor MCP principal
- `WebSearchEngine` - Búsqueda web híbrida
- `FFIManager` - Gestión de integraciones FFI
- `CacheSystem` - Cache inteligente
- `MetricsCollector` - Recolección de métricas

## 🚀 Uso
```rust
// Servidor principal
let server = NuclearServer::new(config).await?;
server.start().await?;
```

## 🤖 Contexto para IA
Código Rust que implementa:

- **Servidor HTTP MCP** con endpoints REST
- **Integración FFI** con Go, Zig, Nim
- **Búsqueda web híbrida** con múltiples estrategias
- **Sistema de cache** distribuido
- **Métricas y monitoring** en tiempo real

**Arquitectura**: Async-first con tokio, memory-safe, zero-cost abstractions.
