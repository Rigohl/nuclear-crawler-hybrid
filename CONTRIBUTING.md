# 🤝 Contributing to Nuclear Crawler Hybrid

Thank you for considering contributing to Nuclear Crawler Hybrid! This document provides guidelines and instructions for contributing to the project.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)

---

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors. We expect all participants to:

- Be respectful and considerate
- Welcome constructive feedback
- Focus on what is best for the community
- Show empathy towards others

### Unacceptable Behavior

- Harassment or discriminatory language
- Personal attacks or trolling
- Publishing others' private information
- Other conduct deemed inappropriate

---

## Getting Started

### Prerequisites

Before contributing, ensure you have:

- **Rust 1.75+**: Install from [rustup.rs](https://rustup.rs)
- **Git**: For version control
- **GitHub Account**: For submitting pull requests
- **Text Editor/IDE**: VS Code, IntelliJ IDEA, or similar

### Optional Tools

For FFI development:
- **Go 1.21+**: For Go integration
- **Zig 0.11+**: For SIMD processing
- **Nim 2.0+**: For HTML parsing
- **Docker**: For containerized testing

---

## Development Setup

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/nuclear-crawler-hybrid.git
cd nuclear-crawler-hybrid

# Add upstream remote
git remote add upstream https://github.com/Rigohl/nuclear-crawler-hybrid.git
```

### 2. Install Dependencies

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development tools
rustup component add rustfmt clippy

# Optional: Install FFI compilers
# Go: https://golang.org/doc/install
# Zig: https://ziglang.org/download/
# Nim: https://nim-lang.org/install.html
```

### 3. Build the Project

```bash
# Build in debug mode
cargo build

# Run tests
cargo test

# Build FFI libraries (optional)
./scripts/build-ffi.sh
```

### 4. Run the Server

```bash
# Run in development mode
cargo run -- --port 8079

# Or run release build
cargo build --release
./target/release/nuclear-mcp --port 8079
```

### 5. Verify Installation

```bash
# Test the server
curl http://localhost:8079/

# Expected response:
# {"status":"ok","version":"0.1.0",...}
```

---

## Project Structure

```
nuclear-crawler-hybrid/
├── src/
│   ├── bin/
│   │   └── nuclear_ultimate.rs      # Main MCP server binary
│   ├── lib.rs                        # Module exports
│   ├── web_search.rs                 # WebSearch tool
│   ├── deepweb_tor.rs                # DeepWeb search tool
│   ├── premium_content_scraper.rs    # Premium scraper tool
│   ├── file_search.rs                # File search tool
│   ├── cache.rs                      # Caching system
│   ├── rate_limit.rs                 # Rate limiter
│   ├── intelligent_storage.rs        # Result storage
│   ├── nuclear_core.rs               # Core utilities
│   ├── go_integration.rs             # Go FFI bindings
│   ├── zig_integration.rs            # Zig FFI bindings
│   ├── nim_integration.rs            # Nim FFI bindings
│   └── jax_integration.rs            # JAX bindings
├── go/
│   └── src/
│       └── stealth_go.go             # Go parallel processor
├── zig/
│   └── src/
│       └── lib.zig                   # Zig SIMD processor
├── nim/
│   └── src/
│       └── nuclear_nim.nim           # Nim HTML parser
├── scripts/
│   ├── build-ffi.sh                  # Build FFI libraries
│   └── jax_pipeline.py               # JAX batch processor
├── tests/
│   └── integration_tests.rs          # Integration tests
├── Dockerfile                        # Docker image definition
├── docker-compose.yml                # Docker Compose config
├── Cargo.toml                        # Rust project manifest
└── README.md                         # Project documentation
```

### Key Modules

| Module | Purpose | Lines |
|--------|---------|-------|
| `nuclear_ultimate.rs` | Main MCP server, HTTP handlers | ~1450 |
| `web_search.rs` | Web search implementation | ~800 |
| `deepweb_tor.rs` | Tor/deep web integration | ~600 |
| `premium_content_scraper.rs` | Premium content extraction | ~500 |
| `file_search.rs` | File pattern matching | ~400 |

---

## Coding Standards

### Rust Style Guidelines

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

1. **Formatting**: Use `rustfmt`
   ```bash
   cargo fmt
   ```

2. **Linting**: Pass `clippy` without warnings
   ```bash
   cargo clippy -- -D warnings
   ```

3. **Naming Conventions**:
   - `snake_case` for functions and variables
   - `PascalCase` for types, traits, enums
   - `SCREAMING_SNAKE_CASE` for constants

4. **Documentation**:
   ```rust
   /// Performs a web search across multiple engines.
   ///
   /// # Arguments
   ///
   /// * `queries` - List of search queries
   /// * `config` - Search configuration
   ///
   /// # Returns
   ///
   /// A `Result` containing search results or an error.
   ///
   /// # Examples
   ///
   /// ```
   /// let results = search(&["rust async"], config)?;
   /// ```
   pub async fn search(queries: &[String], config: Config) -> Result<Vec<SearchResult>> {
       // Implementation
   }
   ```

5. **Error Handling**:
   - Use `Result<T, E>` for fallible operations
   - Prefer `?` operator for error propagation
   - Avoid `.unwrap()` except in tests
   - Provide context with `.context()` or `.map_err()`

6. **Async/Await**:
   - Use `async fn` for I/O operations
   - Spawn tasks with `tokio::spawn()`
   - Use timeouts for external calls
   - Don't block the async runtime

### Code Example

**Good**:
```rust
pub async fn fetch_url(url: &str) -> Result<String> {
    let client = Client::new();
    let response = tokio::time::timeout(
        Duration::from_secs(5),
        client.get(url).send()
    )
    .await
    .context("Request timeout")?
    .context("Failed to send request")?;
    
    let text = response.text().await
        .context("Failed to read response body")?;
    
    Ok(text)
}
```

**Bad**:
```rust
pub async fn fetch_url(url: &str) -> String {
    let client = Client::new();
    client.get(url).send().await.unwrap().text().await.unwrap()
}
```

---

## Testing Guidelines

### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query() {
        let query = "rust async";
        let parsed = parse_query(query);
        assert_eq!(parsed.terms.len(), 2);
    }

    #[tokio::test]
    async fn test_search() {
        let config = Config::default();
        let results = search(&["test"], config).await;
        assert!(results.is_ok());
    }
}
```

### Integration Tests

Place in `tests/` directory:

```rust
// tests/integration_tests.rs
use nuclear_crawler_hybrid::web_search::WebSearch;

#[tokio::test]
async fn test_websearch_tool() {
    let searcher = WebSearch::new().unwrap();
    let results = searcher.search(&["rust"], 10).await;
    assert!(results.is_ok());
    assert!(!results.unwrap().is_empty());
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Test Coverage Goals

- **Core modules**: 80%+ coverage
- **Tool implementations**: 70%+ coverage
- **FFI integrations**: 60%+ coverage (limited by external dependencies)
- **HTTP handlers**: 90%+ coverage

---

## Submitting Changes

### Branch Naming

Use descriptive branch names:

- `feature/add-google-search` - New features
- `fix/rate-limit-bug` - Bug fixes
- `docs/update-readme` - Documentation
- `refactor/cache-module` - Code refactoring
- `test/file-search-tests` - Test additions

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code formatting (no logic change)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

**Examples**:
```
feat(websearch): add Google search engine support

- Implemented Google search API integration
- Added rate limiting for Google requests
- Updated tests for new engine

Closes #123
```

```
fix(cache): prevent cache corruption on concurrent writes

Use RwLock to ensure thread-safe cache access.

Fixes #456
```

### Pull Request Process

1. **Update your fork**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Create feature branch**:
   ```bash
   git checkout -b feature/my-feature
   ```

3. **Make changes and commit**:
   ```bash
   git add .
   git commit -m "feat: add my feature"
   ```

4. **Run tests and checks**:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   ```

5. **Push to your fork**:
   ```bash
   git push origin feature/my-feature
   ```

6. **Open Pull Request**:
   - Go to GitHub and create a PR
   - Fill out the PR template
   - Link related issues
   - Request review

### PR Checklist

Before submitting, ensure:

- [ ] Code follows style guidelines (`cargo fmt`)
- [ ] Linting passes (`cargo clippy -- -D warnings`)
- [ ] All tests pass (`cargo test`)
- [ ] New tests added for new features
- [ ] Documentation updated
- [ ] Commit messages follow conventions
- [ ] PR description is clear and complete
- [ ] FFI libraries compile (if modified)

---

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes (e.g., 1.0.0 → 2.0.0)
- **MINOR**: New features, backward compatible (e.g., 1.0.0 → 1.1.0)
- **PATCH**: Bug fixes, backward compatible (e.g., 1.0.0 → 1.0.1)

### Release Checklist

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run full test suite
4. Build release binary
5. Test release binary
6. Create git tag: `git tag -a v1.0.0 -m "Release 1.0.0"`
7. Push tag: `git push origin v1.0.0`
8. GitHub Actions will create release automatically

### Changelog Format

```markdown
## [1.0.0] - 2025-12-29

### Added
- New Google search engine integration
- Rate limiting per search engine
- Docker multi-platform support

### Changed
- Improved cache performance by 50%
- Updated Tokio to 1.35

### Fixed
- Fixed race condition in cache module
- Resolved FFI library loading on macOS

### Security
- Updated dependencies to fix vulnerabilities
```

---

## Advanced Topics

### FFI Development

#### Go Integration

```bash
# Build Go FFI library
cd go/src
go build -buildmode=c-shared -o ../../libs/libstealth_go.so .
```

Requirements:
- Export functions with `//export FunctionName`
- Use C-compatible types
- Handle errors gracefully

#### Zig SIMD

```bash
# Build Zig library
cd zig/src
zig build-lib lib.zig -dynamic -lc -O ReleaseFast
mv liblib.so ../../libs/libzig_simd.so
```

Requirements:
- Use `export` for public functions
- Ensure SIMD compatibility
- Test on target architectures

#### Nim HTML Parser

```bash
# Build Nim library
cd nim/src
nim c --app:lib --noMain nuclear_nim.nim
mv libnuclear_nim.so ../../libs/
```

Requirements:
- Use `{.exportc.}` pragma
- Handle C string conversion
- Memory management with GC

### Performance Optimization

1. **Profiling**:
   ```bash
   cargo install cargo-flamegraph
   cargo flamegraph --bin nuclear-mcp
   ```

2. **Benchmarking**:
   ```bash
   cargo bench
   ```

3. **Memory Analysis**:
   ```bash
   cargo install cargo-valgrind
   cargo valgrind --bin nuclear-mcp
   ```

---

## Getting Help

### Resources

- **Documentation**: [README.md](README.md), [ARCHITECTURE.md](ARCHITECTURE.md)
- **GitHub Issues**: https://github.com/Rigohl/nuclear-crawler-hybrid/issues
- **Discussions**: https://github.com/Rigohl/nuclear-crawler-hybrid/discussions

### Asking Questions

When asking for help:

1. Check existing issues and documentation first
2. Provide a clear description of the problem
3. Include relevant code snippets
4. Share error messages and logs
5. Describe what you've already tried

---

## Recognition

Contributors are recognized in:

- `CONTRIBUTORS.md` file
- GitHub release notes
- Project README

Thank you for contributing to Nuclear Crawler Hybrid! 🚀

---

**Last Updated**: 2025-12-29  
**Document Version**: 1.0.0
