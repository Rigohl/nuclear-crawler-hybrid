# GitHub Copilot Instructions: nuclear-crawler-hybrid

> **Last Updated**: 2026-01-24 (Post-Integration Update)

## 🎯 TL;DR

**Nuclear Crawler Hybrid** is a unified AI & Data Intelligence platform with:
- **Rust MCP Server** exposing **exactly 7 tools** over JSON-RPC 2.0
- **Chapel AI Training Engine** with 120K+ datasets, 8 parallel systems
- **GitHub MCP Server** (Go) for automation
- **Multi-language datasets** (Mojo, Julia, Python processors)
- **NO MOCKS** - all tests hit real running servers and real data

---

## 🔒 Critical Constraints (DO NOT CHANGE)

### MCP Protocol Rules
- **EXACTLY 7 MCP TOOLS** — Enforced by `test_exactly_7_tools` in `src/mcp/protocol.rs`
- **Tool Names**: `websearch`, `premium`, `file_search`, `scan`, `ai_dataset_trainer`, `parallel_engine`, `osint_intelligence`
- **CI Validation**: `.github/workflows/mcp-validation.yml` and `scripts/validate_7_tools.sh`
- **Adding/Removing Tools**: ❌ FORBIDDEN (breaks CI and tests)

### Integration Rules
- **NO MOCKS/STUBS** — Integration tests use real HTTP/data (`tests/integration_real_mcp.rs`)
- **CHAPEL CORE** — Chapel AI is the ML training engine (ffi/chapel/)
- **FFI HEAVY** — Multi-language FFI (Chapel, Go, Zig, Nim) requires careful builds
- **DATASETS** — models/ contains 120K+ samples, paths are relative from Chapel

### Known Issues
- **bincode v3.0.0** — Contains `compile_error!`, builds may fail (document in PRs)
- **FFI Linking** — Windows/MSVC linking in `build.rs` is complex
- **Binary Name** — Mismatch between `nuclear-mcp` and `nuclear_ultimate` in some tests

---

## 🏗️ Project Structure (Complete)

```
nuclear-crawler-hybrid/
├── ffi/chapel/                    # Chapel AI Training Engine (CORE ML)
│   ├── ai/                        # Core AI modules
│   │   ├── nuclear_chapel_ai.chpl       # Neural network core
│   │   └── unified_nuclear_ai.chpl      # Integrated intelligence
│   ├── training/                  # Training engines
│   │   ├── training_pipeline.chpl       # 3-layer network + Adam
│   │   ├── data_mining.chpl             # K-means, anomaly detection
│   │   └── analysis.chpl                # Statistical analysis
│   ├── tools/                     # Development tools
│   │   ├── code_analyzer.chpl           # Static analysis
│   │   ├── code_repair.chpl             # 4-pass auto-repair
│   │   └── code_reviewer.chpl           # Production certification
│   ├── Makefile                   # Build system (8 engines)
│   ├── space.yaml                 # HuggingFace Spaces config
│   ├── hf_spaces_app.py          # HF deployment app
│   └── *.md                       # Architecture docs
│
├── models/                        # Integrated Datasets (120K+ samples)
│   ├── chapel_osint/              # OSINT datasets (Mojo format)
│   ├── chapel_scraping/           # AI scraping models
│   ├── data/                      # Math datasets
│   │   └── massive_math_dataset.jsonl   # 120K+ samples
│   ├── mega_dataset/              # Large-scale training data
│   ├── powershell_dataset/        # PowerShell training samples
│   ├── trained_models/            # Pre-trained models (Chapel, JAX)
│   ├── ffi_rust/                  # Rust FFI for datasets
│   └── julia_expansion/           # Julia scientific ML tools
│
├── mcp-servers/                   # MCP Server Integrations
│   ├── github/                    # GitHub MCP Server (Go)
│   │   ├── cmd/                   # Command entry points
│   │   ├── pkg/                   # Public packages
│   │   │   ├── github/            # GitHub API tools
│   │   │   └── inventory/         # Resource inventory
│   │   ├── internal/              # Internal packages
│   │   │   └── ghmcp/             # MCP protocol impl
│   │   ├── go.mod                 # Go dependencies
│   │   ├── Makefile               # Go build system
│   │   └── README.md              # Usage instructions
│   └── README.md                  # MCP integration guide
│
├── src/                           # Rust MCP Server (MAIN)
│   ├── mcp/
│   │   ├── protocol.rs            # ⚠️ EXACTLY 7 TOOLS (SACRED)
│   │   ├── server.rs              # JSON-RPC dispatch
│   │   └── tools/                 # Tool implementations
│   │       ├── websearch.rs
│   │       ├── premium_content.rs
│   │       ├── file_search_advanced.rs
│   │       ├── scan_workspace.rs
│   │       └── ai_dataset_trainer.rs
│   ├── core/                      # Core functionality
│   │   ├── web_search.rs          # Web search engine
│   │   ├── premium_content_scraper.rs
│   │   └── file_ops.rs
│   ├── ai/                        # AI integrations
│   │   └── huggingface_integration.rs
│   └── lib.rs                     # Library root
│
├── tests/                         # Integration Tests
│   ├── integration_real_mcp.rs    # Real server tests
│   └── chapel_ffi_integration.rs  # Chapel FFI tests
│
├── .github/
│   ├── workflows/                 # CI/CD Pipelines
│   │   ├── mcp-validation.yml           # MCP 5-tool validation
│   │   ├── chapel-ai-learning-hub.yml   # Chapel training
│   │   ├── ffi-validation.yml           # FFI checks
│   │   └── full-validation.yml          # Complete validation
│   └── copilot-instructions.md    # This file
│
├── README.md                      # Main documentation
├── QUICK_START.md                 # Installation guide
├── INTEGRATION_STATUS.md          # Integration report
├── TOOLS.md                       # 7 MCP tools description
├── Cargo.toml                     # Rust dependencies
└── .cursorrules                   # Cursor behavior rules
```

