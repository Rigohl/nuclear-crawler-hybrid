# HF Space + SSH Simple Connection
## Conectar Space con SSH al Modelo Chapel Existente

---

## 🎯 Plan Ultra-Simple

```
HF Space (VM con SSH) 
    ↓
    └─ SSH Key → HF Model (nuclear-chapel-training)
    └─ Git clone vía SSH
    └─ Ejecuta training
    └─ Push resultados
```

---

## 📋 PASO 1: Generar SSH Key

```powershell
# En tu local
ssh-keygen -t ed25519 -f hf-space-key -N ""
# Genera:
# - hf-space-key (privada)
# - hf-space-key.pub (pública)

# Mostrar la clave pública
type hf-space-key.pub
# Copiar todo (empieza con "ssh-ed25519")
```

---

## 📋 PASO 2: Agregar SSH Key a HF

```
1. Ir a: https://huggingface.co/settings/keys
2. Click "Add SSH key"
3. Pegar contenido de hf-space-key.pub
4. Guardar
```

---

## 📋 PASO 3: Crear Space Vacío

```bash
huggingface-cli repo create \
  --repo_id="nuclear-chapel-space" \
  --type="space" \
  --space_sdk="docker"
```

---

## 📋 PASO 4: Crear Dockerfile Simple

```dockerfile
FROM ubuntu:22.04

# Install essentials
RUN apt-get update && apt-get install -y \
    git openssh-client chapel \
    python3 pip

# Copy SSH key (será agregada en deployment)
RUN mkdir -p /root/.ssh
RUN chmod 700 /root/.ssh

# Clone Chapel model
RUN git clone git@huggingface.co:Kimberlyindiva/nuclear-chapel-training.git /chapel

WORKDIR /chapel

# Setup entrypoint
CMD ["bash", "-c", "while true; do sleep 3600; done"]
```

---

## 📋 PASO 5: Script Training Simple

```bash
#!/bin/bash
# train.sh

cd /chapel

# Pull latest
git pull origin main

# Compile Chapel files
chpl -o chapel_ai chapel_ai.chpl
chpl -o training_pipeline training_pipeline.chpl

# Run training
./training_pipeline --data training/data.csv

# Push results
git add .
git commit -m "Training results $(date)"
git push origin main
```

---

## 📋 PASO 6: Deployar Space con SSH

```powershell
# Clone Space
git clone https://huggingface.co/spaces/Kimberlyindiva/nuclear-chapel-space
cd nuclear-chapel-space

# Agregar Dockerfile
Copy-Item ./Dockerfile .

# Agregar script training
Copy-Item ./train.sh .
chmod +x train.sh

# Push
git add .
git commit -m "Setup Chapel Space with SSH"
git push origin main
```

---

## 🚀 USAR Space desde SSH Remoto

```bash
# Conectarse al Space (HF proporciona SSH)
ssh -i hf-space-key root@nuclear-chapel-space.huggingface.space

# Una vez dentro:
cd /chapel
./train.sh

# Ver logs
tail -f training.log
```

---

## ⚡ Versión Ultra-Simple (Solo Git SSH)

Si quieres **sin Dockerfile** (solo Space básico):

```powershell
# 1. Git config para SSH
git config --global user.name "tu-nombre"
git config --global user.email "tu-email@hf.co"

# 2. Desde Space (SSH):
eval "$(ssh-agent -s)"
ssh-add /ruta/a/hf-space-key

# 3. Clone y training
git clone git@huggingface.co:Kimberlyindiva/nuclear-chapel-training.git
cd nuclear-chapel-training
chpl -o chapel_ai chapel_ai.chpl
./chapel_ai --train
git push origin main
```

---

## 📊 Opciones Comparadas

| Método | Complejidad | Tiempo | SSH | Automatizado |
|--------|-------------|--------|-----|--------------|
| **Dockerfile + SSH** | Media | 5 min | ✅ | ✅ |
| **Space Simple SSH** | Baja | 2 min | ✅ | Manual |
| **Script Shell SSH** | Baja | 1 min | ✅ | ✅ |

**Recomendación**: Usa **Script Shell SSH** + Space simple

---

## ✅ Lista Rápida

- [ ] Generar SSH key (`ssh-keygen`)
- [ ] Agregar a HF Settings (keys.huggingface.co)
- [ ] Crear Space en HF
- [ ] Hacer git clone del Space
- [ ] Agregar Dockerfile mínimo
- [ ] Agregar train.sh
- [ ] Git push al Space
- [ ] Conectar vía SSH cuando necesites ejecutar training
- [ ] Los resultados se pushean automáticamente al modelo

---

## 🔑 Lo más simple posible

```powershell
# TODO EN UNA LÍNEA desde el Space:

ssh -i hf-space-key root@nuclear-chapel-space.huggingface.space \
  "cd /chapel && chpl chapel_ai.chpl && ./chapel_ai --train && git push"
```

---

¿Procedemos con **Script Shell SSH** o quieres **Dockerfile Simple**?

