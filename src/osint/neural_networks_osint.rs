// ============================================================================
// A: NEURAL NETWORKS FOR OSINT - RUST Implementation
// Deep learning for user classification, bot detection, authorship attribution
// ============================================================================

use std::f64;

/// ============================================================================
/// LAYER TRAIT - Generic neural layer
/// ============================================================================
pub trait NeuralLayer {
    fn forward(&mut self, input: &[f64]) -> Vec<f64>;
    fn backward(&mut self, grad_output: &[f64]) -> Vec<f64>;
    fn update_weights(&mut self, learning_rate: f64);
}

/// ============================================================================
/// LINEAR LAYER
/// ============================================================================
#[derive(Clone)]
pub struct LinearLayer {
    pub weights: Vec<Vec<f64>>,
    pub bias: Vec<f64>,
    pub input_cache: Vec<f64>,
    pub grad_weights: Vec<Vec<f64>>,
    pub grad_bias: Vec<f64>,
}

impl LinearLayer {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        let mut weights = vec![vec![0.0; input_size]; output_size];
        let mut bias = vec![0.0; output_size];

        // Xavier initialization
        let scale = (2.0 / (input_size + output_size) as f64).sqrt();
        for i in 0..output_size {
            for j in 0..input_size {
                weights[i][j] = (rand::random::<f64>() - 0.5) * scale;
            }
            bias[i] = 0.0;
        }

        LinearLayer {
            weights,
            bias,
            input_cache: vec![],
            grad_weights: vec![vec![0.0; input_size]; output_size],
            grad_bias: vec![0.0; output_size],
        }
    }

    pub fn forward_impl(&mut self, input: &[f64]) -> Vec<f64> {
        self.input_cache = input.to_vec();
        let output_size = self.weights.len();
        let mut output = vec![0.0; output_size];

        for i in 0..output_size {
            let mut sum = self.bias[i];
            for j in 0..input.len() {
                sum += self.weights[i][j] * input[j];
            }
            output[i] = sum;
        }

        output
    }

    pub fn backward_impl(&mut self, grad_output: &[f64]) -> Vec<f64> {
        let input_size = self.input_cache.len();
        let output_size = self.weights.len();
        let mut grad_input = vec![0.0; input_size];

        // Ensure grad_output has the correct size
        let grad_len = grad_output.len().min(output_size);

        // Compute gradients
        for i in 0..grad_len {
            self.grad_bias[i] += grad_output[i];
            for j in 0..input_size {
                self.grad_weights[i][j] += grad_output[i] * self.input_cache[j];
                grad_input[j] += grad_output[i] * self.weights[i][j];
            }
        }

        grad_input
    }
}

impl NeuralLayer for LinearLayer {
    fn forward(&mut self, input: &[f64]) -> Vec<f64> {
        self.forward_impl(input)
    }

    fn backward(&mut self, grad_output: &[f64]) -> Vec<f64> {
        self.backward_impl(grad_output)
    }

    fn update_weights(&mut self, learning_rate: f64) {
        for i in 0..self.weights.len() {
            self.bias[i] -= learning_rate * self.grad_bias[i];
            for j in 0..self.weights[i].len() {
                self.weights[i][j] -= learning_rate * self.grad_weights[i][j];
            }
            self.grad_bias[i] = 0.0;
            for j in 0..self.weights[i].len() {
                self.grad_weights[i][j] = 0.0;
            }
        }
    }
}

/// ============================================================================
/// ACTIVATION FUNCTIONS
/// ============================================================================
pub fn relu(x: f64) -> f64 {
    x.max(0.0)
}

pub fn relu_derivative(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else {
        0.0
    }
}

pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub fn sigmoid_derivative(x: f64) -> f64 {
    let s = sigmoid(x);
    s * (1.0 - s)
}

pub fn softmax(x: &[f64]) -> Vec<f64> {
    let max = x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exp_sum: f64 = x.iter().map(|v| (v - max).exp()).sum();
    x.iter().map(|v| (v - max).exp() / exp_sum).collect()
}

pub fn tanh_activation(x: f64) -> f64 {
    x.tanh()
}

