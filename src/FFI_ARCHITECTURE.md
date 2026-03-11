# FFI Architecture - HYBRID REAL FFI + WASM FALLBACK (2026)

## Overview

Nuclear Crawler Hybrid uses **Foreign Function Interface (FFI)** and **WebAssembly (WASM)** for performance-critical operations:
- **REAL FFI**: Chapel (multi-platform GPU support)
- **WASM Fallback**: Go, Zig, Nim (portable, browser-compatible)
- **GPU Acceleration**: JAX subprocess integration

### **Chapel FFI** - AI Learning + GPU (PRIMARY, REAL)
Located: `ffi/chapel/`
- Real Chapel compiler integration (v2.1.0+)
- GPU support (CUDA, HIP, Metal experimental)
- Multi-locale distributed computing
- Pattern recognition + AI learning
- **STATUS: ✅ REAL FFI (compile-time detection, fails if not available)**
- Build: `cd ffi/chapel && ./Makefile`
- Integrated in: `src/chapel_integration.rs`

### **Go WASM** - Concurrent HTTP (WASM, Portable)
Located: `ffi/wasm/go/`
- TinyGo → WASM compilation (concurrent model via wasmtime)
- NOT native FFI (build.rs links WASM, not .lib)
- Goroutines simulated in WASM via wasmtime workers
- **STATUS: ⚠️ WASM BUNDLE (not native FFI)**
- Build: `cd ffi/wasm/go && tinygo build -target wasm`
- Integrated in: `src/go_integration.rs` (wasmtime runtime)

### **Zig WASM** - SIMD Operations (WASM, Portable)
Located: `ffi/wasm/zig/`
- Zig SIMD compiled to WASM32
- NOT native FFI (no Windows .lib binding)
- SIMD patterns portable via WASM SIMD proposal
- **STATUS: ⚠️ WASM BUNDLE (not native FFI)**
- Build: `cd ffi/wasm/zig && zig build-lib -target wasm32-freestanding`
- Integrated in: `src/zig_integration.rs` (wasmtime runtime)

### **Nim WASM** - Text Processing (WASM, Portable)
Located: `ffi/wasm/nim/`
- Nim → Emscripten → WASM
- NOT native FFI (no direct Windows .lib)
- HTML parsing + DOM navigation via WASM
- **STATUS: ⚠️ WASM BUNDLE (not native FFI)**
- Build: `cd ffi/wasm/nim && nim js lib.nim`
- Integrated in: `src/nim_integration.rs` (wasmtime runtime)

### **JAX FFI** - GPU Acceleration (Python Subprocess)
Located: NOT in `ffi/` (subprocess integration)
- GPU vectorization (CUDA, HIP, Metal)
- 1536-dim embeddings for ML
- Subprocess spawn: `python -c "import jax; ..."`
- **STATUS: ⚠️ SUBPROCESS (not FFI, requires Python 3.11+ + JAX installed)**
- Integrated in: `src/jax_integration.rs`

## File Structure

```
ffi/
├── chapel/              ← Chapel AI (✅ REAL FFI)
├── wasm/                ← WASM bundles (⚠️ NOT FFI, portable)
│   ├── go/              ← TinyGo WASM
│   ├── zig/             ← Zig WASM
│   ├── nim/             ← Nim Emscripten
│   └── rust/            ← Rust WASM utilities
└── shared/              ← Shared headers (msvc runtime, etc.)
```

## Integration Points

### `build.rs`
- Chapel: **STRICT DETECTION** (fails if not available in production)
- WASM (Go/Zig/Nim): Bundled in binary (always available)
- JAX: Subprocess check at runtime

### Feature Flags
- `chapel_ffi`: Optional (Chapel binary)
- `wasm_ffi`: Default (WASM runtimes bundled)
- `jax_integration`: Optional (requires Python subprocess)

## Fallback Strategy

**Chapel FFI (Primary):**
- If absent: Build fails with clear error message (**NO MOCK**)
- Production assumption: Chapel binary deployed with binary
- Dev: Can build without Chapel if feature disabled

**WASM (Go/Zig/Nim):**
- Always available (bundled WASM binaries)
- NO fallbacks (WASM is the fallback itself)

**JAX (Subprocess):**
- If Python absent: Runtime error
- If JAX absent: Graceful degradation (returns empty result)

## 2026 Versions

- Chapel: 2.1.0+ (GPU support experimental)
- Go TinyGo: 0.32.0+
- Zig: 0.15.2+ (SIMD)
- Nim: 1.6.20+
- JAX: 0.4.28+ (Python 3.11+)

## NO SILENT MOCKS POLICY

- ✅ Chapel fails HARD if not available (compile error, not runtime mock)
- ✅ WASM always available (bundled in binary)
- ✅ JAX subprocess fails visible (not hidden mock)
- ❌ ZERO silent fallbacks to stub implementations
- ❌ ZERO hardcoded test data in production code

## Maintenance

- Chapel: Update build.rs on new Chapel release
- WASM: Rebuild bundles when updating languages
- JAX: Document runtime Python requirements
- Test: CI validates Chapel fails correctly, WASM always works

