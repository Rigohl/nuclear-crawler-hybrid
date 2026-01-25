# Chapel Nuclear Training - Hugging Face & GitHub Spaces Deployment

## 🚀 Hugging Face Model/Space Setup

### Option 1: Create a Hugging Face Model Repository

```bash
# 1. Initialize Hugging Face CLI (if not already done)
huggingface-cli login

# 2. Create new model repository
huggingface-cli repo create \
  --repo_id="nuclear-chapel-training" \
  --type="model" \
  --private

# 3. Clone the HF repo
git clone https://huggingface.co/Kimberlyindiva/nuclear-chapel-training
cd nuclear-chapel-training

# 4. Add Chapel files
cp -r /path/to/local/chapel/files .

# 5. Create model card
cat > README.md << 'EOF'
---
language:
  - chapel
tags:
  - nuclear
  - parallel-programming
  - ai
  - chapel
license: mit
---

# Nuclear Chapel Training

Advanced parallel programming framework for nuclear data analysis using Chapel.

## Installation

```bash
# Download from Hugging Face
git clone https://huggingface.co/Kimberlyindiva/nuclear-chapel-training
cd nuclear-chapel-training

# Compile Chapel files
chpl -o chapel_ai chapel_ai.chpl
chpl -o training_pipeline training_pipeline.chpl
chpl -o unified_ai ai/unified_nuclear_ai.chpl
```

## Files

- **chapel_ai.chpl** - Core AI system (35.5 KB)
- **training_pipeline.chpl** - Training pipeline (22.2 KB)
- **data_mining_engine.chpl** - Data mining capabilities (13.5 KB)
- **scientific_analysis.chpl** - Scientific analysis tools (14.9 KB)
- **ai/** - AI modules with unified models
- **tools/** - Code analysis, review, and repair tools
- **training/** - Training data and pipeline files

## Compilation

### Windows
```powershell
chpl -o chapel_ai chapel_ai.chpl
chpl -o training_pipeline training_pipeline.chpl
```

### Linux/macOS
```bash
chpl -o chapel_ai chapel_ai.chpl
chpl -o training_pipeline training_pipeline.chpl
```

## Usage

```bash
# Run main AI system
./chapel_ai [options]

# Run training pipeline
./training_pipeline --data data.csv

# Analyze code
./tools/code_analyzer --input source.chpl
```

## License

MIT License - See LICENSE file for details
EOF

# 6. Push to Hugging Face
git add .
git commit -m "Initial Chapel training model"
git push origin main
```

### Option 2: Create a Hugging Face Space (FastAPI/Gradio)

```bash
# 1. Create Space on Hugging Face
huggingface-cli repo create \
  --repo_id="nuclear-chapel-space" \
  --type="space" \
  --space_sdk="docker"

# 2. Push Docker-based Space
git clone https://huggingface.co/spaces/Kimberlyindiva/nuclear-chapel-space
cd nuclear-chapel-space

# 3. Create Dockerfile
cat > Dockerfile << 'EOF'
FROM chocoteam/choco-base:latest

# Install Chapel
RUN choco install chapel -y

# Copy Chapel files
COPY ./chapel_files /app/chapel
WORKDIR /app/chapel

# Compile
RUN chpl -o chapel_ai chapel_ai.chpl
RUN chpl -o training_pipeline training_pipeline.chpl

# Expose port for API
EXPOSE 7860

# Create Gradio interface
COPY app.py /app/
CMD ["python", "app.py"]
EOF

# 4. Create app.py (Gradio Interface)
cat > app.py << 'EOF'
import gradio as gr
import subprocess
import os

def run_chapel_analysis(input_file):
    """Run Chapel code analysis on uploaded file"""
    if not input_file:
        return "No file uploaded"
    
    result = subprocess.run(
        ["./tools/code_analyzer", "--input", input_file],
        capture_output=True,
        text=True
    )
    return result.stdout or result.stderr

def run_training(data_file):
    """Run Chapel training pipeline"""
    result = subprocess.run(
        ["./training_pipeline", "--data", data_file],
        capture_output=True,
        text=True
    )
    return result.stdout or result.stderr

# Create Gradio interface
with gr.Blocks() as demo:
    gr.Markdown("# Nuclear Chapel Training")
    gr.Markdown("Advanced parallel programming for nuclear analysis")
    
    with gr.Tab("Code Analysis"):
        file_input = gr.File(label="Upload Chapel code")
        analysis_output = gr.Textbox(label="Analysis Result", lines=10)
        analyze_btn = gr.Button("Analyze")
        analyze_btn.click(run_chapel_analysis, inputs=[file_input], outputs=[analysis_output])
    
    with gr.Tab("Training"):
        data_input = gr.File(label="Upload training data")
        training_output = gr.Textbox(label="Training Result", lines=10)
        train_btn = gr.Button("Train")
        train_btn.click(run_training, inputs=[data_input], outputs=[training_output])
    
    with gr.Tab("About"):
        gr.Markdown("""
        # Nuclear Chapel Training Framework
        
        - **Language**: Chapel (parallel programming)
        - **Purpose**: AI training and nuclear data analysis
        - **Components**:
          - AI System (chapel_ai.chpl)
          - Training Pipeline (training_pipeline.chpl)
          - Code Analysis Tools
          - Data Mining Engine
        """)

demo.launch()
EOF

# 5. Push to Hugging Face
git add .
git commit -m "Add Gradio interface for Chapel"
git push origin main
```

