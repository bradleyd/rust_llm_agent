// pub fn generate_router_prompt(question: &str) -> String {
//     format!(
//         "Decide which tool should be used to help answer the following question.

//         Available tools:
//         - crate_agent: Search crates.io for libraries
//         - github_agent: Search GitHub for example Rust code
//         - docs_agent: Fetch docs from stdlib or docs.rs
//         - none: Just use RAG context and answer directly

//         Question: {}

//         Answer:",
//         question
//     )
// }

// pub fn route_with_llm(question: &str) -> Option<&'static str> {
//     let prompt = generate_router_prompt(question);

//     let output = std::process::Command::new("ollama")
//         .args(["run", "openhermes"])
//         .stdin(std::process::Stdio::piped())
//         .stdout(std::process::Stdio::piped())
//         .spawn()
//         .and_then(|mut child| {
//             use std::io::Write;
//             if let Some(stdin) = child.stdin.as_mut() {
//                 stdin.write_all(prompt.as_bytes())?;
//             }
//             child.wait_with_output()
//         })
//         .ok()?;

//     let raw = String::from_utf8_lossy(&output.stdout).to_string();
//     let result = raw.trim().to_lowercase();

//     match result.as_str() {
//         "crate_agent" => Some("crate_agent"),
//         "github_agent" => Some("github_agent"),
//         "docs_agent" => Some("docs_agent"),
//         "none" => None,
//         _ => None,
//     }
// }
