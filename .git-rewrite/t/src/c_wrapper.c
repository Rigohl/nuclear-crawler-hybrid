// Wrapper C para proporcionar símbolos faltantes para Go y Zig
// Estos símbolos son necesarios cuando Go/Zig se compilan con diferentes toolchains

#ifdef _WIN32
#include <stdint.h>
#include <stdlib.h>

// Stack protector symbols que Zig necesita
// Estas funciones deben estar exportadas para que el linker las encuentre
#ifdef _MSC_VER
// Para MSVC, proporcionar implementaciones básicas
__declspec(dllexport) uintptr_t __stack_chk_guard = 0xDEADBEEF;

__declspec(dllexport) void __stack_chk_fail(void) {
    // Stack check failed - abortar
    abort();
}

__declspec(dllexport) void __chkstk_ms(void) {
    // Stack check para MSVC - función vacía, el compilador la maneja
    return;
}

// Alias para ___chkstk_ms (con 3 guiones bajos) - SQLite lo necesita
__declspec(dllexport) void ___chkstk_ms(void) {
    // Función vacía - el compilador maneja el stack check
    return;
}

// __isnan - SQLite necesita esta función
// MSVC tiene _isnan, pero SQLite compilado con MinGW busca __isnan
__declspec(dllexport) int __isnan(double x) {
    // Comparación manual para NaN (NaN != NaN)
    return (x != x) ? 1 : 0;
}

#else
// Para GCC/MinGW
uintptr_t __stack_chk_guard = 0xDEADBEEF;

void __stack_chk_fail(void) {
    abort();
}

void __chkstk_ms(void) {
    return;
}

void ___chkstk_ms(void) {
    __chkstk_ms();
}
#endif

#endif // _WIN32

