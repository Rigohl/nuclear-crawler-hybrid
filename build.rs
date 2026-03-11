//! Build script para Nuclear Crawler Hybrid
//!
//! ═══════════════════════════════════════════════════════════════════════════
//! FFI REAL - MAXIMUM LANGUAGE FEATURES (FAIL-FAST ENFORCEMENT)
//! ═══════════════════════════════════════════════════════════════════════════
//!
//! Primary Engine: Chapel AI with GPU + Multi-locale + BLAS/LAPACK
//! Secondary: JAX (GPU/TPU), Julia (BLAS), Mojo (SIMD)
//! WASM Portable: Go (goroutines), Zig (SIMD), Nim (parsing)
//! Windows-only: Go/Zig/Nim native FFI via libloading
//!
//! Philosophy: Extract maximum power from each language
//! - Chapel: BlockDist, CyclicDist, GPU kernels, multi-locale parallelism
//! - JAX: XLA compilation, GPU/TPU acceleration
//! - Julia: Native BLAS, multi-threading, distributed arrays
//! - Mojo: SIMD, compile-time execution, 66x faster
//! - Go WASM: Goroutines via TinyGo -> wasmtime (ffi/wasm/go/main.go)
//! - Zig WASM: SIMD via native Zig -> wasmtime (ffi/wasm/zig/main.zig)
//! - Nim WASM: Parsing via Emscripten -> wasmtime (ffi/wasm/nim/main.nim)
//!
//! Build Chapel: cd ffi/chapel && ./build_chapel_real.sh
//! Build WASM:   cd ffi/wasm && ./build_wasm.sh
//! With GPU: GPU_ARCH=sm_86 ./build_chapel_real.sh
//! Distributed: NUM_LOCALES=4 ./build_chapel_real.sh
//!
//! NOTA: No usa cargo:warning= para evitar mensajes "warning:" en la salida
//! ═══════════════════════════════════════════════════════════════════════════

