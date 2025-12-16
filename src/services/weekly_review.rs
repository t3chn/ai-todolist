use crate::models::{Task, User};
use chrono::{Datelike, Timelike, Weekday};
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration;
use teloxide::prelude::*;
use teloxide::types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn start_weekly_review_loop(bot: Bot, pool: Arc<SqlitePool>) {
    tracing::info!("Starting weekly review service...");

    loop {
        if let Err(e) = check_and_send_reviews(&bot, &pool).await {
            tracing::error!("Weekly review check failed: {}", e);
        }

        // Check every hour
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}

async fn check_and_send_reviews(bot: &Bot, pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let now = chrono::Utc::now();

    // Only run on Sundays at 18:00 UTC
    if now.weekday() != Weekday::Sun || now.hour() != 18 {
        return Ok(());
    }

    tracing::info!("Running weekly review...");

    // Get all active users
    let users = User::list_by_segment(pool, "all").await?;

    for user in users {
        if !user.has_active_subscription() {
            continue;
        }

        let review = generate_weekly_review(pool, &user).await?;

        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("📋 Review stale", "stale:review"),
                InlineKeyboardButton::callback("📊 View all", "view_tasks"),
            ]
        ]);

        if let Err(e) = bot.send_message(ChatId(user.telegram_id), review)
            .reply_markup(keyboard)
            .await
        {
            tracing::error!("Failed to send weekly review to user {}: {}", user.id, e);
        } else {
            tracing::info!("Sent weekly review to user {}", user.telegram_id);
        }
    }

    Ok(())
}

async fn generate_weekly_review(pool: &SqlitePool, user: &User) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let completed = Task::count_completed_week(pool, user.id).await;
    let created = Task::count_created_week(pool, user.id).await;
    let pending = Task::count_pending(pool, user.id).await;
    let stale = Task::find_stale(pool, user.id, 7).await.unwrap_or_default().len();

    let name = user.first_name.as_deref().unwrap_or("there");

    let mut message = format!("📊 Weekly Review for {}\n\n", name);

    // Stats
    message.push_str("This week:\n");
    message.push_str(&format!("✅ Completed: {}\n", completed));
    message.push_str(&format!("➕ Added: {}\n", created));
    message.push_str(&format!("📋 Pending: {}\n", pending));

    // Celebration based on completion rate
    if completed > 0 {
        let celebration = match completed {
            1..=4 => "Good progress! 👍",
            5..=9 => "Great week! 🔥",
            10..=19 => "Productive week! ⚡",
            _ => "Incredible productivity! 🏆",
        };
        message.push_str(&format!("\n{}\n", celebration));
    }

    // Stale warning
    if stale > 0 {
        message.push_str(&format!(
            "\n⚠️ {} task{} stale (7+ days). Time to review?\n",
            stale,
            if stale == 1 { "" } else { "s" }
        ));
    }

    message.push_str("\nHave a great week ahead! 🚀");

    Ok(message)
}
