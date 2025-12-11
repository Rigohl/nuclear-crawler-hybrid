# Copilot Instructions for Nuclear Crawler Hybrid

## Project Overview

**NUCLEAR CRAWLER HYBRID** is a high-performance Rust MCP (Model Context Protocol 2025-06-18) server providing massively parallel web search and analysis. It implements 4 primary tools and 23+ integrated modules with FFI integration (Go, Zig, Nim) for extreme performance.

**Key Stats**: 100K goroutines, 55 search sources, 2-second search completion, 2,100+ URLs per query.

---

## Architecture & Components

### Core MCP Server (`src/bin/nuclear_ultimate.rs`)
- **Dual Protocol**: HTTP (Axum on :8080) + STDIO (for Claude/MCP clients)
- **4 Main Tools**: websearch, file_search, analyzer, stats
- **Async Runtime**: Tokio with full feature set
- **Main struct** `SearchEngine` holds all 23 modules as fields

### 23 Integrated Modules (Reference `src/lib.rs`)
```
Core: WebSearch, RealSearchEngines, DeepWeb, MassiveParallel, ParallelCrawler
FFI: GoIntegration, ZigIntegration, NimIntegration, JaxPipeline, MojoProcessor
Bypass: NuclearBypass, NuclearScraper, StealthSystem
Infrastructure: IntelligentStorage (SQLite), Cache, RateLimiter, CircuitBreaker
Utilities: HfIntegration, ProjectAnalyzer, Stats, Utils
```

### FFI Integration Pattern
- **Go FFI** (`go/src/stealth_go.go`): 100K parallel goroutines, stealth headers
- **Zig SIMD** (`zig/src/lib.zig`): Fast hashing, SIMD parsing via `libloading`
- **Nim HTML** (`src/lib.rs` wrapper): Alternative HTML parsing
- **JAX Acceleration** (`scripts/jax_pipeline.py`): Vectorized batch processing

---

## Critical Patterns & Conventions

### Error Handling
- **Return Type**: `Result<Value>` where Value = `serde_json::Value`
- **Never propagate .unwrap()** in tool handlers - use pattern matching or `.ok()`
- **Example**: `src/core_tools.rs` lines 352-394 show correct tool error handling

### Tool Implementation (4 Required Patterns)

**1. WebSearch** (`tool_websearch()`):
- Input: `{"queries": ["term1", "term2"], ...}` (max 5 queries)
- Returns 2,100+ URLs from 55 sources in parallel
- Uses rate limiter + cache checks before execution
- **Key Files**: `src/web_search.rs`, `src/massive_parallel_search.rs`

**2. FileSearch** (`tool_file_search()`):
- Input: `{"search_term": "pattern", "path": "./src"}` 
- Uses Zig SIMD for ultra-fast pattern matching
- Returns results with line numbers and context
- **Key Files**: `src/file_search.rs`

**3. Analyzer** (`tool_analyzer()`) - **CRITICAL**:
- **MUST use LOCAL file analysis only** - NO external commands (cargo, pylint, etc.)
- Input: `{"path": "."}`
- Returns: `{status, summary, files_sample, modules_analyzed}`
- **Implemented pattern**: Recursive `fs::read_dir()` with extension-based classification
- **Key Files**: Lines 1158-1280 in `src/bin/nuclear_ultimate.rs`

**4. Stats** (`tool_stats()`):
- Returns system state: module availability, request counts, storage stats
- No input arguments
- **Key Files**: `src/stats.rs`

### Tokio Async Patterns
- **All network I/O**: Use `tokio::spawn()` for concurrent tasks
- **Timeouts**: Always wrap external calls with `tokio::time::timeout(Duration::from_secs(N), ...)`
- **Rate Limiting**: Call `self.rate_limiter.acquire().await` before bulk operations
- **Example**: Lines 580-630 in nuclear_ultimate.rs show proper timeout + rate limit usage

### Result Building Pattern
```rust
let result = json!({
    "status": "success",
    "count": result_count,
    "data": serializable_data,
    "execution_ms": start.elapsed().as_millis(),
    "modules_used": module_count,
});
Ok(result)
```

---

## Build & Deployment

### Compilation
```bash
# Development (faster, unoptimized)
cargo build

# Release (optimized: 3x opt-level, LTO, single codegen unit)
cargo build --release
```

**Profile Config** (`Cargo.toml [profile.release]`):
- `opt-level = 3` + `lto = "fat"` + `codegen-units = 1`
- Binary location: `target/release/nuclear-mcp.exe` (~20-25MB)

### Execution Modes
```bash
# HTTP Server (localhost:8080)
./target/release/nuclear-mcp --mode http --port 8080

# STDIO (MCP protocol for Claude)
./target/release/nuclear-mcp --mode studio
```

---

## Common Development Tasks

### Running Single Tool for Testing
```bash
# After cargo build, use curl for HTTP mode:
curl -X POST http://localhost:8080/call \
  -H "Content-Type: application/json" \
  -d '{"name": "websearch", "arguments": {"queries": ["rust async"]}}'
```

