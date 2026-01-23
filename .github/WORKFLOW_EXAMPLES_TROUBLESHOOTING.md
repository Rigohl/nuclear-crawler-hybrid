# Ejemplos Prácticos y Troubleshooting
## GitHub Actions FFI Validation & Dependency Analysis

---

## 📋 Ejemplos de Salida Esperada

### FFI Validation - Ejecución Exitosa

```
✅ FFI Validation Summary
================================

build-script: success
linux-ffi-test: success
windows-ffi-build: success (informativo)
ffi-bindings-check: success
ffi-performance-bench: success
ffi-security-audit: success

Detalles:
- build.rs compila ✅
- Tests FFI Linux pasan ✅
- Exactly 5 tools verificados ✅
- Sin mocks en FFI code ✅
- 0 unsafe blocks problemáticos ✅
- cargo-audit: 0 vulnerabilidades ✅
```

### Dependency Analysis - Reporte

```json
{
  "total_dependencies": 42,
  "vulnerabilities": 0,
  "outdated_packages": [
    {
      "name": "serde",
      "current": "1.0.190",
      "latest": "1.0.195"
    }
  ],
  "unsafe_code": {
    "go_integration.rs": 3,
    "zig_integration.rs": 2,
    "nim_integration.rs": 1,
    "jax_integration.rs": 0,
    "chapel_integration.rs": 0
  },
  "licenses": [
    {"name": "axum", "license": "MIT"},
    {"name": "tokio", "license": "MIT"},
    {"name": "serde", "license": "MIT OR Apache-2.0"}
  ]
}
```

### Full Validation - Logs

```
Stage 1: Quick Checks
  ✅ Format check passed
  ✅ Clippy lint passed

Stage 2: Parallel Validation
  ✅ FFI Validation completed (3m 24s)
  ✅ Dependency Validation completed (1m 12s)

Stage 3: Integration Tests
  ✅ All tests passed (4m 18s)
  ✅ integration_real_mcp tests: 42 passed

Stage 4: Build Artifacts
  ✅ Binary uploaded (45 MB)

Stage 5: Summary
  ✅ Pipeline completed successfully
  ⏱️  Total time: 9m 54s
```

---

## 🔧 Troubleshooting Guide

### 1. FFI Validation - Build.rs Error

**Error:**
```
error: failed to run custom build command for `nuclear-mcp v0.1.0`
error: build script failed with code 1
```

**Diagnóstico:**
```bash
# Ver logs completos
cargo build 2>&1 | tail -n 50

# Chequear build.rs syntax
rustc --crate-type bin build.rs -C opt-level=0
```

**Soluciones:**
- Verificar que Go/Zig/Nim están instalados si es Windows
- En Linux, debería usar fallback Rust puro (verifica `build.rs`)
- Chequear permisos en `ffi/` directory

---

### 2. Windows MSVC Build Falla

**Error:**
```
error: Microsoft Visual C++ 14.0 is required
```

**Solución:**
- Es informativo (`continue-on-error: true`), **no bloquea CI**
- Linux fallback es suficiente para validación
- En local, instalar Visual Studio Build Tools

---

### 3. Integration Tests Timeout

**Error:**
```
Error: The operation timed out after 30 minutes
```

**Solución 1:** Aumentar timeout
```yaml
# En el workflow
- name: Run integration tests
  timeout-minutes: 60
  run: cargo test --test integration_real_mcp --release
```

**Solución 2:** Verificar que no hay hang en network
```bash
# Local test
timeout 60 cargo test --test integration_real_mcp --release -- --nocapture
```

---

### 4. Cache Miss Frecuente

**Síntoma:** Workflows lentos, cache nunca hit

**Diagnóstico:**
```bash
# Ver tamaño del cache
du -sh ~/.cargo/registry ~/.cargo/git target/ 2>/dev/null

# Chequear Cargo.lock changes
git diff HEAD~1 Cargo.lock | head -n 20
```

