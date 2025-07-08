#!/bin/bash

# Activate virtual environment
source venv/bin/activate

# Configuration
CRATES_DIR="../sample_docs/crates"
COLLECTION_NAME="crates"
CHUNK_SIZE=800
OVERLAP=100

# Create crates directory if it doesn't exist
mkdir -p "$CRATES_DIR"

# Function to fetch and embed a single crate
fetch_and_embed_crate() {
    local crate_name="$1"
    echo "📦 Fetching documentation for $crate_name..."
    
    if python3 fetch_docsrs.py "$crate_name"; then
        echo "✅ Successfully fetched $crate_name"
    else
        echo "❌ Failed to fetch $crate_name"
        return 1
    fi
}

# Function to embed all crates
embed_all_crates() {
    echo "🔄 Embedding all crate documentation with chunking..."
    
    # Count markdown files in crates directory
    local file_count=$(find "$CRATES_DIR" -name "*.md" | wc -l)
    
    if [ "$file_count" -eq 0 ]; then
        echo "⚠️  No crate documentation found in $CRATES_DIR"
        echo "   Use: $0 fetch <crate_name> to fetch some crates first"
        return 1
    fi
    
    python3 embed_docs.py \
        --dir "$CRATES_DIR" \
        --collection "$COLLECTION_NAME" \
        --chunk-size "$CHUNK_SIZE" \
        --overlap "$OVERLAP"
    
    echo "✅ Embedded $file_count crate documentation files"
}

# Function to clear crate collection
clear_crates() {
    echo "🗑️  Clearing crates collection..."
    python3 -c "
import chromadb
client = chromadb.PersistentClient(path='./chroma_db')
try:
    client.delete_collection('$COLLECTION_NAME')
    print('✅ Cleared crates collection')
except:
    print('ℹ️  Crates collection was already empty')
"
}

# Function to list available crates
list_crates() {
    echo "📋 Available crate documentation:"
    if [ -d "$CRATES_DIR" ]; then
        find "$CRATES_DIR" -name "*.md" -exec basename {} .md \; | sort
    else
        echo "   No crates found"
    fi
}

# Function to show collection stats
show_stats() {
    echo "📊 Crates collection statistics:"
    python3 -c "
import chromadb
client = chromadb.PersistentClient(path='./chroma_db')
try:
    collection = client.get_collection('$COLLECTION_NAME')
    count = collection.count()
    print(f'   Total chunks: {count}')
    if count > 0:
        # Get sample metadata to show crate names
        sample = collection.peek(limit=10)
        if sample['metadatas']:
            crates = set(m.get('title', 'unknown') for m in sample['metadatas'])
            print(f'   Sample crates: {', '.join(list(crates)[:5])}')
except:
    print('   Collection not found or empty')
"
}

# Function to fetch popular Rust crates
fetch_popular() {
    echo "📦 Fetching popular Rust crates..."
    
    local popular_crates=(
        "serde"
        "tokio"
        "clap"
        "reqwest"
        "anyhow"
        "thiserror"
        "uuid"
        "chrono"
        "regex"
        "log"
    )
    
    local success_count=0
    for crate in "${popular_crates[@]}"; do
        if fetch_and_embed_crate "$crate"; then
            ((success_count++))
        fi
        sleep 1  # Be nice to docs.rs
    done
    
    echo "✅ Successfully fetched $success_count/${#popular_crates[@]} popular crates"
}

# Main command handling
case "$1" in
    "fetch")
        if [ -z "$2" ]; then
            echo "Usage: $0 fetch <crate_name>"
            exit 1
        fi
        fetch_and_embed_crate "$2"
        ;;
    "popular")
        fetch_popular
        ;;
    "embed")
        embed_all_crates
        ;;
    "clear")
        clear_crates
        ;;
    "list")
        list_crates
        ;;
    "stats")
        show_stats
        ;;
    "all")
        echo "🚀 Full crate documentation workflow..."
        clear_crates
        fetch_popular
        embed_all_crates
        show_stats
        ;;
    *)
        echo "Crate Documentation Manager"
        echo ""
        echo "Usage: $0 <command> [args]"
        echo ""
        echo "Commands:"
        echo "  fetch <crate>  - Fetch documentation for a specific crate"
        echo "  popular        - Fetch popular Rust crates"
        echo "  embed          - Embed all fetched crates with chunking"
        echo "  clear          - Clear the crates collection"
        echo "  list           - List available crate documentation"
        echo "  stats          - Show collection statistics"
        echo "  all            - Run complete workflow (clear, fetch popular, embed)"
        echo ""
        echo "Examples:"
        echo "  $0 fetch tokio"
        echo "  $0 popular"
        echo "  $0 embed"
        echo "  $0 all"
        ;;
esac
