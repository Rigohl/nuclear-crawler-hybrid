#!/usr/bin/env python3
"""MINIMAL: Upload PowerShell dataset to HF"""
from huggingface_hub import HfApi, login
import os

# Tu token
token = os.getenv("HF_TOKEN") or input("HF Token: ").strip()
if not token:
    print("Usando token guardado...")

api = HfApi(token=token)

# Crear repo
try:
    api.create_repo(
        repo_id="Kimberlyindiva/powershell-devops-mega-dataset",
        repo_type="dataset",
        exist_ok=True
    )
    print("Repo creado/verificado")
except:
    pass

# Upload file
api.upload_file(
    path_or_fileobj="D:/models/powershell_dataset/dataset.json",
    path_in_repo="dataset.json",
    repo_id="Kimberlyindiva/powershell-devops-mega-dataset",
    repo_type="dataset"
)

print("\nSUBIDO!")
print("URL: https://huggingface.co/datasets/Kimberlyindiva/powershell-devops-mega-dataset")
print("Bot HF convertira a Parquet automaticamente")
