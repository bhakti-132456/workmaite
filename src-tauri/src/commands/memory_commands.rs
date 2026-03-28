#[tauri::command]
pub fn ingest_document(path: String) -> Result<String, String> {
    // In a full implementation, this reads the file,
    // chunks it, calls an embedding model, and inserts into doc_vectors.
    // For this prototype, we just acknowledge the drop.
    Ok(format!("Ingested document: {}", path))
}
