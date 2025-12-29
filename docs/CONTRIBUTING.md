# Directriz Focalizada — Plantilla

Propósito
- Definir reglas específicas y focalizadas que se aplican sólo a archivos que coincidan con un patrón `glob` dentro del repositorio.

Metadatos obligatorios
- Nombre: Breve identificador de la directriz
- Scope (glob): Ej.: `.github/instructions/*.md` o `src/**/*.sql`
- Tipo: `Archivos Específicos (Glob)` | `Módulo` | `Lenguaje`
- Severidad: `Alta` | `Media` | `Baja`
- Responsable: Equipo o persona a cargo
- Cadena de verificación (checklist): Lista de criterios que deben cumplirse

Contexto/Media (opcional)
- Añadir contexto adicional, enlaces y ejemplos (archivos de referencia, enlaces a PRs, snippets) que ayuden a entender la regla.

Reglas (contenido)
- Describir reglas concretas por ítem. Soporta secciones por lenguaje o módulo: por ejemplo:
  - SQL: evitar concatenación de cadenas en queries; siempre usar parámetros preparados.
  - Rust: no usar `.unwrap()`; usar `Result<T, E>` y `?`.
  - CI: no almacenar secretos en YAML; usar OIDC/Secrets.

Formato de la regla
- id: unique-key
- description: Explicación corta
- applies_to: glob o lista de paths
- implementation: pasos para aplicar o verificar (manual/automática)
- severity: Alta/Media/Baja
- example_good: snippet correcto
- example_bad: snippet incorrecto

Ejemplo mínimo
```
Name: "SQL-Parameterized-Queries"
Scope: "src/**/*.sql || src/**/*.rs"
Severity: "Alta"
Responsible: "db-team"
Rules:
  - id: "sql-params-001"
    description: "Usar queries parametrizados en todas las consultas dinámicas"
    applies_to: "src/**/*.rs"
    implementation: "Revisar uso de `rusqlite::params!` o `query!` macros; rechazar concatenaciones de strings con variables"
    severity: "Alta"
    example_good: "conn.execute(\"INSERT INTO t VALUES (?,?)\", params![a,b])"
    example_bad: "let q = format!(\"INSERT INTO t VALUES ({},{})\", a, b); conn.execute(&q, [])"
```

Enforcement / Automación
- Indicar si la regla debe ser verificada por herramientas automáticas (linters, CI), por revisión manual, o ambas.
- Proveer comandos o scripts de verificación cuando sea posible.

Historial y cambios
- Mantener historial de cambios con fecha, autor y motivo.

Uso
- Colocar archivos de este tipo en `.github/instructions/` y referenciarlos desde `REGLAS.instructions.md` o documentación de proyecto.

---

Esta plantilla está diseñada para ser legible por humanos y suficiente para crear checks automáticos en CI o en revisiones de PR.
