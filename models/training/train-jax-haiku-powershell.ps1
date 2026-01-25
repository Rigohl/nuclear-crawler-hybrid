# Train JAX/Haiku Models with PowerShell and .NET Information
# Entrena modelos JAX/Haiku con información de PowerShell y .NET
#Requires -Version 7.2

using namespace System
using namespace System.IO
using namespace System.Text
using namespace System.Collections.Generic

$ErrorActionPreference = 'Stop'

Write-Host @"
╔════════════════════════════════════════════════════════════════╗
║     Train JAX/Haiku with PowerShell + .NET Data             ║
╚════════════════════════════════════════════════════════════════╝
"@ -ForegroundColor Cyan

# =============================================================================
# Data Collection
# =============================================================================

class PowerShellDataCollector {
    [string]$ProjectPath
    [List[string]]$MarkdownFiles
    [List[string]]$PowerShellFiles
    [List[string]]$DocumentationFiles
    
    PowerShellDataCollector([string]$path) {
        $this.ProjectPath = $path
        $this.MarkdownFiles = [List[string]]::new()
        $this.PowerShellFiles = [List[string]]::new()
        $this.DocumentationFiles = [List[string]]::new()
    }
    
    [void] CollectAllData() {
        Write-Host "`nRecopilando datos de PowerShell y .NET..." -ForegroundColor Yellow
        
        # Markdown files
        $this.CollectMarkdownFiles()
        
        # PowerShell scripts
        $this.CollectPowerShellFiles()
        
        # Documentation files
        $this.CollectDocumentationFiles()
        
        Write-Host "  ✓ Markdown: $($this.MarkdownFiles.Count) archivos" -ForegroundColor Green
        Write-Host "  ✓ PowerShell: $($this.PowerShellFiles.Count) archivos" -ForegroundColor Green
        Write-Host "  ✓ Documentation: $($this.DocumentationFiles.Count) archivos" -ForegroundColor Green
    }
    
    [void] CollectMarkdownFiles() {
        $patterns = @("*.md", "*.MD")
        $files = Get-ChildItem -Path $this.ProjectPath -Recurse -Include $patterns -File | 
            Where-Object { $_.FullName -notmatch "node_modules|\.git" }
        
        foreach ($file in $files) {
            $this.MarkdownFiles.Add($file.FullName)
        }
    }
    
    [void] CollectPowerShellFiles() {
        $patterns = @("*.ps1", "*.psm1", "*.psd1")
        $files = Get-ChildItem -Path $this.ProjectPath -Recurse -Include $patterns -File | 
            Where-Object { $_.FullName -notmatch "node_modules|\.git" }
        
        foreach ($file in $files) {
            $this.PowerShellFiles.Add($file.FullName)
        }
    }
    
    [void] CollectDocumentationFiles() {
        # Archivos de documentación específicos
        $docFiles = @(
            "POWERSHELL-NET-COMPLETE.md",
            "POWERSHELL-ADVANCED-COMPLETE.md",
            "POWERSHELL-REPOS-COMPLETE.md",
            "NUGET-PACKAGES-COMPLETE.md",
            "POWERSHELL-NET-COMPLETE.md"
        )
        
        foreach ($docFile in $docFiles) {
            $path = Join-Path $this.ProjectPath $docFile
            if (Test-Path $path) {
                $this.DocumentationFiles.Add($path)
            }
        }
    }
    
    [string] CreateTrainingDataset([string]$outputPath) {
        Write-Host "`nCreando dataset de entrenamiento..." -ForegroundColor Yellow
        
        $outputDir = Split-Path $outputPath -Parent
        if (-not (Test-Path $outputDir)) {
            New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
        }
        
        $allData = [List[hashtable]]::new()
        
        # Procesar Markdown
        foreach ($file in $this.MarkdownFiles) {
            try {
                $content = Get-Content -Path $file -Raw -Encoding UTF8
                $allData.Add(@{
                    source = "markdown"
                    file = $file
                    content = $content
                    type = "documentation"
                })
            }
            catch {
                Write-Warning "Error leyendo $file : $_"
            }
        }
        
        # Procesar PowerShell
        foreach ($file in $this.PowerShellFiles) {
            try {
                $content = Get-Content -Path $file -Raw -Encoding UTF8
                $allData.Add(@{
                    source = "powershell"
                    file = $file
                    content = $content
                    type = "code"
                })
            }
            catch {
                Write-Warning "Error leyendo $file : $_"
            }
        }
        
        # Procesar Documentation
        foreach ($file in $this.DocumentationFiles) {
            try {
                $content = Get-Content -Path $file -Raw -Encoding UTF8
                $allData.Add(@{
                    source = "documentation"
                    file = $file
                    content = $content
                    type = "documentation"
                })
            }
            catch {
                Write-Warning "Error leyendo $file : $_"
            }
        }
        
        # Guardar como JSONL
        $stream = [StreamWriter]::new($outputPath, $false, [System.Text.Encoding]::UTF8)
        
        try {
            foreach ($item in $allData) {
                $json = @{
                    id = [Guid]::NewGuid().ToString()
                    source = $item.source
                    file = $item.file
                    content = $item.content
                    type = $item.type
                    timestamp = (Get-Date).ToString("o")
                } | ConvertTo-Json -Depth 5 -Compress
                
                $stream.WriteLine($json)
            }
        }
        finally {
            $stream.Close()
        }
        
        Write-Host "  ✓ Dataset creado: $outputPath" -ForegroundColor Green
        Write-Host "  ✓ Total registros: $($allData.Count)" -ForegroundColor Green
        
        return $outputPath
    }
}

