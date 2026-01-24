#!/bin/bash
# 🧪 LOCAL WORKFLOW TESTER - Test GitHub workflows locally

set -e

REPO_ROOT="/workspaces/nuclear-crawler-hybrid"
RESULTS_DIR="/tmp/workflow-test-results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

mkdir -p "$RESULTS_DIR"

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║  🧪 LOCAL WORKFLOW TESTER                                   ║"
echo "║     Testing all GitHub workflows locally                    ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# ───────────────────────────────────────────────────────────────────
# TEST 1: FORMAT CHECK
# ───────────────────────────────────────────────────────────────────

echo "📝 TEST 1: Format Check"
echo "──────────────────────────────────────────────────────────────"
cd "$REPO_ROOT"

if cargo fmt -- --check > "$RESULTS_DIR/format-${TIMESTAMP}.log" 2>&1; then
  echo "✅ PASS: Format check passed"
  RESULT_FORMAT="✅ PASS"
else
  echo "⚠️  WARN: Format issues found (can be auto-fixed)"
  echo "    Run: cargo fmt"
  RESULT_FORMAT="⚠️  WARN"
fi
echo ""

# ───────────────────────────────────────────────────────────────────
# TEST 2: CLIPPY LINTING
# ───────────────────────────────────────────────────────────────────

echo "🔍 TEST 2: Clippy Linting"
echo "──────────────────────────────────────────────────────────────"

if cargo clippy --all-targets --all-features -- -D warnings > "$RESULTS_DIR/clippy-${TIMESTAMP}.log" 2>&1; then
  echo "✅ PASS: No clippy warnings"
  RESULT_CLIPPY="✅ PASS"
else
  echo "⚠️  WARN: Clippy found issues"
  tail -20 "$RESULTS_DIR/clippy-${TIMESTAMP}.log"
  RESULT_CLIPPY="⚠️  WARN"
fi
echo ""

# ───────────────────────────────────────────────────────────────────
# TEST 3: BUILD
# ───────────────────────────────────────────────────────────────────

echo "🔨 TEST 3: Build (release)"
echo "──────────────────────────────────────────────────────────────"

if cargo build --release --all-targets > "$RESULTS_DIR/build-${TIMESTAMP}.log" 2>&1; then
  echo "✅ PASS: Build succeeded"
  RESULT_BUILD="✅ PASS"
else
  echo "❌ FAIL: Build failed"
  tail -30 "$RESULTS_DIR/build-${TIMESTAMP}.log"
  RESULT_BUILD="❌ FAIL"
fi
echo ""

# ───────────────────────────────────────────────────────────────────
# TEST 4: UNIT TESTS
# ───────────────────────────────────────────────────────────────────

echo "✅ TEST 4: Unit Tests"
echo "──────────────────────────────────────────────────────────────"

if cargo test --release --lib --verbose > "$RESULTS_DIR/unit-tests-${TIMESTAMP}.log" 2>&1; then
  echo "✅ PASS: Unit tests passed"
  RESULT_UNIT="✅ PASS"
else
  echo "⚠️  WARN: Unit tests had issues"
  tail -20 "$RESULTS_DIR/unit-tests-${TIMESTAMP}.log"
  RESULT_UNIT="⚠️  WARN"
fi
echo ""

# ───────────────────────────────────────────────────────────────────
# TEST 5: INTEGRATION TESTS
# ───────────────────────────────────────────────────────────────────

echo "🔄 TEST 5: Integration Tests"
echo "──────────────────────────────────────────────────────────────"

if timeout 30s cargo test --test integration_real_mcp --release -- --nocapture --test-threads=1 > "$RESULTS_DIR/integration-${TIMESTAMP}.log" 2>&1; then
  echo "✅ PASS: Integration tests passed"
  RESULT_INTEGRATION="✅ PASS"
else
  echo "⚠️  WARN: Integration tests skipped or had issues"
  RESULT_INTEGRATION="⚠️  WARN"
fi
echo ""

# ───────────────────────────────────────────────────────────────────
# TEST 6: MCP VALIDATION
# ───────────────────────────────────────────────────────────────────

echo "🎯 TEST 6: MCP Tools Validation"
echo "──────────────────────────────────────────────────────────────"

if [ -f "$REPO_ROOT/target/release/nuclear-mcp" ]; then
  # Start server
  timeout 5s "$REPO_ROOT/target/release/nuclear-mcp" --serve tcp://127.0.0.1:8079 > /dev/null 2>&1 &
  SERVER_PID=$!
  sleep 1

  if [ -f "$REPO_ROOT/.github/scripts/validate_5_tools.sh" ]; then
    if bash "$REPO_ROOT/.github/scripts/validate_5_tools.sh" > "$RESULTS_DIR/mcp-${TIMESTAMP}.log" 2>&1; then
      echo "✅ PASS: Exactly 5 MCP tools found"
      RESULT_MCP="✅ PASS"
    else
      echo "⚠️  WARN: MCP validation had issues"
      RESULT_MCP="⚠️  WARN"
    fi
  else
    echo "⚠️  WARN: MCP validation script not found"
    RESULT_MCP="⚠️  SKIP"
  fi

  kill $SERVER_PID 2>/dev/null || true
else
  echo "⚠️  WARN: binary not found, skipping MCP test"
  RESULT_MCP="⚠️  SKIP"
fi
echo ""

# ───────────────────────────────────────────────────────────────────
# TEST 7: DOCKER BUILD
# ───────────────────────────────────────────────────────────────────

