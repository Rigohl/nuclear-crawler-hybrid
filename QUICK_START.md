# QUICK START - Nuclear Crawler Hybrid

## Installation

### Prerequisites
```bash
# Rust 1.70+
rustc --version

# Optional FFI backends
go version              # Go 1.21+
zig version             # Zig 0.11+
nim --version           # Nim 2.0+
chpl --version          # Chapel 2.0+
python3 --version       # Python 3.8+
```

### Clone & Build
```bash
git clone https://github.com/Rigohl/nuclear-crawler-hybrid
cd nuclear-crawler-hybrid

# Build with all features (recommended)
cargo build --release --all-features

# Or minimal (Rust only, slower)
cargo build --release
```

### Verify Build
```bash
# Check exactly 7 tools registered
cargo test test_exactly_7_tools

# Run all tests
cargo test --lib

# Build completes without errors
cargo build --release 2>&1 | grep -i error
```

---

## Running the Server

### Start MCP Server
```bash
# Standard startup
./target/release/nuclear-mcp

# Output should show:
# 🔥 MCP Server: Starting HTTP listener on 127.0.0.1:8079
# ✅ Loaded 7 MCP tools
# ✅ Chapel AI ready
# ✅ FFI backends: Go, Zig, Nim available
```

### Verify Server Is Running
```bash
# Health check
curl http://localhost:8079/health

# Expected response:
# {"status":"healthy","tools":7,"version":"2.0"}
```

---

## Using the 7 Tools

### 1. WEBSEARCH
```bash
curl -X POST http://localhost:8079/tools/websearch \
  -H "Content-Type: application/json" \
  -d '{
    "query": "lateral movement exploitation techniques",
    "max_results": 50,
    "deep_search": false
  }'
```

### 2. PREMIUM_CONTENT
```bash
curl -X POST http://localhost:8079/tools/premium_content \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://paywall-site.com/article",
    "method": "auto",
    "timeout_seconds": 30
  }'
```

### 3. SCAN_WORKSPACE
```bash
curl -X POST http://localhost:8079/tools/scan_workspace \
  -H "Content-Type: application/json" \
  -d '{
    "path": "/home/user/myproject",
    "language": "rust",
    "include_suggestions": true
  }'
```

### 4. AI_DATASET_TRAINER
```bash
curl -X POST http://localhost:8079/tools/ai_dataset_trainer \
  -H "Content-Type: application/json" \
  -d '{
    "topic": "lateral_movement",
    "sample_count": 7500,
    "diversity": 0.95,
    "include_edge_cases": true
  }'
```

### 5. FILE_SEARCH
```bash
curl -X POST http://localhost:8079/tools/file_search \
  -H "Content-Type: application/json" \
  -d '{
    "path": "/home/user/myproject",
    "keyword": "unwrap()",
    "pattern_type": "error_pattern",
    "recursive": true
  }'
```

### 6. LATERAL_MOVEMENT
```bash
curl -X POST http://localhost:8079/tools/lateral_movement \
  -H "Content-Type: application/json" \
  -d '{
    "operation": "pass_the_hash_psexec",
    "targets": ["192.168.1.10"],
    "credentials": {
      "username": "Administrator",
      "hash": "NTLM_HASH_HERE"
    },
    "command": "whoami"
  }'
```

### 7. CODE_INTELLIGENCE
```bash
curl -X POST http://localhost:8079/tools/code_intelligence \
  -H "Content-Type: application/json" \
  -d '{
    "code": "let x = file.read().unwrap();",
    "language": "rust",
    "analysis_type": "security",
    "include_fixes": true
  }'
```

---

## Python Client Example

```python
import requests
import json

BASE_URL = "http://localhost:8079"

def websearch(query, max_results=50):
    response = requests.post(
        f"{BASE_URL}/tools/websearch",
        json={
            "query": query,
            "max_results": max_results,
            "deep_search": False
        }
    )
    return response.json()

def scan_workspace(path, language="rust"):
    response = requests.post(
        f"{BASE_URL}/tools/scan_workspace",
        json={
            "path": path,
            "language": language,
            "include_suggestions": True
        }
    )
    return response.json()

# Example usage
if __name__ == "__main__":
    # Search the web
    results = websearch("exploit techniques")
    print(f"Found {len(results['results'])} results")

    # Scan a workspace
    scan = scan_workspace("/home/user/myproject")
    print(f"Found {scan['summary']['errors']} errors")
```

---

## Environment Configuration

### Optional: Set Custom Port
```bash
# Default is 8079
export MCP_PORT=9000
./target/release/nuclear-mcp
```

### Optional: Enable Go FFI
```bash
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH
./target/release/nuclear-mcp
```

### Optional: Enable JAX GPU
```bash
export CUDA_VISIBLE_DEVICES=0
./target/release/nuclear-mcp
```

### Optional: Enable Chapel AI
```bash
export CHAPEL_THREADS=8
./target/release/nuclear-mcp
```

---

## Development Workflow

### Run in Debug Mode
```bash
RUST_LOG=debug cargo run --release --bin nuclear-mcp
```

### Run Tests
```bash
# Unit tests only
cargo test --lib

# Integration tests (requires running server)
cargo test --test integration_real_mcp --release

# Test exactly 7 tools
cargo test test_exactly_7_tools
```

### Code Formatting
```bash
cargo fmt -- --check
cargo clippy
```

### Build Individual Features
```bash
# Rust only (no FFI)
cargo build --release

# With Go FFI
cargo build --release --features go_integration

# All FFI backends
cargo build --release --all-features
```

---

## Troubleshooting

### Server won't start
```bash
# Check if port 8079 is in use
lsof -i :8079

# Try different port
MCP_PORT=9000 cargo run --release
```

### Tools return empty results
```bash
# Check server logs
RUST_LOG=info cargo run --release

# Verify internet connectivity
curl -X POST http://localhost:8079/tools/websearch \
  -H "Content-Type: application/json" \
  -d '{"query":"test"}'
```

### FFI libraries not loading
```bash
# Check library path
ldd target/release/nuclear-mcp | grep not\ found

# Add to library path
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH
cargo build --release --all-features
```

### Chapel AI not working
```bash
# Verify Chapel installation
chpl --version

# Recompile Chapel integration
cargo build --release --features chapel_ffi
```

---

## Performance Tuning

### Maximum Web Search Performance
```bash
# Enable all FFI backends
cargo build --release --all-features

# Run with Chapel AI optimization
./target/release/nuclear-mcp

# Query with deep_search enabled
# Uses 50K goroutines × 55 engines = 2,750,000 parallel probes
```

### Maximum Code Analysis Performance
```bash
# Enable Zig SIMD + JAX GPU
cargo build --release --features zig_integration,jax_integration

# Large workspace scan will use:
# - Zig 256-bit SIMD for pattern matching
# - JAX GPU for ML vulnerability detection
```

### Dataset Generation Performance
```bash
# Ensure Chapel AI is available
cargo build --release --features chapel_ffi

# Generate large dataset (10K samples)
# Will use Chapel neural network + WASM acceleration
# Typical time: 8-15 seconds
```

---

## Next Steps

1. **Read ARCHITECTURE.md** - Understand system design
2. **Read MCP_TOOLS.md** - Detailed tool specifications
3. **Read CAPABILITIES.md** - Feature details
4. **Read FFI_INTEGRATION.md** - Backend performance

---

**Last Updated**: 9 de febrero de 2026
