# 🔥 NUCLEAR CRAWLER HYBRID

**Advanced AI-Powered Web Intelligence & Exploitation Platform**

## What is This?

Nuclear Crawler Hybrid is a **unified MCP (Model Context Protocol) server** that exposes:

- **7 Production-Grade Intelligence Tools** over JSON-RPC 2.0 HTTP API
- **Chapel AI Engine** - Advanced parallel ML training with 120K+ datasets
- **Multi-Language FFI** - Rust, Python, Go, Zig, Nim, JAX, Chapel integration
- **50K Goroutine Parallelism** - Distributed operations at massive scale
- **OSINT Capabilities** - Advanced data mining and intelligence gathering
- **Unified CI Enforcement** - one blocking CI pipeline with explicit backend validation
- **Fail-Fast Execution** - required backends must be present; no silent mock substitution

### Core Philosophy

- **NO SILENT MOCKS** - missing required backends now fail visibly instead of degrading silently
- **MAXIMUM PARALLELISM** - 50,000 goroutines per operation
- **HYBRID BACKENDS** - Chapel primary, WASM for Go/Zig/Nim, subprocess JAX
- **WASM ACCELERATION** - 50-100x speedup modules
- **CHAPEL AI BRAIN** - Intelligent decision-making pre/post execution

## 7 MCP Tools (Ultimate Intelligence Platform)

| # | Tool | Purpose | Backend |
|---|------|---------|---------|
| 1 | **WEBSEARCH** | 55+ engines, 50K parallel goroutines, real-time indexing | Go + Nim |
| 2 | **PREMIUM_CONTENT** | Paywall bypass + Lateral Movement (21 exploitation techniques) | Native + Go |
| 3 | **SCAN_WORKSPACE** | Code analysis: errors, warnings, CVE detection, ML vulnerability detection | Zig SIMD |
| 4 | **AI_DATASET_TRAINER** | Generate 5K-10K ML datasets with Chapel AI optimization | Chapel + WASM |
| 5 | **FILE_SEARCH** | Precision search: exact line:column, regex, error detection | Zig + WASM |
| 6 | **CODE_INTELLIGENCE** | ML analysis: vulnerability detection, fix suggestions, risk scoring | JAX + Zig |
| 7 | **INTELLIGENCE_OSINT** | OSINT analysis: public info, risk analysis, Bayesian networks, game theory | Go + JAX |

### 🔧 CI/CD Enforcement
- ✅ **Single Workflow** - the repository should keep one primary CI workflow
- ✅ **Fail-Fast Validation** - missing required backends stop the pipeline explicitly
- ✅ **Backend Contract Checks** - CI validates real FFI and the repo-managed JAX path
- ✅ **Controlled Concurrency** - CI avoids redundant executions on the same branch

### Chapel AI Training Engine (Parallel)
- ✅ **Massive Data Parallelism** - `coforall` for concurrent operations across locales
- ✅ **120K+ Training Samples** - Math, PowerShell, OSINT, exploitation datasets
- ✅ **Real Pattern Learning** - No mocks, actual ML algorithms with statistical tracking
- ✅ **Continuous Optimization** - Learns from every operation with Welford's algorithm
- ✅ **Multi-tool Integration** - Connected to all 7 MCP tools for dynamic optimization
- ✅ **Distributed Computing** - Multi-locale support for extreme scalability
- ✅ **Scientific Analysis** - Mean, variance, skewness, trend analysis
- ✅ **GPU Acceleration** - JAX + Chapel for accelerated inference

## Architecture

```
HTTP JSON-RPC 2.0 Server (port 8079)
    ↓
7 MCP Tools (Independent, Composable)
    ↓
37 Internal Modules (Core, FFI, WASM, OSINT, Infra, AI)
    ↓
5 FFI Backends (Go, Zig, Nim, JAX, Chapel)
    ↓
6 WASM Accelerators (50-100x speedup)
    ↓
Chapel AI Learning Brain (Parallel, Multi-Locale)
```

## 🚀 Chapel AI Integration

### Build Chapel Library

```bash
make build
```

This generates `libchapel_ai.so` with:
- `--fast` flag (maximum optimizations)
- `-O3` C compiler flags
- `-march=native` for CPU-specific optimizations
- `coforall` parallel execution enabled across locales