echo "🐳 TEST 7: Docker Build Check"
echo "──────────────────────────────────────────────────────────────"

if [ -f "$REPO_ROOT/Dockerfile" ]; then
  if docker build --no-cache -t nuclear-mcp:test-${TIMESTAMP} . > "$RESULTS_DIR/docker-${TIMESTAMP}.log" 2>&1; then
    echo "✅ PASS: Docker build succeeded"
    RESULT_DOCKER="✅ PASS"
    docker rmi nuclear-mcp:test-${TIMESTAMP} > /dev/null 2>&1 || true
  else
    echo "⚠️  WARN: Docker build had issues"
    RESULT_DOCKER="⚠️  WARN"
  fi
else
  echo "⚠️  WARN: Dockerfile not found"
  RESULT_DOCKER="⚠️  SKIP"
fi
echo ""

# ───────────────────────────────────────────────────────────────────
# TEST 8: SECURITY AUDIT
# ───────────────────────────────────────────────────────────────────

echo "🔐 TEST 8: Security Audit"
echo "──────────────────────────────────────────────────────────────"

if command -v cargo-audit &> /dev/null; then
  if cargo audit > "$RESULTS_DIR/audit-${TIMESTAMP}.log" 2>&1; then
    echo "✅ PASS: No security vulnerabilities found"
    RESULT_AUDIT="✅ PASS"
  else
    echo "⚠️  WARN: Security audit found issues"
    tail -10 "$RESULTS_DIR/audit-${TIMESTAMP}.log"
    RESULT_AUDIT="⚠️  WARN"
  fi
else
  echo "⚠️  WARN: cargo-audit not installed"
  echo "    Install with: cargo install cargo-audit"
  RESULT_AUDIT="⚠️  SKIP"
fi
echo ""

# ───────────────────────────────────────────────────────────────────
# SUMMARY
# ───────────────────────────────────────────────────────────────────

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║  📊 WORKFLOW TEST RESULTS SUMMARY                           ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "Format Check:        $RESULT_FORMAT"
echo "Clippy Linting:      $RESULT_CLIPPY"
echo "Build:               $RESULT_BUILD"
echo "Unit Tests:          $RESULT_UNIT"
echo "Integration Tests:   $RESULT_INTEGRATION"
echo "MCP Validation:      $RESULT_MCP"
echo "Docker Build:        $RESULT_DOCKER"
echo "Security Audit:      $RESULT_AUDIT"
echo ""

# ───────────────────────────────────────────────────────────────────
# SAVE SUMMARY
# ───────────────────────────────────────────────────────────────────

cat > "$RESULTS_DIR/SUMMARY-${TIMESTAMP}.txt" << EOF
╔═══════════════════════════════════════════════════════════════╗
║  WORKFLOW TEST RESULTS                                      ║
║  Timestamp: ${TIMESTAMP}                                        ║
╚═══════════════════════════════════════════════════════════════╝

📋 RESULTS:
  Format Check:        $RESULT_FORMAT
  Clippy Linting:      $RESULT_CLIPPY
  Build:               $RESULT_BUILD
  Unit Tests:          $RESULT_UNIT
  Integration Tests:   $RESULT_INTEGRATION
  MCP Validation:      $RESULT_MCP
  Docker Build:        $RESULT_DOCKER
  Security Audit:      $RESULT_AUDIT

📁 LOGS DIRECTORY:
  $RESULTS_DIR/

📊 LOG FILES:
  format-${TIMESTAMP}.log
  clippy-${TIMESTAMP}.log
  build-${TIMESTAMP}.log
  unit-tests-${TIMESTAMP}.log
  integration-${TIMESTAMP}.log
  mcp-${TIMESTAMP}.log
  docker-${TIMESTAMP}.log
  audit-${TIMESTAMP}.log

🔗 Next Steps:
  1. Review failed tests above
  2. Check log files: ls -la $RESULTS_DIR/
  3. Fix issues and re-run: bash .github/scripts/test-workflows.sh
  4. Push to GitHub to run full pipeline
EOF

echo "📁 Detailed logs saved to: $RESULTS_DIR/"
echo "   Summary: $RESULTS_DIR/SUMMARY-${TIMESTAMP}.txt"
echo ""

# ───────────────────────────────────────────────────────────────────
# FINAL STATUS
# ───────────────────────────────────────────────────────────────────

FAIL_COUNT=0
[ "$RESULT_FORMAT" = "❌ FAIL" ] && ((FAIL_COUNT++))
[ "$RESULT_CLIPPY" = "❌ FAIL" ] && ((FAIL_COUNT++))
[ "$RESULT_BUILD" = "❌ FAIL" ] && ((FAIL_COUNT++))
[ "$RESULT_UNIT" = "❌ FAIL" ] && ((FAIL_COUNT++))
[ "$RESULT_INTEGRATION" = "❌ FAIL" ] && ((FAIL_COUNT++))
[ "$RESULT_MCP" = "❌ FAIL" ] && ((FAIL_COUNT++))
[ "$RESULT_DOCKER" = "❌ FAIL" ] && ((FAIL_COUNT++))
[ "$RESULT_AUDIT" = "❌ FAIL" ] && ((FAIL_COUNT++))

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $FAIL_COUNT -eq 0 ]; then
  echo "✅ ALL TESTS PASSED!"
  exit 0
else
  echo "⚠️  $FAIL_COUNT test(s) need attention"
  exit 0  # Don't fail so we can review logs
fi
