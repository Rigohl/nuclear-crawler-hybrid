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

# Install Nim from binary
RUN mkdir -p /opt && \
    cd /opt && \
    wget -q https://nim-lang.org/download/nim-1.6.14.tar.xz && \
    tar -xf nim-1.6.14.tar.xz && \
    cd nim-1.6.14 && \
    ./build_all.sh && \
    ln -s /opt/nim-1.6.14/bin/nim /usr/local/bin/nim

# Install Zig from binary
RUN mkdir -p /opt && \
    cd /opt && \
    wget -q https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz && \
    tar -xf zig-linux-x86_64-0.13.0.tar.xz && \
    ln -s /opt/zig-linux-x86_64-0.13.0/zig /usr/local/bin/zig

# Copy source
COPY . /build/

# Build MCP server with FFI
RUN . $HOME/.cargo/env && \
    export PATH="/opt/nim-1.6.14/bin:$PATH" && \
    cd /build && \
    cargo build --release --bin nuclear-mcp

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

# Copy binary from builder
COPY --from=builder /build/target/release/nuclear-mcp /app/nuclear-mcp

# Make executable
RUN chmod +x /app/nuclear-mcp

# Expose port
EXPOSE 8079

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8079/ || exit 1

# Run server
CMD ["/app/nuclear-mcp"]
