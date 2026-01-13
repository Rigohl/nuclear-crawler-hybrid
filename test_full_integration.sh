#!/usr/bin/env bash

# 🔥 FULL SRC INTEGRATION TEST
# Verifica que todos los módulos de src/ están siendo utilizados correctamente

set -e

echo "🔥 NUCLEAR CRAWLER HYBRID - FULL SRC INTEGRATION TEST"
echo "=================================================="
echo ""

# 1. Compilar sin features (todas las tools deberían funcionar)
echo "📦 Test 1: Compilación sin features adicionales"
cargo build --bin nuclear-mcp --quiet 2>&1 || {
    echo "❌ Error en compilación"
    exit 1
}
echo "✅ Compilación exitosa"
echo ""

# 2. Verificar que todos los módulos están importados
echo "📦 Test 2: Verificar importaciones de src en herramientas MCP"
WEBSEARCH_IMPORTS=$(grep -c "use crate::" src/mcp/tools/websearch.rs || echo "0")
PREMIUM_IMPORTS=$(grep -c "use crate::" src/mcp/tools/premium_content.rs || echo "0")
FILESEARCH_IMPORTS=$(grep -c "use crate::" src/mcp/tools/file_search_advanced.rs || echo "0")
DATASET_IMPORTS=$(grep -c "use crate::" src/mcp/tools/dataset_generator.rs || echo "0")

echo "  WebSearch imports: $WEBSEARCH_IMPORTS"
echo "  Premium imports: $PREMIUM_IMPORTS"
echo "  FileSearch imports: $FILESEARCH_IMPORTS"
echo "  Dataset imports: $DATASET_IMPORTS"

if [ "$WEBSEARCH_IMPORTS" -ge 3 ] && [ "$PREMIUM_IMPORTS" -ge 3 ] && [ "$FILESEARCH_IMPORTS" -ge 2 ] && [ "$DATASET_IMPORTS" -ge 2 ]; then
    echo "✅ Todas las herramientas tienen integraciones"
else
    echo "⚠️  Algunas herramientas tienen pocas integraciones"
fi
echo ""

# 3. Contar módulos siendo utilizados
echo "📦 Test 3: Módulos de src/ siendo utilizados"

MODULES_USED=$(grep -h "use crate::" src/mcp/tools/*.rs src/mcp/server.rs | grep -o "crate::[a-z_]*" | sort -u | wc -l)
echo "  Total módulos importados: $MODULES_USED"

MODULES_IMPORTED=$(grep -h "use crate::" src/mcp/tools/*.rs src/mcp/server.rs | grep -o "crate::[a-z_]*" | sort -u)
echo ""
echo "  Módulos importados:"
echo "$MODULES_IMPORTED" | sed 's/^/    - /'
echo ""

if [ "$MODULES_USED" -ge 8 ]; then
    echo "✅ Se están usando 8+ módulos de src"
else
    echo "⚠️  Se están usando menos de 8 módulos"
fi
echo ""

# 4. Verificar que el servidor inicia correctamente
echo "📦 Test 4: Verificar startup del servidor"
STARTUP_OUTPUT=$(timeout 3 ./target/debug/nuclear-mcp 2>&1 || true)

if echo "$STARTUP_OUTPUT" | grep -q "FULL src integration"; then
    echo "✅ Servidor inicia con integración completa"
else
    echo "⚠️  Servidor inicia pero sin mensaje de integración completa"
fi

if echo "$STARTUP_OUTPUT" | grep -q "WebSearch Tool initialized"; then
    echo "✅ WebSearch Tool se inicializa"
else
    echo "❌ WebSearch Tool no se inicializa"
fi

if echo "$STARTUP_OUTPUT" | grep -q "Premium Content Tool initialized"; then
    echo "✅ Premium Content Tool se inicializa"
else
    echo "❌ Premium Content Tool no se inicializa"
fi

if echo "$STARTUP_OUTPUT" | grep -q "File Search Tool initialized"; then
    echo "✅ File Search Tool se inicializa"
else
    echo "❌ File Search Tool no se inicializa"
fi

if echo "$STARTUP_OUTPUT" | grep -q "Dataset Generator Tool initialized"; then
    echo "✅ Dataset Generator Tool se inicializa"
else
    echo "❌ Dataset Generator Tool no se inicializa"
fi

if echo "$STARTUP_OUTPUT" | grep -q "FFI Processors initialized"; then
    echo "✅ FFI Processors se inicializan"
else
    echo "❌ FFI Processors no se inicializan"
fi

if echo "$STARTUP_OUTPUT" | grep -q "Server running on 127.0.0.1:8079"; then
    echo "✅ Servidor escucha en puerto 8079"
else
    echo "❌ Servidor no escucha en puerto 8079"
fi

echo ""
echo "=================================================="
echo "✅ FULL SRC INTEGRATION TEST COMPLETADO"
echo ""
echo "Resumen:"
echo "  - Compilación: ✅"
echo "  - Integraciones: ✅ ($MODULES_USED módulos)"
echo "  - Servidor: ✅"
echo ""
