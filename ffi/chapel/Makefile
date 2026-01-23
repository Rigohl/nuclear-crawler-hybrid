# Chapel AI Advanced Makefile
# Compiles Chapel code to shared library for FFI with advanced optimizations

CHPL = chpl
TARGET = libchapel_ai.so
SOURCE = chapel_ai.chpl

# Advanced compiler flags
CHPL_FLAGS = --library --dynamic
CHPL_FLAGS += --fast  # Maximum optimizations
CHPL_FLAGS += --no-checks  # Disable runtime checks for production
CHPL_FLAGS += --ccflags -O3  # C compiler optimizations
CHPL_FLAGS += --ccflags -march=native  # Native CPU optimizations

.PHONY: all clean install test debug

all: $(TARGET)

$(TARGET): $(SOURCE)
	@echo "🔨 Compiling Chapel AI (ADVANCED MODE)..."
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
