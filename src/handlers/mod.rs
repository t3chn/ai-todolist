use crate::models::{Task, TaskStatus, User};
use sqlx::SqlitePool;
use std::sync::Arc;
use teloxide::{prelude::*, types::ParseMode, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Show help")]
    Help,
    #[command(description = "List your tasks")]
    Tasks,
    #[command(description = "Add a new task")]
    Add,
}

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    pool: Arc<SqlitePool>,
) -> ResponseResult<()> {
    let text = msg.text().unwrap_or_default();
    let telegram_user = msg.from.as_ref();

    if let Ok(cmd) = Command::parse(text, "") {
        match cmd {
            Command::Start => {
                // Create user if not exists
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
                    I'm your smart task assistant. Just send me tasks in natural language:\n\n\
                    • \"Call mom tomorrow at 5pm\"\n\
                    • \"Buy groceries\"\n\
                    • \"Finish report by Friday\"\n\n\
                    Commands:\n\
                    /tasks - View your tasks\n\
                    /add <task> - Add a task\n\
                    /help - Show help",
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
                            let mut response = String::from("📋 *Your Tasks:*\n\n");
                            for task in &tasks {
                                let status = task.status_enum();
                                let due = task.due_at.as_ref()
                                    .map(|d| format!(" 📅 {}", d))
                                    .unwrap_or_default();
                                response.push_str(&format!(
                                    "{} `{}` {}{}\n",
                                    status.emoji(),
                                    task.id,
                                    task.title,
                                    due
                                ));
                            }
                            response.push_str("\n_Use /done <id> to complete a task_");

                            bot.send_message(msg.chat.id, response)
                                .parse_mode(ParseMode::Markdown)
                                .await?;
                        }
                    } else {
                        bot.send_message(msg.chat.id, "Please /start first")
                            .await?;
                    }
                }
            }
            Command::Add => {
                let task_text = text.strip_prefix("/add").unwrap_or("").trim();
                if task_text.is_empty() {
                    bot.send_message(msg.chat.id, "Usage: /add <task title>\n\nExample: /add Buy groceries")
                        .await?;
                } else if let Some(tg_user) = telegram_user {
                    if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                        match Task::create(&pool, user.id, task_text, None, None).await {
                            Ok(task) => {
                                bot.send_message(
                                    msg.chat.id,
                                    format!("✅ Task added!\n\n📝 {}\n🆔 `{}`", task.title, task.id),
                                )
                                .parse_mode(ParseMode::Markdown)
                                .await?;
                            }
                            Err(e) => {
                                tracing::error!("Failed to create task: {}", e);
                                bot.send_message(msg.chat.id, "❌ Failed to create task")
                                    .await?;
                            }
                        }
                    }
                }
            }
        }
    } else if !text.starts_with('/') && !text.is_empty() {
        // Natural language input - create task
        if let Some(tg_user) = telegram_user {
            if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                match Task::create(&pool, user.id, text, None, None).await {
                    Ok(task) => {
                        bot.send_message(
                            msg.chat.id,
                            format!("✅ Task added!\n\n📝 {}\n🆔 `{}`", task.title, task.id),
                        )
                        .parse_mode(ParseMode::Markdown)
                        .await?;
                    }
                    Err(e) => {
                        tracing::error!("Failed to create task: {}", e);
                    }
                }
            } else {
                bot.send_message(msg.chat.id, "Please /start first to begin!")
                    .await?;
            }
        }
    }

    Ok(())
}
