# Testing Guide - Nuclear Crawler Hybrid

## Overview

This document explains the test infrastructure, preconditions for running tests, and why certain tests are marked `#[ignore]`.

## Running Tests

```bash
# Run all library unit tests (fast, no external dependencies)
cargo test --lib

# Run all tests including integration
cargo test --verbose

# Run with all features enabled
cargo test --all-features

# Run without default features (core-only)
cargo test --no-default-features

# Run a specific test
cargo test test_exactly_7_tools

# Run integration tests (requires running server)
cargo test --test integration_real_mcp --release -- --nocapture --test-threads=1
```

## Test Categories

### Unit Tests (`cargo test --lib`)

Located inline in source files with `#[cfg(test)]`. No external dependencies required.

**Examples:**
- `src/infra/deepweb_tor.rs::tests::test_initialization` — verifies TOR config defaults
- `src/mcp/protocol.rs::tests::test_exactly_7_tools` — validates the 7 MCP tool definitions

### Integration Tests (`tests/`)

#### `tests/integration_real_mcp.rs`

Real integration tests that launch the MCP server and exercise all 7 tools over HTTP.

**Preconditions:**
- The `nuclear-mcp` binary must be built: `cargo build --release --bin nuclear-mcp`
- Port 8079 must be free (default MCP port)
- Run with `--test-threads=1` to avoid port conflicts between tests

```bash
cargo test --test integration_real_mcp --release -- --nocapture --test-threads=1
```

#### `tests/chapel_ffi_integration.rs`

Tests Chapel AI FFI integration from Rust.

**Preconditions:**
- Chapel must be installed (see [FFI_ARCHITECTURE.md](src/FFI_ARCHITECTURE.md))
- `libchapel_ai.so` must be built: `cd ffi/chapel && make full-pipeline`
- Environment: `CHAPEL_HOME` must point to Chapel installation

```bash
# Build Chapel library first
cd ffi/chapel && make full-pipeline
# Then run tests
cargo test --test chapel_ffi_integration
```

#### `tests/wasm_integration_tests.rs`

Tests WASM FFI operations.

**Preconditions:**
- `wasm_ffi` feature must be enabled (default)
- WASM modules must be built: `cd ffi/wasm && ./build_wasm.sh`

```bash
cargo test --test wasm_integration_tests --features wasm_ffi
```

## Ignored Tests

Tests marked `#[ignore]` require specific runtime environments. Each ignored test documents its requirements:

```rust
#[test]
#[ignore] // Requires: TOR daemon running (tor service), internet connectivity, 60s timeout
fn test_deepweb_tor_search() { ... }

#[test]
#[ignore] // Requires: INTERNET_AVAILABLE=1, external network access
fn test_live_websearch() { ... }

#[test]
#[ignore] // Requires: JAX_MODELS_PATH env var, GPU or CPU JAX installation
fn test_jax_model_inference() { ... }
```

To run ignored tests:
```bash
# Run a specific ignored test
cargo test test_deepweb_tor_search -- --ignored

# Run all ignored tests (requires all preconditions)
cargo test -- --ignored
```

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `INTERNET_AVAILABLE` | For net tests | Set to `1` to enable network-dependent tests |
| `TOR_AVAILABLE` | For TOR tests | Set to `1` when TOR daemon is running on 127.0.0.1:9050 |
| `JAX_MODELS_PATH` | For JAX tests | Path to pre-trained JAX model files |
| `CHAPEL_HOME` | For Chapel FFI | Path to Chapel installation directory |
| `HF_TOKEN` | For HF tests | HuggingFace API token for model downloads |

## CI Test Validation

The CI pipeline (`.github/workflows/ci.yml`) runs:
1. **validate** — `cargo fmt --check` + `cargo clippy -D warnings`
2. **build-test** — `cargo build --release` + `cargo test --verbose`
3. **chapel-ci** — Chapel syntax check (`make check`)
4. **go-ci** — Go build and test (`go test ./...`)
5. **zig-ci** — Zig SIMD library build
6. **nim-ci** — Nim HTML parser compilation
7. **jax-ci** — JAX import and computation validation
8. **ffi-integration** — Cross-language integration summary
