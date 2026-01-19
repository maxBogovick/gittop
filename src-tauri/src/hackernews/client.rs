use crate::hackernews::HackerNewsStory;
use anyhow::Result;
use reqwest::Client;
use futures::future::join_all;
use serde::Deserialize;

pub struct HackerNewsClient {
    client: Client,
}

#[derive(Debug, Deserialize)]
struct AlgoliaResponse {
    hits: Vec<AlgoliaHit>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AlgoliaHit {
    #[serde(rename = "objectID")]
    object_id: String,
    title: Option<String>,
    url: Option<String>,
    author: Option<String>,
    created_at_i: i64,
    points: Option<i64>,
    num_comments: Option<i64>,
    story_text: Option<String>,
}

impl HackerNewsClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_stories(&self, story_type: &str, page: usize, per_page: usize) -> Result<Vec<HackerNewsStory>> {
        let base = "https://hacker-news.firebaseio.com/v0";
        let endpoint = match story_type {
            "new" => "newstories",
            "best" => "beststories",
            "show" => "showstories",
            "ask" => "askstories",
            "job" => "jobstories",
            _ => "topstories",
        };
        let url = format!("{}/{}.json", base, endpoint);

        let ids: Vec<i64> = self.client.get(&url).send().await?.json().await?;

        let start = (page - 1) * per_page;
        let end = start + per_page;

        if start >= ids.len() {
            return Ok(vec![]);
        }

        let slice = &ids[start..std::cmp::min(end, ids.len())];

        let tasks: Vec<_> = slice.iter().map(|id| {
            let client = self.client.clone();
            let id = *id;
            async move {
                let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
                match client.get(&url).send().await {
                    Ok(resp) => resp.json::<HackerNewsStory>().await.ok(),
                    Err(_) => None,
                }
            }
        }).collect();

        let results = join_all(tasks).await;
        Ok(results.into_iter().flatten().collect())
    }

    pub async fn search_stories(&self, query: &str, sort: &str, time_range: Option<&str>, page: usize) -> Result<Vec<HackerNewsStory>> {
        // Use Algolia HN Search API
        let endpoint = match sort {
            "date" => "search_by_date",
            _ => "search", // popularity
        };

        let mut url = format!(
            "https://hn.algolia.com/api/v1/{}?query={}&tags=story&hitsPerPage=30&page={}",
            endpoint,
            urlencoding::encode(query),
            page - 1
        );

        // Add time filter
        if let Some(range) = time_range {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let seconds_ago = match range {
                "day" => 86400,
                "week" => 604800,
                "month" => 2592000,
                "year" => 31536000,
                _ => 0,
            };

            if seconds_ago > 0 {
                let min_time = now - seconds_ago;
                url.push_str(&format!("&numericFilters=created_at_i>{}", min_time));
            }
        }

        let resp = self.client.get(&url).send().await?;
        let algolia: AlgoliaResponse = resp.json().await?;

        let stories = algolia.hits.into_iter().map(|hit| {
            HackerNewsStory {
                id: hit.object_id.parse().unwrap_or(0),
                title: hit.title,
                url: hit.url,
                by: hit.author,
                time: hit.created_at_i,
                score: hit.points.unwrap_or(0),
                descendants: hit.num_comments.unwrap_or(0),
                kids: vec![],
                type_: Some("story".to_string()),
                text: hit.story_text,
            }
        }).collect();

        Ok(stories)
    }
}
