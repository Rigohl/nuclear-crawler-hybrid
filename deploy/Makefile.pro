# Professional Makefile for Nuclear Crawler Hybrid
# Multi-language build system with intelligent targets

.PHONY: help all validate build test clean format lint pre-commit watch install dev deploy

# Colors for output
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
NC := \033[0m # No Color

# Configuration
RUST_DIR := .
CHAPEL_DIR := ffi/chapel
GO_DIR := mcp-servers/github
CARGO := cargo
CHPL := chpl
GO := go

# Default target
.DEFAULT_GOAL := help

## help: Show this help message
help:
	@echo "$(BLUE)Nuclear Crawler Hybrid - Professional Build System$(NC)"
	@echo ""
	@echo "$(GREEN)Available targets:$(NC)"
	@awk '/^##/ {desc=$$0; gsub(/^## /, "", desc); getline; if ($$0 ~ /^[a-zA-Z_-]+:/) {target=$$1; gsub(/:/, "", target); printf "  $(YELLOW)%-20s$(NC) %s\n", target, desc}}' $(MAKEFILE_LIST)

## all: Build everything (Rust + Chapel + Go)
all: validate build test

## validate: Run critical constraint validation
validate:
	@echo "$(BLUE)🔍 Validating critical constraints...$(NC)"
	@$(CARGO) test test_exactly_5_tools --lib || (echo "$(RED)❌ 5 tools constraint VIOLATED$(NC)" && exit 1)
	@! grep -r "mock_\|Mock\|TODO: implement\|unimplemented!" src/ && echo "$(GREEN)✅ No mocks detected$(NC)" || echo "$(YELLOW)⚠️ Potential mocks found$(NC)"

## build: Build all components
build: build-rust build-chapel build-go
	@echo "$(GREEN)✅ All builds complete$(NC)"

## build-rust: Build Rust MCP server
build-rust:
	@echo "$(BLUE)🦀 Building Rust MCP...$(NC)"
	@$(CARGO) build --release

## build-chapel: Build Chapel AI systems
build-chapel:
	@echo "$(BLUE)🧠 Building Chapel AI...$(NC)"
	@cd $(CHAPEL_DIR) && $(MAKE) full-pipeline

## build-go: Build Go GitHub MCP
build-go:
	@echo "$(BLUE)🐹 Building Go MCP...$(NC)"
	@cd $(GO_DIR) && $(GO) build

## test: Run all tests
test: test-rust test-chapel test-go
	@echo "$(GREEN)✅ All tests passed$(NC)"

## test-rust: Run Rust tests (including 5 tools validation)
test-rust:
	@echo "$(BLUE)🧪 Testing Rust...$(NC)"
	@$(CARGO) test test_exactly_5_tools
	@$(CARGO) test --lib

## test-chapel: Run Chapel tests
test-chapel:
	@echo "$(BLUE)🧪 Testing Chapel...$(NC)"
	@cd $(CHAPEL_DIR) && $(MAKE) test

## test-go: Run Go tests
test-go:
	@echo "$(BLUE)🧪 Testing Go...$(NC)"
	@cd $(GO_DIR) && $(GO) test ./...

## clean: Clean all build artifacts
clean:
	@echo "$(BLUE)🧹 Cleaning...$(NC)"
	@$(CARGO) clean
	@cd $(CHAPEL_DIR) && $(MAKE) clean
	@cd $(GO_DIR) && $(GO) clean
	@echo "$(GREEN)✅ Clean complete$(NC)"

## format: Format all code
format:
	@echo "$(BLUE)📝 Formatting...$(NC)"
	@$(CARGO) fmt
	@cd $(GO_DIR) && $(GO) fmt ./...
	@echo "$(GREEN)✅ Format complete$(NC)"

## lint: Run linters on all code
lint:
	@echo "$(BLUE)🔍 Linting...$(NC)"
	@$(CARGO) clippy -- -D warnings
	@cd $(CHAPEL_DIR) && $(MAKE) check
	@cd $(GO_DIR) && $(GO) vet ./...
	@echo "$(GREEN)✅ Lint complete$(NC)"

## pre-commit: Run all pre-commit checks
pre-commit: format validate lint test
	@echo "$(GREEN)✅ All pre-commit checks passed! Safe to commit.$(NC)"

## watch: Watch for changes and rebuild
watch:
	@echo "$(BLUE)👀 Watching for changes...$(NC)"
	@command -v cargo-watch >/dev/null 2>&1 || $(CARGO) install cargo-watch
	@$(CARGO) watch -x check -x test

## install: Install development dependencies
install:
	@echo "$(BLUE)📦 Installing dependencies...$(NC)"
	@$(CARGO) install cargo-watch cargo-audit
	@echo "$(GREEN)✅ Dependencies installed$(NC)"

## dev: Start development environment
dev:
	@echo "$(BLUE)🚀 Starting development environment...$(NC)"
	@$(CARGO) run --bin nuclear-mcp --release

## deploy: Deploy to HuggingFace (requires auth)
deploy:
	@echo "$(BLUE)🚀 Deploying to HuggingFace...$(NC)"
	@cd $(CHAPEL_DIR) && $(MAKE) deploy-hf

## benchmark: Run performance benchmarks
benchmark:
	@echo "$(BLUE)⚡ Running benchmarks...$(NC)"
	@$(CARGO) bench
	@cd $(CHAPEL_DIR) && $(MAKE) profile

## check-deps: Check for outdated dependencies
check-deps:
	@echo "$(BLUE)🔍 Checking dependencies...$(NC)"
	@$(CARGO) tree --duplicates
	@$(CARGO) audit || true

## ci: Run CI pipeline locally
ci: validate lint build test
	@echo "$(GREEN)✅ CI pipeline passed$(NC)"

## stats: Show project statistics
stats:
	@echo "$(BLUE)📊 Project Statistics$(NC)"
	@echo ""
	@echo "Rust LOC:"
	@find src -name "*.rs" -exec wc -l {} + | tail -1
	@echo ""
	@echo "Chapel LOC:"
	@find $(CHAPEL_DIR) -name "*.chpl" -exec wc -l {} + | tail -1
	@echo ""
	@echo "Go LOC:"
	@find $(GO_DIR) -name "*.go" -exec wc -l {} + | tail -1
	@echo ""
	@echo "Datasets:"
	@find models -name "*.jsonl" -o -name "*.json" | wc -l
