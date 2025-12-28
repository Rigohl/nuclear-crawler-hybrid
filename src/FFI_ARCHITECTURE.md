# FFI Architecture

## Overview

Nuclear Crawler Hybrid uses **Foreign Function Interface (FFI)** for performance-critical operations:

### **Go FFI** - Parallel HTTP Processing
Located: `ffi/go/`
- Real goroutines for concurrent URL processing
- Stealth headers generation
- Proxy support
- Integrated in: `src/go_integration.rs`

### **Zig FFI** - SIMD Operations
Located: `ffi/zig/`
- Hash function acceleration
- Pattern matching
- String processing
- Integrated in: `src/zig_integration.rs`

### **Nim FFI** - Text Processing
Located: `ffi/nim/`
- HTML parsing
- Text extraction
- DOM navigation
- Integrated in: `src/nim_integration.rs`

## File Structure

```
ffi/
├── go/          ← Go libraries and headers
├── zig/         ← Zig source and compiled libs
├── nim/         ← Nim source and compiled libs
├── shared/      ← Shared dependencies (MSVC runtime)
└── README.md    ← FFI documentation
```

## Integration Points

### `build.rs`
- Detects FFI libraries automatically
- Links against available libraries
- Sets conditional compilation flags

### Runtime Detection
Each FFI module checks if its library is available and falls back to Rust pure implementation if needed.

## Maintenance

- Keep source code synchronized with compiled libraries
- Update `build.rs` when adding new FFI functions
- Test fallback behavior without FFI libraries

