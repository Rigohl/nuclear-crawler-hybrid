// Archivo de ejemplo 1: Código Rust con funciones viejas
fn new_function_name(param: i32) -> i32 {
    println!("Esta es la función vieja");
    param * 2
}

fn another_new_function() {
    println!("Otra función que necesita cambio");
}

fn main() {
    let result = new_function_name(42);
    println!("Resultado: {}", result);
    another_new_function();
}