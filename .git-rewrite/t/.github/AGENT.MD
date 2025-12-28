---
description: "Contexto .github - GitHub Automation y Workflows para Nuclear Crawler Hybrid"
agent: "GitHub_Automation"
capabilities: ["ci_cd", "code_quality", "release_management", "issue_tracking"]
triggers: ["pull_request", "push", "issue_opened", "release_created"]
---

# .GITHUB - CONTEXTO DE CARPETA PARA NUCLEAR CRAWLER HYBRID

## DESCRIPCIÓN GENERAL
Esta carpeta contiene toda la automatización de GitHub para NUCLEAR CRAWLER HYBRID, incluyendo workflows de CI/CD, plantillas de issues/PRs, y configuraciones de dependabot. Utiliza TRAE CLI por defecto para análisis avanzado de código Rust.

## CONFIGURACIÓN DE PUERTOS

### Servicios y Puertos
- **JARVIXSERVER**: Puerto 5050 (`http://localhost:5050`)
- **TRAE CLI Server**: Puerto interno 3001, API expuesta en 8080
- **Nuclear MCP Server**: Puerto 8079

### Variables de Entorno TRAE CLI
```bash
export JARVIX_URL=http://localhost:5050  # Conexión a JARVIXSERVER
```

## ARQUITECTURA DE GITHUB AUTOMATION

### Workflows CI/CD

### CI Principal (ci.yml)
- **Triggers**: Push a main, PRs
- **Jobs**:
  - Test en múltiples plataformas (Linux, macOS, Windows)
  - Análisis con TRAE CLI (trae fmt, trae clippy --strict, trae test)
  - Validación de código híbrido (Rust + FFI)
  - Coverage reporting
  - Release builds con trae build --release
- **TRAE CLI Recomendado**: Para análisis avanzado local usar `trae repair` y `trae clippy --strict`

#### Release Workflow (release.yml)
- **Triggers**: Tags de versión
- **Jobs**:
  - Build para múltiples targets
  - Compilación FFI (Go, Zig, Nim)
  - Crear releases en GitHub
  - Generar changelogs

#### Security Scan (security.yml)
- **Triggers**: Schedule semanal, dependabot PRs
- **Jobs**:
  - TRAE CLI análisis de código (clippy --strict --pedantic)
  - Cargo audit para vulnerabilidades de dependencias
  - CodeQL analysis
  - Dependency scanning

### Estructura de Archivos
```
.github/
├── workflows/
│   ├── ci.yml              # CI principal con TRAE CLI
│   ├── release.yml         # Releases automáticos con TRAE CLI
│   ├── security.yml        # Escaneos de seguridad con TRAE CLI
│   └── dependabot.yml      # Actualizaciones de dependencias
├── ISSUE_TEMPLATE/
│   ├── bug_report.md       # Reporte de bugs
│   ├── feature_request.md  # Solicitud de features
│   └── security.md         # Reportes de seguridad
├── PULL_REQUEST_TEMPLATE.md # Template para PRs
├── CODEOWNERS             # Code owners
├── dependabot.yml         # Configuración dependabot
└── FUNDING.yml           # Configuración de funding
```

## CI WORKFLOW PRINCIPAL CON TRAE CLI

### Configuración CI
```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]

    steps:
    - uses: actions/checkout@v4

    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        toolchain: ${{ matrix.rust }}

    - name: Cache dependencies
      uses: Swatinem/rust-cache@v2

    - name: Check formatting
      run: cargo fmt --all -- --check

    - name: Run clippy
      run: cargo clippy -- -D warnings

    - name: Run tests
      run: cargo test --all-targets

    - name: Run integration tests
      run: cargo test --test integration

    - name: Generate coverage
      if: matrix.os == 'ubuntu-latest' && matrix.rust == 'stable'
      uses: codecov/codecov-action@v3
```

**Nota**: Para análisis avanzado local, se recomienda usar TRAE CLI:
```bash
trae repair                    # Análisis y reparación automática
trae clippy --strict          # Verificación estricta de calidad
trae security --audit         # Análisis de seguridad
```

### Optimizaciones de Performance
```yaml
# Cache inteligente para CI
- name: Cache cargo registry
  uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

- name: Cache target directory
  uses: actions/cache@v3
  with:
    path: target
    key: ${{ runner.os }}-target-${{ hashFiles('**/Cargo.lock') }}

- name: Cache FFI libraries
  uses: actions/cache@v3
  with:
    path: libs/
    key: ${{ runner.os }}-libs-${{ hashFiles('go/**', 'zig/**', 'nim/**') }}
```

## RELEASE AUTOMATION

