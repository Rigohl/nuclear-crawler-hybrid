use math_dataset_processor::MathDatasetProcessor;

fn main() {
    println!("Math Dataset Processor - Rust FFI Module");
    println!("Este módulo está diseñado para ser usado desde Python vía PyO3");
    println!("");
    println!("Para compilar:");
    println!("  cargo build --release");
    println!("");
    println!("Para usar desde Python:");
    println!("  import math_dataset_processor");
    println!("  processor = math_dataset_processor.MathDatasetProcessor()");
}