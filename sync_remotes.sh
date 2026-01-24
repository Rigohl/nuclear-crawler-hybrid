#!/bin/bash
# Sync Script for Nuclear Crawler Hybrid (Chapel/JAX)
# Syncs to both GitHub and Hugging Face

# Configuration
GITHUB_REMOTE="origin"
HUGGINGFACE_REMOTE="huggingface"
HUGGINGFACE_URL="https://huggingface.co/spaces/YOUR_USERNAME/nuclear-crawler-hybrid" # User needs to edit this

echo "🚀 Starting Multi-Remote Sync for Nuclear Crawler (Chapel/JAX Edition)"

# 1. Check if Hugging Face remote exists
if ! git remote | grep -q "$HUGGINGFACE_REMOTE"; then
    echo "⚠️  Hugging Face remote not found. Adding it..."
    echo "Please edit this script to set your actual Hugging Face URL!"
    git remote add $HUGGINGFACE_REMOTE $HUGGINGFACE_URL
fi

# 2. Sync loop
echo "🔄 Pulling from GitHub..."
git pull $GITHUB_REMOTE main

echo "🔄 Pulling from Hugging Face..."
git pull $HUGGINGFACE_REMOTE main --allow-unrelated-histories

echo "✅ Code sync complete. Ready to push."
echo "   Run: git push $GITHUB_REMOTE main && git push $HUGGINGFACE_REMOTE main"
