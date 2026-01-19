use reqwest::header;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};
use serde_json::Value;

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
    // We try to capture images if available, or just the first one
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

pub struct EtsyClient {
    client: reqwest::Client,
}

impl EtsyClient {
    pub fn new(api_key: String) -> Self {
        let mut headers = header::HeaderMap::new();
        if let Ok(val) = header::HeaderValue::from_str(&api_key) {
            headers.insert("x-api-key", val);
        }
        
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_default();

        Self { client }
    }

    /// Fetch active listings with optional keyword search and sorting.
    /// This uses the Open API v3 endpoint for active listings.
    pub async fn get_listings(
        &self, 
        keywords: Option<&str>, 
        sort_on: Option<&str>, 
        sort_order: Option<&str>, 
        limit: u32, 
        offset: u32
    ) -> Result<Vec<EtsyListing>> {
        let url = "https://openapi.etsy.com/v3/application/listings/active";
        
        let mut req = self.client.get(url)
            .query(&[("limit", limit.to_string()), ("offset", offset.to_string())]);

        if let Some(k) = keywords {
            if !k.is_empty() {
                req = req.query(&[("keywords", k)]);
            }
        }

        if let Some(s) = sort_on {
            req = req.query(&[("sort_on", s)]);
        }
        
        if let Some(o) = sort_order {
            req = req.query(&[("sort_order", o)]);
        }

        // Attempt to request includes if supported by the specific endpoint variant or proxy
        req = req.query(&[("includes", "Images,Shop")]);

        let resp = req.send().await?;

        if !resp.status().is_success() {
             return Err(anyhow::anyhow!("Etsy API Error: {}", resp.status()));
        }

        let json: Value = resp.json().await?;
        let results = json["results"].as_array().context("No results found in Etsy response")?;

        let mut listings = Vec::new();
        for item in results {
            // Parse basic fields
            let listing_id = item["listing_id"].as_u64().unwrap_or_default();
            let title = item["title"].as_str().unwrap_or_default().to_string();
            let description = item["description"].as_str().map(|s| s.to_string());
            let url = item["url"].as_str().unwrap_or_default().to_string();
            
            // Parse price
            let price = if let Some(p) = item.get("price") {
                Some(EtsyPrice {
                    amount: p["amount"].as_u64().unwrap_or_default(),
                    divisor: p["divisor"].as_u64().unwrap_or(100),
                    currency_code: p["currency_code"].as_str().unwrap_or("USD").to_string(),
                })
            } else {
                None
            };

            let num_favorers = item["num_favorers"].as_u64();
            let creation_tsz = item["creation_tsz"].as_i64(); // v3 might use created_timestamp? Checking both
            let created = if creation_tsz.unwrap_or(0) > 0 { creation_tsz } else { item["created_timestamp"].as_i64() };

            let tags = item["tags"].as_array().map(|arr| {
                arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
            });

            // Parse Images
            let images = item["images"].as_array().map(|arr| {
                arr.iter().map(|img| EtsyImage {
                    url_75x75: img["url_75x75"].as_str().map(|s| s.to_string()),
                    url_170x135: img["url_170x135"].as_str().map(|s| s.to_string()),
                    url_570x_n: img["url_570xN"].as_str().map(|s| s.to_string()),
                    url_fullxfull: img["url_fullxfull"].as_str().map(|s| s.to_string()),
                }).collect()
            });

            // Parse Shop
            let shop = if let Some(s) = item.get("shop") {
                Some(EtsyShop {
                    shop_id: s["shop_id"].as_u64().unwrap_or_default(),
                    shop_name: s["shop_name"].as_str().unwrap_or("Unknown").to_string(),
                })
            } else {
                None
            };

            listings.push(EtsyListing {
                listing_id,
                title,
                description,
                url,
                price,
                num_favorers,
                creation_tsz: created,
                tags,
                images,
                shop,
            });
        }

        Ok(listings)
    }
}
