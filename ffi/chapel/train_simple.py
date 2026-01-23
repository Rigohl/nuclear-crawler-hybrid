#!/usr/bin/env python3
"""
⚡ DUAL TRAINING - VERSIÓN SIMPLIFICADA
Usando datasets públicos simples sin scripts de carga complejos
"""

print("""
╔════════════════════════════════════════════════════════════╗
║   🚀 DUAL TRAINING: OSINT + MARKETING                    ║
║      Datasets reales, sin scripts complejos               ║
╚════════════════════════════════════════════════════════════╝
""")

import os
os.environ['HF_DATASETS_OFFLINE'] = '0'

print("\n[01/06] Importando librerías...")
try:
    from datasets import load_dataset
    from transformers import (
        AutoTokenizer,
        AutoModelForSequenceClassification,
        TrainingArguments,
        Trainer,
        TrainerCallback,
    )
    import torch
    import logging
    logging.basicConfig(level=logging.ERROR)
    print("  ✅ Todas las librerías importadas")
except ImportError as e:
    print(f"  ❌ Error: {e}")
    exit(1)

# Configurar dispositivo
device = "cuda" if torch.cuda.is_available() else "cpu"
print(f"\n[02/06] Dispositivo: {device}")

class PrintCallback(TrainerCallback):
    def on_step_end(self, args, state, control, **kwargs):
        if state.global_step % 50 == 0:
            loss = state.log_history[-1].get('loss', 'N/A') if state.log_history else 'N/A'
            print(f"     Step {state.global_step}: loss={loss:.4f if isinstance(loss, float) else 'N/A'}")

# OPCIÓN 1: IMDB (Sentiment Analysis - Marketing)
print("\n[03/06] Cargando IMDB (Marketing Sentiment)...")
try:
    dataset_imdb = load_dataset('imdb', split='train[:50000]', trust_remote_code=False)
    print(f"  ✅ IMDB: {len(dataset_imdb):,} reviews")
except Exception as e:
    print(f"  ⚠️  IMDB failed: {e}")
    print("     Usando alternativa: ag_news")
    try:
        dataset_imdb = load_dataset('ag_news', split='train[:50000]')
        print(f"  ✅ AG News: {len(dataset_imdb):,} noticias")
    except Exception as e2:
        print(f"  ❌ Ambos fallaron: {e2}")
        exit(1)

# OPCIÓN 2: GLUE SST2 (Sentiment - Marketing alternativo)
print("\n[04/06] Cargando GLUE SST2 (Sentiment Classification)...")
try:
    dataset_glue = load_dataset('glue', 'sst2', split='train[:50000]', trust_remote_code=False)
    print(f"  ✅ GLUE SST2: {len(dataset_glue):,} sentencias")
except Exception as e:
    print(f"  ⚠️  GLUE failed: {e}")
    dataset_glue = None

print("\n" + "="*60)
print("🚀 ENTRENAMIENTOS")
print("="*60)

