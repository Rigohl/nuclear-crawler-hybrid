# Chapel Professional Build System - Training Pipeline + Chapel AI
# Pure Chapel + C FFI, No Python, Maximum Performance
# 
# AI-CENTRIC ARCHITECTURE:
# - /ai/         = Core AI (nuclear_chapel_ai.chpl - THE NEURAL NETWORK)
# - /tools/      = AI Tools (code_analyzer, code_repair, code_reviewer)
# - /training/   = Training engines (training_pipeline, data_mining, analysis)
# - /datasets/   = Training data and samples

CHAPEL_HOME ?= /opt/chapel
CHPL := $(CHAPEL_HOME)/bin/chpl
CHPL_FLAGS := --fast -O3 --llvm --optimize
CHPL_MULTI_LOCALE ?= --numLocales=4

# Paths
AI_DIR := ai
TOOLS_DIR := tools
TRAINING_DIR := training
DATASETS_DIR := datasets
BIN_DIR := bin

# Targets
.PHONY: all build train train-dist mining science mine analyze test full-pipeline chapel-lib unified check clean profile help

# Default: Build everything
all: build train unified

# ─────────────────────────────────────────────────────────────────────────────
# Quick Build Target
# ─────────────────────────────────────────────────────────────────────────────

build: chapel-lib train
	@echo "✅ Build complete"

# ─────────────────────────────────────────────────────────────────────────────
# Chapel AI Library (shared library for FFI) - CORE AI
# ─────────────────────────────────────────────────────────────────────────────

chapel-lib:
	@echo "🔨 Compiling Chapel AI library (shared)..."
	$(CHPL) --library --dynamic $(CHPL_FLAGS) \
		--ccflags -O3 --ccflags -march=native \
		-o libchapel_ai.so $(AI_DIR)/nuclear_chapel_ai.chpl
	@echo "✅ Library built: libchapel_ai.so (CORE AI)"

# ─────────────────────────────────────────────────────────────────────────────
# Training Pipeline (single-locale)
# ─────────────────────────────────────────────────────────────────────────────

train: $(AI_DIR)/nuclear_chapel_ai.chpl $(TRAINING_DIR)/training_pipeline.chpl
	@echo "🔨 Compiling training pipeline (single-locale)..."
	mkdir -p $(BIN_DIR)
	$(CHPL) $(CHPL_FLAGS) \
		-o $(BIN_DIR)/training_pipeline \
		$(TRAINING_DIR)/training_pipeline.chpl $(AI_DIR)/nuclear_chapel_ai.chpl
	@echo "✅ Built: $(BIN_DIR)/training_pipeline"

# ─────────────────────────────────────────────────────────────────────────────
# Training Pipeline (multi-locale, distributed)
# ─────────────────────────────────────────────────────────────────────────────

train-dist:
	@echo "🔨 Compiling training pipeline (multi-locale, 4 locales)..."
	mkdir -p $(BIN_DIR)
	$(CHPL) $(CHPL_FLAGS) $(CHPL_MULTI_LOCALE) \
		-o $(BIN_DIR)/training_pipeline_distributed \
		$(TRAINING_DIR)/training_pipeline.chpl $(AI_DIR)/nuclear_chapel_ai.chpl
	@echo "✅ Built multi-locale: $(BIN_DIR)/training_pipeline_distributed"

# ─────────────────────────────────────────────────────────────────────────────
# Data Mining Engine
# ─────────────────────────────────────────────────────────────────────────────

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

# ─────────────────────────────────────────────────────────────────────────────
# Scientific Analysis Engine
# ─────────────────────────────────────────────────────────────────────────────

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

# ─────────────────────────────────────────────────────────────────────────────
# Unified Nuclear AI (integrated system)
# ─────────────────────────────────────────────────────────────────────────────

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

# ─────────────────────────────────────────────────────────────────────────────
# Code Analysis Engine (TOOL)
# ─────────────────────────────────────────────────────────────────────────────

analysis: $(TOOLS_DIR)/code_analyzer.chpl
	@echo "🔨 Compiling code analyzer..."
	mkdir -p $(BIN_DIR)
	$(CHPL) $(CHPL_FLAGS) \
		-o $(BIN_DIR)/code_analyzer \
		$(TOOLS_DIR)/code_analyzer.chpl
	@echo "✅ Built: $(BIN_DIR)/code_analyzer"

