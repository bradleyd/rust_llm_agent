import sys
import os
import requests
from bs4 import BeautifulSoup

SAVE_DIR = "../sample_docs/crates"

def fetch_crate_doc(crate_name):
    base_url = f"https://docs.rs/{crate_name}/latest/{crate_name}/"
    headers = {"User-Agent": "RustDocFetcher/1.0"}

    response = requests.get(base_url, headers=headers)
    if response.status_code != 200:
        print(f"Failed to fetch docs for {crate_name}: HTTP {response.status_code}")
        return

    soup = BeautifulSoup(response.text, "html.parser")
    content_div = soup.find("main")
    if not content_div:
        print(f"Could not find main content for {crate_name}")
        return

    # Remove scripts and navigation
    for tag in content_div.find_all(["script", "nav", "header", "footer"]):
        tag.decompose()

    text = content_div.get_text(separator="\n", strip=True)

    # Save to file
    os.makedirs(SAVE_DIR, exist_ok=True)
    save_path = os.path.join(SAVE_DIR, f"{crate_name}.md")
    with open(save_path, "w", encoding="utf-8") as f:
        f.write(f"# Crate: {crate_name}\n\n{text}")

    print(f"Saved docs for {crate_name} to {save_path}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python fetch_docsrs.py <crate_name>")
        sys.exit(1)

    crate = sys.argv[1]
    fetch_crate_doc(crate)
