use crate::devto::{client::DevtoClient, DevtoArticle};

// Command to fetch articles list
#[tauri::command]
pub async fn get_devto_articles(
    tag: Option<String>,
    top: Option<i32>,
    state: Option<String>,
    page: u32,
) -> Result<Vec<DevtoArticle>, String> {
    let client = DevtoClient::new();
    // Default per_page to 30 to match UI
    client.get_articles(tag.as_deref(), top, state.as_deref(), page, 30).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_devto_article_details(id: u64) -> Result<DevtoArticle, String> {
    let client = DevtoClient::new();
    client.get_article_details(id).await.map_err(|e| e.to_string())
}
