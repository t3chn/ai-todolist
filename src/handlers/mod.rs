use crate::models::{Task, TaskStatus, User};
use crate::services::AiService;
use chrono::Utc;
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
    #[command(description = "Mark task as done")]
    Done,
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
                    I'm your smart task assistant. Just send me tasks in natural language:\n\n\
                    • \"Call mom tomorrow at 5pm\"\n\
                    • \"Buy groceries\"\n\
                    • \"Finish report by Friday\"\n\n\
                    Commands:\n\
                    /tasks - View your tasks\n\
                    /done <id> - Complete a task\n\
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
            Command::Done => {
                let id_str = text.strip_prefix("/done").unwrap_or("").trim();
                if let Ok(task_id) = id_str.parse::<i64>() {
                    if let Some(task) = Task::find_by_id(&pool, task_id).await {
                        if let Err(e) = Task::update_status(&pool, task_id, TaskStatus::Done).await {
                            tracing::error!("Failed to update task: {}", e);
                            bot.send_message(msg.chat.id, "❌ Failed to complete task")
                                .await?;
                        } else {
                            bot.send_message(
                                msg.chat.id,
                                format!("✅ Completed: {}", task.title),
                            )
                            .await?;
                        }
                    } else {
                        bot.send_message(msg.chat.id, "❌ Task not found")
                            .await?;
                    }
                } else {
                    bot.send_message(msg.chat.id, "Usage: /done <task_id>\n\nExample: /done 1")
                        .await?;
                }
            }
        }
    } else if !text.starts_with('/') && !text.is_empty() {
        // Natural language input - parse with AI or create simple task
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
                            format!("✅ Task added!\n\n📝 {}{}\n🆔 `{}`", task.title, due_str, task.id),
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
            } else {
                bot.send_message(msg.chat.id, "Please /start first to begin!")
                    .await?;
            }
        }
    }

    Ok(())
}
