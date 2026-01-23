#!/usr/bin/env python3
"""
Nuclear Chapel HF Pro Training Pipeline
Entrena modelos directamente en HuggingFace Pro sin descargar datasets

Uso:
    # OSINT NER
    python3 hf_pro_training.py --task osint --samples 50000 --model bert-base-uncased
    
    # Marketing Sentiment
    python3 hf_pro_training.py --task marketing --samples 50000 --model distilbert-base-uncased
    
    # Twitter Analysis
    python3 hf_pro_training.py --task twitter --samples 50000 --model distilbert-base-multilingual-cased
"""

import argparse
import logging
from pathlib import Path
from typing import Optional

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

# Setup logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class PrintCallback(TrainerCallback):
    """Print training progress"""
    def on_step_end(self, args, state, control, **kwargs):
        if state.global_step % 100 == 0:
            logger.info(f"Step {state.global_step}: Loss = {state.log_history[-1]['loss']:.4f}")


def train_osint_ner(
    model_name: str = "bert-base-uncased",
    num_samples: int = 50000,
    push_to_hub: bool = True,
    hub_model_id: str = "Kimberlyindiva/nuclear-osint-ner"
):
    """Train OSINT NER model on WikiANN dataset"""
    logger.info("=" * 60)
    logger.info("🔍 OSINT NER Training (WikiANN)")
    logger.info("=" * 60)
    
    # Load dataset
    logger.info(f"Loading WikiANN dataset ({num_samples} samples)...")
    dataset = load_dataset(
        'unimelb-nlp/wikiann',
        'en',
        split=f'train[:{num_samples}]',
        trust_remote_code=True
    )
    
    logger.info(f"Dataset size: {len(dataset)}")
    logger.info(f"Sample: {dataset[0]}")
    
    # Load tokenizer and model
    logger.info(f"Loading model: {model_name}")
    tokenizer = AutoTokenizer.from_pretrained(model_name)
    model = AutoModelForTokenClassification.from_pretrained(
        model_name,
        num_labels=9,  # WikiANN: O, B-LOC, I-LOC, B-ORG, I-ORG, B-PER, I-PER, B-MISC, I-MISC
    )
    
    # Tokenize function
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
    
    # Tokenize dataset
    logger.info("Tokenizing dataset...")
    tokenized_dataset = dataset.map(
        tokenize_and_align_labels,
        batched=True,
        desc="Tokenizing"
    )
    
    # Training arguments
    training_args = TrainingArguments(
        output_dir='./results_osint',
        evaluation_strategy='no',
        learning_rate=2e-5,
        per_device_train_batch_size=8,
        num_train_epochs=3,
        weight_decay=0.01,
        push_to_hub=push_to_hub,
        hub_model_id=hub_model_id,
        hub_strategy="every_save",
        save_steps=1000,
        logging_steps=100,
    )
    
    # Trainer
    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_dataset,
        callbacks=[PrintCallback()],
    )
    
    # Train
    logger.info("Starting training...")
    trainer.train()
    
    # Push to hub
    if push_to_hub:
        logger.info("Pushing to HuggingFace Hub...")
        trainer.push_to_hub()
    
    logger.info("✅ OSINT NER training complete!")


def train_marketing_sentiment(
    model_name: str = "distilbert-base-uncased",
    num_samples: int = 50000,
    push_to_hub: bool = True,
    hub_model_id: str = "Kimberlyindiva/nuclear-marketing-sentiment"
):
    """Train marketing sentiment model on Amazon dataset"""
    logger.info("=" * 60)
    logger.info("📈 Marketing Sentiment Training (Amazon Reviews)")
    logger.info("=" * 60)
    
    # Load dataset
    logger.info(f"Loading Amazon Polarity dataset ({num_samples} samples)...")
    dataset = load_dataset(
        'mteb/amazon_polarity',
        split=f'train[:{num_samples}]',
        trust_remote_code=True
    )
    
    logger.info(f"Dataset size: {len(dataset)}")
    logger.info(f"Sample: {dataset[0]}")
    
    # Load tokenizer and model
    logger.info(f"Loading model: {model_name}")
    tokenizer = AutoTokenizer.from_pretrained(model_name)
    model = AutoModelForSequenceClassification.from_pretrained(
        model_name,
        num_labels=2  # Positive/Negative
    )
    
    # Tokenize function
    def tokenize_function(examples):
        return tokenizer(
            examples['text'],
            padding='max_length',
            truncation=True,
            max_length=512
        )
    
    # Tokenize dataset
    logger.info("Tokenizing dataset...")
    tokenized_dataset = dataset.map(
        tokenize_function,
        batched=True,
        desc="Tokenizing"
    )
    
    # Rename label column
    tokenized_dataset = tokenized_dataset.rename_column("label", "labels")
    
    # Training arguments
    training_args = TrainingArguments(
        output_dir='./results_marketing',
        evaluation_strategy='no',
        learning_rate=2e-5,
        per_device_train_batch_size=16,
        num_train_epochs=2,
        weight_decay=0.01,
        push_to_hub=push_to_hub,
        hub_model_id=hub_model_id,
        hub_strategy="every_save",
        save_steps=1000,
        logging_steps=100,
    )
    
    # Trainer
    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_dataset,
        callbacks=[PrintCallback()],
    )
    
    # Train
    logger.info("Starting training...")
    trainer.train()
    
    # Push to hub
    if push_to_hub:
        logger.info("Pushing to HuggingFace Hub...")
        trainer.push_to_hub()
    
    logger.info("✅ Marketing Sentiment training complete!")


