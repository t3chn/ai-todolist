use crate::models::{Task, TaskStatus, User};
use crate::services::AiService;
use chrono::Utc;
use sqlx::SqlitePool;
use std::sync::Arc;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode},
    utils::command::BotCommands,
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Show help")]
    Help,
    #[command(description = "List your tasks")]
    Tasks,
}

fn task_keyboard(task_id: i64) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback("✅ Done", format!("done:{}", task_id)),
        InlineKeyboardButton::callback("🗑 Delete", format!("delete:{}", task_id)),
    ]])
}

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    pool: Arc<SqlitePool>,
    ai_service: Option<Arc<AiService>>,
) -> ResponseResult<()> {
    let text = msg.text().unwrap_or_default();
    let telegram_user = msg.from.as_ref();

    if let Ok(cmd) = Command::parse(text, "") {
        match cmd {
            Command::Start => {
                if let Some(tg_user) = telegram_user {
                    let _ = User::get_or_create(
                        &pool,
                        tg_user.id.0 as i64,
                        tg_user.username.as_deref(),
                        Some(&tg_user.first_name),
                    )
                    .await;
                }

                bot.send_message(
                    msg.chat.id,
                    "👋 Welcome to AI Todolist!\n\n\
                    Just send me tasks in natural language:\n\n\
                    • \"Call mom tomorrow at 5pm\"\n\
                    • \"Buy groceries\"\n\
                    • \"Finish report by Friday\"\n\n\
                    /tasks - View your tasks",
                )
                .await?;
            }
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Command::Tasks => {
                if let Some(tg_user) = telegram_user {
                    if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                        let tasks = Task::find_pending_by_user(&pool, user.id).await.unwrap_or_default();

                        if tasks.is_empty() {
                            bot.send_message(msg.chat.id, "📋 No tasks yet!\n\nSend me a message to create one.")
                                .await?;
                        } else {
                            for task in &tasks {
                                let due = task.due_at.as_ref()
                                    .map(|d| format!("\n📅 {}", d))
                                    .unwrap_or_default();

                                bot.send_message(
                                    msg.chat.id,
                                    format!("📝 {}{}", task.title, due),
                                )
                                .reply_markup(task_keyboard(task.id))
                                .await?;
                            }
                        }
                    } else {
                        bot.send_message(msg.chat.id, "Please /start first")
                            .await?;
                    }
                }
            }
        }
    } else if !text.starts_with('/') && !text.is_empty() {
        // Natural language input
        if let Some(tg_user) = telegram_user {
            if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                let (title, due_at) = if let Some(ai) = &ai_service {
                    let current_date = Utc::now().format("%Y-%m-%d %H:%M").to_string();
                    match ai.parse_task(text, &current_date).await {
                        Ok(parsed) => {
                            tracing::info!("AI parsed: {:?}", parsed);
                            (parsed.title, parsed.due_at)
                        }
                        Err(e) => {
                            tracing::warn!("AI parse failed: {}, using raw text", e);
                            (text.to_string(), None)
                        }
                    }
                } else {
                    (text.to_string(), None)
                };

                match Task::create(&pool, user.id, &title, None, due_at.as_deref()).await {
                    Ok(task) => {
                        let due_str = task.due_at.as_ref()
                            .map(|d| format!("\n📅 {}", d))
                            .unwrap_or_default();

                        bot.send_message(
                            msg.chat.id,
                            format!("✅ Added!\n\n📝 {}{}", task.title, due_str),
                        )
                        .reply_markup(task_keyboard(task.id))
                        .await?;
                    }
                    Err(e) => {
                        tracing::error!("Failed to create task: {}", e);
                        bot.send_message(msg.chat.id, "❌ Failed to create task")
                            .await?;
                    }
                }
            } else {
                bot.send_message(msg.chat.id, "Please /start first!")
                    .await?;
            }
        }
    }

    Ok(())
}

pub async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
    pool: Arc<SqlitePool>,
) -> ResponseResult<()> {
    let data = q.data.unwrap_or_default();

    if let Some(task_id_str) = data.strip_prefix("done:") {
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                let _ = Task::update_status(&pool, task_id, TaskStatus::Done).await;

                // Update message
                if let Some(msg) = q.message {
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        format!("✅ {}", task.title),
                    )
                    .await?;
                }

                // Show next task suggestion
                if let Some(user) = q.from.id.0.try_into().ok()
                    .and_then(|tid: i64| {
                        // We need to get user synchronously or use a different approach
                        None::<User>
                    }) {
                    // Would show next task here
                }

                bot.answer_callback_query(q.id).text("✅ Done!").await?;
            }
        }
    } else if let Some(task_id_str) = data.strip_prefix("delete:") {
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                let _ = Task::delete(&pool, task_id).await;

                if let Some(msg) = q.message {
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        format!("🗑 Deleted: {}", task.title),
                    )
                    .await?;
                }

                bot.answer_callback_query(q.id).text("🗑 Deleted").await?;
            }
        }
    }

    Ok(())
}
