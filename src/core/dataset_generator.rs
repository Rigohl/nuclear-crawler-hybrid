/// Nuclear Chapel AI - Dataset Generation System
/// 
/// This module demonstrates the workflow:
/// 1. Rust calls Python generator (via subprocess)
/// 2. Python generates 120K real samples to JSON
/// 3. Rust tokenizes and processes the dataset
/// 4. Chapel AI trains on the massive dataset
/// 
/// Architecture: Rust orchestrates, Python generates real data, Chapel trains

use std::process::Command;
use std::path::Path;

/// Generates massive dataset using real data patterns
/// Calls the Python generator which creates 120K samples
pub fn generate_massive_dataset() -> Result<String, Box<dyn std::error::Error>> {
    println!("🚀 [Rust] Orchestrating massive dataset generation...");
    
    let python_script = "generate_massive_dataset.py";
    
    // Verify Python script exists
    if !Path::new(python_script).exists() {
        return Err("Python generator script not found".into());
    }
    
    // Call Python generator
    println!("📞 [Rust] Calling Python generator...");
    let output = Command::new("python3")
        .arg(python_script)
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Generator failed: {}", stderr).into());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
    
    Ok("Dataset generation complete".to_string())
}

/// Loads the massive dataset from JSON
pub fn load_dataset(path: &str) -> Result<Dataset, Box<dyn std::error::Error>> {
    use std::fs;
    
    println!("📂 [Rust] Loading dataset from: {}", path);
    
    let content = fs::read_to_string(path)?;
    let dataset: Dataset = serde_json::from_str(&content)?;
    
    println!("✅ [Rust] Loaded {} total samples", dataset.metadata.total_samples);
    println!("   • Fake news: {}", dataset.metadata.breakdown.fake_news);
    println!("   • Code samples: {}", dataset.metadata.breakdown.code_samples);
    println!("   • Configurations: {}", dataset.metadata.breakdown.configurations);
    println!("   • Search data: {}", dataset.metadata.breakdown.information_search);
    
    Ok(dataset)
}

/// Prepares dataset for Chapel training
pub fn prepare_for_chapel_training(dataset: &Dataset) -> Result<String, Box<dyn std::error::Error>> {
    println!("🔧 [Rust] Preparing dataset for Chapel AI training...");
    
    // Calculate training splits
    let total = dataset.metadata.total_samples;
    let train_size = (total as f64 * 0.8) as usize;
    let val_size = (total as f64 * 0.1) as usize;
    let test_size = total - train_size - val_size;
    
    println!("📊 [Rust] Training split:");
    println!("   • Training: {} samples ({}%)", train_size, 80);
    println!("   • Validation: {} samples ({}%)", val_size, 10);
    println!("   • Testing: {} samples ({}%)", test_size, 10);
    
    // Verify all data types are present
    assert!(!dataset.fake_news.is_empty(), "Missing fake news samples");
    assert!(!dataset.code_samples.is_empty(), "Missing code samples");
    assert!(!dataset.configurations.is_empty(), "Missing configurations");
    assert!(!dataset.information_search.is_empty(), "Missing search samples");
    
    println!("✅ [Rust] Dataset ready for Chapel AI processing");
    println!("🎯 [Rust] Next: Chapel will train on this real data");
    
    Ok("Preparation complete".to_string())
}

// ============= Data structures =============

#[derive(serde::Deserialize, Debug)]
pub struct Dataset {
    pub metadata: Metadata,
    pub fake_news: Vec<FakeNewsSample>,
    pub code_samples: Vec<CodeSample>,
    pub configurations: Vec<Config>,
    pub information_search: Vec<SearchSample>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Metadata {
    pub version: String,
    pub total_samples: usize,
    pub generated: String,
    pub generator: String,
    pub breakdown: Breakdown,
}

#[derive(serde::Deserialize, Debug)]
pub struct Breakdown {
    pub fake_news: usize,
    pub code_samples: usize,
    pub configurations: usize,
    pub information_search: usize,
}

#[derive(serde::Deserialize, Debug)]
pub struct FakeNewsSample {
    pub id: usize,
    pub text: String,
    pub source: String,
    pub red_flags: Vec<String>,
    pub topic: String,
    pub credibility: f64,
    pub label: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct CodeSample {
    pub id: usize,
    pub pattern: String,
    pub code: String,
    pub category: String,
    pub metrics: CodeMetrics,
    pub smells: Vec<String>,
    pub optimization_potential: f64,
}

#[derive(serde::Deserialize, Debug)]
pub struct CodeMetrics {
    pub complexity: usize,
    pub lines: usize,
    pub functions: usize,
    pub tokens: usize,
    pub cyclomatic: usize,
}

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub id: String,
    pub layers: Vec<usize>,
    pub optimizer: String,
    pub learning_rate: f64,
    pub batch_size: usize,
    pub epochs: usize,
    pub locales: usize,
    pub strategy: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct SearchSample {
    pub id: usize,
    pub query: String,
    pub num_results: usize,
    pub results: Vec<SearchResult>,
    pub consensus_score: f64,
    pub search_category: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct SearchResult {
    pub rank: usize,
    pub source: String,
    pub credibility: f64,
    pub relevance: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dataset_structure() {
        // Verify the dataset can be loaded and processed
        let path = "ffi/chapel/datasets/massive_training_120k.json";
        if std::path::Path::new(path).exists() {
            let result = load_dataset(path);
            assert!(result.is_ok());
            
            let dataset = result.unwrap();
            assert_eq!(dataset.metadata.total_samples, 120000);
            assert!(dataset.fake_news.len() > 0);
            assert!(dataset.code_samples.len() > 0);
        }
    }
}
