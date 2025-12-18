use crate::i18n::I18n;
use crate::models::Task;
use chrono::{NaiveDateTime, Utc};
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration;
use teloxide::prelude::*;
use teloxide::types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup};

fn reminder_keyboard(task_id: i64) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("✅ Done", format!("done:{}", task_id)),
            InlineKeyboardButton::callback("⏰ 1h", format!("snooze:{}:60", task_id)),
            InlineKeyboardButton::callback("📅 Tomorrow", format!("snooze:{}:1440", task_id)),
        ],
    ])
}

pub async fn start_reminder_loop(bot: Bot, pool: Arc<SqlitePool>, _i18n: Arc<I18n>) {
    tracing::info!("Starting reminder service...");

    loop {
        if let Err(e) = check_and_send_reminders(&bot, &pool).await {
            tracing::error!("Reminder check failed: {}", e);
        }

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

fn get_urgency_indicator(due_at: Option<&String>) -> &'static str {
    if let Some(due_str) = due_at {
        if let Ok(due_time) = NaiveDateTime::parse_from_str(due_str, "%Y-%m-%d %H:%M:%S") {
            let now = Utc::now().naive_utc();
            let minutes_until_due = (due_time - now).num_minutes();

            return match minutes_until_due {
                ..=15 => "🔴 Due very soon!",
                16..=30 => "🟡 Due in 30 minutes",
                _ => "⏰ Reminder",
            };
        }
    }
    "⏰ Reminder"
}

async fn check_and_send_reminders(bot: &Bot, pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let due_tasks = Task::find_due_reminders(pool).await?;

    for task in due_tasks {
        let urgency = get_urgency_indicator(task.due_at.as_ref());

        let due_str = task.due_at.as_ref()
            .map(|d| format!("\n📅 Due: {}", d))
            .unwrap_or_default();

        let message = format!("{}\n\n📝 {}{}", urgency, task.title, due_str);

        match bot.send_message(ChatId(task.telegram_id), message)
            .reply_markup(reminder_keyboard(task.id))
            .await
        {
            Ok(_) => {
                Task::clear_reminder(pool, task.id).await?;
                tracing::info!("Sent reminder for task {} to user {}", task.id, task.telegram_id);
            }
            Err(e) => {
                tracing::error!("Failed to send reminder for task {}: {}", task.id, e);
            }
        }
    }

    Ok(())
}
