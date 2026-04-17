# Dockerfile for Nuclear Crawler Hybrid MCP Server with FFI Support
# Go, Nim, Zig, Rust, and FULL CHAPEL integration

# ===== BUILDER STAGE =====
FROM rust:1.80-slim AS builder

WORKDIR /build

ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies including tools for Chapel
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    ca-certificates \
    libssl-dev \
    pkg-config \
    golang-go \
    wget \
    tar \
    python3 \
    bash \
    cmake \
    clang \
    && rm -rf /var/lib/apt/lists/*

RUN update-ca-certificates

# Install Chapel v2.8.0
ENV CHPL_VERSION=2.8.0
ENV CHPL_HOME=/opt/chapel
ENV PATH=$CHPL_HOME/bin/linux64-x86_64:$CHPL_HOME/bin/linux64-x86_64-gnu:$CHPL_HOME/bin:$PATH

RUN wget -q https://github.com/chapel-lang/chapel/releases/download/${CHPL_VERSION}/chapel-${CHPL_VERSION}.tar.gz && \
    tar -xzf chapel-${CHPL_VERSION}.tar.gz && \
    mv chapel-${CHPL_VERSION} /opt/chapel && \
    rm chapel-${CHPL_VERSION}.tar.gz

# Quick build of Chapel compiler (without heavy LLVM)
WORKDIR /opt/chapel
ENV CHPL_LLVM=none
RUN make -j$(nproc)

WORKDIR /build
COPY . /build/

# Build Chapel AI FFI first
WORKDIR /build/ffi/chapel
RUN ./build_chapel_real.sh

# Build Rust MCP server
WORKDIR /build
RUN git config --global http.sslVerify true || true
ENV CARGO_HTTP_CAINFO=/etc/ssl/certs/ca-certificates.crt
RUN cargo build --release --bin nuclear-mcp

# ===== RUNTIME STAGE =====
FROM debian:bookworm-slim

WORKDIR /app

ENV DEBIAN_FRONTEND=noninteractive

# Install minimal runtime dependencies (Chapel output needs standard C libraries and libm)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl3 \
    libc6 \
    libgcc-s1 \
    && rm -rf /var/lib/apt/lists/*

# Copy binaries from builder
COPY --from=builder /build/target/release/nuclear-mcp /app/nuclear-mcp
# Copy the compiled Chapel library (.so)
COPY --from=builder /build/ffi/chapel/libchapel_ai.so /app/ffi/chapel/libchapel_ai.so

# Ensure the system knows where to find libchapel_ai.so
ENV LD_LIBRARY_PATH=/app/ffi/chapel:$LD_LIBRARY_PATH

RUN chmod +x /app/nuclear-mcp

EXPOSE 8079

HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8079/ || true

CMD ["/app/nuclear-mcp"]
