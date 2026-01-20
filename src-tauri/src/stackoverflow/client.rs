use crate::stackoverflow::{StackOverflowResponse, StackOverflowQuestion};
use anyhow::Result;
use reqwest::header;

pub struct StackOverflowClient {
    client: reqwest::Client,
}

impl StackOverflowClient {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("gittop"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_default();

        Self { client }
    }

    pub async fn get_questions(
        &self,
        tag: Option<&str>,
        sort: &str, // activity, votes, creation, hot, week, month
        page: u32,
        per_page: u32,
    ) -> Result<Vec<StackOverflowQuestion>> {
        let url = "https://api.stackexchange.com/2.3/questions";
        
        let mut query = Vec::new();
        query.push(("site", "stackoverflow".to_string()));
        query.push(("page", page.to_string()));
        query.push(("pagesize", per_page.to_string()));
        query.push(("order", "desc".to_string()));
        query.push(("sort", sort.to_string()));
        // Use a filter that includes body? For list view, maybe not needed to save bandwidth.
        // But for drawer we need it. 
        // Filter '!nNPvSNp0_0' includes body. 
        // Standard filter 'default' does not.
        // Let's use 'withbody' wrapper or custom filter if needed. 
        // For now, let's fetch list without body, and maybe details with body?
        // Or just fetch with body for simplicity (text-heavy but okay for desktop).
        // filter=!9_bDDxJY5 gets body.
        query.push(("filter", "!9_bDDxJY5".to_string())); 

        if let Some(t) = tag {
            if !t.is_empty() {
                query.push(("tagged", t.to_string()));
            }
        }

        let resp = self.client.get(url)
            .query(&query)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(anyhow::anyhow!("StackOverflow API Error: {}", resp.status()));
        }

        let response: StackOverflowResponse = resp.json().await?;
        Ok(response.items)
    }

    pub async fn search(
        &self,
        query: &str,
        sort: &str, // activity, votes, creation, relevance
        page: u32,
        per_page: u32,
    ) -> Result<Vec<StackOverflowQuestion>> {
        let url = "https://api.stackexchange.com/2.3/search/advanced";
        
        let mut params = Vec::new();
        params.push(("site", "stackoverflow".to_string()));
        params.push(("page", page.to_string()));
        params.push(("pagesize", per_page.to_string()));
        params.push(("order", "desc".to_string()));
        params.push(("sort", sort.to_string()));
        params.push(("q", query.to_string()));
        // filter=!9_bDDxJY5 gets body.
        params.push(("filter", "!9_bDDxJY5".to_string())); 

        let resp = self.client.get(url)
            .query(&params)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(anyhow::anyhow!("StackOverflow API Error: {}", resp.status()));
        }

        let response: StackOverflowResponse = resp.json().await?;
        Ok(response.items)
    }
}
