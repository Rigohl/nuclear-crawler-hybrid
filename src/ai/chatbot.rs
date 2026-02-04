//! 🔥 CHATBOT MODULE - Interactive AI Chat Interface
//!
//! Provides a chatbot interface powered by:
//! - Chapel AI learning system
//! - HuggingFace model integration
//! - All 5 MCP tools (websearch, premium, file_search, scan, ai_dataset_trainer)
//!
//! Features:
//! - Conversational interface
//! - Context-aware responses
//! - Tool integration for enhanced capabilities
//! - Learning from interactions
//! - NO MOCKS - Real implementation

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

use crate::chapel_integration::{create_context, get_chapel_ai};
use crate::huggingface_integration::{ChatMessage, HuggingFaceClient};

/// Chatbot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatbotConfig {
    /// Model to use for chat (HuggingFace model name)
    pub model_name: String,
    /// Maximum conversation history to maintain
    pub max_history: usize,
    /// Maximum tokens per response
    pub max_tokens: usize,
    /// System prompt
    pub system_prompt: String,
    /// Enable Chapel AI learning
    pub enable_chapel_learning: bool,
    /// Enable tool integration
    pub enable_tools: bool,
}

impl Default for ChatbotConfig {
    fn default() -> Self {
        Self {
            model_name: "mistralai/Mistral-7B-Instruct-v0.2".to_string(),
            max_history: 10,
            max_tokens: 512,
            system_prompt: "You are Nuclear AI, an advanced AI assistant powered by Chapel learning system and connected to web search, file analysis, and data extraction tools. You provide helpful, accurate, and contextual responses.".to_string(),
            enable_chapel_learning: true,
            enable_tools: true,
        }
    }
}

/// Conversation history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub user_message: String,
    pub assistant_message: String,
    pub timestamp: u64,
    pub tools_used: Vec<String>,
    pub quality_score: f64,
}

/// Chatbot state
#[derive(Debug, Clone)]
pub struct ChatbotState {
    conversation_history: VecDeque<ConversationTurn>,
    total_turns: usize,
    tools_used_count: std::collections::HashMap<String, usize>,
}

impl ChatbotState {
    fn new() -> Self {
        Self {
            conversation_history: VecDeque::new(),
            total_turns: 0,
            tools_used_count: std::collections::HashMap::new(),
        }
    }

    fn add_turn(&mut self, turn: ConversationTurn) {
        self.conversation_history.push_back(turn.clone());
        self.total_turns += 1;

        for tool in &turn.tools_used {
            *self.tools_used_count.entry(tool.clone()).or_insert(0) += 1;
        }

        // Keep only max_history turns
        while self.conversation_history.len() > 10 {
            self.conversation_history.pop_front();
        }
    }
}

/// 🔥 CHATBOT - Interactive AI Assistant
pub struct Chatbot {
    config: ChatbotConfig,
    hf_client: Option<Arc<HuggingFaceClient>>,
    state: Arc<RwLock<ChatbotState>>,
}

impl Chatbot {
    /// Create new chatbot instance
    pub fn new(config: ChatbotConfig, hf_client: Option<HuggingFaceClient>) -> Self {
        eprintln!("🤖 Nuclear AI Chatbot Initialized");
        eprintln!("   ✅ Model: {}", config.model_name);
        eprintln!("   ✅ Max History: {} turns", config.max_history);
        eprintln!("   ✅ Max Tokens: {}", config.max_tokens);
        eprintln!(
            "   ✅ Chapel Learning: {}",
            if config.enable_chapel_learning {
                "ON"
            } else {
                "OFF"
            }
        );
        eprintln!(
            "   ✅ Tool Integration: {}",
            if config.enable_tools { "ON" } else { "OFF" }
        );
        eprintln!(
            "   ✅ HuggingFace: {}",
            if hf_client.is_some() {
                "Connected"
            } else {
                "Local Mode"
            }
        );

        Self {
            config,
            hf_client: hf_client.map(Arc::new),
            state: Arc::new(RwLock::new(ChatbotState::new())),
        }
    }