## GitHub Spaces / GitHub Codespaces Setup

### Option 1: GitHub Codespaces with devcontainer

```bash
# 1. Create .devcontainer/devcontainer.json
mkdir -p .devcontainer

cat > .devcontainer/devcontainer.json << 'EOF'
{
  "name": "Chapel Development",
  "image": "mcr.microsoft.com/devcontainers/universal:latest",
  "features": {
    "ghcr.io/devcontainers/features/github-cli:1": {}
  },
  "customizations": {
    "codespaces": {
      "openFiles": [
        "README.md",
        "chapel_ai.chpl"
      ]
    },
    "vscode": {
      "extensions": [
        "ms-vscode.cpptools",
        "charliermarsh.ruff",
        "GitHub.copilot"
      ]
    }
  },
  "postCreateCommand": "choco install chapel -y && chpl --version",
  "remoteUser": "codespace"
}
EOF

# 2. Commit and push
git add .devcontainer/devcontainer.json
git commit -m "Add Chapel Codespaces configuration"
git push
```

### Option 2: GitHub Pages with Chapel Web Interface

```bash
# 1. Create docs/ directory for GitHub Pages
mkdir -p docs

# 2. Create index.html with Chapel info
cat > docs/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Nuclear Chapel Training</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: #333;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            padding: 40px;
            border-radius: 10px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.3);
        }
        h1 { color: #667eea; }
        .file-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
            margin: 20px 0;
        }
        .file-card {
            background: #f5f5f5;
            padding: 20px;
            border-radius: 8px;
            border-left: 4px solid #667eea;
        }
        .file-card h3 { margin-top: 0; color: #667eea; }
        .file-card .size { color: #999; font-size: 0.9em; }
        code {
            background: #f0f0f0;
            padding: 10px;
            border-radius: 5px;
            display: block;
            margin: 10px 0;
            font-family: 'Courier New', monospace;
            overflow-x: auto;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🔬 Nuclear Chapel Training</h1>
        <p>Advanced parallel programming framework for nuclear data analysis using Chapel</p>
        
        <h2>📁 Project Files</h2>
        <div class="file-grid">
            <div class="file-card">
                <h3>chapel_ai.chpl</h3>
                <p>Core AI system for nuclear analysis</p>
                <p class="size">Size: 35.5 KB</p>
            </div>
            <div class="file-card">
                <h3>training_pipeline.chpl</h3>
                <p>Training pipeline for model development</p>
                <p class="size">Size: 22.2 KB</p>
            </div>
            <div class="file-card">
                <h3>unified_nuclear_ai.chpl</h3>
                <p>Unified AI model for nuclear applications</p>
                <p class="size">Size: 21.3 KB</p>
            </div>
            <div class="file-card">
                <h3>code_analyzer.chpl</h3>
                <p>Static code analysis tool</p>
                <p class="size">Size: 13.1 KB</p>
            </div>
            <div class="file-card">
                <h3>code_reviewer.chpl</h3>
                <p>Automated code review system</p>
                <p class="size">Size: 18.2 KB</p>
            </div>
            <div class="file-card">
                <h3>data_mining_engine.chpl</h3>
                <p>Parallel data mining capabilities</p>
                <p class="size">Size: 13.5 KB</p>
            </div>
        </div>

        <h2>🚀 Quick Start</h2>
        <code>
# Clone repository
git clone https://github.com/Kimberlyindiva/nuclear-chapel-training
cd nuclear-chapel-training

# Compile
chpl -o chapel_ai chapel_ai.chpl
chpl -o training_pipeline training_pipeline.chpl

# Run
./chapel_ai
./training_pipeline
        </code>

        <h2>📦 System Requirements</h2>
        <ul>
            <li><strong>Chapel Compiler</strong>: v2.0+ (Latest recommended)</li>
            <li><strong>Operating System</strong>: Linux, macOS, or Windows</li>
            <li><strong>RAM</strong>: 4GB minimum, 8GB+ recommended</li>
            <li><strong>Disk Space</strong>: 500MB for compilation</li>
        </ul>

        <h2>📚 Documentation</h2>
        <ul>
            <li><a href="CHAPELCOMPILATION_GUIDE.md">Compilation Guide</a></li>
            <li><a href="README.md">Main README</a></li>
            <li><a href="https://chapel-lang.org/docs/">Chapel Official Docs</a></li>
        </ul>

        <h2>🔗 Resources</h2>
        <ul>
            <li><a href="https://github.com/Kimberlyindiva/nuclear-chapel-training">GitHub Repository</a></li>
            <li><a href="https://huggingface.co/Kimberlyindiva/nuclear-chapel-training">Hugging Face Model</a></li>
            <li><a href="https://chapel-lang.org/">Chapel Language Homepage</a></li>
        </ul>
    </div>
</body>
</html>
EOF

# 3. Configure GitHub Pages in repository settings
# Settings -> Pages -> Build and deployment -> Source: Deploy from a branch
# Branch: main, Folder: /docs
```

