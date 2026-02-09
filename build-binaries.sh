#!/bin/bash
# 🔥 NUCLEAR CRAWLER HYBRID - BUILD & DEPLOY SCRIPT
# Compila 3 binarios poderosos: nuclear-mcp (7 tools), nuclear-pro (5 tools), nuclear-lite (2 tools)

set -e

echo "🚀 NUCLEAR CRAWLER HYBRID - BUILD SYSTEM"
echo "======================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 1. Clean build
echo -e "${BLUE}[1/4] Cleaning previous build...${NC}"
cargo clean --quiet

# 2. Build library
echo -e "${BLUE}[2/4] Building library (src/lib.rs)...${NC}"
cargo build --release --lib 2>&1 | tail -3

# 3. Build the 3 binaries
echo ""
echo -e "${BLUE}[3/4] Building 3 MCP binaries...${NC}"
echo -e "  🔴 nuclear-mcp   (Full: 7 tools)"
echo -e "  🟡 nuclear-pro   (Pro: 5 tools)"
echo -e "  🟢 nuclear-lite  (Lite: 2 tools)"
echo ""

cargo build --release --bin nuclear-mcp --bin nuclear-pro --bin nuclear-lite 2>&1 | tail -3

# 4. Verify binaries exist
echo ""
echo -e "${BLUE}[4/4] Verifying binaries...${NC}"

BINARIES=(
  "target/release/nuclear-mcp"
  "target/release/nuclear-pro"
  "target/release/nuclear-lite"
)

# Windows adds .exe extension
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
  BINARIES=(
    "target/release/nuclear-mcp.exe"
    "target/release/nuclear-pro.exe"
    "target/release/nuclear-lite.exe"
  )
fi

SUCCESS=true
for binary in "${BINARIES[@]}"; do
  if [ -f "$binary" ]; then
    size=$(ls -lh "$binary" | awk '{print $5}')
    echo -e "${GREEN}✅${NC} $binary ($size)"
  else
    echo -e "${RED}❌${NC} $binary NOT FOUND"
    SUCCESS=false
  fi
done

echo ""
if [ "$SUCCESS" = true ]; then
  echo -e "${GREEN}=== BUILD SUCCESS ===${NC}"
  echo ""
  echo -e "${YELLOW}📦 Binaries ready:${NC}"
  echo "  - nuclear-mcp (Full power - 7 MCP tools)"
  echo "  - nuclear-pro (Professional - 5 MCP tools)"  
  echo "  - nuclear-lite (Minimal - 2 MCP tools)"
  echo ""
  echo -e "${YELLOW}🚀 Ready to deploy/run:${NC}"
  if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    echo "  ./target/release/nuclear-mcp.exe"
    echo "  ./target/release/nuclear-pro.exe"
    echo "  ./target/release/nuclear-lite.exe"
  else
    echo "  ./target/release/nuclear-mcp"
    echo "  ./target/release/nuclear-pro"
    echo "  ./target/release/nuclear-lite"
  fi
else
  echo -e "${RED}=== BUILD FAILED ===${NC}"
  exit 1
fi
