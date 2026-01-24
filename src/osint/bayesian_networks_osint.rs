// ============================================================================
// B: BAYESIAN NETWORKS FOR OSINT - RUST Implementation
// Probabilistic inference for evidence aggregation and confidence scoring
// ============================================================================

use std::collections::HashMap;

/// ============================================================================
/// BAYESIAN NODE - Represents a variable in the network
/// ============================================================================

#[derive(Clone, Debug)]
pub struct BayesianNode {
    pub name: String,
    pub states: Vec<String>,
    pub cpt: Vec<f64>, // Conditional probability table
    pub parents: Vec<String>,
    pub value: Option<String>,
}

impl BayesianNode {
    pub fn new(name: &str, states: Vec<String>) -> Self {
        BayesianNode {
            name: name.to_string(),
            states,
            cpt: vec![],
            parents: vec![],
            value: None,
        }
    }

    pub fn add_parent(&mut self, parent: String) {
        self.parents.push(parent);
    }

    pub fn set_cpt(&mut self, cpt: Vec<f64>) {
        self.cpt = cpt;
    }
}

/// ============================================================================
/// BAYESIAN NETWORK - Directed acyclic graph of probabilistic variables
/// ============================================================================

pub struct BayesianNetwork {
    nodes: HashMap<String, BayesianNode>,
    edges: Vec<(String, String)>, // (parent, child)
}

impl BayesianNetwork {
    pub fn new() -> Self {
        BayesianNetwork {
            nodes: HashMap::new(),
            edges: vec![],
        }
    }

    pub fn add_node(&mut self, node: BayesianNode) {
        self.nodes.insert(node.name.clone(), node);
    }

    pub fn add_edge(&mut self, parent: &str, child: &str) {
        self.edges.push((parent.to_string(), child.to_string()));

        if let Some(child_node) = self.nodes.get_mut(child) {
            child_node.add_parent(parent.to_string());
        }
    }

    pub fn get_node(&self, name: &str) -> Option<&BayesianNode> {
        self.nodes.get(name)
    }

    pub fn get_node_mut(&mut self, name: &str) -> Option<&mut BayesianNode> {
        self.nodes.get_mut(name)
    }

    /// ========================================================================
    /// INFERENCE ENGINES
    /// ========================================================================

    /// Variable Elimination inference
    pub fn variable_elimination(
        &self,
        query_variable: &str,
        evidence: &HashMap<String, String>,
    ) -> HashMap<String, f64> {
        let mut result = HashMap::new();

        // Simplified: Direct probability computation from CPTs
        if let Some(query_node) = self.nodes.get(query_variable) {
            for state in &query_node.states {
                let prob = self.compute_probability(query_variable, state, evidence);
                result.insert(state.clone(), prob);
            }

            // Normalize
            let total: f64 = result.values().sum();
            for prob in result.values_mut() {
                *prob /= total + 1e-10;
            }
        }

        result
    }

    /// Gibbs Sampling for approximate inference
    pub fn gibbs_sampling(
        &self,
        query_variable: &str,
        evidence: &HashMap<String, String>,
        iterations: usize,
    ) -> HashMap<String, f64> {
        let mut samples = HashMap::new();
        let mut current_state = evidence.clone();

        // Initialize unobserved variables randomly
        for (name, node) in &self.nodes {
            if !evidence.contains_key(name) {
                current_state.insert(name.clone(), node.states[0].clone());
            }
        }

        // Gibbs sampling iterations
        for _ in 0..iterations {
            for (name, node) in &self.nodes {
                if !evidence.contains_key(name) {
                    // Sample from conditional distribution
                    let mut probs = vec![];
                    for state in &node.states {
                        let mut temp_state = current_state.clone();
                        temp_state.insert(name.clone(), state.clone());
                        let prob = self.compute_probability(name, state, &evidence);
                        probs.push(prob);
                    }

                    // Normalize and sample
                    let total: f64 = probs.iter().sum();
                    let probs: Vec<f64> = probs.iter().map(|p| p / (total + 1e-10)).collect();

                    let sampled_state = sample_from_distribution(&node.states, &probs);
                    current_state.insert(name.clone(), sampled_state);
                }
            }

            // Record sample
            if let Some(state) = current_state.get(query_variable) {
                *samples.entry(state.clone()).or_insert(0.0) += 1.0;
            }
        }

        // Normalize counts to probabilities
        let total = iterations as f64;
        for count in samples.values_mut() {
            *count /= total;
        }

        samples
    }

    /// Belief Propagation (for tree-structured networks)
    pub fn belief_propagation(&self, evidence: &HashMap<String, String>) -> HashMap<String, f64> {
        let mut beliefs = HashMap::new();

        for (name, node) in &self.nodes {
            for state in &node.states {
                let prob = self.compute_probability(name, state, evidence);
                beliefs.insert(format!("{}={}", name, state), prob);
            }
        }

        beliefs
    }

