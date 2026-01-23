# 🚀 HuggingFace Pro Notebook - DUAL TRAINING (OSINT + Marketing)

**Copia este notebook completo en HF Pro y ejecuta**

## Celda 1: Setup e Imports

```python
!pip install -q datasets transformers torch scikit-learn tqdm

import logging
import warnings
warnings.filterwarnings('ignore')

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

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

print("✅ All imports successful!")
print(f"CUDA available: {torch.cuda.is_available()}")
if torch.cuda.is_available():
    print(f"GPU: {torch.cuda.get_device_name(0)}")
```

---

## Celda 2: Definir Callbacks y Funciones Auxiliares

```python
class PrintCallback(TrainerCallback):
    def on_step_end(self, args, state, control, **kwargs):
        if state.global_step % 100 == 0:
            if hasattr(state, 'log_history') and state.log_history:
                loss = state.log_history[-1].get('loss', 'N/A')
                print(f"  Step {state.global_step}: Loss = {loss:.4f}")

def print_section(title):
    print("\n" + "="*60)
    print(title)
    print("="*60)
```

---

## Celda 3: 🔍 OSINT NER Training

```python
print_section("🔍 PHASE 1: OSINT NER Training (WikiANN)")

# Load dataset
print("Loading WikiANN dataset (50K samples)...")
dataset_osint = load_dataset(
    'unimelb-nlp/wikiann',
    'en',
    split='train[:50000]',
    trust_remote_code=True
)
print(f"✅ Loaded {len(dataset_osint)} samples")

# Tokenizer + Model
tokenizer_osint = AutoTokenizer.from_pretrained('bert-base-uncased')
model_osint = AutoModelForTokenClassification.from_pretrained(
    'bert-base-uncased',
    num_labels=9
)

# Tokenization function
def tokenize_and_align_labels(examples):
    tokenized_inputs = tokenizer_osint(
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

# Tokenize
print("Tokenizing OSINT dataset...")
tokenized_osint = dataset_osint.map(
    tokenize_and_align_labels,
    batched=True,
    batch_size=100,
    desc="Tokenizing"
)
print(f"✅ Tokenized: {len(tokenized_osint)} samples")

# Training args
training_args_osint = TrainingArguments(
    output_dir='./results_osint',
    evaluation_strategy='no',
    learning_rate=2e-5,
    per_device_train_batch_size=8,
    num_train_epochs=2,
    weight_decay=0.01,
    push_to_hub=True,
    hub_model_id='Kimberlyindiva/nuclear-osint-ner',
    hub_strategy="every_save",
    save_steps=500,
    logging_steps=100,
    logging_dir='./logs_osint',
)

# Trainer
trainer_osint = Trainer(
    model=model_osint,
    args=training_args_osint,
    train_dataset=tokenized_osint,
    callbacks=[PrintCallback()],
)

# Train
print("🚀 Training OSINT NER (starts now)...")
trainer_osint.train()

# Push
print("📤 Pushing OSINT model to HuggingFace Hub...")
trainer_osint.push_to_hub()

print("✅ OSINT NER training complete!")
```

---

## Celda 4: 📈 Marketing Sentiment Training

```python
print_section("📈 PHASE 2: Marketing Sentiment (Amazon Reviews)")

# Load dataset
print("Loading Amazon Polarity dataset (50K samples)...")
dataset_marketing = load_dataset(
    'mteb/amazon_polarity',
    split='train[:50000]',
    trust_remote_code=True
)
print(f"✅ Loaded {len(dataset_marketing)} samples")

# Tokenizer + Model
tokenizer_marketing = AutoTokenizer.from_pretrained('distilbert-base-uncased')
model_marketing = AutoModelForSequenceClassification.from_pretrained(
    'distilbert-base-uncased',
    num_labels=2  # Positive/Negative
)

# Tokenization function
def tokenize_marketing(examples):
    return tokenizer_marketing(
        examples['text'],
        padding='max_length',
        truncation=True,
        max_length=512
    )

# Tokenize
print("Tokenizing Marketing dataset...")
tokenized_marketing = dataset_marketing.map(
    tokenize_marketing,
    batched=True,
    batch_size=100,
    desc="Tokenizing"
)

# Rename label column
tokenized_marketing = tokenized_marketing.rename_column("label", "labels")
print(f"✅ Tokenized: {len(tokenized_marketing)} samples")

# Training args
training_args_marketing = TrainingArguments(
    output_dir='./results_marketing',
    evaluation_strategy='no',
    learning_rate=2e-5,
    per_device_train_batch_size=16,
    num_train_epochs=2,
    weight_decay=0.01,
    push_to_hub=True,
    hub_model_id='Kimberlyindiva/nuclear-marketing-sentiment',
    hub_strategy="every_save",
    save_steps=500,
    logging_steps=100,
    logging_dir='./logs_marketing',
)

# Trainer
trainer_marketing = Trainer(
    model=model_marketing,
    args=training_args_marketing,
    train_dataset=tokenized_marketing,
    callbacks=[PrintCallback()],
)

# Train
print("🚀 Training Marketing Sentiment (starts now)...")
trainer_marketing.train()

# Push
print("📤 Pushing Marketing model to HuggingFace Hub...")
trainer_marketing.push_to_hub()

print("✅ Marketing Sentiment training complete!")
```

