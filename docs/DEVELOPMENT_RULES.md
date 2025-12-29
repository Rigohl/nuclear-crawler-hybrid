# Copilot Instructions for Nuclear Crawler Hybrid

## Project Overview

**NUCLEAR CRAWLER HYBRID** is a high-performance Rust MCP (Model Context Protocol 2025-06-18) server providing massively parallel web search and analysis. It implements 2 primary tools and 11 integrated modules with FFI integration (Go, Zig, Nim, JAX) for extreme performance.

**Key Stats**: 100K goroutines, 55 search sources, 2-second search completion, 2,100+ URLs per query.

---

## Architecture & Components

### Core MCP Server (`src/bin/nuclear_ultimate.rs`)
- **Protocol**: HTTP only (Axum on :8079)
- **2 Main Tools**: websearch, file_search
- **Async Runtime**: Tokio with full feature set
- **Main struct** `SearchEngine` holds all 11 modules as fields

### 11 Integrated Modules (Reference `src/lib.rs`)
```
Core: web_search, file_search, nuclear_core, url_helpers
FFI: go_integration, zig_integration, nim_integration, jax_integration
Infrastructure: intelligent_storage, cache, rate_limit
```

### FFI Integration Pattern
- **Go FFI** (`go/src/stealth_go.go`): 100K parallel goroutines, stealth headers
- **Zig SIMD** (`zig/src/lib.zig`): Fast hashing, SIMD parsing via `libloading`
- **Nim HTML** (`src/nim_integration.rs`): Alternative HTML parsing
- **JAX Acceleration** (`src/jax_integration.rs`): Vectorized batch processing via Python FFI
- **Nim HTML** (`src/lib.rs` wrapper): Alternative HTML parsing
- **JAX Acceleration** (`scripts/jax_pipeline.py`): Vectorized batch processing

---

## Critical Patterns & Conventions

### Error Handling
- **Return Type**: `Result<Value>` where Value = `serde_json::Value`
- **Never propagate .unwrap()** in tool handlers - use pattern matching or `.ok()`
- **Example**: `src/core_tools.rs` lines 352-394 show correct tool error handling

### Tool Implementation (4 Required Patterns)

**1. WebSearch** (`tool_websearch()`):
- Input: `{"queries": ["term1", "term2"], ...}` (max 5 queries)
- **Uses ALL 11 modules**: Core, FFI (Go/Zig/Nim/Jax), Bypass (NuclearBypass, StealthSystem), Infra (Storage, Cache, RateLimiter), Utils for maximum power
- Returns 2,100+ URLs from 55 sources in parallel, including **premium content scraping** from Medium.com, ArXiv, research papers, and other high-value sources
- **Full Crawler Scraping**: Extracts real content, not just URLs; uses stealth headers, nuclear bypass for premium sites
- **Storage**: All results saved to `resultados/` folder with timestamps and metadata
- Uses rate limiter + cache checks before execution
- **Key Files**: `src/web_search.rs`, `src/massive_parallel_search.rs`, `src/premium_content_scraper.rs`

**2. FileSearch** (`tool_file_search()`):
- Input: `{"search_term": "pattern", "path": "./src"}`
- **Uses Zig SIMD for ultra-fast pattern matching**
- Returns results with **exact line numbers and context** where errors or patterns are found
- **Purpose**: Identify precise locations of bugs, errors, or specific code issues with line-by-line accuracy
- **Advanced Features**: Semantic search, fuzzy matching, code duplication detection, circular import detection, function complexity analysis, automatic edit suggestions
- **Key Files**: `src/file_search.rs`

### Tokio Async Patterns
- **All network I/O**: Use `tokio::spawn()` for concurrent tasks
- **Timeouts**: Always wrap external calls with `tokio::time::timeout(Duration::from_secs(N), ...)`
- **Rate Limiting**: Call `self.rate_limiter.acquire().await` before bulk operations
- **Example**: Lines 580-630 in nuclear_ultimate.rs show proper timeout + rate limit usage

