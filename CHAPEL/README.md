# 🚀 CHAPEL EDITORS - HPC Parallel Code Editors

Esta carpeta contiene las versiones **C** y **Chapel** del editor paralelo masivo de código.

## 📁 Archivos

| Archivo | Lenguaje | Descripción |
|---------|----------|-------------|
| `c_parallel_editor.c` | **C** | Editor paralelo usando pthreads |
| `chpl_parallel_editor.chpl` | **Chapel** | Editor paralelo usando Chapel HPC |

## ⚡ Diferencias Técnicas

### C Version (c_parallel_editor.c)
- **Paralelismo**: POSIX threads (pthreads)
- **Memoria**: Manual (malloc/free)
- **Compilación**: `gcc -pthread c_parallel_editor.c -o c_editor`
- **Ejecución**: `./c_editor <directorio> <busqueda> <reemplazo>`

### Chapel Version (chpl_parallel_editor.chpl)
- **Paralelismo**: Chapel nativo (coforall, forall)
- **Memoria**: Automática con gestión distribuida
- **Compilación**: `chpl chpl_parallel_editor.chpl -o chpl_editor`
- **Ejecución**: `./chpl_editor <directorio> <busqueda> <reemplazo>`

## 🎯 Características Comunes

- ✅ **Procesamiento paralelo masivo**
- ✅ **Búsqueda y reemplazo con regex**
- ✅ **Load balancing automático**
- ✅ **Reportes detallados de rendimiento**
- ✅ **Modo dry-run para preview**
- ✅ **Integración con Docker**

## 🏗️ Arquitectura

Ambas versiones implementan el mismo algoritmo:

1. **Escaneo recursivo** de directorios
2. **Distribución automática** de archivos entre threads/tasks
3. **Procesamiento paralelo** con regex
4. **Sincronización** de resultados
5. **Reportes** de rendimiento y estadísticas

## 🚀 Uso

### Compilar C Version
```bash
cd CHAPEL
gcc -pthread c_parallel_editor.c -o c_parallel_editor
./c_parallel_editor . "old_text" "new_text"
```

### Compilar Chapel Version (requiere Docker)
```bash
cd CHAPEL
# Usar el script de compilación Docker
../build_chapel_docker.ps1
```

## 📊 Benchmarks

| Versión | Lenguaje | Paralelismo | Memoria |
|---------|----------|-------------|---------|
| C | POSIX Threads | Manual | Manual |
| Chapel | Chapel HPC | Automático | Distribuida |

## 💡 ¿Por qué dos versiones?

- **C**: Para sistemas sin Chapel instalado
- **Chapel**: Para máximo rendimiento HPC en supercomputadoras

---

**Built with 🔥 Nuclear Power**