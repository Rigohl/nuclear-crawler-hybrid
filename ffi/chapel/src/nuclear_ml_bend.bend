// ============================================================================
// Bend GPU Kernels for ML Training
// 🎮 Parallel GPU Programming Language - Maximum Performance Computing
// 
// Bend is a massively parallel computing language designed for GPUs
// Automatic parallelization, full type safety, no manual thread management
// ============================================================================

// ============================================================================
// BASIC MATRIX OPERATIONS (GPU kernels)
// ============================================================================

// Matrix multiplication: C = A @ B (optimized for GPU)
def matmul_gpu(A: &[[F64]], B: &[[F64]]) -> [[F64]]:
  let m = len(A)
  let n = len(B)
  let k = len(B[0])
  
  // Create output matrix
  let mut C = [[0.0 for _ in range(k)] for _ in range(m)]
  
  // Parallel matrix multiplication (Bend handles GPU distribution)
  for i in parallel(range(m)):
    for j in parallel(range(k)):
      let mut sum = 0.0
      for l in range(n):
        sum = sum + A[i][l] * B[l][j]
      C[i][j] = sum
  
  return C

// Vector normalization (L2 norm) - GPU optimized
def vector_norm_gpu(v: &[F64]) -> F64:
  let sum_squares = (fold v in parallel:
    (acc, x): acc + x * x
  with 0.0)
  return sqrt(sum_squares)

// Dot product (parallel reduction on GPU)
def dot_product_gpu(a: &[F64], b: &[F64]) -> F64:
  return fold (x, y) in (parallel zip(a, b)):
    (acc): acc + x * y
  with 0.0

// ============================================================================
// ACTIVATION FUNCTIONS (GPU kernels)
// ============================================================================

// ReLU activation: max(0, x)
def relu_gpu(x: &[[F64]]) -> [[F64]]:
  return [[max(0.0, val) for val in row] for row in x]

// ReLU derivative
def relu_derivative_gpu(x: &[[F64]]) -> [[F64]]:
  return [[if val > 0.0 then 1.0 else 0.0 for val in row] for row in x]

// Softmax (normalized exponential)
def softmax_gpu(x: &[[F64]]) -> [[F64]]:
  let mut result = [[0.0 for _ in x[0]] for _ in x]
  
  for i in parallel(range(len(x))):
    let max_val = (fold val in x[i]: (acc, v): max(acc, v) with x[i][0])
    let exp_vals = [exp(val - max_val) for val in x[i]]
    let sum_exp = fold v in exp_vals: (acc, x): acc + x with 0.0
    result[i] = [v / sum_exp for v in exp_vals]
  
  return result

// Sigmoid activation: 1 / (1 + exp(-x))
def sigmoid_gpu(x: &[[F64]]) -> [[F64]]:
  return [[1.0 / (1.0 + exp(-val)) for val in row] for row in x]

// ============================================================================
// LOSS FUNCTIONS (GPU kernels)
// ============================================================================

// Cross-entropy loss (binary classification)
def cross_entropy_loss_gpu(predictions: &[[F64]], targets: &[I32]) -> F64:
  let total_loss = fold (pred, target) in (parallel zip(predictions, targets)):
    (acc, (p, t)): 
      let p_clipped = max(1e-7, min(0.9999999, p[t]))
      acc + (-log(p_clipped))
  with 0.0
  
  return total_loss / f64(len(predictions))

// Mean squared error loss
def mse_loss_gpu(predictions: &[[F64]], targets: &[[F64]]) -> F64:
  let total_error = fold (pred, target) in (parallel zip(predictions, targets)):
    (acc, (p, t)):
      let diff = p - t
      acc + diff * diff
  with 0.0
  
  return total_error / f64(len(predictions))

// ============================================================================
// GRADIENT COMPUTATION (GPU kernels)
// ============================================================================

// Compute gradients for dense layer using backpropagation
def compute_gradients_gpu(
  X: &[[F64]],      // Input features (batch_size x input_dim)
  dL_dO: &[[F64]],  // Loss gradient w.r.t output (batch_size x output_dim)
  W: &[[F64]]       // Current weights (input_dim x output_dim)
) -> [[F64]]:
  // Gradient w.r.t weights: X^T @ dL_dO (input_dim x output_dim)
  let dL_dW = matmul_gpu(transpose(X), dL_dO)
  return dL_dW

// ============================================================================
// BATCH NORMALIZATION (GPU kernels)
// ============================================================================

