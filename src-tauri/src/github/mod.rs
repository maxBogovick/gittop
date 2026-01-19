pub mod client;
pub mod graphql;
pub mod rest;

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub owner: String,
    pub name: String,
    pub description: Option<String>,
    pub primary_language: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub stars_total: i64,
    pub stars_delta: i64,
    pub forks: i64,
    pub open_issues: i64,
    pub pull_requests_count: i64,
    pub comments_count: i64,
    pub releases_count: i64,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub repository_url: String,
    pub owner_avatar_url: Option<String>,
    pub license: Option<String>,
}