#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════
# Chapel AI - REAL COMPILATION (NO FALLBACKS)
# Build Chapel with maximum advanced features
# ═══════════════════════════════════════════════════════════════════════════

set -e

echo "🔥 Chapel AI - REAL COMPILATION WITH MAXIMUM FEATURES"
echo "════════════════════════════════════════════════════════════════════"

# Detect Chapel compiler
if command -v chpl &> /dev/null; then
    CHPL="chpl"
    echo "✅ Chapel compiler found: $(which chpl)"
    chpl --version
elif [ -n "$CHPL_HOME" ] && [ -x "$CHPL_HOME/bin/chpl" ]; then
    CHPL="$CHPL_HOME/bin/chpl"
    echo "✅ Chapel compiler found at: $CHPL"
    $CHPL --version
else
    echo "❌ ERROR: Chapel compiler not found!"
    echo ""
    echo "Install Chapel:"
    echo "  1. Download from: https://chapel-lang.org/download.html"
    echo "  2. Set CHPL_HOME environment variable"
    echo "  3. Add \$CHPL_HOME/bin to PATH"
    echo ""
    exit 1
fi

# Configuration
CHAPEL_HOME=${CHPL_HOME:-/opt/chapel}
GPU_ARCH=${GPU_ARCH:-none}  # Set to sm_80 (A100), sm_86 (RTX 3090), sm_90 (H100)
NUM_LOCALES=${NUM_LOCALES:-1}  # Set to 4+ for distributed computing

echo ""
echo "📋 Configuration:"
echo "   CHPL_HOME:    $CHAPEL_HOME"
echo "   GPU_ARCH:     $GPU_ARCH"
echo "   NUM_LOCALES:  $NUM_LOCALES"
echo ""

# Advanced compilation flags
# Note: --llvm requires CHPL_LLVM!=none at Chapel build time; use only when available
CHPL_FLAGS="--fast -O3"
CHPL_FLAGS="$CHPL_FLAGS --ccflags -O3 --ccflags -march=native"
CHPL_FLAGS="$CHPL_FLAGS --ccflags -mtune=native"
if [ "${CHPL_LLVM:-none}" != "none" ]; then
    echo "🔧 LLVM backend enabled (CHPL_LLVM=$CHPL_LLVM)"
    CHPL_FLAGS="$CHPL_FLAGS --llvm --optimize"
else
    echo "💡 LLVM backend disabled (CHPL_LLVM=none) — using default backend"
fi

# GPU support
if [ "$GPU_ARCH" != "none" ]; then
    echo "🎮 GPU Support: ENABLED (arch=$GPU_ARCH)"
    CHPL_FLAGS="$CHPL_FLAGS --gpu --gpu-arch=$GPU_ARCH"
else
    echo "💻 GPU Support: DISABLED (CPU only)"
fi

# Multi-locale support
if [ "$NUM_LOCALES" -gt 1 ]; then
    echo "🌐 Distributed Computing: ENABLED ($NUM_LOCALES locales)"
    CHPL_FLAGS="$CHPL_FLAGS --numLocales=$NUM_LOCALES"
else
    echo "🖥️  Single-locale mode"
fi

echo ""
echo "🔨 Compilation flags:"
echo "   $CHPL_FLAGS"
echo ""

# Create output directory
mkdir -p bin

# ═══════════════════════════════════════════════════════════════════════════
# 1. Build Chapel AI Library (Shared Library for FFI)
# ═══════════════════════════════════════════════════════════════════════════
echo "════════════════════════════════════════════════════════════════════"
echo "🧠 [1/8] Building Chapel AI Library (libchapel_ai.so)"
echo "════════════════════════════════════════════════════════════════════"

if [ -f "ai/nuclear_chapel_ai.chpl" ]; then
    $CHPL --library --dynamic $CHPL_FLAGS \
        -o libchapel_ai.so ai/nuclear_chapel_ai.chpl
    echo "✅ Built: libchapel_ai.so"
    ls -lh libchapel_ai.so
else
    echo "⚠️  Skipping: ai/nuclear_chapel_ai.chpl not found"
fi

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# 2. Build Unified Nuclear AI
# ═══════════════════════════════════════════════════════════════════════════
echo "════════════════════════════════════════════════════════════════════"
echo "🧠 [2/8] Building Unified Nuclear AI"
echo "════════════════════════════════════════════════════════════════════"

if [ -f "ai/unified_nuclear_ai.chpl" ]; then
    $CHPL $CHPL_FLAGS \
        -o bin/unified_nuclear_ai ai/unified_nuclear_ai.chpl
    echo "✅ Built: bin/unified_nuclear_ai"
    ls -lh bin/unified_nuclear_ai
else
    echo "⚠️  Skipping: ai/unified_nuclear_ai.chpl not found"
fi

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# 3. Build Training Pipeline
# ═══════════════════════════════════════════════════════════════════════════
echo "════════════════════════════════════════════════════════════════════"
echo "📊 [3/8] Building Training Pipeline"
echo "════════════════════════════════════════════════════════════════════"

