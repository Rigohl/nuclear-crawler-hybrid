# 🔥 ARCHITECTURE - Nuclear Crawler Hybrid
**Advanced Distributed Intelligence Platform with Explicit Backend Contracts**

## System Overview

### Core Runtime Architecture

```
┌─────────────────────────────────────────────────────────┐
│         HTTP JSON-RPC 2.0 Server (Port 8079)           │
└──────────────────┬──────────────────────────────────────┘
                   │
          ┌────────▼────────┐
          │ 7 MCP Tools     │
          │ Independent     │
          │ Composable      │
          └────────┬────────┘
                   │
    ┌──────────────▼──────────────────────────────┐
    │ 37 Internal Modules                         │
    │ Core | FFI | WASM | OSINT | Infra | AI      │
    └──────────────┬──────────────────────────────┘
                   │
          ┌────────▼────────┐
          │ 5 FFI Backends  │
          │ Go Zig Nim JAX  │
          │ Chapel          │
          └─────────────────┘
```

### CI/CD Infrastructure

```
┌──────────────────────────────────────────────────────┐
│              GitHub Actions: single ci.yml          │
└───────────────────────────┬──────────────────────────┘
                            │
                    ┌───────▼────────┐
                    │ Build + Tests  │
                    │ FFI Contracts  │
                    └───────┬────────┘
                            │
                  ┌─────────┴─────────┐
                  │                   │
               Pass                Fail
                  │                   │
                  ▼                   ▼
            Merge candidate      Visible stop
```

## 7 MCP Tools

### 1. WEBSEARCH
Massive web search across multiple engines with Go concurrency and Nim parsing.

### 2. PREMIUM_CONTENT
Protected-content workflows and exploitation-oriented collection paths.

### 3. SCAN_WORKSPACE
Workspace inspection for errors, warnings, dependency risks, and code issues.

### 4. AI_DATASET_TRAINER
Dataset generation and optimization through Chapel-oriented training flows.

### 5. FILE_SEARCH
Precision search with exact matches, regex support, and accelerated scanning.

### 6. CODE_INTELLIGENCE
Code analysis, risk scoring, and ML-assisted interpretation backed by JAX/Zig paths.

### 7. INTELLIGENCE_OSINT
OSINT collection and risk analysis across web and intelligence-oriented sources.

## Module Organization

### Core
- lateral_movement_advanced.rs
- exploit_engine.rs
- premium_content_scraper.rs
- web_search.rs
- dataset_generator.rs
- nuclear_core.rs
- data_management.rs
- url_helpers.rs

### FFI
- go_integration.rs
- zig_integration.rs
- nim_integration.rs
- jax_integration.rs
- chapel_integration.rs

### WASM
- dataset_extractor.rs
- data_search.rs
- file_search.rs
- neural_ops.rs
- real_human_scraper.rs
- ultra_power.rs

### OSINT
- bayesian_networks_osint.rs
- game_theory_osint.rs
- neural_networks_osint.rs
- case_resolver_osint.rs
- nuclear_integration_osint.rs

### Infra
- cache.rs
- rate_limit.rs
- proxy_rotation.rs
- chromium_rendering.rs
- deepweb_tor.rs
- advanced_bypass.rs
- data_extraction.rs
- intelligent_storage.rs

### AI
- chatbot.rs
- huggingface_integration.rs

## Key Architecture Decisions

1. **Each Tool is Independent**
   - Tools compose through shared modules, not by hiding each other internally.

2. **Code Reuse via Dependencies**
   - Core, FFI, WASM, OSINT, Infra, and AI modules remain the shared capability layers.

3. **Chapel AI Integration**
   - Chapel is used for optimization and training paths that explicitly require it.
   - Missing Chapel support is a visible failure for those paths, not a silent downgrade.

4. **Parallelism by Default**
   - Go, Zig, WASM, and Chapel exist to provide real acceleration rather than aspirational claims.

5. **Real Execution, No Silent Degradation**
   - Advertised capabilities must be wired to real implementations.
   - Required backends fail explicitly when missing.
   - Mock or fallback substitution must not be presented as production behavior.

## CI Truth

- The repository should keep one primary CI workflow.
- CI is intended to validate build, tests, and explicit backend contracts.
- Missing required backends are failures, not triggers for hidden repair logic.
- Documentation must describe the repo as it exists, not aspirational recovery systems.

## Architecture Summary

**This unified architecture combines:**
- ✅ **7 MCP Tools** with 5 FFI backends (Go, Zig, Nim, JAX, Chapel)
- ✅ **37 Specialized Modules** for extraction, analysis, OSINT, and infrastructure
- ✅ **50K Goroutine Parallelism** as an implementation goal where the backend is actually wired
- ✅ **Single CI Workflow** for build, tests, and backend contract enforcement
- ✅ **Real Execution** with no mocks and no silent fallback substitution

**Status:** ✅ Production Ready  
**Architecture Version:** 2.1 (Unified + Explicit Backend Contracts)  
**Last Updated:** 11 de marzo de 2026