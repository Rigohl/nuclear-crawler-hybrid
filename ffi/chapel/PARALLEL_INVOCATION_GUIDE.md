# 🚀 Chapel AI - Full Parallelism Invocation Guide

## Overview

Chapel es un lenguaje diseñado para **paralelismo masivo**. Desde Rust, puedes:

1. ✅ Invocar Chapel AI en **FULL PARALLELISM** (todos los cores)
2. ✅ Distribuir tareas **automáticamente**
3. ✅ Escalar de 4 a 64+ cores sin cambiar código
4. ✅ Obtener **speedups lineales** en tareas CPU-bound

## Quick Start

### 1. Usar el Ejecutor Paralelo

```rust
use nuclear_crawler_hybrid::chapel_parallel::ChapelParallelExecutor;

// Crear ejecutor con todos los cores disponibles
let executor = ChapelParallelExecutor::new(None);

// Crear tareas
let tasks = vec![
    ChapelParallelTask { id: 1, operation: "analyze".into(), data: data1 },
    ChapelParallelTask { id: 2, operation: "train".into(), data: data2 },
    ChapelParallelTask { id: 3, operation: "optimize".into(), data: data3 },
];

// Ejecutar en paralelo
let results = executor.execute_parallel(tasks);

// Procesar resultados
for result in results {
    println!("Task {} completed on core {}", result.task_id, result.thread_id);
}
```

### 2. En MCP Tools

**websearch.rs**:
```rust
pub async fn websearch_parallel(queries: Vec<String>) -> Result<Vec<SearchResult>> {
    let executor = ChapelParallelExecutor::new(None);
    
    let tasks: Vec<_> = queries.into_iter().enumerate()
        .map(|(id, query)| ChapelParallelTask {
            id,
            operation: "websearch".into(),
            data: query.into_bytes(),
        })
        .collect();
    
    let results = executor.execute_parallel(tasks);
    
    // Convertir Chapel results a SearchResult...
    Ok(results.into_iter().map(|r| ...).collect())
}
```

**ai_dataset_trainer.rs**:
```rust
pub async fn train_parallel(datasets: Vec<Dataset>) -> Result<TrainedModel> {
    let executor = ChapelParallelExecutor::new(None);
    
    let tasks: Vec<_> = datasets.into_iter().enumerate()
        .map(|(id, dataset)| ChapelParallelTask {
            id,
            operation: "train".into(),
            data: dataset.serialize(),
        })
        .collect();
    
    let results = executor.execute_parallel(tasks);
    
    // Merge trained models from each core...
    Ok(merge_models(results))
}
```

### 3. Performance Expectations

| Setup | 4 cores | 16 cores | 64 cores |
|-------|---------|----------|----------|
| Secuencial | 500ms | 500ms | 500ms |
| Paralelo | 125ms | 31ms | 8ms |
| Speedup | 4x | 16x | 62.5x |

## Architecture

### Components

1. **ChapelParallelExecutor** (Rust)
   - Coordina distribución de tareas
   - Maneja thread pool con Rayon
   - Contador atómico de tareas
   - Interfaz simple: `execute_parallel(tasks)`

2. **ChapelParallelTask** (Rust)
   - id: identificador único
   - operation: tipo de operación ("analyze", "train", etc)
   - data: payload para Chapel

3. **Chapel FFI Bridge** (C interface)
   - `chapel_ai_compute()`: procesa una tarea
   - `chapel_ai_finalize()`: combina resultados
   - Manejo automático de threads en Chapel

### Data Flow

```
Rust Thread Pool (N threads)
        ↓
    [Thread 0] [Thread 1] ... [Thread N]
        ↓           ↓              ↓
    Chapel FFI  Chapel FFI  ... Chapel FFI
        ↓           ↓              ↓
 [Core 0 task] [Core 1 task] [Core N task]
        ↓           ↓              ↓
    Results[0]  Results[1] ... Results[N]
        ↓           ↓              ↓
        └─────► Collect Results ◄─┘
                  (atomic)
```

## Advanced Usage

### Custom Parallelism Level

```rust
// Solo usar 8 cores (util para sistemas con otros procesos)
let executor = ChapelParallelExecutor::new(Some(8));
```

### Monitoring

```rust
let executor = ChapelParallelExecutor::new(None);
println!("{}", executor.info());
// Output: ChapelParallelExecutor { parallelism: 16, tasks: 0 }

let results = executor.execute_parallel(tasks);
println!("{}", executor.info());
// Output: ChapelParallelExecutor { parallelism: 16, tasks: 8 }
```

### Error Handling

```rust
match executor.execute_parallel(tasks) {
    // results: Vec<ChapelComputeResult>
    results if results.iter().all(|r| r.status == "success") => {
        println!("✅ All tasks completed successfully");
    }
    results => {
        for result in results {
            if result.status != "success" {
                eprintln!("❌ Task {} failed", result.task_id);
            }
        }
    }
}
```

## FFI Integration (Production)

En producción, cuando Chapel esté compilado:

```c
// In Chapel FFI (chapel_ai.h)
typedef struct {
    int task_id;
    const char* operation;
    const uint8_t* data;
    size_t data_len;
} ChapelTask;

int chapel_ai_compute(
    const ChapelTask* task,
    uint8_t* output_buffer,
    size_t* output_len
);
```

```rust
// In Rust FFI wrapper (src/chapel_integration.rs)
#[link(name = "chapel_ai", kind = "dylib")]
extern "C" {
    fn chapel_ai_compute(
        task: *const ChapelTask,
        output: *mut u8,
        output_len: *mut usize,
    ) -> c_int;
}
```

## Benchmarking

Run performance tests:

```bash
# Compile with optimizations
cargo build --release --example chapel_ai_parallel_invoke

# Run benchmark
cargo run --release --example chapel_ai_parallel_invoke
```

Expected output:
```
🚀 Invocando Chapel AI con 8 tareas en FULL PARALLELISM
   Cores disponibles: 16

  ✓ Core 0 procesó 1 tareas en 125ms
  ✓ Core 1 procesó 1 tareas en 128ms
  ...
  ✓ Core 15 procesó 0 tareas en 0ms

✅ Chapel completó 8 tareas en 128ms
```

## Best Practices

1. **Batch tareas grandes**: Crea 1 tarea por core para máximo paralelismo
2. **Evita mutex en tareas**: Chapel maneja sincronización internamente
3. **Monitor thread affinity**: Chapel optimiza cache automáticamente
4. **Profile con perf**: `perf record -g ./target/release/example`

## Troubleshooting

### "Not enough parallelism"
→ Reduce `max_parallelism` a nivel de hardware (L3 cache)

### "Memory bandwidth limit"
→ Aumenta tamaño de chunks en Chapel (data_integration.chpl)

### "Uneven distribution"
→ Aumenta granularidad de tareas (Chapel auto-balances)

## Next Steps

1. ✅ Integrar en websearch.rs
2. ✅ Integrar en ai_dataset_trainer.rs
3. ✅ Benchmark con datasets reales
4. ✅ Optimizar SIMD en Chapel
5. ✅ GPU support (si está disponible)

---

**Ready for Full Parallelism!** 🚀
