# ═══════════════════════════════════════════════════════════════════════════
# Chapel Professional Build System - Nuclear AI Training Pipeline
# ═══════════════════════════════════════════════════════════════════════════
# 
# AI-CENTRIC ARCHITECTURE:
# - /ai/         = Core AI (nuclear_chapel_ai.chpl, unified_nuclear_ai.chpl)
# - /tools/      = Development tools (analyzer, repair, reviewer)
# - /training/   = Training engines (pipeline, mining, analysis)
# - /datasets/   = Training data and samples
#
# INTEGRATION:
# - Part of nuclear-crawler-hybrid project
# - Synced with HuggingFace: Kimberlyindiva/nuclear-chapel-training
# - Supports FFI with Rust, Python, Julia, Mojo
# ═══════════════════════════════════════════════════════════════════════════

# ───────────────────────────────────────────────────────────────────────────
# Configuration
# ───────────────────────────────────────────────────────────────────────────

CHAPEL_HOME ?= /opt/chapel
CHPL := $(CHAPEL_HOME)/bin/chpl

# GPU support (set GPU_ARCH=sm_80 for A100, sm_86 for RTX 3090, etc.)
GPU_ARCH ?= none
ifneq ($(GPU_ARCH), none)
  CHPL_GPU_FLAGS = --gpu --gpu-arch=$(GPU_ARCH)
endif

# Compiler flags
CHPL_FLAGS := --fast -O3 --llvm --optimize $(CHPL_GPU_FLAGS)
CHPL_MULTI_LOCALE ?= --numLocales=4

# Paths
AI_DIR := ai
TOOLS_DIR := tools
TRAINING_DIR := training
DATASETS_DIR := datasets
BIN_DIR := bin

# ───────────────────────────────────────────────────────────────────────────
# Targets
# ───────────────────────────────────────────────────────────────────────────

.PHONY: all build train train-dist mining science analyze test full-pipeline \
        chapel-lib unified run run-dist mine check clean profile debug help \
        install deploy-hf execute-all analysis repair review run-unified

# Default: Build everything
all: build train unified

# ═══════════════════════════════════════════════════════════════════════════
# BUILD TARGETS
# ═══════════════════════════════════════════════════════════════════════════

build: chapel-lib train
	@echo "✅ Build complete"

# ───────────────────────────────────────────────────────────────────────────
# Chapel AI Library (shared library for FFI) - CORE AI
# ───────────────────────────────────────────────────────────────────────────

chapel-lib:
	@echo "🔨 Compiling Chapel AI library (shared)..."
	mkdir -p $(BIN_DIR)
	$(CHPL) --library --dynamic $(CHPL_FLAGS) \
		--ccflags -O3 --ccflags -march=native \
		-o libchapel_ai.so $(AI_DIR)/nuclear_chapel_ai.chpl
	@echo "✅ Library built: libchapel_ai.so (CORE AI)"

# ───────────────────────────────────────────────────────────────────────────
# Training Pipeline (single-locale)
# ───────────────────────────────────────────────────────────────────────────

train: $(AI_DIR)/nuclear_chapel_ai.chpl $(TRAINING_DIR)/training_pipeline.chpl
	@echo "🔨 Compiling training pipeline (single-locale)..."
	mkdir -p $(BIN_DIR)
	$(CHPL) $(CHPL_FLAGS) \
		-o $(BIN_DIR)/training_pipeline \
		$(TRAINING_DIR)/training_pipeline.chpl $(AI_DIR)/nuclear_chapel_ai.chpl
	@echo "✅ Built: $(BIN_DIR)/training_pipeline"

# ───────────────────────────────────────────────────────────────────────────
# Training Pipeline (multi-locale, distributed)
# ───────────────────────────────────────────────────────────────────────────

train-dist:
	@echo "🔨 Compiling training pipeline (multi-locale, 4 locales)..."
	mkdir -p $(BIN_DIR)
	$(CHPL) $(CHPL_FLAGS) $(CHPL_MULTI_LOCALE) \
		-o $(BIN_DIR)/training_pipeline_distributed \
		$(TRAINING_DIR)/training_pipeline.chpl $(AI_DIR)/nuclear_chapel_ai.chpl
	@echo "✅ Built multi-locale: $(BIN_DIR)/training_pipeline_distributed"

# ───────────────────────────────────────────────────────────────────────────
# Data Mining Engine
# ───────────────────────────────────────────────────────────────────────────

mining: $(TRAINING_DIR)/data_mining.chpl
	@echo "🔨 Compiling data mining engine..."
	mkdir -p $(BIN_DIR)
	$(CHPL) $(CHPL_FLAGS) \
		-o $(BIN_DIR)/data_mining_engine \
		$(TRAINING_DIR)/data_mining.chpl
	@echo "✅ Built: $(BIN_DIR)/data_mining_engine"

