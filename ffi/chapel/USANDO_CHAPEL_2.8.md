# 🔧 USANDO CHAPEL 2.8 DEL DISCO D

## 📍 Ubicación del Compilador

Tu compilador Chapel 2.8 debería estar en:
- `D:\BACK_CHAPEL_2.8`
- O en `D:\chapel-2.8`

---

## 🚀 Scripts Disponibles

### 1. Compilar Chapel AI (Auto-detecta Chapel 2.8)
```cmd
cd ffi\chapel
build_chapel_jni.bat
```

**Qué hace:**
- Busca automáticamente Chapel 2.8 en D:\BACK_CHAPEL_2.8
- Configura CHAPEL_HOME
- Compila nuclear_chapel_ai.chpl con C API
- Genera libchapel_ai.dll

---

### 2. Explorar Ejemplos de Chapel
```cmd
cd ffi\chapel
explore_chapel_examples.bat
```

**Qué hace:**
- Busca tu instalación de Chapel 2.8
- Muestra la estructura de directorios
- Lista ejemplos disponibles (.chpl)
- Abre explorador de Windows en el directorio
- Muestra información del compilador

---

### 3. Copiar Ejemplos Útiles al Proyecto
```cmd
cd ffi\chapel
copy_chapel_examples.bat
```

**Qué hace:**
- Busca ejemplos relacionados con:
  - Neural networks
  - Matrix operations
  - FFI/C interop
  - Parallel processing
  - Machine learning
- Copia los ejemplos a `chapel_examples/`
- Crea un índice README.txt
- Abre la carpeta con los ejemplos

---

## 📚 Estructura de Chapel 2.8

```
D:\BACK_CHAPEL_2.8\
├── bin\
│   └── chpl.exe          ← Compilador
├── examples\             ← Ejemplos oficiales
│   ├── primers\          ← Tutoriales básicos
│   ├── benchmarks\       ← Ejemplos de rendimiento
│   └── ...
├── test\                 ← Tests (muchos ejemplos aquí)
├── modules\              ← Módulos estándar
└── doc\                  ← Documentación
```

---

## 🔍 Ejemplos Útiles para Nuclear Chapel AI

### Para Neural Networks:
Busca en Chapel 2.8:
- `examples/*neural*`
- `examples/*network*`
- `test/library/packages/LinearAlgebra/*`
- `test/arrays/matrix*`

### Para FFI (C Integration):
- `examples/interop/*`
- `test/interop/*`
- `test/extern/*`

### Para Parallel Processing:
- `examples/primers/forallLoops.chpl`
- `examples/primers/distributions.chpl`
- `test/parallel/*`

### Para Linear Algebra:
- `test/library/packages/LinearAlgebra/*`
- `examples/benchmarks/lulesh/*`

---

## 💡 Cómo Usar los Ejemplos

### 1. Ver un ejemplo:
```cmd
notepad D:\BACK_CHAPEL_2.8\examples\primers\arrays.chpl
```

### 2. Compilar un ejemplo:
```cmd
cd ffi\chapel
D:\BACK_CHAPEL_2.8\bin\chpl.exe D:\BACK_CHAPEL_2.8\examples\primers\arrays.chpl
```

### 3. Ejecutar:
```cmd
.\arrays.exe
```

---

## 🔧 Configuración Manual (Si Auto-detección Falla)

### Windows:
```cmd
set CHAPEL_HOME=D:\BACK_CHAPEL_2.8
set PATH=%CHAPEL_HOME%\bin;%PATH%
chpl --version
```

### Permanente (Variables de Entorno):
1. Win + R → `sysdm.cpl`
2. Pestaña "Opciones avanzadas"
3. "Variables de entorno"
4. Agregar:
   - Variable: `CHAPEL_HOME`
   - Valor: `D:\BACK_CHAPEL_2.8`
5. Editar `PATH` y agregar: `%CHAPEL_HOME%\bin`

---

## 📖 Aprender de los Ejemplos

### Ejemplo 1: Arrays y Matrices
```chpl
// De Chapel 2.8 examples/primers/arrays.chpl
var A: [1..10] real;
A = 1.0;

var B: [1..5, 1..5] real;
forall (i,j) in {1..5, 1..5} do
  B[i,j] = i + j;
```

### Ejemplo 2: Parallel Loops
```chpl
// De Chapel 2.8 examples/primers/forallLoops.chpl
const n = 100;
var A: [1..n] real;

forall i in 1..n do
  A[i] = i * 2.0;
```

