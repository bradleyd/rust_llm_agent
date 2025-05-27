use scraper::{Html, Selector};
use std::env;

fn extract_main_text(html: &str) -> String {
    let document = Html::parse_document(html);
    let selector = Selector::parse("main").unwrap();

    let mut output = String::new();
    for element in document.select(&selector) {
        output.push_str(&element.text().collect::<Vec<_>>().join(" "));
    }

    output
}

fn build_url(crate_input: &str) -> String {
    if crate_input.contains("::") {
        let parts: Vec<&str> = crate_input.split("::").collect();
        let crate_base = parts[0];
        let last = parts.last().unwrap_or(&"");

        // Default to module index if last is lowercase
        if last
            .chars()
            .next()
            .map(|c| c.is_lowercase())
            .unwrap_or(true)
        {
            format!(
                "https://doc.rust-lang.org/{}/{}",
                crate_base,
                parts[1..].join("/") + "/index.html"
            )
        } else {
            // Capitalized = struct, enum, trait, etc.
            format!(
                "https://doc.rust-lang.org/{}/{}",
                crate_base,
                parts[1..parts.len() - 1].join("/") + &format!("/struct.{}.html", last)
            )
        }
    } else if ["std", "core", "alloc"].contains(&crate_input) {
        format!("https://doc.rust-lang.org/{}/index.html", crate_input)
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
    let preview = &extracted[..std::cmp::min(extracted.len(), 1000)];

    let output = serde_json::json!({
        "input": input,
        "url": url,
        "docs": preview
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_std_docs() {
        assert_eq!(build_url("std"), "https://doc.rust-lang.org/std/index.html");
    }

    #[test]
    fn test_std_with_colons() {
        assert_eq!(
            build_url("std::vec"),
            "https://doc.rust-lang.org/std/vec/index.html"
        );
    }

    #[test]
    fn test_std_with_struct() {
        assert_eq!(
            build_url("std::vec::Vec"),
            "https://doc.rust-lang.org/std/vec/struct.Vec.html"
        );
    }

    #[test]
    fn test_crate_docs() {
        assert_eq!(
            build_url("serde_json"),
            "https://docs.rs/serde_json/latest/serde_json/"
        );
    }
}
