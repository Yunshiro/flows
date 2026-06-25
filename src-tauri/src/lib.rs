mod commands;
mod db;
mod models;

use db::Database;
use directories::UserDirs;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_dir = UserDirs::new()
        .and_then(|d| {
            let dir = d.home_dir().join(".flows");
            Some(dir)
        })
        .expect("Cannot determine app data directory");

    let database = Database::new(&app_dir).expect("Failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(database)
        .invoke_handler(tauri::generate_handler![
            commands::todo_cmd::list_todos,
            commands::todo_cmd::add_todo,
            commands::todo_cmd::update_todo,
            commands::todo_cmd::toggle_todo,
            commands::todo_cmd::delete_todo,
            commands::todo_cmd::reorder_todos,
            commands::review_cmd::list_reviews,
            commands::review_cmd::get_review,
            commands::review_cmd::save_review,
            commands::weekly_cmd::generate_weekly_summary,
            commands::weekly_cmd::generate_weekly_summary_stream,
            commands::weekly_cmd::test_llm_connection,
            commands::note_cmd::list_notes,
            commands::note_cmd::read_note,
            commands::note_cmd::save_note,
            commands::note_cmd::create_note,
            commands::note_cmd::delete_note,
            commands::note_cmd::search_notes,
            commands::git_cmd::git_init,
            commands::git_cmd::git_get_remote,
            commands::git_cmd::git_set_remote,
            commands::git_cmd::git_push,
            commands::git_cmd::git_pull,
            commands::git_cmd::git_status,
            commands::git_cmd::git_list_files,
            commands::settings_cmd::get_settings,
            commands::settings_cmd::save_settings,
            commands::llm_config_cmd::list_llm_configs,
            commands::llm_config_cmd::save_llm_config,
            commands::llm_config_cmd::delete_llm_config,
            commands::llm_config_cmd::get_llm_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
