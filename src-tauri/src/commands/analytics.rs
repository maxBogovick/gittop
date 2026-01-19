use tauri::{State, command};
use crate::time_range::TimeRange;
use crate::time_range::resolver::get_start_date;
use crate::github::client::GitHubClient;
use crate::github::Repository;
use crate::cache::postgres::save_repository;
use sqlx::PgPool;
use std::sync::Mutex;

pub struct AppState {
    pub token: Mutex<String>,
    pub etsy_api_key: Mutex<String>,
    pub pool: PgPool,
}

#[command]
pub async fn get_top_repositories(
    range: TimeRange, 
    metric: String, 
    language: Option<String>, 
    keyword: Option<String>,
    page: u32, 
    state: State<'_, AppState>
) -> Result<Vec<Repository>, String> {
    let token = state.token.lock().unwrap().clone();
    if token.is_empty() {
        if let Ok(env_token) = std::env::var("GITHUB_TOKEN") {
             let client = GitHubClient::new(env_token);
             return fetch_top(client, range, metric, language, keyword, page, &state.pool).await;
        }
        return Err("GitHub Token not set. Please set GITHUB_TOKEN environment variable or use the UI.".to_string());
    }
    
    let client = GitHubClient::new(token);
    fetch_top(client, range, metric, language, keyword, page, &state.pool).await
}

async fn fetch_top(
    client: GitHubClient, 
    range: TimeRange, 
    metric: String, 
    language: Option<String>, 
    keyword: Option<String>,
    page: u32, 
    pool: &PgPool
) -> Result<Vec<Repository>, String> {
    let start_date = get_start_date(range);
    let date_str = start_date.format("%Y-%m-%d").to_string();
    
    let sort_param = match metric.as_str() {
        "Stars" => "stars",
        "Forks" => "forks",
        "Updated" => "updated",
        _ => "stars",
    };
    
    // Construct query components
    let k = keyword.as_deref().unwrap_or("").trim();
    let l = language.as_deref().unwrap_or("").trim();
    
    let lang_filter = if l.is_empty() || l == "All" { 
        String::new() 
    } else { 
        format!("language:{}", l) 
    };

    // Query: "{keyword} pushed:>{date} stars:>10 {lang} sort:{sort}"
    let query = format!("{} pushed:>{} stars:>10 {} sort:{}", k, date_str, lang_filter, sort_param);
    
    println!("Executing Top Repos Query (Page {}): '{}'", page, query);

    let repos = client.search_repositories(&query, page).await.map_err(|e| e.to_string())?;
    
    for repo in &repos {
        let _ = save_repository(pool, repo).await.map_err(|e| eprintln!("Failed to save repo: {}", e));
    }
    
    Ok(repos)
}

#[command]
pub async fn get_new_repositories(
    range: TimeRange, 
    metric: Option<String>,
    language: Option<String>, 
    keyword: Option<String>,
    page: u32, 
    state: State<'_, AppState>
) -> Result<Vec<Repository>, String> {
     let token = state.token.lock().unwrap().clone();
    if token.is_empty() {
        if let Ok(env_token) = std::env::var("GITHUB_TOKEN") {
             let client = GitHubClient::new(env_token);
             return fetch_new(client, range, metric, language, keyword, page, &state.pool).await;
        }
        return Err("GitHub Token not set".to_string());
    }

    let client = GitHubClient::new(token);
    fetch_new(client, range, metric, language, keyword, page, &state.pool).await
}

async fn fetch_new(
    client: GitHubClient, 
    range: TimeRange, 
    metric: Option<String>,
    language: Option<String>, 
    keyword: Option<String>,
    page: u32, 
    pool: &PgPool
) -> Result<Vec<Repository>, String> {
    let start_date = get_start_date(range);
    let date_str = start_date.format("%Y-%m-%d").to_string();
    
    let k = keyword.as_deref().unwrap_or("").trim();
    let l = language.as_deref().unwrap_or("").trim();
    
    let lang_filter = if l.is_empty() || l == "All" { 
        String::new() 
    } else { 
        format!("language:{}", l) 
    };

    let sort_param = match metric.as_deref() {
        Some("Forks") => "forks",
        Some("Updated") => "updated",
        _ => "stars", // Default for New is still sorting by stars usually, or created?
        // If I sort by created, I get newest first.
        // If I sort by stars, I get "Newest highly rated".
        // The user request "New Repositories" usually means "Created recently".
        // If I sort by "stars", I see the most popular new ones.
        // If I sort by "updated", I see most recently active new ones.
    };
    
    let query = format!("{} created:>{} stars:>5 {} sort:{}", k, date_str, lang_filter, sort_param);
    
    println!("Executing New Repos Query (Page {}): '{}'", page, query);
    
    let repos = client.search_repositories(&query, page).await.map_err(|e| e.to_string())?;
    
    for repo in &repos {
        let _ = save_repository(pool, repo).await.map_err(|e| eprintln!("Failed to save repo: {}", e));
    }
    
    Ok(repos)
}

#[command]
pub fn refresh_cache() -> Result<(), String> {
    Ok(())
}

#[command]
pub async fn get_repository_readme(owner: String, name: String, state: State<'_, AppState>) -> Result<String, String> {
    let token = state.token.lock().unwrap().clone();
    let client = if token.is_empty() {
        if let Ok(env_token) = std::env::var("GITHUB_TOKEN") {
             GitHubClient::new(env_token)
        } else {
            return Err("GitHub Token not set".to_string());
        }
    } else {
        GitHubClient::new(token)
    };

    client.get_readme(&owner, &name).await.map_err(|e| e.to_string())
}

use crate::commands::export::export_repositories;
#[command]
pub async fn export_repositories_wrapper(repos: Vec<Repository>, format: String) -> Result<String, String> {
    export_repositories(repos, format).await
}