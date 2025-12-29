# Dockerfile for Nuclear Crawler Hybrid MCP Server with FFI Support
# Go, Nim, Zig, Rust integration

# ===== BUILDER STAGE =====
FROM ubuntu:22.04 as builder

WORKDIR /build

ENV DEBIAN_FRONTEND=noninteractive

# Install base dependencies
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    build-essential \
    git \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install Go
RUN apt-get update && apt-get install -y golang-go && rm -rf /var/lib/apt/lists/*

# Optional: Nim, Zig can be added later if needed
# For now, use Rust-only build (faster, more stable)

# Copy source
COPY . /build/

# Build MCP server (Rust-only, pure implementation)
RUN . $HOME/.cargo/env && \
    cd /build && \
    cargo build --release --bin nuclear-mcp && \
    cargo build --release --bin nuclear-data

# ===== RUNTIME STAGE =====
FROM ubuntu:22.04

WORKDIR /app

ENV DEBIAN_FRONTEND=noninteractive

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binaries from builder
COPY --from=builder /build/target/release/nuclear-mcp /app/nuclear-mcp
COPY --from=builder /build/target/release/nuclear-data /app/nuclear-data

# Make executable
RUN chmod +x /app/nuclear-mcp /app/nuclear-data

# Expose port
EXPOSE 8079

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8079/ || exit 1

# Run server
CMD ["/app/nuclear-mcp"]
