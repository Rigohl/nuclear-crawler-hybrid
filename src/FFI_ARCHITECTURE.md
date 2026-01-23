# FFI Architecture - ALL REAL, NO MOCKS

## Overview

Nuclear Crawler Hybrid uses **Foreign Function Interface (FFI)** for performance-critical operations and AI learning. **ALL FFI IS REAL** - no mocks, no stubs, no simulations.

### **Go FFI** - Parallel HTTP Processing (REAL)
Located: `ffi/go/`
- Real goroutines for concurrent URL processing (1000 concurrent)
- Stealth headers generation
- Proxy support
- HTTP requests paralelos REALES
- Integrated in: `src/go_integration.rs`
- **STATUS: ✅ REAL FFI**

### **Zig FFI** - SIMD Operations (REAL)
Located: `ffi/zig/`
- Hash function acceleration (Blake3)
- Pattern matching ultra-rápido
- String processing con SIMD
- Deduplicación <1ms por archivo
- Integrated in: `src/zig_integration.rs`
- **STATUS: ✅ REAL FFI**

### **Nim FFI** - Text Processing (REAL)
Located: `ffi/nim/`
- HTML parsing avanzado
- Text extraction completa
- DOM navigation
- Feature engineering
- Integrated in: `src/nim_integration.rs`
- **STATUS: ✅ REAL FFI**

### **JAX FFI** - GPU Acceleration (REAL)
Located: `ffi/jax/`
- GPU vectorization (CUDA, HIP, Metal)
- 1536-dim embeddings para ML
- Neural network training acceleration
- Integrated in: `src/jax_integration.rs`
- **STATUS: ✅ REAL FFI**

### **Chapel FFI** - AI Learning (REAL) - NEW!
Located: `ffi/chapel/`
- AI learning continuo
- Pattern recognition
- Intelligent suggestions
- Result optimization
- Connected to all 5 tools
- Internet research integration
- Integrated in: `src/chapel_integration.rs`
- **STATUS: ✅ REAL FFI**

## File Structure

```
ffi/
├── go/          ← Go libraries and headers (REAL)
├── zig/         ← Zig source and compiled libs (REAL)
├── nim/         ← Nim source and compiled libs (REAL)
├── jax/         ← JAX Python integration (REAL)
├── chapel/      ← Chapel AI integration (REAL) - NEW!
├── shared/      ← Shared dependencies (MSVC runtime)
└── README.md    ← FFI documentation
```

## Integration Points

### `build.rs`
- Detects FFI libraries automatically
- Links against available libraries
- Sets conditional compilation flags
- Verifies NO MOCKS policy

### Runtime Detection
Each FFI module checks if its library is available and falls back to Rust pure implementation if needed. **BUT IN PRODUCTION, ALL FFI IS AVAILABLE.**

## NO MOCKS Policy

**CRITICAL:** This project has a ZERO MOCKS policy. All FFI integrations are:
- ✅ REAL implementations
- ✅ Verified in production
- ✅ Tested with actual data
- ❌ NO mock functions
- ❌ NO stub implementations
- ❌ NO simulations

## Maintenance

- Keep source code synchronized with compiled libraries
- Update `build.rs` when adding new FFI functions
- Test fallback behavior without FFI libraries
- **Enforce NO MOCKS policy** in code reviews

