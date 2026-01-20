mod github;
mod reddit;
mod etsy;
mod commands;
mod cache;
mod metrics;
mod time_range;
mod devto;
mod stackoverflow;
mod hackernews;
mod medium;
mod hashnode;
mod producthunt;
mod lobsters;
mod crates;
mod indiehackers;

use commands::analytics::{get_top_repositories, get_new_repositories, refresh_cache, get_repository_readme, AppState};
use commands::export::export_repositories;
use commands::reddit::{get_reddit_top, get_reddit_new, search_reddit};
use commands::etsy::{get_etsy_top, get_etsy_new, search_etsy};
use commands::devto::{get_devto_articles, get_devto_article_details};
use commands::stackoverflow::get_stackoverflow_questions;
use commands::hackernews::{get_hackernews_stories, search_hackernews};
use commands::medium::get_medium_articles;
use commands::hashnode::get_hashnode_posts;
use commands::producthunt::get_producthunt_posts;
use commands::lobsters::{get_lobsters_stories, get_lobsters_story_details};
use commands::crates::get_crates;
use commands::indiehackers::get_indiehackers_posts;
use std::sync::Mutex;
use tauri::Manager;
use dotenv::dotenv;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Starting application...");
    // Load environment variables from .env file
    dotenv().ok();
    println!("Loaded .env file");

    let pool = tauri::async_runtime::block_on(async {
        println!("Initializing database pool...");
        match cache::postgres::init_pool().await {
            Ok(p) => {
                println!("Database pool initialized successfully");
                p
            },
            Err(e) => {
                eprintln!("Failed to initialize database: {:?}", e);
                panic!("Failed to initialize database");
            }
        }
    });

    let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
    println!("GITHUB_TOKEN loaded (length: {})", token.len());

    let etsy_key = std::env::var("ETSY_API_KEY").unwrap_or_default();
    println!("ETSY_API_KEY loaded (length: {})", etsy_key.len());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            println!("Tauri setup callback");
            app.manage(AppState { 
                token: Mutex::new(token),
                etsy_api_key: Mutex::new(etsy_key),
                pool 
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_top_repositories,
            get_new_repositories,
            refresh_cache,
            get_repository_readme,
            export_repositories,
            get_reddit_top,
            get_reddit_new,
            search_reddit,
            get_etsy_top,
            get_etsy_new,
            search_etsy,
            get_devto_articles,
            get_devto_article_details,
            get_stackoverflow_questions,
            get_hackernews_stories,
            search_hackernews,
            get_medium_articles,
            get_hashnode_posts,
            get_producthunt_posts,
            get_lobsters_stories,
            get_lobsters_story_details,
            get_crates,
            get_indiehackers_posts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
