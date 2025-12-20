# 📁 Carpeta `go/` - FFI Go Integration

## 🎯 Propósito
Implementación de Foreign Function Interface (FFI) en Go para procesamiento paralelo masivo y stealth requests.

## 🏗️ Arquitectura
- **100K goroutines paralelas** para requests HTTP simultáneos
- **Stealth headers rotativos** para evitar detección
- **Parallel URL fetching** con timeouts inteligentes
- **Integration con Rust** via C bindings

## 📂 Contenido
- `src/` - Código fuente Go
- `stealth_go.h` - Headers C para FFI
- `stealth_go_msvc.h` - Headers específicos para MSVC

## 🔧 Funciones Principales
- `fetch_urls_parallel()` - Fetch masivo de URLs
- `apply_stealth_headers()` - Aplicar headers anti-detección
- `parallel_crawler()` - Crawling distribuido

## 🚀 Compilación
```bash
# Compilar DLL para Windows
.\scripts\compile_go_msvc.ps1

# Verificar funcionamiento
# Se carga automáticamente en nuclear_core.rs
```

## 🤖 Contexto para IA
Esta carpeta contiene la implementación Go que proporciona:
- Paralelización masiva (100K+ goroutines)
- Headers stealth avanzados
- Fetching HTTP distribuido
- Integración perfecta con el núcleo Rust

**Importante**: El código Go se compila a DLL y se carga dinámicamente via `libloading` en Rust.
