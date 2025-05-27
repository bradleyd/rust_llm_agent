import sys
import chromadb
from sentence_transformers import SentenceTransformer

# Load model
model = SentenceTransformer("all-MiniLM-L6-v2")

# ✅ Load persistent Chroma DB
client = chromadb.PersistentClient(path="./chroma_db")
collection = client.get_or_create_collection("rust_docs")

def embed_query(text):
    return model.encode(text).tolist()

def search_docs(query_text, top_k=3):
    embedding = embed_query(query_text)
    results = collection.query(query_embeddings=[embedding], n_results=top_k)
    matches = [
        {"text": doc, "metadata": meta}
        for doc, meta in zip(results["documents"][0], results["metadatas"][0])
    ]
    return matches

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python query_docs.py \"your question here\"")
        sys.exit(1)

    question = sys.argv[1]
    results = search_docs(question)

    import json
    print(json.dumps({"docs": results}, indent=2))
