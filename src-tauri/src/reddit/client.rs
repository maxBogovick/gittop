use reqwest::header;
use crate::reddit::{RedditPost, RedditListing};
use anyhow::{Result, Context};
use serde_json::Value;
use chrono::{DateTime, TimeZone, Utc};

pub struct RedditClient {
    client: reqwest::Client,
}

impl RedditClient {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        // Reddit requires a custom User-Agent
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("desktop:com.maxim.gittop:v0.1.0 (by /u/gittop_dev)"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_default();

        Self { client }
    }

    pub async fn get_posts(&self, subreddit: &str, sort: &str, time: &str, limit: u32, after: Option<String>) -> Result<RedditListing> {
        // url: https://www.reddit.com/r/{subreddit}/{sort}.json?t={time}&limit={limit}&after={after}
        let sub = if subreddit.is_empty() || subreddit.eq_ignore_ascii_case("All") { "all" } else { subreddit };
        let url = format!("https://www.reddit.com/r/{}/{}.json", sub, sort);

        let mut req = self.client.get(&url)
            .query(&[("t", time), ("limit", &limit.to_string())]);
        
        if let Some(a) = after {
            req = req.query(&[("after", &a)]);
        }

        let resp = req.send().await?;

        if !resp.status().is_success() {
             return Err(anyhow::anyhow!("Reddit API Error: {}", resp.status()));
        }

        let json: Value = resp.json().await?;
        
        let children = json["data"]["children"].as_array().context("No posts found")?;
        let after_token = json["data"]["after"].as_str().map(|s| s.to_string());

        let mut posts = Vec::new();
        for child in children {
            let data = &child["data"];
            
            // Handle timestamp (created_utc is f64 epoch)
            let created_epoch = data["created_utc"].as_f64().unwrap_or_default() as i64;
            let created_utc = Utc.timestamp_opt(created_epoch, 0).unwrap();

            posts.push(RedditPost {
                id: data["id"].as_str().unwrap_or_default().to_string(),
                title: data["title"].as_str().unwrap_or_default().to_string(),
                author: data["author"].as_str().unwrap_or_default().to_string(),
                subreddit: data["subreddit"].as_str().unwrap_or_default().to_string(),
                score: data["score"].as_i64().unwrap_or_default(),
                num_comments: data["num_comments"].as_i64().unwrap_or_default(),
                created_utc,
                url: data["url"].as_str().unwrap_or_default().to_string(),
                selftext: data["selftext"].as_str().unwrap_or_default().to_string(),
                permalink: format!("https://www.reddit.com{}", data["permalink"].as_str().unwrap_or_default()),
                thumbnail: data["thumbnail"].as_str().filter(|s| s.starts_with("http")).map(|s| s.to_string()),
            });
        }

        Ok(RedditListing { posts, after: after_token })
    }

    pub async fn search(&self, query: &str, sort: &str, time: &str, limit: u32, after: Option<String>) -> Result<RedditListing> {
        let url = "https://www.reddit.com/search.json";
        
        let mut req = self.client.get(url)
            .query(&[("q", query), ("sort", sort), ("t", time), ("limit", &limit.to_string())]);

        if let Some(a) = after {
            req = req.query(&[("after", &a)]);
        }

        let resp = req.send().await?;
        if !resp.status().is_success() {
             return Err(anyhow::anyhow!("Reddit API Error: {}", resp.status()));
        }

        let json: Value = resp.json().await?;
        let children = json["data"]["children"].as_array().context("No search results found")?;
        let after_token = json["data"]["after"].as_str().map(|s| s.to_string());

        let mut posts = Vec::new();
        for child in children {
            let data = &child["data"];
            let created_epoch = data["created_utc"].as_f64().unwrap_or_default() as i64;
            let created_utc = Utc.timestamp_opt(created_epoch, 0).unwrap();

            posts.push(RedditPost {
                id: data["id"].as_str().unwrap_or_default().to_string(),
                title: data["title"].as_str().unwrap_or_default().to_string(),
                author: data["author"].as_str().unwrap_or_default().to_string(),
                subreddit: data["subreddit"].as_str().unwrap_or_default().to_string(),
                score: data["score"].as_i64().unwrap_or_default(),
                num_comments: data["num_comments"].as_i64().unwrap_or_default(),
                created_utc,
                url: data["url"].as_str().unwrap_or_default().to_string(),
                selftext: data["selftext"].as_str().unwrap_or_default().to_string(),
                permalink: format!("https://www.reddit.com{}", data["permalink"].as_str().unwrap_or_default()),
                thumbnail: data["thumbnail"].as_str().filter(|s| s.starts_with("http")).map(|s| s.to_string()),
            });
        }

        Ok(RedditListing { posts, after: after_token })
    }
}
