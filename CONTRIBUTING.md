# Contributing to Nuclear Crawler Hybrid

First off, thank you for considering contributing to Nuclear Crawler! 🔥

## Code of Conduct

This project adheres to our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### 🐛 Reporting Bugs

Before creating bug reports, please check existing issues. When creating a bug report, include:

- **Clear title** describing the issue
- **Steps to reproduce** the behavior
- **Expected behavior** vs actual behavior
- **Environment details** (OS, Rust version, etc.)
- **Relevant logs** or error messages

### 💡 Suggesting Features

Feature requests are welcome! Please include:

- **Clear description** of the feature
- **Use case** explaining why it's needed
- **Possible implementation** if you have ideas

### 🔧 Pull Requests

1. **Fork** the repository
2. **Create a branch** (`git checkout -b feature/amazing-feature`)
3. **Make your changes**
4. **Run tests** (`cargo test --all-features`)
5. **Run lints** (`cargo clippy --all-targets`)
6. **Format code** (`cargo fmt`)
7. **Commit** (`git commit -m 'feat: add amazing feature'`)
8. **Push** (`git push origin feature/amazing-feature`)
9. **Open a Pull Request**

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/nuclear-crawler-hybrid.git
cd nuclear-crawler-hybrid

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build

# Test
cargo test --all-features

# Lint
cargo clippy --all-targets -- -D warnings

# Format
cargo fmt --all
```

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `style:` Formatting
- `refactor:` Code refactoring
- `test:` Tests
- `chore:` Maintenance

Examples:
```
feat: add stealth mode for anti-detection
fix: resolve connection pool exhaustion
docs: update API documentation
```

## Code Style

- Follow Rust idioms and best practices
- Use `rustfmt` for formatting
- Address all `clippy` warnings
- Write documentation for public APIs
- Add tests for new functionality

## Testing

```bash
# Run all tests
cargo test --all-features

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Questions?

Feel free to open an issue for any questions!

---

Thank you for contributing! 🚀
