use sqlx::PgPool;
use crate::commands::search::UnifiedSearchResult;
use chrono::{Utc, Duration};
use anyhow::Result;
use sqlx::Row;

pub async fn get_cached_results(pool: &PgPool, query_hash: &str) -> Result<Option<Vec<UnifiedSearchResult>>> {
    // Check request existence and freshness
    let row = sqlx::query(
        "SELECT id, updated_at FROM search_requests WHERE query_hash = $1"
    )
    .bind(query_hash)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        let id: i32 = row.get("id");
        let updated_at: chrono::DateTime<Utc> = row.get("updated_at");
        let now = Utc::now();

        // 5 minutes TTL
        if updated_at > now - Duration::minutes(5) {
            println!("Cache HIT for {}", query_hash);
            let rows = sqlx::query(
                "SELECT source, category, external_id, title, description, url, author, points, comment_count, created_at, tags 
                 FROM search_results WHERE request_id = $1"
            )
            .bind(id)
            .fetch_all(pool)
            .await?;

            let results = rows.into_iter().map(|r| UnifiedSearchResult {
                source: r.get("source"),
                category: r.get("category"),
                id: r.get("external_id"),
                title: r.get("title"),
                url: r.get("url"),
                description: r.get("description"),
                author: r.get("author"),
                points: r.get("points"),
                comment_count: r.get("comment_count"),
                created_at: r.get("created_at"),
                tags: r.get("tags"),
            }).collect();

            return Ok(Some(results));
        } else {
            println!("Cache EXPIRED for {}", query_hash);
        }
    } else {
        println!("Cache MISS for {}", query_hash);
    }

    Ok(None)
}

pub async fn save_search_results(pool: &PgPool, query_hash: &str, query_text: &str, results: &[UnifiedSearchResult]) -> Result<()> {
    let mut tx = pool.begin().await?;

    // Upsert request
    let row = sqlx::query(
        "INSERT INTO search_requests (query_hash, query_text, updated_at) 
         VALUES ($1, $2, NOW()) 
         ON CONFLICT (query_hash) DO UPDATE SET updated_at = NOW() 
         RETURNING id"
    )
    .bind(query_hash)
    .bind(query_text)
    .fetch_one(&mut *tx)
    .await?;

    let request_id: i32 = row.get("id");

    // Delete old results
    sqlx::query("DELETE FROM search_results WHERE request_id = $1")
        .bind(request_id)
        .execute(&mut *tx)
        .await?;

    // Insert new results
    for res in results {
        sqlx::query(
            "INSERT INTO search_results (
                request_id, source, category, external_id, title, description, url, 
                author, points, comment_count, created_at, tags
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"
        )
        .bind(request_id)
        .bind(&res.source)
        .bind(&res.category)
        .bind(&res.id)
        .bind(&res.title)
        .bind(&res.description)
        .bind(&res.url)
        .bind(&res.author)
        .bind(res.points)
        .bind(res.comment_count)
        .bind(&res.created_at)
        .bind(&res.tags)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}
