# Nuclear Crawler Hybrid - Full Stack Container
# Chapel + Python + Rust + Go + Node.js + FFI

FROM ubuntu:22.04

# Prevent interactive prompts
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC

# Chapel environment
ENV CHPL_HOME=/opt/chapel
ENV CHPL_VERSION=2.1.0
ENV PATH=${CHPL_HOME}/bin/linux64-x86_64:${PATH}
ENV LD_LIBRARY_PATH=${CHPL_HOME}/lib/linux64-x86_64:${LD_LIBRARY_PATH}

# Install system dependencies
RUN apt-get update && apt-get install -y \
    # Build essentials
    build-essential \
    cmake \
    ninja-build \
    pkg-config \
    git \
    wget \
    curl \
    ca-certificates \
    # Chapel dependencies
    gcc \
    g++ \
    clang \
    llvm \
    m4 \
    perl \
    python3 \
    python3-pip \
    python3-dev \
    # FFI libraries
    libffi-dev \
    libssl-dev \
    libxml2-dev \
    libxslt1-dev \
    zlib1g-dev \
    libbz2-dev \
    libreadline-dev \
    libsqlite3-dev \
    libncurses5-dev \
    libncursesw5-dev \
    xz-utils \
    tk-dev \
    libgdbm-dev \
    libc6-dev \
    liblzma-dev \
    # Browser dependencies for Playwright
    libnss3 \
    libnspr4 \
    libatk1.0-0 \
    libatk-bridge2.0-0 \
    libcups2 \
    libdrm2 \
    libdbus-1-3 \
    libxkbcommon0 \
    libatspi2.0-0 \
    libxcomposite1 \
    libxdamage1 \
    libxfixes3 \
    libxrandr2 \
    libgbm1 \
    libpango-1.0-0 \
    libcairo2 \
    libasound2 \
    && rm -rf /var/lib/apt/lists/*

# Install Chapel
RUN cd /tmp && \
    wget -q https://github.com/chapel-lang/chapel/releases/download/${CHPL_VERSION}/chapel-${CHPL_VERSION}.tar.gz && \
    tar xzf chapel-${CHPL_VERSION}.tar.gz && \
    mv chapel-${CHPL_VERSION} ${CHPL_HOME} && \
    rm chapel-${CHPL_VERSION}.tar.gz && \
    cd ${CHPL_HOME} && \
    . ${CHPL_HOME}/util/setchplenv.bash && \
    make -j$(nproc) && \
    echo "Chapel installed successfully"

# Install Node.js 20
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g npm@latest

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:${PATH}

# Install Go
RUN wget -q https://go.dev/dl/go1.21.0.linux-amd64.tar.gz && \
    tar -C /usr/local -xzf go1.21.0.linux-amd64.tar.gz && \
    rm go1.21.0.linux-amd64.tar.gz
ENV PATH=/usr/local/go/bin:${PATH}

# Install Python packages
RUN pip3 install --no-cache-dir \
    cffi \
    pycparser \
    numpy \
    pandas \
    requests \
    aiohttp \
    beautifulsoup4 \
    lxml \
    selenium \
    playwright \
    scrapy \
    httpx \
    trio \
    uvloop \
    orjson \
    msgpack \
    pyarrow \
    && playwright install chromium

# Install Rust tools
RUN cargo install wasm-pack wasm-bindgen-cli

# Install Node packages
RUN npm install -g --silent \
    puppeteer \
    puppeteer-extra \
    puppeteer-extra-plugin-stealth \
    cheerio \
    axios

# Create workspace
WORKDIR /workspace

# Expose ports
EXPOSE 3000 8000 8080

# Default command
CMD ["/bin/bash"]
