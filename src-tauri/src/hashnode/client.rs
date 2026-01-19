use crate::hashnode::{HashnodeGraphQLResponse, HashnodePost};
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

pub struct HashnodeClient {
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct TagResponse {
    data: Option<TagData>,
}

#[derive(Debug, Deserialize)]
struct TagData {
    tag: Option<TagInfo>,
}

#[derive(Debug, Deserialize)]
struct TagInfo {
    id: String,
}

impl HashnodeClient {
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

    async fn get_tag_id(&self, slug: &str) -> Result<Option<String>> {
        let url = "https://gql.hashnode.com/";
        let query = r#"query GetTag($slug: String!) { tag(slug: $slug) { id } }"#;

        let resp = self.client.post(url)
            .json(&json!({
                "query": query,
                "variables": { "slug": slug.to_lowercase() }
            }))
            .send()
            .await?;

        let tag_resp: TagResponse = resp.json().await?;
        Ok(tag_resp.data.and_then(|d| d.tag).map(|t| t.id))
    }

    pub async fn get_feed(&self, tag: Option<&str>, feed_type: Option<&str>, after: Option<&str>) -> Result<Vec<HashnodePost>> {
        let url = "https://gql.hashnode.com/";

        let query = r#"
            query GetFeed($first: Int!, $after: String, $filter: FeedFilter) {
                feed(first: $first, after: $after, filter: $filter) {
                    edges {
                        node {
                            id
                            title
                            brief
                            url
                            publishedAt
                            readTimeInMinutes
                            reactionCount
                            responseCount
                            author {
                                name
                                username
                                profilePicture
                            }
                            coverImage {
                                url
                            }
                        }
                    }
                }
            }
        "#;

        let feed_type_value = match feed_type {
            Some("RECENT") => "RECENT",
            Some("RELEVANT") => "RELEVANT",
            _ => "FEATURED",
        };

        let mut filter = json!({ "type": feed_type_value });

        // Get tag ID if tag slug provided
        if let Some(t) = tag {
            if !t.is_empty() {
                if let Ok(Some(tag_id)) = self.get_tag_id(t).await {
                    filter.as_object_mut().unwrap().insert("tags".to_string(), json!([tag_id]));
                }
            }
        }

        let variables = json!({
            "first": 30,
            "after": after,
            "filter": filter
        });

        let resp = self.client.post(url)
            .json(&json!({
                "query": query,
                "variables": variables
            }))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Hashnode API Error ({}): {}", status, err_body));
        }

        let gql_resp: HashnodeGraphQLResponse = resp.json().await?;
        Ok(gql_resp.data.feed.edges.into_iter().map(|e| e.node).collect())
    }
}
