use crate::db::Database;
use crate::models::*;
use chrono::Local;
use tauri::State;
use uuid::Uuid;

#[tauri::command(rename_all = "camelCase")]
pub fn list_todos(date: Option<String>, db: State<'_, Database>) -> Result<Vec<Todo>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let target = date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());

    let mut stmt = conn
        .prepare(
            "SELECT id, title, priority, tags, done, est_minutes, actual_minutes,
                    date, created_at, completed_at, sort_order
             FROM todos
             WHERE date = ?1
             ORDER BY done ASC, sort_order ASC, created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([&target], |row| {
            let tags_str: String = row.get(3)?;
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                priority: row.get(2)?,
                tags: serde_json::from_str(&tags_str).unwrap_or_default(),
                done: row.get::<_, i32>(4)? != 0,
                est_minutes: row.get(5)?,
                actual_minutes: row.get(6)?,
                date: row.get(7)?,
                created_at: row.get(8)?,
                completed_at: row.get(9)?,
                sort_order: row.get(10)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut todos: Vec<Todo> = Vec::new();
    for row in rows {
        todos.push(row.map_err(|e| e.to_string())?);
    }
    Ok(todos)
}

#[tauri::command(rename_all = "camelCase")]
pub fn add_todo(
    title: String,
    priority: Option<String>,
    est_minutes: Option<i32>,
    tags: Option<Vec<String>>,
    db: State<'_, Database>,
) -> Result<Todo, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Local::now();
    let id = Uuid::new_v4().to_string();
    let date = now.format("%Y-%m-%d").to_string();
    let created_at = now.format("%Y-%m-%dT%H:%M:%S").to_string();
    let p = priority.unwrap_or_else(|| "medium".into());
    let est = est_minutes.unwrap_or(30);
    let t = tags.unwrap_or_default();
    let tags_json = serde_json::to_string(&t).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO todos (id, title, priority, tags, done, est_minutes, date, created_at, sort_order)
         VALUES (?1, ?2, ?3, ?4, 0, ?5, ?6, ?7, 0)",
        rusqlite::params![id, title, p, tags_json, est, date, created_at],
    )
    .map_err(|e| e.to_string())?;

    Ok(Todo {
        id,
        title,
        priority: p,
        tags: t,
        done: false,
        est_minutes: est,
        actual_minutes: None,
        date,
        created_at,
        completed_at: None,
        sort_order: 0,
    })
}

#[tauri::command(rename_all = "camelCase")]
pub fn update_todo(id: String, changes: TodoChanges, db: State<'_, Database>) -> Result<Todo, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Local::now();

    if let Some(title) = &changes.title {
        conn.execute("UPDATE todos SET title = ?1 WHERE id = ?2", rusqlite::params![title, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(priority) = &changes.priority {
        conn.execute("UPDATE todos SET priority = ?1 WHERE id = ?2", rusqlite::params![priority, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(done) = changes.done {
        if done {
            let now_str = now.format("%Y-%m-%dT%H:%M:%S").to_string();
            conn.execute(
                "UPDATE todos SET done = 1, completed_at = ?1 WHERE id = ?2",
                rusqlite::params![now_str, id],
            )
            .map_err(|e| e.to_string())?;
        } else {
            conn.execute(
                "UPDATE todos SET done = 0, completed_at = NULL WHERE id = ?2",
                rusqlite::params![id],
            )
            .map_err(|e| e.to_string())?;
        }
    }
    if let Some(est) = changes.est_minutes {
        conn.execute("UPDATE todos SET est_minutes = ?1 WHERE id = ?2", rusqlite::params![est, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(actual) = changes.actual_minutes {
        conn.execute("UPDATE todos SET actual_minutes = ?1 WHERE id = ?2", rusqlite::params![actual, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(tags) = &changes.tags {
        let tags_json = serde_json::to_string(tags).map_err(|e| e.to_string())?;
        conn.execute("UPDATE todos SET tags = ?1 WHERE id = ?2", rusqlite::params![tags_json, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(order) = changes.sort_order {
        conn.execute("UPDATE todos SET sort_order = ?1 WHERE id = ?2", rusqlite::params![order, id])
            .map_err(|e| e.to_string())?;
    }

    let mut stmt = conn
        .prepare(
            "SELECT id, title, priority, tags, done, est_minutes, actual_minutes,
                    date, created_at, completed_at, sort_order FROM todos WHERE id = ?1",
        )
        .map_err(|e| e.to_string())?;
    let todo = stmt
        .query_row([&id], |row| {
            let tags_str: String = row.get(3)?;
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                priority: row.get(2)?,
                tags: serde_json::from_str(&tags_str).unwrap_or_default(),
                done: row.get::<_, i32>(4)? != 0,
                est_minutes: row.get(5)?,
                actual_minutes: row.get(6)?,
                date: row.get(7)?,
                created_at: row.get(8)?,
                completed_at: row.get(9)?,
                sort_order: row.get(10)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(todo)
}

#[tauri::command(rename_all = "camelCase")]
pub fn toggle_todo(id: String, db: State<'_, Database>) -> Result<Todo, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let done: bool = conn
        .query_row("SELECT done FROM todos WHERE id = ?1", [&id], |row| {
            row.get::<_, i32>(0).map(|v| v != 0)
        })
        .map_err(|e| format!("Todo not found: {}", e))?;

    let now = Local::now();
    if !done {
        let now_str = now.format("%Y-%m-%dT%H:%M:%S").to_string();
        conn.execute(
            "UPDATE todos SET done = 1, completed_at = ?1 WHERE id = ?2",
            rusqlite::params![now_str, id],
        )
        .map_err(|e| e.to_string())?;
    } else {
        conn.execute(
            "UPDATE todos SET done = 0, completed_at = NULL WHERE id = ?1",
            [&id],
        )
        .map_err(|e| e.to_string())?;
    }

    let mut stmt = conn
        .prepare(
            "SELECT id, title, priority, tags, done, est_minutes, actual_minutes,
                    date, created_at, completed_at, sort_order FROM todos WHERE id = ?1",
        )
        .map_err(|e| e.to_string())?;
    stmt.query_row([&id], |row| {
        let tags_str: String = row.get(3)?;
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            priority: row.get(2)?,
            tags: serde_json::from_str(&tags_str).unwrap_or_default(),
            done: row.get::<_, i32>(4)? != 0,
            est_minutes: row.get(5)?,
            actual_minutes: row.get(6)?,
            date: row.get(7)?,
            created_at: row.get(8)?,
            completed_at: row.get(9)?,
            sort_order: row.get(10)?,
        })
    })
    .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "camelCase")]
pub fn delete_todo(id: String, db: State<'_, Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM todos WHERE id = ?1", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command(rename_all = "camelCase")]
pub fn reorder_todos(ids: Vec<String>, db: State<'_, Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    for (i, id) in ids.iter().enumerate() {
        conn.execute(
            "UPDATE todos SET sort_order = ?1 WHERE id = ?2",
            rusqlite::params![i as i32, id],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
