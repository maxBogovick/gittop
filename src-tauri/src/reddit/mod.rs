pub mod client;

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedditPost {
    pub id: String,
    pub title: String,
    pub author: String,
    pub subreddit: String,
    pub score: i64,
    pub num_comments: i64,
    pub created_utc: DateTime<Utc>,
    pub url: String,
    pub selftext: String,
    pub permalink: String,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedditListing {
    pub posts: Vec<RedditPost>,
    pub after: Option<String>,
}