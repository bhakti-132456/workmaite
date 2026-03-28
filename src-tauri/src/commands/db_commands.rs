use serde::Serialize;
use tauri::State;

use crate::db::Database;

#[derive(Serialize)]
pub struct MessageResponse {
    pub id: i64,
    pub role: String,
    pub content: String,
    pub personality: Option<String>,
    pub created_at: String,
}

#[tauri::command]
pub fn save_message(
    db: State<'_, Database>,
    role: String,
    content: String,
    personality: Option<String>,
) -> Result<i64, String> {
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO messages (role, content, personality) VALUES (?1, ?2, ?3)",
        rusqlite::params![role, content, personality],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn get_messages(db: State<'_, Database>, limit: i64) -> Result<Vec<MessageResponse>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, role, content, personality, created_at FROM messages ORDER BY id DESC LIMIT ?1")
        .map_err(|e| e.to_string())?;

    let messages_iter = stmt
        .query_map([limit], |row| {
            Ok(MessageResponse {
                id: row.get(0)?,
                role: row.get(1)?,
                content: row.get(2)?,
                personality: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut messages: Vec<MessageResponse> = messages_iter.filter_map(Result::ok).collect();
    // Reverse so chronologically ordered
    messages.reverse();
    Ok(messages)
}

#[tauri::command]
pub fn set_context(
    db: State<'_, Database>,
    key: String,
    value: String,
    category: Option<String>,
) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    let cat = category.unwrap_or_else(|| "general".to_string());
    
    conn.execute(
        "INSERT INTO context_ledger (key, value, category, updated_at) 
         VALUES (?1, ?2, ?3, datetime('now'))
         ON CONFLICT(key) DO UPDATE SET 
         value=excluded.value, category=excluded.category, updated_at=datetime('now')",
        rusqlite::params![key, value, cat],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_context(db: State<'_, Database>, key: String) -> Result<Option<String>, String> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT value FROM context_ledger WHERE key = ?1")
        .map_err(|e| e.to_string())?;

    let result = stmt.query_row([key], |row| row.get(0));

    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}
