use reqwest::header;
use scraper::{Html, Selector};
use anyhow::Result;
use crate::etsy::{EtsyListing, EtsyPrice, EtsyImage, EtsyShop};
use regex::Regex;

pub struct EtsyClient {
    client: reqwest::Client,
}

impl EtsyClient {
    pub fn new(_api_key: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT, 
            header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        );
        headers.insert(header::ACCEPT, header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8"));
        headers.insert(header::ACCEPT_LANGUAGE, header::HeaderValue::from_static("en-US,en;q=0.9"));
        
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .cookie_store(true) 
            .build()
            .unwrap_or_default();

        Self { client }
    }

    pub async fn get_listings(
        &self, 
        keywords: Option<&str>, 
        sort_on: Option<&str>, 
        sort_order: Option<&str>, 
        limit: u32, 
        offset: u32
    ) -> Result<Vec<EtsyListing>> {
        let base_url = "https://www.etsy.com/search";
        let page = (offset / limit) + 1;
        
        // Map sort params to Etsy's "order" param
        // Etsy supports: most_relevant, date_desc, price_asc, price_desc
        let order = match (sort_on, sort_order) {
            (Some("created"), Some("desc")) => "date_desc",
            (Some("price"), Some("asc")) => "price_asc",
            (Some("price"), Some("desc")) => "price_desc",
            _ => "most_relevant",
        };

        let query = keywords.unwrap_or("handmade"); // Default to broad search if empty

        let url = format!("{}?q={}&order={}&ref=pagination&page={}", base_url, query, order, page);
        println!("Fetching Etsy URL: {}", url);

        let resp = self.client.get(&url).send().await?;
        
        if !resp.status().is_success() {
             return Err(anyhow::anyhow!("Etsy Scraper Error: {} - Access Denied (Likely Bot Protection)", resp.status()));
        }

        let html_content = resp.text().await?;
        let document = Html::parse_document(&html_content);

        // Selectors
        // Note: These classes are subject to change.
        // We look for the main listing card anchor
        let listing_selector = Selector::parse("a.listing-link").unwrap();
        let title_selector = Selector::parse("h3").unwrap();
        let price_selector = Selector::parse(".currency-value").unwrap();
        let currency_selector = Selector::parse(".currency-symbol").unwrap();
        // let shop_selector = Selector::parse(".wt-text-caption").unwrap(); // Often shop name is in caption
        let img_selector = Selector::parse("img.wt-width-full").unwrap();

        let mut listings = Vec::new();

        for element in document.select(&listing_selector) {
            let url = element.value().attr("href").unwrap_or_default().to_string();
            
            // Extract Listing ID from URL
            // URL format: .../listing/123456789/title...
            let re = Regex::new(r"/listing/(\d+)").unwrap();
            let listing_id = re.captures(&url)
                .and_then(|cap| cap.get(1))
                .map(|m| m.as_str().parse::<u64>().unwrap_or_default())
                .unwrap_or_default();

            if listing_id == 0 { continue; }

            let title = element.select(&title_selector).next()
                .map(|e| e.text().collect::<String>().trim().to_string())
                .unwrap_or_else(|| "Untitled".to_string());

            // Price extraction is tricky as it's often split
            let price_amount_str = element.select(&price_selector).next()
                .map(|e| e.text().collect::<String>().replace(",", ""))
                .unwrap_or_default();
            
            let price_amount = price_amount_str.parse::<f64>().unwrap_or(0.0);
            
            let currency = element.select(&currency_selector).next()
                .map(|e| e.text().collect::<String>().trim().to_string())
                .unwrap_or_else(|| "$".to_string());
            
            let price = Some(EtsyPrice {
                amount: (price_amount * 100.0) as u64,
                divisor: 100,
                currency_code: currency, // Simplified
            });

            // Image
            let img_src = element.select(&img_selector).next()
                .and_then(|e| e.value().attr("src").or(e.value().attr("data-src")))
                .map(|s| s.to_string());

            let images = if let Some(src) = img_src {
                Some(vec![EtsyImage {
                    url_75x75: Some(src.clone()),
                    url_170x135: Some(src.clone()),
                    url_570x_n: Some(src.clone()),
                    url_fullxfull: Some(src),
                }])
            } else {
                None
            };
            
            // Shop Name - highly heuristic
            // Often in a span with class "wt-text-gray" or similar inside the card
            // We verify by checking if it's not the title
            let shop_name = "Etsy Shop".to_string(); // Placeholder or complex parsing required

            listings.push(EtsyListing {
                listing_id,
                title,
                description: None,
                url,
                price,
                num_favorers: None,
                creation_tsz: None, // Hard to scrape from list view
                tags: None,
                images,
                shop: Some(EtsyShop { shop_id: 0, shop_name }),
            });
        }

        // Fallback for when no listings found - maybe parsing failed or layout changed
        if listings.is_empty() {
             println!("No listings parsed. HTML preview: {:.500}", html_content);
        }

        Ok(listings)
    }
}
