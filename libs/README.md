# 📁 Carpeta `libs/` - Librerías Compiladas FFI

## 🎯 Propósito
Contiene las librerías compiladas (DLLs/.so) de las implementaciones FFI en lenguajes nativos.

## 📂 Contenido
- `msvcrt_import.def` - Definiciones de importación MSVC
- `msvcrt_import.exp` - Exports MSVC
- Librerías compiladas (generadas por scripts de build)

## 🔧 Funciones
- **Go Library**: `nuclear_go.dll` - Procesamiento paralelo
- **Zig Library**: `nuclear_zig.dll` - SIMD operations
- **Nim Library**: `nuclear_nim.dll` - HTML parsing avanzado

## 🚀 Generación
```bash
# Compilar todas las FFI libraries
.\scripts\compile_go_msvc.ps1
.\scripts\compile_zig_msvc.ps1
.\scripts\compile_nim_msvc.ps1

# Verificar carga
cargo run  # El sistema detecta y carga automáticamente
```

## 🤖 Contexto para IA
Esta carpeta almacena las librerías nativas compiladas que se cargan dinámicamente:

- **Carga automática** por `nuclear_core.rs`
- **Hot-reload capable** durante desarrollo
- **Cross-platform** (Windows DLLs, Linux .so)
- **Performance crítica** - estas libs proporcionan 90% del rendimiento

**Nota**: Los archivos `.def` y `.exp` son para linking MSVC.
