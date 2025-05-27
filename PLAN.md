# Rust LLM Copilot - Full Setup Guide (Local)

This guide walks you through setting up a local, offline-friendly Rust assistant that can:

- Answer Rust programming questions
- Suggest crates
- Provide idiomatic code examples
- Learn from crate docs, GitHub code, and Rust Book content
- Use agents, RAG, and a local LLM model

---

## ✅ Prerequisites

Install these tools:

### System Tools
```bash
brew install python3 git
pip3 install virtualenv
```

### LLM Runtime (Choose One)
- [Ollama](https://ollama.com): Easy model runner
```bash
brew install ollama
ollama pull phi  # or mistral, llama3, etc.
```

### Rust Toolchain
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cd rust_llm_agent/
cargo build --release --workspace
```

---

## Directory Structure

```
rust_llm_agent/
├── agents/
│   ├── crate_agent/
│   ├── github_agent/         # (stub)
│   └── docs_agent/           # (stub)
├── llm_shell/
│   └── main.rs               # CLI that routes + builds prompt
├── python_embedding/
│   ├── embed_docs.py         # Index .md files into vector DB
│   ├── query_docs.py         # Search vector DB with question
│   ├── fetch_docsrs.py       # Scrape docs.rs and save markdown
│   └── requirements.txt
├── sample_docs/
│   └── serde_json.md         # Collected crate docs
├── PLAN.md
```

---

## 1. Set Up Python Environment

```bash
cd rust_llm_agent/python_embedding
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

---

## 2. Fetch and Embed Docs

```bash
# Fetch docs from docs.rs
python fetch_docsrs.py serde_json
python fetch_docsrs.py reqwest

# Embed them into the vector DB
python embed_docs.py
```

---

## 3. Build Rust Workspace

```bash
cd rust_llm_agent/
cargo build --release --workspace
```

---

## 4. Run the Shell

```bash
cd llm_shell
cargo run --release
```

Example prompt:
```
What’s the idiomatic way to stream JSON using reqwest and tokio?
```

The shell will:
- Route the question to an agent (e.g. `crate_agent`, `github_agent`)
- Fall back to the vector DB (RAG) if no agent matches
- Combine everything into a prompt
- Pass it to `ollama run phi`
- Print the response

---

## 5. Add More Knowledge

Any time you want to expand the assistant’s knowledge:
```bash
python fetch_docsrs.py <crate_name>
python embed_docs.py
```

---

## 6. (Optional) Add More Agents

Each agent:
- Takes a natural language query as input
- Outputs JSON

Place agents in:
```
agents/github_agent/
agents/docs_agent/
```

Build them with:
```bash
cd agents/github_agent
cargo build --release
```

---

## Next Steps

- Add LLM-based router instead of rule-based logic
- Expand embedding to blog posts or GitHub examples
- Track past user interactions in a local history file

---

You're now ready to run a private, smart, Rust-native AI system on your own machine.

