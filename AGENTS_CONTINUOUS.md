# Agentes Autónomos del Nuclear Crawler Hybrid

## Tipos de Agentes

### 1. **DataCollectionAgent** (Recolección de Datos)
```
Responsabilidad: Scrapy inteligente con aprendizaje
- Monitorea 5000+ sitios de alto valor
- Adapta técnicas de stealth basado en patrones históricos
- Mantiene dataset actualizado en tiempo real
- Valida calidad antes de almacenar
- Reporta anomalías al ContinuousLearningAgent
```

### 2. **ContinuousLearningAgent** (Aprendizaje Continuo)
```
Responsabilidad: Entrena modelos sin parar
- Lee scraping_stealth_patterns.json cada hora
- Entrena Chapel AI neural network
- Valida rendimiento contra benchmarks
- Ajusta hiperparámetros automáticamente
- Emite recomendaciones a DataCollectionAgent
Ciclo: DATOS → ENTRENAMIENTO → VALIDACIÓN → OPTIMIZACIÓN → ACCIÓN
```

### 3. **ValidationAgent** (Validación de Calidad)
```
Responsabilidad: Garantiza calidad de datos
- Aplica layer_2 quality_assessment rules
- Detecta duplicados con simhash
- Valida schema y contenido
- Marca datos defectuosos para recolección
- Mantiene métricas de calidad
Umbral mínimo: 0.90 (90% quality score)
```

### 4. **OptimizationAgent** (Optimización)
```
Responsabilidad: Mejora rendimiento sistemático
- Monitorea latencia de scraping
- Detecta botellas de cuello
- Paralleliza con Chapel coforall
- Optimiza distribución de carga (BlockDist)
- Reporta mejoras en CI/CD
```

### 5. **AdaptationAgent** (Adaptación)
```
Responsabilidad: Aprende nuevas técnicas de evasión
- Detecta cambios en sistemas anti-bot
- Prueba nuevas user-agents y headers
- Experimenta con timing patterns
- Evalúa éxito de nuevas técnicas
- Integra exitosas al sistema
```

## Flujo de Orquestación

```
┌─────────────────────────────────────────────────────────────┐
│           Orchestration: CONTINUOUS IMPROVEMENT             │
└─────────────────────────────────────────────────────────────┘

HORA 1:
  1. DataCollectionAgent recolecta 10K registros
  2. ValidationAgent valida (filtra 5% defectuosos)
  3. ContinuousLearningAgent entrena con 9.5K registros

HORA 2:
  4. ValidationAgent detecta patrón: Cloudflare blocks ↑ 15%
  5. AdaptationAgent experimenta 10 nuevas técnicas
  6. 3 técnicas alcanzan 85%+ success rate
  7. Integra técnicas en rutas de scraping

HORA 3:
  8. OptimizationAgent: Latencia promedio ↓ 23%
  9. DataCollectionAgent incrementa velocidad 1.5x
  10. ContinuousLearningAgent re-entrena con nuevos datos

CONTINUO:
  - Chapel AI aprende en background (multi-locale)
  - Reporta a MCP tools cada 30 min
  - Exporta modelo mejorado cada 4 horas
  - Mantiene rolling history de 90 días
```

## Estados y Transiciones

```
[Idle] ──→ [Collecting] ──→ [Processing] ──→ [Training]
          ↑                                      ↓
          └───────────────── [Optimizing] ←─────┘
                                  ↓
                            [Reporting]
                                  ↓
                             [Adapting]
                                  ↓
                            [Back to Collecting]
```

## Communication Protocol (MCP JSON-RPC)

```json
{
  "agent_id": "continuous_learning_1",
  "action": "train_models",
  "timestamp": "2026-01-23T14:30:00Z",
  "payload": {
    "dataset": "scraping_stealth_patterns",
    "layers": [1, 2],
    "samples": 50000,
    "validation_split": 0.15,
    "epochs": 100
  },
  "expected_output": {
    "model_checkpoint": "neural_network_v2.chapel",
    "metrics": {
      "accuracy": ">0.90",
      "quality_score": ">0.88"
    }
  }
}
```

## Deployment

### Local Development
```bash
# Inicia agents en paralelo
./scripts/start_agents.sh \
  --data-agent \
  --learning-agent \
  --validation-agent \
  --optimization-agent \
  --adaptation-agent
```

### Docker Compose
```yaml
services:
  data-agent:
    image: nuclear-mcp:agents
    environment:
      AGENT_TYPE: collection
      DATASET_PATH: /data/scraping_stealth_patterns.json
    
  learning-agent:
    image: nuclear-mcp:agents
    environment:
      AGENT_TYPE: continuous_learning
      CHAPEL_ENABLED: true
```

### Kubernetes (Production)
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nuclear-agents
spec:
  replicas: 3
  selector:
    matchLabels:
      role: agent
  template:
    spec:
      containers:
      - name: continuous-learning
        image: nuclear-mcp:agents:latest
        resources:
          limits:
            memory: "16Gi"
            cpu: "8000m"
```

## Monitoreo

```
Métricas Clave:
  - Datos recolectados/hora: ↑ (meta: 50K)
  - Calidad promedio: ↑ (meta: >0.92)
  - Latencia scraping: ↓ (meta: <2s)
  - Tasa de éxito evasión: ↑ (meta: >0.85)
  - Modelo accuracy: ↑ (meta: >0.91)

Dashboards:
  - Real-time metrics: Prometheus + Grafana
  - Model performance: TensorBoard
  - Agent logs: ELK Stack
  - Alertas: PagerDuty
```

## Error Handling

```
Escenario: Validación falla para >30% de datos
  1. ValidationAgent alerta
  2. DataCollectionAgent pausa recolección
  3. AdaptationAgent diagnostica causa
  4. Propone nuevas técnicas
  5. ContinuousLearningAgent re-entrena
  6. Resume operaciones

Escenario: Accuracy cae <0.85
  1. ContinuousLearningAgent detecta
  2. OptimizationAgent revisa hiperparámetros
  3. AdaptationAgent busca cambios en anti-bot
  4. Re-entrena con nuevos datos
  5. Si persiste > 2 horas → escalada manual
```

## Integration con Chapel AI

```chapel
// En chapel_ai.chpl
procedure continuous_learning_hook(patterns: [?D] PatternData, layer: int) {
  // Los agentes envían datos a Chapel
  // Chapel entrena con paralelismo multi-locale
  // Retorna modelos optimizados
  
  coforall locale in Locales {
    on locale {
      parallel_train_neural_net(patterns, learning_rate, epochs);
      update_adam_optimizer();
      validate_quality_metrics();
    }
  }
}
```

## Success Metrics

| Métrica | Baseline | Target | Timeline |
|---------|----------|--------|----------|
| Data Quality | 0.78 | 0.95 | Week 1 |
| Scraping Success | 0.82 | 0.93 | Week 2 |
| Model Accuracy | 0.81 | 0.92 | Week 3 |
| System Latency | 3.2s | 1.5s | Week 2 |
| Adaptation Speed | 12h | 2h | Week 4 |

