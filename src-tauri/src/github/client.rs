use reqwest::header;
use crate::github::Repository;
use anyhow::{Result, Context};
use serde_json::{json, Value};
use chrono::{DateTime, Utc};

pub struct GitHubClient {
    client: reqwest::Client,
    token: String,
}

impl GitHubClient {
    pub fn new(token: String) -> Self {
        let mut headers = header::HeaderMap::new();
        let val = header::HeaderValue::from_str(&format!("Bearer {}", token));
        if let Ok(v) = val {
            headers.insert(header::AUTHORIZATION, v);
        }
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("gittop"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_default();

        Self { client, token }
    }

    pub async fn search_repositories(&self, query_str: &str, page: u32) -> Result<Vec<Repository>> {
        if page == 1 {
            match self.search_repositories_graphql(query_str).await {
                Ok(repos) => Ok(repos),
                Err(e) => {
                    eprintln!("GraphQL failed: {}. Trying REST fallback...", e);
                    self.search_repositories_rest(query_str, page).await
                }
            }
        } else {
             self.search_repositories_rest(query_str, page).await
        }
    }

    async fn search_repositories_graphql(&self, query_str: &str) -> Result<Vec<Repository>> {
        let query = r#"
        query($query: String!) {
            search(query: $query, type: REPOSITORY, first: 30) {
                edges {
                    node {
                        ... on Repository {
                            name
                            owner { 
                                login 
                                avatarUrl
                            }
                            description
                            primaryLanguage { name }
                            licenseInfo { spdxId }
                            createdAt
                            updatedAt
                            stargazers { totalCount }
                            forks { totalCount }
                            issues(states: OPEN) { totalCount }
                            pullRequests(states: OPEN) { totalCount }
                            releases { totalCount }
                            pushedAt
                            url
                        }
                    }
                }
            }
        }
        "#;

        let body = json!({
            "query": query,
            "variables": {
                "query": query_str
            }
        });

        let resp = self.client.post("https://api.github.com/graphql")
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            eprintln!("GitHub GraphQL Error: Status {}", status);
            return Err(anyhow::anyhow!("GitHub GraphQL Error: {} - {}", status, text));
        }

        let text = resp.text().await?;
        let json: Value = serde_json::from_str(&text).context("Failed to parse JSON")?;

        if let Some(errors) = json.get("errors") {
            return Err(anyhow::anyhow!("GraphQL Error: {:?}", errors));
        }

        let edges = json["data"]["search"]["edges"].as_array().context("No items found")?;
        let mut repos = Vec::new();

        for edge in edges {
            let node = &edge["node"];
            repos.push(Repository {
                owner: node["owner"]["login"].as_str().unwrap_or_default().to_string(),
                name: node["name"].as_str().unwrap_or_default().to_string(),
                description: node["description"].as_str().map(|s| s.to_string()),
                primary_language: node["primaryLanguage"]["name"].as_str().map(|s| s.to_string()),
                created_at: DateTime::parse_from_rfc3339(node["createdAt"].as_str().unwrap_or_default())?.with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(node["updatedAt"].as_str().unwrap_or_default())?.with_timezone(&Utc),
                stars_total: node["stargazers"]["totalCount"].as_i64().unwrap_or_default(),
                stars_delta: 0,
                forks: node["forks"]["totalCount"].as_i64().unwrap_or_default(),
                open_issues: node["issues"]["totalCount"].as_i64().unwrap_or_default(),
                pull_requests_count: node["pullRequests"]["totalCount"].as_i64().unwrap_or_default(),
                comments_count: 0,
                releases_count: node["releases"]["totalCount"].as_i64().unwrap_or_default(),
                last_activity_at: node["pushedAt"].as_str().map(|s| DateTime::parse_from_rfc3339(s).unwrap_or_default().with_timezone(&Utc)),
                repository_url: node["url"].as_str().unwrap_or_default().to_string(),
                owner_avatar_url: node["owner"]["avatarUrl"].as_str().map(|s| s.to_string()),
                license: node["licenseInfo"]["spdxId"].as_str().map(|s| s.to_string()),
            });
        }

        Ok(repos)
    }

    async fn search_repositories_rest(&self, query_str: &str, page: u32) -> Result<Vec<Repository>> {
        let url = "https://api.github.com/search/repositories";
        
        let resp = self.client.get(url)
            .query(&[
                ("q", query_str), 
                ("per_page", "30"),
                ("page", &page.to_string())
            ])
            .send()
            .await?;

        if !resp.status().is_success() {
             let status = resp.status();
             let text = resp.text().await.unwrap_or_default();
             return Err(anyhow::anyhow!("GitHub REST Error: {} - {}", status, text));
        }

        let json: Value = resp.json().await?;
        let items = json["items"].as_array().context("No items found in REST response")?;

        let mut repos = Vec::new();
        for item in items {
            repos.push(Repository {
                owner: item["owner"]["login"].as_str().unwrap_or_default().to_string(),
                name: item["name"].as_str().unwrap_or_default().to_string(),
                description: item["description"].as_str().map(|s| s.to_string()),
                primary_language: item["language"].as_str().map(|s| s.to_string()),
                created_at: DateTime::parse_from_rfc3339(item["created_at"].as_str().unwrap_or_default())?.with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(item["updated_at"].as_str().unwrap_or_default())?.with_timezone(&Utc),
                stars_total: item["stargazers_count"].as_i64().unwrap_or_default(),
                stars_delta: 0,
                forks: item["forks_count"].as_i64().unwrap_or_default(),
                open_issues: item["open_issues_count"].as_i64().unwrap_or_default(),
                pull_requests_count: 0, 
                comments_count: 0,
                releases_count: 0,
                last_activity_at: item["pushed_at"].as_str().map(|s| DateTime::parse_from_rfc3339(s).unwrap_or_default().with_timezone(&Utc)),
                repository_url: item["html_url"].as_str().unwrap_or_default().to_string(),
                owner_avatar_url: item["owner"]["avatar_url"].as_str().map(|s| s.to_string()),
                license: item["license"]["spdx_id"].as_str().map(|s| s.to_string()),
            });
        }

        Ok(repos)
    }

    pub async fn get_readme(&self, owner: &str, name: &str) -> Result<String> {
        let url = format!("https://api.github.com/repos/{}/{}/readme", owner, name);
        
        let resp = self.client.get(&url)
            .header("Accept", "application/vnd.github.raw+json")
            .send()
            .await?;

        if !resp.status().is_success() {
            if resp.status() == reqwest::StatusCode::NOT_FOUND {
                return Ok("No README found.".to_string());
            }
            return Err(anyhow::anyhow!("GitHub API Error: {}", resp.status()));
        }

        let text = resp.text().await?;
        Ok(text)
    }
}
