#!/usr/bin/env python3
"""
Qwen2.5-Coder Fine-Tuning con Datos REALES
Entrena modelo con comportamiento humano auténtico
"""

import json
import os
from typing import List, Dict
from transformers import (
    AutoModelForCausalLM,
    AutoTokenizer,
    Trainer,
    TrainingArguments,
    DataCollatorForLanguageModeling
)
from datasets import Dataset
import torch

class QwenRealPersonalityTrainer:
    """Fine-tune Qwen con personalidad REAL (no corporativa)"""
    
    def __init__(self, model_name: str = "Qwen/Qwen2.5-Coder-7B-Instruct"):
        self.model_name = model_name
        self.model = None
        self.tokenizer = None
        
        print(f"🧠 Inicializando {model_name}...")
    
    def load_model(self):
        """Carga modelo y tokenizer"""
        print("📥 Cargando modelo...")
        
        self.tokenizer = AutoTokenizer.from_pretrained(
            self.model_name,
            trust_remote_code=True
        )
        
        self.model = AutoModelForCausalLM.from_pretrained(
            self.model_name,
            torch_dtype=torch.float16 if torch.cuda.is_available() else torch.float32,
            device_map="auto",
            trust_remote_code=True
        )
        
        # Asegurar pad token
        if self.tokenizer.pad_token is None:
            self.tokenizer.pad_token = self.tokenizer.eos_token
        
        print("✅ Modelo cargado")
    
    def load_real_conversations(self, data_dir: str = 'ai-training/scraping/output') -> List[Dict]:
        """Carga TODAS las conversaciones reales"""
        
        conversations = []
        
        # Reddit
        reddit_file = os.path.join(data_dir, 'reddit_marketing_real.json')
        if os.path.exists(reddit_file):
            with open(reddit_file, 'r', encoding='utf-8') as f:
                reddit_data = json.load(f)
                conversations.extend(self._format_reddit_convs(reddit_data))
                print(f"✅ Reddit: {len(reddit_data)} conversaciones")
        
        # Twitter
        twitter_file = os.path.join(data_dir, 'twitter_marketers_real.json')
        if os.path.exists(twitter_file):
            with open(twitter_file, 'r', encoding='utf-8') as f:
                twitter_data = json.load(f)
                conversations.extend(self._format_twitter_tweets(twitter_data))
                print(f"✅ Twitter: {len(twitter_data)} tweets")
        
        # LinkedIn (si existe)
        linkedin_file = os.path.join(data_dir, 'linkedin_sales_real.json')
        if os.path.exists(linkedin_file):
            with open(linkedin_file, 'r', encoding='utf-8') as f:
                linkedin_data = json.load(f)
                conversations.extend(self._format_linkedin_posts(linkedin_data))
                print(f"✅ LinkedIn: {len(linkedin_data)} posts")
        
        print(f"\n📊 Total conversaciones cargadas: {len(conversations)}")
        return conversations
    
    def _format_reddit_convs(self, reddit_data: List[Dict]) -> List[Dict]:
        """Formatea conversaciones de Reddit para training"""
        formatted = []
        
        for post in reddit_data:
            # Post original
            if post.get('body'):
                formatted.append({
                    'text': f"Pregunta: {post['title']}\n\nRespuesta: {post['body']}",
                    'source': 'reddit',
                    'score': post.get('score', 0),
                    'style': 'conversational'
                })
            
            # Comentarios (conversaciones reales)
            for i, comment in enumerate(post.get('comments', [])):
                if i == 0 and post.get('body'):
                    # Primer comentario en contexto del post
                    formatted.append({
                        'text': f"{post['body']}\n\nComentario: {comment['text']}",
                        'source': 'reddit',
                        'score': comment.get('score', 0),
                        'style': 'conversational'
                    })
                else:
                    # Comentario standalone
                    formatted.append({
                        'text': comment['text'],
                        'source': 'reddit',
                        'score': comment.get('score', 0),
                        'style': 'conversational'
                    })
        
        return formatted
    
    def _format_twitter_tweets(self, twitter_data: List[Dict]) -> List[Dict]:
        """Formatea tweets para training"""
        formatted = []
        
        for tweet in twitter_data:
            # Solo tweets con engagement alto (auténticos)
            if tweet.get('likes', 0) >= 5:
                formatted.append({
                    'text': tweet['text'],
                    'source': 'twitter',
                    'score': tweet.get('likes', 0) + tweet.get('retweets', 0),
                    'style': 'direct'
                })
        
        return formatted
    
    def _format_linkedin_posts(self, linkedin_data: List[Dict]) -> List[Dict]:
        """Formatea posts de LinkedIn para training"""
        formatted = []
        
        for post in linkedin_data:
            formatted.append({
                'text': post['text'],
                'source': 'linkedin',
                'score': int(post.get('reactions', '0').replace(',', '')),
                'style': 'professional_casual'
            })
        
        return formatted
    
    def filter_by_authenticity(self, conversations: List[Dict], min_score: int = 10) -> List[Dict]:
        """Filtra por autenticidad (score alto = más real)"""
        filtered = [c for c in conversations if c.get('score', 0) >= min_score]
        print(f"🔍 Filtrado por autenticidad (score >= {min_score}):")
        print(f"   {len(conversations)} → {len(filtered)} conversaciones")
        return filtered
    
    def prepare_training_data(self, conversations: List[Dict]) -> Dataset:
        """Prepara dataset para training"""
        
        # Tokenizar conversaciones
        texts = [conv['text'] for conv in conversations]
        
        def tokenize_function(examples):
            return self.tokenizer(
                examples['text'],
                truncation=True,
                max_length=512,
                padding='max_length'
            )
        
        dataset = Dataset.from_dict({'text': texts})
        tokenized_dataset = dataset.map(
            tokenize_function,
            batched=True,
            remove_columns=['text']
        )
        
        return tokenized_dataset
    
    def train(self, conversations: List[Dict], output_dir: str = './qwen-marketing-real'):
        """Entrena el modelo con conversaciones reales"""
        
        if not self.model or not self.tokenizer:
            self.load_model()
        
        # Filtrar por calidad
        quality_convs = self.filter_by_authenticity(conversations, min_score=10)
        
        # Preparar dataset
        print("\n📦 Preparando dataset...")
        dataset = self.prepare_training_data(quality_convs)
        
        # Training arguments
        training_args = TrainingArguments(
            output_dir=output_dir,
            num_train_epochs=3,
            per_device_train_batch_size=2,
            gradient_accumulation_steps=4,
            learning_rate=2e-5,
            fp16=torch.cuda.is_available(),
            logging_steps=100,
            save_steps=500,
            save_total_limit=2,
            report_to='none'
        )
        
        # Data collator
        data_collator = DataCollatorForLanguageModeling(
            tokenizer=self.tokenizer,
            mlm=False
        )
        
        # Trainer
        trainer = Trainer(
            model=self.model,
            args=training_args,
            train_dataset=dataset,
            data_collator=data_collator
        )
        
        print("\n🚀 Iniciando training...")
        print(f"   Dataset: {len(quality_convs)} conversaciones")
        print(f"   Epochs: 3")
        print(f"   Batch size: 2 (x4 grad accumulation = 8 effective)")
        print()
        
        # Train!
        trainer.train()
        
        # Guardar modelo final
        print("\n💾 Guardando modelo...")
        self.model.save_pretrained(f"{output_dir}/final")
        self.tokenizer.save_pretrained(f"{output_dir}/final")
        
        print(f"\n✅ TRAINING COMPLETO!")
        print(f"   Modelo guardado en: {output_dir}/final")
        
        return f"{output_dir}/final"


def main():
    """Ejecutar training completo"""
    print("=" * 70)
    print("  QWEN2.5-CODER REAL PERSONALITY TRAINING")
    print("  Fine-tuning con conversaciones AUTÉNTICAS")
    print("=" * 70)
    print()
    
    # 1. Inicializar trainer
    trainer = QwenRealPersonalityTrainer()
    
    # 2. Cargar conversaciones reales
    print("📥 Cargando conversaciones reales...")
    conversations = trainer.load_real_conversations()
    
    if not conversations:
        print("\n❌ ERROR: No se encontraron datos de entrenamiento!")
        print("   Ejecuta primero los scrapers:")
        print("   1. python reddit_scraper.py")
        print("   2. python twitter_scraper.py")
        return
    
    # 3. Entrenar
    model_path = trainer.train(conversations)
    
    print("\n" + "=" * 70)
    print("✅ MODELO LISTO PARA USAR")
    print("=" * 70)
    print(f"Ruta: {model_path}")
    print("\n💡 Next step: python humanizer.py")


if __name__ == '__main__':
    main()
