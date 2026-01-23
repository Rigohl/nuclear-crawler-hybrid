#!/usr/bin/env python3
"""
Training Nuclear Chapel AI with HuggingFace Datasets (No Downloads Required)

This script trains the model directly from HuggingFace datasets using streaming.
Perfect for Kaggle, Google Colab, or any cloud environment.

Usage:
    # Option 1: Train with AG News
    python train_with_huggingface_dataset.py --dataset ag_news --samples 50000

    # Option 2: Train with GLUE SST2
    python train_with_huggingface_dataset.py --dataset glue_sst2 --samples 50000

    # Option 3: Train with xCodeEval
    python train_with_huggingface_dataset.py --dataset xcodeeval --samples 10000
"""

import argparse
import numpy as np
import pickle
import time
from pathlib import Path
from typing import Tuple, Any

import datasets
from sklearn.neural_network import MLPClassifier
from sklearn.preprocessing import StandardScaler
from sklearn.feature_extraction.text import TfidfVectorizer


def load_ag_news(num_samples: int) -> Tuple[np.ndarray, np.ndarray]:
    """Load AG News dataset from HuggingFace (streaming)."""
    print(f"📰 Loading AG News ({num_samples} samples)...")
    data = datasets.load_dataset('fancyzhx/ag_news', split=f'train[:{num_samples}]', 
                                  streaming=False)
    
    texts = [item['text'][:500] for item in data]  # Limit text length
    labels = np.array([item['label'] for item in data])
    
    print(f"   Categories: World(0), Sports(1), Business(2), Sci/Tech(3)")
    print(f"   Unique labels: {np.unique(labels)}")
    
    return texts, labels


def load_glue_sst2(num_samples: int) -> Tuple[np.ndarray, np.ndarray]:
    """Load GLUE SST2 dataset from HuggingFace (sentiment classification)."""
    print(f"😊 Loading GLUE SST2 ({num_samples} samples)...")
    data = datasets.load_dataset('nyu-mll/glue', 'sst2', 
                                  split=f'train[:{num_samples}]', 
                                  streaming=False)
    
    texts = [item['sentence'] for item in data]
    labels = np.array([item['label'] for item in data])
    
    print(f"   Categories: Negative(0), Positive(1)")
    print(f"   Label distribution: {np.bincount(labels)}")
    
    return texts, labels


def load_xcodeeval(num_samples: int) -> Tuple[np.ndarray, np.ndarray]:
    """Load xCodeEval dataset from HuggingFace (code classification)."""
    print(f"💻 Loading xCodeEval ({num_samples} samples)...")
    data = datasets.load_dataset('NTU-NLP-sg/xCodeEval', 'tag_classification',
                                  split=f'train[:{num_samples}]',
                                  streaming=True)
    
    texts = []
    labels = []
    
    for i, item in enumerate(data):
        if i >= num_samples:
            break
        if 'code' in item and 'tags' in item:
            texts.append(item['code'][:500])
            labels.append(hash(str(item['tags'])) % 5)  # 5 synthetic classes
    
    labels = np.array(labels)
    print(f"   Categories: {len(np.unique(labels))} (code tags)")
    
    return texts, labels


def extract_features(texts: list, max_features: int = 10) -> Tuple[np.ndarray, Any]:
    """Extract features from texts using TF-IDF."""
    print(f"🔧 Extracting features ({max_features} features)...")
    
    vectorizer = TfidfVectorizer(
        max_features=max_features,
        analyzer='char',
        ngram_range=(2, 3),
        strip_accents='unicode'
    )
    
    X = vectorizer.fit_transform(texts).toarray()
    print(f"   Shape: {X.shape}")
    
    return X, vectorizer


def train_model(X: np.ndarray, y: np.ndarray, 
                hidden_layers: Tuple = (128, 64, 32)) -> MLPClassifier:
    """Train MLP classifier."""
    print(f"🚀 Training MLPClassifier (layers: {hidden_layers})...")
    
    # Normalize features
    scaler = StandardScaler()
    X_scaled = scaler.fit_transform(X)
    
    # Train
    start = time.time()
    model = MLPClassifier(
        hidden_layer_sizes=hidden_layers,
        max_iter=500,
        random_state=42,
        early_stopping=True,
        validation_fraction=0.2,
        verbose=1
    )
    model.fit(X_scaled, y)
    
    elapsed = time.time() - start
    accuracy = model.score(X_scaled, y)
    
    print(f"✅ Training completed in {elapsed:.0f}s")
    print(f"   Accuracy: {accuracy:.2%}")
    print(f"   Iterations: {model.n_iter_}")
    
    return model, scaler


def save_model(model: MLPClassifier, scaler: StandardScaler, 
               vectorizer: Any, output_path: str = 'nuclear_chapel_ai.pkl'):
    """Save trained model and preprocessors."""
    print(f"💾 Saving model to {output_path}...")
    
    with open(output_path, 'wb') as f:
        pickle.dump({
            'model': model,
            'scaler': scaler,
            'vectorizer': vectorizer,
            'metadata': {
                'accuracy': model.score(scaler.transform(vectorizer.transform(
                    ['test']).toarray()), np.array([0])),
                'timestamp': time.time()
            }
        }, f)
    
    file_size = Path(output_path).stat().st_size / (1024*1024)
    print(f"✅ Model saved ({file_size:.2f} MB)")


def main():
    parser = argparse.ArgumentParser(
        description='Train Nuclear Chapel AI with HuggingFace Datasets'
    )
    parser.add_argument('--dataset', default='ag_news',
                       choices=['ag_news', 'glue_sst2', 'xcodeeval'],
                       help='Dataset to use')
    parser.add_argument('--samples', type=int, default=50000,
                       help='Number of samples to train on')
    parser.add_argument('--output', default='nuclear_chapel_ai.pkl',
                       help='Output model path')
    
    args = parser.parse_args()
    
    print("=" * 60)
    print("🧬 Nuclear Chapel AI - HuggingFace Dataset Training")
    print("=" * 60)
    
    # Load dataset
    if args.dataset == 'ag_news':
        texts, labels = load_ag_news(args.samples)
    elif args.dataset == 'glue_sst2':
        texts, labels = load_glue_sst2(args.samples)
    elif args.dataset == 'xcodeeval':
        texts, labels = load_xcodeeval(args.samples)
    
    # Extract features
    X, vectorizer = extract_features(texts)
    
    # Train model
    model, scaler = train_model(X, labels)
    
    # Save
    save_model(model, scaler, vectorizer, args.output)
    
    print("=" * 60)
    print("🎉 Training pipeline complete!")
    print("=" * 60)


if __name__ == '__main__':
    main()
