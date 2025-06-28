use regex::Regex;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn generate_tool_plan_prompt(question: &str) -> String {
    format!(
        "You are a Rust assistant that chooses which tools to run for answering a question.
Available tools:
- crate_agent: finds crates on crates.io (use for finding crate names or features)
- github_agent: finds example Rust code on GitHub (use for vague questions or tutorials)
- docs_agent: fetches Rust stdlib or crate docs from docs.rs (use for specific functions or stdlib modules)
- none: for questions that don't need a tool and can be answered from general knowledge

Return a JSON array plan of tools to run in order. Each step must include a 'tool' and 'input'.

Question: {}

Plan:",
        question
    )
}

pub fn generate_plan(question: &str) -> Vec<(String, String)> {
    let prompt = generate_tool_plan_prompt(question);

    let output = Command::new("ollama")
        .args(["run", "llama3.2:latest"])
        //.args(["run", "deepseek-r1:8b"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(prompt.as_bytes())?;
            }
            child.wait_with_output()
        });

    if let Ok(result) = output {
        if let Ok(text) = String::from_utf8(result.stdout) {
            println!("Tool plan response: {}", text);

            // Extract first JSON array block using regex
            let json_re = Regex::new(r"(?s)\[\s*\{.*\}\s*\]").unwrap();
            if let Some(mat) = json_re.find(&text) {
                let json_str = mat.as_str();
                if let Ok(json) = serde_json::from_str::<Vec<serde_json::Value>>(json_str) {
                    println!("Parsed JSON: {:#?}", json);
                    return json
                        .into_iter()
                        .filter_map(|step| {
                            let tool = step.get("tool")?.as_str()?.to_string();
                            let input = step.get("input")?.as_str()?.to_string();
                            Some((tool, input))
                        })
                        .collect();
                }
            }
        }
    }

    vec![]
}
