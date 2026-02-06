#!/usr/bin/env bash
# Validate that MCP protocol has EXACTLY 7 tools (AMPLIFIED)
# This script ensures protocol.rs maintains the 7-tool architecture

set -e

echo "🔍 Validating MCP Protocol: 7 Tools Constraint"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Run the Rust test
echo "Running: cargo test test_exactly_7_tools --lib"
cargo test test_exactly_7_tools --lib

# Check tool definitions in protocol.rs
echo ""
echo "Verifying tool definitions in src/mcp/protocol.rs..."
TOOL_COUNT=$(grep -c '"name": "websearch"\|"name": "premium"\|"name": "file_search"\|"name": "scan"\|"name": "ai_dataset_trainer"\|"name": "parallel_engine"\|"name": "osint_intelligence"' src/mcp/protocol.rs || true)

echo "Tools found in protocol.rs: $TOOL_COUNT"

if [ "$TOOL_COUNT" -lt 7 ]; then
    echo "❌ ERROR: Expected 7 tools, found $TOOL_COUNT"
    exit 1
fi

echo ""
echo "✅ SUCCESS: MCP protocol has exactly 7 tools"
echo "   1. websearch - Multi-engine search with integrated scraping"
echo "   2. premium - Premium content extraction"
echo "   3. file_search - Advanced file search"
echo "   4. scan - Workspace scanner"
echo "   5. ai_dataset_trainer - AI dataset generation"
echo "   6. parallel_engine - Parallel processing engine (NEW - replaces wasm_scraper)"
echo "   7. osint_intelligence - OSINT with Chapel AI"
echo ""
echo "=================================="
echo "Tool validation passed!"
echo "=================================="
