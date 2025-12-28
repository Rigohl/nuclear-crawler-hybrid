# Dockerfile for Nuclear Crawler Hybrid MCP Server with FFI Support
# Go, Nim, Zig, Rust integration

# ===== BUILDER STAGE =====
FROM ubuntu:22.04 as builder

WORKDIR /build

ENV DEBIAN_FRONTEND=noninteractive

# Install Rust
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . $HOME/.cargo/env

# Install Go
RUN apt-get install -y golang-go

# Install Nim
RUN apt-get install -y nim

# Install Zig
RUN apt-get install -y zig

# Copy source
COPY . /build/

# Build MCP server with FFI
RUN . $HOME/.cargo/env && \
    cd /build && \
    cargo build --release --bin nuclear-mcp

# ===== RUNTIME STAGE =====
FROM ubuntu:22.04

WORKDIR /app

ENV DEBIAN_FRONTEND=noninteractive

# Install runtime dependencies (minimal)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/target/release/nuclear-mcp /app/nuclear-mcp

# Make executable
RUN chmod +x /app/nuclear-mcp

# Expose port
EXPOSE 8079

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8079/ || exit 1

# Run server
CMD ["/app/nuclear-mcp"]
