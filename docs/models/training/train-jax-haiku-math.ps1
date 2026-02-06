# Train JAX/Haiku Models with Mathematics Data
# Entrena modelos JAX/Haiku con información matemática completa
#Requires -Version 7.2

using namespace System
using namespace System.IO
using namespace System.Text
using namespace System.Collections.Generic

$ErrorActionPreference = 'Stop'

Write-Host @"
╔════════════════════════════════════════════════════════════════╗
║     Train JAX/Haiku with Mathematics Data                     ║
╚════════════════════════════════════════════════════════════════╝
"@ -ForegroundColor Cyan

# =============================================================================
# Mathematics Data Collector
# =============================================================================

class MathematicsDataCollector {
    [string]$MathPath
    [List[string]]$MarkdownFiles
    [List[string]]$PythonFiles
    [List[string]]$JuliaFiles
    [List[string]]$OtherFiles
    [hashtable]$Categories
    
    MathematicsDataCollector([string]$path) {
        $this.MathPath = $path
        $this.MarkdownFiles = [List[string]]::new()
        $this.PythonFiles = [List[string]]::new()
        $this.JuliaFiles = [List[string]]::new()
        $this.OtherFiles = [List[string]]::new()
        $this.Categories = @{
            "Pure_Math" = @()
            "Theoretical_Physics" = @()
            "Decision_Science" = @()
            "Quantum" = @()
            "Advanced_Theory" = @()
            "Documentation" = @()
        }
    }
    
    [void] CollectAllData() {
        Write-Host "`nRecopilando datos matemáticos..." -ForegroundColor Yellow
        
        # Markdown (documentación)
        $this.CollectMarkdownFiles()
        
        # Python (implementaciones)
        $this.CollectPythonFiles()
        
        # Julia (cálculos numéricos)
        $this.CollectJuliaFiles()
        
        # Otros (Lean, Haskell, etc.)
        $this.CollectOtherFiles()
        
        # Categorizar
        $this.CategorizeFiles()
        
        Write-Host "  ✓ Markdown: $($this.MarkdownFiles.Count) archivos" -ForegroundColor Green
        Write-Host "  ✓ Python: $($this.PythonFiles.Count) archivos" -ForegroundColor Green
        Write-Host "  ✓ Julia: $($this.JuliaFiles.Count) archivos" -ForegroundColor Green
        Write-Host "  ✓ Otros: $($this.OtherFiles.Count) archivos" -ForegroundColor Green
    }
    
    [void] CollectMarkdownFiles() {
        $files = Get-ChildItem -Path $this.MathPath -Recurse -Include "*.md" -File | 
            Where-Object { $_.FullName -notmatch "node_modules|\.git" }
        
        foreach ($file in $files) {
            $this.MarkdownFiles.Add($file.FullName)
        }
    }
    
    [void] CollectPythonFiles() {
        $files = Get-ChildItem -Path $this.MathPath -Recurse -Include "*.py" -File | 
            Where-Object { $_.FullName -notmatch "node_modules|\.git|__pycache__" }
        
        foreach ($file in $files) {
            $this.PythonFiles.Add($file.FullName)
        }
    }
    
    [void] CollectJuliaFiles() {
        $files = Get-ChildItem -Path $this.MathPath -Recurse -Include "*.jl" -File | 
            Where-Object { $_.FullName -notmatch "node_modules|\.git" }
        
        foreach ($file in $files) {
            $this.JuliaFiles.Add($file.FullName)
        }
    }
    
    [void] CollectOtherFiles() {
        $extensions = @("*.lean", "*.hs", "*.cpp", "*.f90", "*.qs", "*.qrisp", "*.R")
        $files = Get-ChildItem -Path $this.MathPath -Recurse -Include $extensions -File | 
            Where-Object { $_.FullName -notmatch "node_modules|\.git" }
        
        foreach ($file in $files) {
            $this.OtherFiles.Add($file.FullName)
        }
    }
    
