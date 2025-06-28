import os
import sys
import chromadb
from chromadb import PersistentClient
from pathlib import Path
import argparse

parser = argparse.ArgumentParser(description="Embed Markdown docs into ChromaDB")
parser.add_argument("--dir", required=True, help="Path to docs directory")
parser.add_argument("--collection", required=True, help="ChromaDB collection name")
args = parser.parse_args()

BASE_DIR = Path(__file__).resolve().parent
CHROMA_PATH = BASE_DIR / "chroma_db"
client = PersistentClient(path=str(CHROMA_PATH))

collection = client.get_or_create_collection(name=args.collection)

docs = []
ids = []

for file in Path(args.dir).rglob("*.md"):
    content = file.read_text(encoding="utf-8")
    docs.append(content)
    ids.append(str(file.resolve()))

if not docs:
    print("❌ No .md files found in", args.dir)
    sys.exit(1)

collection.add(
    documents=docs,
    ids=ids[:len(docs)]
)

print(f"✅ Embedded {len(docs)} documents into collection '{args.collection}'")