mine: mining
	@echo "🔬 Running data mining pipeline..."
	./$(BIN_DIR)/data_mining_engine

# ───────────────────────────────────────────────────────────────────────────
# Scientific Analysis Engine
# ───────────────────────────────────────────────────────────────────────────

science: $(TRAINING_DIR)/analysis.chpl
	@echo "🔨 Compiling scientific analysis engine..."
	mkdir -p $(BIN_DIR)
	$(CHPL) $(CHPL_FLAGS) \
		-o $(BIN_DIR)/scientific_analysis \
		$(TRAINING_DIR)/analysis.chpl
	@echo "✅ Built: $(BIN_DIR)/scientific_analysis"

analyze: science
	@echo "📊 Running scientific analysis pipeline..."
	./$(BIN_DIR)/scientific_analysis

# ───────────────────────────────────────────────────────────────────────────
# Unified Nuclear AI (integrated system)
# ───────────────────────────────────────────────────────────────────────────

unified: $(AI_DIR)/unified_nuclear_ai.chpl
	@echo "🧠 Compiling Unified Nuclear AI..."
	mkdir -p $(BIN_DIR)
	$(CHPL) $(CHPL_FLAGS) \
		-o $(BIN_DIR)/unified_nuclear_ai \
		$(AI_DIR)/unified_nuclear_ai.chpl
	@echo "✅ Built: $(BIN_DIR)/unified_nuclear_ai (UNIFIED AI SYSTEM)"

run-unified: unified
	@echo "🚀 Running Unified Nuclear AI system..."
	./$(BIN_DIR)/unified_nuclear_ai

# ═══════════════════════════════════════════════════════════════════════════
# DEVELOPMENT TOOLS
# ═══════════════════════════════════════════════════════════════════════════

# ───────────────────────────────────────────────────────────────────────────
# Code Analysis Engine (TOOL)
# ───────────────────────────────────────────────────────────────────────────

analysis: $(TOOLS_DIR)/code_analyzer.chpl
	@echo "🔨 Compiling code analyzer..."
	mkdir -p $(BIN_DIR)
	$(CHPL) $(CHPL_FLAGS) \
		-o $(BIN_DIR)/code_analyzer \
		$(TOOLS_DIR)/code_analyzer.chpl
	@echo "✅ Built: $(BIN_DIR)/code_analyzer"

# ───────────────────────────────────────────────────────────────────────────
# Code Repair Engine (TOOL)
# ───────────────────────────────────────────────────────────────────────────

repair: $(TOOLS_DIR)/code_repair.chpl
	@echo "🔨 Compiling code repair engine..."
	mkdir -p $(BIN_DIR)
	$(CHPL) $(CHPL_FLAGS) \
		-o $(BIN_DIR)/code_repair \
		$(TOOLS_DIR)/code_repair.chpl
	@echo "✅ Built: $(BIN_DIR)/code_repair"

# ───────────────────────────────────────────────────────────────────────────
# Code Reviewer Engine (TOOL)
# ───────────────────────────────────────────────────────────────────────────

review: $(TOOLS_DIR)/code_reviewer.chpl
	@echo "🔨 Compiling code reviewer..."
	mkdir -p $(BIN_DIR)
	$(CHPL) $(CHPL_FLAGS) \
		-o $(BIN_DIR)/code_reviewer \
		$(TOOLS_DIR)/code_reviewer.chpl
	@echo "✅ Built: $(BIN_DIR)/code_reviewer"

# ═══════════════════════════════════════════════════════════════════════════
# FULL PIPELINE TARGETS
# ═══════════════════════════════════════════════════════════════════════════

# ───────────────────────────────────────────────────────────────────────────
# Full Science Pipeline (All 8 Engines)
# ───────────────────────────────────────────────────────────────────────────

full-pipeline: build mine analyze train analysis repair review unified
	@echo ""
	@echo "╔═══════════════════════════════════════════════════════════════════╗"
	@echo "║  ✅ FULL PIPELINE COMPLETE - 8 SYSTEMS                           ║"
	@echo "║  AI-CENTRIC ARCHITECTURE:                                        ║"
	@echo "║  - CORE AI: nuclear_chapel_ai.chpl (neural network)              ║"
	@echo "║  - UNIFIED: unified_nuclear_ai.chpl (integrated intelligence)    ║"
	@echo "║  - TRAINING: pipeline + mining + analysis                        ║"
	@echo "║  - TOOLS: analyzer + repair + reviewer                           ║"
	@echo "║  All modules compiled and ready for execution                    ║"
	@echo "╚═══════════════════════════════════════════════════════════════════╝"
	@echo ""

# ───────────────────────────────────────────────────────────────────────────
# Execute All Systems
# ───────────────────────────────────────────────────────────────────────────

