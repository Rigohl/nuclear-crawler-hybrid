# Dockerfile for Nuclear Crawler Hybrid MCP Server (Windows with FFI)
# Multi-stage: Builder + Runtime

# ===== BUILDER STAGE =====
FROM mcr.microsoft.com/windows/servercore:ltsc2022 as builder

WORKDIR /build

# Install Rust
RUN powershell -Command \
    $ProgressPreference = 'SilentlyContinue'; \
    Invoke-WebRequest https://sh.rustup.rs/rustup-init.exe -OutFile rustup-init.exe; \
    .\rustup-init.exe -y --no-modify-path; \
    Remove-Item rustup-init.exe

# Install Go
RUN powershell -Command \
    $ProgressPreference = 'SilentlyContinue'; \
    Invoke-WebRequest https://golang.org/dl/go1.21.0.windows-amd64.msi -OutFile go.msi; \
    Start-Process msiexec.exe -ArgumentList '/i', 'go.msi', '/quiet' -Wait; \
    Remove-Item go.msi

# Install Nim
RUN powershell -Command \
    $ProgressPreference = 'SilentlyContinue'; \
    Invoke-WebRequest https://nim-lang.org/download/nim-1.6.14_x64.zip -OutFile nim.zip; \
    Expand-Archive nim.zip -DestinationPath C:\; \
    Remove-Item nim.zip

# Install Visual C++ Build Tools (for compilation)
RUN powershell -Command \
    $ProgressPreference = 'SilentlyContinue'; \
    Invoke-WebRequest https://aka.ms/vs/17/release/vs_BuildTools.exe -OutFile vs_buildtools.exe; \
    .\vs_buildtools.exe --quiet --norestart --wait --add Microsoft.VisualStudio.Workload.MSBuildTools; \
    Remove-Item vs_buildtools.exe

# Copy source code
COPY . /build/

# Build with FFI
RUN powershell -Command \
    $env:Path += ';C:\Users\ContainerAdministrator\.cargo\bin;C:\Program Files\Go\bin;C:\nim-1.6.14\bin'; \
    cd /build; \
    cargo build --release --bin nuclear-mcp 2>&1; \
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

# ===== RUNTIME STAGE =====
FROM mcr.microsoft.com/windows/servercore:ltsc2022

WORKDIR /app

# Install runtime dependencies
RUN powershell -Command \
    $ProgressPreference = 'SilentlyContinue'; \
    Add-WindowsFeature Net-Framework-45-Core

# Copy binary from builder
COPY --from=builder /build/target/release/nuclear-mcp.exe /app/nuclear-mcp.exe

# Expose port
EXPOSE 8079

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD powershell -Command try { $response = Invoke-WebRequest http://localhost:8079/ -UseBasicParsing; if ($response.StatusCode -eq 200) { exit 0 } } catch { exit 1 }

# Run server
ENTRYPOINT ["cmd", "/c", "C:\\app\\nuclear-mcp.exe"]
