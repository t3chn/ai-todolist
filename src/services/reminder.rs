use crate::models::{Task, TaskWithTelegramId};
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

pub async fn start_reminder_loop(bot: Bot, pool: Arc<SqlitePool>) {
    tracing::info!("Starting reminder service...");

    loop {
        if let Err(e) = check_and_send_reminders(&bot, &pool).await {
            tracing::error!("Reminder check failed: {}", e);
        }

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

async fn check_and_send_reminders(bot: &Bot, pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let due_tasks = Task::find_due_reminders(pool).await?;

    for task in due_tasks {
        let due_str = task.due_at.as_ref()
            .map(|d| format!("\n📅 Due: {}", d))
            .unwrap_or_default();

        let message = format!("⏰ Reminder!\n\n📝 {}{}", task.title, due_str);

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
