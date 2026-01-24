# AGENTS

## Objetivo
Este archivo define reglas para agentes y herramientas automaticas en este repo.

## Reglas obligatorias
- No crear archivos .md sin pedido explicito del usuario.
- Actualiza docs existentes; no crees docs nuevas salvo pedido explicito.
- **PROHIBIDO crear documentación nueva** sin que el usuario lo solicite explícitamente por nombre o ruta.
- No mocks: usar datos reales y requests reales.
- No dead code: elimina codigo no usado.
- Mantener exactamente 5 tools en MCP protocol.
- Mostrar warnings y errores; no ocultar.
- No asumir; pedir aclaraciones si falta info.
- Respuestas concisas y directas.

## Flujo antes de cambios
- Leer el archivo completo antes de editar.
- Revisar Cargo.toml y features reales.
- Buscar todos los usos antes de cambiar codigo.
- No analizar /target/.
- Enfocar en .rs salvo pedido contrario.
- Verificar integraciones FFI (Go, Zig, Nim, JAX) si el cambio las toca.

## Estilo y codigo limpio
- Imports ordenados (externos -> internos -> relativos).
- Nombres descriptivos, funciones pequenas, DRY.
- Comentarios solo para logica no obvia.
- snake_case para funciones y variables.
- PascalCase para tipos, traits y enums.
- SCREAMING_SNAKE_CASE para constantes.
- Preferir &str sobre String en parametros.
- Evitar numeros magicos; usar constantes nombradas.

## Manejo de errores
- Manejar Result y Option de forma explicita.
- No unwrap/expect sin justificacion.
- Manejar errores en API, DB y operaciones async; sin bloques vacios.
- Propagar con ? cuando sea apropiado.
- Logs informativos (eprintln! en modo STDIO).

## MCP protocol
- Debe haber exactamente 5 tools en src/mcp/protocol.rs.
- Herramientas oficiales: websearch, premium, file_search,
  scan, ai_dataset_trainer.
- No agregar una 6ta o 7ma tool.

## Memoria MCP
- Al inicio de cada sesion: mcp_copilot-memor_retrieve_rules.
- Usar store/retrieve/list knowledge cuando corresponda.
- Usar MCP neuro bot al inicio para mejorar prompt e info.
- Actualizar MCP memoryr cada 4 movimientos y al final.

## Build y tests obligatorios
- cargo check --all-targets 2>&1 antes de cambios significativos.
- cargo build --release siempre.
- cargo test test_exactly_5_tools siempre.

## Reglas de workspace
- Mantener maximo 6 archivos en la raiz (5 .md + 1 .json).
- Un solo archivo por tema de documentacion.
