# 🦀 Rust LLM Agent — Local Developer Assistant

This project is a local Rust-powered AI assistant that uses embedded documentation, agent tools, and a local LLM to help answer Rust programming questions — with context-aware code generation and crate discovery.

---

## 📦 Features

- **RAG (Retrieval-Augmented Generation)** with ChromaDB
- 🕵️‍♂️ **Agents** for:
  - Crate docs (`docs.rs`)
  - GitHub repo search + README parsing
  - Standard library and Rust Book content
- 🤖 Local LLM (LLaMA3 / Mistral / OpenHermes) integration
- 🧠 Chat interface with memory + tool calling
- 📁 Modular source loading from local docs (`rustup`, crates, scraped pages)

---

## 🛠️ Setup

### 1. Clone the project

```bash
git clone https://github.com/yourname/rust_llm_agent_updated.git
cd rust_llm_agent_updated
```

### 2. Set up Python (for embeddings + ChromaDB)

```bash
cd python_embedding
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

---

## 🧠 Load Documentation into Vector DB

You can load any Rust content (books, docs, READMEs) into `ChromaDB`.

### Supported Sources:
- Local docs via `rustup` (HTML extraction + chunking)
- Crate docs from `docs.rs` (automated fetching)
- Rust book or stdlib
- GitHub crate READMEs

### ✅ Quick Setup - Load All Documentation

**Option 1: Load Rust core docs (Book + Stdlib)**
```bash
cd python_embedding
chmod +x rustup_docs.sh
./rustup_docs.sh
```

**Option 2: Load popular crate documentation**
```bash
cd python_embedding
chmod +x crate_docs.sh
./crate_docs.sh all  # Fetches serde, tokio, clap, etc.
```

**Option 3: Load specific crate docs**
```bash
./crate_docs.sh fetch tokio     # Fetch specific crate
./crate_docs.sh embed           # Embed all fetched crates
```

### Manual Document Loading

Place `.md` or `.txt` files into `sample_docs/` under collection folders:

```bash
sample_docs/rust-book/
sample_docs/rust-docs/
sample_docs/crates/
```

Then run:

```bash
python embed_docs.py --dir ../sample_docs/your-collection --collection your-name --chunk-size 800 --overlap 100
```

All files will be embedded with intelligent chunking and stored in `./chroma_db`.

---

## 🔍 Querying the Knowledge Base

You can manually query docs using:

```bash
python query_docs.py "How do I deserialize JSON into an enum?"
```

This returns the most relevant chunks across all embedded docs.

---

## 💬 Running the Assistant (Chat Mode)

Launch the LLM shell with agent orchestration:

```bash
cargo run --bin llm_shell
```

You can now ask things like:

- "What’s the best way to build a CLI app in Rust?"
- "How do I initialize a `Vec` with a capacity?"
- "Give me an example using `serde` to serialize structs"

The system will:
- Search ChromaDB
- Route queries to agents (GitHub, crate, docs)
- Return helpful, idiomatic answers using a local LLM

---

## 🧱 Agent Overview

### GitHub Discovery Agent (Rust)
- Classifies queries into topics (e.g. `cli`, `web`)
- Uses GitHub API to find top crates
- Pulls README and returns as context

### Docs Agent
- Fetches documentation from `docs.rs` or stdlib/book
- Extracts relevant examples
- Adds results to Chroma

---

## 📦 Rust Project Structure

```
rust_llm_agent/
├── llm_shell/                    # Main chat interface with agent orchestration
├── rag_engine/                   # Core RAG functionality
├── rag_server/                   # HTTP server for RAG queries
├── agents/
│   ├── github_agent/            # GitHub repository discovery
│   ├── docs_agent/              # Documentation fetching
│   └── crate_agent/             # Crate.io integration
├── python_embedding/
│   ├── embed_docs.py            # Document embedding with chunking
│   ├── query_docs.py            # Manual query interface
│   ├── rustup_docs.sh           # Automated Rust docs extraction
│   ├── crate_docs.sh            # Crate documentation manager
│   └── sample_docs/             # Document storage (gitignored)
└── chroma_db/                   # Persistent vector database
```

---

## 🧪 Testing Examples

```bash
# Load Rust core documentation
cd python_embedding
./rustup_docs.sh

# Load popular crate docs
./crate_docs.sh all

# Query manually
python query_docs.py "How do I create a VecDeque?"

# Chat with LLM
cd ../llm_shell
cargo run
```

---

## 📎 TODO (next ideas)

- Allow agent results to include multiple sources
- Stream LLM responses with token buffering
- Add RAG-based fallback when LLM confidence is low
- Use embeddings to determine best agent (vs. keyword match)

---

## 🔧 Advanced Usage

### Crate Documentation Management

The `crate_docs.sh` script provides comprehensive crate documentation management:

```bash
# View all available commands
./crate_docs.sh

# Fetch specific crates
./crate_docs.sh fetch serde
./crate_docs.sh fetch tokio

# Fetch popular crates automatically
./crate_docs.sh popular

# Re-embed all fetched crates
./crate_docs.sh embed

# View collection statistics
./crate_docs.sh stats

# Clear and rebuild everything
./crate_docs.sh all
```

### Custom Documentation Sources

Install additional Rust docs locally:

```bash
rustup component add rust-docs
```

The `rustup_docs.sh` script will automatically extract and embed them with optimal chunking.

---

## License

MIT
