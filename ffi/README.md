# FFI Integration - Unified Structure

## 📦 Consolidated Foreign Function Interface

Esta carpeta contiene todas las librerías compiladas y código fuente para FFI (Foreign Function Interface) con Go, Zig y Nim.

### 📂 Estructura

```
ffi/
├── go/              # Go FFI - Parallel processing + stealth headers
│   ├── src/         # Go source code
│   ├── stealth_go.a
│   ├── stealth_go.lib
│   ├── stealth_go_msvc.a
│   ├── stealth_go_msvc.lib
│   ├── stealth_go.h
│   └── stealth_go_msvc.h
│
├── zig/             # Zig FFI - SIMD hashing + parsing
│   ├── src/         # Zig source code
│   ├── build.zig
│   ├── nuclear_zig.lib
│   ├── lib.lib
│   └── zig-out/     # Build output
│
├── nim/             # Nim FFI - HTML parsing + text extraction
│   ├── src/         # Nim source code
│   └── nuclear_nim.lib
│
└── shared/          # Shared libraries (imported from libs/)
    ├── nuclear_zig.lib
    ├── nuclear_nim.lib
    ├── stealth_go.lib
    └── msvcrt_import.*

```

## 🔥 FFI Modules

### Go Integration (`go/`)
- **Purpose**: Parallel URL processing with real goroutines
- **Features**:
  - Real HTTP client with stealth headers
  - User-Agent rotation
  - Proxy support
  - Multi-threaded processing
- **Usage**: `src/go_integration.rs`

### Zig Integration (`zig/`)
- **Purpose**: SIMD operations for hashing and pattern matching
- **Features**:
  - High-performance hashing
  - SIMD vectorization
  - Fast parsing
- **Usage**: `src/zig_integration.rs`

### Nim Integration (`nim/`)
- **Purpose**: HTML parsing and text extraction
- **Features**:
  - HTML DOM navigation
  - Text content extraction
  - Fast pattern matching
- **Usage**: `src/nim_integration.rs`

## 🔗 Build Integration

The `build.rs` script automatically:
1. Detects FFI libraries in `ffi/go/`, `ffi/zig/`, `ffi/nim/`, and `ffi/shared/`
2. Links them against the Rust binary
3. Sets cfg flags (`has_go`, `has_zig`, `has_nim`) when available

## 📝 Notes

- **Deprecation**: Old structure (`go/`, `zig/`, `nim/`, `libs/`) has been consolidated into `ffi/`
- **Backwards compatibility**: `.gitignore` still excludes old locations for safety
- **Shared libraries**: MSVC runtime imports and cross-FFI dependencies are in `ffi/shared/`

## 🚀 Compilation

```bash
cargo build --release
```

FFI modules are compiled automatically if libraries are present.
