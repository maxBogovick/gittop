pub mod client;

use serde::{Serialize, Deserialize, Deserializer};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevtoArticle {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub social_image: Option<String>,
    pub published_at: DateTime<Utc>,
    
    // Custom deserialization to handle "tag_list": "tag1, tag2" string format
    // returned by Dev.to article details endpoint.
    #[serde(rename = "tag_list", deserialize_with = "deserialize_tags")]
    pub tags: Vec<String>,
    
    pub slug: String,
    pub path: String,
    pub url: String,
    pub canonical_url: String,
    pub comments_count: i64,
    pub public_reactions_count: i64,
    pub positive_reactions_count: i64,
    pub reading_time_minutes: i32,
    pub user: DevtoUser,
    #[serde(default)]
    pub body_html: Option<String>, // Fetched individually
    #[serde(default)]
    pub body_markdown: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevtoUser {
    pub name: String,
    pub username: String,
    pub profile_image: Option<String>,
    pub twitter_username: Option<String>,
    pub github_username: Option<String>,
    pub website_url: Option<String>,
}

pub fn deserialize_tags<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = serde_json::Value::deserialize(deserializer)?;
    match v {
        serde_json::Value::String(s) => Ok(s
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()),
        serde_json::Value::Array(arr) => {
            let mut result = Vec::new();
            for val in arr {
                if let Some(s) = val.as_str() {
                    result.push(s.to_string());
                }
            }
            Ok(result)
        }
        _ => Ok(vec![]),
    }
}
