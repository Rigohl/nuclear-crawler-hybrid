#!/usr/bin/env python3
"""
🚀 Simple HuggingFace Training Script - OSINT + Marketing
Sin dependencias complejas, directo al punto
"""

import os
import sys

print("""
╔═══════════════════════════════════════════════════╗
║  🚀 ENTRENAMIENTO DUAL: OSINT + MARKETING        ║
║     Streaming directo de HuggingFace             ║
╚═══════════════════════════════════════════════════╝
""")

# Verificar token HF
hf_token = os.environ.get('HF_TOKEN')
if not hf_token:
    print("⚠️  HF_TOKEN no encontrado en env")
    print("Por favor, establece: export HF_TOKEN='tu_token_aqui'")
    print("\nO ejecuta: huggingface-cli login")
    sys.exit(1)

print(f"✅ HF_TOKEN detectado (primeros 10 chars: {hf_token[:10]}...)")

# Importar lo mínimo necesario
try:
    from datasets import load_dataset
    print("✅ datasets importado")
except ImportError as e:
    print(f"❌ Error importando datasets: {e}")
    sys.exit(1)

try:
    from transformers import AutoTokenizer, AutoModelForTokenClassification
    print("✅ transformers importado")
except ImportError as e:
    print(f"❌ Error importando transformers: {e}")
    sys.exit(1)

print("\n" + "="*60)
print("FASE 1: Loading datasets...")
print("="*60)

try:
    print("📥 Cargando WikiANN (OSINT)...")
    dataset_osint = load_dataset(
        'unimelb-nlp/wikiann',
        'en',
        split='train[:50000]',
        trust_remote_code=True,
        token=hf_token
    )
    print(f"✅ WikiANN: {len(dataset_osint)} samples")
    print(f"   Columnas: {dataset_osint.column_names}")
    print(f"   Primer ejemplo: {dataset_osint[0]}")
    
except Exception as e:
    print(f"❌ Error cargando WikiANN: {e}")
    sys.exit(1)

try:
    print("\n📥 Cargando Amazon Polarity (Marketing)...")
    dataset_marketing = load_dataset(
        'mteb/amazon_polarity',
        split='train[:50000]',
        trust_remote_code=True,
        token=hf_token
    )
    print(f"✅ Amazon: {len(dataset_marketing)} samples")
    print(f"   Columnas: {dataset_marketing.column_names}")
    print(f"   Primer ejemplo: {dataset_marketing[0]}")
    
except Exception as e:
    print(f"❌ Error cargando Amazon: {e}")
    sys.exit(1)

print("\n" + "="*60)
print("✅ DATASETS CARGADOS EXITOSAMENTE")
print("="*60)

# Probar tokenización
print("\n" + "="*60)
print("FASE 2: Testing tokenización...")
print("="*60)

try:
    tokenizer_osint = AutoTokenizer.from_pretrained('bert-base-uncased')
    print("✅ Tokenizer OSINT cargado")
    
    # Test tokenización
    test_text = "Steve Jobs founded Apple Inc."
    tokens = tokenizer_osint.tokenize(test_text)
    print(f"   Ejemplo: '{test_text}'")
    print(f"   Tokens: {tokens}")
    
except Exception as e:
    print(f"❌ Error con tokenizer OSINT: {e}")
    sys.exit(1)

try:
    tokenizer_marketing = AutoTokenizer.from_pretrained('distilbert-base-uncased')
    print("✅ Tokenizer Marketing cargado")
    
    # Test tokenización
    test_review = "This product is amazing!"
    tokens = tokenizer_marketing.tokenize(test_review)
    print(f"   Ejemplo: '{test_review}'")
    print(f"   Tokens: {tokens}")
    
except Exception as e:
    print(f"❌ Error con tokenizer Marketing: {e}")
    sys.exit(1)

print("\n" + "="*60)
print("✅ TOKENIZACIÓN EXITOSA")
print("="*60)

print("\n" + "="*60)
print("RESUMEN DE DATOS")
print("="*60)

print(f"""
📊 DATASETS LISTOS:

  🔍 OSINT NER (WikiANN)
     • Samples: {len(dataset_osint):,}
     • Idioma: English
     • Tarea: Named Entity Recognition
     • Entidades: 9 clases (O, B-LOC, I-LOC, etc.)
     • Modelo: bert-base-uncased
     • Batch size: 8
     • Epochs: 2
     • ETA: ~15-20 min
     
  📈 MARKETING SENTIMENT (Amazon)
     • Samples: {len(dataset_marketing):,}
     • Tarea: Sentiment Classification
     • Clases: Positive/Negative
     • Modelo: distilbert-base-uncased
     • Batch size: 16
     • Epochs: 2
     • ETA: ~10-15 min

⏱️  TIEMPO TOTAL: ~30-40 minutos
💾 ALMACENAMIENTO: ~2GB

✅ SISTEMA LISTO PARA ENTRENAR
""")

print("="*60)
print("PRÓXIMOS PASOS:")
print("="*60)
print("""
Opción 1: Ejecutar en HuggingFace Pro Notebook
   1. Ve a: https://huggingface.co/Kimberlyindiva
   2. Crea notebook nuevo
   3. Copia celdas de: HF_PRO_NOTEBOOK_DUAL.md
   4. Ejecuta: Shift+Enter en cada celda
   
Opción 2: Ejecutar script dual training
   python3 ffi/chapel/hf_pro_dual_training.py --mode sequential

Opción 3: Ejecutar desde terminal con transformers Trainer
   python3 hf_pro_training.py --task osint
   python3 hf_pro_training.py --task marketing
""")

print("\n✅ VALIDACIÓN COMPLETADA - LISTO PARA ENTRENAR")
print("="*60)
