//
// Archivo de ejemplo Chapel para demostrar el análisis ULTRA-AVANZADO
// Este archivo contiene varios patrones que deberían ser detectados
//

// Dominio sin distribución (debería ser warning)
var D: domain(2) = {1..100, 1..100};
var A: [D] real;

// Array sin dominio explícito (debería ser info)
var B: [1..100] real;

// Task spawning sin sincronización (debería ser warning)
begin {
    var local_sum = 0.0;
    for i in 1..100 do
        local_sum += A[i, i];
}

// Reducción ineficiente (debería ser performance issue)
var total = 0.0;
forall i in 1..100 do
    total += A[i, 1];  // debería usar reduce

// Acceso no-local en bucle paralelo (debería ser performance issue)
forall i in 1..100 do {
    // Acceso potencialmente no-local
    A[i, 1] = B[i] + 1.0;
}

// Uso de while con paralelismo (anti-pattern)
var done = false;
while !done {
    forall i in 1..10 do {
        if A[i, 1] > 100.0 {
            done = true;
        }
    }
}

// Copia innecesaria antes de bucle paralelo
var C = A;  // copia costosa
forall i in 1..100 do {
    C[i, 1] = C[i, 1] * 2.0;
}

// Variable global (debería ser info)
var global_counter = 0;

// coforall sin límites
coforall i in 1..1000 do {
    // potencialmente demasiados tasks
    A[i, 1] = i * 1.0;
}

// Función con parámetros para demostrar buen uso
proc compute_local(ref local_array: [] real, start: int, end: int) {
    forall i in start..end do {
        local_array[i] = local_array[i] * local_array[i];
    }
}

// Uso correcto con distribución
var D_block: domain(2) dmapped Block(boundingBox={1..100, 1..100});
var A_distributed: [D_block] real;

// Operación promovida (buena práctica)
A_distributed = A_distributed + 1.0;

// Reduce correcto
var sum_all = + reduce A_distributed;

// Zippered iteration sugerida
forall (i, j) in zip(1..10, 1..10) do {
    A_distributed[i, j] = i + j;
}

writeln("Ejemplo completado");