### Workflow de Release
```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*.*.*'

jobs:
  release:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Build release
      run: cargo build --release

    - name: Run tests
      run: cargo test --release

    - name: Compile FFI libraries
      run: |
        ./scripts/compile_go_msvc.ps1
        ./scripts/compile_zig_msvc.ps1
        ./scripts/compile_nim_msvc.ps1

    - name: Create GitHub release
      uses: softprops/action-gh-release@v1
      with:
        generate_release_notes: true
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

    - name: Publish to crates.io
      run: cargo publish --token ${{ secrets.CRATES_IO_TOKEN }}
```

**TRAE CLI para validación local antes de release**:
```bash
trae build --release          # Build optimizado con TRAE
trae security --audit         # Verificación de seguridad
trae test --release           # Tests en modo release
```

### Versionado Automático
```yaml
# Configuración para versionado semántico
- name: Bump version
  uses: paulhatch/semantic-version@v5
  with:
    tag_prefix: "v"
    major_pattern: "(MAJOR)"
    minor_pattern: "(MINOR)"
    format: "${major}.${minor}.${patch}"
```

## SECURITY SCANNING

### Security Integration
```yaml
# .github/workflows/security.yml
name: Security Scan

on:
  schedule:
    - cron: '0 0 * * 0'  # Weekly
  pull_request:
    paths:
      - '**/Cargo.lock'
      - '**/Cargo.toml'

jobs:
  audit:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Run cargo-audit
      run: cargo audit

    - name: CodeQL Analysis
      uses: github/codeql-action/init@v2
      with:
        languages: rust

    - name: Perform CodeQL Analysis
      uses: github/codeql-action/analyze@v2

    - name: Dependency Check
      uses: dependency-check/Dependency-Check_Action@main
      with:
        project: 'Nuclear Crawler Hybrid'
        path: '.'
        format: 'ALL'
```

**TRAE CLI recomendado para análisis de seguridad local**:
```bash
trae security --audit         # Análisis completo de seguridad
trae repair                   # Reparación automática de vulnerabilidades
```

## DEPENDABOT CONFIGURATION

### Actualizaciones Automáticas
```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 10
    reviewers:
      - "team/core"
    assignees:
      - "maintainer"
    commit-message:
      prefix: "deps"
      include: "scope"

  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
```

## ISSUE TEMPLATES

### Bug Report Template
```markdown
# .github/ISSUE_TEMPLATE/bug_report.md
---
name: Bug report
about: Create a report to help us improve Nuclear Crawler Hybrid
title: "[BUG] "
labels: bug
assignees: ''

---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Environment:**
 - OS: [e.g. Windows 11]
 - Rust version: [e.g. 1.75.0]
 - Nuclear Crawler Hybrid version: [e.g. 1.0.0]
 - FFI Libraries: [Go/Zig/Nim versions]

**Additional context**
Add any other context about the problem here.
```

### Feature Request Template
```markdown
# .github/ISSUE_TEMPLATE/feature_request.md
---
name: Feature request
about: Suggest an idea for Nuclear Crawler Hybrid
title: "[FEATURE] "
labels: enhancement
assignees: ''

---

**Is your feature request related to a problem? Please describe.**
A clear and concise description of what the problem is. Ex. I'm always frustrated when [...]

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
A clear and concise description of any alternative solutions or features you've considered.

**Additional context**
Add any other context or screenshots about the feature request here.
```

## PULL REQUEST TEMPLATE

### Template para PRs
```markdown
# .github/PULL_REQUEST_TEMPLATE.md
## Description
Please include a summary of the changes and the related issue. Please also include relevant motivation and context.

## Type of change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Refactoring (no functional changes)
- [ ] FFI Integration (Go/Zig/Nim changes)

## Checklist
- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published in downstream modules
- [ ] FFI libraries compile successfully (if modified)

## Testing
Describe the tests that you ran to verify your changes.

## FFI Changes (if applicable)
- [ ] Go FFI updated
- [ ] Zig SIMD updated
- [ ] Nim HTML parser updated
- [ ] Libraries recompiled

## Screenshots (if appropriate)
Add screenshots to help explain your changes.

## Additional Notes
Add any other context about the pull request here.
```

## CODEOWNERS

### Configuración de Code Owners
```plaintext
# .github/CODEOWNERS
# Global owners
* @project-owner @core-team

# Core functionality
src/ @core-team
src/nuclear_core.rs @lead-developer

# FFI Integrations
go/ @ffi-team
zig/ @ffi-team
nim/ @ffi-team
libs/ @ffi-team

# Scripts and automation
scripts/ @devops-team
.github/ @devops-team

# Documentation
*.md @docs-team
README.md @docs-team
```

## HERRAMIENTAS RECOMENDADAS - TRAE CLI