### Adding a New Module
1. Create `src/module_name.rs` with public struct implementing core trait
2. Add field to `SearchEngine` struct in `nuclear_ultimate.rs`
3. Initialize in `SearchEngine::new()` (line ~300)
4. Reference in appropriate tool handler

### Fixing Compilation Errors
- **Brace mismatches**: Check lines around error - Rust's error reporting points to where mismatch is detected, not where cause is
- **FFI issues**: Verify `libloading` can find `.dll`/`.so` files - check `path` in FFI struct constructors
- **Timeout errors**: Increase timeout duration in `tokio::time::timeout(Duration::from_secs(X), ...)`

---

## Key File Reference

| File | Purpose | Key Lines |
|------|---------|-----------|
| `src/lib.rs` | Module declarations + SearchEngine struct | 1-150 |
| `src/bin/nuclear_ultimate.rs` | MCP server + 4 tool handlers | 300-1800 |
| `src/core_tools.rs` | HTTP endpoint tool dispatchers | 100-450 |
| `src/simple_mcp.rs` | STDIO MCP protocol handler | 1000-1200 |
| `Cargo.toml` | Metadata: tools, modules, FFI config | [package.metadata.*] |
| `src/web_search.rs` | Core search implementation | - |
| `src/file_search.rs` | Local file pattern search (Zig SIMD) | - |
| `go/src/stealth_go.go` | Go parallelism via FFI | - |
| `zig/src/lib.zig` | SIMD hash + parsing | - |

---

## Rust Development Standards

### Fundamental Rules
- **NEVER use mocks or simulations** - all code must be real and functional
- **NEVER assume dependencies** - read `Cargo.toml` for actual versions and features
- **ALWAYS verify with real compilation** - use `cargo check --all-targets 2>&1`
- **ALWAYS search for connected files** before changing code
- **ALWAYS handle Result and Option** explicitly - no unwrap() without justification

### Before Any Code Change
1. Run `cargo check --all-targets 2>&1` to verify current state
2. Read `Cargo.toml` for actual versions and available features
3. Use workspace search to find ALL usages of affected code
4. Verify changes don't break other files

### Code Style
- `snake_case` for functions and variables
- `PascalCase` for types, traits, enums
- `SCREAMING_SNAKE_CASE` for constants
- Prefer `&str` over `String` in parameters
- Prefer `impl Trait` over `dyn Trait` when possible
- **Maximum 100 characters per line**

### Error Handling
- Use `Result<T, E>` for fallible operations
- Use `thiserror` for custom errors in libraries
- Use `anyhow` in applications, `thiserror` in libraries
- **NEVER use `.unwrap()` without explicit justification**
- **NEVER use `.expect()` without descriptive message**
- Propagate errors with `?` when appropriate

### Ownership & Borrowing
- Prefer borrowing (`&T`, `&mut T`) over cloning
- Use `Clone` only when necessary
- Avoid unnecessary lifetime annotations
- Understand why compiler requests lifetimes before adding them

### Concurrency (Tokio)
- Prefer channels (`mpsc`) over shared mutexes
- Use `Arc<Mutex<T>>` only when necessary
- Always verify Tokio version and features in `Cargo.toml`
- Never mix async runtimes

### FFI Integration (Go, Zig, Nim)
- `extern "C"` must match exactly with external language definitions
- Use C-compatible types: `i32`, `u32`, `f32`, `*const`, `*mut`
- **Document all unsafe code thoroughly**
- Encapsulate unsafe in safe abstractions
- FFI can be optimized for any language when performance-critical

### Testing
- Use real data, never mock data
- Descriptive names: `test_should_X_when_Y`
- One assert per test when possible
- Integration tests for complete workflows

### Dependencies
- Read `Cargo.toml` before proposing code changes
- Use API compatible with the version the project has
- Don't add dependencies without justification
- Verify feature compatibility

### When Errors Occur
1. Read the complete compiler error message
2. Identify exact file and line number
3. Search what other files use this code
4. Propose minimal fix that doesn't break anything
5. Verify with `cargo check`

---

## Prohibited Practices

❌ **ABSOLUTELY FORBIDDEN**:
- Mocks of any kind
- Simulated or hardcoded test data
- Ignoring compiler warnings - **ALL warnings must be fixed**
- **ZERO dead code or stubs allowed** - all code must be functional
- Unsafe code without documentation
- `.unwrap()` without justification
- `.expect()` without clear error message
- Assuming crate versions - read `Cargo.toml` first

---

## Project-Specific Gotchas & Anti-Patterns

❌ **DO NOT**:
- Call external commands in `analyzer` (was main bug Dec 5-11) - use filesystem APIs instead
- Unwrap Results in tool handlers - wrap in `json!({"error": msg})`
- Block async code with `.block_on()` - use `.await`
- Ignore rate limits - always acquire before bulk ops

✅ **DO**:
- Use `eprintln!()` for debug output (redirects to stderr in STDIO mode)
- Cache results in `memory_cache: DashMap` before returning
- Apply Stealth headers from `self.stealth_system.get_headers()` to requests
- Parallelize with Tokio, not manual thread spawning
