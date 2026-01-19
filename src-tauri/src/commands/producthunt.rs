use crate::producthunt::{client::ProductHuntClient, ProductHuntPost};
use crate::commands::analytics::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_producthunt_posts(state: State<'_, AppState>) -> Result<Vec<ProductHuntPost>, String> {
    let token = std::env::var("PRODUCT_HUNT_TOKEN").unwrap_or_default();
    let client = ProductHuntClient::new(token);
    client.get_top_posts().await.map_err(|e| e.to_string())
}
