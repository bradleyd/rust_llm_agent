import os
import chromadb
from sentence_transformers import SentenceTransformer

DOC_DIR = "../sample_docs"

model = SentenceTransformer("all-MiniLM-L6-v2")

# ✅ Use the new persistent API
client = chromadb.PersistentClient(path="./chroma_db")
collection = client.get_or_create_collection("rust_docs")

def embed_and_store(filename, content):
    chunks = content.split("\\n\\n")
    embeddings = model.encode(chunks)
    for i, chunk in enumerate(chunks):
        collection.add(
            documents=[chunk],
            embeddings=[embeddings[i].tolist()],
            metadatas=[{"source": filename}],
            ids=[f"{filename}_{i}"]
        )

def main():
    for fname in os.listdir(DOC_DIR):
        if not fname.endswith(".md"):
            continue
        path = os.path.join(DOC_DIR, fname)
        with open(path, "r", encoding="utf-8") as f:
            content = f.read()
            embed_and_store(fname, content)
    print("Docs embedded and saved to persistent Chroma DB.")

if __name__ == "__main__":
    main()