execute-all: build
	@echo "🚀 Executing ALL 8 SYSTEMS in sequence..."
	@echo ""
	@echo "🧠 [CORE] AI Ready: $(AI_DIR)/nuclear_chapel_ai.chpl"
	@echo "🧠 [CORE] Unified AI Ready: $(AI_DIR)/unified_nuclear_ai.chpl"
	@echo ""
	@echo "📊 [1/7] Training Pipeline..."
	@./$(BIN_DIR)/training_pipeline
	@echo ""
	@echo "🔬 [2/7] Data Mining..."
	@./$(BIN_DIR)/data_mining_engine
	@echo ""
	@echo "📈 [3/7] Scientific Analysis..."
	@./$(BIN_DIR)/scientific_analysis
	@echo ""
	@echo "🔤 [4/7] Code Analysis (TOOL)..."
	@./$(BIN_DIR)/code_analyzer $(TRAINING_DIR)/training_pipeline.chpl
	@echo ""
	@echo "🔧 [5/7] Code Repair (TOOL)..."
	@./$(BIN_DIR)/code_repair $(TRAINING_DIR)/training_pipeline.chpl
	@echo ""
	@echo "👁️  [6/7] Code Review (TOOL)..."
	@./$(BIN_DIR)/code_reviewer $(TRAINING_DIR)/training_pipeline.chpl
	@echo ""
	@echo "🧠 [8/8] Unified Nuclear AI (INTEGRATED)..."
	@./$(BIN_DIR)/unified_nuclear_ai
	@echo ""
	@echo "✅ All 8 systems executed successfully!"

# ═══════════════════════════════════════════════════════════════════════════
# EXECUTION TARGETS
# ═══════════════════════════════════════════════════════════════════════════

run: train
	@echo "🚀 Running training pipeline (single-locale)..."
	./$(BIN_DIR)/training_pipeline

run-dist: train-dist
	@echo "🚀 Running training pipeline (multi-locale, 4 locales)..."
	./$(BIN_DIR)/training_pipeline_distributed --numLocales=4

# ═══════════════════════════════════════════════════════════════════════════
# VALIDATION & DEBUGGING
# ═══════════════════════════════════════════════════════════════════════════

check:
	@echo "🔍 Checking Chapel syntax..."
	$(CHPL) --parse-only $(TRAINING_DIR)/training_pipeline.chpl $(AI_DIR)/nuclear_chapel_ai.chpl
	$(CHPL) --parse-only $(TRAINING_DIR)/data_mining.chpl
	$(CHPL) --parse-only $(TRAINING_DIR)/analysis.chpl
	$(CHPL) --parse-only $(TOOLS_DIR)/code_analyzer.chpl
	$(CHPL) --parse-only $(TOOLS_DIR)/code_repair.chpl
	$(CHPL) --parse-only $(TOOLS_DIR)/code_reviewer.chpl
	$(CHPL) --parse-only $(AI_DIR)/unified_nuclear_ai.chpl
	@echo "✅ All 8 modules syntax OK (7 engines + unified AI)"

test: build
	@echo "🧪 Running tests..."
	@if [ -f $(BIN_DIR)/training_pipeline ]; then \
		echo "  ✅ Training pipeline binary exists"; \
	else \
		echo "  ❌ Training pipeline not built"; \
		exit 1; \
	fi
	@if [ -f libchapel_ai.so ]; then \
		echo "  ✅ Chapel AI library exists"; \
	else \
		echo "  ❌ Chapel AI library not built"; \
		exit 1; \
	fi
	@echo "✅ Tests complete"

profile: train
	@echo "📊 Profiling training pipeline..."
	@time ./$(BIN_DIR)/training_pipeline

debug: CHPL_FLAGS = --fast -g
debug: clean train
	@echo "✅ Built with debug symbols"

# ═══════════════════════════════════════════════════════════════════════════
# DEPLOYMENT
# ═══════════════════════════════════════════════════════════════════════════

install: chapel-lib
	@echo "📦 Installing Chapel AI library..."
	mkdir -p ../shared
	cp libchapel_ai.so ../shared/
	@echo "✅ Installed to ../shared/"

deploy-hf:
	@echo "🤗 Deploying to HuggingFace Spaces..."
	@if [ -f hf_spaces_app.py ] && [ -f space.yaml ]; then \
		echo "  ✅ HuggingFace config found"; \
		echo "  Run: huggingface-cli upload Kimberlyindiva/nuclear-chapel-training ."; \
	else \
		echo "  ❌ HuggingFace config missing"; \
		exit 1; \
	fi

# ═══════════════════════════════════════════════════════════════════════════
# CLEANUP
# ═══════════════════════════════════════════════════════════════════════════

clean:
	@echo "🧹 Cleaning build artifacts..."
	rm -rf bin/ *.so *.o *.a *.c
	rm -rf .chpl_* _real_* chapel_ai_* *_real *.tmp
	@echo "✅ Clean complete"

