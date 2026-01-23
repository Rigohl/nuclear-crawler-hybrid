//! Example: Basic Chatbot Usage
//!
//! This example demonstrates how to use the Nuclear AI chatbot
//! in local mode (without HuggingFace).

use nuclear_crawler_hybrid::{Chatbot, ChatbotConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🤖 Nuclear AI Chatbot - Basic Example\n");

    // Create chatbot with default config (local mode)
    let config = ChatbotConfig::default();
    let chatbot = Chatbot::new(config, None);

    println!("Chatbot initialized in local mode\n");

    // Simple conversation
    let messages = vec![
        "Hello! What can you do?",
        "Can you help me search for Rust tutorials?",
        "What about file analysis?",
        "help",
    ];

    for (i, message) in messages.iter().enumerate() {
        println!("👤 User (Turn {}): {}", i + 1, message);
        
        let response = chatbot.chat(message).await?;
        
        println!("🤖 Bot: {}\n", response);
        println!("---\n");
    }

    // Get statistics
    let stats = chatbot.get_statistics()?;
    println!("📊 Conversation Statistics:");
    println!("{}\n", serde_json::to_string_pretty(&stats)?);

    // Get history
    let history = chatbot.get_history()?;
    println!("📜 Conversation History:");
    for (i, turn) in history.iter().enumerate() {
        println!("  Turn {}: Quality = {:.2}", i + 1, turn.quality_score);
    }

    Ok(())
}
