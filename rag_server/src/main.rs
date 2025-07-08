use chromadb::client::{ChromaAuthMethod, ChromaClient, ChromaClientOptions, ChromaTokenHeader};
use chromadb::collection::{ChromaCollection, GetOptions, GetResult, QueryOptions};
use serde_json::json;

use tokio;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let client = ChromaClient::new(ChromaClientOptions {
        url: Some("http://localhost:8000".into()),
        ..Default::default()
    })
    .await
    .unwrap();

    let collection: ChromaCollection = client
        .get_or_create_collection("rust-docs", None)
        .await
        .unwrap();

    let query_embeddings = vec![
        // Your embedding vector here - typically 384 or 768 dimensions
        // This is just an example - you need actual embeddings
        vec![0.1, 0.2, 0.3], // Replace with real embedding vector
    ];

    let qo = QueryOptions {
        query_embeddings: Some(query_embeddings),
        query_texts: Some(vec!["A basic struct"]),
        n_results: Some(2),
        where_document: None,
        where_metadata: None,
        include: None,
    };
    let data = collection.query(qo, Some(query_embeddings)).await;
    println!("Query result: {:#?}", data);
    Ok(())
}
