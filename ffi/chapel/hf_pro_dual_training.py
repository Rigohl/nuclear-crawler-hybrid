#!/usr/bin/env python3
"""
🚀 Nuclear Chapel DUAL TRAINING - OSINT + Marketing
Entrena ambos pipelines en paralelo/secuencial en HF Pro

Uso:
    python3 hf_pro_dual_training.py --mode sequential  # OSINT → Marketing (45 min total)
    python3 hf_pro_dual_training.py --mode parallel    # OSINT + Marketing simultáneamente
"""

import argparse
import logging
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import torch
from datasets import load_dataset
from transformers import (
    AutoTokenizer,
    AutoModelForTokenClassification,
    AutoModelForSequenceClassification,
    TrainingArguments,
    Trainer,
    TrainerCallback,
)

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class PrintCallback(TrainerCallback):
    """Print training progress"""
    def on_step_end(self, args, state, control, **kwargs):
        if state.global_step % 100 == 0:
            if hasattr(state, 'log_history') and state.log_history:
                logger.info(f"Step {state.global_step}: Loss = {state.log_history[-1]['loss']:.4f}")


def train_osint_ner_dual(num_samples: int = 50000) -> str:
    """Train OSINT NER model"""
    logger.info("🔍 [OSINT] Loading WikiANN dataset...")
    dataset = load_dataset(
        'unimelb-nlp/wikiann',
        'en',
        split=f'train[:{num_samples}]',
        trust_remote_code=True
    )
    
    logger.info(f"🔍 [OSINT] Loaded {len(dataset)} samples")
    
    tokenizer = AutoTokenizer.from_pretrained('bert-base-uncased')
    model = AutoModelForTokenClassification.from_pretrained(
        'bert-base-uncased',
        num_labels=9
    )
    
    def tokenize_and_align_labels(examples):
        tokenized_inputs = tokenizer(
            examples['tokens'],
            truncation=True,
            is_split_into_words=True,
            max_length=512,
            padding='max_length'
        )
        
        labels = []
        for i, label in enumerate(examples['ner_tags']):
            word_ids = tokenized_inputs.word_ids(batch_index=i)
            label_ids = []
            previous_word_idx = None
            for word_idx in word_ids:
                if word_idx is None:
                    label_ids.append(-100)
                elif word_idx != previous_word_idx:
                    label_ids.append(label[word_idx])
                else:
                    label_ids.append(-100)
                previous_word_idx = word_idx
            labels.append(label_ids)
        
        tokenized_inputs["labels"] = labels
        return tokenized_inputs
    
    logger.info("🔍 [OSINT] Tokenizing dataset...")
    tokenized_dataset = dataset.map(
        tokenize_and_align_labels,
        batched=True,
        desc="Tokenizing OSINT"
    )
    
    training_args = TrainingArguments(
        output_dir='./results_osint_dual',
        evaluation_strategy='no',
        learning_rate=2e-5,
        per_device_train_batch_size=8,
        num_train_epochs=2,  # Reducido para dual training
        weight_decay=0.01,
        push_to_hub=True,
        hub_model_id='Kimberlyindiva/nuclear-osint-ner',
        hub_strategy="every_save",
        save_steps=500,
        logging_steps=50,
    )
    
    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_dataset,
        callbacks=[PrintCallback()],
    )
    
    logger.info("🔍 [OSINT] Starting training...")
    trainer.train()
    trainer.push_to_hub()
    
    logger.info("✅ [OSINT] Training complete!")
    return "nuclear-osint-ner"


