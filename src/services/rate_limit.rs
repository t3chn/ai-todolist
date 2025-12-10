use sqlx::SqlitePool;

/// Rate limit configuration
pub struct RateLimits {
    pub tasks_per_day: i64,
    pub voice_per_day: i64,
    pub ai_calls_per_hour: i64,
}

impl RateLimits {
    pub fn trial() -> Self {
        Self {
            tasks_per_day: 10,
            voice_per_day: 5,
            ai_calls_per_hour: 20,
        }
    }

    pub fn paid() -> Self {
        Self {
            tasks_per_day: 100,
            voice_per_day: 50,
            ai_calls_per_hour: 100,
        }
    }
}

pub struct RateLimiter;

impl RateLimiter {
    /// Check if action is allowed and increment counter
    /// Returns Ok(remaining) or Err(message)
    pub async fn check_and_increment(
        pool: &SqlitePool,
        user_id: i64,
        action_type: &str,
        limit: i64,
        window_minutes: i64,
    ) -> Result<i64, String> {
        let window_start = if window_minutes >= 1440 {
            // Daily window - start of today
            "date('now')"
        } else {
            // Hourly window - start of current hour
            "strftime('%Y-%m-%d %H:00:00', 'now')"
        };

        // Get or create rate limit entry for current window
        let result = sqlx::query_scalar::<_, i64>(&format!(
            r#"
            INSERT INTO rate_limits (user_id, action_type, count, window_start)
            VALUES (?, ?, 1, {window_start})
            ON CONFLICT(user_id, action_type, window_start)
            DO UPDATE SET count = count + 1
            RETURNING count
            "#
        ))
        .bind(user_id)
        .bind(action_type)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Rate limit error: {}", e))?;

        let current_count = result;

        if current_count > limit {
            let remaining_msg = if window_minutes >= 1440 {
                "tomorrow"
            } else {
                "next hour"
            };
            Err(format!(
                "Rate limit reached ({}/{}). Try again {}.",
                current_count - 1, limit, remaining_msg
            ))
        } else {
            Ok(limit - current_count)
        }
    }

    /// Get current usage without incrementing
    pub async fn get_usage(
        pool: &SqlitePool,
        user_id: i64,
        action_type: &str,
        window_minutes: i64,
    ) -> i64 {
        let window_start = if window_minutes >= 1440 {
            "date('now')"
        } else {
            "strftime('%Y-%m-%d %H:00:00', 'now')"
        };

        sqlx::query_scalar::<_, i64>(&format!(
            r#"
            SELECT COALESCE(count, 0) FROM rate_limits
            WHERE user_id = ? AND action_type = ? AND window_start = {window_start}
            "#
        ))
        .bind(user_id)
        .bind(action_type)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .unwrap_or(0)
    }

    /// Clean old entries (call periodically)
    pub async fn cleanup_old(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM rate_limits WHERE window_start < datetime('now', '-2 days')")
            .execute(pool)
            .await?;
        Ok(())
    }
}
