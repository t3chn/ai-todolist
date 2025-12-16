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
        tags: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            r#"
            INSERT INTO tasks (user_id, title, description, due_at, tags)
            VALUES (?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(title)
        .bind(description)
        .bind(due_at)
        .bind(tags)
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

    /// Find tasks due today for a user
    pub async fn find_today_tasks(pool: &SqlitePool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            r#"
            SELECT * FROM tasks
            WHERE user_id = ?
              AND status IN ('pending', 'in_progress')
              AND date(due_at) = date('now')
            ORDER BY due_at ASC
            "#
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// Count pending tasks for a user
    pub async fn count_pending(pool: &SqlitePool, user_id: i64) -> i64 {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM tasks WHERE user_id = ? AND status IN ('pending', 'in_progress')"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0)
    }

    /// Count all tasks for a user
    pub async fn count_by_user(pool: &SqlitePool, user_id: i64) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tasks WHERE user_id = ?")
            .bind(user_id)
            .fetch_one(pool)
            .await
    }

    /// Update task title and/or due_at
    pub async fn update(
        pool: &SqlitePool,
        id: i64,
        title: Option<&str>,
        due_at: Option<Option<&str>>,
    ) -> Result<(), sqlx::Error> {
        if let Some(new_title) = title {
            sqlx::query("UPDATE tasks SET title = ?, updated_at = datetime('now') WHERE id = ?")
                .bind(new_title)
                .bind(id)
                .execute(pool)
                .await?;
        }
        if let Some(new_due) = due_at {
            sqlx::query("UPDATE tasks SET due_at = ?, updated_at = datetime('now') WHERE id = ?")
                .bind(new_due)
                .bind(id)
                .execute(pool)
                .await?;
        }
        Ok(())
    }

    /// Set custom reminder time
    pub async fn set_reminder(pool: &SqlitePool, id: i64, reminder_at: Option<&str>) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE tasks SET reminder_at = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(reminder_at)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Find potential duplicate tasks (similar title, pending status)
    pub async fn find_similar(pool: &SqlitePool, user_id: i64, title: &str) -> Option<Self> {
        // Normalize title for comparison
        let normalized = title.to_lowercase().trim().to_string();

        // Get pending tasks and check for similarity
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks WHERE user_id = ? AND status IN ('pending', 'in_progress')"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .ok()?;

        for task in tasks {
            let task_normalized = task.title.to_lowercase();
            // Check for exact match or substring match
            if task_normalized == normalized
                || task_normalized.contains(&normalized)
                || normalized.contains(&task_normalized) {
                return Some(task);
            }
        }
        None
    }

    /// Find stale tasks (pending, not updated in days_threshold days)
    pub async fn find_stale(pool: &SqlitePool, user_id: i64, days_threshold: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            &format!(
                "SELECT * FROM tasks WHERE user_id = ? AND status IN ('pending', 'in_progress') AND updated_at < datetime('now', '-{} days') ORDER BY updated_at ASC",
                days_threshold
            )
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// Touch task (update updated_at to now)
    pub async fn touch(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE tasks SET updated_at = datetime('now') WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Count tasks completed today
    pub async fn count_completed_today(pool: &SqlitePool, user_id: i64) -> i64 {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM tasks WHERE user_id = ? AND status = 'done' AND date(updated_at) = date('now')"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0)
    }

    /// Count tasks completed this week
    pub async fn count_completed_week(pool: &SqlitePool, user_id: i64) -> i64 {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM tasks WHERE user_id = ? AND status = 'done' AND updated_at > datetime('now', '-7 days')"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0)
    }

    /// Count tasks created this week
    pub async fn count_created_week(pool: &SqlitePool, user_id: i64) -> i64 {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM tasks WHERE user_id = ? AND created_at > datetime('now', '-7 days')"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0)
    }
}
