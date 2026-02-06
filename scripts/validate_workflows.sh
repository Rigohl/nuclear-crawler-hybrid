#!/bin/bash
# CI/CD Workflow Pre-Push Validation
# Run this before pushing changes to workflow files

set -e

echo "🔍 CI/CD Workflow Validation"
echo "=============================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

ERRORS=0
WARNINGS=0

# Check if workflow files were modified
MODIFIED_WORKFLOWS=$(git diff --name-only --cached | grep "^.github/workflows/.*\.yml$" || true)

if [ -z "$MODIFIED_WORKFLOWS" ]; then
    echo "✅ No workflow files modified, skipping validation"
    exit 0
fi

echo "📝 Modified workflows:"
echo "$MODIFIED_WORKFLOWS"
echo ""

# Run Python validator if available
if command -v python3 &> /dev/null; then
    echo "🐍 Running Python validator..."
    if python3 scripts/validate_workflows.py; then
        echo -e "${GREEN}✅ Python validation passed${NC}"
    else
        echo -e "${YELLOW}⚠️  Python validation found issues${NC}"
        WARNINGS=$((WARNINGS + 1))
    fi
    echo ""
fi

# Check for common issues
echo "🔍 Checking for common issues..."

# Check for trailing spaces
echo "  • Checking trailing spaces..."
if echo "$MODIFIED_WORKFLOWS" | xargs grep -n " $" 2>/dev/null; then
    echo -e "${RED}    ❌ Found trailing spaces${NC}"
    ERRORS=$((ERRORS + 1))
else
    echo -e "${GREEN}    ✅ No trailing spaces${NC}"
fi

# Check for tabs (should use spaces)
echo "  • Checking for tabs..."
if echo "$MODIFIED_WORKFLOWS" | xargs grep -P '\t' 2>/dev/null; then
    echo -e "${RED}    ❌ Found tabs (use spaces)${NC}"
    ERRORS=$((ERRORS + 1))
else
    echo -e "${GREEN}    ✅ No tabs found${NC}"
fi

# Check for deprecated actions
echo "  • Checking for deprecated actions..."
if echo "$MODIFIED_WORKFLOWS" | xargs grep -n "actions-rs/toolchain@v1" 2>/dev/null; then
    echo -e "${YELLOW}    ⚠️  Found deprecated actions-rs/toolchain@v1${NC}"
    echo "       Consider: dtolnay/rust-toolchain@stable"
    WARNINGS=$((WARNINGS + 1))
else
    echo -e "${GREEN}    ✅ No deprecated actions${NC}"
fi

# Check for hardcoded script paths
echo "  • Checking script paths..."
if echo "$MODIFIED_WORKFLOWS" | xargs grep -n "python3 scripts/" 2>/dev/null | grep -v "github/scripts" | grep -v "fallback" 2>/dev/null; then
    echo -e "${YELLOW}    ⚠️  Found hardcoded scripts/ path${NC}"
    echo "       Consider: Adding fallback to .github/scripts/"
    WARNINGS=$((WARNINGS + 1))
else
    echo -e "${GREEN}    ✅ Script paths look good${NC}"
fi

# Check for cargo build without error handling
echo "  • Checking cargo commands..."
if echo "$MODIFIED_WORKFLOWS" | xargs grep -n "cargo build\|cargo test\|cargo clippy" 2>/dev/null | grep -v "continue-on-error" | grep -v "#" | head -5; then
    echo -e "${YELLOW}    ⚠️  Found cargo commands without error handling${NC}"
    echo "       Consider: Adding continue-on-error or error filters"
    WARNINGS=$((WARNINGS + 1))
else
    echo -e "${GREEN}    ✅ Cargo commands have error handling${NC}"
fi

echo ""
echo "=============================="
echo "📊 Summary"
echo "=============================="
echo -e "Errors:   ${RED}${ERRORS}${NC}"
echo -e "Warnings: ${YELLOW}${WARNINGS}${NC}"
echo ""

if [ $ERRORS -gt 0 ]; then
    echo -e "${RED}❌ Validation FAILED - Please fix errors before pushing${NC}"
    exit 1
elif [ $WARNINGS -gt 0 ]; then
    echo -e "${YELLOW}⚠️  Validation PASSED with warnings${NC}"
    echo "Continue? (y/N)"
    read -r response
    if [[ ! "$response" =~ ^[Yy]$ ]]; then
        echo "Push cancelled"
        exit 1
    fi
    exit 0
else
    echo -e "${GREEN}✅ Validation PASSED - Ready to push${NC}"
    exit 0
fi
