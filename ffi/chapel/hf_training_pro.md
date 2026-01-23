# HuggingFace Pro Training Pipeline

**Entrena directamente en HuggingFace sin descargar datasets** ⚡

## 🎯 Datasets Disponibles

### OSINT + Web Scraping

#### 1. **WikiANN - NER Multilingüe** (OSINT)
- **Tamaño**: 1M-10M samples
- **Idiomas**: 176 idiomas
- **Tareas**: Named Entity Recognition (Personas, Locaciones, Organizaciones)
- **Descargas HF**: 3.4M
- **Link**: https://hf.co/datasets/unimelb-nlp/wikiann
- **Ideal para**: Extracción de entidades de textos web, OSINT

```python
# Código directo en HF Pro
import datasets
data = datasets.load_dataset('unimelb-nlp/wikiann', split='train[:100000]')
# Extrae: personas, locaciones, organizaciones de Wikipedia
```

#### 2. **CoNLL2003 - NER Clásico**
- **Tamaño**: 14,987 entrenamiento, 3,684 validación
- **Categorías**: PER, LOC, ORG, MISC
- **Tareas**: Named Entity Recognition, Part-of-Speech tagging
- **Descargas HF**: 1.9M
- **Link**: https://hf.co/datasets/eriktks/conll2003
- **Ideal para**: Benchmarking de OSINT, extracción de información

```python
data = datasets.load_dataset('eriktks/conll2003', split='train')
# POS tagging + NER: reconoce personas, lugares, organizaciones
```

#### 3. **Ecom-niverse - E-commerce Web Data** (Scraping)
- **Tamaño**: 100M-1B tokens
- **Fuente**: Web crawling refinado con contexto retail
- **Tareas**: Product descriptions, prices, commerce content
- **Descargas HF**: 33.1K
- **Link**: https://hf.co/datasets/thebajajra/Ecom-niverse
- **Ideal para**: Entrenar modelos de extracción de productos, precios, reviews

```python
data = datasets.load_dataset('thebajajra/Ecom-niverse', split='train[:50000]')
# Contiene: descripciones de productos, precios, metadata
```

---

### Marketing Intelligence

#### 4. **X (Twitter) Dataset - Sentiment + NER** (Marketing)
- **Tamaño**: 1B-10B tokens
- **Fuente**: Real-time tweets desde Bittensor Network
- **Tareas**: Sentiment Analysis, Topic Classification, NER, Summarization
- **Descargas HF**: 111.9K
- **Link**: https://hf.co/datasets/futuremoon/x_dataset_39
- **Ideal para**: Análisis de sentimiento de marcas, detección de trends

```python
data = datasets.load_dataset('futuremoon/x_dataset_39', split='train[:50000]', streaming=True)
# Contiene: tweets, sentimientos, tópicos, entidades
# Perfecto para análisis de marca y campañas
```

#### 5. **Reddit Dataset - Reviews + Opinions** (Marketing)
- **Tamaño**: 10M-100M tokens
- **Fuente**: Reddit data preprocessed desde Bittensor
- **Tareas**: Sentiment Analysis, Topic Classification, Q&A
- **Descargas HF**: 129.5K
- **Link**: https://hf.co/datasets/tensorshield/reddit_dataset_157
- **Ideal para**: Análisis de opiniones de clientes, product feedback

```python
data = datasets.load_dataset('tensorshield/reddit_dataset_157', split='train[:50000]')
# Contiene: opiniones de usuarios, reviews, feedback
# Multilingual sentiment analysis
```

