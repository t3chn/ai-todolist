use crate::models::{Task, TaskStatus, User};
use crate::services::{AiService, ConversationContext, ParsedInput};
use chrono::Utc;
use sqlx::SqlitePool;
use std::sync::Arc;
use teloxide::{
    net::Download,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};
use tokio::io::AsyncWriteExt;

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
    context: Arc<ConversationContext>,
) -> ResponseResult<()> {
    let text = msg.text().unwrap_or_default();
    let telegram_user = msg.from.as_ref();

    if let Ok(cmd) = Command::parse(text, "") {
        match cmd {
            Command::Start => {
                if let Some(tg_user) = telegram_user {
                    let user = User::get_or_create(
                        &pool,
                        tg_user.id.0 as i64,
                        tg_user.username.as_deref(),
                        Some(&tg_user.first_name),
                    )
                    .await;

                    let trial_info = if let Ok(u) = user {
                        if let Some(days) = u.trial_days_remaining() {
                            format!("\n\n🎁 Trial: {} days remaining", days)
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };

                    bot.send_message(
                        msg.chat.id,
                        format!("👋 Welcome to AI Todolist!\n\n\
                        Just send me tasks in natural language:\n\n\
                        • \"Call mom tomorrow at 5pm\"\n\
                        • \"Buy groceries\"\n\
                        • \"Finish report by Friday\"\n\n\
                        🎤 Voice messages supported!\n\
                        ✉️ \"Draft message to...\" for drafts\n\n\
                        /tasks - View your tasks{}", trial_info),
                    )
                    .await?;
                } else {
                    bot.send_message(
                        msg.chat.id,
                        "👋 Welcome to AI Todolist!\n\n/tasks - View your tasks",
                    )
                    .await?;
                }
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
    } else if let Some(voice) = msg.voice() {
        // Voice message handling
        if let Some(tg_user) = telegram_user {
            if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                if let Some(ai) = &ai_service {
                    // Download voice file
                    let file = bot.get_file(&voice.file.id).await?;
                    let mut audio_data = Vec::new();
                    bot.download_file(&file.path, &mut audio_data).await?;

                    // Transcribe with Whisper
                    match ai.transcribe_audio(audio_data).await {
                        Ok(transcript) => {
                            tracing::info!("Transcribed: {}", transcript);

                            // Parse input from transcript
                            let current_date = Utc::now().format("%Y-%m-%d %H:%M").to_string();
                            let user_context = context.get_context(user.id);
                            context.add_message(user.id, "user", &transcript);

                            match ai.parse_input(&transcript, &current_date, user_context.as_deref()).await {
                                Ok(ParsedInput::Task(parsed)) => {
                                    match Task::create(&pool, user.id, &parsed.title, None, parsed.due_at.as_deref()).await {
                                        Ok(task) => {
                                            if task.due_at.is_some() {
                                                let _ = Task::set_reminder_from_due(&pool, task.id).await;
                                            }

                                            let due_str = task.due_at.as_ref()
                                                .map(|d| format!("\n📅 {}", d))
                                                .unwrap_or_default();

                                            bot.send_message(
                                                msg.chat.id,
                                                format!("🎤 \"{}\"\n\n✅ Added!\n\n📝 {}{}", transcript, task.title, due_str),
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
                                }
                                Ok(ParsedInput::Draft { recipient, context: _, draft }) => {
                                    bot.send_message(
                                        msg.chat.id,
                                        format!("🎤 \"{}\"\n\n✉️ Draft for {}:\n\n{}\n\n💡 Copy and send!", transcript, recipient, draft),
                                    )
                                    .await?;
                                }
                                Err(e) => {
                                    tracing::warn!("AI parse failed: {}, using transcript as task", e);
                                    match Task::create(&pool, user.id, &transcript, None, None).await {
                                        Ok(task) => {
                                            bot.send_message(
                                                msg.chat.id,
                                                format!("🎤 \"{}\"\n\n✅ Added!\n\n📝 {}", transcript, task.title),
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
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Transcription failed: {}", e);
                            bot.send_message(msg.chat.id, "❌ Failed to transcribe voice message")
                                .await?;
                        }
                    }
                } else {
                    bot.send_message(msg.chat.id, "❌ Voice messages require AI service")
                        .await?;
                }
            } else {
                bot.send_message(msg.chat.id, "Please /start first!")
                    .await?;
            }
        }
    } else if !text.starts_with('/') && !text.is_empty() {
        // Natural language input
        if let Some(tg_user) = telegram_user {
            if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                if let Some(ai) = &ai_service {
                    let current_date = Utc::now().format("%Y-%m-%d %H:%M").to_string();
                    let user_context = context.get_context(user.id);

                    // Add user message to context
                    context.add_message(user.id, "user", text);

                    match ai.parse_input(text, &current_date, user_context.as_deref()).await {
                        Ok(ParsedInput::Task(parsed)) => {
                            tracing::info!("AI parsed task: {:?}", parsed);
                            match Task::create(&pool, user.id, &parsed.title, None, parsed.due_at.as_deref()).await {
                                Ok(task) => {
                                    if task.due_at.is_some() {
                                        let _ = Task::set_reminder_from_due(&pool, task.id).await;
                                    }

                                    let due_str = task.due_at.as_ref()
                                        .map(|d| format!("\n📅 {}", d))
                                        .unwrap_or_default();

                                    let response = format!("Added task: {}{}", task.title, due_str);
                                    context.add_message(user.id, "assistant", &response);

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
                        }
                        Ok(ParsedInput::Draft { recipient, context: ctx, draft }) => {
                            tracing::info!("AI generated draft for: {}", recipient);
                            let response = format!("Draft for {}: {}", recipient, ctx);
                            context.add_message(user.id, "assistant", &response);

                            bot.send_message(
                                msg.chat.id,
                                format!("✉️ Draft for {}:\n\n{}\n\n💡 Copy and send!", recipient, draft),
                            )
                            .await?;
                        }
                        Err(e) => {
                            tracing::warn!("AI parse failed: {}, creating task with raw text", e);
                            match Task::create(&pool, user.id, text, None, None).await {
                                Ok(task) => {
                                    bot.send_message(
                                        msg.chat.id,
                                        format!("✅ Added!\n\n📝 {}", task.title),
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
                        }
                    }
                } else {
                    // No AI service, create raw task
                    match Task::create(&pool, user.id, text, None, None).await {
                        Ok(task) => {
                            bot.send_message(
                                msg.chat.id,
                                format!("✅ Added!\n\n📝 {}", task.title),
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
    } else if let Some(snooze_data) = data.strip_prefix("snooze:") {
        // Format: snooze:task_id:minutes
        let parts: Vec<&str> = snooze_data.split(':').collect();
        if parts.len() == 2 {
            if let (Ok(task_id), Ok(minutes)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                if let Some(task) = Task::find_by_id(&pool, task_id).await {
                    let _ = Task::snooze_reminder(&pool, task_id, minutes).await;

                    let snooze_text = if minutes == 60 {
                        "1 hour"
                    } else if minutes == 1440 {
                        "tomorrow"
                    } else {
                        "later"
                    };

                    if let Some(msg) = q.message {
                        bot.edit_message_text(
                            msg.chat().id,
                            msg.id(),
                            format!("⏰ Snoozed: {}\n\nI'll remind you {}.", task.title, snooze_text),
                        )
                        .await?;
                    }

                    bot.answer_callback_query(q.id).text(format!("⏰ Snoozed for {}", snooze_text)).await?;
                }
            }
        }
    }

    Ok(())
}
