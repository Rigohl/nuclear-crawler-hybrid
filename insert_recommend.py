import sys

with open("src/core/web_search.rs", "r") as f:
    lines = f.readlines()

output = []
for line in lines:
    output.append(line)
    if "fn calculate_relevance" in line:
        output.insert(len(output)-2, """
    /// Uses Chapel AI to recommend search strategies based on provided sources
    fn recommend_massive_parallel_search(&self, sources: Vec<String>) {
        let chapel_ai = get_chapel_ai();
        if !chapel_ai.is_ffi_available() {
            return;
        }

        let context = create_context(
            "websearch",
            "massive_parallel_search",
            sources.join(","),
            1.0,
        );

        if let Ok(_) = chapel_ai.learn(context) {
            if let Ok(advice) = chapel_ai.get_advice("websearch", "massive_parallel_search") {
                for a in advice {
                    eprintln!("   🧠 Chapel AI Suggestion ({}): {}", a.priority, a.suggestion);
                }
            }
        }
    }
""")

with open("src/core/web_search.rs", "w") as f:
    f.writelines(output)
