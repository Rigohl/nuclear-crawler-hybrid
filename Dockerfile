# Dockerfile for Nuclear Crawler Hybrid MCP Server
FROM rust:latest as builder

WORKDIR /app

# Copy source
COPY . .

# Build MCP server
RUN cargo build --release --bin nuclear-mcp

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/nuclear-mcp /app/nuclear-mcp

# Expose port
EXPOSE 8079

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8079/ || exit 1

# Run server
CMD ["/app/nuclear-mcp"]
