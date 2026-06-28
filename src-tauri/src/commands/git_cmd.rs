use directories::UserDirs;
use std::collections::BTreeSet;
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
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn normalize_branch(branch: String) -> Result<String, String> {
    let branch = branch.trim().to_string();
    if branch.is_empty() {
        return Err("请先选择或填写同步分支。".into());
    }
    run_git(&["check-ref-format", "--branch", &branch])
        .map_err(|_| format!("分支名无效: {}", branch))?;
    Ok(branch)
}

fn ensure_branch_checked_out(branch: &str) -> Result<(), String> {
    if current_branch() == branch {
        return Ok(());
    }
    if run_git(&["checkout", branch]).is_ok() {
        return Ok(());
    }
    run_git(&["checkout", "-b", branch]).map(|_| ())
}

fn parse_branch_lines(output: &str, branches: &mut BTreeSet<String>) {
    for line in output.lines() {
        let branch = line.trim().trim_start_matches('*').trim();
        if branch.is_empty() || branch.contains("HEAD ->") {
            continue;
        }
        let branch = branch.strip_prefix("origin/").unwrap_or(branch);
        if !branch.is_empty() {
            branches.insert(branch.to_string());
        }
    }
}

#[tauri::command(rename_all = "camelCase")]
pub fn git_checkout_branch(branch: String) -> Result<String, String> {
    ensure_git_init()?;
    let branch = normalize_branch(branch)?;
    ensure_branch_checked_out(&branch)?;
    Ok(format!("已切换到 {}", branch))
}

#[tauri::command(rename_all = "camelCase")]
pub fn git_list_branches() -> Result<Vec<String>, String> {
    let dir = notes_dir();
    let git_dir = dir.join(".git");
    if !git_dir.exists() {
        return Ok(vec![]);
    }

    let mut branches = BTreeSet::new();
    let local = run_git(&["branch", "--format=%(refname:short)"]).unwrap_or_default();
    parse_branch_lines(&local, &mut branches);
    let remote = run_git(&["branch", "-r", "--format=%(refname:short)"]).unwrap_or_default();
    parse_branch_lines(&remote, &mut branches);

    let current = current_branch();
    let mut result = Vec::new();
    if !current.is_empty() {
        branches.remove(&current);
        result.push(current);
    }
    result.extend(branches);
    Ok(result)
}

#[tauri::command(rename_all = "camelCase")]
pub fn git_push(branch: String) -> Result<String, String> {
    ensure_git_init()?;
    ensure_remote()?;
    let branch = normalize_branch(branch)?;
    ensure_branch_checked_out(&branch)?;
    run_git(&["add", "-A"])?;
    let status = run_git(&["status", "--porcelain"]).unwrap_or_default();
    if !status.trim().is_empty() {
        let date = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
        run_git(&["commit", "-m", &format!("Flows auto-sync: {}", date)])?;
    }
    run_git(&["push", "-u", "origin", &branch])
}

#[tauri::command(rename_all = "camelCase")]
pub fn git_pull(branch: String) -> Result<String, String> {
    ensure_git_init()?;
    ensure_remote()?;
    let branch = normalize_branch(branch)?;
    let _ = run_git(&["fetch", "origin", &branch]);
    ensure_branch_checked_out(&branch)?;
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
    let branch = current_branch();
    let branch = if branch.is_empty() { "未选择分支".into() } else { branch };
    if status.trim().is_empty() {
        Ok(format!("{} (干净)", branch))
    } else {
        let count = status.lines().count();
        Ok(format!("{} ({} 个文件待同步)", branch, count))
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
