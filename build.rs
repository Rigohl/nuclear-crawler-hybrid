//! Build script para Nuclear Crawler Hybrid
//!
//! FFI REAL con Go, Zig y Nim compilados para MSVC
//!
//! 🔥 ACTIVADO: Go (goroutines), Zig (SIMD), Nim (parsing)
//!
//! NOTA: No usa cargo:warning= para evitar mensajes "warning:" en la salida

use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

const DEBUG_LOG_PATH: &str =
    "c:\\Users\\DELL\\Desktop\\hf_spaces\\NUCLEAR_CRAWLER_HYBRID\\.cursor\\debug.log";

fn write_debug_log(hypothesis_id: &str, location: &str, message: &str, data: &str) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    // #region agent log
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(DEBUG_LOG_PATH)
    {
        let _ = writeln!(
            file,
            r#"{{"sessionId":"debug-session","runId":"pre-fix","hypothesisId":"{hypothesis}","location":"{location}","message":"{message}","data":{data},"timestamp":{ts}}}"#,
            hypothesis = hypothesis_id,
            location = location,
            message = message,
            data = data,
            ts = timestamp
        );
    }
    // #endregion
}

fn main() {
    // Declarar cfgs personalizados para evitar warnings
    println!("cargo::rustc-check-cfg=cfg(has_go)");
    println!("cargo::rustc-check-cfg=cfg(has_zig)");
    println!("cargo::rustc-check-cfg=cfg(has_nim)");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let pid = std::process::id();
    write_debug_log(
        "H1",
        "build.rs:entry",
        "build.rs start",
        &format!(
            r#"{{"pid":{},"manifestDir":"{}"}}"#,
            pid,
            manifest_dir.replace('\\', "\\\\")
        ),
    );

    // ============================================================
    // FFI REAL - GO, ZIG Y NIM ACTIVADOS PARA MÁXIMA POTENCIA
    // ============================================================

    let go_lib_msvc = format!("{}/ffi/go/stealth_go_msvc.lib", manifest_dir);
    let zig_lib_candidates = [
        format!("{}/ffi/zig/nuclear_zig.lib", manifest_dir),
        format!("{}/ffi/zig/lib.lib", manifest_dir),
        format!("{}/ffi/shared/nuclear_zig.lib", manifest_dir),
        format!("{}/ffi/zig/zig-out/lib/nuclear_zig.lib", manifest_dir),
    ];

    let nim_lib_candidates = [
        format!("{}/ffi/nim/nuclear_nim.lib", manifest_dir),
        format!("{}/ffi/shared/nuclear_nim.lib", manifest_dir),
    ];

    // Linkear Go FFI
    let go_exists = std::path::Path::new(&go_lib_msvc).exists();
    write_debug_log(
        "H2",
        "build.rs:go_link",
        "check go lib presence",
        &format!(
            r#"{{"goLibPath":"{}","exists":{}}}"#,
            go_lib_msvc.replace('\\', "\\\\"),
            go_exists
        ),
    );

    if go_exists {
        println!("cargo:rustc-link-search=native={}/ffi/go", manifest_dir);
        println!("cargo:rustc-link-lib=static=stealth_go_msvc");
        println!("cargo:rustc-cfg=has_go");

        // Go runtime dependencies - MSVC CRT + Windows libs
        println!("cargo:rustc-link-lib=dylib=msvcrt");
        println!("cargo:rustc-link-lib=dylib=legacy_stdio_definitions");
        println!("cargo:rustc-link-lib=dylib=ws2_32");
        println!("cargo:rustc-link-lib=dylib=winmm");
        println!("cargo:rustc-link-lib=dylib=ntdll");
    }

    // Intenta detectar y linkear Zig static lib
    let mut zig_found = None;
    for cand in zig_lib_candidates.iter() {
        if std::path::Path::new(cand).exists() {
            zig_found = Some(cand.clone());
            break;
        }
    }

    // If not found, try to build with `zig build` if Zig is installed and zig/ exists
    if zig_found.is_none() {
        let zig_dir = format!("{}/ffi/zig", manifest_dir);
        if std::path::Path::new(&zig_dir).exists() {
            // Try to locate zig executable: first try system PATH, then common user locations
            let mut zig_exec: Option<String> = None;

            // Try plain 'zig' (PATH)
            if let Ok(output) = std::process::Command::new("zig").arg("version").output() {
                if output.status.success() {
                    zig_exec = Some("zig".to_string());
                }
            }

            // Common candidate locations (user indicated Zig is installed under DELL)
            if zig_exec.is_none() {
                let candidates = [
                    "C:/Users/DELL/zig/zig.exe".to_string(),
                    "D:/Users/DELL/zig/zig.exe".to_string(),
                    "C:/Program Files/Zig/zig.exe".to_string(),
                    "C:/zig/zig.exe".to_string(),
                ];

                for cand in candidates.iter() {
                    if std::path::Path::new(cand).exists() {
                        zig_exec = Some(cand.clone());
                        break;
                    }
                }
            }

            if let Some(zig_bin) = zig_exec {
                // Attempt to run `zig build` in zig_dir using the found binary
                let build_result = std::process::Command::new(zig_bin)
                    .arg("build")
                    .current_dir(&zig_dir)
                    .output();

                if let Ok(output) = build_result {
                    let out = String::from_utf8_lossy(&output.stdout).to_string();
                    let err = String::from_utf8_lossy(&output.stderr).to_string();
                    write_debug_log(
                        "HZ1",
                        "build.rs:zig_build",
                        "zig build output",
                        &format!(
                            "{{\"out\":\"{}\",\"err\":\"{}\"}}",
                            out.replace('\\', "\\\\"),
                            err.replace('\\', "\\\\")
                        ),
                    );

                    // After build, re-check candidates
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

    // ⚠️ ZIG FFI ACTIVADO - SIMD para máxima velocidad
    if let Some(zig_path) = zig_found {
        // Extraer el directorio donde está la librería
        if let Some(dir) = std::path::Path::new(&zig_path).parent() {
            println!("cargo:rustc-link-search=native={}", dir.display());
        }
        println!("cargo:rustc-link-lib=static=nuclear_zig");
        println!("cargo:rustc-cfg=has_zig");
        write_debug_log(
            "H4",
            "build.rs:zig_link",
            "zig lib linked",
            &format!("{{\"zigLibPath\":\"{}\"}}", zig_path.replace('\\', "\\\\")),
        );
    }

    // Intenta detectar y linkear Nim static lib
    let mut nim_found = None;
    for cand in nim_lib_candidates.iter() {
        if std::path::Path::new(cand).exists() {
            nim_found = Some(cand.clone());
            break;
        }
    }

    // ⚠️ NIM FFI ACTIVADO - Parsing HTML ultra-rápido
    if let Some(nim_path) = nim_found {
        if let Some(dir) = std::path::Path::new(&nim_path).parent() {
            println!("cargo:rustc-link-search=native={}", dir.display());
        }
        println!("cargo:rustc-link-lib=static=nuclear_nim");
        println!("cargo:rustc-cfg=has_nim");
        write_debug_log(
            "H5",
            "build.rs:nim_link",
            "nim lib linked",
            &format!("{{\"nimLibPath\":\"{}\"}}", nim_path.replace('\\', "\\\\")),
        );
    }

    // ⚠️ ZIG FFI DESACTIVADO - Causa crash:
    // - "thread panic: integer overflow"
    // - "thread has overflowed its stack"
    // Usar implementación Rust pura con blake3/rayon
    // if std::path::Path::new(&zig_lib).exists() {
    //     println!("cargo:rustc-link-search=native={}/zig/zig-out/lib", manifest_dir);
    //     println!("cargo:rustc-link-lib=static=nuclear_zig");
    //     println!("cargo:rustc-cfg=has_zig");
    // }

    // Rerun si cambian las librerías
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=ffi/go/stealth_go_msvc.lib");
    println!("cargo:rerun-if-changed=ffi/zig/nuclear_zig.lib");
    println!("cargo:rerun-if-changed=ffi/nim/nuclear_nim.lib");
    write_debug_log(
        "H3",
        "build.rs:exit",
        "build.rs end",
        &format!(
            r#"{{"goLibLinked":{},"manifestDir":"{}"}}"#,
            go_exists,
            manifest_dir.replace('\\', "\\\\")
        ),
    );
}
