pub mod client;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductHuntPost {
    pub id: String,
    pub name: String,
    pub tagline: String,
    pub url: String,
    #[serde(rename = "votesCount")]
    pub votes_count: i32,
    #[serde(rename = "commentsCount")]
    pub comments_count: i32,
    pub thumbnail: Option<ProductHuntThumbnail>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductHuntThumbnail {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductHuntEdge {
    pub node: ProductHuntPost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductHuntPostsResponse {
    pub edges: Vec<ProductHuntEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductHuntData {
    pub posts: ProductHuntPostsResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductHuntGraphQLResponse {
    pub data: ProductHuntData,
}
