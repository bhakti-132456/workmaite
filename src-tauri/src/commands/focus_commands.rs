use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use crate::db::Database;

// Keep track of active focus session
pub struct FocusState {
    pub active: Arc<AsyncMutex<bool>>,
}

#[tauri::command]
pub async fn start_focus_session(
    state: State<'_, FocusState>,
    db: State<'_, Database>,
    duration_mins: u32,
    personality: Option<String>,
) -> Result<i64, String> {
    let mut active = state.active.lock().await;
    *active = true;

    let p = personality.unwrap_or_else(|| "guide".to_string());
    
    // Record start in DB
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO focus_sessions (started_at, duration_mins, personality) 
         VALUES (datetime('now'), ?1, ?2)",
        rusqlite::params![duration_mins, p],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub async fn end_focus_session(
    state: State<'_, FocusState>,
    db: State<'_, Database>,
    session_id: i64,
    notes: Option<String>,
) -> Result<(), String> {
    let mut active = state.active.lock().await;
    *active = false;

    let conn = db.conn.lock().unwrap();
    conn.execute(
        "UPDATE focus_sessions 
         SET ended_at = datetime('now'), notes = ?1 
         WHERE id = ?2",
        rusqlite::params![notes, session_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
