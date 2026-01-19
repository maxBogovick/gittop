use tauri::{State, command};
use crate::time_range::TimeRange;
use crate::reddit::client::RedditClient;
use crate::reddit::{RedditListing, RedditPost};
use crate::cache::postgres::save_reddit_post;
use crate::commands::analytics::AppState;

#[command]
pub async fn get_reddit_top(
    subreddit: Option<String>,
    time_range: TimeRange,
    limit: Option<u32>,
    after: Option<String>,
    state: State<'_, AppState>
) -> Result<RedditListing, String> {
    let client = RedditClient::new();
    let sub = subreddit.unwrap_or_else(|| "all".to_string());
    let time = map_time_range(time_range);
    let lim = limit.unwrap_or(30);

    let listing = client.get_posts(&sub, "top", time, lim, after).await.map_err(|e| e.to_string())?;
    
    // Save to cache
    for post in &listing.posts {
        let _ = save_reddit_post(&state.pool, post).await;
    }

    Ok(listing)
}

#[command]
pub async fn get_reddit_new(
    subreddit: Option<String>,
    limit: Option<u32>,
    after: Option<String>,
    state: State<'_, AppState>
) -> Result<RedditListing, String> {
    let client = RedditClient::new();
    let sub = subreddit.unwrap_or_else(|| "all".to_string());
    let lim = limit.unwrap_or(30);

    // Reddit "new" doesn't strictly support "t=" but sometimes it does. Usually it's just /new.
    let listing = client.get_posts(&sub, "new", "all", lim, after).await.map_err(|e| e.to_string())?;

    for post in &listing.posts {
        let _ = save_reddit_post(&state.pool, post).await;
    }

    Ok(listing)
}

#[command]
pub async fn search_reddit(
    query: String,
    time_range: TimeRange,
    limit: Option<u32>,
    after: Option<String>,
    state: State<'_, AppState>
) -> Result<RedditListing, String> {
    let client = RedditClient::new();
    let time = map_time_range(time_range);
    let lim = limit.unwrap_or(30);

    let listing = client.search(&query, "relevance", time, lim, after).await.map_err(|e| e.to_string())?;

    for post in &listing.posts {
        let _ = save_reddit_post(&state.pool, post).await;
    }

    Ok(listing)
}

fn map_time_range(range: TimeRange) -> &'static str {
    match range {
        TimeRange::Day => "day",
        TimeRange::Week => "week",
        TimeRange::Month => "month",
        TimeRange::Year => "year",
        // Fallback for others if any
    }
}