# =============================================================================
# JAX/Haiku Training Script
# =============================================================================

function New-JAXHaikuTrainingScript {
    param(
        [string]$DatasetPath,
        [string]$OutputPath = "$env:USERPROFILE\.jax-ml\models\powershell-haiku"
    )
    
    $pythonScript = @"
import os
import json
import jax
import jax.numpy as jnp
import haiku as hk
import optax
from transformers import AutoTokenizer, AutoModel
import torch
import numpy as np
from typing import Dict, List, Tuple
import pickle
from sklearn.decomposition import PCA

print("[OK] Iniciando entrenamiento JAX/Haiku con datos PowerShell/.NET")

# Configuración
dataset_path = r'$DatasetPath'
output_path = r'$OutputPath'
batch_size = 8
learning_rate = 0.001
num_epochs = 10
embedding_dim = 256
hidden_dim = 512

# Crear directorio de salida
os.makedirs(output_path, exist_ok=True)

# Cargar dataset
print("[OK] Cargando dataset...")
training_data = []
with open(dataset_path, 'r', encoding='utf-8') as f:
    for line in f:
        if line.strip():
            training_data.append(json.loads(line))

print(f"[OK] Dataset cargado: {len(training_data)} registros")

# Cargar CodeBERT para embeddings
print("[OK] Cargando CodeBERT para embeddings...")
try:
    tokenizer = AutoTokenizer.from_pretrained('microsoft/codebert-base')
    codebert_model = AutoModel.from_pretrained('microsoft/codebert-base')
    print("[OK] CodeBERT cargado")
except Exception as e:
    print(f"[ERROR] Error cargando CodeBERT: {e}")
    raise

# Función para obtener embeddings
def get_embeddings(texts: List[str], max_length: int = 512) -> np.ndarray:
    """Obtener embeddings de CodeBERT"""
    embeddings_list = []
    for text in texts:
        try:
            # Truncar si es muy largo
            if len(text) > max_length * 4:  # Aproximado
                text = text[:max_length * 4]
            
            inputs = tokenizer(text, return_tensors='pt', truncation=True, max_length=max_length, padding=True)
            with torch.no_grad():
                outputs = codebert_model(**inputs)
                # Usar el embedding promedio del último hidden state
                embedding = outputs.last_hidden_state.mean(dim=1).squeeze().numpy()
                embeddings_list.append(embedding)
        except Exception as e:
            print(f"[WARN] Error procesando texto: {e}")
            # Embedding cero como fallback
            embeddings_list.append(np.zeros(768))
    
    return np.array(embeddings_list)

# Definir modelo Haiku
def haiku_model(x):
    """Modelo de red neuronal con Haiku"""
    # Capa de entrada
    x = hk.Linear(embedding_dim)(x)
    x = jax.nn.relu(x)
    
    # Capas ocultas
    x = hk.Linear(hidden_dim)(x)
    x = jax.nn.relu(x)
    x = hk.Dropout(0.2)(x, True)
    
    x = hk.Linear(hidden_dim)(x)
    x = jax.nn.relu(x)
    x = hk.Dropout(0.2)(x, True)
    
    # Capa de salida (clasificación o regresión)
    x = hk.Linear(embedding_dim)(x)
    return x

# Transformar a función Haiku
haiku_fn = hk.transform(haiku_model)

# Preparar datos
print("[OK] Preparando datos para entrenamiento...")
texts = [item['content'][:2000] for item in training_data[:100]]  # Limitar para prueba
print(f"[OK] Procesando {len(texts)} textos...")

# Obtener embeddings
print("[OK] Generando embeddings con CodeBERT...")
embeddings = get_embeddings(texts)
print(f"[OK] Embeddings generados: shape {embeddings.shape}")

# Reducir dimensión si es necesario
pca = None
if embeddings.shape[1] > embedding_dim:
    pca = PCA(n_components=embedding_dim)
    embeddings = pca.fit_transform(embeddings)
    print(f"[OK] Dimensión reducida a {embedding_dim}")
else:
    # Si es menor, pad con ceros
    if embeddings.shape[1] < embedding_dim:
        padding = np.zeros((embeddings.shape[0], embedding_dim - embeddings.shape[1]))
        embeddings = np.hstack([embeddings, padding])
        print(f"[OK] Dimensión aumentada a {embedding_dim}")

# Convertir a JAX arrays
embeddings_jax = jnp.array(embeddings)

# Inicializar modelo
print("[OK] Inicializando modelo Haiku...")
rng = jax.random.PRNGKey(42)
params = haiku_fn.init(rng, embeddings_jax[:batch_size])
print("[OK] Modelo inicializado")

# Optimizador
optimizer = optax.adam(learning_rate)
opt_state = optimizer.init(params)

# Función de pérdida
def loss_fn(params, x, y):
    pred = haiku_fn.apply(params, rng, x)
    return jnp.mean((pred - y) ** 2)

# Gradiente
grad_fn = jax.grad(loss_fn)

# Entrenamiento
print("[OK] Iniciando entrenamiento...")
for epoch in range(num_epochs):
    epoch_loss = 0.0
    num_batches = 0
    
    # Mini-batches
    for i in range(0, len(embeddings_jax), batch_size):
        batch_x = embeddings_jax[i:i+batch_size]
        batch_y = embeddings_jax[i:i+batch_size]  # Autoencoder: target = input
        
        # Calcular gradientes
        grads = grad_fn(params, batch_x, batch_y)
        
        # Actualizar parámetros
        updates, opt_state = optimizer.update(grads, opt_state)
        params = optax.apply_updates(params, updates)
        
        # Calcular pérdida
        loss = loss_fn(params, batch_x, batch_y)
        epoch_loss += float(loss)
        num_batches += 1
    
    avg_loss = epoch_loss / num_batches if num_batches > 0 else 0.0
    print(f"[OK] Epoch {epoch+1}/{num_epochs}, Loss: {avg_loss:.4f}")

print("[OK] Entrenamiento completado")

# Guardar modelo
print("[OK] Guardando modelo...")
model_path = os.path.join(output_path, "haiku_model.pkl")
with open(model_path, 'wb') as f:
    pickle.dump({
        'params': params,
        'pca': pca if 'pca' in locals() else None,
        'embedding_dim': embedding_dim,
        'hidden_dim': hidden_dim
    }, f)

print(f"[OK] Modelo guardado en: {model_path}")

# Guardar metadata
metadata = {
    'num_samples': len(training_data),
    'num_epochs': num_epochs,
    'batch_size': batch_size,
    'learning_rate': learning_rate,
    'embedding_dim': embedding_dim,
    'hidden_dim': hidden_dim,
    'model_path': model_path
}

metadata_path = os.path.join(output_path, "metadata.json")
with open(metadata_path, 'w', encoding='utf-8') as f:
    json.dump(metadata, f, indent=2)

print(f"[OK] Metadata guardada en: {metadata_path}")
print(f"\n[OK] Entrenamiento completado exitosamente!")
print(f"[OK] Modelo disponible en: {output_path}")
"@
    
    return $pythonScript
}

