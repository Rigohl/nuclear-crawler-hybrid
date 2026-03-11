# Contributing to Nuclear Crawler Hybrid

Thank you for contributing! This guide covers the project structure, coding standards, and how to add new FFI backends.

## Quick Start

```bash
# Clone and build
git clone https://github.com/Rigohl/nuclear-crawler-hybrid
cd nuclear-crawler-hybrid

# Verify your setup
./scripts/verify_setup.sh

# Build Rust core
cargo build --release

# Run tests
cargo test --lib
```

## Project Architecture

- **`src/`** — Rust MCP server (primary language)
- **`ffi/chapel/`** — Chapel AI training engine
- **`ffi/wasm/go/`** — Go WASM module (goroutines)
- **`ffi/wasm/zig/`** — Zig WASM module (SIMD)
- **`ffi/wasm/nim/`** — Nim WASM module (HTML parsing)
- **`mcp-servers/github/`** — Go GitHub MCP server
- **`tests/`** — Integration tests
- **`scripts/`** — Build and validation scripts

## Code Standards

### Rust

- Edition 2021, stable toolchain
- `cargo fmt` before every commit
- `cargo clippy --all-targets -- -D warnings` must pass with zero warnings
- No `panic!()` in production code paths — use `Result<T, E>` instead
- No mocks in integration tests — use real implementations

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --lib
```

### Feature Flags

The crate exposes the following features:

| Feature | Description |
|---------|-------------|
| `core` | Core functionality (enabled by default) |
| `wasm_ffi` | WebAssembly runtime via wasmtime (enabled by default) |
| `premium` | Premium content extraction + advanced analytics |
| `jax` | JAX GPU acceleration bindings |
| `advanced-analytics` | Advanced ML analytics |
| `chapel_ffi` | Chapel AI dynamic library |
| `go_integration` | Go FFI (Windows only) |
| `zig_integration` | Zig SIMD FFI (Windows only) |
| `nim_integration` | Nim HTML parser FFI (Windows only) |
| `chromium_rendering` | Headless Chromium via chromiumoxide |
| `advanced_tor` | Enhanced TOR/I2P routing |

Use `#[cfg(feature = "premium")]` gates for premium-only code paths.

### No Panics Policy

Production code must never call `panic!()`. Use proper error types:

```rust
// ❌ Bad
fn connect() -> Connection {
    todo!()  // or panic!("not implemented")
}

// ✅ Good
fn connect() -> Result<Connection, MyError> {
    Err(MyError::NotImplemented)
}
```

## Adding an FFI Backend

### Overview

FFI backends integrate external language runtimes into the Rust MCP server. The 5 current backends are:

| Backend | Language | Platform | Purpose |
|---------|----------|----------|---------|
| Go | Go 1.21+ | Windows + WASM | Parallel HTTP goroutines |
| Zig | Zig 0.12+ | Windows + WASM | SIMD operations |
| Nim | Nim 1.6+ | Windows + WASM | HTML parsing |
| Chapel | Chapel 2.0+ | Linux/macOS | AI training |
| JAX | Python 3.11+ | Cross-platform | GPU acceleration |

### Adding a New Backend

1. **Create the FFI source** in `ffi/<name>/` or `ffi/wasm/<name>/`

2. **Add build detection in `build.rs`**:
   ```rust
   let mylib_path = format!("{}/ffi/mylib/libmylib.so", manifest_dir);
   if std::path::Path::new(&mylib_path).exists() {
       println!("cargo:rustc-link-lib=dylib=mylib");
       println!("cargo:rustc-cfg=has_mylib");
   }
   ```

3. **Add feature flag in `Cargo.toml`**:
   ```toml
   [features]
   my_backend = []
   ```

4. **Create Rust integration module** in `src/ffi/my_integration.rs`

5. **Add CI job** in `.github/workflows/ci.yml`:
   ```yaml
   my-backend-ci:
     name: 🔧 My Backend CI
     needs: validate
     runs-on: ubuntu-latest
     continue-on-error: true
     steps:
       - uses: actions/checkout@v4
       - name: Setup
         run: # install your language toolchain
       - name: Build
         run: # compile your FFI library
   ```

6. **Add backend documentation** to `src/FFI_ARCHITECTURE.md`

### Build Commands by Backend

```bash
# Go WASM
cd ffi/wasm/go && GOOS=wasip1 GOARCH=wasm go build -o nuclear_go.wasm .

# Zig WASM
cd ffi/wasm/zig && zig build-lib -target wasm32-wasi -O ReleaseFast -dynamic main.zig

# Nim WASM
cd ffi/wasm/nim && nim c -d:release -o:nuclear_nim_parser main.nim

# Chapel (requires Chapel installation)
cd ffi/chapel && make full-pipeline

# Go native (Windows only)
cd ffi/go && go build -buildmode=c-archive -o stealth_go.lib .
```

## MCP Tools

The server exposes **exactly 7 MCP tools**. Do not add or remove tools:

1. `websearch` — Web search across 55+ engines
2. `premium` — Premium content extraction
3. `file_search` — File and code search
4. `scan` — Workspace code analysis
5. `ai_dataset_trainer` — ML training dataset generation
6. `parallel_engine` — Parallel processing (Go+Zig+Chapel)
7. `osint_intelligence` — OSINT intelligence gathering

The tool count is validated by `test_exactly_7_tools` in `src/mcp/protocol.rs`.

## Pull Request Checklist

- [ ] `cargo fmt -- --check` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes (0 warnings)
- [ ] `cargo test --lib` passes
- [ ] No new `panic!()` calls in production code
- [ ] No mocks in integration tests
- [ ] Feature flags added for optional capabilities
- [ ] Documentation updated if needed
- [ ] `./scripts/verify_setup.sh` runs without errors
