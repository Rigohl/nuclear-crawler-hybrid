# Dockerfile for Nuclear Crawler Hybrid MCP Server with FFI Support
# Go, Nim, Zig, Rust integration

# ===== BUILDER STAGE =====
FROM rust:1.80-slim AS builder

WORKDIR /build

ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies
    build-essential \
    git \
    ca-certificates \
    libssl-dev \
    pkg-config \
    golang-go \
    python3 \
    python3-dev \
    python3-pip \
    curl \
    lsb-release \
    && rm -rf /var/lib/apt/lists/*

# Install JAX (CPU version for Docker build compatibility)
RUN python3 -m pip install --break-system-packages jax jaxlib numpy

# Install Chapel (Simulated/Stub or Real download if URL stable - limiting to python for now as Chapel is huge, but we will add the env vars)
# ENV CHPL_HOME=/usr/local/chapel
# ENV PATH="$PATH:$CHPL_HOME/bin"

# Update CA certificates to handle potential SSL issues
RUN update-ca-certificates

# Copy source
COPY . /build/

# Build MCP server (Rust-only, pure implementation)
# Configure git to use system certificates if needed
RUN git config --global http.sslVerify true || true

# Build with cargo, using system certificates
ENV CARGO_HTTP_CAINFO=/etc/ssl/certs/ca-certificates.crt
RUN cargo build --release --bin nuclear-mcp

# ===== RUNTIME STAGE =====
FROM debian:bookworm-slim

WORKDIR /app

ENV DEBIAN_FRONTEND=noninteractive

# Install minimal runtime dependencies
    ca-certificates \
    curl \
    libssl3 \
    python3 \
    python3-pip \
    python3-numpy \
    && rm -rf /var/lib/apt/lists/*

# Install Runtime JAX
RUN python3 -m pip install --break-system-packages jax jaxlib

# Copy binaries from builder
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
