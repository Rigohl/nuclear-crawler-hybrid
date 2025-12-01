# Makefile para C Parallel Code Editor - HPC Power
# ================================================

CC = gcc
CFLAGS = -O3 -march=native -flto -funroll-loops -fomit-frame-pointer -pthread
CFLAGS += -Wall -Wextra -Wpedantic -std=c99
CFLAGS += -D_GNU_SOURCE  # Para funciones POSIX

# Flags HPC adicionales
CFLAGS += -fopenmp  # OpenMP para paralelismo adicional
CFLAGS += -ftree-vectorize  # Auto-vectorización
CFLAGS += -floop-parallelize-all  # Paralelización automática de bucles

LDFLAGS = -pthread -lm -lpcre2-8  # PCRE2 para regex avanzado

TARGET = c_parallel_editor
SRC = c_parallel_editor.c

# Detectar número de cores para optimización
NUM_CORES := $(shell nproc 2>/dev/null || echo 4)
MAKEFLAGS += -j$(NUM_CORES)

.PHONY: all clean install test benchmark help

all: $(TARGET)
	@echo "✅ Compilación completada con optimizaciones HPC"
	@echo "🎯 Binario listo: $(TARGET)"
	@ls -la $(TARGET)

$(TARGET): $(SRC)
	@echo "🔨 Compilando con GCC HPC optimizations..."
	$(CC) $(CFLAGS) $(SRC) -o $(TARGET) $(LDFLAGS)

# Compilación con profiling para optimización adicional
profile: CFLAGS += -pg -g
profile: $(TARGET)
	@echo "📊 Binario con profiling listo para análisis de rendimiento"

# Compilación con debugging
debug: CFLAGS = -g -O0 -Wall -Wextra -pthread -DDEBUG
debug: $(TARGET)
	@echo "🐛 Binario de debug listo"

# Compilación estática para máxima portabilidad
static: LDFLAGS += -static
static: $(TARGET)
	@echo "📦 Binario estático creado"

# Instalar en sistema
install: $(TARGET)
	@echo "📥 Instalando en /usr/local/bin..."
	sudo cp $(TARGET) /usr/local/bin/
	sudo chmod +x /usr/local/bin/$(TARGET)
	@echo "✅ Instalado. Uso: c_parallel_editor"

# Ejecutar pruebas
test: $(TARGET)
	@echo "🧪 Ejecutando pruebas..."
	@echo "📝 Creando archivos de prueba..."
	mkdir -p test_files
	echo 'void oldFunction() { printf("old"); }' > test_files/test1.c
	echo 'oldFunction(); oldFunction();' > test_files/test2.c

	@echo "🔍 Ejecutando dry-run..."
	./$(TARGET) test_files "oldFunction" "newFunction" --dry-run

	@echo "✏️  Aplicando cambios..."
	./$(TARGET) test_files "oldFunction" "newFunction"

	@echo "🔍 Verificando resultados..."
	cat test_files/test1.c
	cat test_files/test2.c

	@echo "🧹 Limpiando..."
	rm -rf test_files

# Benchmark de rendimiento
benchmark: $(TARGET)
	@echo "📊 Ejecutando benchmark de rendimiento..."
	@echo "📝 Preparando archivos de prueba..."
	mkdir -p benchmark_data
	@for i in $$(seq 1 1000); do \
		echo "void function$$i() { oldFunction(); }" > benchmark_data/file$$i.c; \
	done

	@echo "⚡ Ejecutando benchmark..."
	time ./$(TARGET) benchmark_data "oldFunction" "newFunction"

	@echo "🧹 Limpiando..."
	rm -rf benchmark_data

# Información del sistema
info:
	@echo "🖥️  Información del sistema:"
	@echo "   CPU: $$(grep -c ^processor /proc/cpuinfo 2>/dev/null || echo 'N/A') cores"
	@echo "   Memoria: $$(free -h | grep Mem | awk '{print $$2}' 2>/dev/null || echo 'N/A')"
	@echo "   GCC: $$(gcc --version | head -1)"
	@echo "   OpenMP: $$(echo '#include <omp.h>' | gcc -E - | grep -q omp_get_num_threads && echo 'Sí' || echo 'No')"

# Ayuda
help:
	@echo "🔬 C PARALLEL CODE EDITOR - HPC POWER"
	@echo "====================================="
	@echo ""
	@echo "🎯 Comandos disponibles:"
	@echo "  make           - Compilar con optimizaciones HPC"
	@echo "  make debug     - Compilar con debugging"
	@echo "  make profile   - Compilar con profiling"
	@echo "  make static    - Compilar binario estático"
	@echo "  make install   - Instalar en sistema"
	@echo "  make test      - Ejecutar pruebas"
	@echo "  make benchmark - Ejecutar benchmark de rendimiento"
	@echo "  make info      - Información del sistema"
	@echo "  make clean     - Limpiar archivos generados"
	@echo "  make help      - Mostrar esta ayuda"
	@echo ""
	@echo "⚡ Optimizaciones HPC aplicadas:"
	@echo "  • -O3 -march=native -flto"
	@echo "  • -fopenmp -ftree-vectorize"
	@echo "  • -funroll-loops -fomit-frame-pointer"
	@echo "  • Paralelización automática de bucles"

# Limpiar
clean:
	@echo "🧹 Limpiando archivos generados..."
	rm -f $(TARGET) *.o *.gcda *.gcno gmon.out
	rm -rf test_files benchmark_data

# Regla por defecto
.DEFAULT_GOAL := all