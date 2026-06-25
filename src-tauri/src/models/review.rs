use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyReview {
    pub date: String,
    pub content: String,
    pub mood: String,
    pub linked_todo_ids: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

