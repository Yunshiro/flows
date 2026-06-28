use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    #[serde(default = "default_notes_path")]
    pub notes_path: String,
    #[serde(default)]
    pub git_remote_url: String,
    #[serde(default = "default_branch")]
    pub git_branch: String,
    #[serde(default)]
    pub auto_sync_enabled: bool,
    #[serde(default = "default_sync_interval")]
    pub auto_sync_interval_minutes: i32,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_llm_base_url")]
    pub llm_base_url: String,
    #[serde(default)]
    pub llm_api_key: String,
    #[serde(default = "default_llm_model")]
    pub llm_model: String,
}

fn default_notes_path() -> String {
    String::new()
}
fn default_branch() -> String {
    String::new()
}
fn default_sync_interval() -> i32 {
    30
}
fn default_theme() -> String {
    "system".into()
}
fn default_llm_base_url() -> String {
    "https://api.openai.com/v1".into()
}
fn default_llm_model() -> String {
    "deepseek-chat".into()
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            notes_path: String::new(),
            git_remote_url: String::new(),
            git_branch: String::new(),
            auto_sync_enabled: false,
            auto_sync_interval_minutes: 30,
            theme: "system".into(),
            llm_base_url: "https://api.openai.com/v1".into(),
            llm_api_key: String::new(),
            llm_model: "deepseek-chat".into(),
        }
    }
}
