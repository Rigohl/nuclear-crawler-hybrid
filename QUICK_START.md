# 🚀 Quick Start Guide - Nuclear Crawler Hybrid

Get started with the integrated Nuclear Crawler Hybrid platform in minutes.

## Prerequisites

- **Rust** 1.70+ (`rustup install stable`)
- **Chapel** 1.30+ ([Installation Guide](https://chapel-lang.org/download.html))
- **Python** 3.9+ with pip
- **Go** 1.21+ (for GitHub MCP server)
- **Git** for version control

Optional:
- **Julia** 1.9+ (for scientific ML)
- **Mojo** (for high-performance datasets)
- **CUDA** 12.0+ (for GPU acceleration)

## Installation

### 1. Clone Repository

```bash
git clone https://github.com/Rigohl/nuclear-crawler-hybrid.git
cd nuclear-crawler-hybrid
```

### 2. Build Chapel AI Training Engine

```bash
cd ffi/chapel
make full-pipeline
```

This builds:
- Chapel AI library (`libchapel_ai.so`)
- Training pipeline
- Data mining engine
- Scientific analysis
- Code tools (analyzer, repair, reviewer)
- Unified AI system

### 3. Build Rust Components

```bash
cd ../..
cargo build --release
```

### 4. Build GitHub MCP Server

```bash
cd mcp-servers/github
go build -o github-mcp-server
cd ../..
```

### 5. Install Python Dependencies (Optional)

```bash
pip install -r requirements.txt
```

## Basic Usage

### Run Chapel AI Training

```bash
cd ffi/chapel
make run              # Single-locale training
make run-unified      # Run unified AI system
make mine            # Run data mining
make analyze         # Run scientific analysis
```

### Run All Systems

```bash
make execute-all     # Runs all 8 AI systems sequentially
```

### Use MCP Tools (Rust)

```bash
# Web search
cargo run --bin nuclear-mcp -- websearch "Chapel parallel programming"

# File operations
cargo run --bin nuclear-mcp -- file_search "*.rs"

# Workspace scan
cargo run --bin nuclear-mcp -- scan .

# Dataset training
cargo run --bin nuclear-mcp -- ai_dataset_trainer --dataset models/data/
```

### Use GitHub MCP Server

```bash
cd mcp-servers/github
export GITHUB_TOKEN="your_github_token"
./github-mcp-server

# In another terminal, connect via MCP client
# (e.g., Claude Desktop, Cursor, etc.)
```

## Quick Examples

### Example 1: Train Chapel AI on Custom Dataset

```bash
cd ffi/chapel
# Edit training/config.json to point to your dataset
make train
make run
```

### Example 2: Run Code Analysis

```bash
cd ffi/chapel
make analysis
./bin/code_analyzer your_code_file.chpl
```

### Example 3: Use Integrated Datasets

```bash
# Access 120K+ training samples
ls models/data/
ls models/mega_dataset/
ls models/powershell_dataset/

# Train on massive datasets
cd ffi/chapel
# Dataset paths are relative: ../../../models/
make train
```

### Example 4: GitHub Automation via MCP

```bash
cd mcp-servers/github
export GITHUB_TOKEN="ghp_xxxxx"
./github-mcp-server

# Use with your MCP client to:
# - Search repositories
# - Create issues
# - Manage pull requests
# - Run GitHub Actions
```

## Project Structure

```
nuclear-crawler-hybrid/
├── ffi/chapel/              # Chapel AI training engine
│   ├── ai/                  # Core AI modules
│   ├── training/            # Training pipelines
│   ├── tools/               # Development tools
│   └── Makefile             # Build system
│
├── models/                  # Integrated datasets (120K+ samples)
│   ├── data/                # Math datasets
│   ├── mega_dataset/        # Large-scale data
│   ├── powershell_dataset/  # PowerShell data
│   └── trained_models/      # Pre-trained models
│
├── mcp-servers/             # MCP server integrations
│   └── github/              # GitHub MCP server (Go)
│
├── src/                     # Rust source code
│   ├── mcp/                 # MCP tools implementation
│   ├── core/                # Core functionality
│   └── ai/                  # AI integrations
│
├── examples/                # Usage examples
├── tests/                   # Integration tests
└── docs/                    # Documentation
```

## Configuration

### Chapel Environment

```bash
export CHAPEL_HOME=/opt/chapel
export PATH=$PATH:$CHAPEL_HOME/bin
export CHPL_HOME=$CHAPEL_HOME
```

### GPU Support (Optional)

```bash
# For NVIDIA GPUs
cd ffi/chapel
make GPU_ARCH=sm_80 build  # A100
make GPU_ARCH=sm_86 build  # RTX 3090
```

### HuggingFace Sync

The project is synced with HuggingFace:
- **Repo**: [Kimberlyindiva/nuclear-chapel-training](https://huggingface.co/Kimberlyindiva/nuclear-chapel-training)
- **Auto-sync**: Every 6 hours via GitHub Actions

## Next Steps

1. **Read Documentation**:
   - [INTEGRATION_STATUS.md](INTEGRATION_STATUS.md) - Integration details
   - [ffi/chapel/ARCHITECTURE.md](ffi/chapel/ARCHITECTURE.md) - Chapel AI architecture
   - [docs/MULTI_LANGUAGE_ML_ENGINE.md](docs/chapel/MULTI_LANGUAGE_ML_ENGINE.md) - Multi-language guide

2. **Explore Datasets**:
   - Browse `models/` for 120K+ training samples
   - Check `models/README.md` for dataset descriptions

3. **Try MCP Tools**:
   - Run examples in `examples/`
   - Test integration with `tests/`

4. **Customize Training**:
   - Edit `ffi/chapel/training/config.json`
   - Add your own datasets to `models/data/`

## Troubleshooting

### Chapel Not Found

```bash
# Install Chapel
wget https://github.com/chapel-lang/chapel/releases/download/1.30.0/chapel-1.30.0.tar.gz
tar xzf chapel-1.30.0.tar.gz
cd chapel-1.30.0
./configure
make
export PATH=$PATH:$(pwd)/bin
```

### Rust Build Errors

```bash
# Update Rust
rustup update stable

# Clean build
cargo clean
cargo build --release
```

### Go Build Issues

```bash
# Install Go dependencies
cd mcp-servers/github
go mod download
go build
```

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/Rigohl/nuclear-crawler-hybrid/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Rigohl/nuclear-crawler-hybrid/discussions)
- **Documentation**: See `docs/` directory

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Ready to start?** Run `make full-pipeline` in `ffi/chapel/` to build everything! 🚀
