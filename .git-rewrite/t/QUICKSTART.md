# ⚡ GUÍA RÁPIDA

## 1. Compilar (3 minutos)

```powershell
cd "C:\Users\DELL\Desktop\hf_spaces\NUCLEAR_CRAWLER_HYBRID"
cargo build --release
```

Resultado: `target/release/nuclear-mcp.exe`

---

## 2. Ejecutar

### Modo HTTP (Recomendado)
```powershell
.\target\release\nuclear-mcp.exe --mode http --port 8080
```

### Modo STDIO (Claude Desktop)
```powershell
.\target\release\nuclear-mcp.exe --mode studio
```

---

## 3. Probar

### Health Check
```bash
curl http://localhost:8080/health
```

### Búsqueda Web
```bash
curl -X POST http://localhost:8080/call \
  -H "Content-Type: application/json" \
  -d '{"name": "websearch", "arguments": {"queries": ["rust async"]}}'
```

### Estadísticas
```bash
curl -X POST http://localhost:8080/call \
  -H "Content-Type: application/json" \
  -d '{"name": "stats", "arguments": {}}'
```

---

## 4. Configurar Claude Desktop

Edita: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "nuclear-mcp": {
      "command": "C:\\Users\\DELL\\Desktop\\hf_spaces\\NUCLEAR_CRAWLER_HYBRID\\target\\release\\nuclear-mcp.exe",
      "args": ["--mode", "studio"]
    }
  }
}
```

---

## 5. Troubleshooting

| Problema | Solución |
|----------|----------|
| Port in use | `--port 8081` |
| No results | Espera 2s, query más específico |
| Compile error | `cargo clean && cargo build` |
| FFI error | Go FFI requiere `stealth_go_msvc.lib` |

---

## 6. Resultados

Los resultados se guardan en:
```
resultados/
├── nuclear_results.db    # SQLite + FTS5
├── urls_visited.txt      # Log de URLs
└── *.json                # Resultados por búsqueda
```

---

**¿Listo en 5 minutos?** ✅
