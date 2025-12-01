extern crate nuclear_crawler_hybrid;
use nuclear_crawler_hybrid::run_parallel_processor;

fn main() {
    println!("🧪 Testing Chapel Integration...");

    // Test URLs
    let test_urls = vec![
        "https://example.com".to_string(),
        "https://httpbin.org/get".to_string(),
        "https://google.com".to_string(),
    ];

    match run_parallel_processor(test_urls) {
        Ok(result) => {
            println!("✅ Chapel integration successful!");
            println!("📄 Result: {}", result);
        }
        Err(e) => {
            println!("❌ Chapel integration failed: {}", e);
        }
    }
}