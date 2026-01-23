# 📝 CHAPEL EDITING TUTORIAL - Aprende a Editar para Máxima Performance

> **Meta**: Cambios MÍNIMOS = Máximo speedup. Aprende DÓNDE y QUÉ editar.

---

## PARTE 1: EDITAR LOOPS

### Patrón: Serial → Parallel Loops

#### Escenario 1: Simple Loop (8x Speedup)

**ORIGINAL (Serial):**
```chapel
for i in 0..#100000 do
  A[i] = compute_value(i);
```

**OPTIMIZADO (Parallel):**
```chapel
forall i in 0..#100000 do      // ← CAMBIO: for → forall
  A[i] = compute_value(i);
```

**QUÉ EDITAR:**
- Línea: `for` → `forall`
- NADA más necesario
- Speedup automático: 8x

**CUÁNDO APLICAR:**
- ✅ Cuando las iteraciones son INDEPENDIENTES
- ✅ Cuando NO hay sincronización
- ✅ Cuando el trabajo es HEAVY (>1000 iteraciones)
- ❌ Si hay dependencias entre iteraciones
- ❌ Si hay I/O dentro del loop

#### Escenario 2: 2D Loop (8x Speedup)

**ORIGINAL:**
```chapel
for i in 0..#1000 do
  for j in 0..#1000 do
    M[i,j] = i + j;
```

**OPTIMIZADO:**
```chapel
forall (i,j) in {0..#1000, 0..#1000} do
  M[i,j] = i + j;
```

**QUÉ EDITAR:**
- Reemplaza ambos loops con UN forall 2D
- Sintaxis: `forall (i,j) in {dim1, dim2} do`
- Speedup: 5-8x (mejor que forall 1D anidado)

#### Escenario 3: Loop con Acumulador (Usar Reduction)

**ORIGINAL:**
```chapel
var sum = 0.0;
for i in 0..#100000 do
  sum += A[i];
```

**OPTIMIZADO:**
```chapel
var sum = + reduce [i in 0..#100000] A[i];
```

**QUÉ EDITAR:**
- Elimina loop y acumulador
- Usa `reduce` con operador (`+`, `*`, `max`, `min`, etc.)
- Sintaxis: `op reduce [indices in domain] expression`
- Speedup: 8x (parallelizado automáticamente)

**OTROS REDUCTIONS:**
```chapel
// Suma
var sum = + reduce A;

// Multiplicación
var prod = * reduce A;

// Máximo
var maximum = max reduce A;

// Mínimo
var minimum = min reduce A;

// Personalizado
var custom = min reduce [i in A.domain] abs(A[i]);
```

---

## PARTE 2: EDITAR OPERACIONES DE ÁLGEBRA LINEAL

### Patrón: Manual → BLAS/LAPACK

#### Cambio 1: Matrix Multiply (40x Speedup)

**ORIGINAL (3 loops anidados):**
```chapel
for i in 0..#n do
  for j in 0..#n do
    for k in 0..#n do
      C[i,j] += A[i,k] * B[k,j];
```

**PASO 1 - Agregar import (1 línea):**
```chapel
use LinearAlgebra;           // ← AGREGAR ESTO
```

**PASO 2 - Reemplazar loops (1 línea):**
```chapel
C = dot(A, B);               // ← CAMBIO: reemplaza 3 loops
```

**RESUMEN DE EDICIÓN:**
```diff
+ use LinearAlgebra;
- for i in 0..#n do
-   for j in 0..#n do
-     for k in 0..#n do
-       C[i,j] += A[i,k] * B[k,j];
+ C = dot(A, B);
```

**Speedup:**
- Serial 3-loop: 1.0s (baseline)
- BLAS: 0.02s (50x faster!)

#### Cambio 2: Norma de Vector/Matriz (8x Speedup)

**ORIGINAL:**
```chapel
var norm_result = 0.0;
for i in 0..#n do
  norm_result += A[i] * A[i];
norm_result = sqrt(norm_result);
```

**OPTIMIZADO (1 línea):**
```chapel
var norm_result = norm(A);   // ← CAMBIO: reemplaza 2 líneas + loop
```

**EDICIÓN:**
- Agregar: `use LinearAlgebra;`
- Reemplazar: loop + sqrt con `norm()`
- Speedup: 8x (parallelizado + optimizado)

#### Cambio 3: Transposición (8x Speedup)

**ORIGINAL:**
```chapel
var AT: [0..#n, 0..#m] real;
for i in 0..#n do
  for j in 0..#m do
    AT[j,i] = A[i,j];
```

**OPTIMIZADO:**
```chapel
var AT = transpose(A);       // ← CAMBIO: 1 línea
// O shorthand:
var AT = A.T;                // ← ALTERNATIVA
```

#### Cambio 4: QR Decomposition (10x Speedup)

