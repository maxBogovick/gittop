use tauri::{State, command};
use serde::{Serialize, Deserialize};
use crate::commands::analytics::AppState;
use crate::github::client::GitHubClient;
use crate::reddit::client::RedditClient;
use crate::stackoverflow::client::StackOverflowClient;
use crate::hackernews::client::HackerNewsClient;
use crate::devto::client::DevtoClient;
use crate::crates::client::CratesClient;
use crate::etsy::client::EtsyClient;
use crate::medium::client::MediumClient;
use crate::hashnode::client::HashnodeClient;
use crate::producthunt::client::ProductHuntClient;
use crate::lobsters::client::LobstersClient;
use crate::indiehackers::client::IndieHackersClient;
use chrono::{Utc, TimeZone, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnifiedSearchResult {
    pub source: String, 
    pub category: String, // "Code", "Discussion", "Article", "Product"
    pub id: String,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub points: Option<i64>, 
    pub comment_count: Option<i64>,
    pub created_at: Option<String>, 
    pub tags: Vec<String>,
}

#[command]
pub async fn search_all(
    query: String, 
    sources: Option<Vec<String>>,
    time_filter: Option<String>, 
    sort_by: Option<String>,
    page: Option<u32>,
    state: State<'_, AppState>
) -> Result<Vec<UnifiedSearchResult>, String> {
    let page = page.unwrap_or(1);
    println!("Global Search: '{}' Sources: {:?} Time: {:?} Sort: {:?} Page: {}", query, sources, time_filter, sort_by, page);
    let mut results = Vec::new();

    let enabled_sources: std::collections::HashSet<String> = sources
        .unwrap_or_else(|| vec![
            "GitHub".to_string(), 
            "Reddit".to_string(), 
            "StackOverflow".to_string(), 
            "HackerNews".to_string(), 
            "Dev.to".to_string(),
            "Crates.io".to_string(),
            "Etsy".to_string(),
            "Medium".to_string(),
            "Hashnode".to_string(),
            "ProductHunt".to_string(),
            "Lobsters".to_string(),
            "IndieHackers".to_string(),
        ])
        .into_iter()
        .map(|s| s.trim().to_string())
        .collect();
    
    let is_enabled = |s: &str| enabled_sources.contains(s);

    // --- Clients ---
    let github_token = state.token.lock().unwrap().clone();
    let github_client = if is_enabled("GitHub") {
        if !github_token.is_empty() {
            Some(GitHubClient::new(github_token))
        } else {
            std::env::var("GITHUB_TOKEN").ok().map(GitHubClient::new)
        }
    } else { None };
    
    let reddit_client = if is_enabled("Reddit") && page == 1 { Some(RedditClient::new()) } else { None };
    let so_client = if is_enabled("StackOverflow") { Some(StackOverflowClient::new()) } else { None };
    let hn_client = if is_enabled("HackerNews") { Some(HackerNewsClient::new()) } else { None };
    let devto_client = if is_enabled("Dev.to") { Some(DevtoClient::new()) } else { None };
    
    let crates_client = if is_enabled("Crates.io") { Some(CratesClient::new()) } else { None };
    
    let etsy_key = state.etsy_api_key.lock().unwrap().clone();
    let etsy_client = if is_enabled("Etsy") { 
         // Verify we have a key or env var? Client builder usually handles it but we passed it in constructor in our implementation?
         // In `etsy/client.rs`: `pub fn new(_api_key: String)`. It ignores it but takes it.
         Some(EtsyClient::new(etsy_key)) 
    } else { None };

    let medium_client = if is_enabled("Medium") && page == 1 { Some(MediumClient::new()) } else { None };
    let hashnode_client = if is_enabled("Hashnode") && page == 1 { Some(HashnodeClient::new()) } else { None };
    
    let ph_token = std::env::var("PRODUCT_HUNT_TOKEN").unwrap_or_default();
    let ph_client = if is_enabled("ProductHunt") && page == 1 { Some(ProductHuntClient::new(ph_token)) } else { None };
    
    let lobsters_client = if is_enabled("Lobsters") && page == 1 { Some(LobstersClient::new()) } else { None };
    let indie_client = if is_enabled("IndieHackers") && page == 1 { Some(IndieHackersClient::new()) } else { None };


    // --- Helpers ---
    let time_str = time_filter.as_deref().unwrap_or("all");
    let sort_str = sort_by.as_deref().unwrap_or("relevance");

    // --- Tasks ---

    // GitHub
    let q_gh = query.clone();
    let gh_sort = match sort_str { "date" => "updated", "rating" => "stars", _ => "" };
    let mut gh_query = q_gh.clone();
    if time_str != "all" {
        let now = Utc::now();
        let date = match time_str {
            "day" => now - Duration::days(1),
            "week" => now - Duration::weeks(1),
            "month" => now - Duration::days(30),
            "year" => now - Duration::days(365),
            _ => now - Duration::days(3650),
        };
        gh_query = format!("{} pushed:>{}", gh_query, date.format("%Y-%m-%d"));
    }
    if !gh_sort.is_empty() { gh_query = format!("{} sort:{}", gh_query, gh_sort); }
    
    let github_task = tokio::spawn(async move {
        if let Some(client) = github_client {
            match client.search_repositories(&gh_query, page).await {
                Ok(repos) => repos.into_iter().map(|r| UnifiedSearchResult {
                    source: "GitHub".to_string(),
                    category: "Code".to_string(),
                    id: format!("{}/{}", r.owner, r.name),
                    title: format!("{}/{}", r.owner, r.name),
                    url: r.repository_url,
                    description: r.description,
                    author: Some(r.owner),
                    points: Some(r.stars_total),
                    comment_count: Some(r.forks), 
                    created_at: Some(r.created_at.to_string()), 
                    tags: vec![r.primary_language.unwrap_or_default()],
                }).collect(),
                Err(e) => { eprintln!("GitHub Search Error: {}", e); vec![] }
            }
        } else { vec![] }
    });

    // Reddit (Page 1 only)
    let q_red = query.clone();
    let red_sort = match sort_str { "date" => "new", "rating" => "top", _ => "relevance" };
    let red_time = match time_str { "day" => "day", "week" => "week", "month" => "month", "year" => "year", _ => "all" };
    let reddit_task = tokio::spawn(async move {
        if let Some(client) = reddit_client {
            match client.search(&q_red, red_sort, red_time, 25, None).await {
                Ok(listing) => listing.posts.into_iter().map(|p| UnifiedSearchResult {
                    source: "Reddit".to_string(),
                    category: "Discussion".to_string(),
                    id: p.id,
                    title: p.title,
                    url: p.permalink,
                    description: Some(p.selftext.chars().take(200).collect()),
                    author: Some(p.author),
                    points: Some(p.score),
                    comment_count: Some(p.num_comments),
                    created_at: Some(p.created_utc.to_string()),
                    tags: vec![p.subreddit],
                }).collect(),
                Err(e) => { eprintln!("Reddit Search Error: {}", e); vec![] }
            }
        } else { vec![] }
    });

    // StackOverflow
    let q_so = query.clone();
    let so_sort = match sort_str { "date" => "creation", "rating" => "votes", _ => "relevance" };
    let so_task = tokio::spawn(async move {
        if let Some(client) = so_client {
            match client.search(&q_so, so_sort, page, 25).await {
                Ok(questions) => questions.into_iter().map(|q| UnifiedSearchResult {
                    source: "StackOverflow".to_string(),
                    category: "Discussion".to_string(),
                    id: q.question_id.to_string(),
                    title: q.title,
                    url: q.link,
                    description: None, 
                    author: q.owner.display_name,
                    points: Some(q.score),
                    comment_count: Some(q.answer_count),
                    created_at: Some(Utc.timestamp_opt(q.creation_date, 0).unwrap().to_string()),
                    tags: q.tags,
                }).collect(),
                Err(e) => { eprintln!("StackOverflow Search Error: {}", e); vec![] }
            }
        } else { vec![] }
    });

    // HackerNews
    let q_hn = query.clone();
    let hn_sort = match sort_str { "date" => "date", _ => "popularity" };
    let hn_time_str = time_str.to_string();
    let hn_task = tokio::spawn(async move {
        if let Some(client) = hn_client {
            match client.search_stories(&q_hn, hn_sort, Some(&hn_time_str), page as usize).await {
                Ok(stories) => stories.into_iter().map(|s| UnifiedSearchResult {
                    source: "HackerNews".to_string(),
                    category: "Discussion".to_string(),
                    id: s.id.to_string(),
                    title: s.title.unwrap_or_default(),
                    url: s.url.unwrap_or_else(|| format!("https://news.ycombinator.com/item?id={}", s.id)),
                    description: s.text,
                    author: s.by,
                    points: Some(s.score),
                    comment_count: Some(s.descendants),
                    created_at: Some(Utc.timestamp_opt(s.time, 0).unwrap().to_string()),
                    tags: vec![],
                }).collect(),
                Err(e) => { eprintln!("HackerNews Search Error: {}", e); vec![] }
            }
        } else { vec![] }
    });

    // Dev.to
    let q_devto = query.clone();
    let devto_task = tokio::spawn(async move {
        if let Some(client) = devto_client {
            match client.search(&q_devto, page, 15).await {
                Ok(articles) => articles.into_iter().map(|a| UnifiedSearchResult {
                    source: "Dev.to".to_string(),
                    category: "Article".to_string(),
                    id: a.id.to_string(),
                    title: a.title,
                    url: a.url,
                    description: a.description,
                    author: Some(a.user.name),
                    points: Some(a.positive_reactions_count),
                    comment_count: Some(a.comments_count),
                    created_at: Some(a.published_at.to_string()),
                    tags: a.tags,
                }).collect(),
                Err(e) => { eprintln!("Dev.to Search Error: {}", e); vec![] }
            }
        } else { vec![] }
    });

    // Crates.io
    let q_crates = query.clone();
    let crates_sort = match sort_str { "date" => "new", "rating" => "downloads", _ => "relevance" };
    let crates_task = tokio::spawn(async move {
        if let Some(client) = crates_client {
            match client.get_crates(Some(&q_crates), crates_sort, page).await {
                Ok(crates) => crates.into_iter().map(|c| UnifiedSearchResult {
                    source: "Crates.io".to_string(),
                    category: "Code".to_string(),
                    id: c.id.clone(),
                    title: c.name,
                    url: format!("https://crates.io/crates/{}", c.id), // c.id is name?
                    description: c.description,
                    author: None,
                    points: Some(c.downloads as i64),
                    comment_count: None,
                    created_at: Some(c.updated_at),
                    tags: vec![],
                }).collect(),
                Err(e) => { eprintln!("Crates.io Search Error: {}", e); vec![] }
            }
        } else { vec![] }
    });

    // Etsy
    let q_etsy = query.clone();
    let etsy_sort_on = match sort_str { "date" => Some("created"), "rating" => None, _ => None };
    let etsy_order = if sort_str == "date" { Some("desc") } else { None };
    let etsy_offset = (page - 1) * 20; // Default limit 20
    let etsy_task = tokio::spawn(async move {
        if let Some(client) = etsy_client {
            match client.get_listings(Some(&q_etsy), etsy_sort_on, etsy_order, 20, etsy_offset).await {
                Ok(listings) => listings.into_iter().map(|l| UnifiedSearchResult {
                    source: "Etsy".to_string(),
                    category: "Product".to_string(),
                    id: l.listing_id.to_string(),
                    title: l.title,
                    url: l.url,
                    description: l.description,
                    author: l.shop.map(|s| s.shop_name),
                    points: None,
                    comment_count: l.num_favorers.map(|n| n as i64),
                    created_at: None, // creation_tsz not easily available in list
                    tags: vec![],
                }).collect(),
                Err(e) => { eprintln!("Etsy Search Error: {}", e); vec![] }
            }
        } else { vec![] }
    });

    // Medium (Page 1 only, filtered)
    let q_med = query.clone();
    let med_task = tokio::spawn(async move {
        if let Some(client) = medium_client {
            // Treat query as tag if simple? Or just fetch 'technology' tag and filter?
            // If query is a single word, try it as tag.
            let tag = if !q_med.contains(' ') { Some(q_med.as_str()) } else { None };
            match client.get_articles(tag).await {
                Ok(articles) => articles.into_iter()
                    .filter(|a| a.title.to_lowercase().contains(&q_med.to_lowercase()))
                    .map(|a| UnifiedSearchResult {
                        source: "Medium".to_string(),
                        category: "Article".to_string(),
                        id: a.link.clone(),
                        title: a.title,
                        url: a.link,
                        description: Some(a.content.chars().take(200).collect()),
                        author: Some(a.author),
                        points: None,
                        comment_count: None,
                        created_at: Some(a.pub_date),
                        tags: a.categories,
                    }).collect(),
                Err(e) => { eprintln!("Medium Search Error: {}", e); vec![] }
            }
        } else { vec![] }
    });

    // Hashnode (Page 1 only, filtered)
    let q_hash = query.clone();
    let hash_task = tokio::spawn(async move {
        if let Some(client) = hashnode_client {
            // Try query as tag
            let tag = if !q_hash.contains(' ') { Some(q_hash.as_str()) } else { None };
            match client.get_feed(tag, None, None).await {
                Ok(posts) => posts.into_iter()
                    .filter(|p| p.title.to_lowercase().contains(&q_hash.to_lowercase()))
                    .map(|p| UnifiedSearchResult {
                        source: "Hashnode".to_string(),
                        category: "Article".to_string(),
                        id: p.id,
                        title: p.title,
                        url: p.url,
                        description: Some(p.brief),
                        author: Some(p.author.name),
                        points: Some(p.reaction_count as i64),
                        comment_count: Some(p.response_count as i64),
                        created_at: Some(p.published_at),
                        tags: vec![],
                    }).collect(),
                Err(e) => { eprintln!("Hashnode Search Error: {}", e); vec![] }
            }
        } else { vec![] }
    });
    
    // ProductHunt (Page 1 only, filtered)
    let q_ph = query.clone();
    let ph_task = tokio::spawn(async move {
        if let Some(client) = ph_client {
            match client.get_top_posts().await {
                Ok(posts) => posts.into_iter()
                    .filter(|p| p.name.to_lowercase().contains(&q_ph.to_lowercase()) || p.tagline.to_lowercase().contains(&q_ph.to_lowercase()))
                    .map(|p| UnifiedSearchResult {
                        source: "ProductHunt".to_string(),
                        category: "Product".to_string(),
                        id: p.id,
                        title: p.name,
                        url: p.url,
                        description: Some(p.tagline),
                        author: None,
                        points: Some(p.votes_count as i64),
                        comment_count: Some(p.comments_count as i64),
                        created_at: Some(p.created_at),
                        tags: vec![],
                    }).collect(),
                Err(e) => { eprintln!("ProductHunt Search Error: {}", e); vec![] }
            }
        } else { vec![] }
    });

    // Lobsters (Page 1 only, filtered)
    let q_lob = query.clone();
    let lob_task = tokio::spawn(async move {
        if let Some(client) = lobsters_client {
            match client.get_stories("hottest").await {
                Ok(stories) => stories.into_iter()
                    .filter(|s| s.title.to_lowercase().contains(&q_lob.to_lowercase()))
                    .map(|s| UnifiedSearchResult {
                        source: "Lobsters".to_string(),
                        category: "Discussion".to_string(),
                        id: s.short_id,
                        title: s.title,
                        url: s.url,
                        description: None,
                        author: Some(s.submitter_user),
                        points: Some(s.score as i64),
                        comment_count: Some(s.comment_count as i64),
                        created_at: Some(s.created_at),
                        tags: s.tags,
                    }).collect(),
                Err(e) => { eprintln!("Lobsters Search Error: {}", e); vec![] }
            }
        } else { vec![] }
    });

    // IndieHackers (Page 1 only, filtered)
    let q_indie = query.clone();
    let indie_task = tokio::spawn(async move {
        if let Some(client) = indie_client {
            match client.get_posts().await {
                Ok(posts) => posts.into_iter()
                    .filter(|p| p.title.to_lowercase().contains(&q_indie.to_lowercase()) || p.content.to_lowercase().contains(&q_indie.to_lowercase()))
                    .map(|p| UnifiedSearchResult {
                        source: "IndieHackers".to_string(),
                        category: "Discussion".to_string(),
                        id: p.link.clone(), // Link as ID
                        title: p.title,
                        url: p.link,
                        description: Some(p.content.chars().take(200).collect()),
                        author: Some(p.author),
                        points: None,
                        comment_count: None,
                        created_at: Some(p.pub_date),
                        tags: vec![],
                    }).collect(),
                Err(e) => { eprintln!("IndieHackers Search Error: {}", e); vec![] }
            }
        } else { vec![] }
    });


    // Await all
    let (
        r_gh, r_red, r_so, r_hn, r_dev, 
        r_crates, r_etsy, r_med, r_hash, 
        r_ph, r_lob, r_indie
    ) = tokio::join!(
        github_task, reddit_task, so_task, hn_task, devto_task,
        crates_task, etsy_task, med_task, hash_task,
        ph_task, lob_task, indie_task
    );

    results.extend(r_gh.unwrap_or_default());
    results.extend(r_red.unwrap_or_default());
    results.extend(r_so.unwrap_or_default());
    results.extend(r_hn.unwrap_or_default());
    results.extend(r_dev.unwrap_or_default());
    results.extend(r_crates.unwrap_or_default());
    results.extend(r_etsy.unwrap_or_default());
    results.extend(r_med.unwrap_or_default());
    results.extend(r_hash.unwrap_or_default());
    results.extend(r_ph.unwrap_or_default());
    results.extend(r_lob.unwrap_or_default());
    results.extend(r_indie.unwrap_or_default());

    Ok(results)
}
