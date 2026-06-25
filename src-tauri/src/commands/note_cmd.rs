use crate::models::*;
use chrono::Local;
use std::fs;
use std::path::PathBuf;

#[tauri::command(rename_all = "camelCase")]
pub fn list_notes(dir: Option<String>) -> Result<Vec<NoteTreeNode>, String> {
    let notes_dir = get_notes_dir()?;
    let target = if let Some(d) = dir {
        notes_dir.join(d)
    } else {
        notes_dir.clone()
    };

    build_tree(&target, &notes_dir)
}

fn get_notes_dir() -> Result<PathBuf, String> {
    let dir = directories::UserDirs::new()
        .map(|d| d.home_dir().join("flows-notes"))
        .ok_or("Cannot find home directory")?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn build_tree(dir: &PathBuf, root: &PathBuf) -> Result<Vec<NoteTreeNode>, String> {
    let mut children = Vec::new();
    if !dir.exists() {
        return Ok(children);
    }
    let entries = fs::read_dir(dir).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }
        let rel_path = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();

        if path.is_dir() {
            children.push(NoteTreeNode {
                name,
                path: rel_path,
                is_dir: true,
                children: build_tree(&path, root)?,
            });
        } else if name.ends_with(".md") || name.ends_with(".txt") {
            children.push(NoteTreeNode {
                name: name.trim_end_matches(".md").trim_end_matches(".txt").to_string(),
                path: rel_path,
                is_dir: false,
                children: vec![],
            });
        }
    }
    children.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });
    Ok(children)
}

#[tauri::command(rename_all = "camelCase")]
pub fn read_note(path: String) -> Result<Note, String> {
    let notes_dir = get_notes_dir()?;
    let full_path = notes_dir.join(&path);
    let content = fs::read_to_string(&full_path).map_err(|e| e.to_string())?;
    let metadata = fs::metadata(&full_path).map_err(|e| e.to_string())?;
    let created_at = metadata
        .created()
        .ok()
        .and_then(|t| {
            chrono::DateTime::from_timestamp(
                t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs() as i64,
                0,
            )
        })
        .map(|t| t.format("%Y-%m-%dT%H:%M:%S").to_string())
        .unwrap_or_else(|| Local::now().format("%Y-%m-%dT%H:%M:%S").to_string());

    let modified_at = metadata
        .modified()
        .ok()
        .and_then(|t| {
            chrono::DateTime::from_timestamp(
                t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs() as i64,
                0,
            )
        })
        .map(|t| t.format("%Y-%m-%dT%H:%M:%S").to_string())
        .unwrap_or_else(|| Local::now().format("%Y-%m-%dT%H:%M:%S").to_string());

    let title = std::path::Path::new(&path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Untitled".to_string());

    let links: Vec<String> = extract_wiki_links(&content);
    let tags: Vec<String> = extract_tags(&content);

    Ok(Note {
        path,
        title,
        content,
        tags,
        links,
        created_at,
        updated_at: modified_at,
    })
}

#[tauri::command(rename_all = "camelCase")]
pub fn save_note(path: String, content: String) -> Result<Note, String> {
    let notes_dir = get_notes_dir()?;
    let full_path = notes_dir.join(&path);
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(&full_path, &content).map_err(|e| e.to_string())?;
    read_note(path)
}

#[tauri::command(rename_all = "camelCase")]
pub fn create_note(dir: String, title: String) -> Result<Note, String> {
    let notes_dir = get_notes_dir()?;
    let filename = format!("{}.md", title);
    let path = if dir.is_empty() {
        filename
    } else {
        format!("{}/{}", dir, filename)
    };
    let full_path = notes_dir.join(&path);
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(&full_path, "").map_err(|e| e.to_string())?;
    read_note(path)
}

#[tauri::command(rename_all = "camelCase")]
pub fn delete_note(path: String) -> Result<(), String> {
    let notes_dir = get_notes_dir()?;
    let full_path = notes_dir.join(&path);
    fs::remove_file(&full_path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command(rename_all = "camelCase")]
pub fn search_notes(query: String) -> Result<Vec<Note>, String> {
    let notes_dir = get_notes_dir()?;
    let mut results = Vec::new();
    search_dir(&notes_dir, &notes_dir, &query.to_lowercase(), &mut results)?;
    results.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(results)
}

fn search_dir(
    dir: &PathBuf,
    root: &PathBuf,
    query: &str,
    results: &mut Vec<Note>,
) -> Result<(), String> {
    if !dir.exists() {
        return Ok(());
    }
    let entries = fs::read_dir(dir).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_lowercase();
        if path.is_dir() {
            search_dir(&path, root, query, results)?;
        } else if name.ends_with(".md") {
            let content = fs::read_to_string(&path).unwrap_or_default();
            if name.contains(query) || content.to_lowercase().contains(query) {
                let rel_path = path
                    .strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .to_string();
                if let Ok(note) = read_note(rel_path) {
                    results.push(note);
                }
            }
        }
    }
    Ok(())
}

fn extract_wiki_links(content: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut rest = content;
    while let Some(start) = rest.find("[[") {
        rest = &rest[start + 2..];
        if let Some(end) = rest.find("]]") {
            links.push(rest[..end].to_string());
            rest = &rest[end + 2..];
        }
    }
    links
}

fn extract_tags(content: &str) -> Vec<String> {
    let mut tags = Vec::new();
    for word in content.split_whitespace() {
        if word.starts_with('#') && word.len() > 1 {
            tags.push(word[1..].to_string());
        }
    }
    tags
}
