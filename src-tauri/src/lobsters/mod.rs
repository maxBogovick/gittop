pub mod client;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobstersStory {
    pub short_id: String,
    pub title: String,
    pub url: String,
    pub score: i32,
    pub comment_count: i32,
    pub tags: Vec<String>,
    pub created_at: String,
    pub comments_url: String,
    pub submitter_user: String,
    #[serde(default)]
    pub comments: Vec<LobstersComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobstersComment {
    pub short_id: String,
    pub created_at: String,
    pub comment: String,
    pub comment_plain: String,
    pub depth: i32,
    pub commenting_user: String,
    pub score: i32,
    pub parent_comment: Option<String>,
}