---

## ⚡ Quick Commands (Most-Used)

### Rust (Primary Language)
```bash
# Format check
cargo fmt -- --check

# Build (may fail due to bincode issue)
cargo build --release --all-targets

# Run MCP server
cargo run --bin nuclear-mcp --release
# Test: curl http://localhost:8079/health

# Run tests
cargo test --lib
cargo test test_exactly_7_tools
cargo test --test integration_real_mcp --release -- --nocapture --test-threads=1

# Validate 7 tools
./scripts/validate_7_tools.sh
```

### Chapel AI (ML Training)
```bash
cd ffi/chapel

# Build all systems (8 engines)
make full-pipeline

# Individual builds
make train              # Training pipeline
make mining             # Data mining engine
make science            # Scientific analysis
make unified            # Unified AI system
make analysis           # Code analyzer
make repair             # Code repair tool
make review             # Code reviewer

# Run systems
make run                # Run training
make mine               # Run data mining
make analyze            # Run analysis
make run-unified        # Run unified AI
make execute-all        # Run ALL 8 systems

# Development
make check              # Syntax validation
make test               # Run tests
make clean              # Clean artifacts
```

### Go (GitHub MCP Server)
```bash
cd mcp-servers/github

# Build
go build -o github-mcp-server

# Test
go test ./...

# Run (requires GITHUB_TOKEN)
export GITHUB_TOKEN="ghp_xxxxx"
./github-mcp-server
```

### Multi-Language Build (Complete)
```bash
# Build everything
cargo build --release              # Rust
cd ffi/chapel && make full-pipeline  # Chapel
cd ../../mcp-servers/github && go build  # Go
```

---

## 🔧 How MCP Works (Examples)

### MCP JSON-RPC Protocol

**List tools:**
```bash
POST /mcp/tools/list
{
  "jsonrpc": "2.0",
  "id": "1",
  "method": "tools/list",
  "params": {}
}
```

**Call tool (websearch):**
```bash
POST /mcp/tools/call
{
  "jsonrpc": "2.0",
  "id": "1",
  "method": "tools/call",
  "params": {
    "name": "websearch",
    "arguments": {
      "query": "rust async patterns"
    }
  }
}
```

**Rust Helpers:**
```rust
// In src/mcp/protocol.rs
MCPRequest::list_tools()
MCPRequest::call_tool("websearch", json!({"query": "..."}))
```

---

## 📚 Key Components to Know

### 1. MCP Tools (Rust) - SACRED
**Location**: `src/mcp/protocol.rs` and `src/mcp/tools/`

**The 7 Tools:**
1. **websearch** - DuckDuckGo + Brave search integration
2. **premium** - Premium content extraction (Cloudflare bypass)
3. **file_search** - Advanced file operations
4. **scan** - Workspace analysis
5. **ai_dataset_trainer** - Dataset generation for ML
6. **parallel_engine** - Parallel processing engine (Go+SIMD+GPU+Chapel)
7. **osint_intelligence** - OSINT intelligence with Chapel AI