### Result Building Pattern
```rust
let result = json!({
    "status": "success",
    "count": result_count,
    "data": serializable_data,
    "execution_ms": start.elapsed().as_millis(),
    "modules_used": module_count,
});
Ok(result)
```

---

## Build & Deployment

### Compilation
```bash
# Development (faster, unoptimized)
cargo build

# Release (optimized: 3x opt-level, LTO, single codegen unit)
cargo build --release
```

**Profile Config** (`Cargo.toml [profile.release]`):
- `opt-level = 3` + `lto = "fat"` + `codegen-units = 1`
- Binary location: `target/release/nuclear-mcp.exe` (~20-25MB)

### Execution Modes
```bash
# HTTP Server (localhost:8079)
./target/release/nuclear-mcp --mode http --port 8079
```

---

## Common Development Tasks

### Running Single Tool for Testing
```bash
# After cargo build, use curl for HTTP mode:
curl -X POST http://localhost:8080/call \
  -H "Content-Type: application/json" \
  -d '{"name": "websearch", "arguments": {"queries": ["rust async"]}}'
```

### Adding a New Module
1. Create `src/module_name.rs` with public struct implementing core trait
2. Add field to `SearchEngine` struct in `nuclear_ultimate.rs`
3. Initialize in `SearchEngine::new()` (line ~300)
4. Reference in appropriate tool handler

### Fixing Compilation Errors
- **Brace mismatches**: Check lines around error - Rust's error reporting points to where mismatch is detected, not where cause is
- **FFI issues**: Verify `libloading` can find `.dll`/`.so` files - check `path` in FFI struct constructors
- **Timeout errors**: Increase timeout duration in `tokio::time::timeout(Duration::from_secs(X), ...)`

---

## Key File Reference

| File | Purpose | Key Lines |
|------|---------|-----------|
| `src/lib.rs` | Module declarations + SearchEngine struct | 1-150 |
| `src/bin/nuclear_ultimate.rs` | MCP server + 4 tool handlers | 300-1800 |
| `src/core_tools.rs` | HTTP endpoint tool dispatchers | 100-450 |
| `src/simple_mcp.rs` | HTTP MCP protocol handler | 1000-1200 |
| `Cargo.toml` | Metadata: tools, modules, FFI config | [package.metadata.*] |
| `src/web_search.rs` | Core search implementation | - |
| `src/file_search.rs` | Local file pattern search (Zig SIMD) | - |
| `go/src/stealth_go.go` | Go parallelism via FFI | - |
| `zig/src/lib.zig` | SIMD hash + parsing | - |

---

## Rust Development Standards

### Fundamental Rules
- **NEVER use mocks or simulations** - all code must be real and functional
- **NEVER assume dependencies** - read `Cargo.toml` for actual versions and features
- **ALWAYS verify with real compilation** - use `cargo check --all-targets 2>&1`
- **ALWAYS search for connected files** before changing code
- **ALWAYS handle Result and Option** explicitly - no unwrap() without justification
- **ZERO WARNINGS ALLOWED** - All compiler warnings must be fixed before any commit or deployment
- **EVERYTHING MUST COMPILE** - No errors, no warnings; verify with `cargo build --release` after changes

### Before Any Code Change
1. Run `cargo check --all-targets 2>&1` to verify current state
2. Read `Cargo.toml` for actual versions and available features
3. Use workspace search to find ALL usages of affected code
4. Verify changes don't break other files

### Code Style
- `snake_case` for functions and variables
- `PascalCase` for types, traits, enums
- `SCREAMING_SNAKE_CASE` for constants
- Prefer `&str` over `String` in parameters
- Prefer `impl Trait` over `dyn Trait` when possible
- **Maximum 100 characters per line**

### Error Handling
- Use `Result<T, E>` for fallible operations
- Use `thiserror` for custom errors in libraries
- Use `anyhow` in applications, `thiserror` in libraries
- **NEVER use `.unwrap()` without explicit justification**
- **NEVER use `.expect()` without descriptive message**
- Propagate errors with `?` when appropriate

