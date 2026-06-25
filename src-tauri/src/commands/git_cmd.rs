use directories::UserDirs;
use std::process::Command;

fn notes_dir() -> std::path::PathBuf {
    UserDirs::new()
        .map(|d| d.home_dir().join("flows-notes"))
        .unwrap_or_else(|| std::path::PathBuf::from("flows-notes"))
}

fn run_git(args: &[&str]) -> Result<String, String> {
    let dir = notes_dir();
    let output = Command::new("git")
        .args(args)
        .current_dir(&dir)
        .output()
        .map_err(|e| format!("Git 命令失败: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(format!("{}{}", stdout, stderr))
    } else {
        Err(format!("{}{}", stdout, stderr))
    }
}

fn ensure_git_init() -> Result<String, String> {
    let dir = notes_dir();
    let git_dir = dir.join(".git");
    if !git_dir.exists() {
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        run_git(&["init"])?;
        run_git(&["config", "user.name", "Flows"])?;
        run_git(&["config", "user.email", "flows@local.dev"])?;
        Ok("Git 仓库已初始化".into())
    } else {
        Ok("Git 仓库已存在".into())
    }
}

#[tauri::command(rename_all = "camelCase")]
pub fn git_init() -> Result<String, String> {
    ensure_git_init()
}

#[tauri::command(rename_all = "camelCase")]
pub fn git_get_remote() -> Result<String, String> {
    let dir = notes_dir();
    let git_dir = dir.join(".git");
    if !git_dir.exists() {
        return Ok(String::new());
    }
    let remotes = run_git(&["remote"]).unwrap_or_default();
    if !remotes.contains("origin") {
        return Ok(String::new());
    }
    run_git(&["remote", "get-url", "origin"]).map(|s| s.trim().to_string())
}

#[tauri::command(rename_all = "camelCase")]
pub fn git_set_remote(url: String) -> Result<String, String> {
    ensure_git_init()?;
    let remotes = run_git(&["remote"]).unwrap_or_default();
    if remotes.contains("origin") {
        run_git(&["remote", "set-url", "origin", &url])
    } else {
        run_git(&["remote", "add", "origin", &url])
    }
}

fn ensure_remote() -> Result<(), String> {
    let remotes = run_git(&["remote"]).unwrap_or_default();
    if !remotes.contains("origin") {
        return Err("未配置远程仓库。请先在设置中填写远程仓库地址，然后点击「配置」按钮。".into());
    }
    Ok(())
}

fn current_branch() -> String {
    run_git(&["branch", "--show-current"])
        .unwrap_or_else(|_| "main".into())
        .trim()
        .to_string()
}

#[tauri::command(rename_all = "camelCase")]
pub fn git_push() -> Result<String, String> {
    ensure_git_init()?;
    ensure_remote()?;
    run_git(&["add", "-A"])?;
    let status = run_git(&["status", "--porcelain"]).unwrap_or_default();
    if !status.trim().is_empty() {
        let date = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
        run_git(&["commit", "-m", &format!("Flows auto-sync: {}", date)])?;
    }
    let branch = current_branch();
    // Try push with upstream set; if branch doesn't exist on remote, push will create it
    run_git(&["push", "-u", "origin", &branch])
}

#[tauri::command(rename_all = "camelCase")]
pub fn git_pull() -> Result<String, String> {
    ensure_git_init()?;
    ensure_remote()?;
    let branch = current_branch();
    // Try to set upstream first, then pull
    let _ = run_git(&["branch", "--set-upstream-to", &format!("origin/{}", branch)]);
    run_git(&["pull", "--rebase", "origin", &branch])
}

#[tauri::command(rename_all = "camelCase")]
pub fn git_status() -> Result<String, String> {
    let dir = notes_dir();
    let git_dir = dir.join(".git");
    if !git_dir.exists() {
        return Ok("未初始化".into());
    }
    let status = run_git(&["status", "--porcelain"]).unwrap_or_default();
    let branch = run_git(&["branch", "--show-current"]).unwrap_or_else(|_| "?".into());
    if status.trim().is_empty() {
        Ok(format!("{} (干净)", branch.trim()))
    } else {
        let count = status.lines().count();
        Ok(format!("{} ({} 个文件待同步)", branch.trim(), count))
    }
}

#[derive(serde::Serialize)]
pub struct GitFileItem {
    pub status: String,
    pub path: String,
}

#[tauri::command(rename_all = "camelCase")]
pub fn git_list_files() -> Result<Vec<GitFileItem>, String> {
    let dir = notes_dir();
    let git_dir = dir.join(".git");
    if !git_dir.exists() {
        return Ok(vec![]);
    }
    let status = run_git(&["status", "--porcelain"]).unwrap_or_default();
    let mut files = Vec::new();
    for line in status.lines() {
        if line.len() >= 4 {
            files.push(GitFileItem {
                status: line[..2].trim().to_string(),
                path: line[3..].to_string(),
            });
        }
    }
    Ok(files)
}
