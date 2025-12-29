# 🚀 Nuclear Crawler Hybrid - Deployment Guide

This guide covers deploying Nuclear Crawler Hybrid in various environments, from local development to production-ready Docker containers.

---

## Table of Contents

- [Quick Start with Docker](#quick-start-with-docker)
- [Docker Compose Setup](#docker-compose-setup)
- [Building from Source](#building-from-source)
- [Production Deployment](#production-deployment)
- [Environment Configuration](#environment-configuration)
- [Monitoring & Health Checks](#monitoring--health-checks)
- [Troubleshooting](#troubleshooting)

---

## Quick Start with Docker

### Pull Pre-built Image

```bash
# Pull the latest image
docker pull ghcr.io/rigohl/nuclear-crawler-hybrid:latest

# Run the container
docker run -d \
  --name nuclear-mcp \
  -p 8079:8079 \
  -e RUST_LOG=info \
  ghcr.io/rigohl/nuclear-crawler-hybrid:latest
```

### Build Your Own Image

```bash
# Clone the repository
git clone https://github.com/Rigohl/nuclear-crawler-hybrid.git
cd nuclear-crawler-hybrid

# Build the Docker image
docker build -t nuclear-crawler-hybrid:local .

# Run the container
docker run -d \
  --name nuclear-mcp \
  -p 8079:8079 \
  nuclear-crawler-hybrid:local
```

---

## Docker Compose Setup

### Basic Configuration

The repository includes a `docker-compose.yml` file for easy deployment:

```yaml
version: '3.8'

services:
  nuclear-mcp:
    build:
      context: .
      dockerfile: Dockerfile
    container_name: nuclear-mcp-server
    ports:
      - "8079:8079"
    environment:
      - RUST_LOG=info
      - MCP_PORT=8079
      - RATE_LIMIT=100
      - CACHE_TTL=300
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8079/"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 5s
    volumes:
      - ./resultados:/app/resultados
```

### Starting Services

```bash
# Start in background
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Advanced Configuration

For production with persistent volumes and resource limits:

```yaml
version: '3.8'

services:
  nuclear-mcp:
    image: ghcr.io/rigohl/nuclear-crawler-hybrid:latest
    container_name: nuclear-mcp-server
    ports:
      - "8079:8079"
    environment:
      - RUST_LOG=warn
      - MCP_PORT=8079
      - RATE_LIMIT=200
      - CACHE_TTL=600
      - STORAGE_PATH=/data/resultados
    restart: always
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8079/"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 10s
    volumes:
      - nuclear-data:/data/resultados
      - nuclear-cache:/tmp/cache
    deploy:
      resources:
        limits:
          cpus: '4'
          memory: 4G
        reservations:
          cpus: '2'
          memory: 2G
    networks:
      - nuclear-network

  # Optional: nginx reverse proxy with SSL
  nginx:
    image: nginx:alpine
    container_name: nuclear-nginx
    ports:
      - "443:443"
      - "80:80"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./ssl:/etc/nginx/ssl:ro
    depends_on:
      - nuclear-mcp
    networks:
      - nuclear-network

volumes:
  nuclear-data:
  nuclear-cache:

networks:
  nuclear-network:
    driver: bridge
```

---

## Building from Source

### Prerequisites

Ensure you have the following installed:

- **Rust 1.75+**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Go 1.21+** (optional): For Go FFI
- **Zig 0.11+** (optional): For SIMD acceleration
- **Nim 2.0+** (optional): For HTML parsing

### Standard Build

```bash
# Clone repository
git clone https://github.com/Rigohl/nuclear-crawler-hybrid.git
cd nuclear-crawler-hybrid

# Build release binary
cargo build --release

# Binary location: target/release/nuclear-mcp
./target/release/nuclear-mcp --port 8079
```

### Build with FFI Support

```bash
# Install Go dependencies
cd go/src
go build -buildmode=c-shared -o ../../libs/libstealth_go.so .
cd ../..

# Build Zig SIMD library
cd zig/src
zig build-lib lib.zig -dynamic -lc
mv libzig.so ../../libs/libzig_simd.so
cd ../..

# Build Nim HTML parser
cd nim/src
nim c --app:lib --noMain nuclear_nim.nim
mv libnuclear_nim.so ../../libs/
cd ../..

# Build Rust with FFI features
cargo build --release \
  --features go_integration,zig_integration,nim_integration

# Set library path
export LD_LIBRARY_PATH=$PWD/libs:$LD_LIBRARY_PATH

# Run with FFI
./target/release/nuclear-mcp --port 8079
```

### Cross-Compilation

For ARM64 (e.g., Raspberry Pi, Apple Silicon):

```bash
# Install cross-compilation tools
rustup target add aarch64-unknown-linux-gnu

# Build for ARM64
cargo build --release --target aarch64-unknown-linux-gnu

# Binary: target/aarch64-unknown-linux-gnu/release/nuclear-mcp
```

---

## Production Deployment

### Multi-Stage Docker Build

The included `Dockerfile` uses a multi-stage build for optimal size:

```dockerfile
# Stage 1: Builder
FROM ubuntu:22.04 as builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    curl wget build-essential git ca-certificates golang-go

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install Nim and Zig
RUN wget https://nim-lang.org/download/nim-1.6.14.tar.xz && \
    tar -xf nim-1.6.14.tar.xz && \
    cd nim-1.6.14 && ./build_all.sh

RUN wget https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz && \
    tar -xf zig-linux-x86_64-0.13.0.tar.xz

# Copy source and build
COPY . /build/
RUN . $HOME/.cargo/env && cargo build --release --bin nuclear-mcp

# Stage 2: Runtime
FROM ubuntu:22.04

WORKDIR /app

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    ca-certificates curl libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Copy binary and libraries from builder
COPY --from=builder /build/target/release/nuclear-mcp /app/
COPY --from=builder /build/libs /app/libs

# Set library path
ENV LD_LIBRARY_PATH=/app/libs:$LD_LIBRARY_PATH

# Create results directory
RUN mkdir -p /app/resultados

# Expose MCP port
EXPOSE 8079

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8079/ || exit 1

# Run server
CMD ["./nuclear-mcp", "--port", "8079"]
```

### Kubernetes Deployment

Example Kubernetes manifest:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nuclear-mcp
  labels:
    app: nuclear-mcp
spec:
  replicas: 3
  selector:
    matchLabels:
      app: nuclear-mcp
  template:
    metadata:
      labels:
        app: nuclear-mcp
    spec:
      containers:
      - name: nuclear-mcp
        image: ghcr.io/rigohl/nuclear-crawler-hybrid:latest
        ports:
        - containerPort: 8079
          protocol: TCP
        env:
        - name: RUST_LOG
          value: "info"
        - name: MCP_PORT
          value: "8079"
        - name: RATE_LIMIT
          value: "200"
        resources:
          requests:
            memory: "2Gi"
            cpu: "2"
          limits:
            memory: "4Gi"
            cpu: "4"
        livenessProbe:
          httpGet:
            path: /
            port: 8079
          initialDelaySeconds: 10
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /
            port: 8079
          initialDelaySeconds: 5
          periodSeconds: 10
        volumeMounts:
        - name: results-storage
          mountPath: /app/resultados
      volumes:
      - name: results-storage
        persistentVolumeClaim:
          claimName: nuclear-results-pvc
---
apiVersion: v1
kind: Service
metadata:
  name: nuclear-mcp-service
spec:
  selector:
    app: nuclear-mcp
  ports:
  - protocol: TCP
    port: 8079
    targetPort: 8079
  type: LoadBalancer
```

### Reverse Proxy with Nginx

Example `nginx.conf` for production:

```nginx
upstream nuclear_mcp {
    server nuclear-mcp:8079;
    keepalive 32;
}

server {
    listen 443 ssl http2;
    server_name api.yourdomain.com;

    ssl_certificate /etc/nginx/ssl/fullchain.pem;
    ssl_certificate_key /etc/nginx/ssl/privkey.pem;

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=mcp_limit:10m rate=10r/s;
    limit_req zone=mcp_limit burst=20 nodelay;

    location / {
        proxy_pass http://nuclear_mcp;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # Timeouts for long-running requests
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }
}
```

---

## Environment Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Logging level (error, warn, info, debug, trace) |
| `MCP_PORT` | `8079` | HTTP server port |
| `RATE_LIMIT` | `100` | Requests per second per tool |
| `CACHE_TTL` | `300` | Cache TTL in seconds |
| `STORAGE_PATH` | `./resultados` | Results storage directory |
| `MAX_CONCURRENT_REQUESTS` | `1000` | Max concurrent tool invocations |
| `WEBSEARCH_TIMEOUT` | `5` | WebSearch timeout (seconds) |
| `DEEPWEB_TIMEOUT` | `10` | DeepWeb timeout (seconds) |
| `PREMIUM_TIMEOUT` | `15` | Premium scraper timeout (seconds) |
| `FILE_SEARCH_TIMEOUT` | `8` | File search timeout (seconds) |

### Configuration File

Create a `.env` file in the project root:

```bash
# Logging
RUST_LOG=info

# Server
MCP_PORT=8079

# Rate Limiting
RATE_LIMIT=100
MAX_CONCURRENT_REQUESTS=1000

# Caching
CACHE_TTL=300

# Storage
STORAGE_PATH=./resultados

# Tool Timeouts (seconds)
WEBSEARCH_TIMEOUT=5
DEEPWEB_TIMEOUT=10
PREMIUM_TIMEOUT=15
FILE_SEARCH_TIMEOUT=8

# FFI Libraries
GO_LIB_PATH=./libs/libstealth_go.so
ZIG_LIB_PATH=./libs/libzig_simd.so
NIM_LIB_PATH=./libs/libnuclear_nim.so
```

---

## Monitoring & Health Checks

### Health Check Endpoint

The server exposes a health check at the root path:

```bash
curl http://localhost:8079/
```

**Response**:
```json
{
  "status": "ok",
  "version": "0.1.0",
  "uptime_seconds": 3600,
  "tools_available": 4
}
```

### Docker Health Check

Built into `Dockerfile` and `docker-compose.yml`:

```bash
# Check container health
docker ps

# View health logs
docker inspect --format='{{json .State.Health}}' nuclear-mcp-server
```

### Prometheus Metrics (Future)

Planned metrics endpoint at `/metrics`:

```
# HELP nuclear_mcp_requests_total Total number of tool invocations
# TYPE nuclear_mcp_requests_total counter
nuclear_mcp_requests_total{tool="websearch"} 1234

# HELP nuclear_mcp_request_duration_seconds Tool execution duration
# TYPE nuclear_mcp_request_duration_seconds histogram
nuclear_mcp_request_duration_seconds_bucket{tool="websearch",le="1"} 100
nuclear_mcp_request_duration_seconds_bucket{tool="websearch",le="2"} 200
```

### Log Aggregation

For production, use centralized logging:

```yaml
# docker-compose.yml with logging
services:
  nuclear-mcp:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
        labels: "app=nuclear-mcp"
```

Or ship logs to ELK/Loki:

```yaml
services:
  nuclear-mcp:
    logging:
      driver: "fluentd"
      options:
        fluentd-address: "localhost:24224"
        tag: "nuclear-mcp"
```

---

## Troubleshooting

### Common Issues

#### 1. Port Already in Use

**Error**: `Address already in use (os error 98)`

**Solution**:
```bash
# Find process using port 8079
lsof -i :8079

# Kill the process
kill -9 <PID>

# Or use a different port
./nuclear-mcp --port 8080
```

#### 2. FFI Library Not Found

**Error**: `error while loading shared libraries: libstealth_go.so`

**Solution**:
```bash
# Set library path
export LD_LIBRARY_PATH=$PWD/libs:$LD_LIBRARY_PATH

# Or copy to system library directory
sudo cp libs/*.so /usr/local/lib/
sudo ldconfig
```

#### 3. Docker Container Exits Immediately

**Check logs**:
```bash
docker logs nuclear-mcp-server
```

**Common causes**:
- Missing FFI libraries in container
- Port conflict
- Insufficient memory

**Solution**:
```bash
# Rebuild with verbose output
docker build --no-cache -t nuclear-crawler-hybrid:debug .

# Run with more resources
docker run -m 4g --cpus=4 nuclear-crawler-hybrid:debug
```

#### 4. Slow Performance

**Symptoms**: Requests taking longer than expected

**Diagnostics**:
```bash
# Check resource usage
docker stats nuclear-mcp-server

# Enable debug logging
docker restart -e RUST_LOG=debug nuclear-mcp-server
```

**Solutions**:
- Increase resource limits
- Enable FFI features for parallelism
- Adjust rate limits
- Check network latency

#### 5. Cache Not Working

**Symptoms**: Same requests not returning cached results

**Check**:
```bash
# Verify cache TTL
docker exec nuclear-mcp-server env | grep CACHE_TTL

# Check disk space
df -h
```

**Solution**:
```bash
# Increase TTL
docker restart -e CACHE_TTL=600 nuclear-mcp-server

# Clear cache directory if corrupted
rm -rf /tmp/cache/*
```

---

## Performance Tuning

### Resource Allocation

| Deployment | CPU Cores | RAM | Disk I/O |
|------------|-----------|-----|----------|
| **Development** | 2 | 2GB | Standard |
| **Small Production** | 4 | 4GB | SSD |
| **Large Production** | 8+ | 8GB+ | NVMe SSD |

### Optimization Tips

1. **Enable FFI Features**: Use Go, Zig, Nim for maximum performance
2. **Increase Rate Limits**: For dedicated deployments
3. **Use SSD Storage**: For `resultados/` directory
4. **Enable Caching**: Set appropriate TTL values
5. **Load Balancing**: Deploy multiple instances behind nginx
6. **Connection Pooling**: HTTP client reuse (automatic in Rust)

### Benchmarking

```bash
# Install wrk for HTTP benchmarking
sudo apt-get install wrk

# Benchmark MCP server
wrk -t4 -c100 -d30s --latency http://localhost:8079/

# Results:
# Requests/sec:   1000+
# Latency avg:    <100ms
# Latency 99th:   <500ms
```

---

## Security Considerations

### Production Checklist

- [ ] Use HTTPS with valid SSL certificates
- [ ] Implement authentication (API keys, OAuth)
- [ ] Enable rate limiting per IP
- [ ] Restrict CORS origins
- [ ] Use firewall rules to limit access
- [ ] Regular security updates
- [ ] Monitor for suspicious activity
- [ ] Backup `resultados/` directory
- [ ] Use secrets management (not `.env` files)
- [ ] Enable audit logging

### Example Secure Deployment

```yaml
# docker-compose.production.yml
version: '3.8'

services:
  nuclear-mcp:
    image: ghcr.io/rigohl/nuclear-crawler-hybrid:latest
    environment:
      - RUST_LOG=warn
    secrets:
      - api_key
    networks:
      - internal
    deploy:
      mode: replicated
      replicas: 3
      restart_policy:
        condition: on-failure

  nginx:
    image: nginx:alpine
    ports:
      - "443:443"
    volumes:
      - ./nginx-secure.conf:/etc/nginx/nginx.conf:ro
      - /etc/letsencrypt:/etc/nginx/ssl:ro
    networks:
      - internal
      - public

networks:
  internal:
    driver: overlay
    internal: true
  public:
    driver: overlay

secrets:
  api_key:
    external: true
```

---

## Backup & Recovery

### Backup Strategy

```bash
# Backup results directory
tar -czf nuclear-results-$(date +%Y%m%d).tar.gz resultados/

# Backup configuration
tar -czf nuclear-config-$(date +%Y%m%d).tar.gz .env docker-compose.yml

# Upload to S3 (example)
aws s3 cp nuclear-results-*.tar.gz s3://backups/nuclear-mcp/
```

### Automated Backups

Add to crontab:

```cron
# Daily backup at 2 AM
0 2 * * * /usr/local/bin/backup-nuclear-mcp.sh
```

### Recovery

```bash
# Extract backup
tar -xzf nuclear-results-20250629.tar.gz

# Restore to container
docker cp resultados/ nuclear-mcp-server:/app/

# Restart container
docker restart nuclear-mcp-server
```

---

## Support & Resources

- **GitHub Issues**: https://github.com/Rigohl/nuclear-crawler-hybrid/issues
- **Documentation**: See [README.md](README.md) and [ARCHITECTURE.md](ARCHITECTURE.md)
- **Contributing**: See [CONTRIBUTING.md](CONTRIBUTING.md)

---

**Last Updated**: 2025-12-29  
**Document Version**: 1.0.0
