use crate::db::Database;
use crate::models::AppSettings;
use tauri::State;

const SETTINGS_KEY: &str = "app_settings";

#[tauri::command(rename_all = "camelCase")]
pub fn get_settings(db: State<'_, Database>) -> Result<AppSettings, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let result: Result<String, _> = conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        [SETTINGS_KEY],
        |row| row.get(0),
    );

    match result {
        Ok(json) => serde_json::from_str(&json).map_err(|e| e.to_string()),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(AppSettings::default()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command(rename_all = "camelCase")]
pub fn save_settings(settings: AppSettings, db: State<'_, Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let json = serde_json::to_string(&settings).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        rusqlite::params![SETTINGS_KEY, json],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
