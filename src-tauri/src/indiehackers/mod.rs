pub mod client;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndieHackersPost {
    pub title: String,
    pub link: String,
    pub author: String,
    pub pub_date: String,
    pub content: String,
    pub thumbnail: Option<String>,
}
