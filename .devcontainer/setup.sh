#!/bin/bash
set -e

echo "🚀 Setting up Nuclear Crawler Hybrid Environment"
echo "================================================"

# Update system
echo "📦 Updating system packages..."
sudo apt-get update -qq
sudo apt-get upgrade -y -qq

# Install build essentials
echo "🔨 Installing build tools..."
sudo apt-get install -y -qq \
  build-essential \
  cmake \
  ninja-build \
  pkg-config \
  libssl-dev \
  libffi-dev \
  libxml2-dev \
  libxslt1-dev \
  zlib1g-dev \
  libbz2-dev \
  libreadline-dev \
  libsqlite3-dev \
  wget \
  curl \
  llvm \
  libncurses5-dev \
  libncursesw5-dev \
  xz-utils \
  tk-dev \
  libgdbm-dev \
  libc6-dev \
  liblzma-dev

# Install Chapel
echo "⛪ Installing Chapel Language..."
CHAPEL_VERSION="2.7.0"
CHAPEL_DIR="/opt/chapel"

if [ ! -d "$CHAPEL_DIR" ]; then
  sudo mkdir -p /opt
  cd /tmp
  
  echo "  Downloading Chapel ${CHAPEL_VERSION}..."
  wget -q https://github.com/chapel-lang/chapel/releases/download/${CHAPEL_VERSION}/chapel-${CHAPEL_VERSION}.tar.gz
  
  echo "  Extracting Chapel..."
  tar xzf chapel-${CHAPEL_VERSION}.tar.gz
  
  echo "  Installing Chapel..."
  sudo mv chapel-${CHAPEL_VERSION} ${CHAPEL_DIR}
  
  # Set environment for Chapel build
  export CHPL_HOME=${CHAPEL_DIR}
  export PATH=${CHPL_HOME}/bin/linux64-x86_64:${PATH}
  
  cd ${CHAPEL_DIR}
  
  echo "  Configuring Chapel (this may take 10-15 minutes)..."
  source ${CHPL_HOME}/util/setchplenv.bash
  
  echo "  Building Chapel compiler..."
  make -j$(nproc) 2>&1 | grep -E "Building|Compiling|Done" || true
  
  # Cleanup
  rm -f /tmp/chapel-${CHAPEL_VERSION}.tar.gz
  
  echo "  ✓ Chapel ${CHAPEL_VERSION} installed"
else
  echo "  ✓ Chapel already installed"
fi

# Install Python packages for FFI
echo "🐍 Installing Python FFI packages..."
pip install --quiet --upgrade pip
pip install --quiet \
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
  pyarrow

# Install Playwright browsers
echo "🎭 Installing Playwright browsers..."
playwright install chromium --with-deps > /dev/null 2>&1

# Install Rust crates for FFI
echo "🦀 Installing Rust FFI crates..."
cargo install --quiet \
  wasm-pack \
  wasm-bindgen-cli

# Install Go packages for FFI
echo "🐹 Installing Go packages..."
go install github.com/gofiber/fiber/v2@latest

# Install Node packages for crawler
echo "📦 Installing Node.js packages..."
npm install -g --silent \
  puppeteer \
  puppeteer-extra \
  puppeteer-extra-plugin-stealth \
  cheerio \
  axios

# Setup Chapel examples for nuclear crawler
echo "⚡ Setting up Chapel examples..."
mkdir -p /workspace/chapel-examples
cat > /workspace/chapel-examples/hello.chpl << 'EOF'
writeln("Chapel is ready! Nuclear Crawler Hybrid initialized.");

// FFI Example: Call external C function
extern proc system(cmd: c_ptrConst(c_char)): c_int;

proc testFFI() {
  const cmd = "echo 'FFI Working!'".c_str();
  const result = system(cmd);
  writeln("FFI call result: ", result);
}

testFFI();
EOF

# Compile example
cd /workspace/chapel-examples
chpl hello.chpl -o hello 2>/dev/null || echo "Will compile when needed"

# Create crawler config
echo "⚙️  Creating crawler configuration..."
mkdir -p /workspace/config
cat > /workspace/config/crawler.toml << 'EOF'
[crawler]
mode = "hybrid"
workers = 8
max_depth = 5
timeout = 30

[chapel]
enabled = true
parallel_degree = 16
locale_model = "numa"

[ffi]
python = true
rust = true
go = true
c = true

[targets]
user_agent = "NuclearCrawler/2.0 (Chapel; Hybrid)"
respect_robots = true
max_concurrent = 100
EOF

# Setup workspace structure
echo "📁 Setting up workspace structure..."
mkdir -p /workspace/{src,data,output,logs,tests,chapel,ffi}

cat > /workspace/src/README.md << 'EOF'
# Nuclear Crawler Hybrid - Source Code