### Ownership & Borrowing
- Prefer borrowing (`&T`, `&mut T`) over cloning
- Use `Clone` only when necessary
- Avoid unnecessary lifetime annotations
- Understand why compiler requests lifetimes before adding them

### Concurrency (Tokio)
- Prefer channels (`mpsc`) over shared mutexes
- Use `Arc<Mutex<T>>` only when necessary
- Always verify Tokio version and features in `Cargo.toml`
- Never mix async runtimes

### FFI Integration (Go, Zig, Nim)
- `extern "C"` must match exactly with external language definitions
- Use C-compatible types: `i32`, `u32`, `f32`, `*const`, `*mut`
- **Document all unsafe code thoroughly**
- Encapsulate unsafe in safe abstractions
- FFI can be optimized for any language when performance-critical

### Testing
- Use real data, never mock data
- Descriptive names: `test_should_X_when_Y`
- One assert per test when possible
- Integration tests for complete workflows

### Dependencies
- Read `Cargo.toml` before proposing code changes
- Use API compatible with the version the project has
- Don't add dependencies without justification
- Verify feature compatibility

### When Errors Occur
1. Read the complete compiler error message
2. Identify exact file and line number
3. Search what other files use this code
4. Propose minimal fix that doesn't break anything
5. Verify with `cargo check`

---

## Prohibited Practices

❌ **ABSOLUTELY FORBIDDEN**:
- Mocks of any kind
- Simulated or hardcoded test data
- Ignoring compiler warnings - **ALL warnings must be fixed**
- **ZERO dead code or stubs allowed** - all code must be functional
- Unsafe code without documentation
- `.unwrap()` without justification
- `.expect()` without clear error message
- Assuming crate versions - read `Cargo.toml` first

---

## Project-Specific Gotchas & Anti-Patterns

❌ **DO NOT**:
- Call external commands in `analyzer` (was main bug Dec 5-11) - use filesystem APIs instead
- Unwrap Results in tool handlers - wrap in `json!({"error": msg})`
- Block async code with `.block_on()` - use `.await`
- Ignore rate limits - always acquire before bulk ops

✅ **DO**:
- Use `eprintln!()` for debug output (redirects to stderr in HTTP mode)
- Cache results in `memory_cache: DashMap` before returning
- Apply Stealth headers from `self.stealth_system.get_headers()` to requests
- Parallelize with Tokio, not manual thread spawning
## Performance Optimization Best Practices

### General Principles
- **Measure First, Optimize Second:** Always profile and measure before optimizing. Use benchmarks, profilers, and monitoring tools to identify real bottlenecks.
- **Optimize for the Common Case:** Focus on optimizing code paths that are most frequently executed.
- **Avoid Premature Optimization:** Write clear, maintainable code first; optimize only when necessary.
- **Minimize Resource Usage:** Use memory, CPU, network, and disk resources efficiently.
- **Prefer Simplicity:** Simple algorithms and data structures are often faster and easier to optimize.

### Frontend Performance (if applicable)
- **Minimize DOM Manipulations:** Batch updates where possible.
- **Virtual DOM Frameworks:** Use React, Vue, or similar efficiently—avoid unnecessary re-renders.
- **Asset Optimization:** Compress images, minify JS/CSS, use modern formats (WebP, AVIF).
- **Lazy Loading:** Use `loading="lazy"` for images, dynamic imports for JS.
- **Caching:** Set long-lived cache headers for static assets.

### Backend Performance
- **Efficient Algorithms:** Use binary search, quicksort, or hash-based algorithms where appropriate.
- **Concurrency and Parallelism:** Use async/await, threads, or goroutines for I/O-bound tasks.
- **Caching:** Use in-memory caches (Redis, Memcached) for hot data.
- **Database Optimization:** Use indexes, avoid SELECT *, paginate large result sets.
- **API Optimization:** Minimize payloads, use compression, connection pooling.

### Database Performance
- **Query Optimization:** Use indexes, parameterized queries, avoid N+1 queries.
- **Schema Design:** Normalize to reduce redundancy, but denormalize for read-heavy workloads.
- **Transactions:** Keep transactions short to reduce lock contention.
- **Caching and Replication:** Use read replicas, cache query results.

