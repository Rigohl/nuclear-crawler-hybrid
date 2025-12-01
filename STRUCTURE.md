# 📁 Estructura de NUCLEAR_CRAWLER_HYBRID

Proyecto organizado para máxima claridad.

## Estructura

\\\
NUCLEAR_CRAWLER_HYBRID/
├── src/                    # Código fuente Rust
├── scripts/
│   ├── server/            # Scripts de servidor
│   ├── build/             # Scripts de compilación
│   └── *.py               # Scripts Python
├── c_editors/             # Editores en C
├── libs/                  # Librerías compiladas
├── go/                    # Código Go
├── zig/                   # Código Zig
├── examples/              # Ejemplos
├── resultados/            # Resultados de crawling
├── target/                # Build artifacts (Rust)
│
├── Cargo.toml             # Configuración Rust
├── Dockerfile             # Docker principal
├── docker-compose.yml     # Docker Compose
├── Makefile               # Make build
│
└── docs/ (en raíz)
    ├── README.md                          # Principal
    ├── NUCLEAR_EXTREME_CRAWLER.md         # Docs Nuclear
    ├── NUCLEAR_CRAWLER_DOCUMENTATION.md   # Docs técnicos
    ├── REFACTORIZATION_GUIDE.md           # Guía refactorización
    └── REFACTORIZATION_SUMMARY.md         # Resumen refactorización
\\\

## 🚀 Uso Rápido

### Servidor Nuclear
\\\powershell
.\scripts\server\INICIAR_SERVIDOR.bat
\\\

### Build Rust
\\\ash
cargo build --release
\\\

### Docker
\\\ash
docker-compose up
\\\

## 📚 Documentación

Ver archivos .md en raíz para documentación completa.

---

**Organizado:** 2025-12-01 04:36