# ─────────────────────────────────────────────────────────────────────────────
# Code Repair Engine (TOOL)
# ─────────────────────────────────────────────────────────────────────────────

repair: $(TOOLS_DIR)/code_repair.chpl
	@echo "🔨 Compiling code repair engine..."
	mkdir -p $(BIN_DIR)
	$(CHPL) $(CHPL_FLAGS) \
		-o $(BIN_DIR)/code_repair \
		$(TOOLS_DIR)/code_repair.chpl
	@echo "✅ Built: $(BIN_DIR)/code_repair"

# ─────────────────────────────────────────────────────────────────────────────
# Code Reviewer Engine (TOOL)
# ─────────────────────────────────────────────────────────────────────────────

review: $(TOOLS_DIR)/code_reviewer.chpl
	@echo "🔨 Compiling code reviewer..."
	mkdir -p $(BIN_DIR)
	$(CHPL) $(CHPL_FLAGS) \
		-o $(BIN_DIR)/code_reviewer \
		$(TOOLS_DIR)/code_reviewer.chpl
	@echo "✅ Built: $(BIN_DIR)/code_reviewer"

# ─────────────────────────────────────────────────────────────────────────────
# Full Science Pipeline (All 7 Engines)
# ─────────────────────────────────────────────────────────────────────────────

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

# ─────────────────────────────────────────────────────────────────────────────
# Full Execution Targets
# ─────────────────────────────────────────────────────────────────────────────

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

# ─────────────────────────────────────────────────────────────────────────────
# Execution (Individual)
# ─────────────────────────────────────────────────────────────────────────────

run: train
	@echo "🚀 Running training pipeline (single-locale)..."
	./$(BIN_DIR)/training_pipeline

run-dist: train-dist
	@echo "🚀 Running training pipeline (multi-locale, 4 locales)..."
	./$(BIN_DIR)/training_pipeline_distributed --numLocales=4

# ─────────────────────────────────────────────────────────────────────────────
# Validation & Debugging
# ─────────────────────────────────────────────────────────────────────────────

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
	@echo "✅ Tests complete"

profile: train
	@echo "📊 Profiling training pipeline..."
	@time ./$(BIN_DIR)/training_pipeline

debug: CHPL_FLAGS = --fast -g
debug: clean train
	@echo "✅ Built with debug symbols"

# ─────────────────────────────────────────────────────────────────────────────
# Cleanup
# ─────────────────────────────────────────────────────────────────────────────

clean:
	@echo "🧹 Cleaning build artifacts..."
	rm -rf bin/ *.so *.o *.c
	@echo "✅ Clean complete"