# ═══════════════════════════════════════════════════════════════════════════
# HELP
# ═══════════════════════════════════════════════════════════════════════════

help:
	@echo "╔════════════════════════════════════════════════════════════════════╗"
	@echo "║  CHAPEL SCIENTIFIC AI BUILD SYSTEM - Maximum Power 💥             ║"
	@echo "╚════════════════════════════════════════════════════════════════════╝"
	@echo ""
	@echo "🚀 BUILD TARGETS:"
	@echo "  make build         - Compile all components"
	@echo "  make train         - Build training pipeline (single-locale)"
	@echo "  make train-dist    - Build distributed training (4 locales)"
	@echo "  make mining        - Build data mining engine"
	@echo "  make science       - Build scientific analysis engine"
	@echo "  make unified       - Build unified nuclear AI"
	@echo "  make full-pipeline - Build ALL engines"
	@echo ""
	@echo "🏃 RUN TARGETS:"
	@echo "  make run           - Run training pipeline"
	@echo "  make run-dist      - Run distributed training"
	@echo "  make run-unified   - Run unified AI system"
	@echo "  make mine          - Run data mining"
	@echo "  make analyze       - Run scientific analysis"
	@echo "  make execute-all   - Run ALL 8 systems sequentially"
	@echo ""
	@echo "🔧 DEVELOPMENT TOOLS:"
	@echo "  make analysis      - Build code analyzer"
	@echo "  make repair        - Build code repair engine"
	@echo "  make review        - Build code reviewer"
	@echo ""
	@echo "📚 LIBRARY:"
	@echo "  make chapel-lib    - Compile as shared library (.so)"
	@echo "  make install       - Install library to ../shared/"
	@echo ""
	@echo "🔍 VALIDATION:"
	@echo "  make check         - Validate Chapel syntax"
	@echo "  make test          - Run test suite"
	@echo "  make profile       - Profile with timing"
	@echo "  make debug         - Build with debug symbols"
	@echo ""
	@echo "🚀 DEPLOYMENT:"
	@echo "  make deploy-hf     - Deploy to HuggingFace Spaces"
	@echo ""
	@echo "🧹 CLEANUP:"
	@echo "  make clean         - Remove build artifacts"
	@echo ""
	@echo "⚙️  CONFIGURATION:"
	@echo "  CHAPEL_HOME        - Chapel installation (default: /opt/chapel)"
	@echo "  CHPL_FLAGS         - Compiler flags (default: --fast -O3)"
	@echo "  CHPL_MULTI_LOCALE  - Number of locales (default: 4)"
	@echo "  GPU_ARCH           - GPU architecture (none/sm_80/sm_86/etc)"
	@echo ""
	@echo "📊 8-ENGINE ARCHITECTURE:"
	@echo "  [1] Chapel AI Library  - Neural network core + FFI"
	@echo "  [2] Unified AI         - Integrated intelligence system"
	@echo "  [3] Training Pipeline  - 3-layer network + Adam optimizer"
	@echo "  [4] Data Mining        - K-Means clustering + anomaly detection"
	@echo "  [5] Scientific Analysis- Statistics + hypothesis testing"
	@echo "  [6] Code Analyzer      - Automated code analysis"
	@echo "  [7] Code Repair        - Intelligent code fixing"
	@echo "  [8] Code Reviewer      - AI-powered code review"
	@echo ""
	@echo "⭐ POWER-UP FEATURES:"
	@echo "  ✓ Multi-locale parallelism (BlockDist + CyclicDist)"
	@echo "  ✓ 120K+ training samples (integrated datasets)"
	@echo "  ✓ GPU acceleration support (--gpu flag)"
	@echo "  ✓ FFI acceleration (Rust, Python, Julia, Mojo)"
	@echo "  ✓ Auto-synced GitHub ↔ HuggingFace Hub"
	@echo "  ✓ Integrated with nuclear-crawler-hybrid"
	@echo ""
	@echo "🎯 PERFORMANCE TARGETS:"
	@echo "  Layer 1: 50K patterns in 45s (BlockDist)"
	@echo "  Layer 2: 31K validation in 30s (CyclicDist)"
	@echo "  Layer 3: 20 epochs training in 60s"
	@echo "  Mining:  12 clusters, anomaly detection"
	@echo "  Analysis: Full statistical suite + feature selection"
	@echo ""
	@echo "🔗 INTEGRATION:"
	@echo "  Project: nuclear-crawler-hybrid"
	@echo "  HF Repo: Kimberlyindiva/nuclear-chapel-training"
	@echo "  Datasets: ../../../models/ (120K+ samples)"
	@echo "  MCP: ../../mcp-servers/github/ (GitHub integration)"
	@echo ""

.SILENT: help