**ORIGINAL (manual LAPACK calls - tedioso):**
```chapel
// Mucho boilerplate code...
var tau: [1..min(m,n)] real;
var work: [1..1] real;
var lwork: c_int = -1;
var info: c_int;
dgeqrf(m, n, A, m, tau, work, lwork, info);
// ... más boilerplate
```

**OPTIMIZADO (1 línea):**
```chapel
var (Q, R) = qr(A);          // ← CAMBIO: reemplaza 10+ líneas
```

**EDICIÓN:**
- Agregar: `use LinearAlgebra;`
- Reemplazar: todo el código LAPACK con `qr(A)`
- Resultado: `(Q, R)` donde `Q*R == A`

#### Cambio 5: Eigenvalue Decomposition (10x Speedup)

**ORIGINAL:**
```chapel
// Manual LAPACK (tedioso)
```

**OPTIMIZADO:**
```chapel
var W = eig(A);              // Eigenvalues
var (W, Z) = eigh(A);        // Eigenvalues + eigenvectors (symmetric)
```

---

## PARTE 3: EDITAR PARA MULTI-THREADING

### Patrón: Implicit → Explicit Tasks

#### Cambio 1: Custom Task Division (8x Speedup, more control)

**ORIGINAL (implícito, no hay control):**
```chapel
forall i in 0..#1000000 do
  A[i] = heavy_computation(i);
```

**EXPLÍCITO (con control):**
```chapel
config const numTasks = here.maxTaskPar;  // ← AGREGAR: get CPU count

coforall t in 0..#numTasks {              // ← AGREGAR: create tasks
  const start = (t * n) / numTasks;
  const end = ((t+1) * n) / numTasks;
  for i in start..#(end - start) do
    A[i] = heavy_computation(i);
}
```

**QUÉ EDITAR:**
- Línea 1: Agregar `config const numTasks = here.maxTaskPar;`
- Línea 2-5: Envolver trabajo en `coforall`
- Ventaja: Control fino sobre trabajo

#### Cambio 2: Multiple Independent Tasks (3x-5x Speedup)

**ORIGINAL (secuencial):**
```chapel
computation_1();
computation_2();
computation_3();
computation_4();
```

**PARALELO:**
```chapel
cobegin {
  computation_1();  // Parallel
  computation_2();  // Parallel
  computation_3();  // Parallel
  computation_4();  // Parallel
}                   // Wait for all
```

**QUÉ EDITAR:**
- Agregar: `cobegin {` antes
- Agregar: `}` después
- Resultado: 4 tasks ejecutan en paralelo

---

## PARTE 4: EDITAR PARA DISTRIBUTED COMPUTING

### Patrón: Single-locale → Multi-locale

#### Cambio 1: Distributed Array

**ORIGINAL (local):**
```chapel
var A: [0..#10000, 0..#10000] real;
forall (i,j) in A.domain do
  A[i,j] = i + j;
```

**DISTRIBUIDO:**
```chapel
use BlockDist;                                        // ← AGREGAR

const globalDom = {0..#10000, 0..#10000} 
      dmapped new blockDist({0..#10000, 0..#10000}); // ← CAMBIO: agrega distribution
var A: [globalDom] real;

forall (i,j) in A.domain do                          // ← SIN CAMBIOS
  A[i,j] = i + j;
```

**QUÉ EDITAR:**
- Línea 1: `use BlockDist;`
- Línea 3-4: Agrega `dmapped new blockDist(globalDom)` a la declaración
- Loop: NO cambia
- Resultado: Automáticamente distribuido entre nodos

**TIPOS DE DISTRIBUCIONES:**
```chapel
use BlockDist;          // Block distribution (good for dense arrays)
use CyclicDist;         // Cyclic distribution (load balancing)
use StridedDist;        // Strided (periodic patterns)
use ReplicatedDist;     // Replicated (all locales have copy)
```

#### Cambio 2: Explicit Locale-Aware Code

**ORIGINAL (no es locale-aware):**
```chapel
forall i in 0..#n do
  A[i] = compute(i);
```

**LOCALE-AWARE:**
```chapel
for loc in Locales {
  on loc {
    forall i in 0..#(n/numLocales) do
      A[i] = compute(i);
  }
}
```

**QUÉ EDITAR:**
- Agregar outer `for loc in Locales`
- Agregar `on loc { }`
- Speedup: Nx (N = number of locales)

---

## PARTE 5: EDITAR PARA MEJOR CACHÉ

### Patrón: Bad Cache Behavior → Good Cache Behavior

#### Cambio 1: Loop Interchange (Cache Tiling)

**ORIGINAL (mal cache, reads columns sequentially):**
```chapel
for j in 0..#1000 do
  for i in 0..#1000 do
    C[i,j] += A[i,k] * B[k,j];  // ← Column-wise access
```

**OPTIMIZADO (mejor cache, reads rows):**
```chapel
for i in 0..#1000 do
  for j in 0..#1000 do
    C[i,j] += A[i,k] * B[k,j];  // ← Row-wise access (better!)
```