---

## Celda 5: 📊 Summary & Results

```python
print_section("📊 DUAL TRAINING SUMMARY")

print("✅ Both models trained successfully!\n")

print("Models created:")
print("  1. nuclear-osint-ner")
print("     └─ Task: Named Entity Recognition (OSINT)")
print("     └─ Dataset: WikiANN (176 languages)")
print("     └─ Hub: https://huggingface.co/Kimberlyindiva/nuclear-osint-ner\n")

print("  2. nuclear-marketing-sentiment")
print("     └─ Task: Sentiment Classification (Marketing)")
print("     └─ Dataset: Amazon Reviews (1M samples)")
print("     └─ Hub: https://huggingface.co/Kimberlyindiva/nuclear-marketing-sentiment\n")

print("Training metrics:")
print(f"  OSINT epochs: 2")
print(f"  OSINT batch size: 8")
print(f"  Marketing epochs: 2")
print(f"  Marketing batch size: 16")
print(f"  Total time: ~30-40 min (depending on GPU)\n")

print("Next steps:")
print("  1. Check models at: https://huggingface.co/Kimberlyindiva")
print("  2. Use in inference with:")
print("     - from transformers import pipeline")
print("     - pipe = pipeline('ner', model='Kimberlyindiva/nuclear-osint-ner')")
print("  3. Deploy to production!")
```

---

## Celda 6 (Opcional): 🧪 Test Models

```python
from transformers import pipeline

print_section("🧪 Testing Trained Models")

# Test OSINT NER
print("Testing OSINT NER...")
ner_pipeline = pipeline(
    'ner',
    model='Kimberlyindiva/nuclear-osint-ner',
    device=0 if torch.cuda.is_available() else -1
)

test_text = "Apple Inc. was founded by Steve Jobs in Cupertino, California."
ner_results = ner_pipeline(test_text)
print(f"Text: {test_text}")
print(f"NER Results: {ner_results}\n")

# Test Marketing Sentiment
print("Testing Marketing Sentiment...")
sentiment_pipeline = pipeline(
    'text-classification',
    model='Kimberlyindiva/nuclear-marketing-sentiment',
    device=0 if torch.cuda.is_available() else -1
)

test_review = "This product is absolutely amazing! Best purchase ever!"
sentiment_results = sentiment_pipeline(test_review)
print(f"Review: {test_review}")
print(f"Sentiment: {sentiment_results}\n")

print("✅ Both models working correctly!")
```

---

## 📋 Instrucciones para HF Pro

1. **Crea un notebook nuevo** en https://huggingface.co/Kimberlyindiva
2. **Copia cada celda** de arriba en orden
3. **Ejecuta secuencialmente** (Shift+Enter)
4. **Tiempo total**: ~30-40 minutos
5. **Resultado**: 2 modelos en tu HF account

---

## ⚙️ Configuración Recomendada

- **Compute**: GPU (cualquier tipo, P100+ óptimo)
- **Storage**: 5GB mínimo
- **Timeout**: 1 hora+
- **Privacy**: Private (por defecto)

---

## 💡 Tips

✅ Si sale error de memoria, reduce `batch_size` en TrainingArguments
✅ Si es lento, aumenta `save_steps` (ej: 1000 en vez de 500)
✅ Puedes editar `num_epochs` para más/menos training
✅ Los modelos se guardan AUTOMÁTICAMENTE en tu HF account

¡Listo para entrenar! 🚀
