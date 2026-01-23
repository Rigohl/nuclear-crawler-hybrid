# 🤗 HuggingFace Integration Guide

Nuclear Crawler Hybrid now includes full integration with HuggingFace for AI model training and deployment.

## Features

- **Dataset Upload**: Upload generated datasets to HuggingFace Hub
- **Model Fine-tuning**: Fine-tune models on custom datasets
- **Inference API**: Use HuggingFace models for chatbot and other tasks
- **Chapel AI Export**: Export Chapel AI learning data for model training

## Setup

### 1. Install HuggingFace CLI (Optional)

```bash
pip install huggingface_hub
```

### 2. Set Environment Variables

```bash
export HF_TOKEN="your_huggingface_token_here"
export HF_USERNAME="your_username"
```

Get your token from: https://huggingface.co/settings/tokens

### 3. Configuration

```rust
use nuclear_crawler_hybrid::HuggingFaceConfig;

let config = HuggingFaceConfig {
    api_token: "hf_...".to_string(),
    username: "your_username".to_string(),
    ..Default::default()
};
```

## Usage Examples

### Upload Dataset

```rust
use nuclear_crawler_hybrid::{HuggingFaceClient, HuggingFaceConfig};
use nuclear_crawler_hybrid::mcp::tools::AIDatasetTrainerTool;

#[tokio::main]
async fn main() -> Result<()> {
    // Create HuggingFace client
    let config = HuggingFaceConfig::default();
    let client = HuggingFaceClient::new(config)?;
    
    // Generate dataset
    let trainer_config = DatasetTrainerConfig::default();
    let trainer = AIDatasetTrainerTool::new(trainer_config).await?;
    let dataset = trainer.generate_dataset(sources).await?;
    
    // Export to HuggingFace format
    let jsonl = format_dataset_for_hf(&dataset.datapoints)?;
    
    // Upload
    let metadata = HFDatasetMetadata {
        name: "my-dataset".to_string(),
        description: "Training dataset for Nuclear AI".to_string(),
        license: "MIT".to_string(),
        tags: vec!["ml".to_string(), "nlp".to_string()],
        size: dataset.datapoints.len(),
        splits: HashMap::new(),
    };
    
    let repo_url = client.upload_dataset("my-dataset", &jsonl, metadata).await?;
    println!("Dataset uploaded: {}", repo_url);
    
    Ok(())
}
```

### Fine-tune Model

```rust
use nuclear_crawler_hybrid::{HuggingFaceClient, TrainingConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let client = HuggingFaceClient::new(HuggingFaceConfig::default())?;
    
    let training_config = TrainingConfig {
        learning_rate: 2e-5,
        num_epochs: 3,
        batch_size: 8,
        ..Default::default()
    };
    
    let model_url = client.fine_tune_model(
        "mistralai/Mistral-7B-v0.1",
        "my-dataset",
        "my-finetuned-model",
        training_config,
    ).await?;
    
    println!("Model fine-tuning initiated: {}", model_url);
    Ok(())
}
```

### Export Chapel AI Learning Data

```rust
use nuclear_crawler_hybrid::chapel_integration::get_chapel_ai;

fn export_chapel_learning() -> Result<()> {
    let chapel = get_chapel_ai();
    
    // Export to HuggingFace format
    let training_data = chapel.export_for_huggingface()?;
    
    // Save to file
    let json = serde_json::to_string_pretty(&training_data)?;
    std::fs::write("chapel_training_data.jsonl", json)?;
    
    println!("Exported {} training examples", training_data.len());
    Ok(())
}
```

## MCP Tool Integration

The HuggingFace integration works seamlessly with the chatbot MCP tool:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "chatbot",
    "arguments": {
      "message": "Hello, how can you help me?",
      "model": "mistralai/Mistral-7B-Instruct-v0.2",
      "max_tokens": 512,
      "enable_learning": true
    }
  }
}
```

## Best Practices

1. **Token Security**: Never commit your HF_TOKEN to version control
2. **Dataset Quality**: Use the ai_dataset_trainer tool with proper quality thresholds
3. **Model Selection**: Choose base models appropriate for your task
4. **Learning Export**: Regularly export Chapel AI learning data for continuous improvement

## Troubleshooting

### "HuggingFace API token not set"

Make sure to set the `HF_TOKEN` environment variable:
```bash
export HF_TOKEN="hf_..."
```

### Rate Limiting

HuggingFace API has rate limits. For high-volume usage, consider:
- Using a Pro account
- Implementing exponential backoff
- Caching responses

### Model Loading Errors

If models fail to load:
1. Check model name is correct
2. Verify you have access to the model
3. Ensure model supports the inference API

## Resources

- [HuggingFace Hub Documentation](https://huggingface.co/docs/hub)
- [HuggingFace Inference API](https://huggingface.co/docs/api-inference)
- [Model Training Guide](https://huggingface.co/docs/transformers/training)
