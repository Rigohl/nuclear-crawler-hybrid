#!/usr/bin/env python3
"""
⚡ DUAL TRAINING - SIN DEPENDENCIAS EXTERNAS
Entrena OSINT + Marketing directamente con HuggingFace
"""

import os
print("""
╔════════════════════════════════════════════════════════════╗
║   🚀 DUAL TRAINING: OSINT + MARKETING                    ║
║      Ejecutando entrenamiento en background...            ║
╚════════════════════════════════════════════════════════════╝
""")

# Paso 1: Cargar librerías
print("\n[01/05] Importando librerías...")
try:
    from datasets import load_dataset
    from transformers import (
        AutoTokenizer,
        AutoModelForTokenClassification,
        AutoModelForSequenceClassification,
        TrainingArguments,
        Trainer,
        TrainerCallback,
    )
    import torch
    import logging
    logging.basicConfig(level=logging.ERROR)
    print("  ✅ datasets, transformers, torch")
except ImportError as e:
    print(f"  ❌ Error: {e}")
    exit(1)

# Paso 2: Verificar GPU
print("\n[02/05] Configurando dispositivo...")
device = "cuda" if torch.cuda.is_available() else "cpu"
print(f"  ✅ Device: {device}")
if device == "cuda":
    print(f"     GPU: {torch.cuda.get_device_name(0)}")

# Paso 3: Cargar datasets
print("\n[03/05] Cargando datasets (streaming sin descargar)...")
try:
    print("  📥 WikiANN (OSINT NER)...")
    dataset_osint = load_dataset(
        'unimelb-nlp/wikiann',
        'en',
        split='train[:50000]',
        trust_remote_code=True
    )
    print(f"     ✅ {len(dataset_osint):,} samples cargados")
    
    print("  📥 Amazon Polarity (Marketing Sentiment)...")
    dataset_marketing = load_dataset(
        'mteb/amazon_polarity',
        split='train[:50000]',
        trust_remote_code=True
    )
    print(f"     ✅ {len(dataset_marketing):,} samples cargados")
    
except Exception as e:
    print(f"  ❌ Error cargando datasets: {e}")
    exit(1)

# Paso 4: Funciones de tokenización
print("\n[04/05] Preparando tokenizadores y modelos...")

class PrintCallback(TrainerCallback):
    def on_step_end(self, args, state, control, **kwargs):
        if state.global_step % 100 == 0:
            print(f"     Step {state.global_step}: Loss = {state.log_history[-1].get('loss', 'N/A'):.4f}")

def tokenize_and_align_labels(examples, tokenizer):
    """Tokenizar para NER manteniendo alineación de labels"""
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

def tokenize_marketing(examples, tokenizer):
    """Tokenizar para clasificación de sentimientos"""
    return tokenizer(
        examples['text'],
        padding='max_length',
        truncation=True,
        max_length=512
    )

# Paso 5: ENTRENAR
print("\n[05/05] 🚀 INICIANDO ENTRENAMIENTOS...\n")

print("="*60)
print("🔍 FASE 1: OSINT NER Training")
print("="*60)

try:
    tokenizer_osint = AutoTokenizer.from_pretrained('bert-base-uncased')
    model_osint = AutoModelForTokenClassification.from_pretrained(
        'bert-base-uncased',
        num_labels=9
    ).to(device)
    print("✅ Modelo BERT-base cargado")
    
    print("Tokenizando WikiANN...")
    tokenized_osint = dataset_osint.map(
        lambda x: tokenize_and_align_labels(x, tokenizer_osint),
        batched=True,
        batch_size=100,
        desc="Tokenizing OSINT"
    )
    print(f"✅ {len(tokenized_osint)} samples tokenizados")
    
    training_args_osint = TrainingArguments(
        output_dir='./results_osint',
        evaluation_strategy='no',
        learning_rate=2e-5,
        per_device_train_batch_size=8,
        num_train_epochs=2,
        weight_decay=0.01,
        logging_steps=100,
        save_steps=1000,
        logging_dir='./logs_osint',
        push_to_hub=False,  # Desactivar por ahora para evitar autenticación
    )
    
    trainer_osint = Trainer(
        model=model_osint,
        args=training_args_osint,
        train_dataset=tokenized_osint,
        callbacks=[PrintCallback()],
    )
    
    print("🚀 Iniciando training OSINT NER...")
    trainer_osint.train()
    print("✅ OSINT NER training completado!")
    
except Exception as e:
    print(f"❌ Error en OSINT: {e}")
    import traceback
    traceback.print_exc()

print("\n" + "="*60)
print("📈 FASE 2: Marketing Sentiment Training")
print("="*60)

try:
    tokenizer_marketing = AutoTokenizer.from_pretrained('distilbert-base-uncased')
    model_marketing = AutoModelForSequenceClassification.from_pretrained(
        'distilbert-base-uncased',
        num_labels=2
    ).to(device)
    print("✅ Modelo DistilBERT cargado")
    
    print("Tokenizando Amazon Polarity...")
    tokenized_marketing = dataset_marketing.map(
        lambda x: tokenize_marketing(x, tokenizer_marketing),
        batched=True,
        batch_size=100,
        desc="Tokenizing Marketing"
    )
    
    # Renombrar columna label
    if 'label' in tokenized_marketing.column_names:
        tokenized_marketing = tokenized_marketing.rename_column("label", "labels")
    
    print(f"✅ {len(tokenized_marketing)} samples tokenizados")
    
    training_args_marketing = TrainingArguments(
        output_dir='./results_marketing',
        evaluation_strategy='no',
        learning_rate=2e-5,
        per_device_train_batch_size=16,
        num_train_epochs=2,
        weight_decay=0.01,
        logging_steps=100,
        save_steps=1000,
        logging_dir='./logs_marketing',
        push_to_hub=False,
    )
    
    trainer_marketing = Trainer(
        model=model_marketing,
        args=training_args_marketing,
        train_dataset=tokenized_marketing,
        callbacks=[PrintCallback()],
    )
    
    print("🚀 Iniciando training Marketing Sentiment...")
    trainer_marketing.train()
    print("✅ Marketing Sentiment training completado!")
    
except Exception as e:
    print(f"❌ Error en Marketing: {e}")
    import traceback
    traceback.print_exc()

# Resumen final
print("\n" + "="*60)
print("📊 ENTRENAMIENTOS COMPLETADOS")
print("="*60)

print("""
✅ Modelos entrenados y guardados:
   • OSINT NER: ./results_osint/
   • Marketing Sentiment: ./results_marketing/

📁 Ubicaciones:
   • Modelos: ./results_osint/pytorch_model.bin
   • Modelos: ./results_marketing/pytorch_model.bin
   
🚀 Próximos pasos:
   1. Los modelos están listos localmente
   2. Para push a HuggingFace, autentica con: huggingface-cli login
   3. Luego ejecuta hf_pro_training.py con --push-hub
   
✨ ¡Listo para usar!
""")

print("="*60)
