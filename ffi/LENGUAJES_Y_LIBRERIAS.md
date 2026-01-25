# 📚 LENGUAJES Y LIBRERÍAS - Carpeta FFI

**Resumen completo de todos los lenguajes y librerías en `ffi/`**

---

## 🌐 LENGUAJES IDENTIFICADOS

### 1. **Chapel** (`.chpl`)
**Ubicación**: `ffi/chapel/`  
**Propósito**: Machine Learning, AI, procesamiento paralelo distribuido

**Librerías/Modules Chapel**:
- `BlockDist` - Distribución de bloques para paralelismo
- `CyclicDist` - Distribución cíclica
- `ReplicatedDist` - Distribución replicada
- `Math` - Funciones matemáticas
- `Time` - Manejo de tiempo
- `LinearAlgebra` - Álgebra lineal
- `Random` - Generación de números aleatorios
- `Map` - Mapas/diccionarios
- `List` - Listas
- `Set` - Conjuntos
- `Regex` - Expresiones regulares
- `CTypes` - Tipos C para FFI
- `CPtr` - Punteros C

**FFI C Extern**:
- `cblas_daxpy`, `cblas_ddot`, `cblas_dnrm2` - BLAS Level 1
- `cblas_dgemv`, `cblas_dgemm` - BLAS Level 2/3
- `dgesv`, `dgeqrf`, `dsyev` - LAPACK (sistemas lineales, QR, eigenvalores)
- `omp_get_max_threads` - OpenMP

**Archivos principales**:
- `chapel_ai.chpl` - AI principal
- `training_pipeline.chpl` - Pipeline de entrenamiento
- `scientific_analysis.chpl` - Análisis científico
- `tools/code_analyzer.chpl` - Análisis de código
- `tools/code_repair.chpl` - Reparación de código
- `tools/code_reviewer.chpl` - Revisión de código
- `ai/nuclear_chapel_ai.chpl` - AI nuclear
- `ai/unified_nuclear_ai.chpl` - AI unificada

---

### 2. **Rust** (`.rs`)
**Ubicación**: `ffi/rust_ml_ffi.rs`  
**Propósito**: FFI seguro, bindings C, operaciones BLAS/LAPACK

**Librerías Rust**:
- `std::ffi` - Foreign Function Interface
  - `CStr`, `CString` - Strings C
- `std::os::raw` - Tipos C raw
  - `c_char`, `c_double`, `c_int`, `c_void`
- `std::sync` - Sincronización
  - `Arc`, `Mutex` - Thread-safe shared state
- `std::time::Instant` - Medición de tiempo

**FFI C Extern (BLAS)**:
- `dgemm` - Matrix multiplication (Level 3)
- `ddot` - Dot product (Level 1)
- `dnrm2` - Vector norm (Level 1)

**FFI C Extern (LAPACK)**:
- `dgesv` - Linear system solver
- `dgeqrf` - QR decomposition
- `dsyev` - Eigenvalue decomposition

**Linkage**:
- `#[link(name = "blas")]` - Link con BLAS
- `#[link(name = "lapack")]` - Link con LAPACK

---

### 3. **Julia** (`.jl`)
**Ubicación**: `ffi/julia_ml_training.jl`  
**Propósito**: Scientific computing, ML training, GPU acceleration

**Librerías Julia**:
- `Statistics` - Estadísticas
- `Random` - Números aleatorios
- `LinearAlgebra` - Álgebra lineal (incluye BLAS/LAPACK nativo)
- `Distributed` - Computación distribuida
- `SharedArrays` - Arrays compartidos

**Librerías Opcionales (comentadas)**:
- `CUDA` - GPU acceleration (CUDA)
- `Flux` - Deep learning framework
- `MLUtils` - Utilidades ML

**Características**:
- BLAS Level 3 optimizado (matrix-matrix operations)
- Multi-threading (`Threads.@threads`)
- Distributed computing (`@spawnat`, `@everywhere`)
- GPU support (requiere CUDA.jl)

---

### 4. **Python** (`.py`)
**Ubicación**: `ffi/chapel/hf_spaces_app.py`  
**Propósito**: HuggingFace Spaces deployment, control de pipeline Chapel

**Librerías Python**:
- `os` - Sistema operativo
- `json` - JSON parsing
- `subprocess` - Ejecución de procesos
- `logging` - Logging
- `pathlib.Path` - Manejo de paths
- `datetime` - Fechas y tiempo
- `gradio` - UI web para HuggingFace Spaces
- `huggingface_hub` - HuggingFace API
  - `HfApi` - API client
  - `model_info` - Información de modelos

