use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

/// Task with user's telegram_id for reminders
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TaskWithTelegramId {
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
    pub telegram_id: i64,
}

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

    /// Find tasks that need reminders (reminder_at <= now)
    pub async fn find_due_reminders(pool: &SqlitePool) -> Result<Vec<TaskWithTelegramId>, sqlx::Error> {
        sqlx::query_as::<_, TaskWithTelegramId>(
            r#"
            SELECT t.id, t.user_id, t.title, t.description, t.status, t.due_at, t.reminder_at, t.tags, t.created_at, t.updated_at, u.telegram_id
            FROM tasks t
            JOIN users u ON t.user_id = u.id
            WHERE t.reminder_at IS NOT NULL
              AND t.reminder_at <= datetime('now')
              AND t.status IN ('pending', 'in_progress')
            "#,
        )
        .fetch_all(pool)
        .await
    }

    /// Clear reminder (after sending)
    pub async fn clear_reminder(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE tasks SET reminder_at = NULL, updated_at = datetime('now') WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Snooze reminder
    pub async fn snooze_reminder(pool: &SqlitePool, id: i64, minutes: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE tasks SET reminder_at = datetime('now', ? || ' minutes'), updated_at = datetime('now') WHERE id = ?"
        )
        .bind(format!("+{}", minutes))
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Set reminder from due_at (30 min before)
    pub async fn set_reminder_from_due(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE tasks
            SET reminder_at = datetime(due_at, '-30 minutes'),
                updated_at = datetime('now')
            WHERE id = ? AND due_at IS NOT NULL
            "#
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }
}
