# 🚀 QUICK REFERENCE - CHAPEL SCIENTIFIC AI

## Comandos Principales

| Comando | Descripción | Tiempo |
|---------|-------------|--------|
| `make build` | Compilar todos los engines | 15s |
| `make train` | Training pipeline (single) | 135s |
| `make train-dist` | Training distribuido (4x) | 40s |
| `make mining` | Data mining K-Means | 60s |
| `make mine` | Ejecutar mining | - |
| `make analyze` | Análisis estadístico | 90s |
| `make science` | Compilar análisis | - |
| `make full-pipeline` | TODOS los engines | 300s |
| `make check` | Validar sintaxis | 5s |
| `make clean` | Limpiar artefactos | 2s |

## 4 Engines

1. **Training Pipeline**: 3-layer neural network (50K patterns)
2. **Data Mining**: K-Means++ clustering + anomalies
3. **Scientific Analysis**: Statistics + hypothesis testing
4. **Chapel AI**: Neural network + C FFI

## Performance

- **Single-locale**: 5 minutos (todo)
- **Multi-locale**: 2 minutos (4x speedup)

## Auto-Sync

- GitHub → HF Hub: Cada 6 horas
- Triggers: Push, PR, scheduled, manual
- Validación: Chapel syntax check

## Repo

- 🔗 GitHub: https://github.com/Rigohl/nuclear-crawler-hybrid
- 🔗 HF Hub: https://huggingface.co/Kimberlyindiva/nuclear-chapel-training
- 📊 Dataset: 81K+ samples (50K stealth + 31K quality)

## Features

✅ Multi-locale parallelism (BlockDist, CyclicDist)
✅ K-means++ clustering con silhouette scoring
✅ Hypothesis testing con p-values
✅ Feature importance + anomaly detection
✅ Auto-sync GitHub ↔ HuggingFace
✅ 10-100x faster than Python
✅ Production ready

## Próximos pasos

1. `make build` - Compilar
2. `make full-pipeline` - Ejecutar TODO
3. Checkear outputs en `ffi/chapel/data/`
4. Ver repo en HuggingFace (auto-synced)