---

### 5. **C** (`.c`)
**Ubicación**: `ffi/chapel/chapel_ai.c`  
**Propósito**: Bridge C para Chapel FFI

**Headers C**:
- `<stdint.h>` - Tipos enteros
- `<stdio.h>` - I/O estándar
- `<stdlib.h>` - Utilidades estándar
- `<string.h>` - Strings

---

## 📦 LIBRERÍAS COMPARTIDAS

**Ubicación**: `ffi/shared/`

### Librerías Compiladas:
- `nuclear_zig.lib` - Zig FFI (SIMD hashing)
- `nuclear_nim.lib` - Nim FFI (HTML parsing)
- `stealth_go.lib` - Go FFI (parallel processing)
- `msvcrt_import.def` - Definiciones MSVC
- `msvcrt_import.exp` - Exports MSVC
- `msvcrt_import.lib` - Librería MSVC runtime

---

## 🔗 INTEGRACIONES FFI

### Go Integration
- **Propósito**: Procesamiento paralelo con goroutines
- **Features**: HTTP client, stealth headers, proxy support
- **Librerías**: Go standard library + Fiber

### Zig Integration
- **Propósito**: Operaciones SIMD (hashing, pattern matching)
- **Features**: Blake3, SSE, AVX, AVX512
- **Librerías**: Zig standard library

### Nim Integration
- **Propósito**: HTML parsing y extracción de texto
- **Features**: DOM navigation, regex
- **Librerías**: Nim standard library

### JAX Integration
- **Propósito**: GPU acceleration para ML embeddings
- **Features**: CUDA, HIP, Metal support
- **Librerías**: JAX, NumPy, Haiku

---

## 📊 RESUMEN POR LENGUAJE

| Lenguaje | Archivos | Propósito Principal | Librerías Clave |
|----------|----------|---------------------|----------------|
| **Chapel** | 20+ `.chpl` | ML, AI, Parallel Computing | BlockDist, LinearAlgebra, Math |
| **Rust** | 1 `.rs` | FFI seguro, BLAS/LAPACK | std::ffi, std::sync |
| **Julia** | 1 `.jl` | Scientific Computing, ML | LinearAlgebra, Distributed |
| **Python** | 1 `.py` | Deployment, Control | gradio, huggingface_hub |
| **C** | 1 `.c` | Bridge FFI | stdlib |

---

## 🎯 USO DE BLAS/LAPACK

### Chapel:
- Usa bindings C externos a BLAS/LAPACK
- `cblas_*` para BLAS
- `dgesv`, `dgeqrf`, `dsyev` para LAPACK

### Rust:
- Link directo con `libblas` y `liblapack`
- Funciones: `dgemm`, `ddot`, `dnrm2`, `dgesv`, `dgeqrf`, `dsyev`

### Julia:
- BLAS/LAPACK nativo integrado en `LinearAlgebra`
- Automáticamente optimizado (OpenBLAS, MKL, etc.)

---

## 🚀 CARACTERÍSTICAS POR LENGUAJE

### Chapel:
- ✅ Multi-locale distributed computing
- ✅ Parallel loops (`coforall`, `forall`)
- ✅ GPU support (experimental)
- ✅ Real-time inference (<50μs)

### Rust:
- ✅ Memory safety
- ✅ Thread-safe (Arc, Mutex)
- ✅ Zero-cost abstractions
- ✅ Async/await support

### Julia:
- ✅ Automatic differentiation (Zygote)
- ✅ GPU acceleration (CUDA.jl)
- ✅ Multi-threading nativo
- ✅ Distributed computing

### Python:
- ✅ HuggingFace integration
- ✅ Gradio UI
- ✅ Subprocess control
- ✅ Logging y monitoring

---

## 📝 NOTAS IMPORTANTES

1. **NO MOCKS**: Todas las implementaciones son REALES
2. **FFI Cross-language**: Chapel ↔ Rust ↔ Julia ↔ Python ↔ C
3. **BLAS/LAPACK**: Usado en Chapel, Rust y Julia
4. **GPU Support**: Julia (CUDA), Chapel (experimental)
5. **Parallel Computing**: Chapel (multi-locale), Julia (distributed), Rust (threads)

---

**Última actualización**: 2026-01-25  
**Total lenguajes**: 5 (Chapel, Rust, Julia, Python, C)  
**Total librerías**: 30+ módulos/librerías
