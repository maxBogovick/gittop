use crate::medium::MediumArticle;
use anyhow::Result;
use rss::Channel;
use std::io::Cursor;

pub struct MediumClient {
    client: reqwest::Client,
}

impl MediumClient {
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT, 
            reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_default();
        Self { client }
    }

    pub async fn get_articles(&self, tag: Option<&str>) -> Result<Vec<MediumArticle>> {
        let tag_str = tag.unwrap_or("technology");
        let url = format!("https://medium.com/feed/tag/{}", tag_str);

        let bytes = self.client.get(&url).send().await?.bytes().await?;
        let channel = Channel::read_from(Cursor::new(bytes))?;

        let articles = channel.items().iter().map(|item| {
            let content = item.content().unwrap_or_else(|| item.description().unwrap_or("")).to_string();
            let thumbnail = self.extract_thumbnail(&content);
            
            let author = item.author().map(|a| a.to_string())
                .or_else(|| item.dublin_core_ext().and_then(|dc| dc.creators().first().map(|c| c.to_string())))
                .unwrap_or_else(|| "Unknown".to_string());
            
            MediumArticle {
                title: item.title().unwrap_or("Untitled").to_string(),
                link: item.link().unwrap_or("").to_string(),
                author,
                pub_date: item.pub_date().unwrap_or("").to_string(),
                categories: item.categories().iter().map(|c| c.name().to_string()).collect(),
                content,
                thumbnail,
            }
        }).collect();

        Ok(articles)
    }

    fn extract_thumbnail(&self, content: &str) -> Option<String> {
        // Very basic extraction of first img src
        if let Some(start) = content.find("<img") {
            if let Some(src_start) = content[start..].find("src=\"") {
                let actual_start = start + src_start + 5;
                if let Some(src_end) = content[actual_start..].find("\"") {
                    return Some(content[actual_start..actual_start + src_end].to_string());
                }
            }
        }
        None
    }
}
