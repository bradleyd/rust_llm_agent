#!/bin/bash

echo "Extracting Rustup documentation..."
python3 extract_rustup_docs.py

echo "Embedding extracted documentation into Chroma..."
python3 embed_docs.py

echo "Done."
