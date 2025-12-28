# Plantilla de Tarea — Prompt (Invocación Manual)

Propósito
- Facilitar la creación de tareas repetitivas invocadas manualmente desde `.github/prompts/`.

Metadatos (encabezado YAML opcional)
```yaml
name: "nombre_corto_de_tarea"
invocation: "manual" # o "automatica"
priority: "Alta|Media|Baja"
owner: "equipo/usuario"
output_format: "json|markdown|plain"
```

Prompt body (estructurado)
- Contexto: Describe brevemente el repositorio y el objetivo de la tarea.
- Input esperado: Campos y formatos (ej.: `path`, `query`, `max_depth`).
- Constraints: Límites, exclusiones y requisitos de seguridad (e.g., no exponer secrets).
- Steps sugeridos: Pasos que el agente debe seguir.
- Output esperado: Estructura y ejemplos de la salida.

Ejemplo de Plantilla
```
# name: "analyze-module"
# invocation: manual
# priority: Media

Contexto:
Analizar el módulo especificado y devolver un resumen con problemas críticos, warnings, y sugerencias de mejora.

Input:
- path: "src/mymodule"
- max_files: 50

Constraints:
- No ejecutar comandos externos (usar APIs de filesystem)
- No exponer keys/credentials

Steps:
1. Leer archivos hasta max_files
2. Clasificar por extensión
3. Buscar patrones de error comunes (unwrap, expect, TODO comments)
4. Generar resumen con ejemplos

Output (JSON):
{
  "status": "success|error",
  "summary": "...",
  "issues": [ {"file":"...","line":...,"message":"..."} ],
  "suggestions": ["..."]
}
```

Notas
- Mantener plantillas en `.github/prompts/` y referenciarlas desde issues o PR templates.
- Estas plantillas son para invocación manual; las reglas automatizadas deben vivir en `.github/instructions/`.

---

Uso rápido (invocar manualmente desde CI/local):
- Copiar contenido al cuerpo de una Issue o usar una herramienta local que cargue el prompt y lo envíe al agente.
