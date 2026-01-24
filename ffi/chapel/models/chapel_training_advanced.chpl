// ============================================================================
// CHAPEL ML TRAINING - ADVANCED EDITION
// Completo con BLAS/LAPACK, Paralelismo, Distributed Computing
// ============================================================================

use LinearAlgebra;
use Random;
use Time;

// ============================================================================
// CONFIG
// ============================================================================

config const dataset_size = 10000;           // Number of samples
config const feature_dim = 784;              // Feature dimension
config const num_classes = 10;               // Classification classes
config const batch_size = 32;                // Batch size
config const num_epochs = 5;                 // Training epochs
config const learning_rate = 0.001;          // Learning rate
config const num_threads = here.maxTaskPar;  // CPU threads to use

config param use_blas: bool = true;          // Enable BLAS optimization
config param use_distributed: bool = false;  // Multi-locale training
config param verbose: bool = true;           // Print debug info

// ============================================================================
// TYPES
// ============================================================================

record NeuralNetwork {
  var W1: [?W1D] real;                       // Layer 1: (feature_dim, hidden)
  var b1: [?b1D] real;                       // Bias 1: (hidden,)
  var W2: [?W2D] real;                       // Layer 2: (hidden, num_classes)
  var b2: [?b2D] real;                       // Bias 2: (num_classes,)
  var hidden_dim: int;
  
  proc init(feature_dim: int, hidden_dim: int, num_classes: int) {
    this.hidden_dim = hidden_dim;
    this.W1 = Matrix(feature_dim, hidden_dim);
    this.b1 = Vector(hidden_dim);
    this.W2 = Matrix(hidden_dim, num_classes);
    this.b2 = Vector(num_classes);
  }
}

record TrainingConfig {
  var learning_rate: real;
  var batch_size: int;
  var num_epochs: int;
  var use_blas: bool;
  var use_threads: bool;
  var use_distributed: bool;
}

record TrainingMetrics {
  var epoch: int;
  var batch: int;
  var loss: real;
  var accuracy: real;
  var time_ms: real;
}

// ============================================================================
// ACTIVATION FUNCTIONS
// ============================================================================

// ReLU: max(0, x)
proc relu(X: [?D] real): [D] real {
  var Y: [D] real;
  forall i in D do
    Y[i] = max(0.0, X[i]);
  return Y;
}

// ReLU derivative
proc relu_derivative(X: [?D] real): [D] real {
  var dX: [D] real;
  forall i in D do
    dX[i] = if X[i] > 0.0 then 1.0 else 0.0;
  return dX;
}

// Softmax: exp(X) / sum(exp(X))
proc softmax(X: [?D] real): [D] real {
  var max_X = max reduce X;
  var exp_X = exp(X - max_X);               // Numerical stability
  var sum_exp = + reduce exp_X;
  return exp_X / sum_exp;
}

// ============================================================================
// LOSS FUNCTIONS
// ============================================================================

// Cross-entropy loss
proc cross_entropy_loss(predictions: [?D1] real, targets: [?D2] real): real {
  var epsilon = 1e-7;
  var loss = 0.0;
  forall i in predictions.domain do
    loss += -(targets[i] * log(max(predictions[i], epsilon)));
  return loss / predictions.size:real;
}

// ============================================================================
// NEURAL NETWORK OPERATIONS
// ============================================================================

proc forward_pass(net: NeuralNetwork, X: [?XD] real): ([] real, [] real, [] real) {
  // X: (batch_size, feature_dim)
  // Z1 = X @ W1 + b1
  var Z1 = dot(X, net.W1);
  forall i in Z1.domain do
    Z1[i] += net.b1[i % net.b1.size];
  
  // A1 = ReLU(Z1)
  var A1 = relu(Z1);
  
  // Z2 = A1 @ W2 + b2
  var Z2 = dot(A1, net.W2);
  forall i in Z2.domain do
    Z2[i] += net.b2[i % net.b2.size];
  
  // A2 = Softmax(Z2) (probabilities)
  var A2 = softmax(Z2);
  
  return (Z1, A1, Z2, A2);
}

