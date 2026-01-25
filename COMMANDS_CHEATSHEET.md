# ⚡ Commands Cheatsheet - Nuclear Crawler Hybrid

Comandos de una línea para copiar/pegar. **Usa Cmd/Ctrl+F para buscar rápido.**

---

## 🔨 BUILD

```bash
# Build ALL (Rust + Chapel + Go)
cargo build --release && cd ffi/chapel && make full-pipeline && cd ../../mcp-servers/github && go build

# Build Rust MCP only
cargo build --release

# Build Chapel AI only
cd ffi/chapel && make full-pipeline

# Build Go GitHub MCP only
cd mcp-servers/github && go build

# Build specific Chapel system
cd ffi/chapel && make train          # Training only
cd ffi/chapel && make mining         # Mining only
cd ffi/chapel && make science        # Analysis only
cd ffi/chapel && make unified        # Unified AI only
```

---

## ✅ TEST

```bash
# ⚠️ CRITICAL: Validate 5 MCP tools
cargo test test_exactly_5_tools

# Rust unit tests
cargo test --lib

# Rust integration tests (real server)
cargo test --test integration_real_mcp --release -- --nocapture --test-threads=1

# Chapel syntax check
cd ffi/chapel && make check

# Chapel tests
cd ffi/chapel && make test

# Go tests
cd mcp-servers/github && go test ./...

# Go static analysis
cd mcp-servers/github && go vet ./...
```

---

## 🚀 RUN

```bash
# Run Rust MCP Server (port 8079)
cargo run --bin nuclear-mcp --release

# Run Chapel Training
cd ffi/chapel && make train && make run

# Run Chapel Data Mining
cd ffi/chapel && make mine

# Run Chapel Scientific Analysis
cd ffi/chapel && make analyze

# Run Chapel Unified AI
cd ffi/chapel && make run-unified

# Run ALL 8 Chapel Systems
cd ffi/chapel && make execute-all

# Run Go GitHub MCP (needs GITHUB_TOKEN)
export GITHUB_TOKEN="ghp_xxxxx" && cd mcp-servers/github && ./github-mcp-server
```

---

## 🧹 CLEAN

```bash
# Clean ALL
cargo clean && cd ffi/chapel && make clean

# Clean Rust only
cargo clean

# Clean Chapel only
cd ffi/chapel && make clean

# Clean Go only
cd mcp-servers/github && go clean
```

---

## 📝 FORMAT

```bash
# Format Rust
cargo fmt

# Check Rust format (no changes)
cargo fmt -- --check

# Format Go
cd mcp-servers/github && go fmt ./...

# Format ALL
cargo fmt && cd mcp-servers/github && go fmt ./...
```

---

## 🔍 ANALYZE

```bash
# Rust clippy
cargo clippy

# Rust dependency tree
cargo tree

# Rust check (fast compile check)
cargo check

# Chapel help
cd ffi/chapel && make help

# Go vet
cd mcp-servers/github && go vet ./...
```

---

## 📊 INFO

```bash
# Project structure
tree -L 2 -I 'target|node_modules|.git'

# Rust crates
cargo tree --depth 1

# Chapel targets
cd ffi/chapel && make help

# Go modules
cd mcp-servers/github && go list -m all

# Dataset count
find models/ -name "*.jsonl" -o -name "*.json" | wc -l

# Lines of code (Rust)
find src/ -name "*.rs" -exec wc -l {} + | tail -1

# Lines of code (Chapel)
find ffi/chapel/ -name "*.chpl" -exec wc -l {} + | tail -1
```

---

## 🎯 SHORTCUTS (En Cursor Tasks)

```
Cmd/Ctrl + Shift + P → "Tasks: Run Task"

Busca:
- 🔨 Build All
- ✅ Validate 5 MCP Tools
- 🚀 Run MCP Server
- 🧠 Train Chapel AI
- ⚡ Run All Chapel Systems
- 🔬 Data Mining
- 📊 Scientific Analysis
- 🧪 Integration Tests
- 🧹 Clean All
- 📝 Format Code
```

---

## 🐛 DEBUG

