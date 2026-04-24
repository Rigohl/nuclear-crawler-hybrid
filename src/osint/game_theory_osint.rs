// ============================================================================
// C: GAME THEORY FOR OSINT - RUST Implementation
// Adversarial modeling, Nash equilibrium, strategic decision making
// ============================================================================

use std::collections::HashMap;

/// ============================================================================
/// GAME THEORY BASICS
/// ============================================================================
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Payoff {
    pub p1: f64,
    pub p2: f64,
}

impl Payoff {
    pub fn new(p1: f64, p2: f64) -> Self {
        Payoff { p1, p2 }
    }
}

#[derive(Clone, Debug)]
pub struct PayoffMatrix {
    pub name: String,
    pub strategies: Vec<String>,
    pub payoffs: Vec<Vec<Payoff>>, // (player1, player2) payoff
}

impl PayoffMatrix {
    pub fn new(name: &str, strategies: Vec<String>) -> Self {
        let size = strategies.len();
        PayoffMatrix {
            name: name.to_string(),
            strategies,
            payoffs: vec![vec![Payoff::default(); size]; size],
        }
    }

    pub fn set_payoff(&mut self, i: usize, j: usize, payoff: Payoff) {
        if i < self.payoffs.len() && j < self.payoffs[i].len() {
            self.payoffs[i][j] = payoff;
        }
    }

    pub fn get_payoff(&self, i: usize, j: usize) -> Option<Payoff> {
        if i < self.payoffs.len() && j < self.payoffs[i].len() {
            Some(self.payoffs[i][j])
        } else {
            None
        }
    }
}

/// ============================================================================
/// MIXED STRATEGY - Probability distribution over pure strategies
/// ============================================================================
#[derive(Clone, Debug)]
pub struct MixedStrategy {
    pub strategy_name: String,
    pub probabilities: Vec<f64>, // Must sum to 1.0
}

impl MixedStrategy {
    pub fn new(strategy_name: &str, pure_strategies: usize) -> Self {
        // Uniform mixed strategy
        let prob = 1.0 / pure_strategies as f64;
        MixedStrategy {
            strategy_name: strategy_name.to_string(),
            probabilities: vec![prob; pure_strategies],
        }
    }

    pub fn pure_strategy(strategy_name: &str, index: usize, n: usize) -> Self {
        let mut probs = vec![0.0; n];
        probs[index] = 1.0;
        MixedStrategy {
            strategy_name: strategy_name.to_string(),
            probabilities: probs,
        }
    }

    pub fn is_valid(&self) -> bool {
        let sum: f64 = self.probabilities.iter().sum();
        (sum - 1.0).abs() < 1e-6
    }

    pub fn normalize(&mut self) {
        let sum: f64 = self.probabilities.iter().sum();
        for p in self.probabilities.iter_mut() {
            *p /= sum;
        }
    }
}

/// ============================================================================
/// NASH EQUILIBRIUM SOLVER - Lemke-Howson Algorithm
/// ============================================================================
pub struct NashSolver {
    game: PayoffMatrix,
}

impl NashSolver {
    pub fn new(game: PayoffMatrix) -> Self {
        NashSolver { game }
    }

