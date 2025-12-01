# ═══════════════════════════════════════════════════════════════════════════════
# 🔥 NUCLEAR CRAWLER HYBRID - Dockerfile Multi-Stage Empresarial
# ═══════════════════════════════════════════════════════════════════════════════
# Optimizado para: Tamaño mínimo, Seguridad, Multi-arch

# ═══════════════════════════════════════════════════════════════
# STAGE 1: Builder - Compilar binario optimizado
# ═══════════════════════════════════════════════════════════════
FROM rust:1.83-slim-bookworm AS builder

# Instalar dependencias de compilación
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

# Agregar target para compilación estática
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Copiar solo archivos de dependencias primero (cache de capas)
COPY Cargo.toml Cargo.lock ./

# Crear proyecto dummy para cachear dependencias
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    rm -rf src

# Copiar código fuente
COPY src/ src/

# Compilar binario final optimizado
RUN cargo build --release --target x86_64-unknown-linux-musl --bin nuclear-mcp && \
    strip target/x86_64-unknown-linux-musl/release/nuclear-mcp

# ═══════════════════════════════════════════════════════════════
# STAGE 2: Runtime - Imagen mínima de producción
# ═══════════════════════════════════════════════════════════════
FROM scratch AS runtime

# Copiar certificados SSL para HTTPS
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copiar binario optimizado
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/nuclear-mcp /nuclear-mcp

# Usuario no-root para seguridad
USER 1000:1000

# Puerto MCP
EXPOSE 3000

# Healthcheck
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD ["/nuclear-mcp", "--health"]

# Punto de entrada
ENTRYPOINT ["/nuclear-mcp"]
CMD ["--stdio"]

# ═══════════════════════════════════════════════════════════════
# LABELS - Metadatos OCI
# ═══════════════════════════════════════════════════════════════
LABEL org.opencontainers.image.title="Nuclear Crawler Hybrid"
LABEL org.opencontainers.image.description="🔥 MCP Server de búsqueda web NUCLEAR con Go + Zig + Stealth"
LABEL org.opencontainers.image.vendor="Nuclear Systems"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.source="https://github.com/YOUR_USERNAME/nuclear-crawler-hybrid"
