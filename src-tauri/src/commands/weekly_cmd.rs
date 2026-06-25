use crate::db::Database;
use chrono::{Datelike, Local};
use futures_util::StreamExt;
use serde::Serialize;
use tauri::{Emitter, State};

#[derive(Serialize, Clone)]
struct StreamChunk {
    content: String,
    done: bool,
    error: Option<String>,
}

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    stream: bool,
}

fn read_setting(db: &Database, key: &str) -> Option<String> {
    let conn = db.conn.lock().ok()?;
    conn.query_row("SELECT value FROM settings WHERE key = ?1", [key], |row| row.get(0)).ok()
}

fn get_settings_json(db: &Database) -> Option<serde_json::Value> {
    let json_str = read_setting(db, "app_settings")?;
    serde_json::from_str(&json_str).ok()
}

fn build_prompt(db: &Database) -> Result<(String, String, String), String> {
    let now = Local::now();
    let weekday = now.weekday().num_days_from_monday();
    let monday = now.naive_local().date() - chrono::Duration::days(weekday as i64);
    let sunday = monday + chrono::Duration::days(6);
    let week_start = monday.format("%Y-%m-%d").to_string();
    let week_end = sunday.format("%Y-%m-%d").to_string();

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut todo_stmt = conn.prepare(
        "SELECT COUNT(*), SUM(CASE WHEN done = 1 THEN 1 ELSE 0 END)
         FROM todos WHERE date >= ?1 AND date <= ?2",
    ).map_err(|e| e.to_string())?;
    let (total_todos, completed_todos): (i32, i32) = todo_stmt
        .query_row(rusqlite::params![&week_start, &week_end], |row| Ok((row.get(0)?, row.get(1)?)))
        .map_err(|e| e.to_string())?;
    let completion_rate = if total_todos > 0 { (completed_todos as f64 / total_todos as f64 * 100.0) as i32 } else { 0 };

    let mut titles_stmt = conn.prepare(
        "SELECT title, done FROM todos WHERE date >= ?1 AND date <= ?2 ORDER BY date ASC, sort_order ASC",
    ).map_err(|e| e.to_string())?;
    let todos_data: Vec<(String, bool)> = titles_stmt
        .query_map(rusqlite::params![&week_start, &week_end], |row| Ok((row.get(0)?, row.get::<_, i32>(1)? != 0)))
        .map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();

    let mut review_stmt = conn.prepare(
        "SELECT date, content, mood FROM daily_reviews WHERE date >= ?1 AND date <= ?2 ORDER BY date ASC",
    ).map_err(|e| e.to_string())?;
    let reviews: Vec<(String, String, String)> = review_stmt
        .query_map(rusqlite::params![&week_start, &week_end], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();

    let mut context = format!(
        "## 本周 ({}-{}) 数据\n\n待办完成率: {}% ({}/{})\n\n### 待办清单\n",
        week_start, week_end, completion_rate, completed_todos, total_todos
    );
    for (title, done) in &todos_data {
        context.push_str(&format!("- [{}] {}\n", if *done { "x" } else { " " }, title));
    }
    if !reviews.is_empty() {
        context.push_str("\n### 每日复盘\n");
        for (date, content, mood) in &reviews {
            let mood_label = match mood.as_str() { "productive" => "高效", "slacking" => "低效", _ => "一般" };
            let snippet: String = content.chars().filter(|c| *c != '#' && *c != '*').take(200).collect();
            context.push_str(&format!("- {} ({}): {}\n", date, mood_label, snippet));
        }
    }

    let prompt = format!(
        "你是时间管理助手。根据以下本周数据，直接输出一份 Markdown 格式的周报。\n\
         规则：\n\
         - 直接以 ## 标题开头，不要任何前缀语、问候语或解释\n\
         - 不要写\"好的\"、\"以下是\"、\"根据\"这类开头\n\
         - 结构：## 本周概览 / ## 主要成果 / ## 待改进 / ## 下周建议\n\
         - 语言简洁专业，中文\n\n{}",
        context
    );
    Ok((week_start, week_end, prompt))
}

#[tauri::command(rename_all = "camelCase")]
pub async fn generate_weekly_summary_stream(
    app: tauri::AppHandle,
    db: State<'_, Database>,
    config_id: Option<String>,
) -> Result<String, String> {
    let (_week_start, _week_end, prompt) = build_prompt(&db)?;

    // Read config from llm_configs table
    let (base_url, api_key, model) = if let Some(cid) = config_id {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        conn.query_row(
            "SELECT base_url, api_key, model FROM llm_configs WHERE id = ?1",
            [&cid],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?)),
        )
        .map_err(|e| format!("配置未找到: {}", e))?
    } else {
        // Fallback: try settings
        let settings_json = get_settings_json(&db);
        let url = settings_json.as_ref().and_then(|s| s.get("llmBaseUrl")).and_then(|v| v.as_str()).unwrap_or("https://api.openai.com/v1").to_string();
        let key = settings_json.as_ref().and_then(|s| s.get("llmApiKey")).and_then(|v| v.as_str()).unwrap_or("").to_string();
        let m = settings_json.as_ref().and_then(|s| s.get("llmModel")).and_then(|v| v.as_str()).unwrap_or("deepseek-chat").to_string();
        (url, key, m)
    };

    if api_key.is_empty() {
        let chunk = StreamChunk { content: "未配置 API Key，请在设置中配置。".into(), done: true, error: None };
        let _ = app.emit("llm-chunk", chunk);
        return Ok("未配置 API Key".into());
    }

    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let body = ChatRequest {
        model: model.clone(),
        max_tokens: 1500,
        messages: vec![ChatMessage { role: "user".into(), content: prompt }],
        stream: true,
    };

    let app_clone = app.clone();

    // Spawn background task for streaming
    tokio::spawn(async move {
        let client = reqwest::Client::new();
        let result = stream_llm_response(&client, &url, &api_key, &body, &app_clone).await;
        match result {
            Ok(_full) => {
                let _ = app_clone.emit("llm-chunk", StreamChunk {
                    content: String::new(),
                    done: true,
                    error: None,
                });
            }
            Err(e) => {
                let _ = app_clone.emit("llm-chunk", StreamChunk {
                    content: String::new(),
                    done: true,
                    error: Some(format!("LLM 调用失败: {}", e)),
                });
            }
        }
    });

    Ok("started".into())
}

