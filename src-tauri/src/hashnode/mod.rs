pub mod client;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashnodeUser {
    pub name: String,
    pub username: String,
    pub profile_picture: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashnodePost {
    pub id: String,
    pub title: String,
    pub brief: String,
    pub url: String,
    pub cover_image: Option<HashnodeCoverImage>,
    pub author: HashnodeUser,
    pub published_at: String,
    pub read_time_in_minutes: i32,
    pub reaction_count: i32,
    pub response_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashnodeCoverImage {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashnodeFeedResponse {
    pub edges: Vec<HashnodeEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashnodeEdge {
    pub node: HashnodePost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashnodeData {
    pub feed: HashnodeFeedResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashnodeGraphQLResponse {
    pub data: HashnodeData,
}
