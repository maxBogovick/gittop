use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Row;
use crate::github::Repository;
use crate::etsy::EtsyListing;
use anyhow::{Result, Context};
use std::env;

pub async fn init_pool() -> Result<PgPool> {
    // dotenv is loaded in lib.rs

    let database_url = env::var("DATABASE_URL")
        .context("DATABASE_URL must be set in .env")?;
    
    println!("Connecting to database: {}", database_url.split('@').nth(1).unwrap_or("..."));

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("Failed to connect to Postgres")?;

    // Drop table to ensure schema update (Development only approach)
    sqlx::query("DROP TABLE IF EXISTS repositories").execute(&pool).await?;
    sqlx::query("DROP TABLE IF EXISTS reddit_posts").execute(&pool).await?;
    sqlx::query("DROP TABLE IF EXISTS etsy_listings").execute(&pool).await?;
    // We intentionally don't drop search cache tables to persist them, or do we? 
    // For now, let's keep them persistent.

    // Enable pg_trgm for fuzzy search
    sqlx::query("CREATE EXTENSION IF NOT EXISTS pg_trgm").execute(&pool).await?;

    // Create Search Tables
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS search_requests (
            id SERIAL PRIMARY KEY,
            query_hash TEXT NOT NULL UNIQUE,
            query_text TEXT NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS search_results (
            id SERIAL PRIMARY KEY,
            request_id INTEGER NOT NULL REFERENCES search_requests(id) ON DELETE CASCADE,
            source TEXT NOT NULL,
            category TEXT NOT NULL,
            external_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            url TEXT NOT NULL,
            author TEXT,
            points BIGINT,
            comment_count BIGINT,
            created_at TEXT,
            tags TEXT[],
            tsv TSVECTOR
        )"
    )
    .execute(&pool)
    .await?;

    // Function to update tsv
    sqlx::query(
        "CREATE OR REPLACE FUNCTION search_results_tsv_update() RETURNS trigger AS $$
        BEGIN
            NEW.tsv :=
                setweight(to_tsvector('english', coalesce(NEW.title, '')), 'A') ||
                setweight(to_tsvector('english', coalesce(NEW.description, '')), 'B') ||
                setweight(to_tsvector('english', array_to_string(NEW.tags, ' ')), 'C');
            RETURN NEW;
        END
        $$ LANGUAGE plpgsql"
    )
    .execute(&pool)
    .await?;

    // Trigger
    sqlx::query(
        "DROP TRIGGER IF EXISTS tsvectorupdate ON search_results"
    ).execute(&pool).await?;

    sqlx::query(
        "CREATE TRIGGER tsvectorupdate BEFORE INSERT OR UPDATE
        ON search_results FOR EACH ROW EXECUTE FUNCTION search_results_tsv_update()"
    )
    .execute(&pool)
    .await?;

    // Indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_search_results_tsv ON search_results USING GIN(tsv)").execute(&pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_search_results_title_trgm ON search_results USING GIN(title gin_trgm_ops)").execute(&pool).await?;

    // Create table if not exists
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS repositories (
            owner TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            primary_language TEXT,
            created_at TIMESTAMPTZ NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL,
            stars_total BIGINT NOT NULL,
            stars_delta BIGINT NOT NULL,
            forks BIGINT NOT NULL,
            open_issues BIGINT NOT NULL,
            pull_requests_count BIGINT NOT NULL,
            comments_count BIGINT NOT NULL,
            releases_count BIGINT NOT NULL,
            last_activity_at TIMESTAMPTZ,
            repository_url TEXT NOT NULL,
            owner_avatar_url TEXT,
            license TEXT,
            UNIQUE(owner, name)
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS reddit_posts (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            subreddit TEXT NOT NULL,
            score BIGINT NOT NULL,
            num_comments BIGINT NOT NULL,
            created_utc TIMESTAMPTZ NOT NULL,
            url TEXT NOT NULL,
            selftext TEXT,
            permalink TEXT NOT NULL,
            thumbnail TEXT
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS etsy_listings (
            listing_id BIGINT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            url TEXT NOT NULL,
            price_amount BIGINT,
            price_divisor BIGINT,
            price_currency TEXT,
            num_favorers BIGINT,
            creation_tsz BIGINT,
            shop_name TEXT,
            thumbnail_url TEXT
        )"
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

pub async fn save_repository(pool: &PgPool, repo: &Repository) -> Result<()> {
    sqlx::query(
        "INSERT INTO repositories (
            owner, name, description, primary_language, created_at, updated_at,
            stars_total, stars_delta, forks, open_issues, pull_requests_count,
            comments_count, releases_count, last_activity_at, repository_url, owner_avatar_url, license
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        ON CONFLICT (owner, name) DO UPDATE SET
            description = EXCLUDED.description,
            primary_language = EXCLUDED.primary_language,
            created_at = EXCLUDED.created_at,
            updated_at = EXCLUDED.updated_at,
            stars_total = EXCLUDED.stars_total,
            stars_delta = EXCLUDED.stars_delta,
            forks = EXCLUDED.forks,
            open_issues = EXCLUDED.open_issues,
            pull_requests_count = EXCLUDED.pull_requests_count,
            comments_count = EXCLUDED.comments_count,
            releases_count = EXCLUDED.releases_count,
            last_activity_at = EXCLUDED.last_activity_at,
            repository_url = EXCLUDED.repository_url,
            owner_avatar_url = EXCLUDED.owner_avatar_url,
            license = EXCLUDED.license"
    )
    .bind(&repo.owner)
    .bind(&repo.name)
    .bind(&repo.description)
    .bind(&repo.primary_language)
    .bind(repo.created_at)
    .bind(repo.updated_at)
    .bind(repo.stars_total)
    .bind(repo.stars_delta)
    .bind(repo.forks)
    .bind(repo.open_issues)
    .bind(repo.pull_requests_count)
    .bind(repo.comments_count)
    .bind(repo.releases_count)
    .bind(repo.last_activity_at)
    .bind(&repo.repository_url)
    .bind(&repo.owner_avatar_url)
    .bind(&repo.license)
    .execute(pool)
    .await?;
    
    Ok(())
}

use crate::reddit::RedditPost;

pub async fn save_reddit_post(pool: &PgPool, post: &RedditPost) -> Result<()> {
    sqlx::query(
        "INSERT INTO reddit_posts (
            id, title, author, subreddit, score, num_comments, created_utc, url, selftext, permalink, thumbnail
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ON CONFLICT (id) DO UPDATE SET
            title = EXCLUDED.title,
            score = EXCLUDED.score,
            num_comments = EXCLUDED.num_comments,
            selftext = EXCLUDED.selftext,
            thumbnail = EXCLUDED.thumbnail"
    )
    .bind(&post.id)
    .bind(&post.title)
    .bind(&post.author)
    .bind(&post.subreddit)
    .bind(post.score)
    .bind(post.num_comments)
    .bind(post.created_utc)
    .bind(&post.url)
    .bind(&post.selftext)
    .bind(&post.permalink)
    .bind(&post.thumbnail)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn save_etsy_listing(pool: &PgPool, listing: &EtsyListing) -> Result<()> {
    // Extract thumbnail from first image if available
    let thumbnail = listing.images.as_ref()
        .and_then(|imgs| imgs.first())
        .and_then(|img| img.url_170x135.clone());

    let (price_amount, price_divisor, price_currency) = if let Some(p) = &listing.price {
        (Some(p.amount as i64), Some(p.divisor as i64), Some(p.currency_code.clone()))
    } else {
        (None, None, None)
    };
    
    let shop_name = listing.shop.as_ref().map(|s| s.shop_name.clone());

    sqlx::query(
        "INSERT INTO etsy_listings (
            listing_id, title, description, url, price_amount, price_divisor, price_currency,
            num_favorers, creation_tsz, shop_name, thumbnail_url
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ON CONFLICT (listing_id) DO UPDATE SET
            title = EXCLUDED.title,
            description = EXCLUDED.description,
            price_amount = EXCLUDED.price_amount,
            price_divisor = EXCLUDED.price_divisor,
            price_currency = EXCLUDED.price_currency,
            num_favorers = EXCLUDED.num_favorers,
            thumbnail_url = EXCLUDED.thumbnail_url"
    )
    .bind(listing.listing_id as i64)
    .bind(&listing.title)
    .bind(&listing.description)
    .bind(&listing.url)
    .bind(price_amount)
    .bind(price_divisor)
    .bind(price_currency)
    .bind(listing.num_favorers.map(|v| v as i64))
    .bind(listing.creation_tsz.map(|v| v as i64))
    .bind(shop_name)
    .bind(thumbnail)
    .execute(pool)
    .await?;
    
    Ok(())
}


pub async fn get_all_repositories(pool: &PgPool) -> Result<Vec<Repository>> {
    let rows = sqlx::query(
        "SELECT 
        owner, name, description, primary_language, created_at, updated_at,
        stars_total, stars_delta, forks, open_issues, pull_requests_count,
        comments_count, releases_count, last_activity_at, repository_url, owner_avatar_url, license
        FROM repositories"
    )
    .fetch_all(pool)
    .await?;

    let mut repos = Vec::new();
    for row in rows {
        repos.push(Repository {
            owner: row.get("owner"),
            name: row.get("name"),
            description: row.get("description"),
            primary_language: row.get("primary_language"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            stars_total: row.get("stars_total"),
            stars_delta: row.get("stars_delta"),
            forks: row.get("forks"),
            open_issues: row.get("open_issues"),
            pull_requests_count: row.get("pull_requests_count"),
            comments_count: row.get("comments_count"),
            releases_count: row.get("releases_count"),
            last_activity_at: row.get("last_activity_at"),
            repository_url: row.get("repository_url"),
            owner_avatar_url: row.get("owner_avatar_url"),
            license: row.get("license"),
        });
    }

    Ok(repos)
}