### Code Review Checklist for Performance
- [ ] Are there any obvious algorithmic inefficiencies?
- [ ] Are data structures appropriate for their use?
- [ ] Are there unnecessary computations or repeated work?
- [ ] Is caching used where appropriate?
- [ ] Are database queries optimized?
- [ ] Are large payloads paginated or streamed?
- [ ] Are there any memory leaks?
- [ ] Are network requests minimized and batched?

---

## Security & OWASP Guidelines

### A01: Broken Access Control & A10: SSRF
- **Enforce Principle of Least Privilege:** Default to most restrictive permissions.
- **Deny by Default:** Access granted only if explicitly allowed.
- **Validate URLs for SSRF:** Treat user-provided URLs as untrusted; use allow-list validation.

### A02: Cryptographic Failures
- **Use Strong Algorithms:** Argon2 or bcrypt for hashing passwords.
- **Protect Data in Transit:** Default to HTTPS.
- **Protect Data at Rest:** Use AES-256 encryption.
- **Secure Secret Management:** Never hardcode secrets; use environment variables or secret stores.

### A03: Injection
- **Parameterized Queries:** Use prepared statements for database interactions.
- **Sanitize Inputs:** Prevent command injection and XSS.
- **Context-Aware Encoding:** Use proper output encoding for different contexts.

### A05: Security Misconfiguration & A06: Vulnerable Components
- **Secure Defaults:** Disable verbose error messages in production.
- **Security Headers:** Add CSP, HSTS, X-Content-Type-Options.
- **Update Dependencies:** Regularly scan for vulnerabilities (npm audit, Snyk).

### A07: Identification & Authentication Failures
- **Secure Sessions:** Regenerate session IDs after login; use HttpOnly, Secure, SameSite cookies.
- **Rate Limiting:** Implement for authentication and password reset flows.

### A08: Software Integrity Failures
- **Validate Deserialization:** Avoid insecure deserialization; use JSON over Pickle.

### General Security Guidelines
- **Be Explicit About Security:** Explain mitigations (e.g., "Using parameterized query to prevent SQL injection").
- **Educate During Reviews:** Identify vulnerabilities and explain risks.

---

## Python Coding Conventions (if applicable)

- Write clear comments for each function.
- Ensure functions have descriptive names and type hints.
- Provide docstrings following PEP 257.
- Use `typing` module for annotations (e.g., `List[str]`, `Dict[str, int]`).
- Break down complex functions into smaller ones.
- Follow PEP 8: 4 spaces indentation, <79 chars lines.
- Handle edge cases and exceptions gracefully.
- Write unit tests for critical paths.

---

## Go Development Instructions (if applicable)

- Write simple, clear, idiomatic Go code.
- Favor clarity and simplicity.
- Return early to reduce nesting.
- Document exported types, functions, methods.
- Use Go modules for dependency management.
- Follow naming conventions: lowercase packages, MixedCaps for exported.
- Handle errors immediately after calls.
- Use `Result<T, E>` for fallible operations; prefer `?` for propagation.
- Use goroutines and channels for concurrency.
- Profile with `pprof` for performance issues.

---

## Memory Bank & Task Management

### Memory Bank Structure
- **Core Files:** projectbrief.md, productContext.md, activeContext.md, systemPatterns.md, techContext.md, progress.md, tasks/ folder.
- **Workflow:** Read ALL memory bank files at start of tasks. Update after significant changes.
- **Task Management:** Use tasks/_index.md for tracking; individual files for details.

### Task Implementation Process
- Read plan and details completely before implementing.
- Implement systematically, one task at a time.
- Update changes file after each task: Added, Modified, Removed sections.
- Mark tasks complete [x] in plan; add Release Summary at end.

### Success Criteria
- All plan tasks complete [x].
- All files created/updated with working code.
- Changes file updated after every task.

---

## Spec-Driven Workflow

- Implement based on specifications.
- Validate against spec requirements.
- Document divergences from plan with reasons.

---

## DevOps Core Principles (CALMS Framework)

