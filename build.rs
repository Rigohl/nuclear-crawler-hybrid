fn main() {
    // Configurar check-cfg para has_nim
    println!("cargo::rustc-check-cfg=cfg(has_nim)");
    
    let current_dir = std::env::current_dir().unwrap();
    let current_dir_str = current_dir.display().to_string();
    
    // Agregar directorio actual al search path para las librerías
    println!("cargo:rustc-link-search=native={}", current_dir_str);
    
    // Compilar wrapper C para símbolos faltantes (siempre en Windows - SQLite lo necesita)
    #[cfg(windows)]
    {
        cc::Build::new()
            .file("src/c_wrapper.c")
            .compile("c_wrapper");
    }
    
    // Solo enlazar librerías si existen
    #[cfg(feature = "go")]
    {
        if std::path::Path::new("stealth_go.lib").exists() {
            println!("cargo:rustc-link-lib=static=stealth_go");
            // Enlazar con librería de importación para fprintf si existe
            if std::path::Path::new("msvcrt_import.lib").exists() {
                println!("cargo:rustc-link-lib=static=msvcrt_import");
            }
        }
    }
    
    #[cfg(feature = "zig")]
    {
        if std::path::Path::new("zig/nuclear_zig.lib").exists() || 
           std::path::Path::new("nuclear_zig.lib").exists() {
            println!("cargo:rustc-link-lib=static=nuclear_zig");
        }
    }
    
    // En Windows MSVC, configurar linker para resolver símbolos C faltantes
    #[cfg(all(windows, any(feature = "go", feature = "zig")))]
    {
        // Go compilado con gcc necesita fprintf - está en msvcrt pero puede necesitarse explícitamente
        // Zig necesita stack protector - estas funciones están en msvcrt
        // Configurar linker para que resuelva estos símbolos
        println!("cargo:rustc-link-arg=/DEFAULTLIB:msvcrt");
    }
}