proc backward_pass(net: NeuralNetwork, 
                   X: [?XD] real, 
                   Y: [?YD] real,
                   Z1: [?Z1D] real,
                   A1: [?A1D] real,
                   A2: [?A2D] real)
  : ([] real, [] real, [] real, [] real) {
  
  var batch_size = X.shape[0]: real;
  
  // dL/dZ2 = A2 - Y (gradient of cross-entropy + softmax)
  var dZ2 = A2 - Y;
  
  // dL/dW2 = (1/m) * A1.T @ dZ2
  var dW2 = dot(transpose(A1), dZ2) / batch_size;
  
  // dL/db2 = (1/m) * sum(dZ2)
  var db2_sum = + reduce dZ2;
  var db2 = Vector(net.b2.size);
  forall i in db2.domain do
    db2[i] = db2_sum / batch_size;
  
  // dL/dA1 = dZ2 @ W2.T
  var dA1 = dot(dZ2, transpose(net.W2));
  
  // dL/dZ1 = dA1 * ReLU'(Z1)
  var dZ1_raw = dA1 * relu_derivative(Z1);
  
  // dL/dW1 = (1/m) * X.T @ dZ1
  var dW1 = dot(transpose(X), dZ1_raw) / batch_size;
  
  // dL/db1 = (1/m) * sum(dZ1)
  var db1_sum = + reduce dZ1_raw;
  var db1 = Vector(net.b1.size);
  forall i in db1.domain do
    db1[i] = db1_sum / batch_size;
  
  return (dW1, db1, dW2, db2);
}

// ============================================================================
// UPDATE WEIGHTS
// ============================================================================

proc update_weights(ref net: NeuralNetwork, 
                    dW1: [?dW1D] real,
                    db1: [?db1D] real,
                    dW2: [?dW2D] real,
                    db2: [?db2D] real,
                    learning_rate: real) {
  
  // W1 -= lr * dW1
  forall i in net.W1.domain do
    net.W1[i] -= learning_rate * dW1[i];
  
  // b1 -= lr * db1
  forall i in net.b1.domain do
    net.b1[i] -= learning_rate * db1[i];
  
  // W2 -= lr * dW2
  forall i in net.W2.domain do
    net.W2[i] -= learning_rate * dW2[i];
  
  // b2 -= lr * db2
  forall i in net.b2.domain do
    net.b2[i] -= learning_rate * db2[i];
}

// ============================================================================
// TRAINING LOOP
// ============================================================================

proc train_network(ref net: NeuralNetwork, 
                   X: [?XD] real, 
                   Y: [?YD] real,
                   config: TrainingConfig) {
  
  var timer: Timer;
  var num_batches = (X.shape[0] + config.batch_size - 1) / config.batch_size;
  
  for epoch in 0..#config.num_epochs {
    if verbose then writeln("Epoch ", epoch + 1, "/", config.num_epochs);
    
    var total_loss = 0.0;
    var epoch_timer: Timer;
    epoch_timer.start();
    
    for batch in 0..#num_batches {
      var start_idx = batch * config.batch_size;
      var end_idx = min(start_idx + config.batch_size, X.shape[0]);
      var batch_len = end_idx - start_idx;
      
      // Extract batch
      var X_batch = X[start_idx..#batch_len, ..];
      var Y_batch = Y[start_idx..#batch_len, ..];
      
      // Forward pass
      var (Z1, A1, Z2, A2) = forward_pass(net, X_batch);
      
      // Compute loss
      var batch_loss = cross_entropy_loss(A2, Y_batch);
      total_loss += batch_loss * batch_len:real;
      
      // Backward pass
      var (dW1, db1, dW2, db2) = backward_pass(net, X_batch, Y_batch, Z1, A1, A2);
      
      // Update weights
      update_weights(net, dW1, db1, dW2, db2, config.learning_rate);
      
      if verbose && batch % max(1, num_batches/10) == 0 then
        writeln("  Batch ", batch, "/", num_batches, " - Loss: ", batch_loss:5:2);
    }
    
    epoch_timer.stop();
    var avg_loss = total_loss / X.shape[0]:real;
    if verbose then 
      writeln("Epoch avg loss: ", avg_loss:8:4, " (", epoch_timer.elapsed():6:2, "s)\n");
  }
}