/// ============================================================================
/// NEURAL NETWORK FOR OSINT
/// ============================================================================
pub struct OSINTNeuralNetwork {
    layers: Vec<LinearLayer>,
    activations: Vec<String>, // "relu", "sigmoid", "tanh"
    learning_rate: f64,
}

impl OSINTNeuralNetwork {
    pub fn new(layer_sizes: &[usize], activations: Vec<String>, learning_rate: f64) -> Self {
        let mut layers = Vec::new();
        for i in 0..layer_sizes.len() - 1 {
            layers.push(LinearLayer::new(layer_sizes[i], layer_sizes[i + 1]));
        }

        OSINTNeuralNetwork {
            layers,
            activations,
            learning_rate,
        }
    }

    /// Forward pass with activation functions
    pub fn forward(&mut self, input: &[f64]) -> Vec<f64> {
        let mut current = input.to_vec();

        for (i, layer) in self.layers.iter_mut().enumerate() {
            current = layer.forward(&current);

            // Apply activation
            let activation = &self.activations[i];
            current = match activation.as_str() {
                "relu" => current.iter().map(|&x| relu(x)).collect(),
                "sigmoid" => current.iter().map(|&x| sigmoid(x)).collect(),
                "tanh" => current.iter().map(|&x| tanh_activation(x)).collect(),
                "softmax" => softmax(&current),
                _ => current,
            };
        }

        current
    }

    /// Backward pass
    pub fn backward(&mut self, grad_output: &[f64]) -> f64 {
        let mut grad = grad_output.to_vec();

        // Backpropagate through layers in reverse
        for i in (0..self.layers.len()).rev() {
            let activation = &self.activations[i].clone();

            // Apply activation derivative
            grad = match activation.as_str() {
                "relu" => grad
                    .iter()
                    .zip(self.layers[i].input_cache.iter())
                    .map(|(&g, &x)| g * relu_derivative(x))
                    .collect(),
                "sigmoid" => grad
                    .iter()
                    .zip(self.layers[i].input_cache.iter())
                    .map(|(&g, &x)| g * sigmoid_derivative(x))
                    .collect(),
                _ => grad,
            };

            grad = self.layers[i].backward(&grad);
        }

        // Update weights
        for layer in self.layers.iter_mut() {
            layer.update_weights(self.learning_rate);
        }

        // Return average loss
        grad.iter().map(|g| g * g).sum::<f64>() / grad.len() as f64
    }

    /// Train on batch
    pub fn train_batch(
        &mut self,
        x_batch: &[Vec<f64>],
        y_batch: &[Vec<f64>],
        epochs: usize,
    ) -> f64 {
        let mut total_loss = 0.0;

        for _ in 0..epochs {
            for (x, y) in x_batch.iter().zip(y_batch.iter()) {
                let output = self.forward(x);

                // Compute loss gradient (MSE)
                let grad: Vec<f64> = output
                    .iter()
                    .zip(y.iter())
                    .map(|(&o, &t)| 2.0 * (o - t))
                    .collect();

                total_loss += self.backward(&grad);
            }
        }

        total_loss / (x_batch.len() as f64 * epochs as f64)
    }

    /// Predict
    pub fn predict(&mut self, input: &[f64]) -> Vec<f64> {
        self.forward(input)
    }
}

/// ============================================================================
/// SPECIALIZED: BOT CLASSIFIER NEURAL NETWORK
/// ============================================================================
pub struct BotClassifierNN {
    network: OSINTNeuralNetwork,
}

impl BotClassifierNN {
    /// Create bot classifier: 20D input → [hidden1:64, hidden2:32] → 2D output (human/bot)
    pub fn new() -> Self {
        let layer_sizes = vec![20, 64, 32, 2];
        let activations = vec![
            "relu".to_string(),
            "relu".to_string(),
            "sigmoid".to_string(),
        ];
        let network = OSINTNeuralNetwork::new(&layer_sizes, activations, 0.01);

        BotClassifierNN { network }
    }

    pub fn train(&mut self, features: &[Vec<f64>], labels: &[Vec<f64>]) -> f64 {
        self.network.train_batch(features, labels, 10)
    }

    pub fn predict_bot_probability(&mut self, features: &[f64]) -> f64 {
        let output = self.network.predict(features);
        output[1] // Probability of being a bot
    }
}

