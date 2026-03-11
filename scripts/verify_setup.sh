#!/bin/bash
# Nuclear Crawler Hybrid - Setup Verification Script
# Checks all required dependencies and outputs a JSON summary for CI integration

set -euo pipefail

# Colors for human-readable output (suppressed in JSON mode)
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Output mode: pass --json flag for JSON output
JSON_MODE=false
if [[ "${1:-}" == "--json" ]]; then
    JSON_MODE=true
fi

# ─────────────────────────────────────────────────────────────────────────
# Version extraction helpers
# ─────────────────────────────────────────────────────────────────────────

get_rust_version() {
    command -v rustc &>/dev/null && rustc --version 2>/dev/null | awk '{print $2}' || echo "not_found"
}

get_go_version() {
    command -v go &>/dev/null && go version 2>/dev/null | awk '{print $3}' | sed 's/go//' || echo "not_found"
}

get_zig_version() {
    command -v zig &>/dev/null && zig version 2>/dev/null || echo "not_found"
}

get_chapel_version() {
    command -v chpl &>/dev/null && chpl --version 2>/dev/null | head -1 | awk '{print $NF}' || echo "not_found"
}

get_nim_version() {
    command -v nim &>/dev/null && nim --version 2>/dev/null | head -1 | awk '{print $4}' || echo "not_found"
}

get_python_version() {
    command -v python3 &>/dev/null && python3 --version 2>/dev/null | awk '{print $2}' || echo "not_found"
}

get_wasm_pack_version() {
    command -v wasm-pack &>/dev/null && wasm-pack --version 2>/dev/null | awk '{print $2}' || echo "not_found"
}

# ─────────────────────────────────────────────────────────────────────────
# Minimum version checks
# ─────────────────────────────────────────────────────────────────────────

RUST_MIN="1.75"
GO_MIN="1.21"
ZIG_MIN="0.12.0"
PYTHON_MIN="3.11"

version_ge() {
    # Returns 0 if $1 >= $2 (semver comparison)
    [ "$(printf '%s\n' "$1" "$2" | sort -V | head -1)" = "$2" ]
}

# ─────────────────────────────────────────────────────────────────────────
# Gather versions
# ─────────────────────────────────────────────────────────────────────────

RUST_VER=$(get_rust_version)
GO_VER=$(get_go_version)
ZIG_VER=$(get_zig_version)
CHAPEL_VER=$(get_chapel_version)
NIM_VER=$(get_nim_version)
PYTHON_VER=$(get_python_version)
WASM_PACK_VER=$(get_wasm_pack_version)

if $JSON_MODE; then
    # JSON output for CI integration
    cat <<JSON
{
  "rust": "${RUST_VER}",
  "go": "${GO_VER}",
  "zig": "${ZIG_VER}",
  "chapel": "${CHAPEL_VER}",
  "nim": "${NIM_VER}",
  "python": "${PYTHON_VER}",
  "wasm_pack": "${WASM_PACK_VER}"
}
JSON
    exit 0
fi

# ─────────────────────────────────────────────────────────────────────────
# Human-readable output
# ─────────────────────────────────────────────────────────────────────────

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║          NUCLEAR CRAWLER HYBRID - SETUP VERIFICATION          ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

PASSED=0
FAILED=0
WARNING=0

check_version() {
    local tool="$1"
    local version="$2"
    local min_version="$3"
    local required="${4:-true}"

    if [ "$version" = "not_found" ]; then
        if [ "$required" = "true" ]; then
            echo -e "${RED}❌${NC} ${tool}: NOT INSTALLED (required >=${min_version})"
            ((FAILED++))
        else
            echo -e "${YELLOW}⚠️ ${NC} ${tool}: NOT INSTALLED (optional, >=${min_version} recommended)"
            ((WARNING++))
        fi
        return 1
    fi

    if version_ge "$version" "$min_version"; then
        echo -e "${GREEN}✅${NC} ${tool}: ${version} (>=${min_version} ✓)"
        ((PASSED++))
        return 0
    else
        echo -e "${YELLOW}⚠️ ${NC} ${tool}: ${version} (below recommended >=${min_version})"
        ((WARNING++))
        return 1
    fi
}

echo -e "${BLUE}🔍 REQUIRED DEPENDENCIES${NC}"
echo "─────────────────────────────────────────────────────────────────────────"
check_version "Rust (rustc)" "$RUST_VER" "$RUST_MIN" "true"
check_version "Go" "$GO_VER" "$GO_MIN" "true"
check_version "Python" "$PYTHON_VER" "$PYTHON_MIN" "true"

echo ""
echo -e "${BLUE}🔧 OPTIONAL FFI BACKENDS${NC}"
echo "─────────────────────────────────────────────────────────────────────────"
check_version "Zig" "$ZIG_VER" "$ZIG_MIN" "false"
check_version "Chapel (chpl)" "$CHAPEL_VER" "2.0.0" "false"
check_version "Nim" "$NIM_VER" "1.6.0" "false"

echo ""
echo -e "${BLUE}🕸️  WASM TOOLCHAIN${NC}"
echo "─────────────────────────────────────────────────────────────────────────"
check_version "wasm-pack" "$WASM_PACK_VER" "0.12.0" "false"

echo ""
echo -e "${BLUE}📄 KEY FILES${NC}"
echo "─────────────────────────────────────────────────────────────────────────"

check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✅${NC} $1"
        ((PASSED++))
    else
        echo -e "${YELLOW}⚠️ ${NC} $1 not found"
        ((WARNING++))
    fi
}

check_file "Cargo.toml"
check_file "ffi/chapel/Makefile"
check_file "ffi/wasm/go/main.go"
check_file "ffi/wasm/zig/main.zig"
check_file "ffi/wasm/nim/main.nim"

# ─────────────────────────────────────────────────────────────────────────
echo ""
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                          SUMMARY                              ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo -e "✅ Passed:   ${GREEN}$PASSED${NC}"
echo -e "⚠️  Warnings: ${YELLOW}$WARNING${NC}"
echo -e "❌ Failed:   ${RED}$FAILED${NC}"
echo ""

echo "📦 Version Summary (JSON):"
"$0" --json 2>/dev/null || true
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🟢 SYSTEM READY${NC}"
    echo ""
    echo "Quick build commands:"
    echo "  cargo build --release          # Rust core"
    echo "  cd ffi/chapel && make check    # Chapel AI"
    echo "  cd ffi/wasm/go && go build ./... # Go FFI"
    exit 0
else
    echo -e "${RED}🔴 ISSUES FOUND - install missing required dependencies${NC}"
    exit 1
fi