    fn compute_probability(
        &self,
        variable: &str,
        value: &str,
        evidence: &HashMap<String, String>,
    ) -> f64 {
        // Simplified probability computation
        // In a real implementation, this would traverse the CPT structure

        if let Some(node) = self.nodes.get(variable) {
            let state_idx = node.states.iter().position(|s| s == value).unwrap_or(0);

            // Base probability from CPT
            let base_prob = if state_idx < node.cpt.len() {
                node.cpt[state_idx]
            } else {
                1.0 / node.states.len() as f64
            };

            // Adjust based on parent evidence
            let mut adjustment = 1.0;
            for parent in &node.parents {
                if let Some(parent_value) = evidence.get(parent) {
                    if let Some(parent_node) = self.nodes.get(parent) {
                        let parent_idx = parent_node
                            .states
                            .iter()
                            .position(|s| s == parent_value)
                            .unwrap_or(0);
                        let parent_prob =
                            if parent_idx < parent_node.cpt.len() { parent_node.cpt[parent_idx] } else { 0.5 };
                        adjustment *= parent_prob;
                    }
                }
            }

            base_prob * adjustment
        } else {
            0.5 // Default probability
        }
    }
}

fn sample_from_distribution(states: &[String], probabilities: &[f64]) -> String {
    let mut cumulative = 0.0;
    let random = rand::random::<f64>();

    for (i, &prob) in probabilities.iter().enumerate() {
        cumulative += prob;
        if random <= cumulative {
            return states[i].clone();
        }
    }

    states[states.len() - 1].clone()
}

/// ============================================================================
/// SPECIALIZED: OSINT EVIDENCE AGGREGATION NETWORK
/// ============================================================================

pub struct OSINTBayesianNetwork {
    network: BayesianNetwork,
}

impl OSINTBayesianNetwork {
    /// Create OSINT Bayesian Network:
    /// Variables: IsBot, CoordinationScore, EntropyAnomaly, SentimentAnomaly, FinalJudgment
    pub fn new() -> Self {
        let mut network = BayesianNetwork::new();

        // Create nodes
        let mut is_bot = BayesianNode::new("IsBot", vec!["yes".to_string(), "no".to_string()]);
        is_bot.set_cpt(vec![0.05, 0.95]); // Prior: 5% bots
        network.add_node(is_bot);

        let mut coordination_score = BayesianNode::new(
            "CoordinationScore",
            vec!["high".to_string(), "medium".to_string(), "low".to_string()],
        );
        coordination_score.set_cpt(vec![0.1, 0.3, 0.6]); // Prior
        network.add_node(coordination_score);

        let mut entropy_anomaly =
            BayesianNode::new("EntropyAnomaly", vec!["yes".to_string(), "no".to_string()]);
        entropy_anomaly.set_cpt(vec![0.2, 0.8]); // Prior: 20% anomalous
        network.add_node(entropy_anomaly);

        let mut sentiment_anomaly =
            BayesianNode::new("SentimentAnomaly", vec!["yes".to_string(), "no".to_string()]);
        sentiment_anomaly.set_cpt(vec![0.15, 0.85]); // Prior: 15% anomalous
        network.add_node(sentiment_anomaly);

        // Final judgment depends on all evidence
        let mut final_judgment =
            BayesianNode::new("FinalJudgment", vec!["bot".to_string(), "human".to_string()]);
        final_judgment.set_cpt(vec![0.5, 0.5]); // Initial
        network.add_node(final_judgment);

        // Add edges (dependencies)
        network.add_edge("IsBot", "FinalJudgment");
        network.add_edge("CoordinationScore", "FinalJudgment");
        network.add_edge("EntropyAnomaly", "FinalJudgment");
        network.add_edge("SentimentAnomaly", "FinalJudgment");

        OSINTBayesianNetwork { network }
    }

    /// Aggregate multiple evidence sources and output final confidence
    pub fn aggregate_evidence(
        &self,
        bot_probability: f64,
        coordination_level: &str,
        has_entropy_anomaly: bool,
        has_sentiment_anomaly: bool,
    ) -> (String, f64) {
        let mut evidence = HashMap::new();

        // Set evidence
        evidence.insert(
            "IsBot".to_string(),
            if bot_probability > 0.5 { "yes".to_string() } else { "no".to_string() },
        );

        evidence.insert("CoordinationScore".to_string(), coordination_level.to_string());

        evidence.insert(
            "EntropyAnomaly".to_string(),
            if has_entropy_anomaly { "yes".to_string() } else { "no".to_string() },
        );

        evidence.insert(
            "SentimentAnomaly".to_string(),
            if has_sentiment_anomaly { "yes".to_string() } else { "no".to_string() },
        );

        // Perform inference
        let result = self.network.variable_elimination("FinalJudgment", &evidence);

        // Get maximum probability judgment
        let mut max_prob = 0.0;
        let mut judgment = "human".to_string();

        for (state, prob) in result {
            if prob > max_prob {
                max_prob = prob;
                judgment = state;
            }
        }

        (judgment, max_prob)
    }

    /// Bayesian updating: incrementally update beliefs as new evidence arrives
    pub fn update_belief(&self, prior_bot_prob: f64, likelihood_ratio: f64) -> f64 {
        let prior_odds = prior_bot_prob / (1.0 - prior_bot_prob + 1e-10);
        let posterior_odds = prior_odds * likelihood_ratio;
        posterior_odds / (1.0 + posterior_odds)
    }

