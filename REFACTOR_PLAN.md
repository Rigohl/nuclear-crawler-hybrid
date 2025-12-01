# 🔥 NUCLEAR CRAWLER HYBRID - MCP 2025 Refactor Plan

## Objetivo
Configurar el proyecto como un servidor MCP moderno siguiendo el protocolo MCP 2025, usando Axum, eliminando duplicados y asegurando la seguridad.

## Problemas Identificados

### 1. **Duplicación de Código MCP**
- ❌ `mcp_server.rs` - Implementación antigua/legacy (630 líneas)
- ✅ `simple_mcp.rs` - Implementación actual (1174 líneas) - **MANTENER**
- ✅ `main_mcp.rs` - Entry point (25 líneas) - **MANTENER**

**Acción**: Eliminar `mcp_server.rs` completamente

### 2. **Falta de Servidor HTTP con Axum**
Actualmente solo soporta stdio (stdin/stdout). Necesitamos:
- ✅ Servidor Axum HTTP/SSE para MCP 2025
- ✅ Mantener compatibilidad stdio para Claude Desktop
- ✅ WebSocket support para clientes modernos

### 3. **Seguridad**
- ✅ Validación de entrada
- ✅ Rate limiting
- ✅ CORS configurado correctamente
- ✅ TLS/HTTPS support
- ✅ Autenticación opcional (API keys)

## Plan de Implementación

### Fase 1: Limpieza de Duplicados ✅
1. Eliminar `src/mcp_server.rs`
2. Actualizar `src/lib.rs` para remover referencia a `mcp_server`
3. Verificar que `simple_mcp` sea el único módulo MCP

### Fase 2: Modernizar MCP Server con Axum 🔥
1. Crear `src/mcp_axum_server.rs` - Servidor HTTP/SSE con Axum
2. Implementar endpoints MCP 2025:
   - `POST /mcp/initialize` - Inicialización
   - `POST /mcp/tools/list` - Listar herramientas
   - `POST /mcp/tools/call` - Ejecutar herramienta
   - `GET /mcp/sse` - Server-Sent Events para notificaciones
   - `WS /mcp/ws` - WebSocket para comunicación bidireccional
3. Mantener `SimpleMcpServer` para modo stdio
4. Crear wrapper que soporte ambos modos

### Fase 3: Seguridad 🛡️
1. Implementar middleware de seguridad:
   - Rate limiting (tower-governor)
   - CORS (tower-http)
   - Request validation
   - API key authentication (opcional)
2. Sanitización de inputs
3. Logging seguro (sin exponer datos sensibles)

### Fase 4: Configuración y Deployment 📦
1. Actualizar `Cargo.toml` con dependencias necesarias
2. Crear archivo de configuración `mcp_config.toml`
3. Actualizar README con instrucciones
4. Crear scripts de deployment

## Estructura Final

```
src/
├── main_mcp.rs              # Entry point (stdio + HTTP)
├── simple_mcp.rs            # Core MCP logic (mantener)
├── mcp_axum_server.rs       # Nuevo: Servidor Axum HTTP/SSE/WS
├── mcp_security.rs          # Nuevo: Middleware de seguridad
└── lib.rs                   # Actualizar exports
```

## Herramientas MCP Finales (Sin Duplicados)

1. **websearch** - Búsqueda web masiva
2. **deep_web_search** - Búsqueda profunda
3. **ultimas_busquedas** - Historial
4. **stats** - Estadísticas
5. **analizar_proyecto** - Análisis de proyectos
6. **urls_visitadas** - URLs visitadas
7. **scan_project** - Escaneo de proyectos Rust

## Protocolo MCP 2025

### Cambios vs 2024-11-05:
- ✅ Soporte para SSE (Server-Sent Events)
- ✅ WebSocket bidireccional
- ✅ Streaming de respuestas
- ✅ Notificaciones push
- ✅ Mejor manejo de errores
- ✅ Autenticación mejorada

## Seguridad Checklist

- [ ] Validación de todos los inputs
- [ ] Rate limiting configurado
- [ ] CORS restrictivo
- [ ] TLS/HTTPS habilitado
- [ ] Logs sin datos sensibles
- [ ] Timeouts configurados
- [ ] Error handling robusto
- [ ] API key authentication (opcional)
- [ ] Request size limits
- [ ] SQL injection prevention (N/A - no SQL directo)

## Testing

- [ ] Tests unitarios para cada herramienta
- [ ] Tests de integración HTTP
- [ ] Tests de seguridad
- [ ] Tests de performance
- [ ] Tests de protocolo MCP 2025

## Próximos Pasos

1. ✅ Eliminar duplicados
2. 🔥 Implementar servidor Axum
3. 🛡️ Agregar seguridad
4. 📦 Actualizar configuración
5. 📝 Documentar
