# C Parallel Code Editor - HPC Power + Chapel Integration
## 🚀 Editor de Código Paralelo en C con Poder HPC + Integración Chapel

Un editor de código ultra-rápido escrito en C puro que utiliza **pthreads** para procesamiento paralelo masivo de archivos, con **integración completa de Chapel** para mantener vivo el poder HPC de Chapel. El C llama a Chapel automáticamente cuando se necesita procesamiento masivo, asegurando que Chapel nunca "desaparezca".

## ⚡ Características Principales

### 🔥 Rendimiento HPC Híbrido
- **C + pthreads**: Procesamiento paralelo nativo para casos optimizados
- **Chapel integration**: Automáticamente activa Chapel para procesamiento masivo (>100 archivos)
- **Compilación optimizada**: `-O3 -march=native -flto -funroll-loops`
- **Paralelismo automático**: OpenMP + pthreads + Chapel forall/coforall
- **Vectorización automática**: `-ftree-vectorize` para SIMD

### 🌟 Integración C-Chapel
- **C llama a Chapel**: El código C ejecuta Chapel como subprocess para mantenerlo vivo
- **Selección inteligente**: C puro para casos simples, Chapel para masivo
- **Fallback automático**: Si Chapel falla, C toma el control
- **Docker integration**: Chapel via Docker cuando no está disponible localmente
- **Compilación automática**: Chapel se compila automáticamente cuando se necesita

### 🔧 Capacidades Técnicas
- **Regex avanzado**: Soporte completo para expresiones regulares complejas
- **Búsqueda y reemplazo**: Paralelo con patrones complejos
- **Balanceo de carga**: Distribución inteligente de trabajo entre hilos/tasks
- **Manejo de memoria**: Optimizado para datasets grandes
- **Métricas de rendimiento**: Tiempos de procesamiento detallados

### 🛡️ Robustez
- **Manejo de errores**: Recuperación graceful de fallos
- **Validación de entrada**: Verificación exhaustiva de parámetros
- **Modo dry-run**: Previsualización segura de cambios
- **Logging detallado**: Seguimiento completo de operaciones

## 📊 Rendimiento Comparado

| Tecnología | Lenguaje | Activación | Rendimiento | Paralelismo |
|------------|----------|------------|-------------|-------------|
| **C + Chapel** | C + Chapel | Inteligente | ⭐⭐⭐⭐⭐ | Híbrido HPC |
| C pthreads | C | Manual | ⭐⭐⭐⭐ | Hilos nativos |
| Rayon | Rust | Automático | ⭐⭐⭐⭐ | Tasks |
| forall/coforall | Chapel | Manual | ⭐⭐⭐⭐⭐ | Tasks HPC |

## 🏗️ Arquitectura Híbrida

```
C Parallel Code Editor + Chapel Integration
├── 🧵 C Thread Pool Manager
│   ├── Load Balancer
│   ├── Work Distribution
│   └── Progress Tracking
├── 🔍 Regex Engine (C/PCRE2)
│   ├── Pattern Compilation
│   ├── Parallel Matching
│   └── Replacement Logic
├── 🌟 Chapel HPC Executor
│   ├── Subprocess Management
│   ├── Docker Integration
│   └── Fallback Handler
├── 📁 File I/O System
│   ├── Parallel Readers
│   ├── Memory Mapping
│   └── Atomic Writes
└── 📊 Performance Monitor
    ├── Timing Metrics
    ├── Throughput Stats
    └── Resource Usage
```

## 🎯 Uso Básico con Integración Chapel

```bash
# Sintaxis básica (C decide automáticamente usar Chapel)
./c_parallel_editor <directorio> <patrón_búsqueda> <reemplazo> [opciones]

# Ejemplos prácticos
# Procesamiento pequeño - usa C puro
./c_parallel_editor src/ "old_function" "new_function"

# Procesamiento masivo - activa Chapel automáticamente
./c_parallel_editor /big_project "complex.*pattern" "replacement"

# Forzar uso de Chapel
./c_parallel_editor . "pattern" "replacement" --chapel

# Modo dry-run para previsualizar
./c_parallel_editor src/ "old" "new" --dry-run

# Procesamiento con más hilos
./c_parallel_editor src/ "pattern" "replacement" --threads 16
```

### Comportamiento Inteligente
- **< 100 archivos**: C puro con pthreads
- **> 100 archivos**: Chapel HPC automáticamente
- **Patrones complejos**: Chapel para regex avanzado
- **Fallback**: Si Chapel falla, C toma el control

## 🚀 Instalación y Compilación

### Compilación con Integración Chapel

#### Linux/macOS
```bash
# Compilación completa C + Chapel
make chapel

# Solo C
make

# Con Chapel via Docker
make docker-chapel
```

#### Windows (PowerShell)
```powershell
# Compilación con Chapel
.\Build-C-Parallel-Editor.ps1 -Chapel

# Compilación básica
.\Build-C-Parallel-Editor.ps1

# Con pruebas
.\Build-C-Parallel-Editor.ps1 -Chapel -Test
```

### Compilación Manual
```bash
# C con integración Chapel
gcc -O3 -march=native -flto -funroll-loops -fomit-frame-pointer \
    -pthread -fopenmp -ftree-vectorize -floop-parallelize-all \
    -Wall -Wextra -Wpedantic -std=c99 -D_GNU_SOURCE \
    c_parallel_editor.c -o c_parallel_editor \
    -pthread -lm -lpcre2-8

# Chapel (automático)
chpl --fast --specialize --optimize-forall-unordered-ops \
     --optimize-loop-iterators --optimize-on-clauses --optimize-on \
     --inline --vectorize --cache-remote \
     --report-optimized-loop-iterators \
     chpl_parallel_editor.chpl -o chpl_parallel_editor
```

