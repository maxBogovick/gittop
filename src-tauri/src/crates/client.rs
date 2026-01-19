use crate::crates::{CratesResponse, Crate};
use anyhow::Result;
use reqwest::header;

pub struct CratesClient {
    client: reqwest::Client,
}

impl CratesClient {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("gittop (github.com/maxim/gittop)"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_default();

        Self { client }
    }

    pub async fn get_crates(&self, query: Option<&str>, sort: &str, page: u32) -> Result<Vec<Crate>> {
        let url = "https://crates.io/api/v1/crates";
        let mut params = Vec::new();
        params.push(("page", page.to_string()));
        params.push(("per_page", "30".to_string()));
        params.push(("sort", sort.to_string()));

        if let Some(q) = query {
            if !q.is_empty() {
                params.push(("q", q.to_string()));
            }
        }

        let resp = self.client.get(url).query(&params).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow::anyhow!("Crates.io API Error: {}", resp.status()));
        }

        let response: CratesResponse = resp.json().await?;
        Ok(response.crates)
    }
}