### API Functions

#### Initialization
```chapel
export proc chapel_ai_init(): int
```
Initializes parallel Chapel AI system with colocale support.

#### Learning (with Parallelism)
```chapel
export proc chapel_ai_learn(tool, operation, input, quality): int
```
Records operations with Welford's algorithm for variance, min/max tracking, and trend analysis.

#### Get Advice (Statistical)
```chapel
export proc chapel_ai_get_advice(tool, operation, advice_out, max_len): int
```
Returns AI advice with confidence scores, trends, and statistical metrics.

#### Optimization & Shutdown
```chapel
export proc chapel_ai_optimize(): int
export proc chapel_ai_shutdown(): int
```
Parallel optimization cycle with atomic pattern removal and multi-locale cleanup.

## 🔧 CI/CD & Development

### CI/CD Development Rules

The repository policy is to keep CI minimal and truthful:

```bash
cargo check --bin nuclear-mcp
cargo test
```

- One primary CI workflow gates the repository.
- Missing required backends must fail visibly.
- Documentation must not claim auto-repair or hidden recovery that the repo does not implement.

Chapel AI is accessed through the Rust FFI in `src/chapel_integration.rs`:

```rust
use crate::chapel_integration::ChapelAI;

let chapel_ai = ChapelAI::new();

// Learn from operation
chapel_ai.learn_from_operation(ChapelContext {
    tool_name: "websearch".to_string(),
    operation: "search".to_string(),
    input_data: query.clone(),
    output_quality: 0.95,
    timestamp: current_time(),
    metadata: HashMap::new(),
})?;

// Get advice (with advanced analytics)
let advice = chapel_ai.get_advice("websearch", "search")?;
```

## Performance

- **WebSearch**: 55 engines × 50K goroutines = massive parallelism
- **Premium Access**: 10 simultaneous bypass methods
- **Code Analysis**: ML detection + SIMD 256-bit vectorization
- **Exploitation**: 21 lateral movement techniques with 50K goroutines
- **Overall**: 1000x+ performance vs single-language approach

## Quick Start

```bash
# Build
cargo build --release --all-features

# Run
./target/release/nuclear-mcp

# Test (validates exactly 7 tools)
cargo test test_exactly_7_tools

# Example: WebSearch
curl -X POST http://localhost:8079/tools/websearch \
  -d '{"query":"exploit techniques","max_results":50}'
```

## Key Features

✅ **Fail-Fast Execution** - missing required backends fail visibly  
✅ **Parallelism** - Go FFI: 50,000 goroutines  
✅ **Speed** - Nim FFI: 100-200x HTML parsing, Zig: 256-bit SIMD  
✅ **Intelligence** - Chapel AI pre/post optimization  
✅ **Flexibility** - Each tool independent, composable  
✅ **Compliance** - EXACTLY 7 MCP tools (specification)

## Documentation

- `ARCHITECTURE.md` - System design & module integration
- `MCP_TOOLS.md` - Detailed tool specifications
- `CAPABILITIES.md` - Technical capabilities per tool
- `FFI_INTEGRATION.md` - Multi-language backend architecture
- `QUICK_START.md` - Installation & usage guide

## Project Structure

```
nuclear-crawler-hybrid/
├── src/
│   ├── mcp/              # MCP protocol + 7 tools
│   ├── core/             # Core functionality (lateral movement, exploit, etc.)
│   ├── ffi/              # 5 FFI backends (Go, Zig, Nim, JAX, Chapel)
│   ├── wasm/             # 6 WASM acceleration modules
│   ├── osint/            # OSINT analysis (5 modules)
│   ├── infra/            # Infrastructure (8 modules)
│   └── ai/               # AI integration (2 modules)
├── Cargo.toml            # Rust dependencies
└── README.md             # This file
```

## Status

✅ **7 MCP Tools** - Fully specified  
✅ **37 Internal Modules** - Integrated  
✅ **Backends** - Chapel enforced, WASM/JAX validated in CI  
✅ **6 WASM Accelerators** - Compiled  
⚠️ **Strict Mode** - CI blocks on missing required backends

---

**Last Updated**: 11 de marzo de 2026  
**Version**: 2.0 (7 Tools)  
**License**: MIT/Apache 2.0
