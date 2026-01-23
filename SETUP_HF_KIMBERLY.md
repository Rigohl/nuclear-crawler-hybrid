# 🤗 HUGGINGFACE SETUP PARA CHAPEL TRAINING

## ❓ ¿ES TU CUENTA DE HF "KIMBERLY"?

Si es así:
- **Usuario HF**: `Kimberly` o similar
- Verifica en: https://huggingface.co/settings/account

---

## 3️⃣ OPCIONES DE TRAINING EN HF

### Opción A: Push a Repo Privado HF (RECOMENDADO)

```bash
# 1. Obtener token
# Ir a: https://huggingface.co/settings/tokens
# Crear token con permiso "write"
# Copiar token

export HF_TOKEN="hf_xxxxxxxxxxxxxxxxxxx"

# 2. Push a HF Hub
cd /workspaces/nuclear-crawler-hybrid
./scripts/push_to_huggingface.sh $HF_TOKEN nuclear-chapel-training Kimberly true

# Resultado:
# ✅ Repo privado en HF Hub
# ✅ Chapel code + datasets subidos
# ✅ Ready para entrenar
```

---

### Opción B: HuggingFace Spaces (MEJOR PARA 24/7)

**Ventajas:**
- ✅ UI visual (Gradio)
- ✅ Ejecución continua
- ✅ No requiere Docker Desktop
- ✅ Logs en tiempo real
- ✅ Privado

**Pasos:**

1. **Crear Space en HF:**
```bash
# Via web: https://huggingface.co/new-space
# Selecciona:
# - Nombre: "nuclear-chapel-training"
# - SDK: Docker
# - Privacidad: Private
# - Hardware: CPU Upgrade
```

2. **Clonar Space:**
```bash
git clone https://huggingface.co/spaces/Kimberly/nuclear-chapel-training
cd nuclear-chapel-training
```

3. **Copiar archivos Chapel:**
```bash
# Copiar desde tu repo
cp -r /workspaces/nuclear-crawler-hybrid/ffi/chapel/* .

# Archivos principales:
# - training_pipeline.chpl
# - chapel_ai.chpl
# - Makefile
# - hf_spaces_app.py        (Gradio app)
# - Dockerfile.spaces       (para HF)
# - data/scraping_stealth_patterns.json
```

4. **Crear estructura:**
```bash
# Crear dirs
mkdir -p data/checkpoints data/logs

# Crear requirements.txt
cat > requirements.txt << 'EOF'
gradio>=4.0
huggingface-hub>=0.18
numpy>=1.24
scipy>=1.11
EOF
```

5. **Rename Dockerfile:**
```bash
mv Dockerfile.spaces Dockerfile
```

6. **Push a HF Spaces:**
```bash
git add .
git commit -m "Chapel training pipeline with Gradio UI"
git push
```

7. **Acceder a Space:**
```
https://huggingface.co/spaces/Kimberly/nuclear-chapel-training
```

La app Gradio mostrará:
- ✅ Status de Chapel
- ✅ Botones para iniciar training
- ✅ Log en tiempo real
- ✅ Métricas dashboard
- ✅ Checkpoint management

---

### Opción C: HF Datasets (Almacenamiento)

```bash
# 1. Crear dataset privado en HF
huggingface-cli repo create \
  --repo-id="nuclear-training-data" \
  --type dataset \
  --private

# 2. Upload datos
python3 << 'PYTHON'
from huggingface_hub import HfApi

api = HfApi()

api.upload_folder(
    folder_path="ffi/chapel/data/train",
    repo_id="Kimberly/nuclear-training-data",
    repo_type="dataset",
    private=True
)
PYTHON

# Resultado:
# ✅ Datasets versionados en HF
# ✅ Accesible privadamente
# ✅ Control de versiones
```

---

## 📊 COMPARATIVA FINAL

| Característica | GitHub Actions | HF Spaces | HF Hub |
|---|---|---|---|
| Training | 6h máximo | 24/7 continuo | N/A |
| UI | Minimal | Gradio visual | Web |
| Privacidad | ✅ Privado | ✅ Privado | ✅ Privado |
| Storage | 90 días | Persistente | Persistente |
| Costo | FREE | FREE | FREE |
| Docker required | Si | Si | No |

---

## 🎯 RECOMENDACIÓN FINAL

**Usa AMBAS combinadas:**

```
┌─────────────────────────────────────────────┐
│  1. HuggingFace Spaces (Principal)           │
│     • Training 24/7                          │
│     • Gradio UI visual                       │
│     • Link: hf.co/spaces/Kimberly/...        │
│                                              │
│  2. GitHub Actions (Backup)                  │
│     • Trigger en push                        │
│     • Validación rápida (<6h)                │
│     • Reports en PRs                         │
│                                              │
│  3. HF Hub (Storage)                         │
│     • Datasets versionados                   │
│     • Checkpoints de modelos                 │
│     • Control acceso privado                 │
└─────────────────────────────────────────────┘
```

---

## 🚀 QUICK START (5 MINUTOS)

```bash
# Paso 1: Token HF
export HF_TOKEN="hf_xxxx"

# Paso 2: Push repo
cd /workspaces/nuclear-crawler-hybrid
./scripts/push_to_huggingface.sh $HF_TOKEN nuclear-chapel-training Kimberly true

# Paso 3: Crear Space
# Via web: https://huggingface.co/new-space

# Paso 4: Clonar + push
git clone https://huggingface.co/spaces/Kimberly/nuclear-chapel-training
cd nuclear-chapel-training
cp -r ../nuclear-crawler-hybrid/ffi/chapel/* .
git add . && git commit -m "Chapel training" && git push

# Paso 5: Acceder
# https://huggingface.co/spaces/Kimberly/nuclear-chapel-training
# Click "App" para ver Gradio UI
```

---

## ✅ VERIFICACIÓN

Después de setup, deberías tener:

- [ ] Repo privado en HF Hub: `Kimberly/nuclear-chapel-training`
- [ ] Space privado en HF: `Kimberly/nuclear-chapel-training`
- [ ] Dataset privado: `Kimberly/nuclear-training-data`
- [ ] GitHub Actions workflow activo
- [ ] Chapel code + datasets subidos
- [ ] Gradio UI funcionando

---

## 🔐 VARIABLES DE ENTORNO NECESARIAS

Para que todo funcione, necesitas:

```bash
# HuggingFace
export HF_TOKEN="hf_xxxxx"           # De https://huggingface.co/settings/tokens

# GitHub (opcional, ya tendrías)
export GITHUB_TOKEN="ghp_xxxxx"      # De https://github.com/settings/tokens
```

---

## 📞 SOPORTE

Si hay problemas:

1. **Error Docker Desktop**: No necesitas Docker en local (HF Spaces lo maneja)
2. **Token inválido**: Asegúrate de permisos en HF
3. **Space no corre**: Revisa logs en HF Spaces UI
4. **Chapel no compila**: HF Spaces instala Chapel automáticamente

---

**STATUS: 🟢 LISTO PARA DEPLOYAR**

¿Cuál es tu usuario de HuggingFace exactamente?
