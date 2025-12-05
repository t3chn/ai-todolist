use crate::models::{Task, User};
use chrono::Timelike;
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration;
use teloxide::prelude::*;
use teloxide::types::ChatId;

pub async fn start_morning_brief_loop(bot: Bot, pool: Arc<SqlitePool>) {
    tracing::info!("Starting morning brief service...");

    loop {
        if let Err(e) = check_and_send_briefs(&bot, &pool).await {
            tracing::error!("Morning brief check failed: {}", e);
        }

        // Check every minute
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

async fn check_and_send_briefs(bot: &Bot, pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let users = User::find_users_for_morning_brief(pool).await?;

    for user in users {
        let brief = generate_brief(pool, &user).await?;

        if let Err(e) = bot.send_message(ChatId(user.telegram_id), brief).await {
            tracing::error!("Failed to send morning brief to user {}: {}", user.id, e);
        } else {
            tracing::info!("Sent morning brief to user {}", user.telegram_id);
        }
    }

    Ok(())
}

async fn generate_brief(pool: &SqlitePool, user: &User) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let today_tasks = Task::find_today_tasks(pool, user.id).await?;
    let total_pending = Task::count_pending(pool, user.id).await;

    let greeting = match chrono::Utc::now().hour() {
        5..=11 => "Good morning",
        12..=17 => "Good afternoon",
        _ => "Good evening",
    };

    let name = user.first_name.as_deref().unwrap_or("there");

    let mut message = format!("👋 {} {}!\n\n", greeting, name);

    if today_tasks.is_empty() {
        message.push_str("📅 No tasks scheduled for today.\n");
    } else {
        message.push_str(&format!("📅 Today's tasks ({}):\n\n", today_tasks.len()));
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
        message.push_str(&format!("\n📋 {} other pending task(s)\n", other));
    }

    message.push_str("\nHave a productive day! 🚀");

    Ok(message)
}
