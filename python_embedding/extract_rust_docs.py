import os
from pathlib import Path
from bs4 import BeautifulSoup

# Configure source HTML root and output collection type
DOC_ROOTS = {
    "rust-docs": os.path.expanduser("~/.rustup/toolchains/stable-aarch64-apple-darwin/share/doc/rust/html/std"),
    "rust-book": os.path.expanduser("~/.rustup/toolchains/stable-aarch64-apple-darwin/share/doc/rust/html/book"),
}

OUTPUT_BASE = Path(__file__).resolve().parent / "../sample_docs"

def extract_html_text(html_path):
    with open(html_path, "r", encoding="utf-8") as f:
        soup = BeautifulSoup(f, "html.parser")
    # Remove nav/sidebar/footer
    for tag in soup(["nav", "header", "footer", "script", "style"]):
        tag.decompose()
    return soup.get_text(separator="\n", strip=True)

def main():
    for collection, root_path in DOC_ROOTS.items():
        src_path = Path(root_path)
        out_path = OUTPUT_BASE / collection
        out_path.mkdir(parents=True, exist_ok=True)

        for html_file in src_path.rglob("*.html"):
            if html_file.name in ["index.html", "404.html"]:
                continue
            try:
                content = extract_html_text(html_file)
                slug = html_file.stem.replace(".", "_")
                output_file = out_path / f"{slug}.md"
                with open(output_file, "w", encoding="utf-8") as f:
                    f.write(content)
                print(f"✅ Extracted: {output_file}")
            except Exception as e:
                print(f"❌ Failed to extract {html_file}: {e}")

if __name__ == "__main__":
    main()

