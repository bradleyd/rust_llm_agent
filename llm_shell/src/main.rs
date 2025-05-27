use std::io::{self, Write};
use std::process::Command;
mod generate_prompt;
use generate_prompt::generate_prompt;

fn call_local_llm(prompt: &str) -> String {
    let output = Command::new("ollama")
        //.args(["run", "phi"])
        .args(["run", "openhermes"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            {
                let stdin = child.stdin.as_mut().unwrap();
                stdin.write_all(prompt.as_bytes())?;
            }
            let output = child.wait_with_output()?;
            Ok(output)
        });

    match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => "Failed to call local LLM.".to_string(),
    }
}

fn route_to_agent(user_input: &str) -> Option<&'static str> {
    let input = user_input.to_lowercase();

    if input.contains("crate") || input.contains("library") || input.contains("dependency") {
        println!("using crate agent");
        Some("crate_agent")
    } else if input.contains("example") || input.contains("project") || input.contains("github") {
        println!("using github agent");
        Some("github_agent")
    } else if input.contains("how")
        || input.contains("why")
        || input.contains("what")
        || input.contains("program")
    {
        println!("using docs agent");
        Some("docs_agent")
    } else {
        println!("using no agent");
        None
    }
}

fn run_agent(command_path: &str, query: &str) -> Option<serde_json::Value> {
    let output = std::process::Command::new(command_path)
        .arg(query)
        .output()
        .ok()?;

    if output.status.success() {
        let json_str = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str(&json_str).ok()
    } else {
        eprintln!("Agent failed: {}", command_path);
        None
    }
}

fn run_rag_query(query: &str) -> Option<serde_json::Value> {
    let output = std::process::Command::new("python3")
        .arg("python_embedding/query_docs.py")
        .arg(query)
        .output()
        .ok()?;

    if output.status.success() {
        let json_str = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str(&json_str).ok()
    } else {
        eprintln!("RAG query failed");
        None
    }
}

fn main() {
    println!("Enter your Rust crate topic:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let query = input.trim();

    let agent_to_use = route_to_agent(&query);

    let agent_output = match agent_to_use {
        Some("crate_agent") => run_agent("../agents/crate_agent/target/debug/crate_agent", &query),
        Some("github_agent") => {
            run_agent("../agents/github_agent/target/debug/github_agent", &query)
        }
        Some("docs_agent") => run_agent("../agents/docs_agent/target/debug/docs_agent", &query),
        _ => run_rag_query(query),
    };

    // 4. 🔥 Call generate_prompt here
    let prompt = generate_prompt(agent_output.as_ref(), &query);

    // 5. Send prompt to local LLM (Ollama)
    let response = call_local_llm(&prompt);
    println!("\nLLM Response:\n{}", response);
}