    /// Find pure strategy Nash equilibrium
    pub fn find_pure_nash(&self) -> Vec<(usize, usize)> {
        let mut equilibria = Vec::new();
        let n = self.game.strategies.len();

        for i in 0..n {
            for j in 0..n {
                if let Some(payoff) = self.game.get_payoff(i, j) {
                    let mut is_nash = true;

                    // Check if player 1 can improve by deviating
                    for i_prime in 0..n {
                        if i_prime != i {
                            if let Some(payoff_alt) = self.game.get_payoff(i_prime, j) {
                                if payoff_alt.p1 > payoff.p1 {
                                    is_nash = false;
                                    break;
                                }
                            }
                        }
                    }

                    // Check if player 2 can improve by deviating
                    if is_nash {
                        for j_prime in 0..n {
                            if j_prime != j {
                                if let Some(payoff_alt) = self.game.get_payoff(i, j_prime) {
                                    if payoff_alt.p2 > payoff.p2 {
                                        is_nash = false;
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    if is_nash {
                        equilibria.push((i, j));
                    }
                }
            }
        }

        equilibria
    }

    /// Find mixed strategy Nash equilibrium (2x2 games)
    pub fn find_mixed_nash_2x2(&self) -> Option<(MixedStrategy, MixedStrategy)> {
        if self.game.strategies.len() != 2 {
            return None;
        }

        // For 2x2 game: A B vs C D
        let a = self.game.get_payoff(0, 0)?;
        let c = self.game.get_payoff(0, 1)?;
        let e = self.game.get_payoff(1, 0)?;
        let g = self.game.get_payoff(1, 1)?;

        // Player 2's indifference condition
        let p1_prob = if (c.p2 - e.p2).abs() < 1e-10 {
            0.5
        } else {
            (g.p2 - e.p2) / (c.p2 - e.p2 - g.p2 + e.p2)
        };

        // Player 1's indifference condition
        let p2_prob = if (c.p1 - a.p1).abs() < 1e-10 {
            0.5
        } else {
            (g.p1 - a.p1) / (c.p1 - a.p1 - g.p1 + a.p1)
        };

        if (0.0..=1.0).contains(&p1_prob) && (0.0..=1.0).contains(&p2_prob) {
            let mut s1 = MixedStrategy::new("Player1", 2);
            let mut s2 = MixedStrategy::new("Player2", 2);

            s1.probabilities[0] = p1_prob;
            s1.probabilities[1] = 1.0 - p1_prob;

            s2.probabilities[0] = p2_prob;
            s2.probabilities[1] = 1.0 - p2_prob;

            Some((s1, s2))
        } else {
            None
        }
    }

    /// Expected payoff for a mixed strategy profile
    pub fn expected_payoff(&self, s1: &MixedStrategy, s2: &MixedStrategy) -> (f64, f64) {
        let mut payoff1 = 0.0;
        let mut payoff2 = 0.0;

        for i in 0..self.game.strategies.len() {
            for j in 0..self.game.strategies.len() {
                if let Some(payoff) = self.game.get_payoff(i, j) {
                    payoff1 += s1.probabilities[i] * s2.probabilities[j] * payoff.p1;
                    payoff2 += s1.probabilities[i] * s2.probabilities[j] * payoff.p2;
                }
            }
        }

        (payoff1, payoff2)
    }
}

/// ============================================================================
/// OSINT-SPECIFIC GAME THEORY MODELS
/// ============================================================================
pub struct OSINTAdversarialGame {
    defender_strategies: Vec<String>,
    attacker_strategies: Vec<String>,
    payoffs: HashMap<(usize, usize), Payoff>,
}

impl OSINTAdversarialGame {
    /// Create adversarial game between defender (us) and attacker (threat actor)
    /// Defender strategies: Monitor, Block, Ignore, IncreaseMonitoring
    /// Attacker strategies: Attack, Subtle, Hide, Escalate
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let defender_strategies = vec![
            "Monitor".to_string(),
            "Block".to_string(),
            "Ignore".to_string(),
            "IncreaseMonitoring".to_string(),
        ];

        let attacker_strategies = vec![
            "Attack".to_string(),
            "Subtle".to_string(),
            "Hide".to_string(),
            "Escalate".to_string(),
        ];

        let mut payoffs = HashMap::new();

        // Payoff matrix (Defender, Attacker)
        // Monitor vs Attack: good for us, bad for attacker
        payoffs.insert((0, 0), Payoff::new(8.0, -5.0));
        payoffs.insert((0, 1), Payoff::new(6.0, -2.0));
        payoffs.insert((0, 2), Payoff::new(3.0, 2.0));
        payoffs.insert((0, 3), Payoff::new(4.0, 0.0));

        // Block vs *
        payoffs.insert((1, 0), Payoff::new(10.0, -8.0));
        payoffs.insert((1, 1), Payoff::new(7.0, -4.0));
        payoffs.insert((1, 2), Payoff::new(5.0, -6.0));
        payoffs.insert((1, 3), Payoff::new(2.0, -3.0));

        // Ignore vs *
        payoffs.insert((2, 0), Payoff::new(-5.0, 8.0));
        payoffs.insert((2, 1), Payoff::new(-2.0, 6.0));
        payoffs.insert((2, 2), Payoff::new(0.0, 4.0));
        payoffs.insert((2, 3), Payoff::new(-8.0, 9.0));

        // IncreaseMonitoring vs *
        payoffs.insert((3, 0), Payoff::new(9.0, -6.0));
        payoffs.insert((3, 1), Payoff::new(7.0, -3.0));
        payoffs.insert((3, 2), Payoff::new(4.0, 1.0));
        payoffs.insert((3, 3), Payoff::new(5.0, -1.0));

        OSINTAdversarialGame {
            defender_strategies,
            attacker_strategies,
            payoffs,
        }
    }

    pub fn solve_equilibrium(&self) -> Vec<(String, String)> {
        let mut equilibria = Vec::new();

        for (d_idx, d_strat) in self.defender_strategies.iter().enumerate() {
            for (a_idx, a_strat) in self.attacker_strategies.iter().enumerate() {
                if let Some(payoff) = self.payoffs.get(&(d_idx, a_idx)) {
                    let mut is_nash = true;

                    // Check defender's best response
                    for d_idx_alt in 0..self.defender_strategies.len() {
                        if d_idx_alt != d_idx {
                            if let Some(payoff_alt) = self.payoffs.get(&(d_idx_alt, a_idx)) {
                                if payoff_alt.p1 > payoff.p1 {
                                    is_nash = false;
                                    break;
                                }
                            }
                        }
                    }

                    // Check attacker's best response
                    if is_nash {
                        for a_idx_alt in 0..self.attacker_strategies.len() {
                            if a_idx_alt != a_idx {
                                if let Some(payoff_alt) = self.payoffs.get(&(d_idx, a_idx_alt)) {
                                    if payoff_alt.p2 > payoff.p2 {
                                        is_nash = false;
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    if is_nash {
                        equilibria.push((d_strat.clone(), a_strat.clone()));
                    }
                }
            }
        }

        equilibria
    }

    /// Strategic recommendation for defender
    pub fn recommend_strategy(&self) -> String {
        let equilibria = self.solve_equilibrium();

        if equilibria.is_empty() {
            return "IncreaseMonitoring".to_string(); // Safe fallback
        }

        // Find most favorable equilibrium for defender
        let mut best_strategy = "Monitor".to_string();
        let mut best_payoff = f64::NEG_INFINITY;

        for (d_strat, a_strat) in &equilibria {
            let d_idx = self
                .defender_strategies
                .iter()
                .position(|s| s == d_strat)
                .unwrap_or(0);
            let a_idx = self
                .attacker_strategies
                .iter()
                .position(|s| s == a_strat)
                .unwrap_or(0);

            if let Some(payoff) = self.payoffs.get(&(d_idx, a_idx)) {
                if payoff.p1 > best_payoff {
                    best_payoff = payoff.p1;
                    best_strategy = d_strat.clone();
                }
            }
        }

        best_strategy
    }
}

/// ============================================================================
/// SIGNALING AND SCREENING GAMES
/// ============================================================================
pub struct SignalingGame {
    // Sender (threat actor) type probabilities
    pub type_probs: HashMap<String, f64>,
    // Sender strategy: type -> action
    pub sender_strategy: HashMap<String, Vec<f64>>,
    // Receiver belief update
    pub receiver_belief: HashMap<String, f64>,
}

impl SignalingGame {
    pub fn new(types: Vec<String>) -> Self {
        let mut type_probs = HashMap::new();
        for t in &types {
            type_probs.insert(t.clone(), 1.0 / types.len() as f64);
        }

        let receiver_belief = type_probs.clone();
        SignalingGame {
            type_probs,
            sender_strategy: HashMap::new(),
            receiver_belief,
        }
    }

    /// Update beliefs using Bayes rule
    pub fn update_beliefs(&mut self, signal: &str, _evidence: f64) {
        let mut total_posterior = 0.0;

        for (type_label, prior) in &self.type_probs {
            // Likelihood of signal given type (simplified)
            let likelihood = if signal == "aggressive" { 0.8 } else { 0.2 };
            let posterior = prior * likelihood;
            self.receiver_belief.insert(type_label.clone(), posterior);
            total_posterior += posterior;
        }

        // Normalize
        for posterior in self.receiver_belief.values_mut() {
            *posterior /= total_posterior + 1e-10;
        }
    }
}

/// ============================================================================
/// COALITION FORMATION IN NETWORKS
/// ============================================================================
pub struct CoalitionGame {
    pub players: Vec<String>,
    pub coalitions: Vec<Vec<usize>>,
    pub coalition_values: HashMap<Vec<usize>, f64>,
}

impl CoalitionGame {
    pub fn new(players: Vec<String>) -> Self {
        CoalitionGame {
            players,
            coalitions: Vec::new(),
            coalition_values: HashMap::new(),
        }
    }

    pub fn add_coalition(&mut self, coalition: Vec<usize>, value: f64) {
        self.coalitions.push(coalition.clone());
        self.coalition_values.insert(coalition, value);
    }

    /// Shapley value: fair value distribution
    pub fn compute_shapley_value(&self) -> Vec<f64> {
        let n = self.players.len();
        let mut shapley = vec![0.0; n];

        for (player, shapley_val) in shapley.iter_mut().enumerate() {
            let mut contribution = 0.0;
            let mut count = 0;

            // Calculate marginal contribution
            for coalition in &self.coalitions {
                if !coalition.contains(&player) {
                    let without = self.coalition_values.get(coalition).copied().unwrap_or(0.0);

                    let mut with = coalition.clone();
                    with.push(player);
                    let with_val = self.coalition_values.get(&with).copied().unwrap_or(0.0);

                    contribution += with_val - without;
                    count += 1;
                }
            }

            if count > 0 {
                *shapley_val = contribution / count as f64;
            }
        }

        shapley
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payoff_matrix() {
        let mut game = PayoffMatrix::new("TestGame", vec!["A".to_string(), "B".to_string()]);
        game.set_payoff(0, 0, Payoff::new(3.0, 3.0));
        game.set_payoff(0, 1, Payoff::new(0.0, 5.0));
        game.set_payoff(1, 0, Payoff::new(5.0, 0.0));
        game.set_payoff(1, 1, Payoff::new(1.0, 1.0));

        assert_eq!(game.get_payoff(0, 0), Some(Payoff::new(3.0, 3.0)));
    }

    #[test]
    fn test_nash_equilibrium() {
        let mut game = PayoffMatrix::new(
            "PrisonersDilemma",
            vec!["Cooperate".to_string(), "Defect".to_string()],
        );
        game.set_payoff(0, 0, Payoff::new(-1.0, -1.0)); // Both cooperate
        game.set_payoff(0, 1, Payoff::new(-3.0, 0.0)); // Cooperate vs Defect
        game.set_payoff(1, 0, Payoff::new(0.0, -3.0)); // Defect vs Cooperate
        game.set_payoff(1, 1, Payoff::new(-2.0, -2.0)); // Both defect

        let solver = NashSolver::new(game);
        let equilibria = solver.find_pure_nash();

        assert!(equilibria.contains(&(1, 1))); // Mutual defection is Nash
    }

    #[test]
    fn test_adversarial_game() {
        let game = OSINTAdversarialGame::new();
        let equilibria = game.solve_equilibrium();
        let recommendation = game.recommend_strategy();

        println!("Equilibria: {:?}", equilibria);
        println!("Recommended strategy: {}", recommendation);
    }

    #[test]
    fn test_coalition_game() {
        let mut game = CoalitionGame::new(vec!["Alice".to_string(), "Bob".to_string()]);
        game.add_coalition(vec![], 0.0);
        game.add_coalition(vec![0], 5.0);
        game.add_coalition(vec![1], 3.0);
        game.add_coalition(vec![0, 1], 12.0);

        let shapley = game.compute_shapley_value();
        println!("Shapley values: {:?}", shapley);
    }
}
