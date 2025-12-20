//! Build script para Nuclear Crawler Hybrid
//!
//! FFI REAL con Go compilado para MSVC
//! 
//! ⚠️ ZIG FFI DESACTIVADO: Causa crash en Windows
//! ⚠️ NIM FFI DESACTIVADO: No disponible
//! 
//! NOTA: No usa cargo:warning= para evitar mensajes "warning:" en la salida

use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

const DEBUG_LOG_PATH: &str = "c:\\Users\\DELL\\Desktop\\hf_spaces\\NUCLEAR_CRAWLER_HYBRID\\.cursor\\debug.log";

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
    // FFI REAL - SOLO GO (ZIG Y NIM DESACTIVADOS)
    // ============================================================

    let go_lib_msvc = format!("{}/go/stealth_go_msvc.lib", manifest_dir);
    // let zig_lib = format!("{}/zig/zig-out/lib/nuclear_zig.lib", manifest_dir);
    // let nim_lib = format!("{}/nim/nuclear_nim.lib", manifest_dir);

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
        println!("cargo:rustc-link-search=native={}/go", manifest_dir);
        println!("cargo:rustc-link-lib=static=stealth_go_msvc");
        println!("cargo:rustc-cfg=has_go");

        // Go runtime dependencies - MSVC CRT + Windows libs
        println!("cargo:rustc-link-lib=dylib=msvcrt");
        println!("cargo:rustc-link-lib=dylib=legacy_stdio_definitions");
        println!("cargo:rustc-link-lib=dylib=ws2_32");
        println!("cargo:rustc-link-lib=dylib=winmm");
        println!("cargo:rustc-link-lib=dylib=ntdll");
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

    // Nim FFI no disponible
    // if std::path::Path::new(&nim_lib).exists() {
    //     println!("cargo:rustc-link-search=native={}/nim", manifest_dir);
    //     println!("cargo:rustc-link-lib=static=nuclear_nim");
    //     println!("cargo:rustc-cfg=has_nim");
    // }

    // Rerun si cambian las librerías
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=go/stealth_go_msvc.lib");
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
