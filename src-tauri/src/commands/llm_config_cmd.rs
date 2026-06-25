use crate::db::Database;
use crate::models::LlmConfig;
use chrono::Local;
use tauri::State;
use uuid::Uuid;

#[tauri::command(rename_all = "camelCase")]
pub fn list_llm_configs(db: State<'_, Database>) -> Result<Vec<LlmConfig>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, name, base_url, api_key, model, is_default, created_at
             FROM llm_configs ORDER BY is_default DESC, created_at DESC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(LlmConfig {
                id: row.get(0)?,
                name: row.get(1)?,
                base_url: row.get(2)?,
                api_key: row.get(3)?,
                model: row.get(4)?,
                is_default: row.get::<_, i32>(5)? != 0,
                created_at: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut configs = Vec::new();
    for row in rows {
        configs.push(row.map_err(|e| e.to_string())?);
    }
    Ok(configs)
}

#[tauri::command(rename_all = "camelCase")]
pub fn save_llm_config(
    id: Option<String>,
    name: String,
    base_url: String,
    api_key: String,
    model: String,
    is_default: bool,
    db: State<'_, Database>,
) -> Result<LlmConfig, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let config_id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM llm_configs WHERE id = ?1",
            [&config_id],
            |row| row.get::<_, i32>(0).map(|c| c > 0),
        )
        .unwrap_or(false);

    if is_default {
        conn.execute("UPDATE llm_configs SET is_default = 0", [])
            .map_err(|e| e.to_string())?;
    }

    if exists {
        conn.execute(
            "UPDATE llm_configs SET name=?1, base_url=?2, api_key=?3, model=?4, is_default=?5 WHERE id=?6",
            rusqlite::params![name, base_url, api_key, model, is_default as i32, config_id],
        )
        .map_err(|e| e.to_string())?;
    } else {
        conn.execute(
            "INSERT INTO llm_configs (id, name, base_url, api_key, model, is_default, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![config_id, name, base_url, api_key, model, is_default as i32, now],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(LlmConfig {
        id: config_id,
        name,
        base_url,
        api_key,
        model,
        is_default,
        created_at: now,
    })
}

#[tauri::command(rename_all = "camelCase")]
pub fn delete_llm_config(id: String, db: State<'_, Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM llm_configs WHERE id = ?1", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command(rename_all = "camelCase")]
pub fn get_llm_config(id: String, db: State<'_, Database>) -> Result<LlmConfig, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.query_row(
        "SELECT id, name, base_url, api_key, model, is_default, created_at FROM llm_configs WHERE id = ?1",
        [&id],
        |row| {
            Ok(LlmConfig {
                id: row.get(0)?,
                name: row.get(1)?,
                base_url: row.get(2)?,
                api_key: row.get(3)?,
                model: row.get(4)?,
                is_default: row.get::<_, i32>(5)? != 0,
                created_at: row.get(6)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}