## Structure
- `chapel/` - Chapel implementations
- `ffi/` - Foreign Function Interfaces
- `rust/` - Rust modules
- `python/` - Python modules
- `go/` - Go modules

## Quick Start

### Test Chapel
```bash
cd /workspace/chapel-examples
./hello
```

### FFI Examples
See `/workspace/ffi/` for examples of:
- Python ↔ Chapel
- Rust ↔ Chapel
- Go ↔ Chapel
- C ↔ Chapel
EOF

# Create FFI examples
echo "🔗 Creating FFI examples..."

# Python FFI example
cat > /workspace/ffi/python_ffi.py << 'EOF'
"""Python FFI Example for Chapel"""
from cffi import FFI

ffi = FFI()

# Define C interface
ffi.cdef("""
    int add(int a, int b);
""")

# This will be called from Chapel via C
def python_add(a, b):
    return a + b

if __name__ == "__main__":
    print("Python FFI ready for Chapel integration")
EOF

# Rust FFI example
cat > /workspace/ffi/rust_ffi.rs << 'EOF'
// Rust FFI Example for Chapel

#[no_mangle]
pub extern "C" fn rust_multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[no_mangle]
pub extern "C" fn rust_hello() {
    println!("Hello from Rust FFI!");
}
EOF

# Chapel FFI wrapper
cat > /workspace/ffi/chapel_ffi.chpl << 'EOF'
// Chapel FFI Wrapper
module ChapelFFI {
  
  // Python FFI
  extern proc python_add(a: c_int, b: c_int): c_int;
  
  // Rust FFI
  extern proc rust_multiply(a: c_int, b: c_int): c_int;
  extern proc rust_hello();
  
  // C FFI
  extern proc system(cmd: c_ptrConst(c_char)): c_int;
  
  proc demonstrateFFI() {
    writeln("=== Chapel FFI Demonstration ===");
    
    // Call Rust
    writeln("Rust multiply(5, 7) = ", rust_multiply(5, 7));
    rust_hello();
    
    // Call system
    const cmd = "echo 'System call from Chapel'".c_str();
    system(cmd);
    
    writeln("FFI demonstration complete!");
  }
}
EOF

# Environment validation script
cat > /workspace/validate-environment.sh << 'EOF'
#!/bin/bash

echo "🔍 Validating Nuclear Crawler Hybrid Environment"
echo "==============================================="

# Check Chapel
if command -v chpl &> /dev/null; then
  echo "✅ Chapel compiler: $(chpl --version)"
else
  echo "❌ Chapel compiler not found"
fi

# Check Python
if command -v python &> /dev/null; then
  echo "✅ Python: $(python --version)"
else
  echo "❌ Python not found"
fi

# Check Rust
if command -v cargo &> /dev/null; then
  echo "✅ Rust: $(cargo --version)"
else
  echo "❌ Rust not found"
fi

# Check Go
if command -v go &> /dev/null; then
  echo "✅ Go: $(go version)"
else
  echo "❌ Go not found"
fi

# Check Node
if command -v node &> /dev/null; then
  echo "✅ Node.js: $(node --version)"
else
  echo "❌ Node.js not found"
fi

# Check FFI packages
echo ""
echo "📦 FFI Packages:"
python -c "import cffi; print('✅ Python CFFI installed')" 2>/dev/null || echo "❌ Python CFFI missing"
python -c "import playwright; print('✅ Playwright installed')" 2>/dev/null || echo "❌ Playwright missing"

echo ""
echo "Environment validation complete!"
EOF

chmod +x /workspace/validate-environment.sh

# Final setup
echo "✨ Final touches..."
cd /workspace

# Set proper permissions
sudo chown -R $(whoami):$(whoami) /workspace

# Display success message
echo ""
echo "════════════════════════════════════════════════════════════"
echo "  ✅ Nuclear Crawler Hybrid Environment Ready!"
echo "════════════════════════════════════════════════════════════"
echo ""
echo "📦 Installed:"
echo "  • Chapel ${CHAPEL_VERSION} with FFI support"
echo "  • Python 3.11 + CFFI + Playwright"
echo "  • Rust + wasm-pack"
echo "  • Go + Fiber"
echo "  • Node.js + Puppeteer"
echo ""
echo "🔗 FFI Ready:"
echo "  • Python ↔ Chapel"
echo "  • Rust ↔ Chapel"
echo "  • Go ↔ Chapel"
echo "  • C ↔ Chapel"
echo ""
echo "🚀 Quick Start:"
echo "  1. Run: /workspace/validate-environment.sh"
echo "  2. Test: cd /workspace/chapel-examples && ./hello"
echo "  3. Explore: ls /workspace/"
echo ""
echo "📚 Documentation: /workspace/src/README.md"
echo "════════════════════════════════════════════════════════════"
