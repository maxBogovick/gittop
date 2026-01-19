use crate::indiehackers::{client::IndieHackersClient, IndieHackersPost};

#[tauri::command]
pub async fn get_indiehackers_posts() -> Result<Vec<IndieHackersPost>, String> {
    let client = IndieHackersClient::new();
    client.get_posts().await.map_err(|e| e.to_string())
}