#### 6. **Sentiment Analysis Datasets**
- **Tweet Sentiment Extraction**: 27.5K tweets (https://hf.co/datasets/mteb/tweet_sentiment_extraction)
- **Emotion Classification**: 16K tweets con 6 emociones (https://hf.co/datasets/mteb/emotion)
- **Amazon Polarity**: 1M reviews (https://hf.co/datasets/mteb/amazon_polarity)
- **IMDb Reviews**: 100K reviews (https://hf.co/datasets/mteb/imdb)

```python
# Tweet Sentiment
data = datasets.load_dataset('mteb/tweet_sentiment_extraction', split='train')

# Amazon Sentiment (Marketing insights)
data = datasets.load_dataset('mteb/amazon_polarity', split='train[:50000]')
# Contiene: 1M reviews de Amazon
# Perfect para entrenar modelo de análisis de reviews
```

---

## 🚀 Training en HF Pro (Copy-Paste)

### Opción 1: OSINT + NER (WikiANN)

```python
# En HF Pro Notebook
import datasets
from datasets import load_dataset
from transformers import AutoTokenizer, AutoModelForTokenClassification, TrainingArguments, Trainer
import torch

# 1. Cargar dataset OSINT
print("Loading WikiANN dataset...")
dataset = load_dataset('unimelb-nlp/wikiann', 'en', split='train[:50000]')  # 50K ejemplos en inglés

# 2. Tokenizar
tokenizer = AutoTokenizer.from_pretrained('bert-base-uncased')

def tokenize_and_align_labels(examples):
    tokenized_inputs = tokenizer(
        examples['tokens'],
        truncation=True,
        is_split_into_words=True,
        max_length=512,
        padding='max_length'
    )
    
    labels = []
    for i, label in enumerate(examples['tags']):
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

tokenized_dataset = dataset.map(tokenize_and_align_labels, batched=True)

# 3. Cargar modelo y entrenar
model = AutoModelForTokenClassification.from_pretrained('bert-base-uncased', num_labels=9)

training_args = TrainingArguments(
    output_dir='./results_osint',
    evaluation_strategy='no',
    learning_rate=2e-5,
    per_device_train_batch_size=8,
    num_train_epochs=3,
    weight_decay=0.01,
    push_to_hub=True,
    hub_model_id='Kimberlyindiva/nuclear-osint-ner'
)

trainer = Trainer(
    model=model,
    args=training_args,
    train_dataset=tokenized_dataset
)

# 4. Entrenar y push a HF
trainer.train()
trainer.push_to_hub()

print("✅ OSINT Model entrenado y pusheado a HuggingFace!")
```

### Opción 2: Marketing Sentiment (Amazon Reviews)

```python
# En HF Pro Notebook
import datasets
from transformers import AutoTokenizer, AutoModelForSequenceClassification, TextClassificationPipeline
from datasets import load_dataset
import torch

# 1. Cargar Amazon Reviews
print("Loading Amazon Polarity dataset...")
dataset = load_dataset('mteb/amazon_polarity', split='train[:50000]')  # 50K reviews

# 2. Tokenizar
tokenizer = AutoTokenizer.from_pretrained('distilbert-base-uncased')

def tokenize_function(examples):
    return tokenizer(examples['text'], padding='max_length', truncation=True, max_length=512)

tokenized_dataset = dataset.map(tokenize_function, batched=True)

# 3. Modelo
model = AutoModelForSequenceClassification.from_pretrained('distilbert-base-uncased', num_labels=2)

# 4. Training
from transformers import TrainingArguments, Trainer

training_args = TrainingArguments(
    output_dir='./results_marketing',
    evaluation_strategy='no',
    learning_rate=2e-5,
    per_device_train_batch_size=16,
    num_train_epochs=2,
    weight_decay=0.01,
    push_to_hub=True,
    hub_model_id='Kimberlyindiva/nuclear-marketing-sentiment'
)

trainer = Trainer(
    model=model,
    args=training_args,
    train_dataset=tokenized_dataset
)

trainer.train()
trainer.push_to_hub()

print("✅ Marketing Sentiment Model entrenado!")
```

### Opción 3: X/Twitter Analysis (Real-time)

```python
# En HF Pro Notebook
from datasets import load_dataset
from transformers import AutoModelForSequenceClassification, AutoTokenizer, pipeline
import torch

# 1. Cargar X Dataset (con streaming para no descargar todo)
print("Loading X Dataset...")
dataset = load_dataset('futuremoon/x_dataset_39', split='train', streaming=True)

# 2. Tomar primeros 50K ejemplos
print("Filtering to 50k samples...")
dataset_filtered = dataset.take(50000)

# 3. Tokenizer
tokenizer = AutoTokenizer.from_pretrained('distilbert-base-multilingual-cased')

def tokenize_x_tweets(examples):
    texts = [ex['text'] if 'text' in ex else '' for ex in examples]
    return tokenizer(texts, padding='max_length', truncation=True, max_length=280)

tokenized = dataset_filtered.map(tokenize_x_tweets, batched=True, batch_size=100)

# 4. Model para multi-task (sentiment + topic)
model = AutoModelForSequenceClassification.from_pretrained(
    'distilbert-base-multilingual-cased',
    num_labels=5  # 5 clases: muy negativo, negativo, neutral, positivo, muy positivo
)

# 5. Training
from transformers import TrainingArguments, Trainer

training_args = TrainingArguments(
    output_dir='./results_twitter',
    num_train_epochs=2,
    per_device_train_batch_size=32,
    push_to_hub=True,
    hub_model_id='Kimberlyindiva/nuclear-twitter-analyzer'
)

trainer = Trainer(
    model=model,
    args=training_args,
    train_dataset=tokenized
)

trainer.train()
trainer.push_to_hub()

print("✅ Twitter Analyzer Model entrenado!")
```

---

## 📊 Tabla de Capacidades

| Dataset | Tamaño | Idiomas | Tareas | Ideal Para | HF Downloads |
|---------|--------|---------|--------|-----------|--------------|
| **WikiANN** | 1M-10M | 176 | NER | OSINT Entity Extraction | 3.4M |
| **CoNLL2003** | 14.9K | 1 (EN) | NER, POS | Benchmark NER | 1.9M |
| **Ecom-niverse** | 100M-1B | 1 (EN) | Classification, Generation | Product Extraction | 33.1K |
| **X Dataset** | 1B-10B | Multi | Sentiment, NER, Topic | Marketing Analysis | 111.9K |
| **Reddit** | 10M-100M | Multi | Sentiment, QA, Summarization | Customer Feedback | 129.5K |
| **Amazon** | 1M | 1 (EN) | Sentiment Classification | Review Analysis | 1.9K |
| **Tweet Sentiment** | 27.5K | 1 (EN) | Sentiment Extraction | Tweet Analysis | 9.1K |
| **Emotion** | 16K | 1 (EN) | Multi-emotion Classification | Brand Sentiment | 8.9K |

---

## 🔄 Workflow: De Dataset a Modelo Productivo

```
1. HF Pro Notebook
   ├─ Load Dataset (streaming)
   ├─ Tokenize
   ├─ Train (GPU P100 equivalent)
   └─ Push to Hub

2. Modelo en HuggingFace
   ├─ nuclear-osint-ner
   ├─ nuclear-marketing-sentiment
   └─ nuclear-twitter-analyzer

3. Deploy (Pipeline)
   ├─ Use model en inference
   ├─ Integración con API
   └─ Real-time predictions
```

---

## 💡 Tips

✅ **Usa Streaming**: No descarges datasets completos, usa `streaming=True`
✅ **Toma muestras**: `.take(50000)` es suficiente para entrenamientos iniciales
✅ **Push a Hub**: Todos los modelos se guardan automáticamente en tu HF account
✅ **Multilingual**: Datasets como Reddit + X soportan múltiples idiomas
✅ **Real-time**: X Dataset es actualizado continuamente por mineros

---

## 🎯 Próximos Pasos

1. Abre https://huggingface.co/Kimberlyindiva
2. Crea 3 notebooks en HF Pro:
   - `01-osint-ner-training.ipynb` (WikiANN)
   - `02-marketing-sentiment.ipynb` (Amazon)
   - `03-twitter-analyzer.ipynb` (X Dataset)
3. Ejecuta cada uno (5-15 min por training)
4. Modelos automáticamente en tu HF account
5. Usa en producción con `pipeline()`

¡Listo! 🚀