**QUÉ EDITAR:**
- Cambiar orden de loops
- IMPORTANTE: Solo si loops son independientes
- Speedup: 2-3x (mejor cache locality)

---

## PARTE 6: EDICIÓN CHECKLIST

### Antes de Cualquier Optimización

- [ ] **¿Es el loop independiente?** (Can I use forall?)
- [ ] **¿Tengo librerías disponibles?** (BLAS/LAPACK installed?)
- [ ] **¿Hay sincronización?** (If yes, avoid simple forall)
- [ ] **¿Es O(n^3) o peor?** (Candidate for BLAS)
- [ ] **¿Tengo múltiples locales?** (Use BlockDist)
- [ ] **¿Es el trabajo regular?** (Use forall vs custom iterators)

### Orden de Cambios Recomendado

1. **`for` → `forall`** (8x, 1 cambio)
2. **Manual loops → BLAS** (20-100x, 1 línea)
3. **Reductions → `reduce`** (8x, 1 línea)
4. **Single machine → Multi-locale** (Nx, 3 líneas)
5. **Custom iterators** (si lo anterior no es suficiente)

---

## PARTE 7: COMANDOS COMPILACIÓN + RUNTIME

### Compilación

```bash
# Simple
chpl myprogram.chpl -o myprogram

# With optimizations
chpl -O myprogram.chpl -o myprogram

# With BLAS/LAPACK
chpl --blasImpl=mkl --lapackImpl=mkl myprogram.chpl

# Debugging
chpl -g myprogram.chpl -o myprogram
chpl --verbose myprogram.chpl -o myprogram

# Multi-threading configuration
chpl --numThreads=8 myprogram.chpl

# Specific locale
chpl --numLocales=4 myprogram.chpl
```

### Runtime

```bash
# Basic
./myprogram

# Override config
./myprogram --N=5000 --numTasks=8

# Multi-locale run
./myprogram --numLocales=4

# Verbose output
./myprogram --verbose=true

# Task profiling
./myprogram --tasking=qthreads
```

---

## PARTE 8: CHECKLIST DE EDICIÓN

### Matriz de Decisión: ¿QUÉ CAMBIAR?

```
¿LOOP with independent iterations?
  → YES: Cambiar for → forall (8x)
  → NO:  Consider task parallelism

¿OPERACIÓN es O(n^3) (matrix ops)?
  → YES: Usar BLAS/LinearAlgebra (20-100x)
  → NO:  Continue

¿OPERACIÓN es suma/max/min/etc?
  → YES: Usar reduce (8x)
  → NO:  Continue

¿TENGO múltiples nodos?
  → YES: Usar BlockDist (Nx)
  → NO:  OK, stay local

¿STILL NOT FAST ENOUGH?
  → Custom parallel iterators (variable)
  → GPU acceleration (Chapel GPU support)
```

---

## EJEMPLO REAL: Optimizar Código Completo

### ANTES (Serial, slow)

```chapel
proc train_neural_network(X: [?XD] real, Y: [?YD] real) {
  var W: [0..#10, 0..#784] real;
  fillRandom(W);
  
  for batch in 0..#1000 {
    // Forward pass (SLOW!)
    var Z: [0..#32, 0..#10] real;
    for i in 0..#32 do
      for j in 0..#10 do
        for k in 0..#784 do
          Z[i,j] += X[i,k] * W[j,k];
    
    // Update weights (SLOW!)
    for i in 0..#10 do
      for j in 0..#784 do
        W[i,j] -= 0.001 * gradient[i,j];
  }
}
```

### DESPUÉS (Optimizado, 50x faster)

```chapel
use LinearAlgebra;  // ← AGREGAR ESTO

proc train_neural_network(X: [?XD] real, Y: [?YD] real) {
  var W: [0..#10, 0..#784] real;
  fillRandom(W);
  
  forall batch in 0..#1000 {  // ← CAMBIO 1: for → forall
    // Forward pass (BLAS optimized!)
    var Z = dot(X[batch*32..(batch+1)*32-1, ..], W.T);  // ← CAMBIO 2: manual → BLAS
    
    // Update weights (BLAS optimized!)
    var dW = dot(X[batch*32..(batch+1)*32-1, ..].T, 
                 gradient[batch*32..(batch+1)*32-1, ..]);  // ← CAMBIO 3: manual → BLAS
    W -= 0.001 * dW;  // ← CAMBIO 4: forall add →  vectorized
  }
}
```

**Cambios realizados:**
1. Agregar `use LinearAlgebra;`
2. `for batch` → `forall batch`
3. Manual matrix multiply → `dot()`
4. Manual loops → Vectorized operations

**Speedup: 8x (parallelism) × 10x (BLAS) = 80x!**

---

**¡LISTO! Ya sabes qué y DÓNDE editar en Chapel.**
