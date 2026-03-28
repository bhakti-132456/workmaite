use rusqlite::{Connection, Result as SqlResult};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(app_data_dir: PathBuf) -> SqlResult<Self> {
        std::fs::create_dir_all(&app_data_dir).ok();
        let db_path = app_data_dir.join("workmaite.db");
        let conn = Connection::open(&db_path)?;

        // Enable WAL mode for better concurrent performance
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

        // Run migrations
        Self::migrate(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn migrate(conn: &Connection) -> SqlResult<()> {
        conn.execute_batch(
            "
            -- Core messages table
            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                personality TEXT,
                created_at TEXT DEFAULT (datetime('now'))
            );

            -- Context Ledger: persistent KV memory
            CREATE TABLE IF NOT EXISTS context_ledger (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                category TEXT DEFAULT 'general',
                updated_at TEXT DEFAULT (datetime('now'))
            );

            -- Document store for RAG
            CREATE TABLE IF NOT EXISTS documents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_path TEXT NOT NULL,
                title TEXT,
                content TEXT NOT NULL,
                chunk_index INTEGER DEFAULT 0,
                chunk_total INTEGER DEFAULT 1,
                created_at TEXT DEFAULT (datetime('now'))
            );

            -- Focus sessions log
            CREATE TABLE IF NOT EXISTS focus_sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                started_at TEXT NOT NULL,
                ended_at TEXT,
                duration_mins INTEGER NOT NULL,
                personality TEXT,
                check_ins_completed INTEGER DEFAULT 0,
                notes TEXT
            );

            /* 
            -- Vector store for Memory Vault RAG
            -- Requires sqlite-vec extension (vec0) 
            -- Using 384 dimensions for all-MiniLM-L6-v2
            CREATE VIRTUAL TABLE IF NOT EXISTS vec_memory USING vec0(
                id INTEGER PRIMARY KEY,
                embedding FLOAT[384]
            );

            -- Linking table between vector IDs and document chunks
            CREATE TABLE IF NOT EXISTS document_vault (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                vec_id INTEGER,
                source_path TEXT NOT NULL,
                content TEXT NOT NULL,
                metadata TEXT,
                created_at TEXT DEFAULT (datetime('now')),
                FOREIGN KEY(vec_id) REFERENCES vec_memory(id)
            );
            */
            ",
        )?;
        Ok(())
    }
}