## Deployment Checklist

### Hugging Face Deployment
- [ ] Create Hugging Face account at https://huggingface.co
- [ ] Install `huggingface_hub` library:
  ```bash
  pip install huggingface_hub
  ```
- [ ] Login to Hugging Face:
  ```bash
  huggingface-cli login
  ```
- [ ] Create model repository
- [ ] Upload Chapel files and configuration
- [ ] Submit Chapel files to Hub
- [ ] Enable discussions for community

### GitHub Spaces / Codespaces
- [ ] Create GitHub repository
- [ ] Add `.devcontainer/devcontainer.json`
- [ ] Create GitHub Codespaces setup
- [ ] Configure GitHub Pages (docs/)
- [ ] Enable Pages from `/docs` branch
- [ ] Verify website loads correctly

### Verification Steps
```bash
# Test Hugging Face upload
huggingface-cli repo info Kimberlyindiva/nuclear-chapel-training

# Test GitHub Pages
curl -I https://Kimberlyindiva.github.io/nuclear-chapel-training/

# Test Codespaces (after creation)
# Open GitHub Codespaces and verify Chapel compiles
```

## Next Steps

1. **Install Chapel on your system** if not already done
2. **Compile all `.chpl` files** locally to verify they work
3. **Create Hugging Face model repository**
4. **Create GitHub repository** with deployment configurations
5. **Push all files** to both platforms
6. **Test deployments** and verify everything works
7. **Create documentation** on GitHub Pages
8. **Share with community** through Hugging Face Hub and GitHub

## Support

For issues or questions:
- Chapel Documentation: https://chapel-lang.org/docs/
- GitHub Issues: Create in your repository
- Hugging Face Hub: Use discussion forum

---

Generated: 24 Jan 2026
Status: Ready for deployment
