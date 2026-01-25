# Chapel Deployment Workflow - HF + GitHub Spaces

## 🎯 Phase 1: Local Preparation

### Step 1.1: Verify Chapel Installation
```powershell
# Verify Chapel is installed
chpl --version

# Expected output: chpl version x.x.x (...)
```

### Step 1.2: Compile All Chapel Files
```powershell
cd D:\nuclear-chapel-training

# Compile main files
chpl -o chapel_ai chapel_ai.chpl
chpl -o training_pipeline training_pipeline.chpl
chpl -o data_mining data_mining_engine.chpl
chpl -o scientific_analysis scientific_analysis.chpl

# Compile AI modules
chpl -o unified_nuclear_ai ai/unified_nuclear_ai.chpl
chpl -o nuclear_chapel_ai ai/nuclear_chapel_ai.chpl

# Compile tools
chpl -o code_analyzer tools/code_analyzer.chpl
chpl -o code_reviewer tools/code_reviewer.chpl
chpl -o code_repair tools/code_repair.chpl

# Test executables
./chapel_ai --help
./training_pipeline --help
```

### Step 1.3: Prepare Deployment Files
```powershell
# Create deployment directory structure
mkdir deployment/huggingface
mkdir deployment/github-pages
mkdir deployment/docker

# Copy Chapel files
cp *.chpl deployment/huggingface/
cp -r ai deployment/huggingface/
cp -r tools deployment/huggingface/
cp -r training deployment/huggingface/
```

---

## 🎯 Phase 2: Hugging Face Deployment

### Step 2.1: Create HF Account & Repository

```bash
# 1. Go to https://huggingface.co/join
# 2. Create account (or login if exists)
# 3. Create new Model repository:
#    - Name: nuclear-chapel-training
#    - Description: Advanced parallel programming for nuclear analysis
#    - License: MIT
#    - Private: No (unless you want it private)
```

### Step 2.2: Configure Local Git for HF

```bash
# Install/upgrade huggingface_hub
pip install --upgrade huggingface_hub

# Login to Hugging Face
huggingface-cli login

# When prompted, generate token from:
# https://huggingface.co/settings/tokens
# Select: "repo" permission
```

### Step 2.3: Clone HF Repository

```bash
# Clone your HF model repository
git clone https://huggingface.co/Kimberlyindiva/nuclear-chapel-training
cd nuclear-chapel-training

# Configure git
git config user.email "your-email@example.com"
git config user.name "Your Name"
```

### Step 2.4: Upload Chapel Files

```bash
# Copy all Chapel files
cp -r /path/to/local/deployment/huggingface/* .

# Create/update README.md with model card
# (Use the README template from README_HF_DEPLOYMENT.md)

# Add all files
git add .

# Commit
git commit -m "Add Chapel training code and models"

# Push to Hugging Face
git push origin main

# Verify upload
# Go to: https://huggingface.co/Kimberlyindiva/nuclear-chapel-training
```

### Step 2.5: Create HF Space (Optional - Interactive UI)

```bash
# 1. On Hugging Face, create new Space:
#    - Name: nuclear-chapel-space
#    - Type: Docker
#    - Private: No

# 2. Clone the Space repository
git clone https://huggingface.co/spaces/Kimberlyindiva/nuclear-chapel-space
cd nuclear-chapel-space

# 3. Add Dockerfile (from README_HF_DEPLOYMENT.md - "Option 2")

# 4. Add app.py (Gradio interface)

# 5. Commit and push
git add .
git commit -m "Add Chapel interactive interface"
git push origin main

# 6. Space will auto-build and deploy!
# Verify at: https://huggingface.co/spaces/Kimberlyindiva/nuclear-chapel-space
```

---

## 🎯 Phase 3: GitHub Deployment

### Step 3.1: Create GitHub Repository

```bash
# 1. Go to https://github.com/new
# 2. Create repository:
#    - Name: nuclear-chapel-training
#    - Description: Advanced parallel programming for nuclear analysis
#    - Public: Yes (unless you want private)
#    - Add README: No (we'll add ours)
#    - License: MIT
```

