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
    pub submitter_user: LobstersUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobstersUser {
    pub username: String,
}
