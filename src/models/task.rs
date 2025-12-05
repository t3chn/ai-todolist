use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Done,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InProgress => "in_progress",
            Self::Done => "done",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "in_progress" => Self::InProgress,
            "done" => Self::Done,
            "cancelled" => Self::Cancelled,
            _ => Self::Pending,
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Pending => "⏳",
            Self::InProgress => "🔄",
            Self::Done => "✅",
            Self::Cancelled => "❌",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub due_at: Option<String>,
    pub reminder_at: Option<String>,
    pub tags: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Task {
    pub fn status_enum(&self) -> TaskStatus {
        TaskStatus::from_str(&self.status)
    }

    pub async fn create(
        pool: &SqlitePool,
        user_id: i64,
        title: &str,
        description: Option<&str>,
        due_at: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            r#"
            INSERT INTO tasks (user_id, title, description, due_at)
            VALUES (?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(title)
        .bind(description)
        .bind(due_at)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_user(pool: &SqlitePool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks WHERE user_id = ? AND status != 'cancelled' ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    pub async fn find_pending_by_user(pool: &SqlitePool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks WHERE user_id = ? AND status IN ('pending', 'in_progress') ORDER BY due_at ASC NULLS LAST, created_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &SqlitePool, id: i64) -> Option<Self> {
        sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .ok()
            .flatten()
    }

    pub async fn update_status(pool: &SqlitePool, id: i64, status: TaskStatus) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE tasks SET status = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(status.as_str())
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
