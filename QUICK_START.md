# 🚀 Nuclear Crawler Hybrid - Quick Start

## Instalación en Claude Desktop

1. **Lee la guía de instalación:**
   ```bash
   cat INSTALL_CLAUDE_DESKTOP.md
   ```

2. **Configura Claude Desktop con el archivo mcp.json:**
   - Copia `mcp.json` a tu configuración de Claude Desktop
   - Linux/Mac: `~/.config/Claude/claude_desktop_config.json`
   - Windows: `%APPDATA%/Claude/claude_desktop_config.json`

## 📦 Binarios Disponibles

### nuclear-mcp (4.2M)
**MCP Server con 4 tools reales:**
```bash
./nuclear-mcp
# Listening on http://127.0.0.1:8079
```

**Tools:**
- ✅ `websearch` - 50 queries, 5s timeout, 55+ engines
- ✅ `deepweb_search` - 20 queries, 10s timeout, .onion sites
- ✅ `premium_content_scraper` - 20 queries, 15s timeout, Medium/ArXiv/O'Reilly
- ✅ `file_search` - 10 queries, 8s timeout, filesystem + grep

### nuclear-data (805K)
**CLI para gestionar datos extraídos:**
```bash
./nuclear-data stats              # Estadísticas
./nuclear-data search <pattern>   # Buscar resultados
./nuclear-data report             # Generar reporte
./nuclear-data export <format>    # Exportar datos
./nuclear-data categorize         # Categorizar
./nuclear-data deduplicate        # Desduplicar
./nuclear-data sources            # Listar fuentes
./nuclear-data trends             # Analizar tendencias
```

## 📚 Documentación

- **README.md** - Descripción general del proyecto
- **mcp.json** - Configuración MCP (protocolo JSON-RPC 2.0)
- **INSTALL_CLAUDE_DESKTOP.md** - Guía de instalación detallada

## ✅ Verificación

```bash
# Comprobar que los binarios funcionan
./nuclear-mcp --help
./nuclear-data stats
```

---
**Status:** ✅ Producción-ready | 🔥 4 Tools Reales | 🚀 MCP 2025