**Rules:**
- ⚠️ NEVER add an 8th tool
- ⚠️ NEVER remove any of the 7 tools
- ✅ Can modify implementations
- ✅ Can enhance features within tools

### 2. Chapel AI Training Engine
**Location**: `ffi/chapel/`

**8 Systems:**
1. Chapel AI Library (`libchapel_ai.so`)
2. Unified Nuclear AI
3. Training Pipeline
4. Data Mining Engine
5. Scientific Analysis
6. Code Analyzer
7. Code Repair Engine
8. Code Reviewer

**Key Files:**
- `ai/nuclear_chapel_ai.chpl` - Core neural network
- `ai/unified_nuclear_ai.chpl` - Integrated intelligence
- `training/*.chpl` - Training engines
- `tools/*.chpl` - Development tools
- `Makefile` - Build system

**Usage from Rust:**
```rust
// FFI integration in src/chapel_integration.rs
let chapel_ai = ChapelAI::new();
chapel_ai.learn_from_operation(...)?;
let advice = chapel_ai.get_advice(...)?;
```

### 3. Integrated Datasets
**Location**: `models/`

**Contents:**
- 120K+ training samples (Math, PowerShell, OSINT)
- Mojo dataset processors
- Julia scientific ML tools
- Rust FFI implementations
- Pre-trained models (Chapel, JAX)

**Access from Chapel:**
```chapel
// Datasets are relative from ffi/chapel/
// Path: ../../../models/data/massive_math_dataset.jsonl
```

### 4. GitHub MCP Server (Go)
**Location**: `mcp-servers/github/`

**Features:**
- Full GitHub API coverage
- Repository management
- Issue/PR operations
- Code search
- GitHub Actions
- Security alerts

**Usage:**
```bash
export GITHUB_TOKEN="ghp_xxxxx"
cd mcp-servers/github
./github-mcp-server
# Use with Claude Desktop, Cursor, or other MCP clients
```

---

## 🧪 Testing Strategy

### Rust Tests
```bash
# Unit tests
cargo test --lib

# Integration tests (real server)
cargo test --test integration_real_mcp --release -- --nocapture --test-threads=1

# 7-tool validation (CRITICAL)
cargo test test_exactly_7_tools

# Script validation
./scripts/validate_7_tools.sh
```

### Chapel Tests
```bash
cd ffi/chapel

# Syntax validation
make check

# Unit tests
make test

# Full pipeline test
make execute-all
```

### Go Tests
```bash
cd mcp-servers/github
go test ./...
go test -v ./pkg/github/
```

### Integration Testing
1. Build all components (Rust, Chapel, Go)
2. Run Rust integration tests
3. Verify Chapel FFI works
4. Test GitHub MCP server
5. Validate 7 tools constraint

---

## 🚨 Common Issues & Solutions

### Issue 1: bincode compile_error
**Problem**: `cargo build` fails with bincode v3.0.0 compile_error

**Solution**:
- Known issue, document in PRs
- Workflows are aware of this failure
- Focus on code correctness, note the issue

### Issue 2: FFI Linking on Windows
**Problem**: Chapel/Go/Zig/Nim FFI fails to link

**Solution**:
- Check `build.rs` for Windows-specific logic
- Ensure MSVC toolchain is available
- On Linux, pure-Rust fallbacks are used

### Issue 3: Integration Test Fails
**Problem**: `integration_real_mcp.rs` fails

**Solution**:
- Ensure server binary is built: `cargo build --release --bin nuclear-mcp`
- Check binary name (nuclear-mcp vs nuclear_ultimate)
- Run with `--test-threads=1` to avoid port conflicts
- Use `--nocapture` to see output

### Issue 4: Chapel Build Fails
**Problem**: `make full-pipeline` fails in ffi/chapel/

**Solution**:
- Verify Chapel is installed: `chpl --version`
- Set CHAPEL_HOME: `export CHAPEL_HOME=/opt/chapel`
- Check syntax: `make check`
- Build incrementally: `make train`, `make mining`, etc.

### Issue 5: 7-Tool Test Fails
**Problem**: `test_exactly_7_tools` fails