def train_marketing_sentiment_dual(num_samples: int = 50000) -> str:
    """Train Marketing Sentiment model"""
    logger.info("📈 [MARKETING] Loading Amazon Polarity dataset...")
    dataset = load_dataset(
        'mteb/amazon_polarity',
        split=f'train[:{num_samples}]',
        trust_remote_code=True
    )
    
    logger.info(f"📈 [MARKETING] Loaded {len(dataset)} samples")
    
    tokenizer = AutoTokenizer.from_pretrained('distilbert-base-uncased')
    model = AutoModelForSequenceClassification.from_pretrained(
        'distilbert-base-uncased',
        num_labels=2
    )
    
    def tokenize_function(examples):
        return tokenizer(
            examples['text'],
            padding='max_length',
            truncation=True,
            max_length=512
        )
    
    logger.info("📈 [MARKETING] Tokenizing dataset...")
    tokenized_dataset = dataset.map(
        tokenize_function,
        batched=True,
        desc="Tokenizing Marketing"
    )
    
    tokenized_dataset = tokenized_dataset.rename_column("label", "labels")
    
    training_args = TrainingArguments(
        output_dir='./results_marketing_dual',
        evaluation_strategy='no',
        learning_rate=2e-5,
        per_device_train_batch_size=16,
        num_train_epochs=2,  # Reducido para dual training
        weight_decay=0.01,
        push_to_hub=True,
        hub_model_id='Kimberlyindiva/nuclear-marketing-sentiment',
        hub_strategy="every_save",
        save_steps=500,
        logging_steps=50,
    )
    
    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_dataset,
        callbacks=[PrintCallback()],
    )
    
    logger.info("📈 [MARKETING] Starting training...")
    trainer.train()
    trainer.push_to_hub()
    
    logger.info("✅ [MARKETING] Training complete!")
    return "nuclear-marketing-sentiment"


def train_sequential(num_samples: int = 50000):
    """Train OSINT → Marketing secuencialmente"""
    logger.info("=" * 70)
    logger.info("🚀 SEQUENTIAL MODE: OSINT → Marketing")
    logger.info("=" * 70)
    
    # OSINT primero
    logger.info("\n[FASE 1/2] Entrenando OSINT NER...")
    osint_model = train_osint_ner_dual(num_samples)
    
    # Marketing segundo
    logger.info("\n[FASE 2/2] Entrenando Marketing Sentiment...")
    marketing_model = train_marketing_sentiment_dual(num_samples)
    
    logger.info("\n" + "=" * 70)
    logger.info("✅ DUAL TRAINING COMPLETE (Sequential)")
    logger.info("=" * 70)
    logger.info(f"Models trained:")
    logger.info(f"  1. {osint_model}")
    logger.info(f"  2. {marketing_model}")
    logger.info("All models pushed to HuggingFace Hub!")


def train_parallel(num_samples: int = 50000):
    """Train OSINT + Marketing en paralelo"""
    logger.info("=" * 70)
    logger.info("🚀 PARALLEL MODE: OSINT ⟷ Marketing (simultáneamente)")
    logger.info("=" * 70)
    
    with ThreadPoolExecutor(max_workers=2) as executor:
        future_osint = executor.submit(train_osint_ner_dual, num_samples)
        future_marketing = executor.submit(train_marketing_sentiment_dual, num_samples)
        
        osint_model = future_osint.result()
        marketing_model = future_marketing.result()
    
    logger.info("\n" + "=" * 70)
    logger.info("✅ DUAL TRAINING COMPLETE (Parallel)")
    logger.info("=" * 70)
    logger.info(f"Models trained:")
    logger.info(f"  1. {osint_model}")
    logger.info(f"  2. {marketing_model}")
    logger.info("All models pushed to HuggingFace Hub!")


def main():
    parser = argparse.ArgumentParser(
        description="Nuclear Chapel DUAL TRAINING - OSINT + Marketing"
    )
    parser.add_argument(
        '--mode',
        choices=['sequential', 'parallel'],
        default='sequential',
        help='Training mode: sequential (OSINT→Marketing) or parallel (simultaneous)'
    )
    parser.add_argument(
        '--samples',
        type=int,
        default=50000,
        help='Number of samples per dataset'
    )
    
    args = parser.parse_args()
    
    if args.mode == 'sequential':
        train_sequential(args.samples)
    else:
        train_parallel(args.samples)


if __name__ == '__main__':
    main()
