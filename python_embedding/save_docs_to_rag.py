import sys
import subprocess
import json
import chromadb

client = chromadb.PersistentClient(path="./chroma_db")
collection = client.get_or_create_collection("rust_docs")

def run_docs_agent(crate):
    result = subprocess.run(
        ["../agents/docs_agent/target/release/docs_agent", crate],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    print(result.stdout)
    if result.returncode != 0:
        print("Agent failed:", result.stderr)
        return None

    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError as e:
        print(f"[error] Invalid JSON output from docs_agent:\n{result.stdout}")
        return None

def add_to_chroma(crate, doc_text):
    doc_id = f"docs_{crate}"
    collection.add(
        documents=[doc_text],
        metadatas=[{"source": f"{crate}_docs_agent"}],
        ids=[doc_id]
    )
    print(f"✓ Added {crate} to Chroma")

def main():
    if len(sys.argv) < 2:
        print("Usage: python save_docs_to_rag.py <crate_name>")
        sys.exit(1)

    crate = sys.argv[1]
    result = run_docs_agent(crate)
    if result and "docs" in result:
        add_to_chroma(result["input"], result["docs"])

if __name__ == "__main__":
    main()
