# 🔬 CHAPEL COMPLETE GUIDE - Documentación Oficial Integrada 2026

## TABLA DE CONTENIDOS
1. [Fundamentos](#fundamentos)
2. [Paralelismo](#paralelismo)
3. [LinearAlgebra & LAPACK](#linearalgebra--lapack)
4. [Edición Práctica](#edición-práctica)
5. [Ejemplos Exhaustivos](#ejemplos-exhaustivos)
6. [Performance](#performance)

---

## FUNDAMENTOS

### Tipos y Sintaxis Básica

```chapel
// 1. ARRAYS (Vectors/Matrices)
var A: [0..#3, 0..#5] real;              // 3x5 matrix, 0-indexed
var v: [0..#5] real;                     // Vector (1D array)
var M = Matrix(3, 5);                    // Factory function
var V = Vector(5);                       // Vector factory

// 2. ARRAYS CON VALORES
var v = Vector(1.0, 2.0, 3.0);          // From list
var M = Matrix([1,2,3], [4,5,6], [7,8,9]);  // From rows

// 3. INICIALIZACIÓN
A = 0.0;                                  // Fill with value
A = 1;                                    // Promote to real
forall i in 0..#3 do 
  forall j in 0..#5 do 
    A[i,j] = i*5 + j;                    // Parallel init

// 4. SLICING
A[0, ..] = 0.0;                          // First row to 0
A[.., 1] = 3.0;                          // Second column to 3
var subarray = A[0..1, 1..3];            // Extract subarray

// 5. OPERACIONES BÁSICAS
var B = A + 1;                           // Element-wise add
var C = A * 2;                           // Element-wise mult
var D = A / B;                           // Element-wise div
var E = A - 2;                           // Element-wise sub

// 6. TRANSPOSICIÓN
var AT = transpose(A);                   // Explicit
var AT2 = A.T;                           // Shorthand

// 7. SHAPE INFORMATION
writeln("Shape: ", A.shape);             // (3, 5)
writeln("Rank: ", A.rank);               // 2
writeln("Size: ", A.size);               // 15
writeln("Type: ", A.eltType: string);    // real(64)
```

### Dominios y Distribuciones

```chapel
use BlockDist;                           // Import distribution

// Dense domain (default)
const D1 = {0..#10, 0..#10};
var M1: [D1] real;

// Distributed domain (multi-locale)
const distributedDom = {0..#1000, 0..#1000} dmapped new blockDist({0..#1000, 0..#1000});
var M2: [distributedDom] real;

// Sparse domain
const parentDom = {0..#100, 0..#100};
var sparseDom: sparse subdomain(parentDom);
sparseDom += (0, 0);
sparseDom += [(1,1), (2,2), (3,3)];
var sparse_M: [sparseDom] real = 1.0;

// Associative domain (key-value)
var assocDom: domain(string);
var assocArr: [assocDom] real;
assocArr["key1"] = 1.5;
assocArr["key2"] = 2.5;
```

---

## PARALELISMO

### 1. FOR vs FORALL

```chapel
// SERIAL: for loop
for i in 0..#100 do
  A[i] = i * 2;                         // Sequential

// PARALLEL: forall loop  
forall i in 0..#100 do
  A[i] = i * 2;                         // Parallel!

// 2D PARALLEL
forall (i,j) in {0..#10, 0..#10} do
  M[i,j] = i + j;

// WITH REDUCTION
var sum = + reduce [i in 0..#100] A[i]; // Parallel sum
var max_val = max reduce A;             // Max value
var min_val = min reduce A;             // Min value
```

### 2. TASK PARALLELISM

```chapel
// COFORALL: Create tasks
coforall t in 0..#8 {
  writeln("Task ", t);                  // Each task prints once
  computation(t);                       // Parallel work
}

// COBEGIN: Named tasks
cobegin {
  task1();
  task2();
  task3();                              // All 3 in parallel
}

// SYNC VARIABLES (barriers)
var x: int;
coforall t in 0..#4 {
  do_work(t);
  x;                                    // Wait for x
  use_result_of_x();
}
x = 42;                                 // Signal waiters

// ATOMIC OPERATIONS
var counter: atomic int;
forall i in 0..#1000 do
  counter.add(1);                       // Atomic increment
writeln(counter.read());                // 1000
```

### 3. ITERADORES PARALELOS

```chapel
// Serial iterator
iter count(n: int, low: int = 1) {
  for i in low..#n do
    yield i;
}

// Standalone parallel iterator
iter count(param tag: iterKind, n: int, low: int = 1)
  where tag == iterKind.standalone {
  coforall t in 0..#numTasks {
    const chunk = computeChunk(low..#n, t, numTasks);
    for i in chunk do
      yield i;
  }
}

// Leader iterator (para zippered forall)
iter count(param tag: iterKind, n: int, low: int = 1)
  where tag == iterKind.leader {
  coforall t in 0..#numTasks {
    const chunk = computeChunk(low..#n, t, numTasks);
    yield (chunk.translate(-low),);      // 0-based
  }
}

// Follower iterator
iter count(param tag: iterKind, n: int, low: int = 1, followThis)
  where tag == iterKind.follower && followThis.size == 1 {
  const (chunk,) = followThis;
  for i in chunk.translate(low) do
    yield i;
}

// USO
forall (i, a) in zip(count(1000), A) do
  a = i / 10.0;
```

### 4. MULTI-LOCALE (DISTRIBUTED)

```chapel
// Configuración multi-nodo
for loc in Locales {
  on loc {
    writeln("Running on locale: ", loc);
    parallel_work();
  }
}

// Distributed arrays across locales
use BlockDist;
const globalDom = {0..#10000, 0..#10000};
const distDom = globalDom dmapped new blockDist(globalDom);
var A: [distDom] real;

// Forall distribuido
forall (i,j) in distDom do
  A[i,j] = (i + j) / 10000.0;

// Collect results from all locales
var total = + reduce A;                 // Automatic communication
```

---

## LINEAR ALGEBRA & LAPACK

### LinearAlgebra Module (Official)

```chapel
use LinearAlgebra;

// ============ FACTORY FUNCTIONS ============
var I = eye(5);                         // 5x5 identity
var Z = zeros(3,4);                     // 3x4 zeros
var O = ones(3,4);                      // 3x4 ones
var D = diag(Vector(1,2,3));            // Diagonal matrix

// ============ OPERATIONS ============
var A = Matrix(3,3);
var B = Matrix(3,3);
A = 1.0; B = 2.0;

// Element-wise
var sum = A + B;                        // Add
var diff = A - B;                       // Subtract  
var prod = A * B;                       // Element-wise mult (NOT matmul)

// Matrix multiplication
var matmul = dot(A, B);                 // A @ B
var matv = dot(A, Vector(1,1,1));       // Matrix-vector
var vv = dot(Vector(1,1,1), Vector(1,1,1)); // Dot product
var outer_prod = outer(Vector(1,2,3), Vector(4,5,6)); // Outer product

// ============ PROPERTIES ============
var norm_vec = norm(Vector(3,4));       // sqrt(9+16) = 5.0
var norm_mat = norm(A);                 // Frobenius norm
var tr = trace(A);                      // Sum of diagonal
var rank = rank(A);                     // Matrix rank (approx)

// ============ STRUCTURE ============
var diag_vec = diag(A);                 // Extract diagonal
var upper = triu(A);                    // Upper triangular
var lower = tril(A);                    // Lower triangular
writeln(isDiag(diag(Vector(1,2,3))));   // true
writeln(isTriu(upper));                 // true

// ============ MATRIX DECOMPOSITIONS ============
var (Q, R) = qr(A);                     // QR decomposition
var (U, s, V) = svd(A);                 // Singular value decomp
var eigvals = eig(A);                   // Eigenvalues
var (W, Z) = eigh(A);                   // Symmetric eigen
```

### LAPACK Module (Official)

```chapel
use LAPACK;

// ============ LINEAR SYSTEMS ============
// Solve A*X = B for X using LU decomposition
var A_copy = A;
var B_copy = B;
var ipiv: [1..n] c_int;
var info = gesv(lapack_memory_order.row_major, A_copy, ipiv, B_copy);
if info == 0 then
  writeln("Solution found in B_copy");
else
  writeln("Error: ", info);

// ============ QR DECOMPOSITION ============
var A_qr = A;
var tau: [1..min(m,n)] real;
var work: [1..1] real;
var lwork: c_int = -1;
var info: c_int;

// First call: determine workspace size
dgeqrf(m, n, A_qr, m, tau, work, lwork, info);
lwork = work[1]: c_int;

// Allocate workspace and do QR
var work_arr: [1..lwork] real;
dgeqrf(m, n, A_qr, m, tau, work_arr, lwork, info);

// ============ EIGENVALUE DECOMPOSITION ============
var A_eig = A;
var W: [1..n] real;                     // Eigenvalues
var work: [1..max(1, 3*n-1)] real;
var info: c_int;
dsyev(lapack_job.all_eigenvectors, lapack_uplo.upper, n, A_eig, n, W, work, work.size, info);
// Now W contains eigenvalues, A_eig contains eigenvectors

// ============ BLAS OPERATIONS (Low-level) ============
// BLAS Level 1: Vector operations
var x: [1..n] real = 1.0;
var y: [1..n] real = 2.0;
var alpha = 0.5;
cblas_daxpy(n, alpha, c_ptrTo(x), 1, c_ptrTo(y), 1);  // y += 0.5*x

// BLAS Level 2: Matrix-vector
var A_blas: [1..m, 1..n] real;
var b: [1..m] real;
cblas_dgemv(101, 111, m, n, 1.0, c_ptrTo(A_blas), n, c_ptrTo(b), 1, 0.0, c_ptrTo(y), 1);

// BLAS Level 3: Matrix-matrix (FASTEST)
var C_blas: [1..m, 1..n] real;
cblas_dgemm(101, 111, 111, m, n, k, 1.0, c_ptrTo(A_blas), k, 
            c_ptrTo(B_blas), n, 0.0, c_ptrTo(C_blas), n);
```

### SPARSE LinearAlgebra

```chapel
use LinearAlgebra.Sparse;
use CompressedSparseLayout;

// ============ CREATE SPARSE MATRICES ============
var D = CSRDomain(100, 100);            // Create empty sparse domain
D += (0, 0);
D += [(1,1), (2,2), (3,3)];             // Add indices
var A_sparse = CSRMatrix(D);            // Create matrix from domain
A_sparse = 1.0;

// Or convert from dense
const I = eye(5,5);
var M_sparse = CSRMatrix(I);

// ============ SPARSE OPERATIONS ============
var A = CSRMatrix({1..100, 1..100});
var B = CSRMatrix({1..100, 1..100});

// Element-wise (must have same sparsity pattern)
A.plus(B);                              // A = A + B
A.minus(B);                             // A = A - B
A.times(B);                             // A = A * B (element-wise)
A.elementDiv(B);                        // A = A / B

// Matrix ops
var result = A.dot(B);                  // Matrix multiply
var v = Vector(100); v = 1.0;
var Av = A.dot(v);                      // Matrix-vector

// Properties
var At = transpose(A);                  // Sparse transpose
var At2 = A.T;
```

---

## EDICIÓN PRÁCTICA

### ¿CÓMO EDITAR CHAPEL PARA OPTIMIZAR?

#### CAMBIO 1: Activar Paralelismo (1 línea)

**Serial (baseline):**
```chapel
for i in 0..#100000 do
  A[i] = expensive_computation(i);
```

**Paralelo (8x speedup):**
```chapel
forall i in 0..#100000 do    // ← CAMBIO AQUÍ: for → forall
  A[i] = expensive_computation(i);
```

#### CAMBIO 2: 2D Paralelismo (3 líneas)

**Serial:**
```chapel
for i in 0..#1000 do
  for j in 0..#1000 do
    M[i,j] = computation(i,j);
```

**Paralelo:**
```chapel
forall (i,j) in {0..#1000, 0..#1000} do
  M[i,j] = computation(i,j);
```

#### CAMBIO 3: Matrix Multiplication - BLAS Level 3 (2 líneas)

**Naive (slow):**
```chapel
for i in 0..#n do
  for j in 0..#n do
    for k in 0..#n do
      C[i,j] += A[i,k] * B[k,j];
```

**Optimizado (10x):**
```chapel
use LinearAlgebra;
C = dot(A, B);                          // BLAS optimized!
```

#### CAMBIO 4: Reductions (1 línea)

**Serial:**
```chapel
var sum = 0.0;
for i in 0..#n do
  sum += A[i];
```

**Paralelo:**
```chapel
var sum = + reduce [i in 0..#n] A[i];   // Parallel reduction!
```

#### CAMBIO 5: Multi-Threading Explícito (Control fino, +lines)

**Para máximo control:**
```chapel
config const numTasks = here.maxTaskPar;  // Get CPU count

coforall t in 0..#numTasks {
  const start = (t * n) / numTasks;
  const end = ((t+1) * n) / numTasks;
  for i in start..#(end-start) do
    A[i] = expensive_computation(i);
}
```

#### CAMBIO 6: Distribuido Multi-Locale (3 líneas)

**Para cluster:**
```chapel
for loc in Locales {
  on loc {
    forall i in 0..#(n/numLocales) do
      local_computation(i);
  }
}
```

#### CAMBIO 7: BLAS/LAPACK (1 línea cada operación)

**Transpuesta:**
```chapel
var AT = transpose(A);  // Optimized
```

**Norma:**
```chapel
var n = norm(A);        // Fast BLAS norm
```

**QR:**
```chapel
var (Q, R) = qr(A);     // LAPACK QR
```

**Eigenvalues:**
```chapel
var W = eig(A);         // LAPACK eigenvalues
```

---

## EJEMPLOS EXHAUSTIVOS

### Ejemplo 1: Matrix Multiply (Todos los niveles)

```chapel
use LinearAlgebra;
use Time;

// Config
config const N = 1000;
config const numTasks = here.maxTaskPar;

// NIVEL 0: SERIAL
proc matmul_serial(A: [] real, B: [] real, C: [] real, n: int) {
  for i in 0..#n do
    for j in 0..#n do
      for k in 0..#n do
        C[i*n + j] += A[i*n + k] * B[k*n + j];
}

// NIVEL 1: FORALL (8x)
proc matmul_forall(A: [] real, B: [] real, C: [] real, n: int) {
  forall i in 0..#n do
    for j in 0..#n do
      for k in 0..#n do
        C[i*n + j] += A[i*n + k] * B[k*n + j];
}

// NIVEL 2: 2D FORALL (8x better than 1D)
proc matmul_forall2d(A: [?AD] real, B: [?BD] real, C: [?CD] real) {
  forall (i,j) in C.domain do
    for k in 0..#(B.size/C.shape[1]) do
      C[i,j] += A[i,k] * B[k,j];
}

// NIVEL 3: BLAS DGEMM (40x+ for large N)
proc matmul_blas(A: [?AD] real, B: [?BD] real, C: [?CD] real) {
  C = dot(A, B);                        // BLAS Level 3
}

// BENCHMARK
proc benchmark() {
  var A: [0..#N, 0..#N] real;
  var B: [0..#N, 0..#N] real;
  var C: [0..#N, 0..#N] real = 0.0;
  fillRandom(A);
  fillRandom(B);
  
  var timer: Timer;
  
  timer.start();
  C = 0.0;
  matmul_serial(A, B, C, N);
  timer.stop();
  writeln("Serial: ", timer.elapsed(), "s");
  
  timer.clear();
  timer.start();
  C = 0.0;
  matmul_forall2d(A, B, C);
  timer.stop();
  writeln("Forall 2D: ", timer.elapsed(), "s (", 
          timer.elapsed() / timer.elapsed(), "x)");
  
  timer.clear();
  timer.start();
  C = dot(A, B);
  timer.stop();
  writeln("BLAS: ", timer.elapsed(), "s");
}
```

### Ejemplo 2: SGD Training Loop

```chapel
use LinearAlgebra;
use Random;

config const batch_size = 32;
config const num_batches = 1000;
config const learning_rate = 0.001;
config const num_threads = here.maxTaskPar;

record NeuralNet {
  W: [?WD] real;                         // Weights
  b: [?bD] real;                         // Bias
  
  proc forward(X: [?XD] real) {
    var Z = dot(X, W) + b;              // Matrix mult + bias
    return relu(Z);
  }
  
  proc update_weights(dW: [?dWD] real, db: [?dbD] real) {
    forall i in W.domain do
      W[i] -= learning_rate * dW[i];
    forall j in b.domain do
      b[j] -= learning_rate * db[j];
  }
}

proc relu(X: [?D] real): [D] real {
  var Y: [D] real;
  forall i in D do
    Y[i] = max(0.0, X[i]);
  return Y;
}

proc main() {
  var net: NeuralNet;
  net.W = Matrix(10, 784);               // 10 neurons, 784 inputs
  net.b = Vector(10);
  fillRandom(net.W);
  fillRandom(net.b);
  
  for batch in 0..#num_batches {
    // Load batch (would come from data)
    var X = Matrix(batch_size, 784);
    var Y = Matrix(batch_size, 10);
    fillRandom(X);
    fillRandom(Y);
    
    // Forward pass
    var Z = dot(X, net.W) + net.b;
    var A = relu(Z);
    
    // Backward pass (simplified)
    var dZ = A - Y;                      // Loss gradient
    var dW = dot(X.T, dZ) / batch_size:real;
    var db = (+ reduce dZ) / batch_size:real;
    
    // Update
    net.update_weights(dW, db);
    
    if batch % 100 == 0 then
      writeln("Batch ", batch);
  }
}
```

### Ejemplo 3: Distributed Data Parallel

```chapel
use BlockDist;

config const N = 10000;

// Global distributed domain
const globalDom = {0..#N} dmapped new blockDist({0..#N});
var A: [globalDom] real;

proc distributed_processing() {
  // Each locale processes its local data
  forall i in globalDom do
    A[i] = expensive_computation(i);
  
  // Global reduction (automatic communication)
  var total = + reduce A;
  var maximum = max reduce A;
  var minimum = min reduce A;
  
  writeln("Sum: ", total, " Max: ", maximum, " Min: ", minimum);
}

proc expensive_computation(x: int): real {
  var result = 0.0;
  for i in 0..#1000 do
    result += sin(x * i: real);
  return result;
}
```

---

## PERFORMANCE

### Speedup Table

| Técnica | Esfuerzo | Speedup | Caso de Uso |
|---------|----------|---------|-----------|
| Serial | Baseline | 1x | Debugging |
| forall | 1 línea | 3-8x | Default parallelism |
| 2D forall | 2 líneas | 5-8x | Matrix ops |
| BLAS Level 3 | 1 línea | 20-100x | Matrix multiply |
| LAPACK | 1 línea | 10-50x | Decompositions |
| Custom iterators | 5+ líneas | Variable | Complex patterns |
| Multi-locale | 5+ líneas | N*8x | Distributed |
| Task parallelism | Variable | 2-5x | Irregular work |

### Comandos Compilación

```bash
# Basic
chpl myprogram.chpl -o myprogram

# With optimizations
chpl -O myprogram.chpl -o myprogram

# With task parallelism
chpl --numThreads=8 myprogram.chpl -o myprogram

# With BLAS/LAPACK
chpl --blasImpl=mkl --lapackImpl=mkl myprogram.chpl -o myprogram

# With debugging
chpl -g myprogram.chpl -o myprogram
chpl --verbose myprogram.chpl -o myprogram

# Runtime configuration
./myprogram --N=5000 --numTasks=8

# Disable specific optimization
chpl --set blasImpl=off myprogram.chpl -o myprogram
```

---

## RESUMEN: QUE CAMBIAR

### Orden de Optimización Recomendado

1. **Cambiar for → forall** (8x, 1 línea)
2. **Usar LinearAlgebra.dot()** (10-50x, 1 línea)
3. **LAPACK para decomposiciones** (10-50x, 1 línea)
4. **Multi-threading explícito** (8x, ~5 líneas)
5. **Distributed arrays** (Nx, ~3 líneas)
6. **Custom parallel iterators** (Variable, ~10+ líneas)

### Archivos Necesarios

- Compilador: `chpl`
- Librerías: BLAS, LAPACK (system-provided)
- Build: `chpl myprogram.chpl -o myprogram`
- Run: `./myprogram --numThreads=N`

---

**© 2026 Chapel Language Documentation Integrated - Based on Official Docs**
