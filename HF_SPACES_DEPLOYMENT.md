# HuggingFace Spaces Deployment Guide

## Quick Deploy to HF Spaces

### Option 1: Direct Upload (Recommended for now)

1. Go to: https://huggingface.co/new-space
2. Create space:
   - Name: `nuclear-chapel-ai`
   - License: MIT
   - Space SDK: Docker

3. In the created space, upload files:
   ```bash
   # Clone the space
   git clone https://huggingface.co/spaces/YOUR_USERNAME/nuclear-chapel-ai
   cd nuclear-chapel-ai
   
   # Copy files
   cp -r ../nuclear-crawler-hybrid/ffi/chapel/* .
   
   # Commit
   git add .
   git commit -m "Add Nuclear Chapel AI"
   git push
   ```

### Option 2: GitHub to Spaces Sync

1. Create workflow in `.github/workflows/sync-to-spaces.yml`
2. On push to main, automatically sync chapel files to HF Spaces
3. Space automatically redeploys

### Option 3: Docker Direct

```bash
# Build image with Chapel
docker build -f Dockerfile.chapel -t nuclear-chapel-ai .

# Push to HF
docker tag nuclear-chapel-ai:latest registry.huggingface.co/spaces/YOUR_USERNAME/nuclear-chapel-ai/latest
docker push registry.huggingface.co/spaces/YOUR_USERNAME/nuclear-chapel-ai/latest
```

## Dockerfile for HF Spaces

Create `Dockerfile.chapel`:

```dockerfile
FROM ghcr.io/chapel/chapel:latest

WORKDIR /app

# Copy Chapel AI files
COPY ffi/chapel/ /app/

# Build all systems
RUN cd /app && make full-pipeline

# Expose port
EXPOSE 7860

# Run gradio interface (if created)
CMD ["python3", "app.py"]
```

## Gradio Interface

Create `app.py` for interactive web UI:

```python
import gradio as gr
import subprocess
import json

def detect_fake_news(content, source):
    """Detect fake information"""
    # Call Chapel AI via subprocess
    result = subprocess.run([
        "./bin/unified_nuclear_ai", 
        "--analyze", content, source
    ], capture_output=True, text=True)
    return result.stdout

def scientific_analysis(hypothesis, evidence):
    """Perform scientific analysis"""
    result = subprocess.run([
        "./bin/unified_nuclear_ai",
        "--science", hypothesis, evidence
    ], capture_output=True, text=True)
    return result.stdout

# Gradio Interface
with gr.Blocks(title="🧠 Nuclear Chapel AI") as demo:
    gr.Markdown("""
    # 🧠 Nuclear Chapel AI
    
    Pure Chapel implementation of distributed AI with:
    - Fake news detection
    - Scientific analysis
    - Parallel search
    - Neural networks
    """)
    
    with gr.Tabs():
        with gr.Tab("Fake Detection"):
            content = gr.Textbox(label="Content to analyze")
            source = gr.Textbox(label="Source")
            result = gr.Textbox(label="Result", interactive=False)
            gr.Button("Analyze").click(detect_fake_news, [content, source], result)
        
        with gr.Tab("Scientific Analysis"):
            hypothesis = gr.Textbox(label="Hypothesis")
            evidence = gr.Textbox(label="Evidence")
            sci_result = gr.Textbox(label="Analysis", interactive=False)
            gr.Button("Analyze").click(scientific_analysis, [hypothesis, evidence], sci_result)

if __name__ == "__main__":
    demo.launch(server_name="0.0.0.0", server_port=7860)
```

## Files Structure for HF Spaces

```
nuclear-chapel-ai/  (HF Space)
├── Dockerfile              # Chapel-based build
├── README.md              # HF Spaces README
├── app.py                 # Gradio interface
├── ai/
│   ├── nuclear_chapel_ai.chpl
│   └── unified_nuclear_ai.chpl
├── tools/
│   ├── code_analyzer.chpl
│   ├── code_repair.chpl
│   └── code_reviewer.chpl
├── training/
│   ├── training_pipeline.chpl
│   ├── data_mining.chpl
│   └── analysis.chpl
└── Makefile
```

## Deployment Steps

1. **Create HF Space**
   - Go to https://huggingface.co/new-space
   - Select Docker SDK
   - Create space

2. **Push Code**
   ```bash
   git clone https://huggingface.co/spaces/YOUR_USERNAME/nuclear-chapel-ai
   cd nuclear-chapel-ai
   cp -r /path/to/chapel/* .
   git add .
   git commit -m "Initial Chapel AI deployment"
   git push
   ```

3. **HF Spaces automatically:**
   - Builds Dockerfile
   - Runs `docker build`
   - Deploys container
   - Exposes at `https://huggingface.co/spaces/YOUR_USERNAME/nuclear-chapel-ai`

4. **Monitor**
   - Check logs in Space settings
   - View container status
   - Test interface at public URL

## Performance Optimization

For HF Spaces with limited resources:

1. Use Python wrapper around Chapel (faster startup)
2. Pre-compile Chapel binaries
3. Use GPU if available (--gpu flag)
4. Cache results in Redis/SQLite

## Next Steps

1. Create `app.py` with Gradio interface
2. Build Dockerfile with Chapel
3. Push to HF Spaces
4. Test live deployment
5. Add documentation to Space README

## References

- HF Spaces: https://huggingface.co/spaces
- Docker SDK: https://huggingface.co/docs/hub/spaces-sdks-docker
- Chapel on HF: https://huggingface.co/spaces?sdk=docker&sort=likes&language=chapel

---

**Status:** Ready for deployment  
**Next:** Create app.py and push to HF Spaces
