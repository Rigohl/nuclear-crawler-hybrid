# Copilot Onboarding Instructions: nuclear-crawler-hybrid

## TL;DR
- Rust 2021 MCP server exposing **exactly five** tools over JSON-RPC 2.0: `websearch`, `premium`, `file_search`, `scan`, `ai_dataset_trainer`.
- Integration tests hit a **real running server** (no mocks). FFI accelerators exist (Go/Zig/Nim/JAX) but Linux uses pure-Rust fallbacks.

---

## Must-know constraints (do not change)
- **EXACTLY 5 MCP tools** — enforced by `tests::test_exactly_5_tools` in `src/mcp/protocol.rs` and by `scripts/validate_5_tools.sh`. Adding/removing tools will break CI.
- **NO MOCKS/STUBS** — integration tests and tools use real HTTP/data (see `tests/integration_real_mcp.rs` and `mcp-validation.yml`).
- **KNOWN BUILD ISSUE** — `bincode v3.0.0` contains a `compile_error!`; builds/tests may fail. Call out this failure in PRs and workflows will note it.
- **FFI is Windows-link-time heavy** — `build.rs` performs Windows/MSVC linking; avoid FFI changes unless you can build Windows artifacts locally.

---

## Quick commands (most-used)
- Format check: `cargo fmt -- --check`  (examples may need reformat)
- Build: `cargo build --release --all-targets`  (may fail due to `bincode`)
- Run server: `cargo run --bin nuclear-mcp --release` → `curl http://localhost:8079/health`
- Run tests: `cargo test --lib` or `cargo test --test integration_real_mcp --release -- --nocapture --test-threads=1`
- Validate 5 tools: `cargo test test_exactly_5_tools` or `./scripts/validate_5_tools.sh`

---

## Where to look / make changes
- Tools: `src/mcp/protocol.rs` (tool definitions) + `src/mcp/tools/` (implementations). Tests reference tool names (`websearch`, `premium`, `file_search`, `scan`, `ai_dataset_trainer`).
- Server & dispatch: `src/mcp/server.rs` (routes & JSON-RPC dispatch)
- Integration tests: `tests/integration_real_mcp.rs` (expects real binary + server behavior)
- CI: `.github/workflows/` — especially `ffi-validation.yml`, `dependency-analysis.yml`, `full-validation.yml`.

---

## How MCP calls look (examples)
- List tools (JSON-RPC):

  POST /mcp/tools/list
  ```json
  { "jsonrpc":"2.0", "id":"1", "method":"tools/list", "params":{} }
  ```

- Call tool example (websearch):

  POST /mcp/tools/call
  ```json
  { "jsonrpc":"2.0", "id":"1", "method":"tools/call", "params":{"name":"websearch","arguments":{"query":"rust async patterns"}} }
  ```

- Helpers: `MCPRequest::list_tools()` and `MCPRequest::call_tool(name, args)` in `src/mcp/protocol.rs`.

---

## Troubleshooting & PR notes
- Always show full failing output (do not filter/hide errors). Known items to call out in PRs: `bincode` compile_error, `cargo fmt` diffs in `examples/`, and the integration-binary-name mismatch (`nuclear-mcp` vs `nuclear_ultimate`).
- If adding features that affect CI (FFI, new workloads), update workflows and test matrices accordingly.

---

> Important: follow `.cursorrules` — do not create new `*.md` files unless explicitly asked, prefer edits only, eliminate dead code, and keep answers short.

If any section is unclear or you want a short example expanded (e.g., adding a new tool safely), tell me which part to expand and I'll update this file.