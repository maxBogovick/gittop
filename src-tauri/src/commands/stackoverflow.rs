use crate::stackoverflow::{client::StackOverflowClient, StackOverflowQuestion};

#[tauri::command]
pub async fn get_stackoverflow_questions(
    tag: Option<String>,
    sort: Option<String>,
    page: u32,
) -> Result<Vec<StackOverflowQuestion>, String> {
    let client = StackOverflowClient::new();
    let sort_param = sort.unwrap_or_else(|| "votes".to_string());
    // Default per_page 30
    client.get_questions(tag.as_deref(), &sort_param, page, 30).await.map_err(|e| e.to_string())
}
