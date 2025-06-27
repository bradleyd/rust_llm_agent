use scraper::{Html, Selector};
use std::env;
use url::Url;

fn extract_main_text(html: &str) -> String {
    let document = Html::parse_document(html);
    let main_selector = Selector::parse("main").unwrap();
    let mut output = String::new();

    if let Some(main_element) = document.select(&main_selector).next() {
        let selector = Selector::parse("h1, h2, h3, h4, h5, h6, p, pre, a").unwrap();
        for element in main_element.select(&selector) {
            match element.value().name() {
                "pre" => {
                    let code = element.text().collect::<String>();
                    output.push_str(&format!("\n```rust\n{}```\n", code));
                }
                "a" => {
                    let link_text = element.text().collect::<String>();
                    if let Some(href) = element.value().attr("href") {
                        output.push_str(&format!("[{}]({})", link_text, href));
                    } else {
                        output.push_str(&link_text);
                    }
                }
                _ => {
                    let text = element.text().collect::<String>();
                    if !text.trim().is_empty() {
                        output.push_str("\n");
                        output.push_str(&text);
                        output.push_str("\n");
                    }
                }
            }
        }
    }

    output.trim().to_string()
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

    let extracted = extract_main_text(&body);
    let preview = &extracted[..std::cmp::min(extracted.len(), 4000)];

    let output = serde_json::json!({
        "input": input,
        "url": url,
        "docs": preview
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
