//! Example: Chatbot with HuggingFace Integration
//!
//! This example demonstrates how to use the Nuclear AI chatbot
//! with HuggingFace models for advanced AI capabilities.
//!
//! Prerequisites:
//! - Set HF_TOKEN environment variable
//! - Ensure you have access to the model

use nuclear_crawler_hybrid::{Chatbot, ChatbotConfig, HuggingFaceClient, HuggingFaceConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🤖 Nuclear AI Chatbot - HuggingFace Example\n");

    // Check if HF_TOKEN is set
    if std::env::var("HF_TOKEN").is_err() {
        eprintln!("⚠️  HF_TOKEN not set. Please set it to use HuggingFace models:");
        eprintln!("   export HF_TOKEN=\"hf_...\"");
        eprintln!("\nFalling back to local mode...\n");
    }

    // Create HuggingFace client
    let hf_config = HuggingFaceConfig::default();
    let hf_client = HuggingFaceClient::new(hf_config).ok();

    // Create chatbot with HuggingFace support
    let chatbot_config = ChatbotConfig {
        model_name: "mistralai/Mistral-7B-Instruct-v0.2".to_string(),
        max_tokens: 512,
        enable_chapel_learning: true,
        enable_tools: true,
        ..Default::default()
    };

    let chatbot = Chatbot::new(chatbot_config, hf_client);

    println!("Chatbot initialized with HuggingFace support\n");

    // Advanced conversation
    let messages = vec![
        "Hello! I'm working on a Rust project and need help.",
        "Can you explain how async/await works in Rust?",
        "What are the best practices for error handling?",
        "Search for recent Rust performance optimization techniques",
    ];

    for (i, message) in messages.iter().enumerate() {
        println!("👤 User (Turn {}): {}", i + 1, message);
        
        let response = chatbot.chat(message).await?;
        
        println!("🤖 Bot: {}\n", response);
        
        // Show which tools were detected
        let history = chatbot.get_history()?;
        if let Some(turn) = history.last() {
            if !turn.tools_used.is_empty() {
                println!("🔧 Tools detected: {:?}\n", turn.tools_used);
            }
        }
        
        println!("---\n");
    }

    // Get final statistics
    let stats = chatbot.get_statistics()?;
    println!("📊 Final Statistics:");
    println!("{}\n", serde_json::to_string_pretty(&stats)?);

    // Show conversation quality
    let history = chatbot.get_history()?;
    let avg_quality: f64 = history.iter().map(|t| t.quality_score).sum::<f64>() 
        / history.len() as f64;
    println!("📈 Average conversation quality: {:.2}", avg_quality);

    Ok(())
}