### Comandos TRAE CLI Recomendados para Desarrollo Local
```bash
# Análisis y reparación automática del código
trae repair

# Verificación estricta de calidad de código
trae clippy --strict

# Análisis de seguridad completo
trae security --audit

# Verificación de dependencias
trae deps --check-updates

# Análisis específico para proyectos híbridos (FFI)
trae repair --hybrid
trae security --audit --ffi
```

### Beneficios de Usar TRAE CLI Localmente
- **Análisis Six Sigma**: Métricas DPMO para calidad de código
- **Detección de Seguridad**: Bloques unsafe, unwrap calls, panics
- **Auto-Reparación**: Sugerencias automáticas de mejora
- **Cache Inteligente**: Análisis rápido con .trae/cache
- **Integración JARVIXSERVER**: Capacidades MCP extendidas

**Nota**: TRAE CLI es una herramienta externa recomendada para desarrollo local. Los workflows de CI/CD usan comandos estándar de Cargo para mantener compatibilidad universal.

## TESTING DE WORKFLOWS

### Tests de CI Locales
```bash
# Ejecutar CI localmente con act
act -j test

# Ejecutar solo tests de Rust
act -j test --container-architecture linux/amd64

# Debug de workflows
act -j test --verbose
```

**Para análisis avanzado local, usar TRAE CLI**:
```bash
trae repair                    # Análisis completo antes de commit
trae test --all               # Tests con TRAE
```

### Validación de Templates
```rust
// tests/github_templates_test.rs
#[test]
fn test_issue_templates() {
    // Verify templates exist and are valid
    assert!(Path::new(".github/ISSUE_TEMPLATE/bug_report.md").exists());
    assert!(Path::new(".github/ISSUE_TEMPLATE/feature_request.md").exists());

    // Check template content for Nuclear Crawler Hybrid
    let bug_template = fs::read_to_string(".github/ISSUE_TEMPLATE/bug_report.md").unwrap();
    assert!(bug_template.contains("BUG"));
    assert!(bug_template.contains("FFI Libraries"));
}

#[test]
fn test_workflow_uses_trae() {
    // Validate YAML syntax of workflows and TRAE usage
    let ci_workflow: serde_yaml::Value = serde_yaml::from_str(
        &fs::read_to_string(".github/workflows/ci.yml").unwrap()
    ).unwrap();

    assert!(ci_workflow.get("jobs").is_some());
    assert!(ci_workflow.get("on").is_some());

    // Check for TRAE CLI usage
    let steps = ci_workflow.get("jobs").unwrap()
        .get("test").unwrap()
        .get("steps").unwrap();

    let trae_step = steps.as_sequence().unwrap().iter()
        .find(|step| step.get("name").unwrap().as_str().unwrap().contains("TRAE"));

    assert!(trae_step.is_some());
}
```

## MANEJO DE ERRORES

### Recuperación de Workflows
```bash
# Re-run failed workflow
gh workflow run ci.yml

# Ver logs detallados
gh run view <run-id> --log

# Debug local con TRAE
act --list
act -j test --artifact-server-path /tmp/artifacts
trae ci --simulate
```

### Troubleshooting
```yaml
# Workflow para debugging
name: Debug
on: workflow_dispatch

jobs:
  debug:
    runs-on: ubuntu-latest
    steps:
    - run: |
        echo "Debugging Nuclear Crawler Hybrid CI"
        rustc --version
        cargo --version
        trae --version
        echo "FFI Libraries:"
        ls -la libs/
        echo "Environment variables:"
        env | grep -E "(RUST|CARGO|TRAE)" | sort
```

## CONTEXTO DE DESARROLLO PARA NUCLEAR CRAWLER HYBRID

La carpeta .github automatiza todo el ciclo de desarrollo del NUCLEAR CRAWLER HYBRID, desde CI/CD con TRAE CLI hasta gestión de releases. Garantiza calidad de código híbrido, seguridad y distribución automática.

### Puntos de Contacto
- **CI/CD Pipeline**: Workflows ejecutan automáticamente análisis TRAE en pushes/PRs
- **Release Management**: Tags generan releases con compilación FFI automática
- **Security**: Escaneos semanales con TRAE CLI y análisis de dependencias
- **TRAE-CLI**: Comandos integrados para gestión de GitHub y análisis híbrido

### Mejores Prácticas para Proyecto Híbrido
- Mantener workflows modulares y reutilizables
- Usar caching para optimizar tiempos de CI (Rust + FFI)
- Configurar dependabot para actualizaciones seguras
- Documentar procesos de release claramente
- Monitorear costos de GitHub Actions
- Usar secrets para tokens sensibles
- Implementar branch protection rules
- Configurar required status checks
- Validar compilación FFI en todos los targets
- Mantener sincronización entre lenguajes (Go/Zig/Nim)
