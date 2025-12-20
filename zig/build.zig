const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});

    // Librería estática para Rust FFI - Zig 0.15.2 API
    const root_module = b.createModule(.{
        .root_source_file = b.path("src/lib.zig"),
        .target = target,
    });

    const lib = b.addLibrary(.{
        .name = "nuclear_zig",
        .root_module = root_module,
        .linkage = .static,
    });

    // Habilitar C ABI para FFI
    lib.linkLibC();

    // En Windows, Zig normalmente resolverá las librerías del CRT a través de
    // `linkLibC()`. Explicitar `msvcrt` a veces falla en ciertos toolchains,
    // así que confiamos en `linkLibC()` para mayor compatibilidad.

    b.installArtifact(lib);

    // Tests
    const test_module = b.createModule(.{
        .root_source_file = b.path("src/lib.zig"),
        .target = target,
    });

    const tests = b.addTest(.{
        .root_module = test_module,
    });

    const run_tests = b.addRunArtifact(tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_tests.step);
}