```bash
# Rust with backtrace
RUST_BACKTRACE=1 cargo run

# Rust with full backtrace
RUST_BACKTRACE=full cargo test

# Rust with logs
RUST_LOG=debug cargo run

# Chapel with debug
cd ffi/chapel && make debug

# Chapel with profiling
cd ffi/chapel && make profile
```

---

## 📦 DEPENDENCIES

```bash
# Update Rust deps
cargo update

# Check Rust outdated
cargo outdated

# Add Rust dependency
cargo add <crate>

# Go mod tidy
cd mcp-servers/github && go mod tidy

# Go update deps
cd mcp-servers/github && go get -u ./...
```

---

## 🔐 ENVIRONMENT

```bash
# Set CHAPEL_HOME (Linux/Mac)
export CHAPEL_HOME=/opt/chapel
export PATH=$PATH:$CHAPEL_HOME/bin

# Set CHAPEL_HOME (Windows PowerShell)
$env:CHAPEL_HOME = "C:\chapel"
$env:PATH += ";$env:CHAPEL_HOME\bin"

# Set GITHUB_TOKEN
export GITHUB_TOKEN="ghp_your_token_here"

# Set Rust backtrace
export RUST_BACKTRACE=1

# Set Rust log level
export RUST_LOG=debug
```

---

## 🎓 LEARNING

```bash
# Chapel version
chpl --version

# Rust version
rustc --version && cargo --version

# Go version
go version

# Show Chapel help
cd ffi/chapel && make help

# Show cargo help
cargo --help

# Show available make targets
cd ffi/chapel && make help
```

---

## 🚦 ONE-LINERS FOR COMMON TASKS

### Quick Validation (Before Commit)
```bash
cargo fmt && cargo test test_exactly_5_tools && cd ffi/chapel && make check
```

### Full Build & Test
```bash
cargo build --release && cargo test test_exactly_5_tools && cd ffi/chapel && make full-pipeline && make test
```

### Clean & Rebuild
```bash
cargo clean && cd ffi/chapel && make clean && cd ../.. && cargo build --release && cd ffi/chapel && make full-pipeline
```

### Start Development Environment
```bash
# Terminal 1: MCP Server
cargo run --bin nuclear-mcp --release

# Terminal 2: Chapel Training (optional)
cd ffi/chapel && make train && make run
```

### Pre-PR Checklist
```bash
cargo fmt -- --check && \
cargo test test_exactly_5_tools && \
cargo test --lib && \
cd ffi/chapel && make check && make test && \
cd ../../mcp-servers/github && go test ./... && go vet ./...
```

---

## 💡 PRO TIPS

### Faster Builds
```bash
# Rust incremental compilation (already default)
# Use cargo check instead of cargo build for quick feedback
cargo check

# Chapel parallel make (if supported)
cd ffi/chapel && make -j8 full-pipeline
```

### Watch for Changes
```bash
# Rust watch (needs cargo-watch)
cargo install cargo-watch
cargo watch -x check

# Chapel watch (with entr if installed)
find ffi/chapel -name "*.chpl" | entr make -C ffi/chapel check
```

### Quick Tests
```bash
# Test specific function
cargo test test_exactly_5_tools

# Test with output
cargo test -- --nocapture

# Test specific file
cargo test --test integration_real_mcp
```

---

## 🆘 TROUBLESHOOTING

### Build Fails
```bash
# 1. Clean everything
cargo clean && cd ffi/chapel && make clean

# 2. Update dependencies
cargo update

# 3. Check environment
echo $CHAPEL_HOME
echo $RUST_BACKTRACE

# 4. Verify installations
chpl --version
rustc --version
go version
```

### Tests Fail
```bash
# 1. Run with verbose output
cargo test -- --nocapture --test-threads=1

# 2. Check if 5 tools constraint is met
cargo test test_exactly_5_tools

# 3. Verify server builds
cargo build --release --bin nuclear-mcp
```

### Chapel Errors
```bash
# 1. Syntax check
cd ffi/chapel && make check

# 2. Verify CHAPEL_HOME
ls $CHAPEL_HOME/bin/chpl

# 3. Check dataset paths
ls ../../../models/  # from ffi/chapel/
```

---

**Guarda este archivo en marcadores para acceso rápido!**

**Cursor Shortcut**: `Cmd/Ctrl + P` → busca "COMMANDS_" → enter
