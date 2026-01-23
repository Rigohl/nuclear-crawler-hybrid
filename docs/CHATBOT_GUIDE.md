# 🤖 Chatbot Guide

Nuclear Crawler Hybrid includes an intelligent chatbot powered by Chapel AI and optional HuggingFace models.

## Features

- **Conversational Interface**: Natural language interaction
- **Tool Integration**: Access to all 5 MCP tools
- **Chapel AI Learning**: Learns from every conversation
- **Context Awareness**: Maintains conversation history
- **Multiple Models**: Support for HuggingFace models or local mode

## Quick Start

### Using the MCP Tool

Call the chatbot tool via the MCP protocol:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "chatbot",
    "arguments": {
      "message": "Hello! Can you help me search for React tutorials?"
    }
  }
}
```

### Using in Rust Code

```rust
use nuclear_crawler_hybrid::{Chatbot, ChatbotConfig};

#[tokio::main]
async fn main() -> Result<()> {
    // Create chatbot with default config
    let config = ChatbotConfig::default();
    let chatbot = Chatbot::new(config, None);
    
    // Chat
    let response = chatbot.chat("Hello!").await?;
    println!("Bot: {}", response);
    
    Ok(())
}
```

## Configuration

```rust
use nuclear_crawler_hybrid::ChatbotConfig;

let config = ChatbotConfig {
    model_name: "mistralai/Mistral-7B-Instruct-v0.2".to_string(),
    max_history: 10,
    max_tokens: 512,
    system_prompt: "You are Nuclear AI...".to_string(),
    enable_chapel_learning: true,
    enable_tools: true,
};
```

### Configuration Options

- **model_name**: HuggingFace model to use (default: Mistral-7B-Instruct)
- **max_history**: Number of conversation turns to remember (default: 10)
- **max_tokens**: Maximum tokens per response (default: 512)
- **system_prompt**: Initial system prompt for the model
- **enable_chapel_learning**: Enable Chapel AI learning (default: true)
- **enable_tools**: Enable tool integration (default: true)

## Tool Integration

The chatbot can automatically detect and suggest tools based on user requests:

### Web Search
```
User: "Find information about Rust async programming"
Bot: "I can help you search! [Tool: websearch available]"
```

### File Analysis
```
User: "Check my code for errors"
Bot: "I can analyze your files! [Tool: file_search available]"
```

### Workspace Scan
```
User: "Scan my project for issues"
Bot: "I'll scan your workspace! [Tool: scan available]"
```

## Chapel AI Learning

The chatbot learns from every interaction:

```rust
// Chapel AI automatically:
// 1. Tracks conversation quality
// 2. Identifies successful patterns
// 3. Provides suggestions for improvement
// 4. Exports data for model training

let chapel = get_chapel_ai();
let stats = chapel.get_statistics()?;
println!("Learning stats: {:?}", stats);
```

## Conversation Management

### Get History
```rust
let history = chatbot.get_history()?;
for turn in history {
    println!("User: {}", turn.user_message);
    println!("Bot: {}", turn.assistant_message);
}
```

### Clear History
```rust
chatbot.clear_history()?;
```

### Get Statistics
```rust
let stats = chatbot.get_statistics()?;
println!("Total turns: {}", stats["total_turns"]);
println!("Average quality: {}", stats["avg_quality"]);
```

## HuggingFace Integration

### With HuggingFace Model

```rust
use nuclear_crawler_hybrid::{Chatbot, ChatbotConfig, HuggingFaceClient, HuggingFaceConfig};

let hf_config = HuggingFaceConfig::default();
let hf_client = HuggingFaceClient::new(hf_config)?;

let chatbot_config = ChatbotConfig {
    model_name: "meta-llama/Llama-2-7b-chat-hf".to_string(),
    ..Default::default()
};

let chatbot = Chatbot::new(chatbot_config, Some(hf_client));
```

### Local Mode (No HuggingFace)

If `HF_TOKEN` is not set, the chatbot runs in local mode with rule-based responses:

```rust
let chatbot = Chatbot::new(ChatbotConfig::default(), None);
// Works without HuggingFace API
```

## Advanced Usage

### Custom System Prompt

```rust
let config = ChatbotConfig {
    system_prompt: r#"You are a specialized Rust programming assistant.
You have deep knowledge of Rust, async programming, and web development.
You can access tools for web search, file analysis, and code scanning."#.to_string(),
    ..Default::default()
};
```

### Multi-turn Conversations

```rust
let chatbot = Chatbot::new(ChatbotConfig::default(), None);

// Turn 1
let response1 = chatbot.chat("What is Rust?").await?;
println!("Bot: {}", response1);

// Turn 2 (context preserved)
let response2 = chatbot.chat("Can you search for Rust tutorials?").await?;
println!("Bot: {}", response2);
```

## MCP Protocol Examples

### Simple Chat
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "chatbot",
    "arguments": {
      "message": "Hello!"
    }
  }
}
```

### With Model Selection
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "chatbot",
    "arguments": {
      "message": "Explain async Rust",
      "model": "codellama/CodeLlama-7b-Instruct-hf",
      "max_tokens": 1024
    }
  }
}
```

## Best Practices

1. **Set Clear Context**: Use descriptive system prompts
2. **Enable Learning**: Keep Chapel AI learning enabled for improvements
3. **Manage History**: Clear history when switching topics
4. **Choose Right Model**: Select models appropriate for your use case
5. **Monitor Quality**: Check conversation statistics regularly

## Supported Models

The chatbot works with any HuggingFace model that supports the Inference API:

- **General Chat**: Mistral-7B-Instruct, Llama-2-Chat
- **Code**: CodeLlama, StarCoder
- **Specialized**: Domain-specific fine-tuned models

## Troubleshooting

### "No response generated"
- Check HF_TOKEN is set correctly
- Verify model name is correct
- Ensure model supports inference API

### Poor Response Quality
- Try a different model
- Adjust max_tokens
- Improve system prompt
- Check conversation history length

### Memory Issues
- Reduce max_history
- Clear history more frequently
- Use smaller models

## Examples

See the `examples/` directory for:
- `chatbot_basic.rs` - Simple chatbot usage
- `chatbot_with_hf.rs` - HuggingFace integration
- `chatbot_tools.rs` - Tool integration examples
