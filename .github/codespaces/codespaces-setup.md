# 🚀 GitHub Codespaces Setup - Nuclear Crawler Hybrid

## Quick Start

Este Codespace viene **pre-configurado** con:
- ⛪ **Chapel 2.1.0** (lenguaje HPC con paralelismo)
- 🐍 **Python 3.11** + CFFI + Playwright
- 🦀 **Rust** + wasm-pack
- 🐹 **Go 1.21**
- 📦 **Node.js 20** + Puppeteer
- 🔗 **FFI completo** (todos los lenguajes interconectados)

## Uso Inmediato

### 1. Validar el Environment (30 segundos)

```bash
./validate-environment.sh
```

**Output esperado**:
```
✅ Chapel compiler: 2.1.0
✅ Python: 3.11.x
✅ Rust: 1.x
✅ Go: 1.21.x
✅ Node.js: 20.x
✅ Python CFFI installed
✅ Playwright installed
```

---

### 2. Test Chapel (1 minuto)

```bash
cd /workspace/chapel-examples
./hello
```

**Output esperado**:
```
Chapel is ready! Nuclear Crawler Hybrid initialized.
FFI Working!
FFI call result: 0
```

---

### 3. Test FFI Multi-Lenguaje (2 minutos)

#### Python FFI
```bash
cd /workspace/ffi
python python_ffi.py
```

#### Rust FFI
```bash
cd /workspace/ffi
rustc --crate-type=cdylib rust_ffi.rs
# Ahora puedes llamarlo desde Chapel
```

#### Chapel FFI Wrapper
```bash
cd /workspace/ffi
chpl chapel_ffi.chpl -o chapel_ffi
./chapel_ffi
```

---

## Estructura del Workspace

```
/workspace/
├── chapel-examples/       # Ejemplos de Chapel
│   └── hello.chpl        # Hello world con FFI
├── ffi/                  # Foreign Function Interfaces
│   ├── python_ffi.py     # Python ↔ Chapel
│   ├── rust_ffi.rs       # Rust ↔ Chapel
│   └── chapel_ffi.chpl   # Chapel wrapper
├── config/               # Configuración del crawler
│   └── crawler.toml      # Config principal
├── src/                  # Código fuente
│   ├── chapel/          # Código Chapel
│   ├── rust/            # Código Rust
│   ├── python/          # Código Python
│   └── go/              # Código Go
├── data/                 # Datos de entrada
├── output/               # Resultados
├── logs/                 # Logs
└── tests/                # Tests
```

---

## Nuclear Crawler - Inicio Rápido

### Arquitectura Híbrida

Este proyecto usa **Chapel como orquestador** y FFI para:
- **Python**: Web scraping (BeautifulSoup, Scrapy)
- **Rust**: Performance crítico + WASM
- **Go**: Concurrency y networking
- **Node.js**: Browser automation (Puppeteer)

### Ejemplo de Workflow

```chapel
// nuclear_crawler.chpl
use ChapelFFI;

// Orquestar con Chapel (paralelismo masivo)
forall url in urls do {
  // Delegar scraping a Python
  const html = python_scrape(url);
  
  // Procesar con Rust (velocidad)
  const data = rust_parse(html);
  
  // Guardar con Go (concurrency)
  go_save_to_db(data);
}
```

---

## Configuración Avanzada

### Chapel Parallelism

```bash
# Configurar número de locales
export CHPL_RT_NUM_THREADS_PER_LOCALE=16

# Configurar NUMA
export CHPL_LOCALE_MODEL=numa

# Compilar con optimizaciones
chpl --fast myprogram.chpl
```

### FFI Loading

```chapel
// Cargar biblioteca externa
extern proc my_function(arg: c_int): c_int;

// Usar desde Chapel
const result = my_function(42);
```

---

## Troubleshooting

### Chapel no compila

```bash
# Verificar CHPL_HOME
echo $CHPL_HOME
# Debe mostrar: /opt/chapel

# Re-source environment
source $CHPL_HOME/util/setchplenv.bash
```

### Python CFFI error

```bash
# Reinstalar CFFI
pip install --upgrade cffi pycparser
```

### Playwright falla

```bash
# Reinstalar browsers
playwright install chromium --with-deps
```

---

## Performance Tips

### Chapel Optimization

```bash
# Compilación optimizada
chpl --fast --optimize myprogram.chpl

# Con profiling
chpl --profile myprogram.chpl
```

### Parallel Degree

```chapel
// Controlar paralelismo
config const parallelDegree = 16;

forall i in 1..100 with (maxDegree=parallelDegree) do {
  // código paralelo
}
```

---

## Recursos

### Chapel
- [Chapel Docs](https://chapel-lang.org/docs/)
- [Chapel Examples](https://github.com/chapel-lang/chapel/tree/main/test/examples)
- [Chapel FFI Guide](https://chapel-lang.org/docs/technotes/extern.html)

### FFI
- [CFFI Docs](https://cffi.readthedocs.io/)
- [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html)
- [Go CGo](https://golang.org/cmd/cgo/)

### Nuclear Crawler
- [Project README](/workspace/README.md)
- [Tools Guide](/workspace/TOOLS.md)
- [Skills Guide](/workspace/SKILLS.md)

---

## Next Steps

1. ✅ Valida environment: `./validate-environment.sh`
2. ✅ Test Chapel: `cd chapel-examples && ./hello`
3. ✅ Explora FFI: `cd ffi && ls`
4. ✅ Lee config: `cat config/crawler.toml`
5. ✅ Comienza a codear! 🚀

---

**Status**: ✅ Codespace Ready  
**Chapel**: 2.1.0 con FFI completo  
**Languages**: Python, Rust, Go, Node.js  
**Performance**: Optimizado para HPC

**¡Happy Coding!** ⛪🚀
