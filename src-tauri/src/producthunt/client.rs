use crate::producthunt::{ProductHuntGraphQLResponse, ProductHuntPost};
use anyhow::Result;
use serde_json::json;
use reqwest::header;

pub struct ProductHuntClient {
    client: reqwest::Client,
    token: String,
}

impl ProductHuntClient {
    pub fn new(token: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("gittop"));
        if !token.is_empty() {
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            );
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_default();

        Self { client, token }
    }

    pub async fn get_top_posts(&self) -> Result<Vec<ProductHuntPost>> {
        if self.token.is_empty() {
            return Err(anyhow::anyhow!("Product Hunt API Token not found. Please set PRODUCT_HUNT_TOKEN."));
        }

        let url = "https://api.producthunt.com/v2/api/graphql";
        
        let query = r#"
            query GetTopPosts {
                posts(first: 30, order: VOTES) {
                    edges {
                        node {
                            id
                            name
                            tagline
                            url
                            votesCount
                            commentsCount
                            createdAt
                            thumbnail {
                                url
                            }
                        }
                    }
                }
            }
        "#;

        let resp = self.client.post(url)
            .json(&json!({ "query": query }))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(anyhow::anyhow!("Product Hunt API Error: {}", resp.status()));
        }

        let gql_resp: ProductHuntGraphQLResponse = resp.json().await?;
        Ok(gql_resp.data.posts.edges.into_iter().map(|e| e.node).collect())
    }
}
