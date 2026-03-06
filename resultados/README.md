# Resultados - Nuclear Crawler Hybrid

Esta carpeta contiene los resultados generados por el MCP Server.

## Estructura

```
resultados/
├── searches/        # Resultados de busquedas web (websearch tool)
│   └── YYYY-MM-DD_query.json
├── osint/           # Reportes de inteligencia OSINT
│   └── YYYY-MM-DD_target_report.json
├── scans/           # Resultados de escaneo de workspace
│   └── YYYY-MM-DD_scan.json
├── ai_training/     # Datasets y metricas de entrenamiento AI
│   └── YYYY-MM-DD_training_metrics.json
└── exports/         # Exportaciones finales y reportes combinados
    └── YYYY-MM-DD_export.json
```

## Formato de Archivos

Todos los resultados se guardan en JSON con la siguiente estructura base:

```json
{
  "timestamp": "2026-03-06T12:00:00Z",
  "tool": "websearch",
  "query": "...",
  "results_count": 42,
  "data": [...]
}
```

## Notas

- Los archivos son generados automaticamente por el servidor MCP
- El sistema de almacenamiento inteligente (`IntelligentStorage`) gestiona estos archivos
- Para exportar resultados use el endpoint: `POST /mcp/tools/call` con tool `file_search`
- Los archivos mayores a 10MB se comprimen automaticamente