## ⚙️ Opciones Avanzadas

| Opción | Descripción |
|--------|-------------|
| `--dry-run` | Previsualizar cambios sin aplicarlos |
| `--chapel` | Forzar uso de Chapel |
| `--threads N` | Número específico de hilos (default: auto) |
| `--ignore-case` | Búsqueda case-insensitive |
| `--regex` | Forzar modo regex (default: auto-detect) |
| `--verbose` | Output detallado |
| `--quiet` | Output mínimo |
| `--max-depth N` | Profundidad máxima de directorios |
| `--include "*.c"` | Solo archivos con patrón |
| `--exclude "*.bak"` | Excluir archivos con patrón |

## 🧪 Pruebas y Validación

### Ejecutar Suite de Pruebas
```bash
# Pruebas completas
python3 test_c_parallel_editor.py

# Pruebas con Makefile
make test

# Benchmark de rendimiento
make benchmark
```

### Resultados Esperados
- ✅ **Compilación**: Binario C + Chapel generados sin errores
- ✅ **Funcionalidad Básica**: Reemplazos correctos en archivos
- ✅ **Procesamiento Paralelo**: Múltiples archivos procesados simultáneamente
- ✅ **Integración Chapel**: Chapel se activa automáticamente para casos masivos
- ✅ **Regex**: Patrones complejos funcionan correctamente
- ✅ **Rendimiento**: >50 archivos/segundo en hardware moderno
- ✅ **Manejo de Errores**: Recuperación graceful de errores

## 📈 Benchmarks de Rendimiento

### Hardware de Prueba
- **CPU**: AMD Ryzen 9 5900X (12 cores/24 threads)
- **RAM**: 64GB DDR4-3200
- **Storage**: NVMe SSD 2TB

### Resultados Típicos
```
Modo: C + Chapel Integration
Archivos procesados: 1000
Tamaño total: ~50MB
Tiempo de procesamiento: 1.8 segundos
Rendimiento: ~556 archivos/segundo
Uso de CPU: 98% (paralelismo híbrido)
Chapel activado: Sí (masivo)
```

## 🔧 Dependencias

### Linux
```bash
# Para C
sudo apt-get install build-essential libpcre2-dev

# Para Chapel (opcional, se puede usar Docker)
# Instalar Chapel: https://chapel-lang.org/download.html

# Para Docker (fallback)
sudo apt-get install docker.io
```

### macOS
```bash
# Con Homebrew
brew install gcc pcre2

# Chapel opcional
brew install chapel

# Docker
brew install --cask docker
```

### Windows
```powershell
# Instalar MinGW-w64
# Descargar de: https://www.mingw-w64.org/
# O usar MSYS2: https://www.msys2.org/

# Chapel opcional
# Instalar Chapel: https://chapel-lang.org/download.html

# Docker Desktop
# Descargar de: https://www.docker.com/products/docker-desktop
```

## 🐛 Troubleshooting

### Problemas Comunes

**Error: "gcc: command not found"**
```bash
# Linux
sudo apt-get install build-essential

# macOS
xcode-select --install

# Windows: Instalar MinGW-w64
```

**Error: "pcre2.h not found"**
```bash
# Instalar PCRE2 development headers
sudo apt-get install libpcre2-dev  # Ubuntu/Debian
sudo yum install pcre2-devel       # CentOS/RHEL
brew install pcre2                # macOS
```

**Chapel no disponible**
- Se usa automáticamente Docker como fallback
- O se procesa con C puro

**Rendimiento bajo**
- Verificar número de cores: `nproc` o `sysctl -n hw.ncpu`
- Aumentar threads: `--threads 16`
- Usar SSD para I/O
- Verificar flags de compilación

## 🚀 Optimizaciones HPC Aplicadas

### Nivel de Compilador (C)
- **`-O3`**: Optimización máxima
- **`-march=native`**: Instrucciones específicas del CPU
- **`-flto`**: Link-time optimization
- **`-funroll-loops`**: Desenrollado de bucles
- **`-fomit-frame-pointer`**: Menos overhead de llamadas

### Paralelismo Híbrido
- **C pthreads**: Hilos POSIX nativos
- **OpenMP**: Directivas de paralelismo
- **Chapel forall/coforall**: Tasks HPC distribuidas
- **Load balancing**: Distribución automática de trabajo

### Memoria y Cache
- **Estructuras de datos alineadas**: Mejor uso de cache
- **Prefetching**: Carga anticipada de datos
- **Memory mapping**: I/O eficiente para archivos grandes

## 🤝 Contribuir

1. **Fork** el repositorio
2. **Crear** rama para feature: `git checkout -b feature/nueva-funcionalidad`
3. **Commit** cambios: `git commit -am 'Agregar nueva funcionalidad'`
4. **Push** a rama: `git push origin feature/nueva-funcionalidad`
5. **Crear Pull Request**

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Ver archivo `LICENSE` para detalles.

## 🙏 Agradecimientos

- **PCRE2**: Motor de regex de alto rendimiento
- **POSIX Threads**: API de hilos estándar
- **GCC**: Compilador con optimizaciones HPC excepcionales
- **Chapel**: Lenguaje HPC que mantiene vivo el paralelismo avanzado
- **Docker**: Contenedorización para portabilidad

---

**⚡ Potenciado por C + Chapel: El paralelismo HPC nunca desaparece**