### Ejemplo 3: FFI con C
```chpl
// De Chapel 2.8 test/interop/
extern proc c_function(x: c_int): c_int;

var result = c_function(42);
```

---

## 🎯 Integrar Ejemplos en Nuclear Chapel AI

### Ejemplo: Agregar función de Chapel 2.8 a tu AI

**De ejemplo de Chapel:**
```chpl
// Encontrado en Chapel 2.8 examples/
proc matrixMultiply(A: [?D1] real, B: [?D2] real) {
  var C: [D1.dim(0), D2.dim(1)] real;
  forall (i,j) in {D1.dim(0), D2.dim(1)} do
    for k in D1.dim(1) do
      C[i,j] += A[i,k] * B[k,j];
  return C;
}
```

**Agregar a nuclear_chapel_ai.chpl:**
```chpl
// En ffi/chapel/ai/nuclear_chapel_ai.chpl
// Agregar después de imports:

// Matrix multiplication from Chapel 2.8 examples
proc matmulOptimized(A: [] real, B: [] real): [] real {
  // ... código del ejemplo adaptado ...
}

// Usar en NeuralLayer:
proc forwardPass(input: [] real): [] real {
  var z = matmulOptimized(weights, input) + biases;
  // ... resto del código ...
}
```

---

## 🔥 Workflow Completo

### 1. Explorar Ejemplos
```cmd
cd ffi\chapel
explore_chapel_examples.bat
```

### 2. Copiar Ejemplos Útiles
```cmd
copy_chapel_examples.bat
```

### 3. Revisar Ejemplos Copiados
```cmd
cd chapel_examples
dir *.chpl
notepad [ejemplo].chpl
```

### 4. Adaptar Código
- Abre `ai/nuclear_chapel_ai.chpl`
- Copia funciones útiles de los ejemplos
- Adapta tipos y nombres de variables

### 5. Compilar
```cmd
build_chapel_jni.bat
```

### 6. Probar
```cmd
cd ..\..
test_chapel_connection.bat
```

---

## 🐛 Troubleshooting

### "chpl.exe not found"
**Problema:** El script no encuentra Chapel 2.8

**Solución:**
1. Verifica que existe: `D:\BACK_CHAPEL_2.8\bin\chpl.exe`
2. O edita `build_chapel_jni.bat` línea 13:
   ```bat
   set SEARCH_PATHS=(^
       "D:\TU_RUTA\bin\chpl.exe" ^
       ...
   )
   ```

---

### "Module not found"
**Problema:** Un ejemplo de Chapel 2.8 usa módulos que no encuentra

**Solución:**
```cmd
set CHPL_MODULE_PATH=%CHAPEL_HOME%\modules
```

O en el código Chapel:
```chpl
use Path;  // En lugar de ruta absoluta
```

---

### "Incompatible Chapel version"
**Problema:** Código de ejemplo no compila

**Solución:**
- Chapel 2.8 debería ser compatible
- Revisa sintaxis (puede haber cambiado entre versiones)
- Consulta documentación: `D:\BACK_CHAPEL_2.8\doc\`

---

## 📞 Recursos

### En tu instalación:
- Documentación: `D:\BACK_CHAPEL_2.8\doc\`
- Ejemplos: `D:\BACK_CHAPEL_2.8\examples\`
- Tests (más ejemplos): `D:\BACK_CHAPEL_2.8\test\`

### Online:
- Chapel 2.8 Docs: https://chapel-lang.org/docs/2.8/
- Chapel Language Spec: https://chapel-lang.org/spec/
- Chapel Examples: https://github.com/chapel-lang/chapel/tree/main/test

---

## ✅ Checklist

- [ ] Ejecutar `explore_chapel_examples.bat`
- [ ] Ver estructura de Chapel 2.8
- [ ] Ejecutar `copy_chapel_examples.bat`
- [ ] Revisar ejemplos copiados en `chapel_examples/`
- [ ] Identificar código útil para tu AI
- [ ] Adaptar ejemplos a `nuclear_chapel_ai.chpl`
- [ ] Compilar con `build_chapel_jni.bat`
- [ ] Probar con `test_chapel_connection.bat`

---

**Creado:** 9 de febrero de 2026  
**Para:** Usuario con Chapel 2.8 en D:\BACK_CHAPEL_2.8  
**Propósito:** Aprovechar ejemplos de Chapel para mejorar Nuclear Chapel AI
