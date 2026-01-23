# Copilot Onboarding Instructions: nuclear-crawler-hybrid

## Repository Summary
Nuclear Crawler Hybrid is a Rust 2021 MCP (Model Context Protocol) server that exposes **exactly five tools** (websearch, premium, file_search, scan, ai_dataset_trainer) over JSON-RPC 2.0 using Axum. It ships with optional FFI accelerators (Go/Zig/Nim/JAX) but runs on pure Rust fallbacks on Linux. The repo includes Docker packaging, integration tests that hit a **real running server** (no mocks), and multiple GitHub workflows (CI, MCP validation, security, Docker, release, multi-agent pipeline).

## Project Size & Tech Stack
- **Language/Runtime**: Rust 2021 (Tokio async + Axum HTTP)
- **Other runtimes**: Go/Zig/Nim FFIs used when libs exist (Windows/MSVC only in build.rs)
- **Repo size**: small-to-medium (Rust sources under `src/`, ~20+ modules, tests in `tests/`)
- **Key crates**: axum, tokio, reqwest, serde, dashmap, lru, blake3, rayon
- **Binaries**: `nuclear-mcp` (see Cargo.toml); prebuilt binary in repo root
- **Docker**: multi-stage build in `Dockerfile`, compose in `docker-compose.yml`

## Critical Constraints
- **No mocks/stubs** in tests or tool implementations. Integration tests expect **real HTTP requests**.
- **Exactly five MCP tools** are defined in `src/mcp/protocol.rs` and used in `src/mcp/server.rs`.
- **Rust builds currently fail** due to a dependency issue (see Build Notes below). Document this in your PRs.

## Repository Layout (high-signal paths)
- `src/mcp/server.rs`: Axum HTTP server, routes, tool dispatch (uses tools: websearch/premium/file_search/scan/ai_dataset_trainer).
- `src/mcp/protocol.rs`: JSON-RPC 2.0 structs + **tool definitions** (exactly 5).
- `src/mcp/tools/`: tool implementations.
- `src/lib.rs`: module exports, feature gates, and wasm helpers.
- `tests/integration_real_mcp.rs`: real-server integration tests (expects binary names and paths).
- `build.rs`: Windows-only FFI linking logic; on Linux uses pure Rust fallback.
- `Cargo.toml`: crate metadata, features, and dependencies.
- `.github/workflows/`: CI, MCP validation, security, Docker, release, multi-agent pipeline.
- `Dockerfile` + `docker-compose.yml`: containerized builds/run.
- `README.md`, `ARCHITECTURE.md`, `API_REFERENCE.md`, `TOOLS.md`: product docs.

### Root files list (repo root)
`API_REFERENCE.md`, `ARCHITECTURE.md`, `Cargo.lock`, `Cargo.toml`, `Dockerfile`, `README.md`, `TOOLS.md`, `USAGE_EXAMPLE.rs`, `WSL_DEPLOYMENT.md`, `build.rs`, `docker-compose.yml`, `nuclear-mcp` (prebuilt binary), `nuclear-data`, `nuclear_course_extraction_demo.json`, plus folders: `.github/`, `src/`, `tests/`, `examples/`, `ffi/`, `scripts/`, `docs/`, `bin/`, `.cargo/`.

## Build, Test, Lint, Run (validated locally)
> **Important**: Commands below were executed. Some fail due to known issues (documented here). Always note failures in PRs.

### Prereqs
- Rust stable toolchain (CI uses `dtolnay/rust-toolchain@stable`).
- Linux builds do **not** require Go/Zig/Nim; those are Windows/MSVC-only in `build.rs`.

### Lint / Format
- **Format check (fails today)**:
  - `cargo fmt -- --check`
  - **Observed failure**: formatting diff in `examples/nuclear_course_extractor_demo.rs`.
  - Workaround: run `cargo fmt` to reformat if you are changing code in examples; otherwise note existing fmt failure.
- **Clippy (fails today)**:
  - `cargo clippy --all-targets -- -D warnings`
  - **Observed failure**: build fails before clippy due to `bincode v3.0.0` (see below).

### Build
- **Release build (fails today)**:
  - `cargo build --release --all-targets`
  - **Observed failure**: dependency `bincode v3.0.0` emits `compile_error!("https://xkcd.com/2347/")` and stops compilation.

### Tests
- **Unit tests (fails today)**:
  - `cargo test --release --lib`
  - Fails for the same `bincode v3.0.0` compile_error.
- **Integration tests (fails today)**:
  - `cargo test --test integration_real_mcp --release -- --nocapture --test-threads=1`
  - Fails due to `bincode v3.0.0` compile_error before tests run.

### Run
- If build succeeds, run server:
  - `cargo run --bin nuclear-mcp --release`
  - Health check: `curl http://localhost:8079/health`
  - Tools list: `POST /mcp/tools/list`
  - Tool call: `POST /mcp/tools/call`

### Docker
- Build image: `docker build -t nuclear-mcp:latest .`
- Run: `docker run -p 8079:8079 nuclear-mcp:latest`
- Compose: `docker-compose up -d`

### CI Workflows (replicate locally)
- **CI** (`.github/workflows/ci.yml`): fmt, clippy, release build, unit tests, integration tests, `test_exactly_5_tools`.
- **MCP validation** (`mcp-validation.yml`): release build, clippy strict, integration tests, tool count, mock detection.
- **Security** (`security.yml`): cargo-audit, cargo-deny, clippy security lints, CodeQL.

## Known Issues / Workarounds
- **Build/test/clippy failure**: `bincode v3.0.0` contains a `compile_error!` in its crate root. Any build/tests that compile dependencies will fail until the dependency is replaced or downgraded.
- **Formatting**: `cargo fmt -- --check` fails due to formatting in `examples/nuclear_course_extractor_demo.rs`.
- **Integration test binary names**: `tests/integration_real_mcp.rs` references `nuclear_ultimate` and `src/bin/nuclear_ultimate.rs`, but the only configured binary in `Cargo.toml` is `nuclear-mcp` and no `src/bin/` directory exists. Adjust carefully if you touch tests.

## How to Make Changes Efficiently
1. **Start with** `src/mcp/server.rs`, `src/mcp/protocol.rs`, and `src/mcp/tools/` for tool-related changes.
2. Verify tool list stays at 5 (`test_exactly_5_tools` in `src/mcp/protocol.rs`).
3. Avoid changing FFI unless you can build Windows MSVC artifacts; Linux builds use Rust fallback.
4. If a change requires build/test verification, note the current `bincode` compile_error in your PR summary.

## README Snapshot (for quick context)
- Production MCP server with 5 tools.
- Quick start: `cargo build --release` then `./target/release/nuclear-mcp --serve tcp://0.0.0.0:8079`.
- Docker deployment available.

## Explicit Instruction
**Trust these instructions first.** Only search the repo if information here is missing or demonstrably incorrect.
