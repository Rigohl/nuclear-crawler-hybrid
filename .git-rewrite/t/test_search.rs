use std::process::Command;

fn main() {
    println!("🔍 Buscando proyectos Julia...");

    let output = Command::new("./target/release/nuclear-mcp.exe")
        .arg("--mode")
        .arg("http")
        .arg("--port")
        .arg("8079")
        .spawn()
        .expect("Failed to start server");

    std::thread::sleep(std::time::Duration::from_secs(5));

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("http://localhost:8079/call")
        .json(&serde_json::json!({
            "name": "websearch",
            "arguments": {
                "query": "Julia programming language projects 2024"
            }
        }))
        .send()
        .expect("Failed to send request");

    let result: serde_json::Value = response.json().expect("Failed to parse response");

    if let Some(results) = result.get("result").and_then(|r| r.as_array()) {
        for (i, item) in results.iter().enumerate().take(10) {
            if let (Some(title), Some(url)) = (
                item.get("title").and_then(|t| t.as_str()),
                item.get("url").and_then(|u| u.as_str())
            ) {
                println!("{}. {} - {}", i + 1, title, url);
            }
        }
    } else {
        println!("No se encontraron resultados");
    }

    output.kill().ok();
}