# Nuclear Crawler Hybrid - Project Structure

**Last Updated**: 2026-02-06  
**Philosophy**: REAL FFI with maximum language features, NO fallbacks

## 📂 Root Structure

```
nuclear-crawler-hybrid/
├── README.md                    # Main documentation
├── FFI_IMPLEMENTATION_COMPLETE.md  # FFI status
├── Cargo.toml                   # Rust dependencies
├── build.rs                     # Build script (FFI linking)
├── Makefile.pro                 # Professional Makefile
├── docker-compose.yml           # Docker configuration
├── Dockerfile                   # Container definition
│
├── ffi/                         # 🔥 FOREIGN FUNCTION INTERFACE (REAL)
├── src/                         # 🦀 Rust source code
├── docs/                        # 📚 Documentation & resources
├── tests/                       # 🧪 Integration tests
├── scripts/                     # 🔧 Build & automation scripts
├── mcp-servers/                 # 🔌 MCP server integrations
└── resultados/                  # 📊 Search results output
```

## 🔥 FFI Directory (PRIMARY ENGINE)

**Philosophy**: Extract maximum power from each language

```
ffi/
├── chapel/                      # 🧠 CHAPEL AI (PRIMARY ENGINE)
│   ├── ai/                      # Core neural networks
│   │   ├── nuclear_chapel_ai.chpl      # 2-layer NN + Adam optimizer
│   │   └── unified_nuclear_ai.chpl     # Integrated AI system
│   ├── training/                # Training engines
│   │   ├── training_pipeline.chpl      # 3-layer training pipeline
│   │   ├── data_mining.chpl            # K-means + anomaly detection
│   │   └── analysis.chpl               # Statistical analysis
│   ├── tools/                   # Development tools
│   │   ├── code_analyzer.chpl          # Code complexity analysis
│   │   ├── code_repair.chpl            # Intelligent code fixing
│   │   └── code_reviewer.chpl          # AI-powered code review
│   ├── mcp_integration/         # MCP direct integration
│   ├── Makefile                 # 8-engine build system
│   ├── build_chapel_real.sh     # Advanced compilation script
│   └── bin/                     # Compiled binaries (output)
│
├── jax/                         # GPU-accelerated ML
│   └── src/nuclear_jax.py       # JAX embeddings
│
├── mojo/                        # High-performance ML
│   └── mojo_chapel_bridge.mojo  # Chapel-Mojo bridge
│
├── julia_ml_training.jl         # Julia scientific ML
├── rust_ml_ffi.rs               # Rust FFI bindings
│
├── shared/                      # Compiled libraries (Windows)
│   ├── nuclear_zig.lib          # Zig SIMD
│   ├── nuclear_nim.lib          # Nim HTML parser
│   └── stealth_go.lib           # Go parallel HTTP
│
├── README.md                    # FFI guide
├── README_REAL_FFI.md           # Comprehensive FFI documentation
├── BUILD_INSTRUCTIONS.md        # Build instructions
└── LENGUAJES_Y_LIBRERIAS.md     # Language reference
```

### Chapel AI - 8 Engines

1. **Chapel AI Library** (`libchapel_ai.so`) - Neural network core
2. **Unified Nuclear AI** - Integrated intelligence system
3. **Training Pipeline** - 3-layer network + Adam optimizer
4. **Data Mining Engine** - K-means clustering + anomaly detection
5. **Scientific Analysis** - Statistics + hypothesis testing
6. **Code Analyzer** - Automated code analysis
7. **Code Repair** - Intelligent code fixing
8. **Code Reviewer** - AI-powered code review

## 🦀 Source Directory

```
src/
├── mcp/                         # MCP protocol implementation
│   ├── protocol.rs              # JSON-RPC 2.0 protocol (5 TOOLS)
│   ├── server.rs                # MCP server
│   └── tools/                   # Tool implementations
│       ├── websearch.rs
│       ├── premium_content.rs
│       ├── file_search_advanced.rs
│       ├── scan_workspace.rs
│       └── ai_dataset_trainer.rs
│
├── core/                        # Core functionality
│   ├── web_search.rs            # Web search engine
│   ├── premium_content_scraper.rs
│   └── file_ops.rs
│
├── ai/                          # AI integrations
│   └── huggingface_integration.rs
│
└── lib.rs                       # Library root
```

## 📚 Documentation Directory

```
docs/
├── examples/                    # Code examples
│   ├── chatbot_basic.rs
│   ├── chatbot_with_hf.rs
│   └── train_with_huggingface_dataset.py
│
├── models/                      # ML models & datasets (62M)
│   ├── chapel_osint/            # OSINT datasets
│   ├── chapel_scraping/         # AI scraping models
│   ├── data/                    # Math datasets (120K+ samples)
│   ├── mega_dataset/            # Large-scale training data
│   ├── powershell_dataset/      # PowerShell training
│   ├── trained_models/          # Pre-trained models
│   ├── ffi_rust/                # Rust FFI implementations
│   └── julia_expansion/         # Julia scientific tools
│
├── ai-training/                 # AI training pipeline
│   ├── curation/                # Dataset curation
│   ├── osint/                   # OSINT scraping
│   ├── scraping/                # Web scraping
│   └── training/                # Model training
│
├── agents/                      # Agent systems
│   └── unified_agent_system.chpl
│
├── chapel/                      # Chapel documentation
│
├── CHATBOT_GUIDE.md
├── CONTRIBUTING.md
├── HUGGINGFACE_INTEGRATION.md
└── dashboard.html
```

