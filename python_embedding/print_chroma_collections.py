import chromadb

client = chromadb.PersistentClient(path="./chroma_db")  # or chromadb.HttpClient()
collections = client.list_collections()
print(collections)

