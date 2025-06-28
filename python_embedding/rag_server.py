from fastapi import FastAPI, Query
from chromadb import PersistentClient
from fastapi.middleware.cors import CORSMiddleware
import uvicorn

# Initialize ChromaDB client
client = PersistentClient(path="./chroma_db")

# Load known collections
collections = {
    "rust-docs": client.get_or_create_collection("rust-docs"),
    "rust-book": client.get_or_create_collection("rust-book"),
    "crates": client.get_or_create_collection("crates"),
}

app = FastAPI()
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

@app.get("/query")
def query_docs(collection: str = Query(...), query: str = Query(...)):
    col = collections.get(collection)
    if not col:
        return {"error": f"Collection '{collection}' not found"}
    results = col.query(query_texts=[query], n_results=2)
    return {"docs": results["documents"]}

if __name__ == "__main__":
    uvicorn.run("rag_server:app", host="127.0.0.1", port=8000, reload=True)
