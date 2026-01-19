use crate::hackernews::{client::HackerNewsClient, HackerNewsStory};

#[tauri::command]
pub async fn get_hackernews_stories(
    story_type: Option<String>,
    page: u32,
) -> Result<Vec<HackerNewsStory>, String> {
    let client = HackerNewsClient::new();
    let type_param = story_type.unwrap_or_else(|| "top".to_string());
    client.get_stories(&type_param, page as usize, 30).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_hackernews(
    query: String,
    sort: Option<String>,
    time_range: Option<String>,
    page: u32,
) -> Result<Vec<HackerNewsStory>, String> {
    let client = HackerNewsClient::new();
    let sort_param = sort.unwrap_or_else(|| "popularity".to_string());
    client.search_stories(&query, &sort_param, time_range.as_deref(), page as usize)
        .await
        .map_err(|e| e.to_string())
}