if [ -f "training_pipeline.chpl" ] && [ -f "ai/nuclear_chapel_ai.chpl" ]; then
    $CHPL $CHPL_FLAGS \
        -o bin/training_pipeline \
        training_pipeline.chpl ai/nuclear_chapel_ai.chpl
    echo "✅ Built: bin/training_pipeline"
    ls -lh bin/training_pipeline
elif [ -f "training_pipeline.chpl" ]; then
    $CHPL $CHPL_FLAGS \
        -o bin/training_pipeline \
        training_pipeline.chpl
    echo "✅ Built: bin/training_pipeline (standalone)"
    ls -lh bin/training_pipeline
else
    echo "⚠️  Skipping: training_pipeline.chpl not found"
fi

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# 4. Build Data Mining Engine
# ═══════════════════════════════════════════════════════════════════════════
echo "════════════════════════════════════════════════════════════════════"
echo "🔬 [4/8] Building Data Mining Engine"
echo "════════════════════════════════════════════════════════════════════"

if [ -f "training/data_mining.chpl" ]; then
    $CHPL $CHPL_FLAGS \
        -o bin/data_mining_engine training/data_mining.chpl
    echo "✅ Built: bin/data_mining_engine"
    ls -lh bin/data_mining_engine
elif [ -f "data_mining.chpl" ]; then
    $CHPL $CHPL_FLAGS \
        -o bin/data_mining_engine data_mining.chpl
    echo "✅ Built: bin/data_mining_engine (root)"
    ls -lh bin/data_mining_engine
else
    echo "⚠️  Skipping: data_mining.chpl not found (create training/data_mining.chpl to enable)"
fi

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# 5. Build Scientific Analysis Engine
# ═══════════════════════════════════════════════════════════════════════════
echo "════════════════════════════════════════════════════════════════════"
echo "📈 [5/8] Building Scientific Analysis Engine"
echo "════════════════════════════════════════════════════════════════════"

if [ -f "training/analysis.chpl" ]; then
    $CHPL $CHPL_FLAGS \
        -o bin/scientific_analysis training/analysis.chpl
    echo "✅ Built: bin/scientific_analysis"
    ls -lh bin/scientific_analysis
elif [ -f "scientific_analysis.chpl" ]; then
    $CHPL $CHPL_FLAGS \
        -o bin/scientific_analysis scientific_analysis.chpl
    echo "✅ Built: bin/scientific_analysis (root)"
    ls -lh bin/scientific_analysis
else
    echo "⚠️  Skipping: scientific_analysis.chpl not found in training/ or root"
fi

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# 6. Build Code Analyzer (Tool)
# ═══════════════════════════════════════════════════════════════════════════
echo "════════════════════════════════════════════════════════════════════"
echo "🔤 [6/8] Building Code Analyzer"
echo "════════════════════════════════════════════════════════════════════"

if [ -f "tools/code_analyzer.chpl" ]; then
    $CHPL $CHPL_FLAGS \
        -o bin/code_analyzer tools/code_analyzer.chpl
    echo "✅ Built: bin/code_analyzer"
    ls -lh bin/code_analyzer
else
    echo "⚠️  Skipping: tools/code_analyzer.chpl not found"
fi

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# 7. Build Code Repair Engine (Tool)
# ═══════════════════════════════════════════════════════════════════════════
echo "════════════════════════════════════════════════════════════════════"
echo "🔧 [7/8] Building Code Repair Engine"
echo "════════════════════════════════════════════════════════════════════"

if [ -f "tools/code_repair.chpl" ]; then
    $CHPL $CHPL_FLAGS \
        -o bin/code_repair tools/code_repair.chpl
    echo "✅ Built: bin/code_repair"
    ls -lh bin/code_repair
else
    echo "⚠️  Skipping: tools/code_repair.chpl not found"
fi

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# 8. Build Code Reviewer (Tool)
# ═══════════════════════════════════════════════════════════════════════════
echo "════════════════════════════════════════════════════════════════════"
echo "👁️  [8/8] Building Code Reviewer"
echo "════════════════════════════════════════════════════════════════════"

if [ -f "tools/code_reviewer.chpl" ]; then
    $CHPL $CHPL_FLAGS \
        -o bin/code_reviewer tools/code_reviewer.chpl
    echo "✅ Built: bin/code_reviewer"
    ls -lh bin/code_reviewer
else
    echo "⚠️  Skipping: tools/code_reviewer.chpl not found"
fi

echo ""
echo "════════════════════════════════════════════════════════════════════"
echo "✅ CHAPEL AI COMPILATION COMPLETE"
echo "════════════════════════════════════════════════════════════════════"
echo ""
echo "📦 Output:"
ls -lh libchapel_ai.so bin/* 2>/dev/null || echo "   No binaries built"
echo ""
echo "🚀 Next Steps:"
echo "   1. Test library: ldd libchapel_ai.so"
echo "   2. Run training: ./bin/training_pipeline"
echo "   3. Run unified AI: ./bin/unified_nuclear_ai"
echo "   4. Enable GPU: GPU_ARCH=sm_86 $0"
echo "   5. Enable distributed: NUM_LOCALES=4 $0"
echo ""