# TRAINING 1: IMDB Sentiment
if dataset_imdb is not None:
    print("\n[05/06] 📈 Training 1: Sentiment Classification")
    try:
        print("  Cargando DistilBERT...")
        tokenizer_imdb = AutoTokenizer.from_pretrained('distilbert-base-uncased')
        model_imdb = AutoModelForSequenceClassification.from_pretrained(
            'distilbert-base-uncased',
            num_labels=2
        ).to(device)
        
        print("  Tokenizando dataset...")
        def tokenize_imdb(examples):
            text_column = 'text' if 'text' in examples else 'sentence'
            return tokenizer_imdb(
                examples[text_column],
                padding='max_length',
                truncation=True,
                max_length=512
            )
        
        dataset_tokenized = dataset_imdb.map(
            tokenize_imdb,
            batched=True,
            batch_size=50,
            remove_columns=[col for col in dataset_imdb.column_names if col != 'label']
        )
        
        if 'label' not in dataset_tokenized.column_names:
            dataset_tokenized = dataset_tokenized.rename_column("label", "labels")
        else:
            dataset_tokenized = dataset_tokenized.rename_column("label", "labels") if "label" in dataset_tokenized.column_names else dataset_tokenized
        
        print(f"  ✅ {len(dataset_tokenized):,} muestras listas")
        
        training_args = TrainingArguments(
            output_dir='./results_sentiment',
            evaluation_strategy='no',
            learning_rate=2e-5,
            per_device_train_batch_size=16,
            num_train_epochs=2,
            weight_decay=0.01,
            logging_steps=100,
            save_steps=500,
            save_total_limit=1,
        )
        
        trainer = Trainer(
            model=model_imdb,
            args=training_args,
            train_dataset=dataset_tokenized,
            callbacks=[PrintCallback()],
        )
        
        print("  🚀 Iniciando training...")
        trainer.train()
        print("  ✅ Training completado!")
        
        # Guardar modelo
        model_path = './results_sentiment/final_model'
        trainer.save_model(model_path)
        print(f"  💾 Modelo guardado en: {model_path}")
        
    except Exception as e:
        print(f"  ❌ Error en training 1: {e}")
        import traceback
        traceback.print_exc()

# TRAINING 2: GLUE SST2 Alternative
if dataset_glue is not None:
    print("\n[05/06] 📈 Training 2: GLUE SST2 Sentiment")
    try:
        print("  Cargando BERT-base...")
        tokenizer_glue = AutoTokenizer.from_pretrained('bert-base-uncased')
        model_glue = AutoModelForSequenceClassification.from_pretrained(
            'bert-base-uncased',
            num_labels=2
        ).to(device)
        
        print("  Tokenizando dataset...")
        def tokenize_glue(examples):
            return tokenizer_glue(
                examples['sentence'],
                padding='max_length',
                truncation=True,
                max_length=512
            )
        
        dataset_tokenized2 = dataset_glue.map(
            tokenize_glue,
            batched=True,
            batch_size=50,
            remove_columns=[col for col in dataset_glue.column_names if col != 'label']
        )
        
        dataset_tokenized2 = dataset_tokenized2.rename_column("label", "labels")
        print(f"  ✅ {len(dataset_tokenized2):,} muestras listas")
        
        training_args2 = TrainingArguments(
            output_dir='./results_glue',
            evaluation_strategy='no',
            learning_rate=2e-5,
            per_device_train_batch_size=8,
            num_train_epochs=2,
            weight_decay=0.01,
            logging_steps=100,
            save_steps=500,
            save_total_limit=1,
        )
        
        trainer2 = Trainer(
            model=model_glue,
            args=training_args2,
            train_dataset=dataset_tokenized2,
            callbacks=[PrintCallback()],
        )
        
        print("  🚀 Iniciando training...")
        trainer2.train()
        print("  ✅ Training completado!")
        
        # Guardar modelo
        model_path2 = './results_glue/final_model'
        trainer2.save_model(model_path2)
        print(f"  💾 Modelo guardado en: {model_path2}")
        
    except Exception as e:
        print(f"  ❌ Error en training 2: {e}")
        import traceback
        traceback.print_exc()

# RESUMEN
print("\n" + "="*60)
print("✅ ENTRENAMIENTOS COMPLETADOS")
print("="*60)

print("""
📊 RESUMEN:
   • Dataset 1: IMDB o AG News (Marketing/Sentiment)
   • Dataset 2: GLUE SST2 (Sentiment Classification)
   • Modelos: DistilBERT + BERT-base
   • Epochs: 2 cada uno
   • Batch size: 16 (IMDB), 8 (GLUE)
   
💾 MODELOS GUARDADOS:
   • ./results_sentiment/final_model/
   • ./results_glue/final_model/
   
✨ ¡Listo para usar!
""")