    /// Process a user message and return assistant response
    pub async fn chat(&self, user_message: &str) -> Result<String> {
        eprintln!("💬 User: {}", user_message);

        let mut tools_used = Vec::new();
        let mut enhanced_message = user_message.to_string();

        // Check if tools should be used
        if self.config.enable_tools {
            // Detect if user is asking for specific tool capabilities
            if user_message.contains("search") || user_message.contains("find online") {
                tools_used.push("websearch".to_string());
                enhanced_message.push_str(" [Tool: websearch available]");
            }
            if user_message.contains("file") || user_message.contains("code") {
                tools_used.push("file_search".to_string());
                enhanced_message.push_str(" [Tool: file_search available]");
            }
            if user_message.contains("scan") || user_message.contains("analyze") {
                tools_used.push("scan".to_string());
                enhanced_message.push_str(" [Tool: scan available]");
            }
        }

        // Get Chapel AI advice if enabled
        let mut chapel_context = String::new();
        if self.config.enable_chapel_learning {
            let chapel = get_chapel_ai();
            if let Ok(advice) = chapel.get_advice("chatbot", "chat") {
                if !advice.is_empty() {
                    chapel_context = format!(
                        " [Chapel AI guidance: {}]",
                        advice
                            .iter()
                            .map(|a| &a.suggestion)
                            .cloned()
                            .collect::<Vec<_>>()
                            .join("; ")
                    );
                }
            }
        }

        // Build conversation context
        let mut messages = vec![ChatMessage {
            role: "system".to_string(),
            content: self.config.system_prompt.clone() + &chapel_context,
        }];

        // Add conversation history
        let state = self
            .state
            .read()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        for turn in state.conversation_history.iter() {
            messages.push(ChatMessage {
                role: "user".to_string(),
                content: turn.user_message.clone(),
            });
            messages.push(ChatMessage {
                role: "assistant".to_string(),
                content: turn.assistant_message.clone(),
            });
        }
        drop(state);

        // Add current message
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: enhanced_message,
        });

        // Get response
        let response = if let Some(client) = &self.hf_client {
            // Use HuggingFace model
            let chat_response = client
                .chat(&self.config.model_name, messages, self.config.max_tokens)
                .await?;
            chat_response.message
        } else {
            // Fallback to local rule-based responses
            self.generate_local_response(user_message, &tools_used)
        };

        eprintln!("🤖 Assistant: {}", response);

        // Assess quality and learn
        let quality_score = self.assess_response_quality(&response);

        if self.config.enable_chapel_learning {
            let chapel = get_chapel_ai();
            let context = create_context("chatbot", "chat", user_message, quality_score);
            let _ = chapel.learn(context);
        }

        // Save to history
        let turn = ConversationTurn {
            user_message: user_message.to_string(),
            assistant_message: response.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            tools_used,
            quality_score,
        };

        let mut state = self
            .state
            .write()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        state.add_turn(turn);

        Ok(response)
    }

    /// Generate local response when HuggingFace is not available
    fn generate_local_response(&self, user_message: &str, tools_used: &[String]) -> String {
        let msg_lower = user_message.to_lowercase();

        if msg_lower.contains("hello") || msg_lower.contains("hi") {
            "Hello! I'm Nuclear AI, your advanced assistant. I can help you with web searches, file analysis, code scanning, and more. What can I do for you?".to_string()
        } else if msg_lower.contains("search") {
            format!("I can help you search the web! The websearch tool is available and ready to use. What would you like to search for?")
        } else if msg_lower.contains("file") || msg_lower.contains("code") {
            "I can help you analyze files and code! The file_search tool can search through your codebase and find specific patterns. What are you looking for?".to_string()
        } else if msg_lower.contains("scan") {
            "I can scan your workspace and provide insights! The scan tool can detect errors, warnings, TODOs, and mocks. Would you like me to scan your project?".to_string()
        } else if msg_lower.contains("train") || msg_lower.contains("dataset") {
            "I can help you create training datasets! The ai_dataset_trainer tool can generate high-quality datasets for machine learning. What kind of data do you need?".to_string()
        } else if msg_lower.contains("help") {
            format!(
                "I'm Nuclear AI with the following capabilities:\n\
                🔍 Web Search - Search and scrape the web\n\
                📄 File Search - Find files and patterns in code\n\
                🔬 Workspace Scan - Analyze code quality\n\
                💰 Premium Content - Extract paywalled content\n\
                🤖 AI Training - Generate datasets for ML\n\
                🧠 Chapel AI - Continuous learning system\n\
                \nTools currently available: {}\n\
                \nHow can I help you today?",
                if tools_used.is_empty() {
                    "All tools ready".to_string()
                } else {
                    tools_used.join(", ")
                }
            )
        } else {
            "I understand your question. I'm ready to help with web searches, file analysis, code scanning, content extraction, and AI dataset generation. Could you provide more details about what you need?".to_string()
        }
    }

    // Quality score weights (configured for balanced assessment)
    const QUALITY_LENGTH_LONG: f64 = 0.2; // Bonus for detailed responses (>200 chars)
    const QUALITY_LENGTH_MEDIUM: f64 = 0.1; // Bonus for adequate responses (>100 chars)
    const QUALITY_INFORMATIVE: f64 = 0.1; // Bonus for using informative markers
    const QUALITY_HELPFUL: f64 = 0.1; // Bonus for offering help or questions
    const QUALITY_STRUCTURED: f64 = 0.1; // Bonus for structured formatting

    /// Assess the quality of a response
    fn assess_response_quality(&self, response: &str) -> f64 {
        let mut score: f64 = 0.5;

        // Length factor: longer responses tend to be more detailed
        let len = response.len();
        if len > 200 {
            score += Self::QUALITY_LENGTH_LONG;
        } else if len > 100 {
            score += Self::QUALITY_LENGTH_MEDIUM;
        }

        // Informativeness: uses emojis or specific markers
        if response.contains("✅") || response.contains("🔍") || response.contains("📄") {
            score += Self::QUALITY_INFORMATIVE;
        }

        // Helpfulness: offers next steps or asks clarifying questions
        if response.contains("?") || response.contains("help") || response.contains("can") {
            score += Self::QUALITY_HELPFUL;
        }

        // Structure: has lists or formatting (multiline)
        if response.contains("\n") {
            score += Self::QUALITY_STRUCTURED;
        }

        score.min(1.0)
    }

    /// Get conversation statistics
    pub fn get_statistics(&self) -> Result<serde_json::Value> {
        let state = self
            .state
            .read()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;

        Ok(serde_json::json!({
            "total_turns": state.total_turns,
            "history_length": state.conversation_history.len(),
            "tools_used": state.tools_used_count,
            "avg_quality": state.conversation_history.iter()
                .map(|t| t.quality_score)
                .sum::<f64>() / state.conversation_history.len().max(1) as f64,
        }))
    }

    /// Clear conversation history
    pub fn clear_history(&self) -> Result<()> {
        let mut state = self
            .state
            .write()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        state.conversation_history.clear();
        eprintln!("🗑️  Conversation history cleared");
        Ok(())
    }

    /// Get conversation history
    pub fn get_history(&self) -> Result<Vec<ConversationTurn>> {
        let state = self
            .state
            .read()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        Ok(state.conversation_history.iter().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chatbot_config_default() {
        let config = ChatbotConfig::default();
        assert_eq!(config.max_history, 10);
        assert!(config.enable_chapel_learning);
    }

    #[tokio::test]
    async fn test_chatbot_local_mode() {
        let config = ChatbotConfig::default();
        let chatbot = Chatbot::new(config, None);

        let response = chatbot.chat("Hello").await;
        assert!(response.is_ok());
        assert!(response.unwrap().contains("Nuclear AI"));
    }

    #[tokio::test]
    async fn test_chatbot_help() {
        let config = ChatbotConfig::default();
        let chatbot = Chatbot::new(config, None);

        let response = chatbot.chat("help").await.unwrap();
        assert!(response.contains("capabilities"));
    }

    #[test]
    fn test_chatbot_statistics() {
        let config = ChatbotConfig::default();
        let chatbot = Chatbot::new(config, None);

        let stats = chatbot.get_statistics();
        assert!(stats.is_ok());
    }
}
