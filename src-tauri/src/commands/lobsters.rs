use crate::lobsters::{client::LobstersClient, LobstersStory};

#[tauri::command]
pub async fn get_lobsters_stories(story_type: Option<String>) -> Result<Vec<LobstersStory>, String> {
    let client = LobstersClient::new();
    client.get_stories(&story_type.unwrap_or_else(|| "hottest".to_string())).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_lobsters_story_details(short_id: String) -> Result<LobstersStory, String> {
    let client = LobstersClient::new();
    client.get_story_details(&short_id).await.map_err(|e| e.to_string())
}