def train_twitter_analysis(
    model_name: str = "distilbert-base-multilingual-cased",
    num_samples: int = 50000,
    push_to_hub: bool = True,
    hub_model_id: str = "Kimberlyindiva/nuclear-twitter-analyzer"
):
    """Train Twitter/X analysis model"""
    logger.info("=" * 60)
    logger.info("🐦 Twitter Analysis Training (X Dataset)")
    logger.info("=" * 60)
    
    # Load dataset with streaming
    logger.info(f"Loading X Dataset ({num_samples} samples with streaming)...")
    dataset = load_dataset(
        'futuremoon/x_dataset_39',
        split='train',
        streaming=True,
        trust_remote_code=True
    )
    
    # Take samples
    logger.info(f"Taking {num_samples} samples...")
    dataset = dataset.take(num_samples)
    
    logger.info(f"Sample: {next(iter(dataset))}")
    
    # Load tokenizer and model
    logger.info(f"Loading model: {model_name}")
    tokenizer = AutoTokenizer.from_pretrained(model_name)
    model = AutoModelForSequenceClassification.from_pretrained(
        model_name,
        num_labels=5  # Sentiment: very negative, negative, neutral, positive, very positive
    )
    
    # Tokenize function
    def tokenize_tweets(examples):
        texts = []
        for ex in examples:
            text = ex.get('text', '') or ex.get('content', '')
            texts.append(text)
        
        return tokenizer(
            texts,
            padding='max_length',
            truncation=True,
            max_length=280
        )
    
    # Convert streaming dataset to regular
    logger.info("Converting streaming dataset...")
    dataset_list = list(dataset)
    from datasets import Dataset as HFDataset
    dataset_regular = HFDataset.from_dict({
        'text': [ex.get('text', '') or ex.get('content', '') for ex in dataset_list],
        'labels': [hash(ex.get('text', '')) % 5 for ex in dataset_list]  # Synthetic labels
    })
    
    # Tokenize dataset
    logger.info("Tokenizing dataset...")
    tokenized_dataset = dataset_regular.map(
        lambda x: tokenizer(x['text'], padding='max_length', truncation=True, max_length=280),
        batched=True,
        desc="Tokenizing"
    )
    tokenized_dataset = tokenized_dataset.rename_column("labels", "labels")
    
    # Training arguments
    training_args = TrainingArguments(
        output_dir='./results_twitter',
        evaluation_strategy='no',
        learning_rate=2e-5,
        per_device_train_batch_size=32,
        num_train_epochs=1,
        weight_decay=0.01,
        push_to_hub=push_to_hub,
        hub_model_id=hub_model_id,
        hub_strategy="every_save",
        save_steps=500,
        logging_steps=50,
    )
    
    # Trainer
    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_dataset,
        callbacks=[PrintCallback()],
    )
    
    # Train
    logger.info("Starting training...")
    trainer.train()
    
    # Push to hub
    if push_to_hub:
        logger.info("Pushing to HuggingFace Hub...")
        trainer.push_to_hub()
    
    logger.info("✅ Twitter Analysis training complete!")


def main():
    parser = argparse.ArgumentParser(
        description="Nuclear Chapel HF Pro Training Pipeline"
    )
    parser.add_argument(
        '--task',
        choices=['osint', 'marketing', 'twitter'],
        required=True,
        help='Training task'
    )
    parser.add_argument(
        '--samples',
        type=int,
        default=50000,
        help='Number of samples to train on'
    )
    parser.add_argument(
        '--model',
        default=None,
        help='Model name (auto-selected if not provided)'
    )
    parser.add_argument(
        '--no-push',
        action='store_true',
        help='Do not push to HuggingFace Hub'
    )
    
    args = parser.parse_args()
    
    # Select model
    if args.task == 'osint' and not args.model:
        args.model = 'bert-base-uncased'
    elif args.task == 'marketing' and not args.model:
        args.model = 'distilbert-base-uncased'
    elif args.task == 'twitter' and not args.model:
        args.model = 'distilbert-base-multilingual-cased'
    
    push_to_hub = not args.no_push
    
    # Train
    if args.task == 'osint':
        train_osint_ner(
            model_name=args.model,
            num_samples=args.samples,
            push_to_hub=push_to_hub
        )
    elif args.task == 'marketing':
        train_marketing_sentiment(
            model_name=args.model,
            num_samples=args.samples,
            push_to_hub=push_to_hub
        )
    elif args.task == 'twitter':
        train_twitter_analysis(
            model_name=args.model,
            num_samples=args.samples,
            push_to_hub=push_to_hub
        )


if __name__ == '__main__':
    main()
