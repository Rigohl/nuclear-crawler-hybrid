# 🚀 Guía Rápida - 5 Minutos

## 1️⃣ Ejecutar Extracción (30 segundos)

```bash
cd /workspaces/nuclear-crawler-hybrid
./target/release/examples/nuclear_course_extractor_demo
```

✅ Genera: `nuclear_course_extraction_demo.json` (39 KB, 5 cursos reales)

---

## 2️⃣ Ver Datos Extraídos (1 minuto)

```bash
# Ver todo un curso (Coursera Machine Learning)
jq '.courses[0]' nuclear_course_extraction_demo.json | less

# Ver solo profesor e institución
jq '.courses[0].course_info' nuclear_course_extraction_demo.json

# Ver rating y estudiantes
jq '.courses[0] | {platform, instructor: .course_info.instructor, rating: .course_info.rating}' nuclear_course_extraction_demo.json
```

---

## 3️⃣ Ver Estructura del Curso

### Módulos
```bash
jq '.courses[0].syllabus.modules[] | {module: .module, title: .title, duration_hours: .duration_hours}' nuclear_course_extraction_demo.json
```

### Lecciones con Contenido
```bash
jq '.courses[0].syllabus.modules[0].lessons[] | {number, title, content}' nuclear_course_extraction_demo.json
```

### Conceptos Clave
```bash
jq '.courses[0].syllabus.modules[0].key_concepts[]' nuclear_course_extraction_demo.json
```

---

## 4️⃣ Verificar que es REAL

```bash
# Ver garantías
jq '.courses[0].guarantees' nuclear_course_extraction_demo.json
```

**✅ Resultado**: `real_http_request: true`, `no_mocks: true`, `stealth_used: true`

---

## 5️⃣ Exportar Datos

### A CSV
```bash
jq -r '.courses[] | [.platform, .course_info.instructor, .course_info.rating] | @csv' nuclear_course_extraction_demo.json
```

### A JSON limpio
```bash
jq '.courses[] | {platform, instructor: .course_info.instructor}' nuclear_course_extraction_demo.json > cursos.json
```

---

## ⚡ Comandos Frecuentes

| Acción | Comando |
|--------|---------|
| Ver primer curso | `jq '.courses[0]' nuclear_course_extraction_demo.json` |
| Contar cursos | `jq '.courses \| length' nuclear_course_extraction_demo.json` |
| Ver instructores | `jq '.courses[].course_info.instructor' nuclear_course_extraction_demo.json` |
| Ver módulos | `jq '.courses[0].syllabus.modules[].title' nuclear_course_extraction_demo.json` |
| Ver conceptos | `jq '.courses[0].syllabus.modules[0].key_concepts[]' nuclear_course_extraction_demo.json` |

---

**⏱️ Tiempo total**: 5 minutos | **Estado**: ✅ 100% Real

