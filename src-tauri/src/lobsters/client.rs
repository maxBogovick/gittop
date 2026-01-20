use crate::lobsters::LobstersStory;
use anyhow::Result;

pub struct LobstersClient {
    client: reqwest::Client,
}

impl LobstersClient {
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

    pub async fn get_stories(&self, story_type: &str) -> Result<Vec<LobstersStory>> {
        let url = match story_type {
            "newest" => "https://lobste.rs/newest.json",
            _ => "https://lobste.rs/hottest.json",
        };

        let resp = self.client.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow::anyhow!("Lobsters API Error: {}", resp.status()));
        }

        let stories: Vec<LobstersStory> = resp.json().await?;
        Ok(stories)
    }

    pub async fn get_story_details(&self, short_id: &str) -> Result<LobstersStory> {
        let url = format!("https://lobste.rs/s/{}.json", short_id);
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow::anyhow!("Lobsters API Error: {}", resp.status()));
        }

        let story: LobstersStory = resp.json().await?;
        Ok(story)
    }
}