**Solution**:
- Check `src/mcp/protocol.rs` - should have exactly 7 tools
- Tool names must match: websearch, premium, file_search, scan, ai_dataset_trainer, parallel_engine, osint_intelligence
- Never add or remove tools
- Run `./scripts/validate_7_tools.sh` for details

---

## 📝 Contribution Guidelines

### When Adding Features

1. **Identify Component**: Rust/Chapel/Go/Datasets?
2. **Read Existing Code**: Understand current implementation
3. **NO MOCKS**: Use real implementations only
4. **Build & Test**: Verify in target language
5. **Integration Test**: Run cross-component tests
6. **Document**: Update docs if requested (not automatically)

### When Modifying MCP Tools

- ✅ Can enhance tool implementations
- ✅ Can add parameters to existing tools
- ✅ Can improve performance
- ❌ CANNOT add 6th tool
- ❌ CANNOT remove any tool
- ❌ CANNOT change tool names

### When Working with Chapel

- Read existing .chpl files first
- Use `make check` for syntax validation
- Build incrementally (not always full-pipeline)
- Test with `make test`
- Verify FFI integration from Rust

### When Working with Datasets

- Datasets are in models/
- Access from Chapel: `../../../models/`
- 120K+ samples available
- Mojo/Julia processors are pre-integrated
- Don't modify dataset structure without discussion

---

## 🎯 Success Criteria

A change is complete ONLY when ALL pass:

### Rust Success
- [x] `cargo build --release` succeeds (or documents bincode issue)
- [x] `cargo test test_exactly_7_tools` passes
- [x] `cargo test --lib` passes
- [x] `cargo test --test integration_real_mcp` passes
- [x] No dead code remains
- [x] `cargo fmt -- --check` passes

### Chapel Success (if applicable)
- [x] `make full-pipeline` succeeds
- [x] `make test` passes
- [x] `libchapel_ai.so` exists
- [x] FFI integration works from Rust

### Go Success (if applicable)
- [x] `go build` succeeds
- [x] `go test ./...` passes
- [x] Server runs with GITHUB_TOKEN

### Integration Success
- [x] All language components build
- [x] Cross-language FFI works
- [x] Datasets accessible
- [x] Documentation updated (if requested)
- [x] All warnings documented

---

## 🔗 Important Links

### External Repositories
- **Main Repo**: https://github.com/Rigohl/nuclear-crawler-hybrid
- **HuggingFace**: https://huggingface.co/Kimberlyindiva/nuclear-chapel-training
- **Datasets Repo**: https://github.com/Rigohl/mojo-mega-dataset-system
- **GitHub MCP**: https://github.com/modelcontextprotocol/servers

### Documentation
- `README.md` - Main project documentation
- `QUICK_START.md` - Installation and quick start
- `INTEGRATION_STATUS.md` - Integration details and statistics
- `TOOLS.md` - 7 MCP tools detailed description
- `ffi/chapel/ARCHITECTURE.md` - Chapel AI architecture
- `docs/chapel/MULTI_LANGUAGE_ML_ENGINE.md` - Multi-language guide
- `mcp-servers/README.md` - MCP server integration guide

### Configuration
- `.cursorrules` - Cursor/Copilot behavior rules
- `.github/copilot-instructions.md` - This file
- `Cargo.toml` - Rust dependencies
- `ffi/chapel/Makefile` - Chapel build system
- `mcp-servers/github/go.mod` - Go dependencies

---

## 💡 Tips for Copilot

### General Tips
- **Always read .cursorrules first** - Contains project-specific rules
- **Never create .md files** - Only update existing docs if requested
- **Show full errors** - Never filter or hide warnings
- **Use real data** - No mocks, no stubs, no simulations
- **Verify builds** - Always test after changes

### Multi-Language Tips
- **Rust First** - Primary language for MCP server
- **Chapel for ML** - Training, mining, analysis
- **Go for GitHub** - MCP server automation
- **Respect FFI boundaries** - Each language has its role

### Testing Tips
- **5-tool test is CRITICAL** - Must pass always
- **Integration tests are real** - They hit actual servers
- **Multi-threaded aware** - Use --test-threads=1 for integration
- **CI validates everything** - Check workflows for requirements

---

> **Remember**: Follow `.cursorrules` strictly. When in doubt, ask before making changes. This is a complex multi-language project with tight integration constraints.

**Last Updated**: 2026-01-24  
**Version**: 2.0 (Post-Integration)  
**Status**: ✅ All components integrated and documented
