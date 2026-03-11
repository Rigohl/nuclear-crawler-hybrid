# FFI Architecture - Real Backend Contracts (2026)

## Overview

Nuclear Crawler Hybrid uses explicit backend contracts for performance-critical operations:
- **REAL FFI**: Chapel shared library
- **WASM Runtimes**: Go, Zig, Nim portable backends
- **GPU Acceleration**: Repo-managed JAX subprocess backend

### **Chapel FFI** - AI Learning + GPU (PRIMARY, REAL)
Located: `ffi/chapel/`
- Real Chapel compiler integration (v2.1.0+)
- GPU support (CUDA, HIP, Metal experimental)
- Multi-locale distributed computing
- Pattern recognition + AI learning
- **STATUS: ✅ REAL FFI (compile-time detection, fails if not available)**
- Build: `cd ffi/chapel && ./Makefile`
- Integrated in: `src/chapel_integration.rs`

### **Go WASM** - Concurrent HTTP (Portable Runtime Backend)
Located: `ffi/wasm/go/`
- TinyGo → WASM compilation (concurrent model via wasmtime)
- NOT native FFI (build.rs links WASM, not .lib)
- Goroutines simulated in WASM via wasmtime workers
- **STATUS: ✅ EXPLICIT WASM BACKEND**
- Build: `cd ffi/wasm/go && tinygo build -target wasm`
- Integrated in: `src/go_integration.rs` (wasmtime runtime)

### **Zig WASM** - SIMD Operations (Portable Runtime Backend)
Located: `ffi/wasm/zig/`
- Zig SIMD compiled to WASM32
- NOT native FFI (no Windows .lib binding)
- SIMD patterns portable via WASM SIMD proposal
- **STATUS: ✅ EXPLICIT WASM BACKEND**
- Build: `cd ffi/wasm/zig && zig build-lib -target wasm32-freestanding`
- Integrated in: `src/zig_integration.rs` (wasmtime runtime)

### **Nim WASM** - Text Processing (Portable Runtime Backend)
Located: `ffi/wasm/nim/`
- Nim → Emscripten → WASM
- NOT native FFI (no direct Windows .lib)
- HTML parsing + DOM navigation via WASM
- **STATUS: ✅ EXPLICIT WASM BACKEND**
- Build: `cd ffi/wasm/nim && nim js lib.nim`
- Integrated in: `src/nim_integration.rs` (wasmtime runtime)

### **JAX Backend** - GPU Acceleration (Python Subprocess)
Located: `ffi/jax/real_jax_backend.py`
- GPU vectorization (CUDA, HIP, Metal)
- 1536-dim embeddings for ML
- Repo-managed subprocess entrypoint for JAX execution
- **STATUS: ✅ EXPLICIT SUBPROCESS BACKEND (requires Python 3.11+ + JAX installed)**
- Integrated in: `src/ffi/jax_integration.rs`

## File Structure

```
ffi/
├── chapel/              ← Chapel AI (✅ REAL FFI)
├── jax/                 ← Repo-managed JAX backend
├── wasm/                ← Portable runtime backends
│   ├── go/              ← TinyGo WASM
│   ├── zig/             ← Zig WASM
│   ├── nim/             ← Nim Emscripten
│   └── rust/            ← Rust WASM utilities
└── shared/              ← Shared headers (msvc runtime, etc.)
```

## Integration Points

### `build.rs`
- Chapel: strict detection and visible failure if missing
- Go/Zig/Nim static libraries: linked when available on supported platforms
- JAX: explicit subprocess contract checked at runtime

### Feature Flags
- `chapel_ffi`: Optional (Chapel binary)
- `wasm_ffi`: Default (WASM runtimes bundled)
- `jax_integration`: Optional (requires Python subprocess)

## Backend Strategy

**Chapel FFI (Primary):**
- If absent: Build fails with clear error message (**NO MOCK**)
- Production assumption: Chapel binary deployed with binary
- Dev: Can build without Chapel if feature disabled

**WASM (Go/Zig/Nim):**
- Runtime backend must be wired explicitly
- No silent substitution to stub implementations

**JAX (Subprocess):**
- If Python absent: Runtime error
- If JAX absent: Runtime error

## 2026 Versions

- Chapel: 2.1.0+ (GPU support experimental)
- Go TinyGo: 0.32.0+
- Zig: 0.15.2+ (SIMD)
- Nim: 1.6.20+
- JAX: 0.4.28+ (Python 3.11+)

## NO SILENT MOCKS POLICY

- ✅ Chapel fails HARD if not available (compile error, not runtime mock)
- ✅ WASM backends must be present and wired explicitly
- ✅ JAX subprocess fails visible (not hidden mock)
- ❌ ZERO silent fallbacks to stub implementations
- ❌ ZERO hardcoded test data in production code

## Maintenance

- Chapel: Update build.rs on new Chapel release
- WASM: Rebuild bundles when updating languages
- JAX: Keep ffi/jax/real_jax_backend.py aligned with runtime requirements
- Test: CI validates missing required backends fail visibly

