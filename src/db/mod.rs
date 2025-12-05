use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;

pub async fn init_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // Ensure data directory exists
    if let Some(path) = database_url.strip_prefix("sqlite:") {
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent).ok();
        }
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Run migrations
    sqlx::query(include_str!("../../migrations/001_init.sql"))
        .execute(&pool)
        .await?;

    // Run subscription migration (ignore errors if columns exist)
    for stmt in include_str!("../../migrations/002_subscriptions.sql").split(';') {
        let stmt = stmt.trim();
        if !stmt.is_empty() {
            let _ = sqlx::query(stmt).execute(&pool).await;
        }
    }

    tracing::info!("Database initialized");
    Ok(pool)
}
