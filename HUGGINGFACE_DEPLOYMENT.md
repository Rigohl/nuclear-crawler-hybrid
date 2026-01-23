# Chapel Training Pipeline - HuggingFace Hub Deployment

## 🚀 Opciones de Deployment

### Opción 1: GitHub Actions (Rápido, <6 horas)
- ✅ Ya configurado
- ✅ Trigger automático en push
- ⚠️  Limitado a 6 horas por job
- 🔗 `.github/workflows/chapel-training-pipeline.yml`

**Uso:**
```bash
git push origin main
# Workflow automático ejecuta training
```

### Opción 2: HuggingFace Spaces (24/7, Continuo)
- ✅ Ejecución continua
- ✅ Interfaz Gradio visual
- ✅ Persistent storage
- ✅ Privado
- 🔗 `ffi/chapel/hf_spaces_app.py`

**Setup:**
```bash
# 1. Crear Spaces privado en HuggingFace
huggingface-cli repo create \
  --type space \
  --private \
  nuclear-chapel-training

# 2. Clonar y pushear
git clone https://huggingface.co/spaces/USERNAME/nuclear-chapel-training
cd nuclear-chapel-training
cp -r ../nuclear-crawler-hybrid/ffi/chapel/* .
git add . && git commit -m "Chapel training pipeline" && git push
```

### Opción 3: HuggingFace Hub (Privado)
- ✅ Almacenamiento de modelos
- ✅ Datasets privados
- ✅ Version control de checkpoints
- 🔗 `scripts/push_to_huggingface.sh`

**Setup:**
```bash
# 1. Generar HF token: https://huggingface.co/settings/tokens
export HF_TOKEN="hf_xxxxxxxxxxxx"

# 2. Push a repo privado
./scripts/push_to_huggingface.sh $HF_TOKEN nuclear-chapel-training
```

---

## 📋 Configuración Detallada

### HuggingFace Spaces (RECOMENDADO para training 24/7)

#### Requisitos:
1. Cuenta HuggingFace (gratuita)
2. Token con permisos de escritura
3. Espacio Docker o Gradio

#### Pasos:

**1. Crear Space Privado:**
```bash
# Via web: https://huggingface.co/new-space
# O via CLI:
huggingface-cli repo create \
  --repo-id="nuclear-chapel-training" \
  --type space \
  --private \
  --space-sdk docker \
  --space-hardware cpu-upgrade
```

**2. Clonar y Configurar:**
```bash
git clone https://huggingface.co/spaces/YOUR_USERNAME/nuclear-chapel-training
cd nuclear-chapel-training

# Copiar archivos del repo
cp -r /workspaces/nuclear-crawler-hybrid/ffi/chapel/* .

# Crear estructura
mkdir -p data/checkpoints data/datasets data/logs
```

**3. Crear requirements.txt:**
```
gradio==4.0+
huggingface-hub==0.18+
numpy==1.24+
scipy==1.11+
```

**4. Pushear a HuggingFace:**
```bash
git add .
git commit -m "Chapel training pipeline for continuous learning"
git push
```

#### La app Gradio mostrará:
- Status de Chapel environment
- Botones para iniciar training (1 o 4 locales)
- Log en tiempo real
- Dashboard de métricas
- Manage checkpoints

---

### HuggingFace Datasets (para almacenar datos)

**Crear dataset privado:**
```bash
# 1. Crear en web
# https://huggingface.co/new-dataset

# 2. O via CLI
huggingface-cli repo create \
  --repo-id="nuclear-training-data" \
  --type dataset \
  --private
```

**Upload datos:**
```bash
# Login
huggingface-cli login

# Subir dataset
python3 << 'PYTHON'
from huggingface_hub import HfApi

api = HfApi()

# Upload dataset
api.upload_folder(
    folder_path="ffi/chapel/data/train",
    repo_id="YOUR_USERNAME/nuclear-training-data",
    repo_type="dataset",
    private=True
)
PYTHON
```

---

## 🔄 Flujo de Entrenamiento Continuo

