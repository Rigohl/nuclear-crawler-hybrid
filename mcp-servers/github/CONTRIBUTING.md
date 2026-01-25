# Contributing to GitHub MCP Server

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites

- Go 1.24 or later
- Docker (optional, for containerized development)
- Git

### Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/github/github-mcp-server.git
   cd github-mcp-server
   ```

2. **Set up development environment**
   ```bash
   ./script/setup-dev
   ```

3. **Configure your environment**
   - Copy `.env.example` to `.env`
   - Add your GitHub Personal Access Token to `.env`
   - See `.env.example` for all available configuration options

4. **Run the server locally**
   ```bash
   make run
   # or
   go run ./cmd/github-mcp-server stdio
   ```

## Development Workflow

### Using Make

We provide a `Makefile` with common development tasks:

```bash
make help          # Show all available commands
make build         # Build the binary
make test          # Run tests
make lint          # Run linters
make check         # Run all checks (format, vet, lint, test)
make docker-build  # Build Docker image
make docker-run    # Run Docker container
```

### Running Tests

```bash
# Run all tests
make test

# Run tests with coverage
make test-coverage

# Run specific test package
go test ./pkg/toolset/...
```

### Code Quality

```bash
# Format code
make fmt

# Run linters
make lint

# Run all checks
make check
```

### Building

```bash
# Build binary
make build

# Build Docker image
make docker-build
```

## Project Structure

```
github-mcp-server/
├── cmd/
│   └── github-mcp-server/    # Main application entry point
├── pkg/                      # Package code
│   ├── toolset/              # Tool implementations
│   └── ...                   # Other packages
├── internal/                 # Internal packages
├── docs/                     # Documentation
├── script/                   # Utility scripts
└── .github/                   # GitHub workflows and templates
```

## Making Changes

1. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Follow Go best practices
   - Add tests for new functionality
   - Update documentation as needed

3. **Run checks**
   ```bash
   make check
   ```

4. **Commit your changes**
   - Use clear, descriptive commit messages
   - Follow conventional commit format when possible

5. **Push and create a pull request**

## Code Style

- Follow [Effective Go](https://go.dev/doc/effective_go) guidelines
- Use `gofmt` for formatting
- Follow the existing code style in the project
- Run `make lint` before committing

## Testing

- Write tests for new features
- Maintain or improve test coverage
- Test both success and error cases
- Use table-driven tests when appropriate

## Documentation

- Update README.md for user-facing changes
- Add doc comments for exported functions and types
- Update examples if API changes
- Keep CHANGELOG.md updated for significant changes

## Security

- Never commit tokens or secrets
- Use environment variables for sensitive data
- Follow security best practices
- Report security issues privately

## Questions?

- Check existing issues and discussions
- Open a new issue for bugs or feature requests
- See [SUPPORT.md](SUPPORT.md) for more help

Thank you for contributing! 🎉