**Soluciones:**
```yaml
# Opción 1: Cache por rama
key: ${{ runner.os }}-${{ github.ref }}-cargo-${{ hashFiles('**/Cargo.lock') }}

# Opción 2: Cache más agresivo
cache-hit-depth: 10  # Allow partial hits

# Opción 3: No cachear target/ si es muy grande
path: |
  ~/.cargo/registry
  ~/.cargo/git
  # NO: target
```

---

### 5. Clippy FFI Warnings

**Error:**
```
warning: FFI types `*mut ...` should not have `unsafe` in the function signature
```

**Solución:**
```rust
// ❌ Mal
pub unsafe fn ffi_function(ptr: *mut c_void) {
    // ...
}

// ✅ Bien
pub fn ffi_function(ptr: *mut c_void) {
    unsafe {
        // FFI call aquí
    }
}
```

**En el workflow:**
```yaml
- name: Lint FFI code with clippy
  run: cargo clippy --all-targets -- \
    -W clippy::ffi_safe_defined_types \
    -W clippy::unsafe_ffi_bindings \
    -D warnings
```

---

### 6. Cargo-Audit Falla

**Error:**
```
error: 1 security vulnerability found
```

**Diagnóstico:**
```bash
cargo audit --json | jq '.vulnerabilities[] | {advisory: .advisory, severity: .metadata.severity}'
```

**Soluciones:**

Opción 1: Upgrade dependency
```bash
cargo update package-name
```

Opción 2: Usar advisory db más nuevo
```bash
cargo audit --fetch 2>&1
```

Opción 3: Allow específico (último recurso)
```bash
cargo audit --ignore RUSTSEC-2024-XXXX
```

---

### 7. Mocks Detectados en FFI

**Error:**
```
ERROR: Found mocks/stubs in FFI code
```

**Solución:**
```bash
# Encontrar mocks
grep -r "mock\|stub\|todo!" src/go_integration.rs

# Reemplazar con implementación real
vim src/go_integration.rs
```

Palabras prohibidas en FFI:
```
❌ mock()
❌ stub()
❌ todo!()
❌ unimplemented!()
```

---

### 8. Dependency Tree Explosión

**Síntoma:** Workflow muy lento, muchas dependencias

**Análisis:**
```bash
cargo tree --duplicates | wc -l
cargo tree --depth 1 | grep -v "├──"
```

**Soluciones:**
```bash
# Buscar bloat
cargo bloat --release

# Remover deps no usadas
grep -r "^use " src/ | grep -v "std::\|crate::" | sort | uniq

# Usar default-features = false si es posible
[dependencies]
serde = { version = "1.0", default-features = false }
```

---

### 9. Performance Regression

**Error:**
```
Binary size increased from 45 MB to 52 MB
```

**Análisis:**
```bash
# Comparar binarios
ls -lh target/release/nuclear_mcp{,.old}

# Analizar símbolos
cargo bloat --release -n 20

# Optimizar
# En Cargo.toml:
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

---

### 10. PR Comment No Se Postea

**Síntoma:** Job `generate-dependency-report` pasa pero no hay comment

**Solución:**

Verificar permisos:
```yaml
permissions:
  contents: read
  issues: write        # ← Required for comments
  pull-requests: write # ← Required for PR comments
```

Chequear condición:
```yaml
- name: Comment PR
  if: github.event_name == 'pull_request'  # ← Usar este exacto
  uses: actions/github-script@v7
  with:
    script: |
      github.rest.issues.createComment({
        issue_number: context.issue.number,
        owner: context.repo.owner,
        repo: context.repo.repo,
        body: 'Report: ...'
      })
```

---

## 🎯 Diagnóstico Rápido

### Script de diagnóstico local

```bash
#!/bin/bash
echo "=== Quick Diagnosis ==="

echo "1. Tool count:"
grep -c '"name":' src/mcp/protocol.rs

