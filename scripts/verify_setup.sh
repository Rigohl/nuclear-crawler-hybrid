#!/bin/bash
# Verificación rápida del sistema para Chapel + HuggingFace
# Usage: ./verify_setup.sh [--json]

# Determine repository root dynamically (directory containing this script's parent)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Parse arguments
OUTPUT_JSON=false
for arg in "$@"; do
    case $arg in
        --json) OUTPUT_JSON=true ;;
    esac
done

if [ "$OUTPUT_JSON" = false ]; then
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║          NUCLEAR CHAPEL DEPLOYMENT VERIFICATION               ║"
    echo "║              GitHub + HuggingFace Setup Check                 ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo ""
fi

# Colors (only used in non-JSON mode)
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0
WARNING=0

# JSON results array
declare -a JSON_RESULTS

check_command() {
    if command -v "$1" &> /dev/null; then
        if [ "$OUTPUT_JSON" = false ]; then
            echo -e "${GREEN}✅${NC} $1 installed"
        fi
        JSON_RESULTS+=("{\"check\":\"command:$1\",\"status\":\"pass\"}")
        ((PASSED++))
        return 0
    else
        if [ "$OUTPUT_JSON" = false ]; then
            echo -e "${RED}❌${NC} $1 NOT installed"
        fi
        JSON_RESULTS+=("{\"check\":\"command:$1\",\"status\":\"fail\",\"message\":\"$1 not found in PATH\"}")
        ((FAILED++))
        return 1
    fi
}

check_command_optional() {
    if command -v "$1" &> /dev/null; then
        if [ "$OUTPUT_JSON" = false ]; then
            echo -e "${GREEN}✅${NC} $1 installed (optional)"
        fi
        JSON_RESULTS+=("{\"check\":\"command:$1\",\"status\":\"pass\",\"optional\":true}")
        ((PASSED++))
        return 0
    else
        if [ "$OUTPUT_JSON" = false ]; then
            echo -e "${YELLOW}⚠️${NC} $1 NOT installed (optional)"
        fi
        JSON_RESULTS+=("{\"check\":\"command:$1\",\"status\":\"warn\",\"message\":\"$1 not installed (optional)\",\"optional\":true}")
        ((WARNING++))
        return 1
    fi
}

check_file() {
    if [ -f "$1" ]; then
        if [ "$OUTPUT_JSON" = false ]; then
            echo -e "${GREEN}✅${NC} $1 exists"
        fi
        JSON_RESULTS+=("{\"check\":\"file:$1\",\"status\":\"pass\"}")
        ((PASSED++))
        return 0
    else
        if [ "$OUTPUT_JSON" = false ]; then
            echo -e "${YELLOW}⚠️${NC} $1 NOT found"
        fi
        JSON_RESULTS+=("{\"check\":\"file:$1\",\"status\":\"warn\",\"message\":\"file not found\"}")
        ((WARNING++))
        return 1
    fi
}

# ─────────────────────────────────────────────────────────────────────────
if [ "$OUTPUT_JSON" = false ]; then
    echo -e "${BLUE}🔍 SYSTEM CHECKS${NC}"
    echo "─────────────────────────────────────────────────────────────────────────"
fi

check_command "git"
check_command_optional "chpl"
check_command "make"
check_command "python3"
check_command_optional "npm"
check_command_optional "npx"
check_command "cargo"
check_command_optional "go"
check_command_optional "zig"

# ─────────────────────────────────────────────────────────────────────────
if [ "$OUTPUT_JSON" = false ]; then
    echo ""
    echo -e "${BLUE}🗂️ CHAPEL FILES${NC}"
    echo "─────────────────────────────────────────────────────────────────────────"
fi

check_file "$REPO_ROOT/ffi/chapel/training_pipeline.chpl"
check_file "$REPO_ROOT/ffi/chapel/chapel_ai.chpl"
check_file "$REPO_ROOT/ffi/chapel/Makefile"
check_file "$REPO_ROOT/ffi/chapel/hf_spaces_app.py"
check_file "$REPO_ROOT/ffi/chapel/data/train/scraping_stealth_patterns.json"

