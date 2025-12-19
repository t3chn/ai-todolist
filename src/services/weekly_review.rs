use crate::i18n::I18n;
use crate::models::{Task, User};
use chrono::{Datelike, Timelike, Weekday};
use fluent::FluentArgs;
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration;
use teloxide::prelude::*;
use teloxide::types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn start_weekly_review_loop(bot: Bot, pool: Arc<SqlitePool>, i18n: Arc<I18n>) {
    tracing::info!("Starting weekly review service...");

    loop {
        if let Err(e) = check_and_send_reviews(&bot, &pool, &i18n).await {
            tracing::error!("Weekly review check failed: {}", e);
        }

        // Check every hour
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}

async fn check_and_send_reviews(bot: &Bot, pool: &SqlitePool, i18n: &I18n) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

        let lang = user.lang();
        let review = generate_weekly_review(pool, &user, i18n, lang).await?;

        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback(&i18n.t(lang, "btn-review-stale"), "stale:review"),
                InlineKeyboardButton::callback(&i18n.t(lang, "btn-view-all"), "view_tasks"),
            ]
        ]);

        if let Err(e) = bot.send_message(ChatId(user.telegram_id), review)
            .parse_mode(teloxide::types::ParseMode::Html)
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

async fn generate_weekly_review(pool: &SqlitePool, user: &User, i18n: &I18n, lang: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let completed = Task::count_completed_week(pool, user.id).await;
    let created = Task::count_created_week(pool, user.id).await;
    let pending = Task::count_pending(pool, user.id).await;
    let stale = Task::find_stale(pool, user.id, 7).await.unwrap_or_default().len();

    let name = user.first_name.as_deref().unwrap_or("there");

    // Title
    let mut message = format!("{}\n", i18n.t(lang, "weekly-title"));

    // Greeting
    let mut args = FluentArgs::new();
    args.set("name", name.to_string());
    message.push_str(&format!("{}\n\n", i18n.t_args(lang, "weekly-greeting", &args)));

    // "This week:" header
    message.push_str(&format!("{}\n", i18n.t(lang, "weekly-this-week")));

    // Stats
    let mut count_args = FluentArgs::new();
    count_args.set("count", completed as i64);
    message.push_str(&format!("{}\n", i18n.t_args(lang, "weekly-completed", &count_args)));

    count_args = FluentArgs::new();
    count_args.set("count", created as i64);
    message.push_str(&format!("{}\n", i18n.t_args(lang, "weekly-created", &count_args)));

    count_args = FluentArgs::new();
    count_args.set("count", pending as i64);
    message.push_str(&format!("{}\n", i18n.t_args(lang, "weekly-pending", &count_args)));

    // Celebration based on completion rate
    if completed > 0 {
        let celebration_key = match completed {
            1..=4 => "weekly-celebrate-good",
            5..=9 => "weekly-celebrate-great",
            10..=19 => "weekly-celebrate-productive",
            _ => "weekly-celebrate-incredible",
        };
        message.push_str(&format!("\n{}\n", i18n.t(lang, celebration_key)));
    }

    // Stale warning
    if stale > 0 {
        let mut stale_args = FluentArgs::new();
        stale_args.set("count", stale as i64);
        message.push_str(&format!("\n{}\n", i18n.t_args(lang, "weekly-stale-warning", &stale_args)));
    }

    message.push_str(&format!("\n{}", i18n.t(lang, "weekly-outro")));

    Ok(message)
}