async fn stream_llm_response(
    client: &reqwest::Client,
    url: &str,
    api_key: &str,
    body: &ChatRequest,
    app: &tauri::AppHandle,
) -> Result<String, String> {
    let resp = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("content-type", "application/json")
        .json(body)
        .send()
        .await
        .map_err(|e| format!("HTTP 错误: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("API 错误 {}: {}", status.as_u16(), text));
    }

    let mut stream = resp.bytes_stream();
    let mut buffer = String::new();
    let mut full_content = String::new();

    while let Some(chunk) = stream.next().await {
        let bytes = chunk.map_err(|e| format!("流错误: {}", e))?;
        let text = String::from_utf8_lossy(&bytes);
        buffer.push_str(&text);

        // Process complete SSE lines
        while let Some(pos) = buffer.find('\n') {
            let line = buffer[..pos].trim().to_string();
            buffer = buffer[pos + 1..].to_string();

            if line.is_empty() || line.starts_with(':') { continue; }
            if line.starts_with("data: ") {
                let data = &line[6..];
                if data == "[DONE]" { break; }
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                    // Only emit actual content, skip reasoning/thinking tokens
                    if let Some(delta) = json["choices"][0]["delta"]["content"].as_str() {
                        if !delta.is_empty() {
                            full_content.push_str(delta);
                            let _ = app.emit("llm-chunk", StreamChunk {
                                content: delta.to_string(),
                                done: false,
                                error: None,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(full_content)
}

#[tauri::command(rename_all = "camelCase")]
pub fn test_llm_connection(
    base_url: String,
    api_key: String,
    model: String,
) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let body = ChatRequest {
        model: model.to_string(),
        max_tokens: 50,
        messages: vec![ChatMessage { role: "user".into(), content: "回复'连接成功'".into() }],
        stream: false,
    };

    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let resp = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .map_err(|e| format!("HTTP 错误: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        return Err(format!("API 错误 {}: {}", status.as_u16(), text));
    }

    let json: serde_json::Value = resp.json().map_err(|e| format!("解析失败: {}", e))?;
    let content = json["choices"]
        .as_array()
        .and_then(|arr: &Vec<serde_json::Value>| arr.first())
        .and_then(|c| c["message"]["content"].as_str())
        .unwrap_or("(空响应)");

    Ok(format!("连接成功！模型回复: {}", content))
}

#[tauri::command(rename_all = "camelCase")]
pub fn generate_weekly_summary(db: State<'_, Database>) -> Result<String, String> {
    // Non-streaming fallback — builds a local summary without API call
    let (week_start, week_end, _prompt) = build_prompt(&db)?;

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT COUNT(*), SUM(CASE WHEN done = 1 THEN 1 ELSE 0 END)
         FROM todos WHERE date >= ?1 AND date <= ?2",
    ).map_err(|e| e.to_string())?;
    let (total_todos, completed_todos): (i32, i32) = stmt
        .query_row(rusqlite::params![&week_start, &week_end], |row| Ok((row.get(0)?, row.get(1)?)))
        .map_err(|e| e.to_string())?;
    let completion_rate = if total_todos > 0 { (completed_todos as f64 / total_todos as f64 * 100.0) as i32 } else { 0 };

    let mut titles_stmt = conn.prepare(
        "SELECT title, done FROM todos WHERE date >= ?1 AND date <= ?2 AND done = 1 ORDER BY date ASC",
    ).map_err(|e| e.to_string())?;
    let done_titles: Vec<String> = titles_stmt
        .query_map(rusqlite::params![&week_start, &week_end], |row| Ok(row.get(0)?))
        .map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();

    let mut review_stmt = conn.prepare(
        "SELECT date, content, mood FROM daily_reviews WHERE date >= ?1 AND date <= ?2 ORDER BY date ASC",
    ).map_err(|e| e.to_string())?;
    let reviews: Vec<(String, String, String)> = review_stmt
        .query_map(rusqlite::params![&week_start, &week_end], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();

    let mut lines = vec![
        format!("## 本周总结 ({}—{})", week_start, week_end),
        String::new(),
        format!("### 完成情况"),
        format!("- 完成率: {}% ({}/{})", completion_rate, completed_todos, total_todos),
    ];
    if !done_titles.is_empty() {
        lines.push(String::new());
        lines.push(format!("### 完成事项 ({} 项)", done_titles.len()));
        for t in &done_titles { lines.push(format!("- {}", t)); }
    }
    if !reviews.is_empty() {
        lines.push(String::new());
        lines.push("### 每日复盘".to_string());
        for (date, content, mood) in &reviews {
            let icon = match mood.as_str() { "productive" => ">", "slacking" => "<", _ => "=" };
            let s: String = content.chars().filter(|c| *c != '#' && *c != '*').take(100).collect();
            lines.push(format!("- {} {}: {}", icon, date, s));
        }
    }
    lines.push(String::new());
    lines.push("---".to_string());
    lines.push("*由 Flows 生成（离线模式，配置 API 后可启用 AI 流式生成）*".to_string());

    Ok(lines.join("\n"))
}
