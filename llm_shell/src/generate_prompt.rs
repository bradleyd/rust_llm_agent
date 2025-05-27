use serde_json::Value;

pub fn generate_prompt(agent_info: Option<&Value>, user_question: &str) -> String {
    let mut context_sections = Vec::new();

    if let Some(info) = agent_info {
        if info.get("results").is_some() {
            context_sections.push("Crate Info:".to_string());
            for item in info["results"].as_array().unwrap_or(&vec![]) {
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

        if info.get("docs").is_some() {
            context_sections.push("Documentation Snippets:".to_string());
            for doc in info["docs"].as_array().unwrap_or(&vec![]) {
                let text = doc.get("text").and_then(|v| v.as_str()).unwrap_or("");
                context_sections.push(format!("> {}", text));
            }
        }
    }

    let context = if context_sections.is_empty() {
        "No additional context available.".to_string()
    } else {
        context_sections.join("\n")
    };

    format!(
        "You are a Rust expert assistant. Use the following context to answer the question idiomatically. If a crate or documentation snippet is relevant, refer to it.

Context:
{}

Question: {}

Answer:",
        context, user_question
    )
}
