use crate::hashnode::{client::HashnodeClient, HashnodePost};

#[tauri::command]
pub async fn get_hashnode_posts(
    tag: Option<String>,
    feed_type: Option<String>,
    after: Option<String>,
) -> Result<Vec<HashnodePost>, String> {
    let client = HashnodeClient::new();
    client.get_feed(tag.as_deref(), feed_type.as_deref(), after.as_deref()).await.map_err(|e| e.to_string())
}
