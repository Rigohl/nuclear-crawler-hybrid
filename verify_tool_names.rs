fn get_tool_names() -> Vec<String> {
    vec![
        "websearch".to_string(),
        "premium".to_string(),
        "file_search".to_string(),
        "scan".to_string(),
        "ai_dataset_trainer".to_string(),
        "parallel_engine".to_string(),
        "osint_intelligence".to_string(),
    ]
}

fn main() {
    let names = get_tool_names();
    println!("{:?}", names);
}
