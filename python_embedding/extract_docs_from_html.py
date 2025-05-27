import os
from bs4 import BeautifulSoup

DOC_DIR = "../agents/crate_agent/target/doc"
OUT_DIR = "../sample_docs"

os.makedirs(OUT_DIR, exist_ok=True)

def extract_text_from_html(file_path):
    with open(file_path, "r", encoding="utf-8") as f:
        soup = BeautifulSoup(f, "html.parser")

    # Remove navigation and script elements
    for tag in soup(["nav", "script", "style", "footer", "header"]):
        tag.decompose()

    main_content = soup.find("main")
    text = main_content.get_text(separator="\n", strip=True) if main_content else soup.get_text(separator="\n", strip=True)
    return text

def main():
    for root, _, files in os.walk(DOC_DIR):
        for file in files:
            if file.endswith(".html") and "src/" not in root:
                full_path = os.path.join(root, file)
                crate_name = os.path.basename(root)
                output_file = os.path.join(OUT_DIR, f"{crate_name}.md")
                try:
                    content = extract_text_from_html(full_path)
                    with open(output_file, "w", encoding="utf-8") as f:
                        f.write(f"# Extracted from {file}\n\n")
                        f.write(content)
                    print(f"✓ {output_file}")
                except Exception as e:
                    print(f"Failed to parse {full_path}: {e}")

if __name__ == "__main__":
    main()