/// ============================================================================
/// SPECIALIZED: AUTHORSHIP ATTRIBUTION NETWORK
/// ============================================================================
pub struct AuthorshipNN {
    network: OSINTNeuralNetwork,
}

impl AuthorshipNN {
    /// Create authorship network: 5D linguistic profile → embedding space
    pub fn new() -> Self {
        let layer_sizes = vec![5, 16, 8, 4];
        let activations = vec!["relu".to_string(), "relu".to_string(), "tanh".to_string()];
        let network = OSINTNeuralNetwork::new(&layer_sizes, activations, 0.01);

        AuthorshipNN { network }
    }

    pub fn get_embedding(&mut self, linguistic_profile: &[f64]) -> Vec<f64> {
        self.network.predict(linguistic_profile)
    }

    pub fn similarity(&mut self, profile1: &[f64], profile2: &[f64]) -> f64 {
        let emb1 = self.get_embedding(profile1);
        let emb2 = self.get_embedding(profile2);

        // Cosine similarity
        let dot: f64 = emb1.iter().zip(emb2.iter()).map(|(a, b)| a * b).sum();
        let norm1 = emb1.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm2 = emb2.iter().map(|x| x * x).sum::<f64>().sqrt();

        dot / (norm1 * norm2 + 1e-10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmoid_derivative() {
        let epsilon = 1e-6;

        let sd_0 = sigmoid_derivative(0.0);
        assert!((sd_0 - 0.25).abs() < epsilon);

        let s_1 = sigmoid(1.0);
        let sd_1 = sigmoid_derivative(1.0);
        assert!((sd_1 - (s_1 * (1.0 - s_1))).abs() < epsilon);

        let s_m1 = sigmoid(-1.0);
        let sd_m1 = sigmoid_derivative(-1.0);
        assert!((sd_m1 - (s_m1 * (1.0 - s_m1))).abs() < epsilon);

        let sd_10 = sigmoid_derivative(10.0);
        assert!(sd_10 < 1e-4);

        let sd_m10 = sigmoid_derivative(-10.0);
        assert!(sd_m10 < 1e-4);
    }

    #[test]
    fn test_linear_layer() {
        let mut layer = LinearLayer::new(3, 2);
        let input = vec![1.0, 2.0, 3.0];
        let output = layer.forward(&input);

        assert_eq!(output.len(), 2);
    }

    #[test]
    fn test_bot_classifier() {
        let mut classifier = BotClassifierNN::new();
        let features = vec![vec![
            100.0, 5000.0, 10.0, 0.2, 0.3, 180.0, 50.0, 5000.0, 100.0, 5.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.5, 2.0, 0.0, 0.1, 0.0,
        ]];
        let labels = vec![vec![1.0, 0.0]]; // Human

        let loss = classifier.train(&features, &labels);
        println!("Training loss: {}", loss);
    }

    #[test]
    fn test_softmax() {
        let input = vec![1.0, 2.0, 3.0];
        let output = softmax(&input);

        // Check length
        assert_eq!(output.len(), 3);

        // Check range [0, 1]
        for &val in &output {
            assert!(val >= 0.0 && val <= 1.0);
        }

        // Check sum is 1.0
        let sum: f64 = output.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);

        // Check relative values (higher input -> higher output)
        assert!(output[2] > output[1]);
        assert!(output[1] > output[0]);

        // Numerical stability test (max trick)
        let input_large = vec![1000.0, 1001.0, 1002.0];
        let output_large = softmax(&input_large);
        let sum_large: f64 = output_large.iter().sum();
        assert!((sum_large - 1.0).abs() < 1e-10);
        assert!(!output_large[0].is_nan());
    }

    #[test]
    fn test_activations() {
        // Sigmoid
        assert!((sigmoid(0.0) - 0.5).abs() < 1e-10);
        assert!(sigmoid(100.0) > 0.99);
        assert!(sigmoid(-100.0) < 0.01);

        // Sigmoid derivative
        let d = sigmoid_derivative(0.0);
        assert!((d - 0.25).abs() < 1e-10);

        // Tanh
        assert!((tanh_activation(0.0)).abs() < 1e-10);
        assert!(tanh_activation(100.0) > 0.99);
        assert!(tanh_activation(-100.0) < -0.99);
    }
}
