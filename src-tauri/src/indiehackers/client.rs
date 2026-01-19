use crate::indiehackers::IndieHackersPost;
use anyhow::Result;

pub struct IndieHackersClient {
    client: reqwest::Client,
}

impl IndieHackersClient {
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_default();
        Self { client }
    }

    pub async fn get_posts(&self) -> Result<Vec<IndieHackersPost>> {
        // Indie Hackers removed their RSS feed and uses a SPA with Firebase backend
        // We'll try to fetch data from their public Firebase endpoint
        let url = "https://indie-hackers.firebaseio.com/threads.json?orderBy=\"bumpedAt\"&limitToLast=30";
        let resp = self.client.get(url).send().await?;

        if !resp.status().is_success() {
            return Err(anyhow::anyhow!("Indie Hackers API Error: {}", resp.status()));
        }

        let data: serde_json::Value = resp.json().await?;

        let mut posts: Vec<IndieHackersPost> = data
            .as_object()
            .map(|obj| {
                obj.iter().filter_map(|(id, thread)| {
                    let title = thread.get("title")?.as_str()?.to_string();
                    let username = thread.get("username").and_then(|u| u.as_str()).unwrap_or("unknown");
                    let link = format!("https://www.indiehackers.com/post/{}", id);
                    let content = thread.get("content")
                        .and_then(|c| c.as_str())
                        .or_else(|| thread.get("linkDescription").and_then(|d| d.as_str()))
                        .unwrap_or("")
                        .to_string();
                    let bumped_at = thread.get("bumpedAt").and_then(|t| t.as_i64()).unwrap_or(0);
                    let thumbnail = thread.get("linkImageUrl").and_then(|u| u.as_str()).map(|s| s.to_string());

                    Some(IndieHackersPost {
                        title,
                        link,
                        author: username.to_string(),
                        pub_date: format_timestamp(bumped_at),
                        content,
                        thumbnail,
                    })
                }).collect::<Vec<_>>()
            })
            .unwrap_or_default();

        // Sort by date (newest first) - bumpedAt is embedded in pub_date parsing
        posts.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));

        Ok(posts)
    }
}

fn format_timestamp(ts: i64) -> String {
    use chrono::{TimeZone, Utc};
    if ts > 0 {
        Utc.timestamp_millis_opt(ts)
            .single()
            .map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
            .unwrap_or_default()
    } else {
        String::new()
    }
}