### En GitHub Actions:
```
Push → Trigger workflow
  ↓
Chapel environment setup
  ↓
Single-locale training (Layer 1,2,3)
  ↓
Multi-locale training (distributed)
  ↓
Metrics reported
  ↓
Artifacts uploaded (logs, reports)
```

### En HuggingFace Spaces:
```
Usuario accede a Space
  ↓
Click "Start Training"
  ↓
Chapel pipeline ejecuta en cloud
  ↓
Log actualiza en tiempo real
  ↓
Métricas se visualizan
  ↓
Checkpoint se guarda en persistent storage
  ↓
Siguiente iteración automática (si habilitado)
```

---

## 📊 Comparativa

| Característica | GitHub Actions | HF Spaces | HF Hub |
|---|---|---|---|
| Costo | Gratuito | Gratuito | Gratuito |
| Tiempo max | 6 horas | Ilimitado | N/A |
| Continuo | No (triggers) | Sí (24/7) | N/A |
| UI | Minimal | Gradio (visual) | Web |
| Privacidad | Privado | Privado | Privado |
| Storage | Artifacts (90d) | Persistent | Versioning |
| Paralelismo | Single job | Multi-locale ready | N/A |

---

## 🔐 Seguridad

### GitHub:
- Token almacenado en secrets
- Workflows visibles en repo público
- Artifacts privados (90 días)

### HuggingFace:
- Repo privado (no visible)
- Token en environment variable
- Persistent storage encriptado
- Access control por usuario

---

## 🚀 Recomendación

**Para máximo poder + continuo learning:**

```
+─────────────────────────────────────────────────+
│  1. HuggingFace Spaces (PRINCIPAL)              │
│     - Ejecución 24/7                            │
│     - UI visual Gradio                          │
│     - Storage persistente                       │
│     - Distribuido (4 locales)                   │
│                                                 │
│  2. GitHub Actions (BACKUPS)                    │
│     - Triggers on push/schedule                 │
│     - Validation pipeline                       │
│     - Reports en PRs                            │
│                                                 │
│  3. HF Hub (ALMACENAMIENTO)                     │
│     - Datasets versionados                      │
│     - Checkpoints de modelos                    │
│     - Control de acceso                         │
+─────────────────────────────────────────────────+
```

---

## 📝 Instrucciones Paso a Paso

### SETUP COMPLETO EN 5 MINUTOS:

**Paso 1: Crear HF token**
- Ir a: https://huggingface.co/settings/tokens
- Click "New token" → seleccionar "write"
- Copiar token

**Paso 2: Crear Space privado**
```bash
export HF_TOKEN="hf_xxxx"
huggingface-cli login --token $HF_TOKEN

huggingface-cli repo create \
  --repo-id="nuclear-chapel-training" \
  --type space \
  --private \
  --space-sdk docker
```

**Paso 3: Clonar y pushear**
```bash
git clone https://huggingface.co/spaces/YOUR_USERNAME/nuclear-chapel-training
cd nuclear-chapel-training

# Copiar archivos
cp -r /path/to/nuclear-crawler-hybrid/ffi/chapel/* .
cp -r /path/to/nuclear-crawler-hybrid/ffi/chapel/hf_spaces_app.py .
cp -r /path/to/nuclear-crawler-hybrid/ffi/chapel/Dockerfile.spaces Dockerfile

# Crear requirements.txt
echo "gradio\nhuggingface-hub\nnumpy\nscipy" > requirements.txt

# Push
git add . && git commit -m "Chapel training" && git push
```

**Paso 4: Acceder a Space**
- URL: `https://huggingface.co/spaces/YOUR_USERNAME/nuclear-chapel-training`
- Click "App" para abrir Gradio
- Iniciar training

---

## 🎯 Resultados Esperados

Después de setup, tendrás:

✅ **GitHub Actions** ejecutando cada push
✅ **HF Spaces** con UI visual para training
✅ **HF Hub** almacenando datasets y checkpoints
✅ **Continuous Learning** 24/7 sin interrupciones
✅ **Multi-locale** Chapel distribuido en cloud
✅ **Privado** - Todo encriptado y privado

---

**Status: 🟢 LISTO PARA PRODUCCIÓN**
