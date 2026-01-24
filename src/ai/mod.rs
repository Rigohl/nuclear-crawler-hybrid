//! 🤖 AI SUITE - Chatbot & Model Integration

pub mod chatbot;
pub mod huggingface_integration;

pub use chatbot::{Chatbot, ChatbotConfig};
pub use huggingface_integration::{HuggingFaceClient, HuggingFaceConfig};
