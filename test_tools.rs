use std::collections::HashMap;

// Mock the struct since we don't have dependencies
#[derive(Debug, Clone)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
}

pub enum ToolProfile {
    Full,
    Pro,
    Lite,
}

pub fn get_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition { name: "websearch".to_string(), description: "desc".to_string() },
        ToolDefinition { name: "premium".to_string(), description: "desc".to_string() },
        ToolDefinition { name: "file_search".to_string(), description: "desc".to_string() },
        ToolDefinition { name: "scan".to_string(), description: "desc".to_string() },
        ToolDefinition { name: "ai_dataset_trainer".to_string(), description: "desc".to_string() },
        ToolDefinition { name: "parallel_engine".to_string(), description: "desc".to_string() },
        ToolDefinition { name: "osint_intelligence".to_string(), description: "desc".to_string() },
    ]
}

pub fn get_tool_definition(name: &str) -> Option<ToolDefinition> {
    get_tool_definitions().into_iter().find(|t| t.name == name)
}

pub fn tool_exists(name: &str) -> bool {
    get_tool_definitions().iter().any(|t| t.name == name)
}

pub fn get_tool_names() -> Vec<String> {
    get_tool_definitions()
        .iter()
        .map(|t| t.name.clone())
        .collect()
}

pub fn get_profile_tool_names(profile: ToolProfile) -> Vec<&'static str> {
    match profile {
        ToolProfile::Full => vec![
            "websearch",
            "premium",
            "file_search",
            "scan",
            "ai_dataset_trainer",
            "parallel_engine",
            "osint_intelligence",
        ],
        ToolProfile::Pro => vec![
            "websearch",
            "premium",
            "file_search",
            "scan",
            "ai_dataset_trainer",
        ],
        ToolProfile::Lite => vec!["websearch", "scan"],
    }
}

fn test_tool_exists() {
    assert!(tool_exists("websearch"));
    assert!(tool_exists("osint_intelligence"));
    assert!(!tool_exists("non_existent_tool"));
    assert!(!tool_exists(""));
    println!("test_tool_exists passed!");
}

fn test_get_tool_definition() {
    let def = get_tool_definition("websearch");
    assert!(def.is_some());
    assert_eq!(def.unwrap().name, "websearch");

    let missing = get_tool_definition("fake_tool_123");
    assert!(missing.is_none());
    println!("test_get_tool_definition passed!");
}

fn test_get_profile_tool_names() {
    let full_tools = get_profile_tool_names(ToolProfile::Full);
    assert_eq!(full_tools.len(), 7);
    assert!(full_tools.contains(&"websearch"));

    let pro_tools = get_profile_tool_names(ToolProfile::Pro);
    assert_eq!(pro_tools.len(), 5);
    assert!(pro_tools.contains(&"ai_dataset_trainer"));

    let lite_tools = get_profile_tool_names(ToolProfile::Lite);
    assert_eq!(lite_tools.len(), 2);
    assert!(lite_tools.contains(&"scan"));
    println!("test_get_profile_tool_names passed!");
}

fn main() {
    test_tool_exists();
    test_get_tool_definition();
    test_get_profile_tool_names();
}
