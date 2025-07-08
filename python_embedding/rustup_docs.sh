#!/bin/bash

# Activate virtual environment
source venv/bin/activate

echo "Extracting Rust documentation from HTML files..."
python3 extract_rust_docs.py

echo "Embedding rust-docs with chunking..."
python3 embed_docs.py --dir ../sample_docs/rust-docs --collection rust-docs --chunk-size 800 --overlap 100

echo "Embedding rust-book with chunking..."
python3 embed_docs.py --dir ../sample_docs/rust-book --collection rust-book --chunk-size 800 --overlap 100

echo "Done! Updated collections with chunked documents."
