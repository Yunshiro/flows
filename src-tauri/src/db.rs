use rusqlite::{Connection, Result as SqliteResult};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(app_dir: &PathBuf) -> SqliteResult<Self> {
        std::fs::create_dir_all(app_dir).ok();
        let db_path = app_dir.join("flows.db");
        let conn = Connection::open(db_path)?;

        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

        Self::migrate(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn migrate(conn: &Connection) -> SqliteResult<()> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS todos (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                priority TEXT NOT NULL DEFAULT 'medium',
                tags TEXT NOT NULL DEFAULT '[]',
                done INTEGER NOT NULL DEFAULT 0,
                est_minutes INTEGER NOT NULL DEFAULT 30,
                actual_minutes INTEGER,
                date TEXT NOT NULL,
                created_at TEXT NOT NULL,
                completed_at TEXT,
                sort_order INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS daily_reviews (
                date TEXT PRIMARY KEY,
                content TEXT NOT NULL DEFAULT '',
                mood TEXT NOT NULL DEFAULT 'normal',
                linked_todo_ids TEXT NOT NULL DEFAULT '[]',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS llm_configs (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                base_url TEXT NOT NULL DEFAULT 'https://api.openai.com/v1',
                api_key TEXT NOT NULL DEFAULT '',
                model TEXT NOT NULL DEFAULT 'deepseek-chat',
                is_default INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            );"
        )?;
        Ok(())
    }
}
