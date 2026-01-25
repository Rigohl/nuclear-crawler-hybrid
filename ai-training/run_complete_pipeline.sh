#!/bin/bash
# Complete AI Training Pipeline
# Scraping → Curation → Training

set -e

echo "════════════════════════════════════════════════════════════════════"
echo "  COMPLETE AI TRAINING PIPELINE"
echo "  Scraping → Detection → Curation → Training"
echo "════════════════════════════════════════════════════════════════════"
echo ""

# Check Python
if ! command -v python &> /dev/null; then
    echo "❌ Python not found. Install Python 3.11+"
    exit 1
fi

echo "✅ Python $(python --version)"
echo ""

# FASE 1: SCRAPING
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📥 FASE 1: SCRAPING (15-20 minutos)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Reddit
if [ -f "scraping/output/reddit_marketing_real.json" ]; then
    echo "⏭️  Reddit data already exists, skipping..."
else
    echo "🔍 Scraping Reddit..."
    python scraping/reddit_scraper.py || echo "⚠️  Reddit scraping failed (API keys?)"
fi

# Twitter
if [ -f "scraping/output/twitter_marketers_real.json" ]; then
    echo "⏭️  Twitter data already exists, skipping..."
else
    echo "🔍 Scraping Twitter..."
    python scraping/twitter_scraper.py || echo "⚠️  Twitter scraping failed (API keys?)"
fi

# OSINT (opcional)
echo "🔍 OSINT scraping..."
python osint/osint_scraper.py || echo "⚠️  OSINT scraping failed"

echo ""
echo "✅ Scraping completo"
echo ""

# FASE 2: CURACIÓN
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 FASE 2: CURACIÓN (2-3 minutos)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "1. Detectando datos sintéticos..."
python curation/synthetic_detector.py

echo ""
echo "2. Curando dataset (filtering + ranking)..."
python curation/dataset_curator.py

echo ""
echo "✅ Curación completa"
echo ""

# FASE 3: TRAINING
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🧠 FASE 3: TRAINING (10 min - 2 horas)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if curated dataset exists
if [ ! -f "curation/output/curated_dataset_"*".json" ]; then
    echo "❌ No curated dataset found!"
    echo "   Run curation first: python curation/dataset_curator.py"
    exit 1
fi

CURATED_DATASET=$(ls -t curation/output/curated_dataset_*.json | head -1)
echo "📦 Using dataset: $CURATED_DATASET"
echo ""

echo "🚀 Training Qwen2.5-Coder with curated data..."
python training/fine_tune_qwen.py --dataset "$CURATED_DATASET"

echo ""
echo "✅ Training completo"
echo ""

# SUMMARY
echo "════════════════════════════════════════════════════════════════════"
echo "  ✅  PIPELINE COMPLETO"
echo "════════════════════════════════════════════════════════════════════"
echo ""
echo "Resultados:"
echo "  • Scraped data: scraping/output/"
echo "  • Curated dataset: $CURATED_DATASET"
echo "  • Trained model: models/qwen-marketing-real/final/"
echo ""
echo "Next steps:"
echo "  1. Test model: python test_chatbot.py"
echo "  2. Deploy: python deploy/chatbot_server.py"
echo "  3. Share: huggingface-cli upload models/qwen-marketing-real/final"
echo ""
echo "🎉 ¡Chatbot listo para producción!"
