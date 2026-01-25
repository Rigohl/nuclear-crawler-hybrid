# Solo SSH - Setup Más Simple Posible

---

## 🎯 3 Pasos = Listo

### PASO 1: Generar SSH Key

```powershell
ssh-keygen -t ed25519 -f hf-key -N ""

# Muestra la pública
type hf-key.pub
```

Copiar todo (empieza con `ssh-ed25519`).

---

### PASO 2: Agregar a HF

1. Ir a: `https://huggingface.co/settings/keys`
2. Click "Add SSH key"
3. Pegar contenido
4. Guardar

---

### PASO 3: Conectar y Entrenar

```bash
# Clone vía SSH
git clone git@huggingface.co:Kimberlyindiva/nuclear-chapel-training.git
cd nuclear-chapel-training

# Usar SSH key
export GIT_SSH_COMMAND="ssh -i /ruta/a/hf-key"

# Compilar
chpl -o chapel_ai chapel_ai.chpl
chpl -o training_pipeline training_pipeline.chpl

# Entrenar
./training_pipeline --data training/data.csv

# Push resultados
git add .
git commit -m "Training results"
git push origin main
```

---

## ✅ Listo (eso es todo)

Cada vez que entrenes, repites el PASO 3.

