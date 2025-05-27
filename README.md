# Option 1: Python + Chroma Vector DB

This directory contains Python scripts for embedding and querying Rust-related documents.
- `embed_docs.py`: Loads markdown/text files and stores them in a Chroma vector DB.
- `query_docs.py`: Embeds a question and returns top matches from the DB.
- `sample_docs/`: Place your Rust docs or crate README files here.

To use:
```bash
pip install -r requirements.txt
python embed_docs.py
python query_docs.py "How do I deserialize JSON into an enum?"
```