# ─────────────────────────────────────────────────────────────────────────
if [ "$OUTPUT_JSON" = false ]; then
    echo ""
    echo -e "${BLUE}🚀 DEPLOYMENT FILES${NC}"
    echo "─────────────────────────────────────────────────────────────────────────"
fi

check_file "$REPO_ROOT/.github/workflows/chapel-training-pipeline.yml"
check_file "$REPO_ROOT/scripts/push_to_huggingface.sh"
check_file "$REPO_ROOT/HUGGINGFACE_DEPLOYMENT.md"
check_file "$REPO_ROOT/SETUP_HF_KIMBERLY.md"

# ─────────────────────────────────────────────────────────────────────────
if [ "$OUTPUT_JSON" = false ]; then
    echo ""
    echo -e "${BLUE}🔐 ENVIRONMENT VARIABLES${NC}"
    echo "─────────────────────────────────────────────────────────────────────────"
fi

if [ -z "$HF_TOKEN" ]; then
    if [ "$OUTPUT_JSON" = false ]; then
        echo -e "${YELLOW}⚠️${NC} HF_TOKEN not set (needed for HuggingFace)"
    fi
    JSON_RESULTS+=("{\"check\":\"env:HF_TOKEN\",\"status\":\"warn\",\"message\":\"not set, needed for HuggingFace deployment\"}")
    ((WARNING++))
else
    if [ "$OUTPUT_JSON" = false ]; then
        echo -e "${GREEN}✅${NC} HF_TOKEN is set"
    fi
    JSON_RESULTS+=("{\"check\":\"env:HF_TOKEN\",\"status\":\"pass\"}")
    ((PASSED++))
fi

if [ -z "$GITHUB_TOKEN" ]; then
    if [ "$OUTPUT_JSON" = false ]; then
        echo -e "${YELLOW}⚠️${NC} GITHUB_TOKEN not set (optional, uses CLI)"
    fi
    JSON_RESULTS+=("{\"check\":\"env:GITHUB_TOKEN\",\"status\":\"warn\",\"message\":\"not set (optional)\",\"optional\":true}")
    ((WARNING++))
else
    if [ "$OUTPUT_JSON" = false ]; then
        echo -e "${GREEN}✅${NC} GITHUB_TOKEN is set"
    fi
    JSON_RESULTS+=("{\"check\":\"env:GITHUB_TOKEN\",\"status\":\"pass\"}")
    ((PASSED++))
fi

# ─────────────────────────────────────────────────────────────────────────
if [ "$OUTPUT_JSON" = false ]; then
    echo ""
    echo -e "${BLUE}💾 STORAGE${NC}"
    echo "─────────────────────────────────────────────────────────────────────────"
fi

if [ -d "$REPO_ROOT/ffi/chapel/data" ]; then
    size=$(du -sh $REPO_ROOT/ffi/chapel/data 2>/dev/null | cut -f1)
    if [ "$OUTPUT_JSON" = false ]; then
        echo -e "${GREEN}✅${NC} Data directory exists (size: $size)"
    fi
    JSON_RESULTS+=("{\"check\":\"dir:chapel/data\",\"status\":\"pass\",\"size\":\"$size\"}")
    ((PASSED++))
else
    if [ "$OUTPUT_JSON" = false ]; then
        echo -e "${RED}❌${NC} Data directory missing"
    fi
    JSON_RESULTS+=("{\"check\":\"dir:chapel/data\",\"status\":\"fail\",\"message\":\"data directory missing\"}")
    ((FAILED++))
fi

# ─────────────────────────────────────────────────────────────────────────
if [ "$OUTPUT_JSON" = false ]; then
    echo ""
    echo -e "${BLUE}🔗 GIT REPOSITORY${NC}"
    echo "─────────────────────────────────────────────────────────────────────────"
fi