### Culture (C)
- Foster collaborative, blameless culture.
- Shared responsibility, trust, continuous learning.
- Feedback loops, cross-functional teams.

### Automation (A)
- Automate CI/CD pipelines, IaC, testing, monitoring.
- Infrastructure as Code (Terraform, Ansible).
- Automated testing: unit, integration, security scans.

### Lean (L)
- Eliminate waste, maximize flow, deliver value continuously.
- Value stream mapping, just-in-time delivery.

### Measurement (M)
- Track DORA metrics: Deployment Frequency, Lead Time for Changes, Change Failure Rate, Mean Time to Recovery.
- Monitoring, logging, dashboards.

### Sharing (S)
- Knowledge sharing, documentation, communication channels.
- Pair programming, internal meetups.

---

## AI Prompt Engineering & Safety Best Practices

### Prompt Engineering Fundamentals
- **Clarity, Context, Constraints:** Be explicit, provide background, specify formats.
- **Patterns:** Zero-shot, Few-shot, Chain-of-Thought, Role Prompting.
- **Anti-patterns:** Avoid ambiguity, verbosity, prompt injection, overfitting.

### Safety & Bias Mitigation
- **Red-teaming:** Test for harmful/biased outputs.
- **Mitigation:** Inclusive language, moderation APIs, human review.
- **Responsible AI:** Transparency, explainability, data privacy.

### Security
- **Prevent Injection:** Sanitize inputs, validate URLs.
- **Data Leakage:** Avoid echoing sensitive data.
- **Compliance:** Follow Microsoft/Google/OpenAI principles.

### Testing & Validation
- **Automated Evaluation:** Define metrics (accuracy, safety, consistency).
- **Human Review:** Peer review, feedback cycles.
- **Continuous Improvement:** Monitor, update prompts.

### Templates
- **Prompt Design Checklist:** Task definition, context, constraints, examples, safety.
- **Safety Review Checklist:** Content safety, bias, security, compliance.

---

## REGLAS ESENCIALES PARA NUCLEAR CRAWLER HYBRID (ENFOQUE EN REPARAR, RUST, MÓDULOS, ANÁLISIS, ERRORES Y MCP AXUM 2025)

### Arquitectura y Módulos Esenciales
- **Usa TODOS los 42 módulos integrados:** Core (WebSearch, FileSearch), FFI (Go, Zig, Nim, Jax, Mojo), Bypass (NuclearBypass, StealthSystem), Infra (Storage, Cache, RateLimiter), Utils. Nunca omitas ninguno en implementaciones. **Web Search usa TODOS para scraping premium y almacenamiento.**
- **Análisis completo obligatorio:** Para `analyzer`, usa solo filesystem APIs (no comandos externos); clasifica archivos por extensión; analiza recursivamente con `fs::read_dir`.
- **MCP Protocolo HTTP Axum 2025:** HTTP-only protocolo (puerto :8079). Herramientas: websearch (max 5 queries, 2100+ URLs), file_search (Zig SIMD con análisis avanzado de errores y warnings). Usa Tokio async, timeouts, rate limits. Result: `json!({"status": "success", ...})`. Nunca unwrap en handlers.

### Estándares Rust para Reparar y Uso Completo
- **Manejo de errores robusto:** Usa `Result<T, E>` y `?`; nunca `.unwrap()` sin justificación. Propaga con contexto. Valida inputs/outputs. Loggea con `eprintln!` en HTTP mode.
- **Workflow para reparar:** Diagnostica con logs/stack traces. Fix mínimo verificado. Rollback plan. Documenta reparaciones. Monitorea post-fix.
- **Uso de módulos completo:** Inicializa TODOS en `SearchEngine::new()`. Referencia en tool handlers. Verifica con `cargo check --all-targets`.
- **FFI y concurrencia:** Go/Zig/Nim via `libloading`. Tokio async con `spawn()`, timeouts, rate limiter. No blocking en hot paths.
- **Prohibido:** Mocks/simulaciones. Dead code/stubs. Unsafe sin doc. `.unwrap()`/`.expect()` sin motivo. Ignorar warnings.