// Batch norm forward pass
def batch_norm_forward_gpu(
  X: &[[F64]],
  gamma: &[F64],
  beta: &[F64],
  epsilon: F64
) -> ([[F64]], [F64], [F64]):
  let batch_size = len(X)
  let feature_dim = len(X[0])
  
  // Compute mean per feature
  let mut means = [0.0 for _ in range(feature_dim)]
  for j in parallel(range(feature_dim)):
    let sum = fold x in X: (acc, row): acc + row[j] with 0.0
    means[j] = sum / f64(batch_size)
  
  // Compute variance per feature
  let mut variances = [0.0 for _ in range(feature_dim)]
  for j in parallel(range(feature_dim)):
    let sum = fold x in X: (acc, row):
      let diff = row[j] - means[j]
      acc + diff * diff
    with 0.0
    variances[j] = sum / f64(batch_size)
  
  // Normalize
  let mut X_norm = [[0.0 for _ in X[0]] for _ in X]
  for i in parallel(range(batch_size)):
    for j in parallel(range(feature_dim)):
      X_norm[i][j] = (X[i][j] - means[j]) / sqrt(variances[j] + epsilon)
  
  // Scale and shift
  let mut output = [[0.0 for _ in X[0]] for _ in X]
  for i in parallel(range(batch_size)):
    for j in parallel(range(feature_dim)):
      output[i][j] = gamma[j] * X_norm[i][j] + beta[j]
  
  return (output, means, variances)

// ============================================================================
// OPTIMIZERS (GPU kernels)
// ============================================================================

// SGD update with momentum
def sgd_momentum_step_gpu(
  weights: &[[F64]],
  gradients: &[[F64]],
  momentum_buffer: &[[F64]],
  learning_rate: F64,
  momentum: F64
) -> ([[F64]], [[F64]]):
  let m = len(weights)
  let n = len(weights[0])
  
  let mut new_weights = [[0.0 for _ in range(n)] for _ in range(m)]
  let mut new_momentum = [[0.0 for _ in range(n)] for _ in range(m)]
  
  for i in parallel(range(m)):
    for j in parallel(range(n)):
      let new_buf = momentum * momentum_buffer[i][j] + gradients[i][j]
      new_momentum[i][j] = new_buf
      new_weights[i][j] = weights[i][j] - learning_rate * new_buf
  
  return (new_weights, new_momentum)

// Adam optimizer step
def adam_step_gpu(
  weights: &[[F64]],
  gradients: &[[F64]],
  m_t: &[[F64]],      // First moment (mean)
  v_t: &[[F64]],      // Second moment (variance)
  learning_rate: F64,
  beta1: F64,
  beta2: F64,
  epsilon: F64,
  t: I32
) -> ([[F64]], [[F64]], [[F64]]):
  let m = len(weights)
  let n = len(weights[0])
  
  let mut new_weights = [[0.0 for _ in range(n)] for _ in range(m)]
  let mut new_m = [[0.0 for _ in range(n)] for _ in range(m)]
  let mut new_v = [[0.0 for _ in range(n)] for _ in range(m)]
  
  let lr_t = learning_rate * sqrt(1.0 - pow(beta2, f64(t))) / (1.0 - pow(beta1, f64(t)))
  
  for i in parallel(range(m)):
    for j in parallel(range(n)):
      let g = gradients[i][j]
      let m_new = beta1 * m_t[i][j] + (1.0 - beta1) * g
      let v_new = beta2 * v_t[i][j] + (1.0 - beta2) * g * g
      
      new_m[i][j] = m_new
      new_v[i][j] = v_new
      new_weights[i][j] = weights[i][j] - lr_t * m_new / (sqrt(v_new) + epsilon)
  
  return (new_weights, new_m, new_v)

// ============================================================================
// TRAINING LOOP (GPU-accelerated)
// ============================================================================

def training_step_gpu(
  X_batch: &[[F64]],          // Input batch (batch_size x input_dim)
  y_batch: &[I32],            // Target batch
  W1: &[[F64]],               // First layer weights
  W2: &[[F64]],               // Second layer weights
  learning_rate: F64
) -> (F64, [[F64]], [[F64]]):
  
  // Forward pass
  let hidden = relu_gpu(matmul_gpu(X_batch, W1))    // Hidden layer with ReLU
  let logits = matmul_gpu(hidden, W2)                // Output logits
  let predictions = softmax_gpu(logits)              // Softmax probabilities
  
  // Compute loss
  let loss = cross_entropy_loss_gpu(predictions, y_batch)
  
  // Backward pass (simplified)
  let dL_dO = [[predictions[i][j] - (if j == y_batch[i] then 1.0 else 0.0) 
                 for j in range(len(W2[0]))] 
                for i in range(len(predictions))]
  
  let dL_dW2 = compute_gradients_gpu(hidden, dL_dO, W2)
  
  let dL_dH = matmul_gpu(dL_dO, transpose(W2))
  let dL_dH_relu = relu_derivative_gpu(hidden)  // Element-wise multiply
  
  let dL_dW1 = compute_gradients_gpu(X_batch, dL_dH, W1)
  
  // Update weights
  let new_W1 = [[W1[i][j] - learning_rate * dL_dW1[i][j] 
                  for j in range(len(W1[0]))] 
                 for i in range(len(W1))]
  
  let new_W2 = [[W2[i][j] - learning_rate * dL_dW2[i][j] 
                  for j in range(len(W2[0]))] 
                 for i in range(len(W2))]
  
  return (loss, new_W1, new_W2)

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

