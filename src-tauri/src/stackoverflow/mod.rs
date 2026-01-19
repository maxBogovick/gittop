pub mod client;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackOverflowOwner {
    pub account_id: Option<i64>,
    pub reputation: Option<i64>,
    pub user_id: Option<i64>,
    pub user_type: Option<String>,
    pub profile_image: Option<String>,
    pub display_name: Option<String>,
    pub link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackOverflowQuestion {
    pub question_id: i64,
    pub title: String,
    pub link: String,
    pub is_answered: bool,
    pub view_count: i64,
    pub answer_count: i64,
    pub score: i64,
    pub last_activity_date: i64,
    pub creation_date: i64,
    pub last_edit_date: Option<i64>,
    pub tags: Vec<String>,
    pub owner: StackOverflowOwner,
    // Details (fetched separately or included if filter allows)
    pub body: Option<String>, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackOverflowResponse {
    pub items: Vec<StackOverflowQuestion>,
    pub has_more: bool,
    pub quota_max: i64,
    pub quota_remaining: i64,
}
