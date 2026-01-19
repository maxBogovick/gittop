use crate::medium::{client::MediumClient, MediumArticle};

#[tauri::command]
pub async fn get_medium_articles(
    tag: Option<String>,
) -> Result<Vec<MediumArticle>, String> {
    let client = MediumClient::new();
    client.get_articles(tag.as_deref()).await.map_err(|e| e.to_string())
}