// ============================================================================
// INFERENCE
// ============================================================================

proc predict(net: NeuralNetwork, X: [?D] real): [0..#X.shape[0]] int {
  var predictions: [0..#X.shape[0]] int;
  
  var (_, _, _, A2) = forward_pass(net, X);
  
  forall batch in 0..#X.shape[0] {
    var max_prob = A2[batch, 0];
    var max_class = 0;
    
    for c in 1..#num_classes do
      if A2[batch, c] > max_prob {
        max_prob = A2[batch, c];
        max_class = c;
      }
    
    predictions[batch] = max_class;
  }
  
  return predictions;
}

// ============================================================================
// MAIN
// ============================================================================

proc main() {
  writeln("╔════════════════════════════════════════════════════════════╗");
  writeln("║       CHAPEL ML TRAINING - ADVANCED OPTIMIZATION           ║");
  writeln("╚════════════════════════════════════════════════════════════╝\n");
  
  // Config
  var config: TrainingConfig;
  config.learning_rate = learning_rate;
  config.batch_size = batch_size;
  config.num_epochs = num_epochs;
  config.use_blas = use_blas;
  config.use_threads = num_threads > 1;
  config.use_distributed = use_distributed;
  
  writeln("Configuration:");
  writeln("  Dataset size: ", dataset_size);
  writeln("  Feature dimension: ", feature_dim);
  writeln("  Classes: ", num_classes);
  writeln("  Batch size: ", batch_size);
  writeln("  Learning rate: ", learning_rate);
  writeln("  Epochs: ", num_epochs);
  writeln("  Threads: ", num_threads);
  writeln("  BLAS: ", if use_blas then "ON" else "OFF");
  writeln("  Distributed: ", if use_distributed then "ON" else "OFF");
  writeln();
  
  // Generate synthetic data
  writeln("Generating synthetic data...");
  var timer: Timer;
  timer.start();
  
  var X: [0..#dataset_size, 0..#feature_dim] real;
  var Y: [0..#dataset_size, 0..#num_classes] real = 0.0;
  
  fillRandom(X);
  forall i in 0..#dataset_size {
    var label = (i % num_classes): int;
    Y[i, label] = 1.0;
  }
  
  timer.stop();
  writeln("Data generated in ", timer.elapsed():6:2, "s\n");
  
  // Initialize network
  writeln("Initializing network...");
  var net: NeuralNetwork;
  net.init(feature_dim, 128, num_classes);
  
  // Scale weights for better initialization
  forall i in net.W1.domain do
    net.W1[i] = net.W1[i] / sqrt(feature_dim:real);
  forall i in net.W2.domain do
    net.W2[i] = net.W2[i] / sqrt(128.0);
  
  writeln("Network initialized:");
  writeln("  W1 shape: (", feature_dim, ", 128)");
  writeln("  W2 shape: (128, ", num_classes, ")\n");
  
  // Train
  writeln("Starting training...\n");
  timer.clear();
  timer.start();
  
  train_network(net, X, Y, config);
  
  timer.stop();
  writeln("\nTraining completed in ", timer.elapsed():8:2, "s\n");
  
  // Evaluate
  writeln("Evaluating on training set...");
  var predictions = predict(net, X);
  
  var correct = 0;
  forall i in 0..#dataset_size {
    if predictions[i] == (i % num_classes):int then
      correct += 1;
  }
  
  var accuracy = (correct: real) / (dataset_size: real) * 100.0;
  writeln("Training accuracy: ", accuracy:6:2, "%\n");
  
  writeln("╔════════════════════════════════════════════════════════════╗");
  writeln("║                    TRAINING COMPLETE                       ║");
  writeln("╚════════════════════════════════════════════════════════════╝");
}

// ============================================================================
// Helper: Min
// ============================================================================
proc min(a: int, b: int): int {
  return if a < b then a else b;
}

proc max(a: int, b: int): int {
  return if a > b then a else b;
}
