pub mod client;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HackerNewsStory {
    pub id: i64,
    pub title: Option<String>, // Sometimes missing if deleted?
    pub url: Option<String>,
    pub by: Option<String>,
    pub time: i64,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub descendants: i64, // comments count
    #[serde(default)]
    pub kids: Vec<i64>, // comment ids
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub text: Option<String>,
}