### Step 3.2: Clone to Local

```bash
git clone https://github.com/Kimberlyindiva/nuclear-chapel-training.git
cd nuclear-chapel-training

# Configure git
git config user.email "your-email@example.com"
git config user.name "Your Name"
```

### Step 3.3: Add Chapel Files

```bash
# Copy Chapel source code
cp -r /path/to/local/chapel/files .

# Create .devcontainer/ for Codespaces
mkdir -p .devcontainer

# Add devcontainer.json (from README_HF_DEPLOYMENT.md - "Option 1")
# This enables GitHub Codespaces support
```

### Step 3.4: Configure GitHub Pages

```bash
# Create docs/ directory for GitHub Pages
mkdir -p docs

# Add index.html (from README_HF_DEPLOYMENT.md - "Option 2")

# Add CNAME (optional, for custom domain)
echo "nuclear-chapel.yourdomain.com" > docs/CNAME

# Commit everything
git add .
git commit -m "Initial Chapel repository with docs and Codespaces support"
git push origin main
```

### Step 3.5: Enable GitHub Pages

```
On GitHub.com:
1. Go to Settings → Pages
2. Build and deployment:
   - Source: Deploy from a branch
   - Branch: main
   - Folder: /docs
3. Click Save
4. Wait ~1 minute for deployment
5. Visit: https://Kimberlyindiva.github.io/nuclear-chapel-training/
```

### Step 3.6: Enable Discussions & Codespaces

```
On GitHub.com:
1. Settings → Features:
   ✓ Discussions (enable)
   ✓ Projects (enable)
   ✓ Wiki (optional)

2. Go to Code → Codespaces → Create on main branch
   (This will use .devcontainer/devcontainer.json)
```

---

## 📊 Deployment Verification

### Verify Hugging Face

```bash
# Check model repository
curl -s https://huggingface.co/api/models/Kimberlyindiva/nuclear-chapel-training | jq '.id'

# Check Space (if created)
curl -s https://huggingface.co/api/spaces/Kimberlyindiva/nuclear-chapel-space | jq '.status'
```

### Verify GitHub

```bash
# Check repository
curl -s https://api.github.com/repos/Kimberlyindiva/nuclear-chapel-training | jq '.name'

# Check Pages deployment
curl -I https://Kimberlyindiva.github.io/nuclear-chapel-training/
# Should return: HTTP/2 200

# Check Codespaces support
curl -s https://api.github.com/repos/Kimberlyindiva/nuclear-chapel-training/contents/.devcontainer/devcontainer.json | jq '.name'
```

---

## 🔄 Continuous Deployment

### Auto-Update Workflow (GitHub Actions)

Create `.github/workflows/sync-deployment.yml`:

```yaml
name: Sync Deployment

on:
  push:
    branches: [main]
  workflow_dispatch:

jobs:
  sync-huggingface:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Push to Hugging Face
        run: |
          git remote add huggingface https://Kimberlyindiva:${{ secrets.HF_TOKEN }}@huggingface.co/Kimberlyindiva/nuclear-chapel-training.git
          git push huggingface main

  build-documentation:
    runs-on: ubuntu-latest
    needs: sync-huggingface
    steps:
      - uses: actions/checkout@v3
      
      - name: Update docs
        run: |
          # Generate documentation
          mkdir -p docs
          cp CHAPELCOMPILATION_GUIDE.md docs/
          cp README_HF_DEPLOYMENT.md docs/
      
      - name: Commit and push
        run: |
          git config user.name "GitHub Actions"
          git config user.email "actions@github.com"
          git add docs/
          git commit -m "Update documentation" || true
          git push
```

### To Use Auto-Deployment:

