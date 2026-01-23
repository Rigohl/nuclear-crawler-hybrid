# Chapel Professional Build System - Training Pipeline + Chapel AI
# Pure Chapel + C FFI, No Python, Maximum Performance

CHAPEL_HOME ?= /opt/chapel
CHPL := $(CHAPEL_HOME)/bin/chpl
CHPL_FLAGS := --fast -O3 --llvm --optimize
CHPL_MULTI_LOCALE ?= --numLocales=4

# Targets
.PHONY: all train train-multi run run-multi chapel-lib check clean profile help

# Default: Build everything
all: chapel-lib train

# ─────────────────────────────────────────────────────────────────────────────
# Chapel AI Library (shared library for FFI)
# ─────────────────────────────────────────────────────────────────────────────

chapel-lib:
	@echo "🔨 Compiling Chapel AI library (shared)..."
	$(CHPL) --library --dynamic $(CHPL_FLAGS) \
		--ccflags -O3 --ccflags -march=native \
		-o libchapel_ai.so chapel_ai.chpl
	@echo "✅ Library built: libchapel_ai.so"

# ─────────────────────────────────────────────────────────────────────────────
# Training Pipeline (single-locale)
# ─────────────────────────────────────────────────────────────────────────────

train: chapel_ai.chpl training_pipeline.chpl
	@echo "🔨 Compiling training pipeline (single-locale)..."
	mkdir -p bin
	$(CHPL) $(CHPL_FLAGS) \
		-o bin/training_pipeline \
		training_pipeline.chpl chapel_ai.chpl
	@echo "✅ Built: bin/training_pipeline"

# ─────────────────────────────────────────────────────────────────────────────
# Training Pipeline (multi-locale, distributed)
# ─────────────────────────────────────────────────────────────────────────────

train-multi:
	@echo "🔨 Compiling training pipeline (multi-locale, 4 locales)..."
	mkdir -p bin
	$(CHPL) $(CHPL_FLAGS) $(CHPL_MULTI_LOCALE) \
		-o bin/training_pipeline_distributed \
		training_pipeline.chpl chapel_ai.chpl
	@echo "✅ Built multi-locale: bin/training_pipeline_distributed"

# ─────────────────────────────────────────────────────────────────────────────
# Execution
# ─────────────────────────────────────────────────────────────────────────────

run: train
	@echo "🚀 Running training pipeline (single-locale)..."
	./bin/training_pipeline

run-multi: train-multi
	@echo "🚀 Running training pipeline (multi-locale, 4 locales)..."
	./bin/training_pipeline_distributed --numLocales=4

# ─────────────────────────────────────────────────────────────────────────────
# Validation & Debugging
# ─────────────────────────────────────────────────────────────────────────────

check:
	@echo "🔍 Checking Chapel syntax..."
	$(CHPL) --parse-only training_pipeline.chpl chapel_ai.chpl
	@echo "✅ Syntax OK"

profile: train
	@echo "📊 Profiling training pipeline..."
	@time ./bin/training_pipeline

debug: CHPL_FLAGS = --fast -g
debug: clean train
	@echo "✅ Built with debug symbols"

# ─────────────────────────────────────────────────────────────────────────────
# Cleanup
# ─────────────────────────────────────────────────────────────────────────────

clean:
	@echo "🧹 Cleaning build artifacts..."
	rm -rf bin/ *.so *.o
	@echo "✅ Clean complete"

# ─────────────────────────────────────────────────────────────────────────────
# Help
# ─────────────────────────────────────────────────────────────────────────────

help:
	@echo "╔════════════════════════════════════════════════════════════════════╗"
	@echo "║  CHAPEL BUILD SYSTEM - Training Pipeline + AI                    ║"
	@echo "╚════════════════════════════════════════════════════════════════════╝"
	@echo ""
	@echo "Main Targets:"
	@echo "  make all           - Build everything (chapel-lib + train)"
	@echo "  make train         - Build training pipeline (single-locale)"
	@echo "  make train-multi   - Build for distributed execution (multi-locale)"
	@echo "  make run           - Build and run training pipeline"
	@echo "  make run-multi     - Build and run multi-locale version"
	@echo ""
	@echo "Library:"
	@echo "  make chapel-lib    - Compile Chapel AI as shared library"
	@echo ""
	@echo "Validation:"
	@echo "  make check         - Check Chapel syntax"
	@echo "  make profile       - Run with timing measurements"
	@echo "  make debug         - Build with debug symbols"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean         - Remove build artifacts"
	@echo ""
	@echo "Environment Variables:"
	@echo "  CHAPEL_HOME        - Chapel installation (default: /opt/chapel)"
	@echo "  CHPL_MULTI_LOCALE  - Locales for distributed (default: 4)"
	@echo ""

	@echo "   ⚡ Optimizations: MAXIMUM"
	@echo "   🧵 Parallelism: coforall + forall"
	@echo "   🌐 Distributed: Multi-locale support"
	$(CHPL) $(CHPL_FLAGS) $(SOURCE) -o $(TARGET)
	@echo "✅ Chapel AI compiled: $(TARGET)"
	@echo "   📊 Size: $$(du -h $(TARGET) | cut -f1)"

debug: CHPL_FLAGS = --library --dynamic --debug --savec .
debug: $(SOURCE)
	@echo "🔨 Compiling Chapel AI (DEBUG MODE)..."
	$(CHPL) $(CHPL_FLAGS) $(SOURCE) -o $(TARGET)
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
help:
	@echo "Chapel AI Advanced Build System"
	@echo ""
	@echo "Targets:"
	@echo "  make          - Compile with maximum optimizations"
	@echo "  make debug    - Compile with debug symbols"
	@echo "  make install  - Install to ../libs/"
	@echo "  make test     - Run validation tests"
	@echo "  make clean    - Remove build artifacts"
	@echo ""
	@echo "Features:"
	@echo "  ⚡ Maximum optimizations (-O3, --fast)"
	@echo "  🧵 Parallel processing (coforall)"
	@echo "  🌐 Distributed computing ready"
	@echo "  📊 Statistical analysis"
	@echo "  🎯 Path optimization algorithms"
