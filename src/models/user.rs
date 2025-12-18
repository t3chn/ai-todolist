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
    // Admin
    pub is_banned: Option<i64>,
    pub banned_at: Option<String>,
    pub ban_reason: Option<String>,
    pub last_active_at: Option<String>,
    // Language preference (en, ru)
    pub language: Option<String>,
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

    /// Get days remaining in trial (ceiling - partial day counts as full)
    pub fn trial_days_remaining(&self) -> Option<i64> {
        let now = Utc::now();
        if let Some(trial_ends) = &self.trial_ends_at {
            if let Ok(exp) = chrono::NaiveDateTime::parse_from_str(trial_ends, "%Y-%m-%d %H:%M:%S") {
                let duration = exp.and_utc() - now;
                if duration.num_seconds() > 0 {
                    // Ceiling: any partial day counts as 1 day
                    let days = (duration.num_seconds() + 86399) / 86400;
                    return Some(days);
                }
            }
        }
        None
    }

    /// Get days remaining in subscription (ceiling)
    pub fn subscription_days_remaining(&self) -> Option<i64> {
        let now = Utc::now();
        if let Some(expires) = &self.subscription_expires_at {
            if let Ok(exp) = chrono::NaiveDateTime::parse_from_str(expires, "%Y-%m-%d %H:%M:%S") {
                let duration = exp.and_utc() - now;
                if duration.num_seconds() > 0 {
                    let days = (duration.num_seconds() + 86399) / 86400;
                    return Some(days);
                }
            }
        }
        None
    }

    /// Get human-readable subscription status
    pub fn subscription_status(&self) -> String {
        // Check paid subscription first
        if let Some(days) = self.subscription_days_remaining() {
            return format!("✅ Active ({} days)", days);
        }
        // Check trial
        if let Some(days) = self.trial_days_remaining() {
            if days <= 2 {
                return format!("⚠️ Trial ends in {} day{}", days, if days == 1 { "" } else { "s" });
            }
            return format!("🎁 Trial ({} days)", days);
        }
        // Expired
        "❌ Expired".to_string()
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

    /// Update language preference
    pub async fn update_language(pool: &SqlitePool, id: i64, language: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET language = ?, updated_at = datetime('now') WHERE id = ?"
        )
        .bind(language)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Get language code (defaults to "en")
    pub fn lang(&self) -> &str {
        self.language.as_deref().unwrap_or("en")
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

    // ============ Admin Methods ============

    /// Check if user is banned
    pub fn is_banned(&self) -> bool {
        self.is_banned.unwrap_or(0) == 1
    }

    /// Ban user
    pub async fn ban(pool: &SqlitePool, user_id: i64, reason: Option<&str>) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET is_banned = 1, banned_at = datetime('now'), ban_reason = ? WHERE id = ?"
        )
        .bind(reason)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Unban user
    pub async fn unban(pool: &SqlitePool, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET is_banned = 0, banned_at = NULL, ban_reason = NULL WHERE id = ?"
        )
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Grant subscription for N days
    pub async fn grant_subscription(pool: &SqlitePool, user_id: i64, days: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE users SET
                subscription_expires_at = datetime(
                    COALESCE(
                        CASE WHEN subscription_expires_at > datetime('now') THEN subscription_expires_at ELSE NULL END,
                        datetime('now')
                    ),
                    ? || ' days'
                ),
                subscription_type = 'paid',
                updated_at = datetime('now')
            WHERE id = ?
            "#
        )
        .bind(format!("+{}", days))
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Update last active timestamp
    pub async fn touch_last_active(pool: &SqlitePool, telegram_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET last_active_at = datetime('now') WHERE telegram_id = ?")
            .bind(telegram_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Get all users with pagination
    pub async fn list_all(pool: &SqlitePool, limit: i64, offset: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    /// Search users by username or telegram_id
    pub async fn search(pool: &SqlitePool, query: &str) -> Result<Vec<Self>, sqlx::Error> {
        let query_pattern = format!("%{}%", query);
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username LIKE ? OR CAST(telegram_id AS TEXT) LIKE ? OR first_name LIKE ? LIMIT 20"
        )
        .bind(&query_pattern)
        .bind(&query_pattern)
        .bind(&query_pattern)
        .fetch_all(pool)
        .await
    }

    /// Get banned users
    pub async fn list_banned(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE is_banned = 1 ORDER BY banned_at DESC")
            .fetch_all(pool)
            .await
    }

    /// Get stats for admin dashboard
    pub async fn admin_stats(pool: &SqlitePool) -> Result<AdminStats, sqlx::Error> {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(pool)
            .await?;

        let trial: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE subscription_type = 'trial' AND trial_ends_at > datetime('now')"
        )
            .fetch_one(pool)
            .await?;

        let paid: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE subscription_type = 'paid' AND subscription_expires_at > datetime('now')"
        )
            .fetch_one(pool)
            .await?;

        let expired: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE (subscription_type = 'trial' AND trial_ends_at <= datetime('now')) OR (subscription_type = 'paid' AND subscription_expires_at <= datetime('now'))"
        )
            .fetch_one(pool)
            .await?;

        let banned: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE is_banned = 1")
            .fetch_one(pool)
            .await?;

        let active_7d: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE last_active_at > datetime('now', '-7 days')"
        )
            .fetch_one(pool)
            .await?;

        let active_30d: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE last_active_at > datetime('now', '-30 days')"
        )
            .fetch_one(pool)
            .await?;

        let new_today: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE created_at > datetime('now', '-1 day')"
        )
            .fetch_one(pool)
            .await?;

        let new_week: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE created_at > datetime('now', '-7 days')"
        )
            .fetch_one(pool)
            .await?;

        Ok(AdminStats {
            total,
            trial,
            paid,
            expired,
            banned,
            active_7d,
            active_30d,
            new_today,
            new_week,
        })
    }

    /// Get users by subscription segment
    pub async fn list_by_segment(pool: &SqlitePool, segment: &str) -> Result<Vec<Self>, sqlx::Error> {
        let query = match segment {
            "trial" => "SELECT * FROM users WHERE subscription_type = 'trial' AND trial_ends_at > datetime('now') AND (is_banned IS NULL OR is_banned = 0)",
            "paid" => "SELECT * FROM users WHERE subscription_type = 'paid' AND subscription_expires_at > datetime('now') AND (is_banned IS NULL OR is_banned = 0)",
            "expired" => "SELECT * FROM users WHERE ((subscription_type = 'trial' AND trial_ends_at <= datetime('now')) OR (subscription_type = 'paid' AND subscription_expires_at <= datetime('now'))) AND (is_banned IS NULL OR is_banned = 0)",
            "all" | _ => "SELECT * FROM users WHERE is_banned IS NULL OR is_banned = 0",
        };
        sqlx::query_as::<_, User>(query).fetch_all(pool).await
    }

    /// Display name for admin UI
    pub fn display_name(&self) -> String {
        if let Some(username) = &self.username {
            format!("@{}", username)
        } else if let Some(name) = &self.first_name {
            name.clone()
        } else {
            format!("User {}", self.telegram_id)
        }
    }
}

#[derive(Debug)]
pub struct AdminStats {
    pub total: i64,
    pub trial: i64,
    pub paid: i64,
    pub expired: i64,
    pub banned: i64,
    pub active_7d: i64,
    pub active_30d: i64,
    pub new_today: i64,
    pub new_week: i64,
}
