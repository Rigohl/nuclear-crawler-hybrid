//! 🔥 MCP TOOL: Chatbot - Interactive AI Assistant
//!
//! This tool provides a conversational interface powered by:
//! - Chapel AI learning system
//! - HuggingFace models (optional)
//! - Integration with all other MCP tools

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::OnceLock;

use crate::chatbot::{Chatbot, ChatbotConfig};
use crate::huggingface_integration::{HuggingFaceClient, HuggingFaceConfig};

/// Chatbot tool arguments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatbotArgs {
    /// User message
    pub message: String,
    /// Optional: specific model to use
    pub model: Option<String>,
    /// Optional: max tokens for response
    pub max_tokens: Option<usize>,
    /// Optional: enable Chapel AI learning
    pub enable_learning: Option<bool>,
}

/// Chatbot tool result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatbotResult {
    pub response: String,
    pub model_used: String,
    pub tokens_used: usize,
    pub tools_available: Vec<String>,
    pub conversation_turns: usize,
}

/// Global chatbot instance (thread-safe singleton)
static CHATBOT: OnceLock<Chatbot> = OnceLock::new();

/// Initialize the global chatbot
fn get_or_create_chatbot() -> &'static Chatbot {
    CHATBOT.get_or_init(|| {
        // Try to create HuggingFace client if token is set
        let hf_client = if std::env::var("HF_TOKEN").is_ok() {
            let config = HuggingFaceConfig::default();
            HuggingFaceClient::new(config).ok()
        } else {
            None
        };

        let config = ChatbotConfig::default();
        Chatbot::new(config, hf_client)
    })
}

/// Execute chatbot tool
pub async fn execute_chatbot(args: ChatbotArgs) -> Result<Value> {
    eprintln!("🤖 Chatbot Tool: Processing message...");

    let chatbot = get_or_create_chatbot();

    // Chat with the bot
    let response = chatbot.chat(&args.message).await?;

    // Get statistics
    let stats = chatbot.get_statistics()?;
    let turns = stats["total_turns"].as_u64().unwrap_or(0) as usize;

    let result = ChatbotResult {
        response,
        model_used: "nuclear-ai".to_string(),
        tokens_used: args.max_tokens.unwrap_or(512),
        tools_available: vec![
            "websearch".to_string(),
            "file_search".to_string(),
            "scan".to_string(),
            "premium".to_string(),
            "ai_dataset_trainer".to_string(),
        ],
        conversation_turns: turns,
    };

    Ok(json!(result))
}

/// Get chatbot history
pub async fn get_chatbot_history() -> Result<Value> {
    let chatbot = get_or_create_chatbot();
    let history = chatbot.get_history()?;
    Ok(json!(history))
}

/// Clear chatbot history
pub async fn clear_chatbot_history() -> Result<Value> {
    let chatbot = get_or_create_chatbot();
    chatbot.clear_history()?;
    Ok(json!({"status": "cleared"}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chatbot_tool() {
        let args = ChatbotArgs {
            message: "Hello".to_string(),
            model: None,
            max_tokens: None,
            enable_learning: Some(true),
        };

        let result = execute_chatbot(args).await;
        assert!(result.is_ok());
    }
}
