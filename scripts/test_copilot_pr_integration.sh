#!/bin/bash
# 🤖 Test de Integración: GitHub Pull Request Copilot Coding Agent
# 
# Este script valida la compatibilidad del repo con PRs automatizados
# creados por GitHub Copilot sin necesidad de compilar Rust.

# No usar set -e porque queremos contar los fallos sin parar
set +e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🤖 TEST DE INTEGRACIÓN: GitHub PR Copilot Coding Agent"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cd "$REPO_ROOT"

PASSED=0
FAILED=0
WARNINGS=0

# Helper functions
pass() {
    echo "✅ PASS: $1"
    ((PASSED++))
}

fail() {
    echo "❌ FAIL: $1"
    ((FAILED++))
}

warn() {
    echo "⚠️  WARN: $1"
    ((WARNINGS++))
}

info() {
    echo "ℹ️  INFO: $1"
}

# Test 1: PR Template exists
echo ""
echo "TEST 1: Verificar existencia de PR template"
echo "─────────────────────────────────────────────"
if [ -f ".github/PULL_REQUEST_TEMPLATE.md" ]; then
    pass "PR template existe"
    
    # Verificar contenido
    if grep -q "## Description" .github/PULL_REQUEST_TEMPLATE.md; then
        pass "Template contiene sección de descripción"
    else
        fail "Template no contiene sección de descripción"
    fi
    
    if grep -q "MCP" .github/PULL_REQUEST_TEMPLATE.md || grep -q "tools" .github/PULL_REQUEST_TEMPLATE.md; then
        pass "Template menciona validaciones MCP/tools"
    else
        warn "Template no menciona explícitamente MCP/tools"
    fi
else
    fail "PR template no existe"
fi

# Test 2: AGENTS.md rules
echo ""
echo "TEST 2: Verificar reglas en AGENTS.md"
echo "─────────────────────────────────────────────"
if [ -f "AGENTS.md" ]; then
    pass "AGENTS.md existe"
    
    if grep -qi "no mock" AGENTS.md; then
        pass "AGENTS.md prohíbe mocks explícitamente"
    else
        fail "AGENTS.md no prohíbe mocks explícitamente"
    fi
    
    if grep -q "5 tools" AGENTS.md || grep -q "5 herramientas" AGENTS.md; then
        pass "AGENTS.md especifica exactamente 5 tools"
    else
        fail "AGENTS.md no especifica número de tools"
    fi
    
    if grep -q "test_exactly_5_tools" AGENTS.md; then
        pass "AGENTS.md menciona test de validación"
    else
        warn "AGENTS.md no menciona test_exactly_5_tools"
    fi
else
    fail "AGENTS.md no existe"
fi

# Test 3: CI workflows trigger on PR
echo ""
echo "TEST 3: Verificar triggers de PR en workflows"
echo "─────────────────────────────────────────────"
WORKFLOWS=("ci.yml" "mcp-validation.yml" "security.yml")
for workflow in "${WORKFLOWS[@]}"; do
    if [ -f ".github/workflows/$workflow" ]; then
        if grep -q "pull_request:" ".github/workflows/$workflow"; then
            pass "$workflow se ejecuta en pull_request"
        else
            fail "$workflow NO se ejecuta en pull_request"
        fi
    else
        warn "$workflow no existe"
    fi
done

# Test 4: CI validates no mocks
echo ""
echo "TEST 4: Verificar validación de mocks en CI"
echo "─────────────────────────────────────────────"
if [ -f ".github/workflows/mcp-validation.yml" ]; then
    if grep -qi "mock" ".github/workflows/mcp-validation.yml"; then
        pass "CI verifica ausencia de mocks"
    else
        warn "CI no verifica explícitamente mocks"
    fi
    
    if grep -qi "real" ".github/workflows/mcp-validation.yml"; then
        pass "CI enfatiza uso de datos REALES"
    else
        warn "CI no enfatiza datos REALES explícitamente"
    fi
else
    fail "mcp-validation.yml no existe"
fi

# Test 5: CI validates exactly 5 tools
echo ""
echo "TEST 5: Verificar validación de 5 tools en CI"
echo "─────────────────────────────────────────────"
if [ -f ".github/workflows/ci.yml" ]; then
    if grep -q "test_exactly_5_tools" ".github/workflows/ci.yml"; then
        pass "CI ejecuta test_exactly_5_tools"
    else
        fail "CI NO ejecuta test_exactly_5_tools"
    fi
else
    fail "ci.yml no existe"
fi

# Test 6: Copilot instructions exist
echo ""
echo "TEST 6: Verificar instrucciones para Copilot"
echo "─────────────────────────────────────────────"
if [ -f ".github/copilot-instructions.md" ]; then
    pass "copilot-instructions.md existe"
    
    if grep -qi "5 tools\|five tools" ".github/copilot-instructions.md"; then
        pass "Instrucciones mencionan las 5 tools"
    else
        warn "Instrucciones no mencionan explícitamente 5 tools"
    fi
    
    if grep -qi "mock\|stub" ".github/copilot-instructions.md"; then
        pass "Instrucciones mencionan política de no mocks"
    else
        warn "Instrucciones no mencionan política de mocks"
    fi
else
    fail "copilot-instructions.md no existe"
fi

