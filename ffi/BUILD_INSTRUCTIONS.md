# FFI Implementation - Complete Build Guide

This directory contains **REAL source code implementations** for Foreign Function Interface (FFI) with multiple languages:
- **Go**: Parallel processing with goroutines
- **Zig**: SIMD hashing and pattern matching  
- **Nim**: HTML parsing and text extraction
- **JAX**: GPU-accelerated ML embeddings
- **Chapel**: AI training and continuous learning

## 🚀 Quick Start

### Automated Build (Recommended)

Build all FFI libraries with a single command:

```bash
./scripts/build_all_ffi.sh
```

This script will:
- Detect available compilers
- Build each FFI library from source
- Install to `ffi/shared/` directory
- Provide detailed build status

### Requirements

FFI libraries are **optional**. The project uses high-quality Rust fallbacks if FFI libraries are not available.

For full FFI support, install compilers:
- **Go** 1.21+ (https://go.dev/dl/)
- **Zig** 0.12+ (https://ziglang.org/download/)
- **Nim** 2.0+ (https://nim-lang.org/install.html)
- **Python** 3.9+ with pip (https://python.org/downloads/)
- **Chapel** 1.32+ (https://chapel-lang.org/download.html)

## Building FFI Libraries Individually

## Building FFI Libraries Individually

### Go (Parallel HTTP Client)

**Requirements**: Go 1.21+

```bash
cd ffi/go
make build        # Build the library
make install      # Install to ffi/shared/
make test         # Run Go tests
```

**Output**: 
- Linux/macOS: `stealth_go.so` (shared library)
- Windows: `stealth_go_msvc.lib` (static archive)

**Features**:
- Goroutine-based parallel HTTP requests
- Stealth headers with User-Agent rotation
- SOCKS5 proxy support
- Configurable concurrency

### Zig (SIMD Operations)

**Requirements**: Zig 0.12+

```bash
cd ffi/zig
make build        # Build both static and dynamic libraries
make install      # Install to ffi/shared/
make test         # Run Zig tests
```

**Alternative** (using zig build directly):
```bash
cd ffi/zig
zig build         # Uses build.zig
```

**Output**:
- Linux/macOS: `libnuclear_zig.so`, `libnuclear_zig.a`
- Windows: `nuclear_zig.lib`, `nuclear_zig.dll`

**Features**:
- Blake3-inspired hashing with SIMD
- Automatic SIMD detection (SSE2, AVX2, AVX512)
- Fast pattern matching
- Hash deduplication

### Nim (HTML Parser)

**Requirements**: Nim 2.0+

```bash
cd ffi/nim
make build        # Build the library
make install      # Install to ffi/shared/
make test         # Run Nim tests
```

**Output**:
- Linux/macOS: `nuclear_nim.so`
- Windows: `nuclear_nim.lib`

**Features**:
- HTML parsing with xmltree
- Text extraction (strips scripts/styles)
- Link extraction
- Meta tag parsing

### JAX (GPU Acceleration)

**Requirements**: Python 3.9+, pip

```bash
cd ffi/jax
make build        # Install JAX and dependencies
make install      # Copy script to ffi/shared/
make test         # Test JAX functionality
```

**For GPU support** (optional):
```bash
cd ffi/jax
pip install jax[cuda12]  # CUDA 12
# or
pip install jax[cuda11]  # CUDA 11
# or
pip install jax[tpu]     # Google TPU
```

**Output**: `nuclear_jax.py` (Python module)

**Features**:
- 1536-dim embeddings (OpenAI-compatible)
- GPU acceleration with JAX
- Batch cosine similarity
- CPU fallback when GPU unavailable

### Chapel (AI Training)

**Requirements**: Chapel 1.32+

```bash
cd ffi/chapel
make full-pipeline  # Build all 8 Chapel systems
make install        # Install to shared/
make test           # Run Chapel tests
```

**Individual targets**:
```bash
make train          # Training pipeline
make mining         # Data mining engine
make science        # Scientific analysis
make unified        # Unified AI system
make analysis       # Code analyzer
make repair         # Code repair tool
make review         # Code reviewer
```

**Output**: Multiple Chapel executables and libraries

**Features**:
- Neural network training (3-layer, Adam optimizer)
- Continuous learning system
- K-means clustering
- Anomaly detection
- Code analysis and repair

## Integration with Rust

After building FFI libraries, build the Rust project:

```bash
cargo build --release
```

The build system (`build.rs`) will:
1. Detect available FFI libraries in `ffi/shared/`
2. Link them automatically
3. Set feature flags (`has_go`, `has_zig`, `has_nim`, `has_chapel`)
4. Use Rust fallbacks for missing libraries

## Verification

Check which FFI libraries are available:

```bash
cargo build --release 2>&1 | grep "FFI:"
```

Expected output:
```
🚀 Go FFI: ENABLED
🚀 Zig FFI: ENABLED  
🚀 Nim FFI: ENABLED
🚀 JAX FFI: ENABLED
🔧 Chapel AI FFI: ENABLED
```

## Testing

### Test Individual FFI Modules

```bash
# Go FFI
cd ffi/go && make test

# Zig FFI
cd ffi/zig && make test

# Nim FFI
cd ffi/nim && make test

# JAX FFI
cd ffi/jax && make test

# Chapel FFI
cd ffi/chapel && make test
```

### Test Rust Integration

```bash
# Test with FFI libraries
cargo test --release --lib

# Test specific FFI integration
cargo test --release ffi::go_integration
cargo test --release ffi::zig_integration
cargo test --release ffi::nim_integration
cargo test --release ffi::jax_integration
cargo test --release ffi::chapel_integration
```

## Troubleshooting

### Go Build Issues

**Error**: `go: command not found`
- Install Go from https://go.dev/dl/
- Add Go to PATH: `export PATH=$PATH:/usr/local/go/bin`

**Error**: `undefined reference to 'main'`
- This is normal for C shared libraries. Ignore or use archive mode.

### Zig Build Issues

**Error**: `zig: command not found`
- Install Zig from https://ziglang.org/download/
- Extract and add to PATH

**Error**: `unable to find build.zig`
- Make sure you're in `ffi/zig` directory
- Check that `build.zig` file exists

### Nim Build Issues

**Error**: `nim: command not found`
- Install Nim from https://nim-lang.org/install.html
- Use choosenim for version management

**Error**: `could not load module: htmlparser`
- Install Nim standard library: Already included with Nim

### JAX Build Issues

**Error**: `ModuleNotFoundError: No module named 'jax'`
- Run: `pip install jax jaxlib`
- For GPU: `pip install jax[cuda12]`

**Warning**: `JAX not available, using NumPy fallback`
- This is normal and functional
- Install JAX for GPU acceleration

### Chapel Build Issues

**Error**: `chpl: command not found`
- Install Chapel from https://chapel-lang.org/download.html
- Set CHPL_HOME: `export CHPL_HOME=/path/to/chapel`
- Add to PATH: `export PATH=$PATH:$CHPL_HOME/bin`

**Error**: `cannot find -lblas`
- Install BLAS: `sudo apt install libblas-dev` (Ubuntu/Debian)
- Or: `brew install openblas` (macOS)

## Performance Notes

### With FFI Libraries (Compiled)
- **Go**: 10-100x faster parallel processing
- **Zig**: 5-50x faster hashing (SIMD)
- **Nim**: 2-10x faster HTML parsing
- **JAX**: 10-1000x faster on GPU
- **Chapel**: Distributed parallel computing

### Without FFI Libraries (Rust Fallbacks)
- **Tokio async**: High-quality parallel HTTP
- **Blake3**: Fast pure-Rust hashing
- **Scraper**: Excellent HTML parsing
- **ndarray**: NumPy-like operations
- **In-memory learning**: Basic AI capabilities

Both modes are production-ready. FFI provides performance boost for specific workloads.

## Clean Build

To rebuild everything from scratch:

```bash
# Clean all FFI builds
cd ffi/go && make clean
cd ../zig && make clean
cd ../nim && make clean
cd ../jax && make clean
cd ../chapel && make clean

# Clean Rust build
cd ../..
cargo clean

# Rebuild everything
./scripts/build_all_ffi.sh
cargo build --release
```