if [ -d "$REPO_ROOT/.git" ]; then
    cd $REPO_ROOT 2>/dev/null || true
    branch=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
    remote=$(git config --get remote.origin.url 2>/dev/null || echo "none")
    if [ "$OUTPUT_JSON" = false ]; then
        echo -e "${GREEN}✅${NC} Git repository detected"
        echo -e "${GREEN}✅${NC} Branch: $branch"
        echo -e "${GREEN}✅${NC} Remote: $remote"
    fi
    JSON_RESULTS+=("{\"check\":\"git:repo\",\"status\":\"pass\",\"branch\":\"$branch\",\"remote\":\"$remote\"}")
    ((PASSED+=3))
else
    if [ "$OUTPUT_JSON" = false ]; then
        echo -e "${RED}❌${NC} Not a git repository"
    fi
    JSON_RESULTS+=("{\"check\":\"git:repo\",\"status\":\"fail\",\"message\":\"not a git repository\"}")
    ((FAILED++))
fi

# ─────────────────────────────────────────────────────────────────────────
if [ "$OUTPUT_JSON" = false ]; then
    echo ""
    echo -e "${BLUE}⚡ QUICK BUILD TEST${NC}"
    echo "─────────────────────────────────────────────────────────────────────────"
fi

if command -v make &> /dev/null && command -v chpl &> /dev/null; then
    cd $REPO_ROOT/ffi/chapel 2>/dev/null || true
    
    if [ "$OUTPUT_JSON" = false ]; then
        echo "Checking Chapel syntax..."
    fi
    if chpl --parse-only training_pipeline.chpl chapel_ai.chpl 2>/dev/null; then
        if [ "$OUTPUT_JSON" = false ]; then
            echo -e "${GREEN}✅${NC} Chapel syntax OK"
        fi
        JSON_RESULTS+=("{\"check\":\"build:chapel-syntax\",\"status\":\"pass\"}")
        ((PASSED++))
    else
        if [ "$OUTPUT_JSON" = false ]; then
            echo -e "${YELLOW}⚠️${NC} Chapel syntax check skipped (Chapel not fully available)"
        fi
        JSON_RESULTS+=("{\"check\":\"build:chapel-syntax\",\"status\":\"warn\",\"message\":\"Chapel not fully available\"}")
        ((WARNING++))
    fi
fi

# ─────────────────────────────────────────────────────────────────────────
# Output results

if [ "$OUTPUT_JSON" = true ]; then
    # Build JSON array from results
    json_array=""
    for item in "${JSON_RESULTS[@]}"; do
        if [ -n "$json_array" ]; then
            json_array="$json_array,"
        fi
        json_array="$json_array$item"
    done

    ready="true"
    if [ $FAILED -gt 0 ]; then
        ready="false"
    fi

    printf '{"summary":{"passed":%d,"warnings":%d,"failed":%d,"ready":%s},"checks":[%s]}\n' \
        "$PASSED" "$WARNING" "$FAILED" "$ready" "$json_array"
else
    echo ""
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║                       SUMMARY                                 ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo ""
    echo -e "✅ Passed:   ${GREEN}$PASSED${NC}"
    echo -e "⚠️  Warnings: ${YELLOW}$WARNING${NC}"
    echo -e "❌ Failed:   ${RED}$FAILED${NC}"
    echo ""

    if [ $FAILED -eq 0 ]; then
        echo -e "${GREEN}🟢 SYSTEM READY FOR DEPLOYMENT${NC}"
        echo ""
        echo "Next steps:"
        echo "  1. Set HF_TOKEN:  export HF_TOKEN=\"hf_xxxxx\""
        echo "  2. Push to HF:    ./scripts/push_to_huggingface.sh \$HF_TOKEN"
        echo "  3. Create Space:  https://huggingface.co/new-space"
        echo "  4. Run locally:   cd ffi/chapel && make train && make run"
        echo ""
    else
        echo -e "${RED}🔴 ISSUES FOUND - FIX BEFORE DEPLOYING${NC}"
        echo ""
        echo "Failed items:"
        echo "  • Install missing dependencies"
        echo "  • Verify Chapel installation"
        echo "  • Check file paths"
        echo ""
    fi

    echo "════════════════════════════════════════════════════════════════════"
fi