```bash
# 1. Create GitHub PAT token
#    https://github.com/settings/tokens

# 2. Create HF API token
#    https://huggingface.co/settings/tokens

# 3. Add secrets to GitHub repository:
#    - Settings → Secrets → New repository secret
#    - Name: GH_TOKEN (your GitHub PAT)
#    - Name: HF_TOKEN (your HF token)

# 4. Commit .github/workflows/sync-deployment.yml

# After this, each push to main will auto-sync to both platforms!
```

---

## 📋 Quick Reference Checklist

### Before Deployment
- [ ] Chapel compiler installed and verified
- [ ] All `.chpl` files compile successfully locally
- [ ] All executables run without errors
- [ ] README.md created with instructions
- [ ] LICENSE file created (MIT)
- [ ] .devcontainer/devcontainer.json created
- [ ] docs/index.html created
- [ ] Makefile created for easy compilation

### Hugging Face Setup
- [ ] HF account created
- [ ] Model repository created
- [ ] Repository cloned locally
- [ ] All files uploaded
- [ ] model_index.json created (optional)
- [ ] README.md has model card
- [ ] Space created (optional)
- [ ] Verified at huggingface.co

### GitHub Setup
- [ ] GitHub account logged in
- [ ] Repository created
- [ ] Files pushed to main
- [ ] Pages enabled in Settings
- [ ] Docs deployed successfully
- [ ] Codespaces configured
- [ ] Discussions enabled
- [ ] Verified at github.io

### Post-Deployment
- [ ] Test GitHub Pages loads correctly
- [ ] Test Chapel code view on GitHub
- [ ] Create GitHub Codespaces and verify compilation
- [ ] Create HF Space and test interface
- [ ] Share repository links
- [ ] Add badges to README
- [ ] Enable CI/CD workflows

---

## 📱 Share Your Deployment

### Badge URLs

```markdown
# GitHub
[![GitHub](https://img.shields.io/badge/GitHub-Repository-blue?logo=github)](https://github.com/Kimberlyindiva/nuclear-chapel-training)

# Hugging Face
[![Hugging Face](https://img.shields.io/badge/Hugging%20Face-Model-ffd21e?logo=huggingface)](https://huggingface.co/Kimberlyindiva/nuclear-chapel-training)

# Chapel Language
[![Chapel](https://img.shields.io/badge/Language-Chapel-4f9fbe)](https://chapel-lang.org)

# License MIT
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
```

### Add to README:

```markdown
## 🔗 Links

- 🌐 **GitHub**: [nuclear-chapel-training](https://github.com/Kimberlyindiva/nuclear-chapel-training)
- 🤗 **Hugging Face**: [Model](https://huggingface.co/Kimberlyindiva/nuclear-chapel-training) | [Space](https://huggingface.co/spaces/Kimberlyindiva/nuclear-chapel-space)
- 📖 **Documentation**: [GitHub Pages](https://Kimberlyindiva.github.io/nuclear-chapel-training/)
- 📝 **Codespaces**: [Open in Codespaces](https://codespaces.new/Kimberlyindiva/nuclear-chapel-training)
```

---

## 🆘 Troubleshooting

### Hugging Face Push Failed
```bash
# Check credentials
huggingface-cli whoami

# Re-login if needed
huggingface-cli logout
huggingface-cli login

# Try push again
git push origin main
```

### GitHub Pages Not Loading
```bash
# Check Settings → Pages → Build & deployment
# Verify branch is "main" and folder is "/docs"
# Wait 1 minute after enabling
# Check: https://Kimberlyindiva.github.io/nuclear-chapel-training/
```

### Codespaces Build Failed
```bash
# Check .devcontainer/devcontainer.json syntax
# Verify all commands are valid
# Try rebuilding: Codespaces → Rebuild Container
```

### Chapel Compilation Errors in Space
```bash
# Update Dockerfile to use latest Chapel
# Use: FROM --platform=linux/amd64 ubuntu:22.04
# Add: RUN apt-get update && apt-get install -y chapel

# Or use choco: FROM chocoteam/choco-base:latest
#              RUN choco install chapel -y
```

---

Generated: 24 Jan 2026
Status: Ready for execution
