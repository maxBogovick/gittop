pub mod client;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyListing {
    pub listing_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub price: Option<EtsyPrice>,
    pub num_favorers: Option<u64>,
    pub creation_tsz: Option<i64>, // epoch seconds
    pub tags: Option<Vec<String>>,
    pub images: Option<Vec<EtsyImage>>, 
    pub shop: Option<EtsyShop>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyPrice {
    pub amount: u64,
    pub divisor: u64,
    pub currency_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyImage {
    pub url_75x75: Option<String>,
    pub url_170x135: Option<String>,
    #[serde(rename = "url_570xN")]
    pub url_570x_n: Option<String>,
    pub url_fullxfull: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyShop {
    pub shop_id: u64,
    pub shop_name: String,
}

pub use client::EtsyClient;