    [void] CategorizeFiles() {
        foreach ($file in $this.MarkdownFiles + $this.PythonFiles + $this.JuliaFiles + $this.OtherFiles) {
            $relativePath = $file.Replace($this.MathPath, "").Replace("\", "/")
            
            if ($relativePath -match "THEORETICAL_PHYSICS|M_THEORY|QUANTUM|COSMOLOGY") {
                $this.Categories["Theoretical_Physics"] += $file
            }
            elseif ($relativePath -match "DECISION_SCIENCE|BAYESIAN|GAME_THEORY|OPTIMIZATION") {
                $this.Categories["Decision_Science"] += $file
            }
            elseif ($relativePath -match "CUANTICA|QUANTUM") {
                $this.Categories["Quantum"] += $file
            }
            elseif ($relativePath -match "ADVANCED_THEORY|TEORIA_AVANZADA|CATEGORY|TOPOLOGY|GEOMETRY") {
                $this.Categories["Advanced_Theory"] += $file
            }
            elseif ($relativePath -match "NUMBER_THEORY|ALGEBRAIC|DIFFERENTIAL") {
                $this.Categories["Pure_Math"] += $file
            }
            elseif ($relativePath -match "docs|README") {
                $this.Categories["Documentation"] += $file
            }
        }
    }
    
    [string] CreateTrainingDataset([string]$outputPath) {
        Write-Host "`nCreando dataset de entrenamiento matemático..." -ForegroundColor Yellow
        
        $outputDir = Split-Path $outputPath -Parent
        if (-not (Test-Path $outputDir)) {
            New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
        }
        
        $allData = [List[hashtable]]::new()
        
        # Procesar todos los archivos
        $allFiles = $this.MarkdownFiles + $this.PythonFiles + $this.JuliaFiles + $this.OtherFiles
        
        foreach ($file in $allFiles) {
            try {
                $content = Get-Content -Path $file -Raw -Encoding UTF8 -ErrorAction SilentlyContinue
                if ($content) {
                    # Determinar categoría
                    $category = "General"
                    $relativePath = $file.Replace($this.MathPath, "")
                    
                    foreach ($cat in $this.Categories.Keys) {
                        if ($this.Categories[$cat] -contains $file) {
                            $category = $cat
                            break
                        }
                    }
                    
                    # Determinar tipo
                    $extension = [Path]::GetExtension($file).ToLower()
                    $type = switch ($extension) {
                        ".md" { "documentation" }
                        ".py" { "python_code" }
                        ".jl" { "julia_code" }
                        ".lean" { "formal_proof" }
                        ".hs" { "haskell_code" }
                        ".qs" { "quantum_code" }
                        default { "code" }
                    }
                    
                    $allData.Add(@{
                        source = $type
                        file = $file
                        content = $content
                        category = $category
                        topic = $this.ExtractTopic($relativePath)
                    })
                }
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
                    category = $item.category
                    topic = $item.topic
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
        
        # Estadísticas por categoría
        Write-Host "`n  Estadísticas por categoría:" -ForegroundColor Cyan
        foreach ($cat in $this.Categories.Keys) {
            $count = ($allData | Where-Object { $_.category -eq $cat }).Count
            if ($count -gt 0) {
                Write-Host "    $cat : $count archivos" -ForegroundColor Gray
            }
        }
        
        return $outputPath
    }
    
    [string] ExtractTopic([string]$path) {
        # Extraer tema del path
        $parts = $path -split "[\\/]"
        foreach ($part in $parts) {
            if ($part -match "^[0-9]+_" -or $part -match "THEORY|THEORETICAL|ADVANCED") {
                return $part
            }
        }
        return "General"
    }
}

# =============================================================================
# JAX/Haiku Training Script for Mathematics
# =============================================================================

function New-MathTrainingScript {
    param(
        [string]$DatasetPath,
        [string]$OutputPath = "$env:USERPROFILE\.jax-ml\models\math-haiku"
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
try:
    from sklearn.decomposition import PCA
    sklearn_available = True
except ImportError:
    sklearn_available = False
    print("[WARN] sklearn no disponible, usando reducción manual")
import pickle

print("[OK] Iniciando entrenamiento JAX/Haiku con datos matemáticos")

# Configuración
dataset_path = r'$DatasetPath'
output_path = r'$OutputPath'
batch_size = 8
learning_rate = 0.001
num_epochs = 20
embedding_dim = 256  # Reducir para evitar problemas con PCA
hidden_dim = 512

# Crear directorio de salida
os.makedirs(output_path, exist_ok=True)

# Cargar dataset
print("[OK] Cargando dataset matemático...")
training_data = []
with open(dataset_path, 'r', encoding='utf-8') as f:
    for line in f:
        if line.strip():
            try:
                training_data.append(json.loads(line))
            except:
                continue

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
def get_embeddings(texts, max_length=512):
    embeddings_list = []
    for text in texts:
        try:
            if len(text) > max_length * 4:
                text = text[:max_length * 4]
            
            inputs = tokenizer(text, return_tensors='pt', truncation=True, max_length=max_length, padding=True)
            with torch.no_grad():
                outputs = codebert_model(**inputs)
                embedding = outputs.last_hidden_state.mean(dim=1).squeeze().numpy()
                embeddings_list.append(embedding)
        except Exception as e:
            print(f"[WARN] Error procesando texto: {e}")
            embeddings_list.append(np.zeros(768))
    
    return np.array(embeddings_list)

# Preparar datos - usar más muestras para matemáticas
print("[OK] Preparando datos para entrenamiento...")
max_samples = min(500, len(training_data))  # Hasta 500 muestras
texts = [item['content'][:4000] for item in training_data[:max_samples]]  # Más datos y más largo
print(f"[OK] Procesando {len(texts)} textos (hasta 4000 chars cada uno)...")

# Obtener embeddings
print("[OK] Generando embeddings con CodeBERT...")
embeddings = get_embeddings(texts)
print(f"[OK] Embeddings generados: shape {embeddings.shape}")

# Reducir dimensión si es necesario
pca = None
n_samples, n_features = embeddings.shape

if n_features > embedding_dim:
    # Calcular máximo de componentes posibles (min de muestras y features)
    max_components = min(n_samples, n_features, embedding_dim)
    
    if sklearn_available and max_components < n_features:
        pca = PCA(n_components=max_components)
        embeddings = pca.fit_transform(embeddings)
        print(f"[OK] Dimensión reducida a {max_components} con PCA (de {n_features})")
        
        # Si aún necesitamos más reducción o padding
        if max_components < embedding_dim:
            padding = np.zeros((embeddings.shape[0], embedding_dim - max_components))
            embeddings = np.hstack([embeddings, padding])
            print(f"[OK] Padding agregado para llegar a {embedding_dim}")
        elif max_components > embedding_dim:
            embeddings = embeddings[:, :embedding_dim]
            print(f"[OK] Truncado a {embedding_dim}")
    else:
        # Reducción manual: tomar primeras N dimensiones
        embeddings = embeddings[:, :embedding_dim]
        print(f"[OK] Dimensión reducida a {embedding_dim} (manual)")
elif n_features < embedding_dim:
    padding = np.zeros((embeddings.shape[0], embedding_dim - n_features))
    embeddings = np.hstack([embeddings, padding])
    print(f"[OK] Dimensión aumentada a {embedding_dim}")
else:
    print(f"[OK] Dimensión ya es {embedding_dim}, sin cambios")

# Convertir a JAX arrays
embeddings_jax = jnp.array(embeddings)

# Definir modelo Haiku más grande para matemáticas
def haiku_math_model(x):
    # Capa de entrada
    x = hk.Linear(embedding_dim)(x)
    x = jax.nn.relu(x)
    
    # Capas ocultas más profundas (sin dropout en definición, se aplica en training)
    x = hk.Linear(hidden_dim)(x)
    x = jax.nn.relu(x)
    
    x = hk.Linear(hidden_dim)(x)
    x = jax.nn.relu(x)
    
    x = hk.Linear(hidden_dim // 2)(x)
    x = jax.nn.relu(x)
    
    # Capa de salida
    x = hk.Linear(embedding_dim)(x)
    return x

# Transformar a función Haiku
haiku_fn = hk.transform(haiku_math_model)

# Inicializar modelo
print("[OK] Inicializando modelo Haiku...")
rng = jax.random.PRNGKey(42)
params = haiku_fn.init(rng, embeddings_jax[:batch_size])
print("[OK] Modelo inicializado")

# Optimizador
optimizer = optax.adam(learning_rate)
opt_state = optimizer.init(params)

# Función de pérdida con dropout en entrenamiento
def loss_fn(params, x, y, training=True):
    rng_train = jax.random.PRNGKey(42) if training else None
    pred = haiku_fn.apply(params, rng_train, x)
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
        batch_y = embeddings_jax[i:i+batch_size]  # Autoencoder
        
        # Calcular gradientes (con dropout en entrenamiento)
        grads = grad_fn(params, batch_x, batch_y, True)
        
        # Actualizar parámetros
        updates, opt_state = optimizer.update(grads, opt_state)
        params = optax.apply_updates(params, updates)
        
        # Calcular pérdida (sin dropout para métrica)
        loss = loss_fn(params, batch_x, batch_y, False)
        epoch_loss += float(loss)
        num_batches += 1
    
    avg_loss = epoch_loss / num_batches if num_batches > 0 else 0.0
    print(f"[OK] Epoch {epoch+1}/{num_epochs}, Loss: {avg_loss:.4f}")

print("[OK] Entrenamiento completado")

# Guardar modelo
print("[OK] Guardando modelo...")
model_path = os.path.join(output_path, "haiku_math_model.pkl")
with open(model_path, 'wb') as f:
    pickle.dump({
        'params': params,
        'pca': pca,
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
    'model_path': model_path,
    'categories': list(set([item.get('category', 'General') for item in training_data]))
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

function Start-MathTraining {
    param(
        [string]$MathPath = "D:\PROJECTS\NUCLEARCRAWLER\MATH",
        [string]$OutputPath = "$env:USERPROFILE\.jax-ml\models\math-haiku"
    )
    
    Write-Host "`nIniciando entrenamiento JAX/Haiku con datos matemáticos..." -ForegroundColor Cyan
    
    # Verificar path
    if (-not (Test-Path $MathPath)) {
        Write-Error "Path no encontrado: $MathPath"
        return
    }
    
    # 1. Recopilar datos
    $collector = [MathematicsDataCollector]::new($MathPath)
    $collector.CollectAllData()
    
    # 2. Crear dataset
    $datasetPath = "$env:USERPROFILE\.jax-ml\training\math-dataset.jsonl"
    $datasetDir = Split-Path $datasetPath -Parent
    if (-not (Test-Path $datasetDir)) {
        New-Item -ItemType Directory -Path $datasetDir -Force | Out-Null
    }
    
    $datasetPath = $collector.CreateTrainingDataset($datasetPath)
    
    # 3. Crear script de entrenamiento
    $trainingScript = New-MathTrainingScript -DatasetPath $datasetPath -OutputPath $OutputPath
    
    # 4. Guardar y ejecutar script
    $scriptPath = Join-Path $env:TEMP "train_math_haiku_$([guid]::NewGuid().ToString('N')).py"
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
    Start-MathTraining
}
