#!/usr/bin/env bash
# Script para validar 5 tools exactos en MCP protocol
set -e

echo "=================================="
echo "Validating MCP Tool Count..."
echo "=================================="

TOOL_COUNT=$(grep -c '"name":' src/mcp/protocol.rs || echo "0")

echo "Tools found in src/mcp/protocol.rs: $TOOL_COUNT"

if [ "$TOOL_COUNT" -eq 5 ]; then
    echo "✅ Exactly 5 tools verified"
    
    # Run cargo test
    echo ""
    echo "Running test_exactly_5_tools..."
    cargo test test_exactly_5_tools --lib --release -- --nocapture
    
else
    echo "❌ ERROR: Expected 5 tools, found $TOOL_COUNT"
    echo ""
    echo "Tools found:"
    grep '"name":' src/mcp/protocol.rs || true
    exit 1
fi

echo ""
echo "=================================="
echo "Tool validation passed!"
echo "=================================="