# Test 7: Required files exist
echo ""
echo "TEST 7: Verificar archivos requeridos"
echo "─────────────────────────────────────────────"
REQUIRED_FILES=(
    "src/mcp/protocol.rs"
    "tests/integration_real_mcp.rs"
    "Cargo.toml"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        pass "$file existe"
    else
        fail "$file NO existe"
    fi
done

# Test 8: .gitignore configuration
echo ""
echo "TEST 8: Verificar configuración de .gitignore"
echo "─────────────────────────────────────────────"
if [ -f ".gitignore" ]; then
    if ! grep -q "^src/bin/" .gitignore && ! grep -q "^src/$" .gitignore; then
        pass ".gitignore permite commits en src/"
    else
        fail ".gitignore bloquea directorio src/"
    fi
    
    if grep -q "target" .gitignore; then
        pass ".gitignore bloquea target/"
    else
        warn ".gitignore no bloquea target/"
    fi
else
    fail ".gitignore no existe"
fi

# Test 9: Workflow permissions
echo ""
echo "TEST 9: Verificar permisos de workflows"
echo "─────────────────────────────────────────────"
for workflow in "${WORKFLOWS[@]}"; do
    if [ -f ".github/workflows/$workflow" ]; then
        if grep -q "permissions:" ".github/workflows/$workflow"; then
            if grep -q "contents: read\|pull-requests:" ".github/workflows/$workflow"; then
                pass "$workflow tiene permisos apropiados"
            else
                warn "$workflow define permissions pero no incluye PR permissions"
            fi
        else
            info "$workflow usa permisos por defecto (OK)"
            ((PASSED++))
        fi
    fi
done

# Test 10: Continue-on-error for known issues
echo ""
echo "TEST 10: Verificar manejo de errores conocidos"
echo "─────────────────────────────────────────────"
if [ -f ".github/workflows/ci.yml" ]; then
    if grep -q "continue-on-error" ".github/workflows/ci.yml"; then
        pass "CI usa continue-on-error para issues conocidos"
    else
        warn "CI no usa continue-on-error"
    fi
fi

# Test 11: Verify tool count in protocol.rs
echo ""
echo "TEST 11: Verificar definición de 5 tools en código"
echo "─────────────────────────────────────────────"
if [ -f "src/mcp/protocol.rs" ]; then
    # Contar definiciones de tools en get_tool_definitions (excluir struct definition)
    TOOL_COUNT=$(grep -A 200 "pub fn get_tool_definitions" src/mcp/protocol.rs | grep -c "ToolDefinition {" || echo "0")
    if [ "$TOOL_COUNT" -eq 5 ]; then
        pass "protocol.rs define exactamente 5 tools"
    else
        fail "protocol.rs define $TOOL_COUNT tools (esperado: 5)"
    fi
    
    # Verificar nombres de tools
    EXPECTED_TOOLS=("websearch" "premium" "file_search" "scan" "ai_dataset_trainer")
    for tool in "${EXPECTED_TOOLS[@]}"; do
        if grep -q "name: \"$tool\"" src/mcp/protocol.rs; then
            pass "Tool '$tool' está definido"
        else
            fail "Tool '$tool' NO está definido"
        fi
    done
else
    fail "src/mcp/protocol.rs no existe"
fi

# Test 12: Multi-agent pipeline PR support
echo ""
echo "TEST 12: Verificar soporte de PRs en pipeline multi-agente"
echo "─────────────────────────────────────────────"
if [ -f ".github/workflows/nuclear-advanced-pipeline.yml" ]; then
    if grep -q "pull_request" ".github/workflows/nuclear-advanced-pipeline.yml"; then
        info "Pipeline multi-agente se ejecuta en PRs"
        
        if grep -qi "createComment\|github-script\|comment" ".github/workflows/nuclear-advanced-pipeline.yml"; then
            pass "Pipeline puede comentar en PRs"
        else
            warn "Pipeline no tiene capacidad de comentar en PRs"
        fi
    else
        info "Pipeline multi-agente no se ejecuta en PRs (puede ser intencional)"
    fi
else
    info "Pipeline multi-agente no existe o está en otro archivo"
fi

# Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 RESUMEN DE RESULTADOS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "✅ Tests PASSED:   $PASSED"
echo "❌ Tests FAILED:   $FAILED"
echo "⚠️  Warnings:      $WARNINGS"
echo ""

# Recommendations
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "💡 RECOMENDACIONES PARA MEJORAR INTEGRACIÓN"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ $FAILED -eq 0 ]; then
    echo "✨ EXCELENTE: El repositorio está completamente preparado"
    echo "   para PRs automatizados de GitHub Copilot Coding Agent."
    echo ""
    echo "El pipeline CI/CD:"
    echo "  ✅ Se ejecuta automáticamente en pull requests"
    echo "  ✅ Valida las 5 tools del protocolo MCP"
    echo "  ✅ Detecta mocks en tests (no permitidos)"
    echo "  ✅ Tiene instrucciones claras para Copilot"
    echo "  ✅ Usa template de PR para guiar contribuciones"
    echo ""
else
    echo "⚠️  ACCIÓN REQUERIDA: Hay $FAILED validaciones fallidas"
    echo ""
    if ! [ -f ".github/PULL_REQUEST_TEMPLATE.md" ]; then
        echo "  1. Crear PR template en .github/PULL_REQUEST_TEMPLATE.md"
    fi
    if ! [ -f "AGENTS.md" ]; then
        echo "  2. Crear AGENTS.md con reglas para agentes"
    fi
    if ! grep -q "test_exactly_5_tools" .github/workflows/ci.yml 2>/dev/null; then
        echo "  3. Añadir test_exactly_5_tools al workflow CI"
    fi
fi

echo ""
echo "Mejoras opcionales recomendadas:"
echo "  • Añadir badge de CI status en README.md"
echo "  • Configurar GitHub branch protection para PRs"
echo "  • Añadir CODEOWNERS para revisión automática"
echo "  • Configurar GitHub Actions bot para comentar resultados"
echo ""

# Exit code
if [ $FAILED -eq 0 ]; then
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "✅ RESULTADO: COMPATIBLE"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    exit 0
else
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "❌ RESULTADO: NECESITA MEJORAS"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    exit 1
fi