fn main() {
    // Declarar cfgs personalizados para evitar warnings
    println!("cargo::rustc-check-cfg=cfg(has_go)");
    println!("cargo::rustc-check-cfg=cfg(has_zig)");
    println!("cargo::rustc-check-cfg=cfg(has_nim)");
    println!("cargo::rustc-check-cfg=cfg(has_chapel)");
    
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // ════════════════════════════════════════════════════════════════════════
    // VERSIÓN FFI TRACKING - DETECTAR Y VALIDAR VERSIONES
    // ════════════════════════════════════════════════════════════════════════
    
    println!("cargo:rustc-env=NFI_VALIDATION=strict");
    println!("cargo:rustc-env=FFI_GO_REQUIRED_VERSION=1.21.0");
    println!("cargo:rustc-env=FFI_ZIG_REQUIRED_VERSION=0.15.2");
    println!("cargo:rustc-env=FFI_CHAPEL_REQUIRED_VERSION=2.0.0");

    // ════════════════════════════════════════════════════════════════════════
    // FFI REAL - MAXIMUM LANGUAGE FEATURES
    // ════════════════════════════════════════════════════════════════════════
    //
    // PRIMARY ENGINE (Cross-platform):
    //   - Chapel AI: GPU + Multi-locale + BLAS/LAPACK
    //   - JAX: XLA + GPU/TPU
    //   - Julia: Native BLAS + Distributed
    //
    // WINDOWS-ONLY ENGINES:
    //   - Go: Goroutines + CGO/MSVC (parallel HTTP)
    //   - Zig: SIMD intrinsics (hashing)
    //   - Nim: Macros + C++ interop (HTML parsing)
    //
    // Philosophy: fail fast when required real backends are missing
    // ════════════════════════════════════════════════════════════════════════

    // Solo intentar linkear FFI en Windows (no en Linux)
    if cfg!(target_os = "windows") {
        let go_lib_candidates = [
            format!("{}/ffi/go/stealth_go_msvc.lib", manifest_dir),
            format!("{}/ffi/go/stealth_go.lib", manifest_dir),
            format!("{}/ffi/shared/stealth_go.lib", manifest_dir),
        ];

        let zig_lib_candidates = [
            format!("{}/ffi/zig/nuclear_zig.lib", manifest_dir),
            format!("{}/ffi/shared/nuclear_zig.lib", manifest_dir),
            format!("{}/ffi/zig/zig-out/lib/nuclear_zig.lib", manifest_dir),
        ];

        let nim_lib_candidates = [
            format!("{}/ffi/nim/nuclear_nim.lib", manifest_dir),
            format!("{}/ffi/shared/nuclear_nim.lib", manifest_dir),
        ];

        // Linkear Go FFI (solo Windows)
        let mut go_found = None;
        for cand in go_lib_candidates.iter() {
            if std::path::Path::new(cand).exists() {
                go_found = Some(cand.clone());
                break;
            }
        }

        if let Some(go_path) = go_found {
            if let Some(dir) = std::path::Path::new(&go_path).parent() {
                println!("cargo:rustc-link-search=native={}", dir.display());
            }
            let lib_name = if go_path.contains("stealth_go_msvc") {
                "stealth_go_msvc"
            } else {
                "stealth_go"
            };
            println!("cargo:rustc-link-lib=static={}", lib_name);
            println!("cargo:rustc-cfg=has_go");

            // Go runtime dependencies - MSVC CRT + Windows libs
            println!("cargo:rustc-link-lib=dylib=msvcrt");
            println!("cargo:rustc-link-lib=dylib=legacy_stdio_definitions");
            println!("cargo:rustc-link-lib=dylib=ws2_32");
            println!("cargo:rustc-link-lib=dylib=winmm");
            println!("cargo:rustc-link-lib=dylib=ntdll");
            eprintln!("Go FFI: ENABLED (found {})", go_path);
        }

        // Intenta detectar y linkear Zig static lib (SOLO WINDOWS)
        let mut zig_found = None;
        for cand in zig_lib_candidates.iter() {
            if std::path::Path::new(cand).exists() {
                zig_found = Some(cand.clone());
                break;
            }
        }

        // If not found, try to build with `zig build` if Zig is installed
        if zig_found.is_none() {
            let zig_dir = format!("{}/ffi/zig", manifest_dir);
            if std::path::Path::new(&zig_dir).exists() {
                if let Ok(output) = std::process::Command::new("zig").arg("version").output() {
                    if output.status.success() {
                        let _ = std::process::Command::new("zig")
                            .arg("build")
                            .current_dir(&zig_dir)
                            .output();
                        for cand in zig_lib_candidates.iter() {
                            if std::path::Path::new(cand).exists() {
                                zig_found = Some(cand.clone());
                                break;
                            }
                        }
                    }
                }
            }
        }

        if let Some(zig_path) = zig_found {
            if let Some(dir) = std::path::Path::new(&zig_path).parent() {
                println!("cargo:rustc-link-search=native={}", dir.display());
            }
            println!("cargo:rustc-link-lib=static=nuclear_zig");
            println!("cargo:rustc-cfg=has_zig");
            eprintln!("Zig FFI: ENABLED (found {})", zig_path);
        }

        // Nim static lib (SOLO WINDOWS)
        let mut nim_found = None;
        for cand in nim_lib_candidates.iter() {
            if std::path::Path::new(cand).exists() {
                nim_found = Some(cand.clone());
                break;
            }
        }

        if let Some(nim_path) = nim_found {
            if let Some(dir) = std::path::Path::new(&nim_path).parent() {
                println!("cargo:rustc-link-search=native={}", dir.display());
            }
            println!("cargo:rustc-link-lib=static=nuclear_nim");
            println!("cargo:rustc-cfg=has_nim");
            eprintln!("Nim FFI: ENABLED (found {})", nim_path);
        }
    }

    // ════════════════════════════════════════════════════════════════════════
    // CHAPEL AI FFI - PRIMARY ENGINE (Cross-platform)
    // GPU + Multi-locale + BLAS/LAPACK + Advanced Parallelism
    // ════════════════════════════════════════════════════════════════════════

    // First, try system Chapel installation
    let chapel_home = std::env::var("CHPL_HOME").ok();
    let has_system_chapel = if let Some(home) = &chapel_home {
        let lib_path = format!("{}/lib/libchapel.a", home);
        std::path::Path::new(&lib_path).exists()
    } else {
        false
    };

    // Second, check local compiled version
    let chapel_lib_path = format!("{}/ffi/chapel/libchapel_ai.so", manifest_dir);
    let has_local_chapel = std::path::Path::new(&chapel_lib_path).exists();

    if has_system_chapel {
        if let Some(home) = chapel_home {
            println!("cargo:rustc-link-search=native={}/lib", home);
            println!("cargo:rustc-link-lib=static=chapel");
            println!("cargo:rustc-cfg=has_chapel");
            eprintln!(
                "🧠 Chapel AI FFI: ENABLED (System Chapel found at {})",
                home
            );
        }
    } else if has_local_chapel {
        let chapel_lib_dir = format!("{}/ffi/chapel", manifest_dir);
        println!("cargo:rustc-link-search=native={}", chapel_lib_dir);
        println!("cargo:rustc-link-lib=dylib=chapel_ai");
        // Add rpath using absolute path so binary can find libchapel_ai.so
        // when run from any directory
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}/ffi/chapel",
            manifest_dir
        );
        println!("cargo:rustc-cfg=has_chapel");
        eprintln!("🧠 Chapel AI FFI: ENABLED (Local libchapel_ai.so found)");
        eprintln!("   ✅ Binary will use rpath: {}/ffi/chapel", manifest_dir);
    } else {
        panic!(
            "Chapel AI FFI is required on this repository. Missing system Chapel or ffi/chapel/libchapel_ai.so. Build it with `cd ffi/chapel && ./build_chapel_real.sh` or install Chapel and set CHPL_HOME."
        );
    }

    if !cfg!(target_os = "windows") {
        // En Linux/Unix: Chapel + JAX + Julia (no Go/Zig/Nim)
        eprintln!("📝 build.rs: Non-Windows platform ({})", target_os);
        eprintln!("   → FFI Go/Zig/Nim: Windows only");
        eprintln!("   → Chapel/JAX/Julia: Cross-platform (Linux + Windows)");
        eprintln!("   → No implicit substitution is advertised for missing Windows-only static libraries");
    }

    // Rerun si cambian las librerías FFI
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=ffi/go/stealth_go_msvc.lib");
    println!("cargo:rerun-if-changed=ffi/zig/nuclear_zig.lib");
    println!("cargo:rerun-if-changed=ffi/nim/nuclear_nim.lib");
    println!("cargo:rerun-if-changed=ffi/chapel/libchapel_ai.so");
    println!("cargo:rerun-if-changed=ffi/chapel/ai/nuclear_chapel_ai.chpl");
    println!("cargo:rerun-if-changed=ffi/chapel/ai/unified_nuclear_ai.chpl");
    println!("cargo:rerun-if-changed=ffi/shared/stealth_go.lib");
    println!("cargo:rerun-if-changed=ffi/shared/nuclear_zig.lib");
    println!("cargo:rerun-if-changed=ffi/shared/nuclear_nim.lib");
}
