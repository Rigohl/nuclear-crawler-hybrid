// Archivo de ejemplo 2: Más código para reemplazo masivo
fn new_function_name(value: f64) -> f64 {
    println!("Procesando valor: {}", value);
    value + 10.0
}

fn calculate_old_way(data: Vec<i32>) -> i32 {
    let mut sum = 0;
    for item in data {
        sum += new_function_name(item as f64) as i32;
    }
    sum
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let result = calculate_old_way(data);
    println!("Suma total: {}", result);
}