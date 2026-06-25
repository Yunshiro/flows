use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub path: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub links: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteTreeNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub children: Vec<NoteTreeNode>,
}