    /// Perform inference with evidence
    pub fn infer(&self, evidence: &HashMap<String, String>) -> HashMap<String, f64> {
        self.network.gibbs_sampling("FinalJudgment", evidence, 1000)
    }
}

/// ============================================================================
/// NAIVE BAYES CLASSIFIER FOR QUICK DECISIONS
/// ============================================================================

pub struct OSINTNaiveBayes {
    class_priors: HashMap<String, f64>,
    feature_likelihoods: HashMap<String, HashMap<String, f64>>,
    classes: Vec<String>,
}

impl OSINTNaiveBayes {
    pub fn new() -> Self {
        OSINTNaiveBayes {
            class_priors: HashMap::new(),
            feature_likelihoods: HashMap::new(),
            classes: vec!["bot".to_string(), "human".to_string()],
        }
    }

    pub fn train(&mut self, X: &[Vec<f64>], y: &[String]) {
        let n_samples = X.len();
        let n_features = if n_samples > 0 { X[0].len() } else { 0 };

        // Compute class priors
        for class in &self.classes {
            let count = y.iter().filter(|c| *c == class).count();
            self.class_priors.insert(class.clone(), count as f64 / n_samples as f64);
        }

        // Compute feature likelihoods (Gaussian assumption)
        for feature_idx in 0..n_features {
            let feature_name = format!("feature_{}", feature_idx);

            for class in &self.classes {
                let class_values: Vec<f64> = X
                    .iter()
                    .zip(y.iter())
                    .filter(|(_, c)| *c == class)
                    .map(|(x, _)| x[feature_idx])
                    .collect();

                if !class_values.is_empty() {
                    let mean = class_values.iter().sum::<f64>() / class_values.len() as f64;
                    let variance = class_values
                        .iter()
                        .map(|v| (v - mean).powi(2))
                        .sum::<f64>()
                        / class_values.len() as f64;

                    // Ensure entry exists
                    self.feature_likelihoods.entry(feature_name.clone()).or_insert_with(HashMap::new);

                    // Store mean and variance
                    if let Some(inner_map) = self.feature_likelihoods.get_mut(&feature_name) {
                        inner_map.insert(format!("{}_mean", class), mean);
                        inner_map.insert(format!("{}_var", class), variance + 1e-9);
                    }
                }
            }
        }
    }

    pub fn predict(&self, x: &[f64]) -> (String, f64) {
        let mut posteriors = HashMap::new();

        for class in &self.classes {
            let mut posterior = self.class_priors.get(class).cloned().unwrap_or(0.5).ln();

            for (feature_idx, &value) in x.iter().enumerate() {
                let feature_name = format!("feature_{}", feature_idx);

                if let Some(likelihoods) = self.feature_likelihoods.get(&feature_name) {
                    let mean = likelihoods
                        .get(&format!("{}_mean", class))
                        .cloned()
                        .unwrap_or(0.0);
                    let var = likelihoods
                        .get(&format!("{}_var", class))
                        .cloned()
                        .unwrap_or(1.0);

                    // Gaussian likelihood
                    let likelihood = -0.5 * (value - mean).powi(2) / var
                        - 0.5 * var.ln()
                        - 0.5 * std::f64::consts::LN_2 * std::f64::consts::PI;
                    posterior += likelihood;
                }
            }

            posteriors.insert(class.clone(), posterior);
        }

        // Convert log posteriors to probabilities
        let max_posterior = posteriors.values().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exp_sum: f64 = posteriors
            .values()
            .map(|&p| (p - max_posterior).exp())
            .sum();

        let class = posteriors
            .iter()
            .max_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap())
            .map(|(c, _)| c.clone())
            .unwrap_or("human".to_string());

        let confidence = ((posteriors.get(&class).cloned().unwrap_or(0.0) - max_posterior).exp()
            / exp_sum)
            .max(0.5)
            .min(0.99);

        (class, confidence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bayesian_network() {
        let network = OSINTBayesianNetwork::new();
        let (judgment, confidence) =
            network.aggregate_evidence(0.8, "high", true, false);
        println!("Judgment: {} (confidence: {})", judgment, confidence);
        assert!(confidence >= 0.0 && confidence <= 1.0);
    }

    #[test]
    fn test_bayesian_updating() {
        let network = OSINTBayesianNetwork::new();
        let updated_prob = network.update_belief(0.05, 10.0);
        println!("Updated probability: {}", updated_prob);
        assert!(updated_prob > 0.05); // Should increase with positive evidence
    }

    #[test]
    fn test_naive_bayes() {
        let mut nb = OSINTNaiveBayes::new();
        let X = vec![
            vec![0.1, 0.2, 0.3, 0.4, 0.5],
            vec![0.2, 0.3, 0.4, 0.5, 0.6],
        ];
        let y = vec!["bot".to_string(), "human".to_string()];
        nb.train(&X, &y);

        let (pred, conf) = nb.predict(&vec![0.15, 0.25, 0.35, 0.45, 0.55]);
        println!("Prediction: {} (confidence: {})", pred, conf);
    }
}
