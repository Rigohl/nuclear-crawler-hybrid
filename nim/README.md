# 📁 Carpeta `nim/` - Integración FFI con Nim

## 🎯 Propósito
Integración de Nim para parsing HTML de alto rendimiento y procesamiento de texto.

## 🏗️ Arquitectura
- **Nim FFI bindings** para Rust
- **HTML parsing** optimizado
- **Text extraction** con expresiones regulares
- **Memory-safe** interfaces

## 📂 Contenido
- `src/` - Código fuente Nim
- Headers generados para FFI
- Librerías compiladas (en libs/)

## 🔧 Funciones
- `parse_html_content()` - Parsing HTML completo
- `extract_text_nodes()` - Extracción de texto
- `find_elements_by_selector()` - Búsqueda CSS-like
- `sanitize_html()` - Limpieza de HTML

## 🚀 Uso
```rust
// Carga automática desde nuclear_core.rs
let nim_parser = NimParser::new()?;

// Parsing HTML
let parsed = nim_parser.parse_html(&html_content)?;
```

## 🤖 Contexto para IA
Nim proporciona:

- **Velocidad de compilación** excepcional
- **Sintaxis Python-like** fácil de entender
- **Zero-cost abstractions** para performance
- **Interoperabilidad** perfecta con Rust

**Patrón**: Nim para lógica de parsing, Rust para control de flujo y memoria.
