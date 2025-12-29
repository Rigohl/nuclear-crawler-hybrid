# 🔥 CODE QUALITY STANDARDS - Nuclear MCP AIToolkit

Este documento define los estándares de calidad para el proyecto **nuclear-crawler-hybrid**, especialmente enfocado en la validación de herramientas MCP sin mocks.

## 📋 Tabla de Contenidos

1. [Testing Philosophy](#testing-philosophy)
2. [MCP Tool Requirements](#mcp-tool-requirements)
3. [Real Server Testing](#real-server-testing)
4. [No Mocks Policy](#no-mocks-policy)
5. [Integration Test Guidelines](#integration-test-guidelines)
6. [Performance Requirements](#performance-requirements)

---

## 🎯 Testing Philosophy

### NO MOCKS, NO STUBS, NO FIXTURES

El proyecto **nuclear-crawler-hybrid** utiliza una filosofía de testing que **RECHAZA completamente los mocks, stubs y fixtures** para las herramientas MCP:

```
❌ PROHIBIDO:
  - mock_data
  - stub_responses
  - fixture_examples
  - test_only_implementations
  - fake_data

✅ REQUERIDO:
  - Real HTTP requests
  - Real data from servers
  - Integration tests contra servidor VIVO
  - Validación contra datos reales
  - Tiempos reales de ejecución
```

### Razón

Las herramientas MCP interact con sistemas reales:
- **websearch**: Busca en 55+ motores de búsqueda reales
- **deepweb_search**: Accede a redes TOR/I2P reales
- **premium_content_scraper**: Extrae contenido real de Medium, ArXiv, etc.
- **file_search**: Busca palabras reales en archivos del proyecto

Por lo tanto, los tests **DEBEN usar datos reales** para garantizar que las herramientas funcionan correctamente en producción.

---

## 📡 MCP Tool Requirements

### 1. Tool Specifications

Cada herramienta MCP debe:

| Herramienta | Max Queries | Timeout | Fuentes |
|---|---|---|---|
| **websearch** | 50 | 5s | 55+ motores, TOR, DeepWeb |
| **deepweb_search** | 20 | 10s | .onion sites, 1000+ fuentes oscuras |
| **premium_content_scraper** | 20 | 15s | Medium, ArXiv, O'Reilly, Manning |
| **file_search** | 10 | 8s | Archivos locales del proyecto |

### 2. Input Schema Validation

Cada herramienta DEBE validar:

```json
{
  "type": "object",
  "properties": {
    "queries": {
      "type": "array",
      "items": {"type": "string"},
      "minItems": 1,
      "maxItems": 50  // o la que corresponda
    }
  },
  "required": ["queries"],
  "additionalProperties": false
}
```

**IMPORTANTE**: Solo se aceptan arrays de strings. NO se aceptan objetos complejos como parámetros.

### 3. Response Format

Todas las respuestas DEBEN ser JSON-RPC 2.0 válidas:

```json
{
  "jsonrpc": "2.0",
  "id": <request_id>,
  "result": {
    "status": "success",
    "tool": "<tool_name>",
    "data": <real_data>,
    "execution_ms": <milliseconds>,
    "count": <number_of_results>
  }
}
```

**O en caso de error:**

```json
{
  "jsonrpc": "2.0",
  "id": <request_id>,
  "error": {
    "code": -32603,
    "message": "<error_description>"
  }
}
```

---

## 🧪 Real Server Testing

### Integration Test Structure

Todo test de integración DEBE:

1. **Compilar el servidor MCP**
   ```bash
   cargo build --bin nuclear_ultimate --release
   ```

2. **Iniciar el servidor en background**
   - Usar Tokio runtime
   - Escuchar en puerto 8079
   - Responder a health checks en GET /

3. **Esperar a que esté listo**
   - Retry logic: máximo 10 intentos
   - Timeout total: 30 segundos

4. **Hacer requests HTTP REALES**
   - POST a http://localhost:8079/call
   - Payloads JSON-RPC 2.0 reales
   - NO requests hardcodeadas

5. **Validar respuestas REALES**
   - JSON-RPC 2.0 compliance
   - Datos reales (no mocks)
   - Tiempos dentro de límites

### Test Execution

```bash
# Ejecutar tests de integración REAL
cargo test --test integration_real_mcp --release -- --nocapture

# Ejecutar con log completo
RUST_LOG=debug cargo test --test integration_real_mcp --release -- --nocapture --test-threads=1
```

---

## 🚫 No Mocks Policy

### Explicit Prohibition

```rust
// ❌ PROHIBIDO - NUNCA hacer esto:
let mock_response = json!({
  "status": "mock_data",
  "results": ["fake_url_1", "fake_url_2"]
});

// ❌ PROHIBIDO - Variables temporales:
let test_only_results = vec!["example1", "example2"];

// ❌ PROHIBIDO - Stubs:
fn stub_websearch() -> Value { json!({"status": "ok"}) }
```

### Acceptable Alternative: Real Data

```rust
// ✅ CORRECTO - Usar datos REALES:
let response = self.web_search.search_real(queries).await?;

// ✅ CORRECTO - Hacer requests HTTP reales:
let client = Client::new();
let response = client.post(url).json(&payload).send().await?;

// ✅ CORRECTO - Validar datos reales:
validate_real_data(&response)?; // Busca indicadores de mock
```

### Detection Mechanism

El test automaticamente busca indicadores de mocks:

```rust
let mock_indicators = [
  "mock_data",
  "stub_",
  "fixture_",
  "example_",
  "test_only",
  "fake_",
  "dummy_",
];

for indicator in &mock_indicators {
  if response_string.contains(indicator) {
    return Err("Contains mock data - NOT ALLOWED".to_string());
  }
}
```

---

## 📝 Integration Test Guidelines

### Crear un nuevo Integration Test

1. **Ubicación**: `tests/integration_*.rs`

2. **Estructura**:
```rust
use reqwest::Client;
use serde_json::{json, Value};

#[tokio::test]
async fn test_my_tool_real() {
    // 1. Compilar server
    compile_mcp().expect("Build failed");

    // 2. Iniciar server
    let mut server = start_mcp_server().expect("Start failed");

    // 3. Esperar que esté listo
    wait_for_server_ready(10).await.expect("Timeout");

    // 4. Hacer request REAL
    let params = json!({
        "name": "my_tool",
        "arguments": { "queries": ["real_query"] }
    });

    let response = send_jsonrpc_request("tools/call", params, 1).await.unwrap();

    // 5. Validar respuesta REAL
    validate_jsonrpc_response(&response).unwrap();
    validate_real_data(&response).unwrap();

    // 6. Cleanup
    server.kill().unwrap();
}
```

3. **Validaciones obligatorias**:
   - ✅ JSON-RPC 2.0 compliance
   - ✅ Real data (no mocks)
   - ✅ Timeout validation
   - ✅ HTTP status codes
   - ✅ Response structure

4. **Helpers disponibles**:
   - `compile_mcp()` - Compilar servidor
   - `start_mcp_server()` - Iniciar en background
   - `wait_for_server_ready()` - Esperar a que esté listo
   - `send_jsonrpc_request()` - Hacer request JSON-RPC
   - `validate_jsonrpc_response()` - Validar protocolo
   - `validate_real_data()` - Validar datos reales
   - `validate_timeout()` - Validar tiempos

---

## ⚡ Performance Requirements

### Timeout Limits (ESTRICTOS)

```
websearch:               5 segundos MÁXIMO
deepweb_search:        10 segundos MÁXIMO
premium_content_scraper: 15 segundos MÁXIMO
file_search:            8 segundos MÁXIMO
```

### Measurement

Cada respuesta DEBE incluir `execution_ms`:

```json
{
  "status": "success",
  "execution_ms": 2345
}
```

El test valida: `execution_ms <= timeout_seconds * 1000`

### Rate Limiting

El servidor implementa rate limiting:

```rust
pub rate_limit: Arc<RateLimiter>,  // 10 req/s, burst 20
```

Los tests validar que funciona correctamente intentando múltiples requests rápido.

---

## ✅ Checklist para Pull Requests

Antes de mergear una PR que toque MCP tools:

- [ ] ✅ Todos los tests pasan: `cargo test --test integration_real_mcp`
- [ ] ✅ No hay warnings: `cargo clippy -- -D warnings`
- [ ] ✅ Build release exitoso: `cargo build --release`
- [ ] ✅ NO hay mocks/stubs en el código
- [ ] ✅ Todas las herramientas probadas contra servidor REAL
- [ ] ✅ Timeouts respetados
- [ ] ✅ Response JSON-RPC 2.0 válida
- [ ] ✅ PR template completado
- [ ] ✅ CODEOWNERS revisó cambios

---

## 🔍 Debugging & Troubleshooting

### Problema: Test falla en CI pero pasa localmente

**Causas comunes**:
1. Red instable en CI
2. Servidor tarda más en iniciar
3. Rate limiting diferente

**Soluciones**:
```bash
# Aumentar timeouts para debugging
STARTUP_TIMEOUT=60 cargo test --test integration_real_mcp

# Ejecutar con logs completos
RUST_LOG=debug cargo test --test integration_real_mcp -- --nocapture --test-threads=1
```

### Problema: "Contains mock data - NOT ALLOWED"

**Causa**: La respuesta contiene indicadores de mock

**Solución**:
```rust
// Cambiar esto:
json!({"status": "mock_data", "example": "value"})

// A esto:
json!({"status": "success", "data": real_data})
```

### Problema: Timeout en test

**Causa**: El servidor tarda mucho en responder

**Solución**:
1. Verificar que el servidor compila en release: `-O`
2. Verificar configuración de rate limiting
3. Aumentar timeout en CI si es necesario

---

## 📚 References

- [MCP Specification](https://modelcontextprotocol.io/)
- [JSON-RPC 2.0 Spec](https://www.jsonrpc.org/specification)
- [Project README](../README.md)
- [MCP Setup Guide](../MCP_SETUP_GUIDE.md)
- [Integration Tests](../tests/integration_real_mcp.rs)

---

## 🤝 Contributing

Para contribuir:

1. Leer este documento completamente
2. Asegurar que los tests pasen localmente
3. NO introducir mocks
4. Seguir la estructura JSON-RPC 2.0
5. Respetar timeouts configurados
6. Pasar el CODE REVIEW

---

**Last Updated**: 2025-01-29
**Version**: 1.0.0
**Status**: ✅ PRODUCTION READY
