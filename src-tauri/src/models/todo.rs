use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub priority: String,
    pub tags: Vec<String>,
    pub done: bool,
    pub est_minutes: i32,
    pub actual_minutes: Option<i32>,
    pub date: String,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TodoChanges {
    pub title: Option<String>,
    pub priority: Option<String>,
    pub done: Option<bool>,
    pub est_minutes: Option<i32>,
    pub actual_minutes: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub sort_order: Option<i32>,
}