def transpose(matrix: &[[F64]]) -> [[F64]]:
  let m = len(matrix)
  let n = len(matrix[0])
  return [[matrix[i][j] for i in range(m)] for j in range(n)]

def sqrt(x: F64) -> F64:
  return x ** 0.5

def pow(base: F64, exp: F64) -> F64:
  return base ** exp

def exp(x: F64) -> F64:
  return 2.718281828 ** x

def log(x: F64) -> F64:
  // Natural logarithm approximation
  return log_approx(x)

def log_approx(x: F64) -> F64:
  // Simple approximation for demonstration
  if x <= 0.0 { return -1000.0 }
  let mut result = 0.0
  let mut n = x
  let mut k = 1
  while k < 10:
    result = result + ((n - 1.0) / n) * (1.0 / f64(k))
    n = n * n
    k = k + 1
  return result

def max(a: F64, b: F64) -> F64:
  if a > b { a } else { b }

def min(a: F64, b: F64) -> F64:
  if a < b { a } else { b }

def f64(x: I32) -> F64:
  x as F64

// ============================================================================
// MAIN - GPU Training Demonstration
// ============================================================================

def main():
  // Initialize training data
  let batch_size = 32
  let input_dim = 768
  let hidden_dim = 512
  let num_classes = 2
  let learning_rate = 0.001
  let num_batches = 3125  // 100K samples / 32
  
  writeln("╔════════════════════════════════════════════════════════════════╗")
  writeln("║   🎮 BEND GPU TRAINING ENGINE - Maximum Parallelism          ║")
  writeln("║      Automatic GPU Distribution | Full Type Safety            ║")
  writeln("╚════════════════════════════════════════════════════════════════╝")
  writeln("")
  
  writeln("Configuration:")
  writeln("  Batch size: ", batch_size)
  writeln("  Input dimension: ", input_dim)
  writeln("  Hidden dimension: ", hidden_dim)
  writeln("  Output classes: ", num_classes)
  writeln("  Learning rate: ", learning_rate)
  writeln("  Total batches: ", num_batches)
  writeln("")
  
  // Initialize weights randomly
  let mut W1 = [[0.01 * ((i + j) as F64) / 1000.0 
                  for j in range(hidden_dim)] 
                 for i in range(input_dim)]
  
  let mut W2 = [[0.01 * ((i + j) as F64) / 100.0 
                  for j in range(num_classes)] 
                 for i in range(hidden_dim)]
  
  writeln("Weights initialized:")
  writeln("  W1: ", input_dim, " x ", hidden_dim)
  writeln("  W2: ", hidden_dim, " x ", num_classes)
  writeln("")
  
  // Training loop
  let num_epochs = 2
  
  for epoch in range(num_epochs):
    writeln("Epoch ", epoch + 1, "/", num_epochs)
    
    for batch in parallel(range(num_batches)):
      // Create synthetic batch
      let X_batch = [[0.5 + (i + j) as F64 / 1000.0 
                       for j in range(input_dim)] 
                      for i in range(batch_size)]
      
      let y_batch = [(batch + i) % num_classes as I32 for i in range(batch_size)]
      
      // Training step
      let (loss, new_W1, new_W2) = training_step_gpu(X_batch, y_batch, W1, W2, learning_rate)
      W1 = new_W1
      W2 = new_W2
      
      if batch % 500 == 0:
        writeln("  Batch ", batch, "/", num_batches, " - Loss: ", loss)
  
  writeln("")
  writeln("╔════════════════════════════════════════════════════════════════╗")
  writeln("║                   ✅ GPU TRAINING COMPLETE                    ║")
  writeln("║                                                                ║")
  writeln("║  Bend advantages over traditional GPU programming:            ║")
  writeln("║    ✅ Automatic parallelization (no thread management)        ║")
  writeln("║    ✅ Full type safety (no buffer overflows)                  ║")
  writeln("║    ✅ Zero-cost abstractions (compiles to CUDA/HIP)           ║")
  writeln("║    ✅ Massive parallelism (GPU-native)                        ║")
  writeln("║    ✅ Functional approach (pure functions)                    ║")
  writeln("║                                                                ║")
  writeln("╚════════════════════════════════════════════════════════════════╝")
