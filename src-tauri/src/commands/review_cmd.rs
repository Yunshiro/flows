use crate::db::Database;
use crate::models::*;
use chrono::{Datelike, Local};
use tauri::State;

#[tauri::command(rename_all = "camelCase")]
pub fn list_reviews(
    year: Option<i32>,
    month: Option<u32>,
    db: State<'_, Database>,
) -> Result<Vec<DailyReview>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Local::now();
    let y = year.unwrap_or(now.year());
    let m = month.unwrap_or(now.month());
    let prefix = format!("{}-{:02}", y, m);

    let mut stmt = conn
        .prepare(
            "SELECT date, content, mood, linked_todo_ids, created_at, updated_at
             FROM daily_reviews
             WHERE date LIKE ?1 || '%'
             ORDER BY date DESC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([&prefix], |row| {
            let linked_str: String = row.get(3)?;
            Ok(DailyReview {
                date: row.get(0)?,
                content: row.get(1)?,
                mood: row.get(2)?,
                linked_todo_ids: serde_json::from_str(&linked_str).unwrap_or_default(),
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut reviews = Vec::new();
    for row in rows {
        reviews.push(row.map_err(|e| e.to_string())?);
    }
    Ok(reviews)
}

#[tauri::command(rename_all = "camelCase")]
pub fn get_review(date: String, db: State<'_, Database>) -> Result<Option<DailyReview>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT date, content, mood, linked_todo_ids, created_at, updated_at
             FROM daily_reviews WHERE date = ?1",
        )
        .map_err(|e| e.to_string())?;

    let result = stmt.query_row([&date], |row| {
        let linked_str: String = row.get(3)?;
        Ok(DailyReview {
            date: row.get(0)?,
            content: row.get(1)?,
            mood: row.get(2)?,
            linked_todo_ids: serde_json::from_str(&linked_str).unwrap_or_default(),
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    });

    match result {
        Ok(review) => Ok(Some(review)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command(rename_all = "camelCase")]
pub fn save_review(
    content: String,
    mood: Option<String>,
    linked_todo_ids: Option<Vec<String>>,
    db: State<'_, Database>,
) -> Result<DailyReview, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Local::now();
    let date = now.format("%Y-%m-%d").to_string();
    let now_str = now.format("%Y-%m-%dT%H:%M:%S").to_string();
    let m = mood.unwrap_or_else(|| "normal".into());
    let linked = linked_todo_ids.unwrap_or_default();
    let linked_json = serde_json::to_string(&linked).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO daily_reviews (date, content, mood, linked_todo_ids, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?5)
         ON CONFLICT(date) DO UPDATE SET
           content = excluded.content,
           mood = excluded.mood,
           linked_todo_ids = excluded.linked_todo_ids,
           updated_at = excluded.updated_at",
        rusqlite::params![date, content, m, linked_json, now_str],
    )
    .map_err(|e| e.to_string())?;

    Ok(DailyReview {
        date,
        content,
        mood: m,
        linked_todo_ids: linked,
        created_at: now_str.clone(),
        updated_at: now_str,
    })
}
