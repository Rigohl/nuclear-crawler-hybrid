#!/bin/bash

# Nuclear MCP - Search Results Export Tool
# Extrae y organiza resultados de búsquedas del servidor MCP

set -e

RESULTS_DIR="resultados"
EXPORT_DIR="resultados/exports"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║      🔍 Nuclear MCP - Search Results Extractor & Exporter     ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Crear directorio de exports si no existe
mkdir -p "$EXPORT_DIR"

# Función para procesar y organizar resultados
process_results() {
    local search_type=$1
    local query=$2

    echo "📊 Procesando resultados de: $search_type"
    echo "   Query: $query"

    # Crear estructura de carpetas
    local export_subdir="$EXPORT_DIR/$search_type"
    mkdir -p "$export_subdir"

    # Buscar archivos de resultados
    find "$RESULTS_DIR/$search_type" -type f -name "*.json" 2>/dev/null | while read file; do
        # Copiar y formatear
        cp "$file" "$export_subdir/$(basename "$file")"
        echo "   ✓ Guardado: $(basename "$file")"
    done
}

# Mostrar estructura actual
echo "📁 Estructura de resultados:"
echo ""

if [ -d "$RESULTS_DIR/websearch" ]; then
    echo "websearch/:"
    find "$RESULTS_DIR/websearch" -type f | wc -l
    echo "  archivo(s)"
fi

if [ -d "$RESULTS_DIR/file_search" ]; then
    echo "file_search/:"
    find "$RESULTS_DIR/file_search" -type f | wc -l
    echo "  archivo(s)"
fi

if [ -d "$RESULTS_DIR/deepweb_search" ]; then
    echo "deepweb_search/:"
    find "$RESULTS_DIR/deepweb_search" -type f | wc -l
    echo "  archivo(s)"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Procesar cada tipo
for search_type in websearch file_search deepweb_search; do
    if [ -d "$RESULTS_DIR/$search_type" ]; then
        process_results "$search_type"
    fi
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📂 Resultados organizados en: $EXPORT_DIR"
echo ""
echo "✅ Extracción completada"
