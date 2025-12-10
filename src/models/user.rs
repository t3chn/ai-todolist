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
    pub trial_ends_at: Option<String>,
    pub subscription_expires_at: Option<String>,
    pub subscription_type: Option<String>,
    // Referral system
    pub referral_code: Option<String>,
    pub referred_by: Option<i64>,
    pub referral_count: Option<i64>,
    pub bonus_days: Option<i64>,
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
            INSERT INTO users (telegram_id, username, first_name, trial_ends_at, subscription_type)
            VALUES (?, ?, ?, datetime('now', '+7 days'), 'trial')
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

    /// Find users who need morning brief now (current hour:minute matches their brief time)
    pub async fn find_users_for_morning_brief(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        // Get current time in format HH:MM
        sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE morning_brief_time = strftime('%H:%M', 'now')
            "#,
        )
        .fetch_all(pool)
        .await
    }

    /// Find by internal id
    pub async fn find_by_id(pool: &SqlitePool, id: i64) -> Option<Self> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .ok()
            .flatten()
    }

    /// Check if user has active subscription or trial
    pub fn has_active_subscription(&self) -> bool {
        let now = Utc::now();

        // Check subscription first
        if let Some(expires) = &self.subscription_expires_at {
            if let Ok(exp) = DateTime::parse_from_rfc3339(expires) {
                if exp > now {
                    return true;
                }
            }
            // Also try SQLite format
            if let Ok(exp) = chrono::NaiveDateTime::parse_from_str(expires, "%Y-%m-%d %H:%M:%S") {
                if exp.and_utc() > now {
                    return true;
                }
            }
        }

        // Check trial
        if let Some(trial_ends) = &self.trial_ends_at {
            if let Ok(exp) = DateTime::parse_from_rfc3339(trial_ends) {
                if exp > now {
                    return true;
                }
            }
            if let Ok(exp) = chrono::NaiveDateTime::parse_from_str(trial_ends, "%Y-%m-%d %H:%M:%S") {
                if exp.and_utc() > now {
                    return true;
                }
            }
        }

        false
    }

    /// Get days remaining in trial
    pub fn trial_days_remaining(&self) -> Option<i64> {
        let now = Utc::now();
        if let Some(trial_ends) = &self.trial_ends_at {
            if let Ok(exp) = chrono::NaiveDateTime::parse_from_str(trial_ends, "%Y-%m-%d %H:%M:%S") {
                let days = (exp.and_utc() - now).num_days();
                if days >= 0 {
                    return Some(days);
                }
            }
        }
        None
    }

    /// Activate subscription
    pub async fn activate_subscription(pool: &SqlitePool, id: i64, months: i64) -> Result<(), sqlx::Error> {
        let expires = format!("+{} months", months);
        sqlx::query(
            "UPDATE users SET subscription_expires_at = datetime('now', ?), subscription_type = 'monthly', updated_at = datetime('now') WHERE id = ?"
        )
        .bind(expires)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Update timezone
    pub async fn update_timezone(pool: &SqlitePool, id: i64, timezone: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET timezone = ?, updated_at = datetime('now') WHERE id = ?"
        )
        .bind(timezone)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Update morning brief time
    pub async fn update_morning_brief_time(pool: &SqlitePool, id: i64, time: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET morning_brief_time = ?, updated_at = datetime('now') WHERE id = ?"
        )
        .bind(time)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Generate and set referral code for user
    pub async fn ensure_referral_code(pool: &SqlitePool, id: i64) -> Result<String, sqlx::Error> {
        // Check if user already has a code
        let existing: Option<String> = sqlx::query_scalar(
            "SELECT referral_code FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .flatten();

        if let Some(code) = existing {
            return Ok(code);
        }

        // Generate new code (base36 of user id + random suffix)
        let code = format!("ref{:x}{:04x}", id, rand::random::<u16>());

        sqlx::query(
            "UPDATE users SET referral_code = ? WHERE id = ?"
        )
        .bind(&code)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(code)
    }

    /// Find user by referral code
    pub async fn find_by_referral_code(pool: &SqlitePool, code: &str) -> Option<Self> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE referral_code = ?")
            .bind(code)
            .fetch_optional(pool)
            .await
            .ok()
            .flatten()
    }

    /// Create user with referral
    pub async fn create_with_referral(
        pool: &SqlitePool,
        telegram_id: i64,
        username: Option<&str>,
        first_name: Option<&str>,
        referrer_id: i64,
    ) -> Result<Self, sqlx::Error> {
        // Create user with referral
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (telegram_id, username, first_name, trial_ends_at, subscription_type, referred_by)
            VALUES (?, ?, ?, datetime('now', '+7 days'), 'trial', ?)
            RETURNING *
            "#,
        )
        .bind(telegram_id)
        .bind(username)
        .bind(first_name)
        .bind(referrer_id)
        .fetch_one(pool)
        .await?;

        // Increment referrer's count and add bonus days
        sqlx::query(
            r#"
            UPDATE users
            SET referral_count = COALESCE(referral_count, 0) + 1,
                bonus_days = COALESCE(bonus_days, 0) + 7,
                trial_ends_at = datetime(COALESCE(trial_ends_at, datetime('now')), '+7 days'),
                updated_at = datetime('now')
            WHERE id = ?
            "#
        )
        .bind(referrer_id)
        .execute(pool)
        .await?;

        Ok(user)
    }

    /// Get referral stats
    pub fn referral_stats(&self) -> (i64, i64) {
        (
            self.referral_count.unwrap_or(0),
            self.bonus_days.unwrap_or(0),
        )
    }
}
