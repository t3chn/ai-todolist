use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub telegram_id: i64,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub timezone: String,
    pub morning_brief_time: String,
    pub created_at: String,
    pub updated_at: String,
}

impl User {
    pub async fn find_by_telegram_id(pool: &SqlitePool, telegram_id: i64) -> Option<Self> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE telegram_id = ?")
            .bind(telegram_id)
            .fetch_optional(pool)
            .await
            .ok()
            .flatten()
    }

    pub async fn create(
        pool: &SqlitePool,
        telegram_id: i64,
        username: Option<&str>,
        first_name: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (telegram_id, username, first_name)
            VALUES (?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(telegram_id)
        .bind(username)
        .bind(first_name)
        .fetch_one(pool)
        .await
    }

    pub async fn get_or_create(
        pool: &SqlitePool,
        telegram_id: i64,
        username: Option<&str>,
        first_name: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        if let Some(user) = Self::find_by_telegram_id(pool, telegram_id).await {
            Ok(user)
        } else {
            Self::create(pool, telegram_id, username, first_name).await
        }
    }
}