echo "2. Mocks in FFI:"
grep -r "mock\|stub" src/*_integration.rs || echo "None found ✅"

echo "3. Build test:"
cargo build --release 2>&1 | tail -n 3

echo "4. Audit:"
cargo audit --deny warnings 2>&1 | tail -n 3

echo "5. Format:"
cargo fmt -- --check 2>&1 | tail -n 1

echo "6. Clippy:"
cargo clippy --all-targets -- -D warnings 2>&1 | tail -n 3
```

Guardar en `scripts/quick_diagnosis.sh` y ejecutar:
```bash
chmod +x scripts/quick_diagnosis.sh
./scripts/quick_diagnosis.sh
```

---

## 📊 Metricas de Éxito

### Después de implementar workflows:

✅ **FFI Validation**
- [ ] Exactamente 5 tools reportados
- [ ] 0 mocks en FFI code
- [ ] Linux build success
- [ ] Windows build attempted (ok si fail)
- [ ] 0 vulnerabilidades conocidas

✅ **Dependency Analysis**
- [ ] 0 CVEs encontrados
- [ ] Features no usadas identificadas (si existen)
- [ ] Tree de deps lineal (sin explosión)
- [ ] Licenses compatibles

✅ **Performance**
- [ ] Build time < 10 minutos
- [ ] Binary size < 100 MB
- [ ] Cache hit rate > 70%
- [ ] No memory leaks en tests

---

## 🔍 Debugging Workflow en Tiempo Real

### Habilitar debug logs

```yaml
- name: Enable debug
  run: |
    export RUST_LOG=debug
    export RUST_BACKTRACE=full
  env:
    ACTIONS_STEP_DEBUG: 'true'
```

### Ver variables de contexto

```yaml
- name: Debug context
  run: |
    echo "Event: ${{ github.event_name }}"
    echo "Ref: ${{ github.ref }}"
    echo "SHA: ${{ github.sha }}"
    echo "Actor: ${{ github.actor }}"
```

### SSH access (para self-hosted runners)

```yaml
- name: Setup SSH for debugging
  if: failure()
  uses: mxschmitt/action-tmate@v3
  with:
    limit-access-to-actor: true
    timeout-minutes: 15
```

---

## 📈 Evolución Esperada

### Semana 1
- Workflows ejecutan sin errores
- PRs tienen validación completa
- Algunos jobs pueden timeout

### Semana 2-3
- Cache hit rate > 50%
- Performance stabiliza
- Pocos false positives

### Mes 1+
- Cache hit rate > 70%
- Zero-overhead validation
- Metrics dashboard activo

---

## 💡 Tips Avanzados

### 1. Reutilizar workflows

```yaml
# En .github/workflows/shared-ffi.yml
on:
  workflow_call:
    inputs:
      platforms:
        type: string
        default: 'linux,windows'

jobs:
  ffi-test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: ${{ fromJson(inputs.platforms) }}
```

Luego en otro workflow:
```yaml
jobs:
  call-shared:
    uses: ./.github/workflows/shared-ffi.yml
    with:
      platforms: '["ubuntu-latest", "windows-latest"]'
```

### 2. Artefactos condicionales

```yaml
- name: Upload on failure
  if: failure()
  uses: actions/upload-artifact@v4
  with:
    name: debug-logs
    path: |
      cargo-build.log
      ffi-test.log
```

### 3. Enviar notificaciones personalizadas

```yaml
- name: Notify on vulnerability
  if: |
    steps.audit.outcome == 'failure' ||
    contains(steps.audit.outputs.result, 'vulnerability')
  uses: 8398a7/action-slack@v3
  with:
    status: ${{ job.status }}
    text: '⚠️ Security vulnerability detected!'
    webhook_url: ${{ secrets.SLACK_WEBHOOK }}
```

---

**Documento creado:** 2026-01-23  
**Versión:** 1.0  
**Status:** ✅ Ready for Use