# =============================================================================
# Main Training Function
# =============================================================================

function Start-JAXHaikuTraining {
    param(
        [string]$ProjectPath = ".",
        [string]$OutputPath = "$env:USERPROFILE\.jax-ml\models\powershell-haiku"
    )
    
    Write-Host "`nIniciando entrenamiento JAX/Haiku..." -ForegroundColor Cyan
    
    # 1. Recopilar datos
    $collector = [PowerShellDataCollector]::new($ProjectPath)
    $collector.CollectAllData()
    
    # 2. Crear dataset
    $datasetPath = "$env:USERPROFILE\.jax-ml\training\powershell-net-dataset.jsonl"
    $datasetDir = Split-Path $datasetPath -Parent
    if (-not (Test-Path $datasetDir)) {
        New-Item -ItemType Directory -Path $datasetDir -Force | Out-Null
    }
    
    $datasetPath = $collector.CreateTrainingDataset($datasetPath)
    
    # 3. Crear script de entrenamiento
    $trainingScript = New-JAXHaikuTrainingScript -DatasetPath $datasetPath -OutputPath $OutputPath
    
    # 4. Guardar y ejecutar script
    $scriptPath = Join-Path $env:TEMP "train_jax_haiku_$([guid]::NewGuid().ToString('N')).py"
    $trainingScript | Out-File -FilePath $scriptPath -Encoding UTF8
    
    Write-Host "`nEjecutando entrenamiento..." -ForegroundColor Yellow
    Write-Host "  Esto puede tomar varios minutos..." -ForegroundColor Gray
    
    try {
        python $scriptPath
        Write-Host "`n✓ Entrenamiento completado exitosamente!" -ForegroundColor Green
        Write-Host "  Modelo guardado en: $OutputPath" -ForegroundColor Cyan
    }
    catch {
        Write-Error "Error durante el entrenamiento: $_"
    }
    finally {
        Remove-Item $scriptPath -ErrorAction SilentlyContinue
    }
}

# Si se ejecuta directamente
if ($MyInvocation.InvocationName -ne '.') {
    Start-JAXHaikuTraining
}
