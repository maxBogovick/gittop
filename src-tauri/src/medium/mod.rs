pub mod client;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediumArticle {
    pub title: String,
    pub link: String,
    pub author: String,
    pub pub_date: String,
    pub categories: Vec<String>,
    pub content: String, // Description or content:encoded
    pub thumbnail: Option<String>, // Extracted from content if possible
}
