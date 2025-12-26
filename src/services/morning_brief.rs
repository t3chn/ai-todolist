use crate::i18n::I18n;
use crate::models::{Task, User};
use chrono::Timelike;
use fluent::FluentArgs;
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration;
use teloxide::prelude::*;
use teloxide::types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn start_morning_brief_loop(bot: Bot, pool: Arc<SqlitePool>, i18n: Arc<I18n>) {
    tracing::info!("Starting morning brief service...");

    loop {
        if let Err(e) = check_and_send_briefs(&bot, &pool, &i18n).await {
            tracing::error!("Morning brief check failed: {}", e);
        }

        // Check every minute
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

async fn check_and_send_briefs(bot: &Bot, pool: &SqlitePool, i18n: &I18n) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Only send briefs to admin users (briefs disabled for regular users)
    let admin_ids: Vec<i64> = std::env::var("ADMIN_IDS")
        .unwrap_or_default()
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let users = User::find_users_for_morning_brief(pool).await?;

    for user in users {
        // Skip non-admin users
        if !admin_ids.contains(&user.telegram_id) {
            continue;
        }
        let lang = user.lang();
        let brief = generate_brief(pool, &user, i18n, lang).await?;

        let keyboard = InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::callback(&i18n.t(lang, "btn-view-tasks"), "view_tasks"),
        ]]);

        if let Err(e) = bot.send_message(ChatId(user.telegram_id), brief)
            .reply_markup(keyboard)
            .await
        {
            tracing::error!("Failed to send morning brief to user {}: {}", user.id, e);
        } else {
            tracing::info!("Sent morning brief to user {}", user.telegram_id);
        }
    }

    Ok(())
}

async fn generate_brief(pool: &SqlitePool, user: &User, i18n: &I18n, lang: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let today_tasks = Task::find_today_tasks(pool, user.id).await?;
    let total_pending = Task::count_pending(pool, user.id).await;

    let greeting_key = match chrono::Utc::now().hour() {
        5..=11 => "brief-greeting-morning",
        12..=17 => "brief-greeting-afternoon",
        _ => "brief-greeting-evening",
    };

    let name = user.first_name.as_deref().unwrap_or("there");

    // Greeting
    let mut args = FluentArgs::new();
    args.set("name", name.to_string());
    let mut message = format!("{}\n\n", i18n.t_args(lang, greeting_key, &args));

    if today_tasks.is_empty() {
        message.push_str(&format!("{}\n", i18n.t(lang, "brief-no-tasks")));
    } else {
        // Today's tasks header
        let mut count_args = FluentArgs::new();
        count_args.set("count", today_tasks.len() as i64);
        message.push_str(&format!("{}\n\n", i18n.t_args(lang, "brief-today-header", &count_args)));

        for task in &today_tasks {
            let time = task.due_at.as_ref()
                .and_then(|d| d.split(' ').nth(1))
                .map(|t| format!(" at {}", t))
                .unwrap_or_default();
            message.push_str(&format!("• {}{}\n", task.title, time));
        }
    }

    if total_pending > today_tasks.len() as i64 {
        let other = total_pending - today_tasks.len() as i64;
        let mut other_args = FluentArgs::new();
        other_args.set("count", other);
        message.push_str(&format!("\n{}\n", i18n.t_args(lang, "brief-other-pending", &other_args)));
    }

    message.push_str(&format!("\n{}", i18n.t(lang, "brief-outro")));

    Ok(message)
}