### Patrones Críticos para Análisis y Reparación
- **Error handling:** `Result<Value>` en tools; pattern matching, no unwrap. Timeout: `tokio::time::timeout(Duration::from_secs(N), ...)`.
- **Rate limiting:** `self.rate_limiter.acquire().await` antes de bulk ops. Cache con `DashMap`.
- **Build/Release:** `cargo build --release` (opt-level 3, LTO, codegen-units 1). Binary ~20-25MB.
- **Testing:** Real data, no mocks. Unit/integration tests. `cargo test`.
- **Reparar bugs:** Verifica `Cargo.toml` versiones. Search ALL usages. Fix mínimo, test. No dead code.

### Reglas Esenciales para Reparar y Organizar
- **Diagnostica antes de reparar:** Logs, métricas, causa raíz.
- **Repara mínimo:** Fix pequeño, verifica con pruebas.
- **Usa todos módulos:** Nunca omitas FFI o infra en análisis.
- **Manejo errores:** Result/Option, contexto, logging, recuperación.
- **Organiza código:** Modular, nombres descriptivos, documentación.
- **Evita ediciones innecesarias:** Solo si bug probado o mejora medida.
- **Análisis completo:** Filesystem APIs, clasificación por extensión, recursivo.
- **MCP Axum:** HTTP-only, 2 tools, async, timeouts, rate limits.
- **Verifica compatibilidad:** `cargo check`, no warnings.
- **Compila sin warnings:** Siempre `cargo build --release` sin errores ni warnings.
- **Documenta cambios:** Por qué, cómo, impacto.

### Lo Faltante Añadido
- **Tokio patterns:** `spawn()` para concurrente, `timeout()` para límites, `rate_limiter` para control.
- **Result building:** `json!({"status": "success", "data": ..., "execution_ms": ...})`.
- **FFI integration:** Go (100K goroutines), Zig (SIMD hashing), Nim (HTML parsing), Jax (batch processing).
- **Stealth features:** Headers from `self.stealth_system`, cache checks.
- **Deployment:** HTTP server `--mode http --port 8079`.
- **Performance:** Optimizado para 2s search, 2100+ URLs, 100K goroutines.
- **Security:** No hardcoded secrets, validate URLs, sanitize inputs.
- **Versionado:** Semantic, features flags, `cargo mod tidy`.
- **Debugging:** `eprintln!` para logs, flame graphs para perf, heap dumps para leaks.

---

## Plantillas y Directrices Recomendadas

Se agregan plantillas para facilitar la creación y aplicación de reglas y tareas:

- Directriz Focalizada: `.github/instructions/Directriz_Focalizada.instructions.md` — plantilla para crear reglas específicas por `glob` (ej.: reglas sólo para SQL, sólo para archivos de CI, etc.).
- Plantilla de Tarea (Prompt): `.github/prompts/Plantilla_de_Tarea.prompt.md` — plantilla para definir prompts de invocación manual (tareas repetitivas y estructuradas).

Coloca reglas específicas en `.github/instructions/` y prompts de invocación manual en `.github/prompts/`. Las reglas automatizables deben incluir un campo `implementation` con comando o script de verificación para CI.

Ejemplo de flujo recomendado:
1. Crear `Directriz_Focalizada` para la regla (con `applies_to` como glob).
2. Añadir verificación automática en CI (si es posible).
3. Para tareas manuales o ad-hoc, usar una entrada en `.github/prompts/` y lanzarla desde una Issue o herramienta interna.

---

## Próximos pasos realizados por el asistente
- Se han creado las plantillas en el repositorio (`.github/instructions/Directriz_Focalizada.instructions.md` y `.github/prompts/Plantilla_de_Tarea.prompt.md`).
- Se han guardado en memoria (`memoryr`) para referencia y uso por agentes.

Si quieres, puedo:
- Ejecutar un chequeo automático (local) que liste archivos que coinciden con ejemplos de `Directriz_Focalizada`.
- Generar una `Directriz_Focalizada` real para SQL o para Rust basada en las reglas de este documento.
