use sqlx::{PgPool, Row};
use serde::{Deserialize, Serialize};

/// Link struct representing a shortened URL in the database
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Link {
    pub id: i64,
    pub short_code: String,
    pub original_url: String,
    pub click_count: i32,
}

/// Create a new shortened link
/// Returns the ID of the newly created link
pub async fn create_link(
    pool: &PgPool,
    short_code: &str,
    original_url: &str,
) -> Result<i64, sqlx::Error> {
    let rec = sqlx::query(
        "INSERT INTO links (short_code, original_url) VALUES ($1, $2) RETURNING id"
    )
    .bind(short_code)
    .bind(original_url)
    .fetch_one(pool)
    .await?;

    Ok(rec.get("id"))
}

/// Get a link by its short code
/// Returns None if the link doesn't exist
pub async fn get_link_by_code(
    pool: &PgPool,
    short_code: &str,
) -> Result<Option<Link>, sqlx::Error> {
    let link = sqlx::query_as::<_, Link>(
        "SELECT id, short_code, original_url, click_count FROM links WHERE short_code = $1"
    )
    .bind(short_code)
    .fetch_optional(pool)
    .await?;

    Ok(link)
}

/// Get all links from the database
/// Useful for admin dashboard or listing all shortened URLs
pub async fn get_all_links(pool: &PgPool) -> Result<Vec<Link>, sqlx::Error> {
    let links = sqlx::query_as::<_, Link>(
        "SELECT id, short_code, original_url, click_count FROM links ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(links)
}

/// Increment the click count for a specific short code
/// Called whenever someone uses a shortened link
pub async fn increment_clicks(
    pool: &PgPool,
    short_code: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE links SET click_count = click_count + 1 WHERE short_code = $1")
        .bind(short_code)
        .execute(pool)
        .await?;

    Ok(())
}

/// Delete a link by its short code
/// Returns the number of rows deleted (0 if not found, 1 if deleted)
pub async fn delete_link(
    pool: &PgPool,
    short_code: &str,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM links WHERE short_code = $1")
        .bind(short_code)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

/// Check if a short code already exists
/// Useful for preventing duplicates when creating custom short codes
pub async fn short_code_exists(
    pool: &PgPool,
    short_code: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("SELECT EXISTS(SELECT 1 FROM links WHERE short_code = $1)")
        .bind(short_code)
        .fetch_one(pool)
        .await?;

    Ok(result.get(0))
}

/// Get link statistics (total links and total clicks)
#[derive(Debug, Serialize)]
pub struct LinkStats {
    pub total_links: i64,
    pub total_clicks: i64,
}

pub async fn get_stats(pool: &PgPool) -> Result<LinkStats, sqlx::Error> {
    let stats = sqlx::query(
        "SELECT COUNT(*) as total_links, COALESCE(SUM(click_count), 0) as total_clicks FROM links"
    )
    .fetch_one(pool)
    .await?;

    Ok(LinkStats {
        total_links: stats.get("total_links"),
        total_clicks: stats.get("total_clicks"),
    })
}