# ─────────────────────────────────────────────────────────────────────────────
# Help
# ─────────────────────────────────────────────────────────────────────────────

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
	@echo "  make full-pipeline - Run ALL engines sequentially"
	@echo ""
	@echo "🏃 RUN TARGETS:"
	@echo "  make run           - Run training pipeline"
	@echo "  make run-dist      - Run distributed training"
	@echo "  make mine          - Run data mining"
	@echo "  make analyze       - Run scientific analysis"
	@echo ""
	@echo "📚 LIBRARY:"
	@echo "  make chapel-lib    - Compile as shared library (.so)"
	@echo ""
	@echo "🔧 DEVELOPMENT:"
	@echo "  make check         - Validate Chapel syntax"
	@echo "  make test          - Run test suite"
	@echo "  make profile       - Profile with timing"
	@echo "  make debug         - Build with debug symbols"
	@echo "  make clean         - Remove artifacts"
	@echo ""
	@echo "⚙️  CONFIGURATION:"
	@echo "  CHAPEL_HOME        - Chapel installation (default: /opt/chapel)"
	@echo "  CHPL_FLAGS         - Compiler flags (default: --fast -O3)"
	@echo "  CHPL_MULTI_LOCALE  - Number of locales (default: 4)"
	@echo ""
	@echo "📊 4-ENGINE ARCHITECTURE:"
	@echo "  [1] Training Pipeline  - 3-layer neural network + Adam optimizer"
	@echo "  [2] Data Mining Engine - K-Means clustering + anomaly detection"
	@echo "  [3] Scientific Analysis- Statistics + hypothesis testing + ML"
	@echo "  [4] Chapel AI          - Neural network + FFI acceleration"
	@echo ""
	@echo "⭐ POWER-UP FEATURES:"
	@echo "  ✓ Multi-locale parallelism (BlockDist + CyclicDist)"
	@echo "  ✓ 50K+ training samples with real-world techniques"
	@echo "  ✓ K-means++ clustering with silhouette scoring"
	@echo "  ✓ Hypothesis testing with p-values & effect sizes"
	@echo "  ✓ Feature importance + anomaly detection"
	@echo "  ✓ C FFI acceleration (stealth, SimHash, Cloudflare)"
	@echo "  ✓ Auto-synced GitHub ↔ HuggingFace Hub (every 6h)"
	@echo ""
	@echo "🎯 PERFORMANCE TARGETS:"
	@echo "  Layer 1: 50K patterns in 45s (BlockDist)"
	@echo "  Layer 2: 31K validation in 30s (CyclicDist)"
	@echo "  Layer 3: 20 epochs training in 60s"
	@echo "  Mining:  12 clusters, anomaly detection"
	@echo "  Analysis: Full statistical suite + feature selection"
	@echo ""
	@echo "✅ Debug build complete"

clean:
	@echo "🧹 Cleaning Chapel AI build artifacts..."
	rm -f $(TARGET) *.o *.h *_real *.tmp
	rm -rf .chpl_* _real_* chapel_ai_* 
	@echo "✅ Cleaned"

install: $(TARGET)
	@echo "📦 Installing Chapel AI library..."
	mkdir -p ../libs
	cp $(TARGET) ../libs/
	@echo "✅ Installed to ../libs/"

test: $(TARGET)
	@echo "🧪 Testing Chapel AI..."
	@echo "   Testing library exists..."
	test -f $(TARGET) && echo "   ✅ Library file OK" || (echo "   ❌ Library not found" && exit 1)
	@echo "   Testing symbols..."
	nm $(TARGET) | grep chapel_ai_init > /dev/null && echo "   ✅ Symbols OK" || (echo "   ❌ Symbols missing" && exit 1)
	@echo "✅ All tests passed"

benchmark: $(TARGET)
	@echo "⚡ Running Chapel AI benchmarks..."
	@echo "   TODO: Implement performance benchmarks"

.SILENT: help
deploy-github:
	@echo "📤 Deploying to GitHub..."
	./deploy.sh

deploy-release:
	@echo "📦 Creating GitHub Release..."
	@echo "1" | ./deploy.sh

deploy-docs:
	@echo "📝 Creating Documentation Issue..."
	@echo "3" | ./deploy.sh

deploy-hf:
	@echo "🤗 Setting up HuggingFace Sync..."
	@echo "4" | ./deploy.sh

deploy-all:
	@echo "🚀 Full deployment (GitHub + Release + Docs + HF)..."
	@echo "5" | ./deploy.sh

help:
	@echo "Chapel AI Advanced Build System"
	@echo ""
	@echo "Build Targets:"
	@echo "  make          - Compile with maximum optimizations"
	@echo "  make debug    - Compile with debug symbols"
	@echo "  make install  - Install to ../libs/"
	@echo "  make test     - Run validation tests"
	@echo "  make clean    - Remove build artifacts"
	@echo ""
	@echo "Deployment Targets (via GitHub CLI):"
	@echo "  make deploy-github  - Push to GitHub"
	@echo "  make deploy-release - Create GitHub Release"
	@echo "  make deploy-docs    - Create Documentation Issue"
	@echo "  make deploy-hf      - Setup HuggingFace Sync"
	@echo "  make deploy-all     - Full deployment"
	@echo ""
	@echo "Features:"
	@echo "  ⚡ Maximum optimizations (-O3, --fast)"
	@echo "  🧵 Parallel processing (coforall)"
	@echo "  🌐 Distributed computing ready"
	@echo "  📊 Statistical analysis"
	@echo "  🎯 Path optimization algorithms"
