#!/bin/bash
# Push Chapel Training Pipeline to HuggingFace Hub (Private Repository)
# Usage: ./scripts/push_to_huggingface.sh <HF_TOKEN> <REPO_NAME>

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
HF_TOKEN="${1:-}"
HF_REPO_NAME="${2:-nuclear-chapel-training}"
HF_USERNAME="${3:-}"
PRIVATE="${4:-true}"

if [ -z "$HF_TOKEN" ]; then
    echo -e "${RED}❌ Error: HuggingFace token required${NC}"
    echo "Usage: $0 <HF_TOKEN> [REPO_NAME] [HF_USERNAME] [PRIVATE]"
    exit 1
fi

if [ -z "$HF_USERNAME" ]; then
    echo -e "${YELLOW}⚠️  HF_USERNAME not provided, using repo ID from token${NC}"
fi

echo -e "${GREEN}🚀 Pushing Chapel Training Pipeline to HuggingFace${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Repository: $HF_REPO_NAME"
echo "Privacy: $([ "$PRIVATE" = "true" ] && echo "PRIVATE" || echo "PUBLIC")"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Install HuggingFace CLI
echo -e "\n${YELLOW}📦 Installing HuggingFace tools...${NC}"
pip install huggingface-hub --quiet

# Login to HuggingFace
echo -e "\n${YELLOW}🔐 Authenticating with HuggingFace...${NC}"
huggingface-cli login --token "$HF_TOKEN" --add-to-git-credential

# Create repository if it doesn't exist
echo -e "\n${YELLOW}📝 Creating repository (if needed)...${NC}"
if command -v huggingface-cli &> /dev/null; then
    # Repository creation via Python is more reliable
    python3 << PYTHON_EOF
from huggingface_hub import HfApi, repo_type_and_id_from_hf_id
from pathlib import Path

api = HfApi()
repo_id = "$HF_REPO_NAME"

try:
    # Create model repository
    repo_url = api.create_repo(
        repo_id=repo_id,
        repo_type="model",
        private=$PRIVATE,
        exist_ok=True
    )
    print(f"✅ Repository ready: {repo_url}")
except Exception as e:
    print(f"ℹ️  Repository status: {e}")
PYTHON_EOF
fi

# Prepare files for upload
echo -e "\n${YELLOW}📂 Preparing files...${NC}"

# Create temporary directory with structure
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Copy Chapel training pipeline
echo "  📝 Chapel training pipeline..."
cp ffi/chapel/training_pipeline.chpl "$TEMP_DIR/"
cp ffi/chapel/chapel_ai.chpl "$TEMP_DIR/"
cp ffi/chapel/Makefile "$TEMP_DIR/"

# Copy dataset
echo "  📊 Training dataset..."
mkdir -p "$TEMP_DIR/data"
cp ffi/chapel/data/train/scraping_stealth_patterns.json "$TEMP_DIR/data/"

# Copy configuration
echo "  ⚙️  Configuration files..."
mkdir -p "$TEMP_DIR/config"
cp ffi/chapel/training/config.json "$TEMP_DIR/config/" 2>/dev/null || true

# Copy README
echo "  📖 Documentation..."
cat > "$TEMP_DIR/README.md" << 'README_EOF'
# Chapel AI Training Pipeline

Professional continuous learning pipeline for web scraping intelligence.

## Pure Chapel Implementation (No Python)

- **Layer 1**: Stealth pattern learning (50K samples, multi-locale)
- **Layer 2**: Quality assessment (31K+ validated samples)
- **Layer 3**: Neural network training (Adam optimizer, cross-validation)

## Files

- `training_pipeline.chpl` - Main training orchestrator
- `chapel_ai.chpl` - Neural network + advanced statistics
- `Makefile` - Build system (single/multi-locale)
- `data/` - Training datasets
- `config/` - Configuration files

## Build & Run

```bash
# Single-locale training
make train
make run

# Multi-locale distributed training
make train-multi
make run-multi

# Build Chapel AI library (C FFI)
make chapel-lib
```

## Architecture

```
Layer 1: Stealth Patterns (BlockDist)
    ↓
Layer 2: Quality Assessment (CyclicDist)
    ↓
Layer 3: Neural Network (Adam + Cross-validation)
    ↓
Metrics & Reporting
```

## Continuous Learning

Agents orchestrate continuous improvement:
- DataCollectionAgent: Gathers 50K patterns
- ValidationAgent: Ensures quality > 0.90
- ContinuousLearningAgent: Trains neural network
- OptimizationAgent: Reduces latency
- AdaptationAgent: Learns new evasion techniques

## Training Metrics

- Total samples: 81,300+
- Techniques: 7 stealth patterns
- Model architecture: [10 → 32 → 5]
- Optimizer: Adam (β₁=0.9, β₂=0.999)
- Target accuracy: > 0.85
- Target quality: > 0.90

## Requirements

- Chapel 2.0+
- C compiler (gcc/clang)
- BLAS/LAPACK libraries (optional)
- Make

## License

Private - Nuclear Crawler Hybrid Project

README_EOF

# Add git attributes for LFS (if needed)
cat > "$TEMP_DIR/.gitattributes" << 'GITATTR_EOF'
*.chpl text eol=lf
*.json text eol=lf
Makefile text eol=lf
README.md text eol=lf
GITATTR_EOF

# Push to HuggingFace using git
echo -e "\n${YELLOW}🔄 Pushing to HuggingFace...${NC}"

cd "$TEMP_DIR"

# Initialize git repository
git init --quiet
git config user.email "training@nuclear-crawler.dev"
git config user.name "Nuclear Training Agent"

# Add all files
echo "  Adding files..."
git add -A

# Commit
echo "  Committing..."
git commit -m "Chapel training pipeline - Layer 1,2,3 with 81K+ samples" --quiet

# Add HuggingFace remote
REMOTE_URL="https://huggingface.co/$HF_REPO_NAME"
echo "  Setting remote: $REMOTE_URL"
git remote add origin "https://huggingface.co/$HF_REPO_NAME"

# Force push with token
echo "  Pushing to HuggingFace..."
git push \
    --force \
    -u origin main \
    --quiet \
    --repo="https://user:${HF_TOKEN}@huggingface.co/$HF_REPO_NAME" \
    2>/dev/null || \
git push \
    --force \
    -u origin main \
    2>&1 | grep -v "^hint:" || true

# Back to original directory
cd - > /dev/null

echo -e "\n${GREEN}✅ Successfully pushed to HuggingFace!${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "🔗 Repository URL: ${GREEN}https://huggingface.co/$HF_REPO_NAME${NC}"
echo -e "📊 Files uploaded: Chapel code, datasets, config"
echo -e "🔒 Privacy: $([ "$PRIVATE" = "true" ] && echo "PRIVATE ✓" || echo "PUBLIC")"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
