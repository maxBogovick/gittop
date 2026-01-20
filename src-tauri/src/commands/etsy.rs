use tauri::{State, command};
use crate::etsy::client::EtsyClient;
use crate::etsy::EtsyListing;
use crate::commands::analytics::AppState;
use crate::cache::postgres::save_etsy_listing;
use sqlx::PgPool;

#[command]
pub async fn get_etsy_top(
    limit: Option<u32>,
    offset: Option<u32>,
    state: State<'_, AppState>
) -> Result<Vec<EtsyListing>, String> {
    // We no longer require an API key, but we pass a dummy or existing one to the client constructor
    let api_key = state.etsy_api_key.lock().unwrap().clone();
    let client = EtsyClient::new(api_key);
    fetch_listings(client, None, "score", "desc", limit, offset, &state.pool).await
}

#[command]
pub async fn get_etsy_new(
    limit: Option<u32>,
    offset: Option<u32>,
    state: State<'_, AppState>
) -> Result<Vec<EtsyListing>, String> {
    let api_key = state.etsy_api_key.lock().unwrap().clone();
    let client = EtsyClient::new(api_key);
    // Map "new" to Etsy's "date_desc" logic in client
    fetch_listings(client, None, "created", "desc", limit, offset, &state.pool).await
}

#[command]
pub async fn search_etsy(
    query: String,
    limit: Option<u32>,
    offset: Option<u32>,
    state: State<'_, AppState>
) -> Result<Vec<EtsyListing>, String> {
    let api_key = state.etsy_api_key.lock().unwrap().clone();
    let client = EtsyClient::new(api_key);
    fetch_listings(client, Some(&query), "score", "desc", limit, offset, &state.pool).await
}

async fn fetch_listings(
    client: EtsyClient,
    query: Option<&str>,
    sort_on: &str,
    sort_order: &str,
    limit: Option<u32>,
    offset: Option<u32>,
    pool: &PgPool
) -> Result<Vec<EtsyListing>, String> {
    let lim = limit.unwrap_or(30);
    let off = offset.unwrap_or(0);
    
    let listings = client.get_listings(query, Some(sort_on), Some(sort_order), lim, off)
        .await
        .map_err(|e| e.to_string())?;

    for listing in &listings {
        let _ = save_etsy_listing(pool, listing).await;
    }

    Ok(listings)
}