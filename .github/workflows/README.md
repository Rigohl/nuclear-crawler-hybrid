# GitHub Actions Workflows

This directory contains the CI/CD workflows for the Nuclear Crawler Hybrid project.

## 🎯 Core Workflows

### CI/CD Pipelines

- **`ci.yml`** - Main CI pipeline with auto-retry and self-healing
  - Validates critical constraints (5 MCP tools, NO MOCKS policy)
  - Rust build, test, and validation with automatic retry
  - Chapel AI compilation and testing
  - Go GitHub MCP server testing
  - Security audits

- **`master-validation.yml`** - Comprehensive FFI and dependency validation
  - Multi-stage validation (format, lint, security, FFI, build, test)
  - FFI integration testing with retry mechanisms
  - Cross-platform builds (Linux, Windows, macOS)
  - Performance checks and Docker builds

- **`ci-self-healing.yml`** - 🆕 Automatic failure recovery and monitoring
  - Monitors main workflow failures
  - Applies automatic repairs (cache rebuild, dependency updates, formatting)
  - Creates GitHub Issues for irreparable failures
  - Manages model checkpoint backups
  - Docker health checks
  - Runs every 30 minutes

### Specialized Workflows

- **`chapel-ai-learning-hub.yml`** - Chapel AI continuous learning engine
- **`docker-build.yml`** - Docker image builds
- **`security.yml`** - Security scanning
- **`wasm-build.yml`** - WebAssembly builds
- **`ml-training-unified.yml`** - ML training pipeline
- **`sync-hf-github.yml`** - HuggingFace synchronization

## 🔧 Resilience Features

### Automatic Retry Mechanisms

All major workflows include:
- Automatic retry on failure with `cargo clean && cargo build`
- Cache rebuild on corruption detection
- Dependency update attempts
- Format auto-fix
- Multi-level fallback strategies

### Error Detection & Auto-Repair

Supported error patterns:
- ✅ Bincode compile errors
- ✅ Formatting issues
- ✅ Clippy warnings
- ✅ Cache corruption
- ✅ Dependency resolution failures
- ✅ Out of memory errors
- ✅ Lock file conflicts

### Model Checkpoint Management

- Automatic backup every 30 minutes
- Validation and corruption detection
- Dockerized recovery system
- 90-day retention

### Intelligent Notifications

- Automatic GitHub Issue creation on failure
- Failure classification and recommendations
- Deduplication (comments on existing issues)
- Labels: `ci-failure`, `auto-created`, `needs-investigation`

## 📚 Documentation

- **[RESILIENCE.md](RESILIENCE.md)** - Complete resilience system documentation
- **[copilot-instructions.md](copilot-instructions.md)** - AI agent guidelines

## Workflow Execution Status

**Last Triggered:** $(date)
**Status:** Running on push for validation

### Production Workflows
- ✅ ci.yml (on push, pull_request)
- ✅ master-validation.yml (on push, pull_request, weekly)
- ✅ ci-self-healing.yml (on workflow failure, every 30 min)
- ✅ chapel-ai-learning-hub.yml (on push, every 6 hours)
- ✅ dependency-tools-intelligence.yml (Monday 3 AM UTC + on push)
- ✅ auto-improvements-agent.yml (Monday 4 AM UTC + on push)  
- ✅ advanced-library-optimization.yml (Monday 6 AM UTC + on push)

All production-ready and executing without errors.

## 🚀 Quick Commands

```bash
# Validate workflows locally
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"

# Test auto-repair system
python3 scripts/validate_system.py --enhanced

# Test checkpoint management
python3 scripts/checkpoint_manager.py validate

# Manual workflow trigger
gh workflow run ci-self-healing.yml --ref main
```

## 🆘 Troubleshooting

See [RESILIENCE.md](RESILIENCE.md) for detailed troubleshooting guide.

