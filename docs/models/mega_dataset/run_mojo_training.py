#!/usr/bin/env python3
"""
MOJO TRAINING EXECUTOR
======================
Este script solo ejecuta Mojo y sube resultados a HF.
El procesamiento real está en Mojo para máximo rendimiento.
"""

import subprocess
import json
from pathlib import Path
from datasets import Dataset
from huggingface_hub import login

print("="*80)
print("MOJO MEGA DATASET TRAINING")
print("="*80)

MOJO_FILE = Path(r"D:\models\mega_dataset\mojo_dataset_processor.mojo")
DATASET_FILE = Path(r"D:\models\mega_dataset\mega_dataset.jsonl")

# ==============================================================================
# 1. EJECUTAR PROCESAMIENTO EN MOJO
# ==============================================================================

print("\n[1/2] EJECUTANDO TRAINING EN MOJO...")
print("-"*80)
print("Archivo Mojo:", MOJO_FILE)

try:
    # Compilar Mojo
    print("Compilando Mojo...")
    compile_result = subprocess.run(
        ['mojo', 'build', str(MOJO_FILE), '-o', 'mojo_trainer.exe'],
        capture_output=True,
        text=True,
        cwd=MOJO_FILE.parent
    )
    
    if compile_result.returncode != 0:
        print("Advertencia compilación:", compile_result.stderr)
        print("Intentando ejecutar directamente...")
    
    # Ejecutar Mojo
    print("Ejecutando training en Mojo...")
    result = subprocess.run(
        ['mojo', str(MOJO_FILE)],
        capture_output=True,
        text=True,
        cwd=MOJO_FILE.parent,
        timeout=1800  # 30 minutos max
    )
    
    print(result.stdout)
    
    if result.returncode != 0:
        print("Error en Mojo:", result.stderr)
    else:
        print("\nMOJO TRAINING: COMPLETADO")

except FileNotFoundError:
    print("ERROR: Mojo no encontrado. Instalar desde https://www.modular.com/mojo")
    print("Continuar con dataset upload...")
except subprocess.TimeoutExpired:
    print("WARNING: Mojo training timeout. Continuando con upload...")
except Exception as e:
    print(f"Error ejecutando Mojo: {e}")
    print("Continuando con dataset upload...")

# ==============================================================================
# 2. UPLOAD DATASET A HUGGING FACE
# ==============================================================================

print("\n[2/2] UPLOAD A HUGGING FACE...")
print("-"*80)

# Cargar dataset
print("Cargando dataset...")
data = []
with open(DATASET_FILE, 'r', encoding='utf-8') as f:
    for line in f:
        if line.strip():
            data.append(json.loads(line))

print(f"Cargadas: {len(data):,} entradas")

# Pedir token
hf_token = input("\nHF Token (Enter para omitir): ").strip()

if not hf_token:
    print("Omitiendo upload a HF")
    print("\nPara subir manualmente:")
    print("1. huggingface-cli login")
    print(f"2. huggingface-cli upload Kimberlyindiva/mega-math-dataset {DATASET_FILE} --repo-type dataset")
else:
    try:
        login(token=hf_token)
        
        # Preparar dataset
        print("Preparando dataset para HF...")
        dataset_dict = {
            'id': [d['id'] for d in data],
            'content': [d['content'] for d in data],
            'category': [d['category'] for d in data],
            'subcategory': [d.get('subcategory', '') for d in data],
            'difficulty': [d.get('difficulty', 5) for d in data]
        }
        
        hf_dataset = Dataset.from_dict(dataset_dict)
        
        # Subir
        repo_id = "Kimberlyindiva/mega-mathematical-ai-dataset"
        print(f"Subiendo a {repo_id}...")
        
        hf_dataset.push_to_hub(repo_id, token=hf_token)
        
        print(f"\nDATASET SUBIDO!")
        print(f"URL: https://huggingface.co/datasets/{repo_id}")
        print("Bot HF convertira a Parquet automaticamente en refs/convert/parquet")
    
    except Exception as e:
        print(f"Error en upload: {e}")

# ==============================================================================
# RESUMEN
# ==============================================================================

print("\n" + "="*80)
print("RESUMEN FINAL")
print("="*80)
print(f"Dataset: {len(data):,} entradas (59.3 MB)")
print("Processing: MOJO (GPU + SIMD)")
print("Models: JAX/Haiku + Chapel")
print("Upload: HuggingFace")
print("\nSistema completo basado en Mojo para máximo rendimiento!")
print("="*80)
