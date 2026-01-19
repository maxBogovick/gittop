use crate::lobsters::{client::LobstersClient, LobstersStory};

#[tauri::command]
pub async fn get_lobsters_stories(story_type: Option<String>) -> Result<Vec<LobstersStory>, String> {
    let client = LobstersClient::new();
    client.get_stories(&story_type.unwrap_or_else(|| "hottest".to_string())).await.map_err(|e| e.to_string())
}
