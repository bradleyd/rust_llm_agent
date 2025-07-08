import os
import sys
import chromadb
from chromadb import PersistentClient
from pathlib import Path
import argparse
import re
import hashlib

def chunk_text(text, chunk_size=800, overlap=100):
    """Split text into overlapping chunks"""
    words = text.split()
    chunks = []
    
    for i in range(0, len(words), chunk_size - overlap):
        chunk = ' '.join(words[i:i + chunk_size])
        if chunk.strip():
            chunks.append(chunk)
        
        # Stop if we've reached the end
        if i + chunk_size >= len(words):
            break
    
    return chunks

def extract_title(content):
    """Extract title from markdown content"""
    lines = content.split('\n')
    for line in lines:
        if line.startswith('# '):
            return line[2:].strip()
    return None

def create_chunk_id(file_path, chunk_index):
    """Create unique ID for chunk"""
    base = f"{file_path}#{chunk_index}"
    return hashlib.md5(base.encode()).hexdigest()

parser = argparse.ArgumentParser(description="Embed Markdown docs into ChromaDB with chunking")
parser.add_argument("--dir", required=True, help="Path to docs directory")
parser.add_argument("--collection", required=True, help="ChromaDB collection name")
parser.add_argument("--chunk-size", type=int, default=800, help="Chunk size in words")
parser.add_argument("--overlap", type=int, default=100, help="Overlap between chunks")
args = parser.parse_args()

BASE_DIR = Path(__file__).resolve().parent
CHROMA_PATH = BASE_DIR / "chroma_db"
client = PersistentClient(path=str(CHROMA_PATH))

collection = client.get_or_create_collection(name=args.collection)

docs = []
ids = []
metadatas = []

for file in Path(args.dir).rglob("*.md"):
    content = file.read_text(encoding="utf-8")
    title = extract_title(content)
    
    chunks = chunk_text(content, args.chunk_size, args.overlap)
    
    for i, chunk in enumerate(chunks):
        docs.append(chunk)
        ids.append(create_chunk_id(str(file.resolve()), i))
        metadatas.append({
            "file_path": str(file.resolve()),
            "file_name": file.name,
            "chunk_index": i,
            "total_chunks": len(chunks),
            "title": title or file.stem
        })

if not docs:
    print("❌ No .md files found in", args.dir)
    sys.exit(1)

collection.add(
    documents=docs,
    ids=ids,
    metadatas=metadatas
)

print(f"✅ Embedded {len(docs)} chunks from {len(set(m['file_path'] for m in metadatas))} files into collection '{args.collection}'")

