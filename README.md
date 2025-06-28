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
- Local docs via `rustup`
- Crate docs via scraping
- Rust book or stdlib
- GitHub crate READMEs

### ✅ Embed Docs

Place `.md` or `.txt` files into `python_embedding/sample_docs` under subfolders like:

```bash
sample_docs/rust-book/
sample_docs/std/
sample_docs/crate-clap/
```

Then run:

```bash
python embed_docs.py
```

All files will be embedded and stored in `./chroma_db`.

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
rust_llm_agent_updated/
├── llm_shell/           # Main loop for chat + agent orchestration
├── agents/
│   └── github_discovery_agent/  # Rust-based agent for GitHub lookups
├── python_embedding/
│   ├── embed_docs.py
│   ├── query_docs.py
│   └── sample_docs/
└── chroma_db/           # Persistent vector DB
```

---

## 🧪 Testing Examples

```bash
# Embed new docs
python embed_docs.py

# Query manually
python query_docs.py "How do I create a VecDeque?"

# Chat with LLM
cargo run --bin llm_shell
```

---

## 📎 TODO (next ideas)

- Allow agent results to include multiple sources
- Stream LLM responses with token buffering
- Add RAG-based fallback when LLM confidence is low
- Use embeddings to determine best agent (vs. keyword match)

---

## 💡 Tip

Install Rust docs and books locally:

```bash
rustup component add rust-docs
```

Then extract them into `sample_docs/` for embedding.

---

## License

MIT
