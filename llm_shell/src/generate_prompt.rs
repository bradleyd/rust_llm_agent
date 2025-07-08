use serde_json::Value;

pub fn format_agent_output_for_llm(tool_name: &str, agent_info: &Value) -> String {
    let mut context_sections = Vec::new();

    context_sections.push(format!("From {}:", tool_name));

    if agent_info.get("results").is_some() {
        context_sections.push("Crate Info:".to_string());
        for item in agent_info["results"].as_array().unwrap_or(&vec![]) {
            let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let desc = item
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let ver = item.get("version").and_then(|v| v.as_str()).unwrap_or("");
            let homepage = item.get("homepage").and_then(|v| v.as_str()).unwrap_or("");
            context_sections.push(format!(
                "- {} (v{}): {}
  {}",
                name, ver, desc, homepage
            ));
        }
    }

    if agent_info.get("docs").is_some() {
        context_sections.push("Documentation Snippets:".to_string());
        for doc in agent_info["docs"].as_array().unwrap_or(&vec![]) {
            let text = doc.get("text").and_then(|v| v.as_str()).unwrap_or("");
            context_sections.push(format!("> {}", text));
        }
    }

    // If no specific sections were found, just include the raw JSON string for debugging/fallback
    if context_sections.len() == 1 { // Only "From {tool_name}:" was added
        context_sections.push(format!("Raw output: {}", agent_info));
    }

    context_sections.join("\n")
}
