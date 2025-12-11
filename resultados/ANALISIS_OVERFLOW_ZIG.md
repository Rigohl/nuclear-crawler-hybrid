# ============================================================================
# 🔬 ANÁLISIS MATEMÁTICO AVANZADO - NUCLEAR CRAWLER HYBRID
# Investigación: Integer Overflow y Stack Overflow en Zig FFI
# Fecha: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")
# ============================================================================

## PROBLEMA DETECTADO
- Error: "integer overflow" en thread Zig
- Error: "thread has overflowed its stack" 
- Exit code: 3221225725 (0xC00000FD = STATUS_STACK_OVERFLOW)

## ANÁLISIS MATEMÁTICO

### 1. Integer Overflow Analysis
El código Zig tiene operaciones que pueden causar overflow:
- `hash = ((hash << 5) + hash) + byte`  
- Cuando hash es grande, (hash << 5) puede overflow u64

### 2. Stack Overflow Analysis  
- Windows default stack: 1MB por thread
- Zig spawn threads con stack default: 8MB
- El HTML grande de Raycast (~500KB-2MB) causa recursión profunda
- Threads array [16] en stack = 16 * 8 bytes = 128 bytes OK
- PERO: slices y data copies van al stack temporalmente

### 3. SOLUCIÓN MATEMÁTICA

#### A. Limitar datos ANTES de FFI (ya implementado: 10MB)
#### B. Usar operaciones seguras en Zig (@addWithOverflow, @mulWithOverflow)
#### C. Reducir stack usage moviendo data al heap

## RECOMENDACIÓN FINAL
Deshabilitar temporalmente FFI Zig para HTML grande
Usar Rust puro con Rayon (más seguro, casi igual de rápido)
