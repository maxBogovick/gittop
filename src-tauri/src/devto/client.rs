use crate::devto::DevtoArticle;
use anyhow::Result;
use reqwest::header;

pub struct DevtoClient {
    client: reqwest::Client,
}

impl DevtoClient {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        // Dev.to encourages a User-Agent
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("gittop"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_default();

        Self { client }
    }

    pub async fn get_articles(
        &self,
        tag: Option<&str>,
        top: Option<i32>, // Number of days for "top" articles. If None, it might return latest or default.
        state: Option<&str>, // "fresh", "rising", "all"
        page: u32,
        per_page: u32,
    ) -> Result<Vec<DevtoArticle>> {
        let url = "https://dev.to/api/articles";
        
        let mut query = Vec::new();
        query.push(("page", page.to_string()));
        query.push(("per_page", per_page.to_string()));

        if let Some(t) = tag {
            if !t.is_empty() {
                query.push(("tag", t.to_string()));
            }
        }

        if let Some(days) = top {
            query.push(("top", days.to_string()));
        }

        if let Some(s) = state {
            query.push(("state", s.to_string()));
        }

        let resp = self.client.get(url)
            .query(&query)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Dev.to API Error: {} - {}", status, text));
        }

        let articles: Vec<DevtoArticle> = resp.json().await?;
        Ok(articles)
    }

    pub async fn get_article_details(&self, id: u64) -> Result<DevtoArticle> {
        let url = format!("https://dev.to/api/articles/{}", id);
        let resp = self.client.get(&url).send().await?;
        
        if !resp.status().is_success() {
             return Err(anyhow::anyhow!("Dev.to API Error: {}", resp.status()));
        }

        let article: DevtoArticle = resp.json().await?;
        Ok(article)
    }

    pub async fn search(
        &self,
        query: &str,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<DevtoArticle>> {
        // Dev.to API doesn't have a full text search endpoint exposed clearly.
        // We will treat the query as a tag for now.
        // Remove spaces and special chars to make it a valid tag? 
        // Or just pass it as is and let the API decide.
        // If query has spaces, it's likely not a tag. 
        if query.contains(' ') {
            return Ok(vec![]); // Skip complex queries for now to avoid bad results
        }

        self.get_articles(Some(query), None, None, page, per_page).await
    }
}
