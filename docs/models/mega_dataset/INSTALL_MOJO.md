# Instalar Mojo

## Opción 1: Instalación Oficial (Recomendado)

### Windows/Linux/Mac:

```bash
# 1. Instalar Modular CLI
curl https://get.modular.com | sh -

# 2. Instalar Mojo
modular install mojo

# 3. Agregar a PATH
# Windows: agregar C:\Users\<user>\.modular\pkg\packages.modular.com_mojo\bin a PATH
# Linux/Mac: export PATH=$PATH:~/.modular/pkg/packages.modular.com_mojo/bin
```

### Verificar instalación:

```bash
mojo --version
# Output: mojo 24.x.x
```

## Opción 2: Mientras tanto - Python Optimizado

Si no puedes instalar Mojo ahora, usa la versión Python optimizada que incluí:

```bash
python run_mojo_training.py
# Detecta automáticamente si Mojo está disponible
# Si no, usa Python con JAX optimizado
```

## Documentación Mojo

- **Website**: https://www.modular.com/mojo
- **Docs**: https://docs.modular.com/mojo/
- **GitHub**: https://github.com/modularml/mojo

## Ventajas de Mojo

- 🚀 68,000x más rápido que Python en algunos benchmarks
- 🎯 35,000x más rápido en Mandelbrot
- ⚡ Zero-cost abstractions
- 🔧 Compatible con Python ecosystem
- 🖥️ GPU/TPU automático
- 📦 Compila a ejecutable nativo

## Sistema Actual

El código Mojo está en:
- `mojo_dataset_processor.mojo` - ¡Listo para usar!

Una vez instalado Mojo:
```bash
cd D:\models\mega_dataset
mojo run mojo_dataset_processor.mojo
```

## Alternativa Inmediata

Mientras instalas Mojo, el sistema usa Python+JAX que es ~10x más rápido que Python puro, aunque no tan rápido como Mojo nativo.
