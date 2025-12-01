#!/usr/bin/env bash
# Script para verificar y demostrar la conservación de binarios Chapel

echo "🔍 Verificando compilados de Chapel..."
echo ""

# Buscar en target/release
if [ -f "target/release/parallel_processor_chapel" ] || [ -f "target/release/parallel_processor_chapel.exe" ]; then
    echo "✅ Chapel binary encontrado en target/release/"
    ls -lh target/release/parallel_processor_chapel* 2>/dev/null || ls -lh target/release/parallel_processor_chapel.exe
    echo ""
    echo "📋 Información:"
    file target/release/parallel_processor_chapel* 2>/dev/null || file target/release/parallel_processor_chapel.exe
else
    echo "⚠️  No se encontró Chapel binary"
    echo ""
    echo "Para compilar Chapel automáticamente:"
    echo "1. Instala Chapel:"
    echo "   - Windows: choco install chapel"
    echo "   - Linux: sudo apt install chapel"
    echo "   - macOS: brew install chapel"
    echo ""
    echo "2. Compila con Cargo:"
    echo "   cargo build --release"
    echo ""
    echo "3. El binario se guardará en: target/release/parallel_processor_chapel"
fi

echo ""
echo "🔧 Estructura de compilación:"
echo "chapel/"
echo "├── parallel_processor.chpl  (fuente)"
echo "└── (compilado a) → target/release/parallel_processor_chapel*"
echo ""
echo "✅ Los binarios se conservan automáticamente con cada cargo build"
