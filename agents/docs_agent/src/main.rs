use scraper::{Html, Selector};
use std::env;

fn extract_docs_as_json(html: &str) -> serde_json::Value {
    let document = Html::parse_document(html);
    let main_selector = Selector::parse("main").unwrap();
    let mut docs = Vec::new();

    if let Some(main_element) = document.select(&main_selector).next() {
        let content_selector = Selector::parse("p, pre").unwrap();
        for element in main_element.select(&content_selector) {
            let name = element.value().name();
            let text = element.text().collect::<String>().trim().to_string();

            if text.is_empty() {
                continue;
            }

            let entry = if name == "pre" {
                serde_json::json!({
                    "type": "code",
                    "content": text
                })
            } else { // "p"
                serde_json::json!({
                    "type": "paragraph",
                    "content": text
                })
            };
            docs.push(entry);
        }
    }

    serde_json::Value::Array(docs)
}

fn build_url(crate_input: &str) -> String {
    if crate_input.contains("::") {
        let parts: Vec<&str> = crate_input.split("::").collect();
        let crate_base = parts[0];
        let last = parts.last().unwrap_or(&"");

        if ["std", "core", "alloc"].contains(&crate_base) {
            if last.chars().next().map(|c| c.is_lowercase()).unwrap_or(true) {
                format!(
                    "https://doc.rust-lang.org/{}/{}/index.html",
                    crate_base,
                    parts[1..].join("/")
                )
            } else {
                format!(
                    "https://doc.rust-lang.org/{}/{}/struct.{}.html",
                    crate_base,
                    parts[1..parts.len() - 1].join("/"),
                    last
                )
            }
        } else {
            format!("https://docs.rs/{}/latest/{}/?search={}", crate_base, crate_base, parts[1..].join("::"))
        }
    } else {
        format!("https://docs.rs/{}/latest/{}/", crate_input, crate_input)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: docs_agent <crate_name>");
        std::process::exit(1);
    }

    let input = &args[1];
    let url = build_url(input);

    let body = reqwest::get(&url).await?.text().await?;

    let docs = extract_docs_as_json(&body);

    let output = serde_json::json!({
        "input": input,
        "url": url,
        "docs": docs
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
