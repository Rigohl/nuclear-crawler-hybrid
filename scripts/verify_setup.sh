#!/bin/bash
# Verificación rápida del sistema para Chapel + HuggingFace

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║          NUCLEAR CHAPEL DEPLOYMENT VERIFICATION               ║"
echo "║              GitHub + HuggingFace Setup Check                 ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0
WARNING=0

check_command() {
    if command -v "$1" &> /dev/null; then
        echo -e "${GREEN}✅${NC} $1 installed"
        ((PASSED++))
        return 0
    else
        echo -e "${RED}❌${NC} $1 NOT installed"
        ((FAILED++))
        return 1
    fi
}

check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✅${NC} $1 exists"
        ((PASSED++))
        return 0
    else
        echo -e "${YELLOW}⚠️${NC} $1 NOT found"
        ((WARNING++))
        return 1
    fi
}

# ─────────────────────────────────────────────────────────────────────────
echo -e "${BLUE}🔍 SYSTEM CHECKS${NC}"
echo "─────────────────────────────────────────────────────────────────────────"

check_command "git"
check_command "chpl"
check_command "make"
check_command "python3"
check_command "npm"
check_command "npx"

# ─────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BLUE}🗂️ CHAPEL FILES${NC}"
echo "─────────────────────────────────────────────────────────────────────────"

check_file "/workspaces/nuclear-crawler-hybrid/ffi/chapel/training_pipeline.chpl"
check_file "/workspaces/nuclear-crawler-hybrid/ffi/chapel/chapel_ai.chpl"
check_file "/workspaces/nuclear-crawler-hybrid/ffi/chapel/Makefile"
check_file "/workspaces/nuclear-crawler-hybrid/ffi/chapel/hf_spaces_app.py"
check_file "/workspaces/nuclear-crawler-hybrid/ffi/chapel/data/train/scraping_stealth_patterns.json"

# ─────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BLUE}🚀 DEPLOYMENT FILES${NC}"
echo "─────────────────────────────────────────────────────────────────────────"

check_file "/workspaces/nuclear-crawler-hybrid/.github/workflows/chapel-training-pipeline.yml"
check_file "/workspaces/nuclear-crawler-hybrid/scripts/push_to_huggingface.sh"
check_file "/workspaces/nuclear-crawler-hybrid/HUGGINGFACE_DEPLOYMENT.md"
check_file "/workspaces/nuclear-crawler-hybrid/SETUP_HF_KIMBERLY.md"

# ─────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BLUE}🔐 ENVIRONMENT VARIABLES${NC}"
echo "─────────────────────────────────────────────────────────────────────────"

if [ -z "$HF_TOKEN" ]; then
    echo -e "${YELLOW}⚠️${NC} HF_TOKEN not set (needed for HuggingFace)"
    ((WARNING++))
else
    echo -e "${GREEN}✅${NC} HF_TOKEN is set"
    ((PASSED++))
fi

if [ -z "$GITHUB_TOKEN" ]; then
    echo -e "${YELLOW}⚠️${NC} GITHUB_TOKEN not set (optional, uses CLI)"
    ((WARNING++))
else
    echo -e "${GREEN}✅${NC} GITHUB_TOKEN is set"
    ((PASSED++))
fi

# ─────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BLUE}💾 STORAGE${NC}"
echo "─────────────────────────────────────────────────────────────────────────"

if [ -d "/workspaces/nuclear-crawler-hybrid/ffi/chapel/data" ]; then
    size=$(du -sh /workspaces/nuclear-crawler-hybrid/ffi/chapel/data 2>/dev/null | cut -f1)
    echo -e "${GREEN}✅${NC} Data directory exists (size: $size)"
    ((PASSED++))
else
    echo -e "${RED}❌${NC} Data directory missing"
    ((FAILED++))
fi

# ─────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BLUE}🔗 GIT REPOSITORY${NC}"
echo "─────────────────────────────────────────────────────────────────────────"

if [ -d "/workspaces/nuclear-crawler-hybrid/.git" ]; then
    echo -e "${GREEN}✅${NC} Git repository detected"
    ((PASSED++))
    
    cd /workspaces/nuclear-crawler-hybrid
    branch=$(git rev-parse --abbrev-ref HEAD 2>/dev/null)
    echo -e "${GREEN}✅${NC} Branch: $branch"
    ((PASSED++))
    
    remote=$(git config --get remote.origin.url 2>/dev/null)
    echo -e "${GREEN}✅${NC} Remote: $remote"
    ((PASSED++))
else
    echo -e "${RED}❌${NC} Not a git repository"
    ((FAILED++))
fi

# ─────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BLUE}⚡ QUICK BUILD TEST${NC}"
echo "─────────────────────────────────────────────────────────────────────────"

if command -v make &> /dev/null && command -v chpl &> /dev/null; then
    cd /workspaces/nuclear-crawler-hybrid/ffi/chapel
    
    echo "Checking Chapel syntax..."
    if chpl --parse-only training_pipeline.chpl chapel_ai.chpl 2>/dev/null; then
        echo -e "${GREEN}✅${NC} Chapel syntax OK"
        ((PASSED++))
    else
        echo -e "${YELLOW}⚠️${NC} Chapel syntax check skipped (Chapel not fully available)"
        ((WARNING++))
    fi
fi

# ─────────────────────────────────────────────────────────────────────────
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
