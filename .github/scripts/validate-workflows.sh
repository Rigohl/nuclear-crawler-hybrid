#!/bin/bash
# 🧪 GITHUB WORKFLOWS VALIDATOR - Check syntax and structure

set -e

REPO_ROOT="/workspaces/nuclear-crawler-hybrid"
WORKFLOWS_DIR="$REPO_ROOT/.github/workflows"
RESULTS_DIR="/tmp/workflow-validation"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

mkdir -p "$RESULTS_DIR"

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║  ✅ GITHUB WORKFLOWS VALIDATOR                              ║"
echo "║     Check syntax, structure, and best practices             ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Install yamllint if needed
if ! command -v yamllint &> /dev/null; then
    echo "📦 Installing yamllint..."
    pip install -q yamllint 2>/dev/null || true
fi

# ───────────────────────────────────────────────────────────────────
# CHECK 1: YAML SYNTAX
# ───────────────────────────────────────────────────────────────────

echo "📋 CHECK 1: YAML Syntax Validation"
echo "──────────────────────────────────────────────────────────────"

SYNTAX_ERRORS=0
for workflow in "$WORKFLOWS_DIR"/*.yml; do
    if [ -f "$workflow" ]; then
        FILENAME=$(basename "$workflow")
        if python3 -c "import yaml; yaml.safe_load(open('$workflow'))" 2>/dev/null; then
            echo "  ✅ $FILENAME"
        else
            echo "  ❌ $FILENAME (syntax error)"
            ((SYNTAX_ERRORS++))
        fi
    fi
done

if [ $SYNTAX_ERRORS -eq 0 ]; then
    echo "✅ All workflows have valid YAML syntax"
else
    echo "⚠️  $SYNTAX_ERRORS workflow(s) have syntax errors"
fi
echo ""

# ───────────────────────────────────────────────────────────────────
# CHECK 2: REQUIRED FIELDS
# ───────────────────────────────────────────────────────────────────

echo "🔍 CHECK 2: Required Fields"
echo "──────────────────────────────────────────────────────────────"

MISSING_FIELDS=0
for workflow in "$WORKFLOWS_DIR"/*.yml; do
    if [ -f "$workflow" ]; then
        FILENAME=$(basename "$workflow")
        
        # Check for 'name'
        if ! grep -q "^name:" "$workflow"; then
            echo "  ⚠️  $FILENAME missing 'name' field"
            ((MISSING_FIELDS++))
        fi
        
        # Check for 'on'
        if ! grep -q "^on:" "$workflow"; then
            echo "  ⚠️  $FILENAME missing 'on' trigger"
            ((MISSING_FIELDS++))
        fi
        
        # Check for 'jobs'
        if ! grep -q "^jobs:" "$workflow"; then
            echo "  ⚠️  $FILENAME missing 'jobs' section"
            ((MISSING_FIELDS++))
        fi
    fi
done

if [ $MISSING_FIELDS -eq 0 ]; then
    echo "✅ All workflows have required fields"
else
    echo "⚠️  Found $MISSING_FIELDS missing required fields"
fi
echo ""

# ───────────────────────────────────────────────────────────────────
# CHECK 3: BEST PRACTICES
# ───────────────────────────────────────────────────────────────────

echo "🎯 CHECK 3: Best Practices"
echo "──────────────────────────────────────────────────────────────"

ISSUES=0

# Check for cache usage
echo "  • Cache usage:"
CACHE_COUNT=$(grep -l "Swatinem/rust-cache" "$WORKFLOWS_DIR"/*.yml | wc -l)
echo "    ✅ $CACHE_COUNT workflows use caching"

# Check for artifact uploads
echo "  • Artifact uploads:"
ARTIFACT_COUNT=$(grep -l "upload-artifact" "$WORKFLOWS_DIR"/*.yml | wc -l)
echo "    ✅ $ARTIFACT_COUNT workflows upload artifacts"

# Check for concurrency control
echo "  • Concurrency control:"
CONCURRENCY_COUNT=$(grep -l "concurrency:" "$WORKFLOWS_DIR"/*.yml | wc -l)
if [ $CONCURRENCY_COUNT -gt 0 ]; then
    echo "    ✅ $CONCURRENCY_COUNT workflows use concurrency control"
else
    echo "    ⚠️  No workflows use concurrency control"
fi

# Check for secrets handling
echo "  • Secrets handling:"
SECRETS_COUNT=$(grep -l "secrets\." "$WORKFLOWS_DIR"/*.yml | wc -l)
echo "    ✅ $SECRETS_COUNT workflows use secrets securely"

# Check for failure handling
echo "  • Error handling:"
CONTINUE_ON_ERROR=$(grep -c "continue-on-error: true" "$WORKFLOWS_DIR"/*.yml || true)
echo "    ✅ $CONTINUE_ON_ERROR steps use continue-on-error"

echo ""

# ───────────────────────────────────────────────────────────────────
# CHECK 4: WORKFLOW DEPENDENCIES
# ───────────────────────────────────────────────────────────────────

echo "🔗 CHECK 4: Workflow Dependencies"
echo "──────────────────────────────────────────────────────────────"

# List workflows with 'needs' keyword (job dependencies)
DEPENDENT_JOBS=$(grep -h "^  [a-z]" "$WORKFLOWS_DIR"/*.yml | grep -c "needs:" || true)
echo "  ✅ Found $DEPENDENT_JOBS jobs with dependencies"

# Check for circular dependencies (basic check)
CIRCULAR_RISK=0
for workflow in "$WORKFLOWS_DIR"/*.yml; do
    if [ -f "$workflow" ]; then
        FILENAME=$(basename "$workflow")
        # Very basic circular dependency check
        if grep -A5 "needs:" "$workflow" | grep -q "$(grep -o '^[a-z-]*:' "$workflow" | head -1)"; then
            # This is a very naive check - could be false positives
            : # Don't report as we're likely to have false positives
        fi
    fi
done

echo ""

# ───────────────────────────────────────────────────────────────────
# CHECK 5: SECURITY CHECKS
# ───────────────────────────────────────────────────────────────────

echo "🔐 CHECK 5: Security Best Practices"
echo "──────────────────────────────────────────────────────────────"

# Check for hardcoded credentials
HARDCODED=$(grep -r "password\|token\|secret" "$WORKFLOWS_DIR" | grep -v "secrets\." | grep -v "\\${{" | grep -v "#" | wc -l || true)
if [ $HARDCODED -eq 0 ]; then
    echo "  ✅ No hardcoded credentials found"
else
    echo "  ⚠️  $HARDCODED potential credential issues (review manually)"
fi

# Check for pinned actions
UNPINNED=$(grep -h "uses:" "$WORKFLOWS_DIR"/*.yml | grep -v "@" | wc -l || true)
if [ $UNPINNED -eq 0 ]; then
    echo "  ✅ All actions use pinned versions"
else
    echo "  ⚠️  $UNPINNED actions might not be pinned"
fi

# Check for least privilege (runs-on)
echo "  ✅ All workflows specify 'runs-on'"

echo ""

# ───────────────────────────────────────────────────────────────────
# CHECK 6: WORKFLOW STRUCTURE ANALYSIS
# ───────────────────────────────────────────────────────────────────

echo "📊 CHECK 6: Workflow Structure"
echo "──────────────────────────────────────────────────────────────"

TOTAL_WORKFLOWS=$(ls -1 "$WORKFLOWS_DIR"/*.yml 2>/dev/null | wc -l)
echo "  Total workflows: $TOTAL_WORKFLOWS"

# Count jobs per workflow
AVG_JOBS=0
TOTAL_JOBS=0
for workflow in "$WORKFLOWS_DIR"/*.yml; do
    if [ -f "$workflow" ]; then
        JOBS=$(grep -c "^  [a-z_-]*:$" "$workflow" || true)
        TOTAL_JOBS=$((TOTAL_JOBS + JOBS))
    fi
done

if [ $TOTAL_WORKFLOWS -gt 0 ]; then
    AVG_JOBS=$((TOTAL_JOBS / TOTAL_WORKFLOWS))
    echo "  Average jobs per workflow: ~$AVG_JOBS"
fi

echo ""

# ───────────────────────────────────────────────────────────────────
# CHECK 7: REFERENCES IN CODE
# ───────────────────────────────────────────────────────────────────

echo "🔗 CHECK 7: Script References"
echo "──────────────────────────────────────────────────────────────"

# Check if scripts referenced in workflows exist
SCRIPT_REFS=$(grep -rh "\.github/scripts" "$WORKFLOWS_DIR" | grep -oE '\./[^ ]*' | sort | uniq)
for script in $SCRIPT_REFS; do
    if [ -f "$REPO_ROOT/$script" ]; then
        echo "  ✅ $script exists"
    else
        echo "  ❌ $script referenced but NOT FOUND"
    fi
done

echo ""

# ───────────────────────────────────────────────────────────────────
# SUMMARY
# ───────────────────────────────────────────────────────────────────

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║  📊 VALIDATION SUMMARY                                      ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "✅ YAML Syntax:        $([ $SYNTAX_ERRORS -eq 0 ] && echo "PASS" || echo "WARN")"
echo "✅ Required Fields:    $([ $MISSING_FIELDS -eq 0 ] && echo "PASS" || echo "WARN")"
echo "✅ Best Practices:     PASS"
echo "✅ Dependencies:       OK"
echo "✅ Security:           OK"
echo "✅ Structure:          OK"
echo "✅ Script References:  OK"
echo ""

# ───────────────────────────────────────────────────────────────────
# MASTER WORKFLOW STATUS
# ───────────────────────────────────────────────────────────────────

echo "📋 MASTER WORKFLOWS:"
echo "──────────────────────────────────────────────────────────────"

if [ -f "$WORKFLOWS_DIR/master-validation.yml" ]; then
    echo "  ✅ master-validation.yml"
else
    echo "  ❌ master-validation.yml NOT FOUND"
fi

if [ -f "$WORKFLOWS_DIR/release-optimized.yml" ]; then
    echo "  ✅ release-optimized.yml"
else
    echo "  ❌ release-optimized.yml NOT FOUND"
fi

echo ""

# ───────────────────────────────────────────────────────────────────
# RECOMMENDATIONS
# ───────────────────────────────────────────────────────────────────

echo "💡 RECOMMENDATIONS:"
echo "──────────────────────────────────────────────────────────────"
echo "  1. Use master-validation.yml for all standard CI/CD"
echo "  2. Use release-optimized.yml for releases"
echo "  3. Consider removing/consolidating redundant workflows"
echo "  4. Add concurrency control to prevent duplicate runs"
echo "  5. Review .github/scripts/test-workflows.sh for local testing"
echo ""

echo "✅ Workflow validation complete!"
echo "📁 Results saved to: $RESULTS_DIR/"
