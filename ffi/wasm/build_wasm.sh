#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════
# 🔥 WASM BUILD SCRIPT - Compile Go, Zig, Nim to WebAssembly
# ═══════════════════════════════════════════════════════════════════════════
#
# Usage:
#   ./build_wasm.sh [all|go|zig|nim]
#
# Requirements:
#   Go:  tinygo (https://tinygo.org/)
#   Zig: zig 0.12+ (https://ziglang.org/)
#   Nim: nim + emscripten (https://nim-lang.org/)
#
# Output:
#   ffi/wasm/go/go_parallel.wasm   - Parallel processing (goroutines)
#   ffi/wasm/zig/zig_simd.wasm     - SIMD operations (hashing, matching)
#   ffi/wasm/nim/nim_parser.wasm   - HTML parsing & extraction
#
# Powers ALL 7 MCP Tools:
#   websearch       <- Go (parallel fetch) + Nim (HTML parse)
#   premium         <- Nim (content extract) + Go (parallel)
#   file_search     <- Zig (SIMD pattern match) + Zig (hashing)
#   scan            <- Zig (SIMD) + Go (parallel scan)
#   ai_dataset_trainer <- Go (data process) + Zig (transform)
#   parallel_engine <- Go (goroutines) + Zig (SIMD batch)
#   osint_intelligence <- Go (parallel fetch) + Nim (parse) + Zig (score)
# ═══════════════════════════════════════════════════════════════════════════

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WASM_DIR="${SCRIPT_DIR}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo ""
echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║  🔥 NUCLEAR WASM BUILD SYSTEM - Maximum Power                    ║"
echo "║  Compiling Go + Zig + Nim → WebAssembly                          ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

build_go() {
    echo -e "${BLUE}[GO WASM]${NC} Building parallel processor..."
    
    if ! command -v tinygo &> /dev/null; then
        echo -e "${YELLOW}⚠️  TinyGo not found. Install: https://tinygo.org/getting-started/install/${NC}"
        echo -e "${YELLOW}    Fallback: Using standard Go WASM compilation${NC}"
        
        if command -v go &> /dev/null; then
            cd "${WASM_DIR}/go"
            GOOS=js GOARCH=wasm go build -o go_parallel.wasm main.go
            echo -e "${GREEN}✅ Go WASM built (standard): go_parallel.wasm${NC}"
        else
            echo -e "${RED}❌ Go not found. Skip Go WASM build.${NC}"
            return 1
        fi
    else
        cd "${WASM_DIR}/go"
        tinygo build -o go_parallel.wasm -target wasm -no-debug -opt=2 -scheduler=asyncify main.go
        echo -e "${GREEN}✅ Go WASM built (TinyGo optimized): go_parallel.wasm${NC}"
    fi
    
    if [ -f go_parallel.wasm ]; then
        SIZE=$(du -h go_parallel.wasm | cut -f1)
        echo -e "${GREEN}   Size: ${SIZE}${NC}"
    fi
}

build_zig() {
    echo -e "${BLUE}[ZIG WASM]${NC} Building SIMD processor..."
    
    if ! command -v zig &> /dev/null; then
        echo -e "${RED}❌ Zig not found. Install: https://ziglang.org/download/${NC}"
        return 1
    fi
    
    cd "${WASM_DIR}/zig"
    zig build-lib -target wasm32-wasi -O ReleaseFast -dynamic main.zig -femit-bin=zig_simd.wasm 2>&1 || \
    zig build-lib -target wasm32-freestanding -O ReleaseFast main.zig -femit-bin=zig_simd.wasm 2>&1 || \
    echo -e "${YELLOW}⚠️  Zig WASM build failed - check Zig version compatibility${NC}"
    
    if [ -f zig_simd.wasm ]; then
        SIZE=$(du -h zig_simd.wasm | cut -f1)
        echo -e "${GREEN}✅ Zig WASM built: zig_simd.wasm (${SIZE})${NC}"
    fi
}

build_nim() {
    echo -e "${BLUE}[NIM WASM]${NC} Building HTML parser..."
    
    if ! command -v nim &> /dev/null; then
        echo -e "${RED}❌ Nim not found. Install: https://nim-lang.org/install.html${NC}"
        return 1
    fi
    
    cd "${WASM_DIR}/nim"
    
    if command -v emcc &> /dev/null; then
        nim c -d:release -d:emscripten --os:linux --cpu:wasm32 --gc:arc \
              --passC:"-s WASM=1 -O3" \
              --passL:"-s EXPORTED_FUNCTIONS=['_parse_html','_extract_links','_extract_metadata','_count_words','_detect_language','_alloc','_dealloc']" \
              -o:nim_parser.wasm main.nim
        echo -e "${GREEN}✅ Nim WASM built: nim_parser.wasm${NC}"
    else
        echo -e "${YELLOW}⚠️  Emscripten not found. Compiling Nim natively for testing.${NC}"
        nim c -d:release --gc:arc -o:nim_parser_native main.nim 2>/dev/null || true
    fi
    
    if [ -f nim_parser.wasm ]; then
        SIZE=$(du -h nim_parser.wasm | cut -f1)
        echo -e "${GREEN}   Size: ${SIZE}${NC}"
    fi
}

# Parse arguments
TARGET="${1:-all}"

case "$TARGET" in
    go)
        build_go
        ;;
    zig)
        build_zig
        ;;
    nim)
        build_nim
        ;;
    all)
        echo "Building ALL WASM modules..."
        echo ""
        build_go || true
        echo ""
        build_zig || true
        echo ""
        build_nim || true
        ;;
    *)
        echo "Usage: $0 [all|go|zig|nim]"
        exit 1
        ;;
esac

echo ""
echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║  ✅ WASM BUILD COMPLETE                                          ║"
echo "║  Modules power ALL 7 MCP tools via wasmtime runtime              ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

# List built modules
echo "Built WASM modules:"
for f in "${WASM_DIR}"/go/*.wasm "${WASM_DIR}"/zig/*.wasm "${WASM_DIR}"/nim/*.wasm; do
    if [ -f "$f" ]; then
        SIZE=$(du -h "$f" | cut -f1)
        echo "  ✅ $f (${SIZE})"
    fi
done
