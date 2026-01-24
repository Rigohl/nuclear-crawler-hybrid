# ⚡ WORKFLOWS QUICK REFERENCE

## 🚀 For Developers

### Local Testing (before pushing)
```bash
# Test all workflows locally
bash .github/scripts/validate-workflows.sh

# OR run specific tests (requires Rust)
bash .github/scripts/test-workflows.sh
```

### Push to GitHub
```bash
git add .
git commit -m "Your changes"
git push origin your-branch

# GitHub will automatically run: master-validation.yml
```

### Create a Release
```bash
# Tag a release
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0

# GitHub will automatically run: release-optimized.yml
# Creates binaries for Linux, macOS, Windows
# Builds Docker image
# Creates GitHub Release with artifacts
```

---

## 📋 WORKFLOW REFERENCE

### master-validation.yml
**Runs on**: `push` to main/dev, `pull_request` to main/dev  
**Duration**: ~20-30 min  
**What it does**:
- ✅ Format check
- ✅ Linting (clippy)
- ✅ Build (release)
- ✅ Unit tests
- ✅ Integration tests
- ✅ MCP validation (exactly 5 tools)
- ✅ Performance checks
- ✅ Docker build
- ✅ Security audit

**View logs**: GitHub Actions → master-validation → [job]

### release-optimized.yml
**Runs on**: Git tag push (v*), manual dispatch  
**Duration**: ~45-60 min  
**What it does**:
- ✅ Build for Linux, macOS (Intel/ARM), Windows
- ✅ Build Docker image
- ✅ Create GitHub Release
- ✅ Upload artifacts
- ✅ Generate release notes

**View logs**: GitHub Actions → release-optimized → [job]

### dependency-analysis.yml (Optional)
**Runs on**: Schedule (weekly) or manual  
**What it does**:
- ✅ Analyze dependencies
- ✅ Check for vulnerabilities
- ✅ Generate report

---

## 🔍 Workflow Scripts

### .github/scripts/validate_5_tools.sh
**Purpose**: Verify exactly 5 MCP tools  
**Usage**:
```bash
bash .github/scripts/validate_5_tools.sh
```

### .github/scripts/test-workflows.sh
**Purpose**: Run all tests locally  
**Usage**:
```bash
bash .github/scripts/test-workflows.sh
```
**Requirements**: Rust toolchain, Python3

### .github/scripts/validate-workflows.sh
**Purpose**: Check workflow syntax & structure  
**Usage**:
```bash
bash .github/scripts/validate-workflows.sh
```

### .github/scripts/check_performance_thresholds.py
**Purpose**: Validate performance metrics  
**Usage**:
```bash
python3 .github/scripts/check_performance_thresholds.py
```

### .github/scripts/push_to_huggingface.sh
**Purpose**: Upload artifacts to Hugging Face  
**Usage**:
```bash
bash .github/scripts/push_to_huggingface.sh <HF_TOKEN> <REPO_NAME>
```

---

## 🐛 Troubleshooting

### Workflow fails locally
```bash
# Check Python dependencies
pip install pyyaml yamllint transformers

# Check Rust
rustc --version
cargo --version

# Run validation
bash .github/scripts/validate-workflows.sh
```

### Workflow fails on GitHub
1. Click on the failed workflow
2. Check the job logs
3. Common issues:
   - Build dependency issue (bincode compile_error)
   - Test timeout
   - Network issue

### MCP Tools validation fails
```bash
# Verify exactly 5 tools
bash .github/scripts/validate_5_tools.sh

# Check if server is running
curl http://localhost:8079/health
```

---

## 📊 Workflow Status Dashboard

Check status at: https://github.com/Rigohl/nuclear-crawler-hybrid/actions

### All Workflows
```
✅ master-validation.yml      - CI/CD pipeline
✅ release-optimized.yml      - Release builds
✅ dependency-analysis.yml    - Dependency checks
✅ sync-hf-github.yml         - HF synchronization
```

### Legacy Workflows (being phased out)
```
⚠️  ci.yml                     - Use master-validation instead
⚠️  release.yml               - Use release-optimized instead
⚠️  security.yml              - Included in master-validation
⚠️  docker-build.yml          - Included in master-validation
```

---

## 🎯 Common Tasks

### I pushed code and want to check if it passes
1. Go to https://github.com/Rigohl/nuclear-crawler-hybrid/actions
2. Look for "master-validation" workflow
3. Wait for it to complete (20-30 min)
4. Check if all jobs passed ✅

### I want to create a release
1. Ensure all tests pass
2. Create a tag: `git tag -a v1.0.0 -m "Release v1.0.0"`
3. Push the tag: `git push origin v1.0.0`
4. GitHub automatically:
   - Runs `release-optimized.yml`
   - Builds all binaries
   - Creates Docker image
   - Creates GitHub Release (5-10 min)
5. Check: https://github.com/Rigohl/nuclear-crawler-hybrid/releases

### I want to test locally before pushing
```bash
# Option 1: Test workflows
bash .github/scripts/validate-workflows.sh

# Option 2: Run full tests (requires Rust)
bash .github/scripts/test-workflows.sh

# Option 3: Just validate format
cargo fmt -- --check
```

### I want to add a new test
1. Edit `master-validation.yml`
2. Add new job under `jobs:`
3. Follow the pattern of existing jobs
4. Push and GitHub will run it

### I want to see performance metrics
```bash
# Generate report
python3 .github/scripts/generate_advanced_report.py

# Check thresholds
python3 .github/scripts/check_performance_thresholds.py
```

---

## 🚀 Performance Tips

### Make workflows faster
1. Use caching: `uses: Swatinem/rust-cache@v2`
2. Parallelize jobs: Use `needs: [job1, job2]`
3. Use `continue-on-error: true` for non-critical jobs
4. Avoid duplicate work

### Reduce build time
1. Use release cache
2. Skip integration tests on simple PRs
3. Use matrix strategy for platforms

---

## 📚 Documentation

- [WORKFLOWS_OPTIMIZATION_REPORT.md](.github/WORKFLOWS_OPTIMIZATION_REPORT.md) - Full report
- [.github/workflows/](workflows/) - All workflow files
- [.github/scripts/](scripts/) - Helper scripts
- [Contributing Guide](docs/CONTRIBUTING.md)

---

## 💬 Questions?

- Check [WORKFLOWS_OPTIMIZATION_REPORT.md](.github/WORKFLOWS_OPTIMIZATION_REPORT.md)
- Review workflow YAML files in `.github/workflows/`
- Check script comments in `.github/scripts/`
- Open an issue for problems

---

**Last Updated**: January 24, 2026  
**Status**: Production Ready  
**Contact**: @devops-team
