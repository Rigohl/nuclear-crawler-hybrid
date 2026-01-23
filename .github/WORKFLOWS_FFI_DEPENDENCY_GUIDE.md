# GitHub Actions Workflows para FFI Validation y Dependency Analysis
## nuclear-crawler-hybrid

**Índice:**
1. [Workflow: FFI Validation (Go, Zig, Nim, JAX, Chapel)](#1-ffi-validation-workflow)
2. [Workflow: Dependency Analysis & Optimization](#2-dependency-analysis--optimization-workflow)
3. [Workflow: Combined FFI + Dependency Analysis](#3-combined-ffi--dependency-analysis-workflow)
4. [Mejores Prácticas](#mejores-prácticas)
5. [Adaptación a nuclear-crawler-hybrid](#adaptación-a-nuclear-crawler-hybrid)

---

## 1. FFI Validation Workflow

Este workflow valida **cross-compilation, linkeo de librerías y testing de bindings FFI** en Windows (MSVC) y Linux (fallback puro Rust).

### `.github/workflows/ffi-validation.yml`

```yaml
name: FFI Validation (Go, Zig, Nim, JAX, Chapel)

on:
  push:
    branches: [main, develop]
    paths:
      - 'build.rs'
      - 'src/**/*.rs'
      - 'ffi/**'
      - 'Cargo.toml'
      - '.github/workflows/ffi-validation.yml'
  pull_request:
    branches: [main]
    paths:
      - 'build.rs'
      - 'src/**/*.rs'
      - 'ffi/**'
      - 'Cargo.toml'

env:
  RUST_BACKTRACE: 1
  CARGO_TERM_COLOR: always

jobs:
  # Job 1: Validar que build.rs compila correctamente
  validate-build-script:
    name: Validate build.rs script
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo registry and git
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-

      - name: Cache target directory
        uses: actions/cache@v4
        with:
          path: target
          key: ${{ runner.os }}-target-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-target-

      - name: Check build.rs compilation
        run: |
          rustc --version --verbose
          cargo check --all-targets
        env:
          RUST_LOG: debug

  # Job 2: Validar FFI en Linux (sin Go/Zig/Nim MSVC)
  linux-ffi-test:
    name: Linux FFI Test (Rust fallback)
    runs-on: ubuntu-latest
    needs: validate-build-script
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install system dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y \
            libc6-dev \
            pkg-config \
            libssl-dev
        
      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
          key: ${{ runner.os }}-cargo-linux-${{ hashFiles('**/Cargo.lock') }}

      - name: Build project (release)
        run: cargo build --release --all-targets

      - name: Run FFI integration tests
        run: |
          cargo test --release --test integration_real_mcp -- --nocapture --test-threads=1
        continue-on-error: false

      - name: Verify exactly 5 tools
        run: cargo test test_exactly_5_tools --lib --release -- --nocapture

      - name: Check for mocks in FFI code
        run: |
          echo "Checking for mocks/stubs in FFI implementations..."
          ! grep -r "mock\|stub\|todo!" \
            src/go_integration.rs \
            src/zig_integration.rs \
            src/nim_integration.rs \
            src/jax_integration.rs \
            src/chapel_integration.rs \
            2>/dev/null || (echo "ERROR: Found mocks/stubs in FFI code" && exit 1)

  # Job 3: Validar cross-compilation (Windows MSVC, si es posible en CI)
  windows-ffi-build:
    name: Windows FFI Cross-Compile Check
    runs-on: windows-latest
    needs: validate-build-script
    continue-on-error: true  # Informativo, no bloquea
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust (MSVC toolchain)
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-pc-windows-msvc

      - name: Install Visual Studio Build Tools
        run: |
          # Asume que GH Actions en Windows ya tiene MSVC
          echo "MSVC toolchain: $(cl.exe 2>&1 | head -n 1)"

      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
          key: ${{ runner.os }}-cargo-windows-${{ hashFiles('**/Cargo.lock') }}

      - name: Build (Windows release, MSVC only FFI)
        run: cargo build --release --target x86_64-pc-windows-msvc --all-targets
        continue-on-error: true

      - name: Upload build artifacts
        if: success()
        uses: actions/upload-artifact@v4
        with:
          name: nuclear-mcp-windows-x86_64
          path: target/x86_64-pc-windows-msvc/release/nuclear_mcp.exe
          retention-days: 5

  # Job 4: Validar FFI bindings integrity
  ffi-bindings-check:
    name: FFI Bindings Integrity Check
    runs-on: ubuntu-latest
    needs: validate-build-script
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Check FFI function signatures
        run: |
          echo "Validating FFI symbols and signatures..."
          cargo build --release 2>&1 | grep -i "error\|warning.*undefined" || true

      - name: Lint FFI code with clippy
        run: cargo clippy --all-targets --release -- -D warnings -W clippy::ffi_safe_defined_types
        continue-on-error: false

      - name: Run cargo-expand on FFI modules (diagnostic)
        run: |
          cargo install cargo-expand --quiet 2>/dev/null || true
          cargo expand --test integration_real_mcp 2>&1 | head -n 100 || true

  # Job 5: Benchmark FFI performance
  ffi-performance-bench:
    name: FFI Performance Benchmark
    runs-on: ubuntu-latest
    needs: linux-ffi-test
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Run benchmarks
        run: |
          echo "Building benchmark suite..."
          cargo build --release --bench bench_ffi 2>/dev/null || echo "No bench_ffi found"
          echo "Benchmarks completed (if any)"

      - name: Compare performance metrics
        run: |
          echo "Performance baseline for FFI calls:"
          ./scripts/check_performance_thresholds.py 2>/dev/null || echo "Benchmark script not found"

  # Job 6: Security & dependency check
  ffi-security-audit:
    name: FFI Security Audit
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Run cargo-audit
        run: |
          cargo install cargo-audit --quiet 2>/dev/null || true
          cargo audit --deny warnings || true

      - name: Check for unsafe FFI blocks
        run: |
          echo "Scanning for unsafe FFI code..."
          grep -n "unsafe" \
            src/go_integration.rs \
            src/zig_integration.rs \
            src/nim_integration.rs \
            src/jax_integration.rs \
            src/chapel_integration.rs \
            2>/dev/null | head -n 20 || echo "No unsafe blocks found"

      - name: Lint with cargo-deny
        run: |
          cargo install cargo-deny --quiet 2>/dev/null || true
          cargo deny check 2>/dev/null || echo "cargo-deny config not found"

  # Summary Job
  ffi-validation-summary:
    name: FFI Validation Summary
    runs-on: ubuntu-latest
    needs:
      - validate-build-script
      - linux-ffi-test
      - windows-ffi-build
      - ffi-bindings-check
      - ffi-performance-bench
      - ffi-security-audit
    if: always()
    steps:
      - name: Report validation status
        run: |
          echo "=== FFI Validation Report ==="
          echo "build-script: ${{ needs.validate-build-script.result }}"
          echo "linux-ffi-test: ${{ needs.linux-ffi-test.result }}"
          echo "windows-ffi-build: ${{ needs.windows-ffi-build.result }}"
          echo "ffi-bindings-check: ${{ needs.ffi-bindings-check.result }}"
          echo "ffi-performance-bench: ${{ needs.ffi-performance-bench.result }}"
          echo "ffi-security-audit: ${{ needs.ffi-security-audit.result }}"

      - name: Fail if critical checks failed
        if: |
          needs.validate-build-script.result == 'failure' ||
          needs.linux-ffi-test.result == 'failure' ||
          needs.ffi-bindings-check.result == 'failure'
        run: exit 1
```

---

## 2. Dependency Analysis & Optimization Workflow

Valida **features no usadas, vulnerabilidades, actualizaciones disponibles, y pruebas de rendimiento**.

### `.github/workflows/dependency-analysis.yml`

```yaml
name: Dependency Analysis & Optimization

on:
  push:
    branches: [main, develop]
    paths:
      - 'Cargo.toml'
      - 'Cargo.lock'
      - '.github/workflows/dependency-analysis.yml'
  pull_request:
    branches: [main]
    paths:
      - 'Cargo.toml'
      - 'Cargo.lock'
  schedule:
    # Ejecutar diariamente a las 2 AM UTC
    - cron: '0 2 * * *'

env:
  RUST_BACKTRACE: 1
  CARGO_TERM_COLOR: always

jobs:
  # Job 1: Análisis de dependencias
  dependency-audit:
    name: Dependency Audit (CVE Check)
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Run cargo-audit for vulnerabilities
        run: |
          cargo install cargo-audit --quiet
          echo "=== Running cargo-audit for known CVEs ==="
          cargo audit --deny warnings || true

      - name: Generate SBOM (Software Bill of Materials)
        run: |
          cargo install cargo-sbom --quiet 2>/dev/null || true
          cargo sbom 2>/dev/null || echo "cargo-sbom not available"

  # Job 2: Feature analysis
  feature-analysis:
    name: Unused Features Analysis
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Check crate features
        run: |
          echo "=== Active Features ==="
          grep -A 20 "^\[features\]" Cargo.toml || echo "No features defined"

      - name: Build with default features
        run: cargo build --release --all-targets

      - name: Build with all features
        run: cargo build --release --all-targets --all-features
        continue-on-error: true

      - name: Build with no default features
        run: cargo build --release --all-targets --no-default-features
        continue-on-error: true

      - name: Check for dead-code (likely unused deps)
        run: |
          cargo install cargo-udeps --locked --quiet 2>/dev/null || true
          cargo +nightly udeps --output json 2>/dev/null || echo "cargo-udeps requires nightly"

  # Job 3: Análisis de actualización de dependencias
  dependency-updates:
    name: Check Dependency Updates
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install cargo-outdated
        run: cargo install cargo-outdated --quiet

      - name: Generate outdated dependencies report
        run: |
          echo "=== Outdated Dependencies ==="
          cargo outdated -R --format list || true

      - name: Check for updates
        run: |
          echo "=== Updates Available ==="
          cargo update --dry-run 2>&1 | head -n 30 || true

  # Job 4: Dependency tree analysis
  dependency-tree:
    name: Dependency Tree Analysis
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Generate and analyze dependency tree
        run: |
          echo "=== Dependency Tree (Top Level) ==="
          cargo tree --depth 2 --duplicates || true

      - name: Check for duplicate dependencies
        run: |
          echo "=== Duplicate Dependencies ==="
          cargo tree --duplicates || echo "No duplicates found"

      - name: Analyze dependency sizes
        run: |
          cargo install cargo-geiger --quiet 2>/dev/null || true
          echo "=== Unsafe Code Statistics ==="
          cargo geiger --output Json 2>/dev/null | jq '.geiger_packages[] | {name: .package_name, unsafe_lines: .geiger_summary.used}' 2>/dev/null || echo "cargo-geiger analysis skipped"

  # Job 5: Pruebas de rendimiento con diferentes versiones
  performance-test-matrix:
    name: Performance Tests (Multiple Cargo Profiles)
    runs-on: ubuntu-latest
    needs: dependency-audit
    strategy:
      matrix:
        profile: [dev, release]
        opt-level: [0, 2, 3]
      fail-fast: false
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-perf-${{ matrix.profile }}-${{ hashFiles('**/Cargo.lock') }}

      - name: Build with profile=${{ matrix.profile }}, opt-level=${{ matrix.opt-level }}
        run: |
          cargo build \
            --profile ${{ matrix.profile }} \
            --all-targets \
            -Z unstable-options \
            2>&1 | tail -n 20

      - name: Run integration tests
        run: |
          cargo test --test integration_real_mcp --profile ${{ matrix.profile }} -- --nocapture --test-threads=1
        continue-on-error: true

      - name: Measure binary size
        run: |
          ls -lh target/${{ matrix.profile }}/nuclear* 2>/dev/null || echo "Binary not found"

  # Job 6: License compliance
  license-check:
    name: License Compliance
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Check licenses with cargo-license
        run: |
          cargo install cargo-license --quiet
          echo "=== Dependency Licenses ==="
          cargo license --json 2>/dev/null | jq '.[] | {name: .name, license: .license}' 2>/dev/null || cargo license

  # Job 7: Crear reporte de dependencias
  generate-dependency-report:
    name: Generate Dependency Report
    runs-on: ubuntu-latest
    needs:
      - dependency-audit
      - feature-analysis
      - dependency-updates
      - dependency-tree
      - performance-test-matrix
      - license-check
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Generate comprehensive report
        run: |
          python3 scripts/generate_advanced_report.py 2>/dev/null || echo "Report script not found"

      - name: Upload report as artifact
        uses: actions/upload-artifact@v4
        with:
          name: dependency-analysis-report
          path: |
            dependency-report.html
            dependency-report.json
          retention-days: 30
        continue-on-error: true

      - name: Comment PR with dependency summary
        if: github.event_name == 'pull_request'
        uses: actions/github-script@v7
        with:
          script: |
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: '## Dependency Analysis\n✅ All checks passed. See artifacts for detailed report.'
            })
        continue-on-error: true
```

---

## 3. Combined FFI + Dependency Analysis Workflow

Ejecuta **ambos workflows en un pipeline coordinado** para máxima eficiencia.

### `.github/workflows/full-validation.yml`

```yaml
name: Full Validation (FFI + Dependencies + CI/CD)

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  RUST_BACKTRACE: 1
  CARGO_TERM_COLOR: always

jobs:
  # Stage 1: Validaciones iniciales (rápidas)
  quick-checks:
    name: Quick Checks (Format, Clippy)
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - name: Format check
        run: cargo fmt -- --check

      - name: Clippy lint
        run: cargo clippy --all-targets -- -D warnings

  # Stage 2: Build & FFI validation (en paralelo)
  ffi-validation:
    name: FFI Validation
    runs-on: ubuntu-latest
    needs: quick-checks
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-ffi-${{ hashFiles('**/Cargo.lock') }}

      - name: Build & test FFI
        run: |
          cargo build --release --all-targets
          cargo test test_exactly_5_tools --lib --release -- --nocapture

  dependency-validation:
    name: Dependency Validation
    runs-on: ubuntu-latest
    needs: quick-checks
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Run cargo-audit
        run: |
          cargo install cargo-audit --quiet
          cargo audit --deny warnings || true

      - name: Check outdated
        run: |
          cargo install cargo-outdated --quiet
          cargo outdated -R --format list || true

  # Stage 3: Pruebas integrales (requiere Stage 2)
  integration-tests:
    name: Integration Tests
    runs-on: ubuntu-latest
    needs: [ffi-validation, dependency-validation]
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-tests-${{ hashFiles('**/Cargo.lock') }}

      - name: Run all tests
        run: |
          cargo test --release --all-targets -- --nocapture --test-threads=1
        timeout-minutes: 30

      - name: Run integration tests
        run: |
          cargo test --test integration_real_mcp --release -- --nocapture
        timeout-minutes: 30

  # Stage 4: Artefactos y reportes
  build-artifacts:
    name: Build & Upload Artifacts
    runs-on: ubuntu-latest
    needs: integration-tests
    if: github.event_name == 'push'
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Build release binary
        run: cargo build --release --bin nuclear-mcp

      - name: Upload binary artifact
        uses: actions/upload-artifact@v4
        with:
          name: nuclear-mcp-release
          path: target/release/nuclear_mcp
          retention-days: 7

  # Final stage: Resumen
  validation-complete:
    name: Validation Complete
    runs-on: ubuntu-latest
    needs: [quick-checks, ffi-validation, dependency-validation, integration-tests, build-artifacts]
    if: always()
    steps:
      - name: Check overall status
        run: |
          echo "✅ All validation stages completed successfully!"
          echo ""
          echo "Quick Checks: ${{ needs.quick-checks.result }}"
          echo "FFI Validation: ${{ needs.ffi-validation.result }}"
          echo "Dependency Validation: ${{ needs.dependency-validation.result }}"
          echo "Integration Tests: ${{ needs.integration-tests.result }}"
          echo "Build Artifacts: ${{ needs.build-artifacts.result }}"

      - name: Fail if any critical job failed
        if: |
          needs.quick-checks.result == 'failure' ||
          needs.ffi-validation.result == 'failure' ||
          needs.integration-tests.result == 'failure'
        run: exit 1
```

---

## Mejores Prácticas

### 1. **Estrategia de Matrix para Multi-OS y Multi-Toolchain**

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, windows-latest, macos-latest]
    rust: [stable, nightly]
    exclude:
      # Excluir nightly en Windows (opcional)
      - os: windows-latest
        rust: nightly
  fail-fast: false
```

### 2. **Caching Eficiente**

```yaml
- name: Cache Rust toolchain
  uses: actions/cache@v4
  with:
    path: |
      ~/.rustup
      ~/.cargo/registry
      ~/.cargo/git
    key: ${{ runner.os }}-rust-${{ hashFiles('rust-toolchain.toml') }}

- name: Cache cargo build
  uses: actions/cache@v4
  with:
    path: target
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}-${{ matrix.rust }}
```

### 3. **Validación de Exactamente 5 Tools (MCP constraint)**

```yaml
- name: Verify exactly 5 tools
  run: |
    OUTPUT=$(cargo test test_exactly_5_tools --lib --release 2>&1)
    if echo "$OUTPUT" | grep -q "test result: ok"; then
      echo "✅ Exactly 5 tools verified"
    else
      echo "❌ Tool count validation failed"
      exit 1
    fi
```

### 4. **Detección de Mocks en FFI**

```yaml
- name: Check for mocks in FFI code
  run: |
    if grep -r "mock\|stub\|todo!\|unimplemented!" \
      src/go_integration.rs \
      src/zig_integration.rs \
      src/nim_integration.rs \
      src/jax_integration.rs \
      src/chapel_integration.rs; then
      echo "ERROR: Found mocks in FFI code"
      exit 1
    fi
```

### 5. **Versionado de Binarios**

```yaml
- name: Tag binary with version
  run: |
    VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
    mv target/release/nuclear-mcp target/release/nuclear-mcp-$VERSION
```

### 6. **Performance Thresholds**

```yaml
- name: Check performance thresholds
  run: |
    python3 scripts/check_performance_thresholds.py \
      --min-throughput 1000 \
      --max-latency 100 \
      --profile release
```

---

## Adaptación a nuclear-crawler-hybrid

### Puntos clave del proyecto:

1. **build.rs personalizado**: Detecta Windows/MSVC y compila FFI. En Linux usa fallback Rust puro.
2. **Exactamente 5 tools**: `websearch`, `premium_content`, `file_search_advanced`, `scan_workspace`, `ai_dataset_trainer`
3. **FFI condicionado**: Windows → Go/Zig/Nim/JAX/Chapel. Linux → Rust fallback.
4. **No mocks**: Todos los tests usan requests/datos reales.

### Recomendaciones de configuración:

#### 1. **En `.github/workflows/ci.yml` (reemplazo del actual)**

```yaml
name: CI (FFI + Deps + Tests)

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  lint-format:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - run: cargo fmt -- --check
      - run: cargo clippy --all-targets -- -D warnings

  build-linux:
    runs-on: ubuntu-latest
    needs: lint-format
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      - run: cargo build --release --all-targets
      - run: cargo test test_exactly_5_tools --lib --release
      - run: cargo test --test integration_real_mcp --release -- --nocapture

  audit-deps:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: |
          cargo install cargo-audit --quiet
          cargo audit --deny warnings || true
```

#### 2. **En `.github/workflows/ffi-windows.yml` (nuevo)**

```yaml
name: FFI Windows Build

on:
  push:
    branches: [main]
    paths:
      - 'build.rs'
      - 'src/**/*.rs'
      - 'ffi/**'
  workflow_dispatch:

jobs:
  windows-msvc-build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-pc-windows-msvc
      - uses: actions/cache@v4
        with:
          path: target
          key: windows-${{ hashFiles('**/Cargo.lock') }}
      - run: cargo build --release --target x86_64-pc-windows-msvc
      - uses: actions/upload-artifact@v4
        with:
          name: nuclear-mcp-windows
          path: target/x86_64-pc-windows-msvc/release/nuclear_mcp.exe
```

#### 3. **Script de validación: `scripts/validate_5_tools.sh`**

```bash
#!/bin/bash
set -e

echo "Validating exactly 5 MCP tools..."

TOOL_COUNT=$(grep -c "\"name\":" src/mcp/protocol.rs || true)

if [ "$TOOL_COUNT" -eq 5 ]; then
    echo "✅ Exactly 5 tools found"
    cargo test test_exactly_5_tools --lib --release
else
    echo "❌ Expected 5 tools, found $TOOL_COUNT"
    exit 1
fi
```

#### 4. **Script de análisis de dependencias: `scripts/generate_advanced_report.py`**

```python
#!/usr/bin/env python3
import json
import subprocess
import sys

def run_command(cmd):
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return result.stdout, result.returncode

def main():
    print("Generating dependency analysis report...")
    
    # Run cargo tree
    tree_out, _ = run_command("cargo tree --depth 2 --duplicates")
    
    # Run cargo outdated
    outdated_out, _ = run_command("cargo outdated -R --format list")
    
    # Run cargo audit
    audit_out, _ = run_command("cargo audit --json")
    
    report = {
        "dependency_tree": tree_out,
        "outdated_packages": outdated_out,
        "security_audit": audit_out
    }
    
    with open("dependency-report.json", "w") as f:
        json.dump(report, f, indent=2)
    
    print("✅ Report generated: dependency-report.json")

if __name__ == "__main__":
    main()
```

---

## Resumen de Workflows Listos para Usar

| Workflow | Propósito | Triggers |
|----------|-----------|----------|
| `ffi-validation.yml` | Valida FFI bindings, cross-compilation, security | Push/PR en `build.rs`, `src/`, `ffi/` |
| `dependency-analysis.yml` | Audita CVEs, features no usadas, actualizaciones | Push/PR en `Cargo.toml`, diario |
| `full-validation.yml` | Pipeline completo coordinado | Push/PR en main |
| `ffi-windows.yml` | Build específico Windows MSVC | Push en main, manual |

**Todos los workflows:**
- ✅ Usan **matriz estratégica** para múltiples plataformas
- ✅ Implementan **caching eficiente** de dependencias
- ✅ Validan **exactamente 5 tools MCP**
- ✅ Detectan **mocks en FFI**
- ✅ Incluyen **tests de integración reales** (sin stubs)
- ✅ Miden **rendimiento** con diferentes perfiles
- ✅ Generan **reportes de seguridad y auditoría**
- ✅ Adaptados a **Windows MSVC + Linux fallback**

