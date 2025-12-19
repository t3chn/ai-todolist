use crate::i18n::I18n;
use crate::models::Task;
use chrono::{NaiveDateTime, Utc};
use fluent::FluentArgs;
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration;
use teloxide::prelude::*;
use teloxide::types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup};

fn reminder_keyboard(task_id: i64, i18n: &I18n, lang: &str) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback(&i18n.t(lang, "btn-done"), format!("done:{}", task_id)),
            InlineKeyboardButton::callback(&i18n.t(lang, "btn-snooze-1h"), format!("snooze:{}:60", task_id)),
            InlineKeyboardButton::callback(&i18n.t(lang, "btn-snooze-tomorrow"), format!("snooze:{}:1440", task_id)),
        ],
    ])
}

pub async fn start_reminder_loop(bot: Bot, pool: Arc<SqlitePool>, i18n: Arc<I18n>) {
    tracing::info!("Starting reminder service...");

    loop {
        if let Err(e) = check_and_send_reminders(&bot, &pool, &i18n).await {
            tracing::error!("Reminder check failed: {}", e);
        }

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

fn get_urgency_key(due_at: Option<&String>) -> &'static str {
    if let Some(due_str) = due_at {
        if let Ok(due_time) = NaiveDateTime::parse_from_str(due_str, "%Y-%m-%d %H:%M:%S") {
            let now = Utc::now().naive_utc();
            let minutes_until_due = (due_time - now).num_minutes();

            return match minutes_until_due {
                ..=15 => "reminder-urgent",
                16..=30 => "reminder-soon",
                _ => "reminder-normal",
            };
        }
    }
    "reminder-normal"
}

async fn check_and_send_reminders(bot: &Bot, pool: &SqlitePool, i18n: &I18n) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let due_tasks = Task::find_due_reminders(pool).await?;

    for task in due_tasks {
        let lang = task.language.as_deref().unwrap_or("en");
        let urgency_key = get_urgency_key(task.due_at.as_ref());
        let urgency = i18n.t(lang, urgency_key);

        let due_str = if let Some(d) = task.due_at.as_ref() {
            let mut args = FluentArgs::new();
            args.set("due", d.clone());
            format!("\n{}", i18n.t_args(lang, "reminder-due", &args))
        } else {
            String::new()
        };

        let message = format!("{}\n\n📝 {}{}", urgency, task.title, due_str);

        match bot.send_message(ChatId(task.telegram_id), message)
            .reply_markup(reminder_keyboard(task.id, i18n, lang))
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
