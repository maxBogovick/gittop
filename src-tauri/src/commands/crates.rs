use crate::crates::{client::CratesClient, Crate};

#[tauri::command]
pub async fn get_crates(query: Option<String>, sort: Option<String>, page: u32) -> Result<Vec<Crate>, String> {
    let client = CratesClient::new();
    client.get_crates(query.as_deref(), &sort.unwrap_or_else(|| "downloads".to_string()), page).await.map_err(|e| e.to_string())
}