## 🧪 Tests Directory

```
tests/
├── integration_real_mcp.rs      # Real MCP server tests
└── chapel_ffi_integration.rs    # Chapel FFI tests
```

## 🔧 Scripts Directory

```
scripts/
├── build_all_ffi.sh             # Build all FFI libraries
├── setup_chapel.sh              # Setup Chapel environment
├── autorepair_chapel.ps1        # Chapel auto-repair
├── osint_dataset_generator.py   # OSINT dataset generation
├── advanced_optimization_agent.py
└── ...                          # Other automation scripts
```

## 🔌 MCP Servers Directory

```
mcp-servers/
├── github/                      # GitHub MCP Server (Go)
│   ├── cmd/                     # Command entry points
│   ├── pkg/                     # Public packages
│   ├── internal/                # Internal packages
│   └── go.mod                   # Go dependencies
│
└── README.md                    # MCP integration guide
```

## 📊 Resultados Directory

```
resultados/                      # Search results output
└── (generated search results)
```

## 🎯 Key Files in Root

### Essential Files (ONLY)

1. **README.md** - Main project documentation
2. **FFI_IMPLEMENTATION_COMPLETE.md** - FFI status and guide
3. **Cargo.toml** - Rust dependencies and project configuration
4. **build.rs** - Build script (FFI linking, compilation detection)
5. **Makefile.pro** - Professional build system
6. **docker-compose.yml** - Container orchestration
7. **Dockerfile** - Container definition

### Development Files

- `.gitignore` - Git ignore patterns
- `.cursorrules` - Cursor AI rules
- `.env.example` - Environment variables template

## 🏗️ Build Flow

### 1. Chapel AI (Primary Engine)

```bash
# Build Chapel with maximum features
cd ffi/chapel

# CPU only
./build_chapel_real.sh

# With GPU (100x faster)
GPU_ARCH=sm_86 ./build_chapel_real.sh

# With distributed computing (4 nodes)
NUM_LOCALES=4 ./build_chapel_real.sh

# Maximum power: GPU + Distributed
GPU_ARCH=sm_80 NUM_LOCALES=8 ./build_chapel_real.sh
```

### 2. Rust Project

```bash
# Build Rust with FFI
cargo build --release

# Run tests
cargo test --release

# Run MCP server
cargo run --bin nuclear-mcp --release
```

### 3. Integration

The `build.rs` script automatically:
1. Detects Chapel installation (system or local)
2. Links `libchapel_ai.so` if available
3. Links Go/Zig/Nim libraries on Windows
4. Provides clear instructions if FFI not available

## 📋 Design Principles

### 1. Clean Root Directory

**Rule**: Maximum 5 MD files + 2 config files in root
- ✅ README.md
- ✅ FFI_IMPLEMENTATION_COMPLETE.md
- ✅ Cargo.toml
- ✅ build.rs
- ✅ Makefile.pro
- ✅ docker-compose.yml
- ✅ Dockerfile

### 2. Organized Structure

**Folders**:
- `ffi/` - All FFI code and libraries
- `src/` - Rust source code
- `docs/` - Documentation and resources
- `tests/` - Integration tests
- `scripts/` - Build and automation scripts
- `mcp-servers/` - MCP server implementations
- `resultados/` - Search results output

### 3. Real FFI (No Fallbacks)

**Primary Engine**: Chapel AI with GPU + Multi-locale
**Philosophy**: Use maximum power from each language
**Windows-only**: Go, Zig, Nim
**Cross-platform**: Chapel, JAX, Julia, Mojo

### 4. Advanced Features

**Chapel**:
- BlockDist, CyclicDist, ReplicatedDist
- GPU kernels (`--gpu --gpu-arch=sm_XX`)
- Multi-locale (`--numLocales=N`)
- BLAS/LAPACK integration
- Atomic operations, parallel reductions

**Performance**: 90x+ speedup with GPU + distributed computing

## 🚀 Quick Start

```bash
# 1. Build Chapel AI
cd ffi/chapel
GPU_ARCH=sm_86 ./build_chapel_real.sh
cd ../..

# 2. Build Rust project
cargo build --release

# 3. Run MCP server
cargo run --bin nuclear-mcp --release

# 4. Test
cargo test --release
```

## 📚 Documentation Links

- [FFI Guide](ffi/README_REAL_FFI.md) - Comprehensive FFI documentation
- [Chapel Makefile](ffi/chapel/Makefile) - 8-engine build system
- [Build Instructions](ffi/BUILD_INSTRUCTIONS.md) - Build guide
- [Main README](README.md) - Project overview

## 🎯 Summary

- **Root**: Clean with only essential files
- **FFI**: Real compilation with maximum features
- **Chapel**: Primary engine with GPU + multi-locale
- **Structure**: Organized, logical, maintainable
- **Philosophy**: NO FALLBACKS - Real power only
