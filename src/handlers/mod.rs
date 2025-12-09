use crate::models::{Task, TaskStatus, User};
use crate::services::{AiService, ConversationContext, ParsedInput, PendingEdit, ProposedEdit};
use chrono::Utc;
use sqlx::SqlitePool;
use std::sync::Arc;
use teloxide::{
    net::Download,
    prelude::*,
    types::{ChatAction, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup, KeyboardRemove},
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
    #[command(description = "Today's tasks")]
    Today,
    #[command(description = "Settings")]
    Settings,
}

fn task_keyboard(task_id: i64) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("✅ Done", format!("done:{}", task_id)),
            InlineKeyboardButton::callback("🗑 Delete", format!("delete:{}", task_id)),
        ],
        vec![
            InlineKeyboardButton::callback("✏️ Edit", format!("edit:{}", task_id)),
            InlineKeyboardButton::callback("⏰ Remind", format!("remind:{}", task_id)),
        ],
    ])
}

/// Format task as a nice card
fn format_task(task: &Task) -> String {
    let mut lines = vec![format!("📝 <b>{}</b>", task.title)];

    if let Some(due) = &task.due_at {
        lines.push(format!("📅 {}", due));
    }

    if let Some(reminder) = &task.reminder_at {
        lines.push(format!("🔔 Reminder: {}", reminder));
    }

    lines.join("\n")
}

/// Get timezone from coordinates using a simple lookup
fn timezone_from_coords(lat: f64, lon: f64) -> String {
    // Simple timezone estimation based on longitude
    // More accurate would be to use a timezone database or API
    let offset_hours = (lon / 15.0).round() as i32;

    // Map common regions
    if lat > 35.0 && lat < 72.0 && lon > -10.0 && lon < 40.0 {
        // Europe
        if lon < 5.0 { return "Europe/London".to_string(); }
        if lon < 15.0 { return "Europe/Paris".to_string(); }
        if lon < 25.0 { return "Europe/Berlin".to_string(); }
        if lon < 35.0 { return "Europe/Kyiv".to_string(); }
        return "Europe/Moscow".to_string();
    }
    if lat > 35.0 && lat < 72.0 && lon > 35.0 && lon < 180.0 {
        // Russia/Asia
        if lon < 60.0 { return "Europe/Moscow".to_string(); }
        if lon < 90.0 { return "Asia/Yekaterinburg".to_string(); }
        if lon < 120.0 { return "Asia/Novosibirsk".to_string(); }
        return "Asia/Vladivostok".to_string();
    }
    if lat > 25.0 && lat < 50.0 && lon > -130.0 && lon < -60.0 {
        // North America
        if lon < -115.0 { return "America/Los_Angeles".to_string(); }
        if lon < -100.0 { return "America/Denver".to_string(); }
        if lon < -85.0 { return "America/Chicago".to_string(); }
        return "America/New_York".to_string();
    }

    // Fallback: UTC offset
    format!("Etc/GMT{:+}", -offset_hours)
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

                    let trial_info = if let Ok(ref u) = user {
                        if let Some(days) = u.trial_days_remaining() {
                            format!("\n\n🎁 Trial: {} days remaining", days)
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };

                    let welcome_keyboard = InlineKeyboardMarkup::new(vec![
                        vec![
                            InlineKeyboardButton::callback("📋 My tasks", "view_tasks"),
                            InlineKeyboardButton::callback("🌍 Set timezone", "settings:timezone"),
                        ],
                    ]);

                    bot.send_message(
                        msg.chat.id,
                        format!(
"👋 Welcome, {}!

I'm your AI-powered task manager.

━━━━━━━━━━━━━━━━━━━━
🚀 <b>Quick start</b>
━━━━━━━━━━━━━━━━━━━━

Just send me a task:
• \"Call mom tomorrow at 5pm\"
• \"Buy groceries\"
• 🎤 Or send a voice message!

━━━━━━━━━━━━━━━━━━━━
✨ <b>Features</b>
━━━━━━━━━━━━━━━━━━━━

📝 Natural language tasks
⏰ Smart reminders
🎤 Voice input
✉️ Message drafts
{}

💡 <b>Tip:</b> Set your timezone for accurate reminders!", tg_user.first_name, trial_info),
                    )
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .reply_markup(welcome_keyboard)
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
            Command::Today => {
                if let Some(tg_user) = telegram_user {
                    if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                        let tasks = Task::find_today_tasks(&pool, user.id).await.unwrap_or_default();

                        if tasks.is_empty() {
                            bot.send_message(msg.chat.id, "📅 No tasks for today!\n\nSend me a task with a due date.")
                                .await?;
                        } else {
                            bot.send_message(msg.chat.id, format!("📅 Today's tasks ({}):", tasks.len()))
                                .await?;

                            for task in &tasks {
                                let time = task.due_at.as_ref()
                                    .and_then(|d| d.split(' ').nth(1))
                                    .map(|t| format!(" at {}", t))
                                    .unwrap_or_default();

                                bot.send_message(
                                    msg.chat.id,
                                    format!("📝 {}{}", task.title, time),
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
            Command::Settings => {
                if let Some(tg_user) = telegram_user {
                    if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                        let settings_keyboard = InlineKeyboardMarkup::new(vec![
                            vec![
                                InlineKeyboardButton::callback("🌍 Change timezone", "settings:timezone"),
                            ],
                            vec![
                                InlineKeyboardButton::callback("⏰ Change brief time", "settings:brief_time"),
                            ],
                        ]);

                        bot.send_message(
                            msg.chat.id,
                            format!(
                                "⚙️ Settings\n\n\
                                🌍 Timezone: {}\n\
                                ⏰ Morning brief: {}\n\n\
                                Tap a button to change:",
                                user.timezone,
                                user.morning_brief_time
                            ),
                        )
                        .reply_markup(settings_keyboard)
                        .await?;
                    } else {
                        bot.send_message(msg.chat.id, "Please /start first")
                            .await?;
                    }
                }
            }
        }
    } else if let Some(voice) = msg.voice() {
        // Voice message handling with progressive updates
        if let Some(tg_user) = telegram_user {
            if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                if let Some(ai) = &ai_service {
                    // Send initial processing message
                    let progress_msg = bot.send_message(msg.chat.id, "🎤 Processing voice...")
                        .await?;

                    // Download voice file
                    let file = bot.get_file(&voice.file.id).await?;
                    let mut audio_data = Vec::new();
                    bot.download_file(&file.path, &mut audio_data).await?;

                    // Transcribe with Whisper
                    match ai.transcribe_audio(audio_data).await {
                        Ok(transcript) => {
                            tracing::info!("Transcribed: {}", transcript);

                            // Check if there's a pending edit that should use this voice input
                            if let Some(pending) = context.take_pending_edit(user.id) {
                                match pending {
                                    PendingEdit::Title(task_id) => {
                                        // Smart edit with AI - parse voice instruction
                                        if let Some(task) = Task::find_by_id(&pool, task_id).await {
                                            let current_date = Utc::now().format("%Y-%m-%d %H:%M").to_string();

                                            match ai.parse_task_edit(&task.title, &transcript, &current_date).await {
                                                Ok(parsed) => {
                                                    // Show preview and ask for confirmation
                                                    let proposed = ProposedEdit {
                                                        task_id,
                                                        old_title: task.title.clone(),
                                                        new_title: parsed.title.clone(),
                                                        new_due_at: parsed.due_at.clone(),
                                                    };

                                                    context.set_pending_edit(user.id, PendingEdit::ConfirmEdit(proposed.clone()));

                                                    let due_change = match (&task.due_at, &parsed.due_at) {
                                                        (Some(old), Some(new)) if old != new => format!("\n📅 {} → {}", old, new),
                                                        (None, Some(new)) => format!("\n📅 → {}", new),
                                                        (Some(old), None) => format!("\n📅 {} → ❌", old),
                                                        _ => String::new(),
                                                    };

                                                    let confirm_keyboard = InlineKeyboardMarkup::new(vec![vec![
                                                        InlineKeyboardButton::callback("✅ Apply", format!("confirm_edit:{}", task_id)),
                                                        InlineKeyboardButton::callback("❌ Cancel", format!("cancel_edit:{}", task_id)),
                                                    ]]);

                                                    bot.edit_message_text(
                                                        msg.chat.id,
                                                        progress_msg.id,
                                                        format!(
                                                            "🎤 \"{}\"\n\n\
                                                            📝 <b>Preview:</b>\n\n\
                                                            <s>{}</s>\n\
                                                            ↓\n\
                                                            <b>{}</b>{}\n\n\
                                                            Apply?",
                                                            transcript, task.title, parsed.title, due_change
                                                        ),
                                                    )
                                                    .parse_mode(teloxide::types::ParseMode::Html)
                                                    .reply_markup(confirm_keyboard)
                                                    .await?;
                                                }
                                                Err(e) => {
                                                    tracing::warn!("Failed to parse voice edit: {}", e);
                                                    let _ = bot.edit_message_text(
                                                        msg.chat.id,
                                                        progress_msg.id,
                                                        format!("🎤 \"{}\"\n\n❌ Couldn't understand\n\n💡 Try: \"change time to 5pm\"", transcript),
                                                    ).await;
                                                }
                                            }
                                        }
                                        return Ok(());
                                    }
                                    PendingEdit::Reminder(task_id) => {
                                        // Parse reminder time from transcript
                                        let current_date = Utc::now().format("%Y-%m-%d %H:%M").to_string();

                                        match ai.parse_reminder_time(&transcript, &current_date).await {
                                            Ok(reminder_at) => {
                                                let _ = Task::set_reminder(&pool, task_id, Some(&reminder_at)).await;

                                                if let Some(task) = Task::find_by_id(&pool, task_id).await {
                                                    bot.edit_message_text(
                                                        msg.chat.id,
                                                        progress_msg.id,
                                                        format!("🎤 \"{}\"\n\n⏰ Reminder set!\n\n📝 {}\n🔔 {}", transcript, task.title, reminder_at),
                                                    )
                                                    .reply_markup(task_keyboard(task_id))
                                                    .await?;
                                                }
                                            }
                                            Err(e) => {
                                                tracing::warn!("Failed to parse reminder time: {}", e);
                                                let _ = bot.edit_message_text(
                                                    msg.chat.id,
                                                    progress_msg.id,
                                                    format!("🎤 \"{}\"\n\n❌ Couldn't understand the time\n\n💡 Try: \"завтра в 9\" or \"in 2 hours\"", transcript),
                                                ).await;
                                            }
                                        }
                                        return Ok(());
                                    }
                                    PendingEdit::Timezone => {
                                        // Parse timezone from transcript
                                        match ai.parse_timezone(&transcript).await {
                                            Ok(timezone) => {
                                                let _ = User::update_timezone(&pool, user.id, &timezone).await;

                                                bot.edit_message_text(
                                                    msg.chat.id,
                                                    progress_msg.id,
                                                    format!("🎤 \"{}\"\n\n✅ Timezone set to: {}", transcript, timezone),
                                                )
                                                .await?;
                                            }
                                            Err(e) => {
                                                tracing::warn!("Failed to parse timezone: {}", e);
                                                let _ = bot.edit_message_text(
                                                    msg.chat.id,
                                                    progress_msg.id,
                                                    format!("🎤 \"{}\"\n\n❌ Couldn't determine timezone\n\n💡 Try a major city name", transcript),
                                                ).await;
                                            }
                                        }
                                        return Ok(());
                                    }
                                    PendingEdit::ConfirmEdit(_) => {
                                        // Voice during confirm - cancel
                                        let _ = bot.edit_message_text(
                                            msg.chat.id,
                                            progress_msg.id,
                                            "❌ Edit cancelled. Use the buttons to confirm or cancel.",
                                        ).await;
                                        return Ok(());
                                    }
                                }
                            }

                            // Update progress with transcript
                            let _ = bot.edit_message_text(
                                msg.chat.id,
                                progress_msg.id,
                                format!("🔄 \"{}\"\n\nParsing...", transcript),
                            ).await;

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

                                            bot.edit_message_text(
                                                msg.chat.id,
                                                progress_msg.id,
                                                format!("🎤 \"{}\"\n\n✅ Added!\n\n📝 {}{}", transcript, task.title, due_str),
                                            )
                                            .reply_markup(task_keyboard(task.id))
                                            .await?;
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to create task: {}", e);
                                            let _ = bot.edit_message_text(
                                                msg.chat.id,
                                                progress_msg.id,
                                                "❌ Couldn't create task\n\nSomething went wrong.\n\n💡 Try typing your task instead",
                                            ).await;
                                        }
                                    }
                                }
                                Ok(ParsedInput::Draft { recipient, context: _, draft }) => {
                                    let _ = bot.edit_message_text(
                                        msg.chat.id,
                                        progress_msg.id,
                                        format!("🎤 \"{}\"\n\n✉️ Draft for {}:\n\n{}\n\n💡 Copy and send!", transcript, recipient, draft),
                                    ).await;
                                }
                                Ok(ParsedInput::Rejected { reason }) => {
                                    let _ = bot.edit_message_text(
                                        msg.chat.id,
                                        progress_msg.id,
                                        format!("🎤 \"{}\"\n\n🤖 {}", transcript, reason),
                                    ).await;
                                }
                                Err(e) => {
                                    tracing::warn!("AI parse failed: {}, using transcript as task", e);
                                    match Task::create(&pool, user.id, &transcript, None, None).await {
                                        Ok(task) => {
                                            bot.edit_message_text(
                                                msg.chat.id,
                                                progress_msg.id,
                                                format!("🎤 \"{}\"\n\n✅ Added!\n\n📝 {}", transcript, task.title),
                                            )
                                            .reply_markup(task_keyboard(task.id))
                                            .await?;
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to create task: {}", e);
                                            let _ = bot.edit_message_text(
                                                msg.chat.id,
                                                progress_msg.id,
                                                "❌ Couldn't create task\n\nSomething went wrong.\n\n💡 Try typing your task instead",
                                            ).await;
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Transcription failed: {}", e);
                            let _ = bot.edit_message_text(
                                msg.chat.id,
                                progress_msg.id,
                                "🎤 Couldn't understand voice\n\nThe audio wasn't clear enough.\n\n💡 Try speaking closer to mic or type your task",
                            ).await;
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
    } else if let Some(location) = msg.location() {
        // Handle location for timezone detection
        if let Some(tg_user) = telegram_user {
            if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                let timezone = timezone_from_coords(location.latitude, location.longitude);
                let _ = User::update_timezone(&pool, user.id, &timezone).await;

                // Remove keyboard
                bot.send_message(
                    msg.chat.id,
                    format!("✅ Timezone set to: {}\n\nYour reminders will now use this timezone.", timezone),
                )
                .reply_markup(KeyboardRemove::new())
                .await?;
            }
        }
    } else if !text.starts_with('/') && !text.is_empty() {
        // Check for pending edit first
        if let Some(tg_user) = telegram_user {
            if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                // Handle pending edit if exists
                if let Some(pending) = context.take_pending_edit(user.id) {
                    match pending {
                        PendingEdit::Title(task_id) => {
                            // Smart edit with AI - parse instruction and show preview
                            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                                if let Some(ai) = &ai_service {
                                    let _ = bot.send_chat_action(msg.chat.id, ChatAction::Typing).await;
                                    let current_date = Utc::now().format("%Y-%m-%d %H:%M").to_string();

                                    match ai.parse_task_edit(&task.title, text, &current_date).await {
                                        Ok(parsed) => {
                                            // Show preview and ask for confirmation
                                            let proposed = ProposedEdit {
                                                task_id,
                                                old_title: task.title.clone(),
                                                new_title: parsed.title.clone(),
                                                new_due_at: parsed.due_at.clone(),
                                            };

                                            // Store proposed edit for confirmation
                                            context.set_pending_edit(user.id, PendingEdit::ConfirmEdit(proposed.clone()));

                                            let due_change = match (&task.due_at, &parsed.due_at) {
                                                (Some(old), Some(new)) if old != new => format!("\n📅 {} → {}", old, new),
                                                (None, Some(new)) => format!("\n📅 → {}", new),
                                                (Some(old), None) => format!("\n📅 {} → ❌", old),
                                                _ => String::new(),
                                            };

                                            let confirm_keyboard = InlineKeyboardMarkup::new(vec![vec![
                                                InlineKeyboardButton::callback("✅ Apply", format!("confirm_edit:{}", task_id)),
                                                InlineKeyboardButton::callback("❌ Cancel", format!("cancel_edit:{}", task_id)),
                                            ]]);

                                            bot.send_message(
                                                msg.chat.id,
                                                format!(
                                                    "📝 <b>Preview:</b>\n\n\
                                                    <s>{}</s>\n\
                                                    ↓\n\
                                                    <b>{}</b>{}\n\n\
                                                    Apply?",
                                                    task.title, parsed.title, due_change
                                                ),
                                            )
                                            .parse_mode(teloxide::types::ParseMode::Html)
                                            .reply_markup(confirm_keyboard)
                                            .await?;
                                        }
                                        Err(e) => {
                                            tracing::warn!("Failed to parse edit: {}", e);
                                            bot.send_message(
                                                msg.chat.id,
                                                "❌ Couldn't understand the edit instruction\n\n💡 Try: \"change time to 5pm\" or \"replace John with Mike\"",
                                            )
                                            .await?;
                                        }
                                    }
                                } else {
                                    // No AI - just replace title
                                    let _ = Task::update(&pool, task_id, Some(text), None).await;
                                    bot.send_message(msg.chat.id, format!("✅ Updated: {}", text)).await?;
                                }
                            } else {
                                bot.send_message(msg.chat.id, "❌ Task not found").await?;
                            }
                            return Ok(());
                        }
                        PendingEdit::ConfirmEdit(_) => {
                            // User sent text instead of clicking button - cancel
                            bot.send_message(
                                msg.chat.id,
                                "❌ Edit cancelled. Use the buttons to confirm or cancel.",
                            )
                            .await?;
                            return Ok(());
                        }
                        PendingEdit::Reminder(task_id) => {
                            // Parse custom reminder time with AI
                            if let Some(ai) = &ai_service {
                                let _ = bot.send_chat_action(msg.chat.id, ChatAction::Typing).await;
                                let current_date = Utc::now().format("%Y-%m-%d %H:%M").to_string();

                                match ai.parse_reminder_time(text, &current_date).await {
                                    Ok(reminder_at) => {
                                        let _ = Task::set_reminder(&pool, task_id, Some(&reminder_at)).await;

                                        if let Some(task) = Task::find_by_id(&pool, task_id).await {
                                            bot.send_message(
                                                msg.chat.id,
                                                format!("⏰ Reminder set!\n\n📝 {}\n🔔 {}", task.title, reminder_at),
                                            )
                                            .reply_markup(task_keyboard(task_id))
                                            .await?;
                                        }
                                    }
                                    Err(e) => {
                                        tracing::warn!("Failed to parse reminder time: {}", e);
                                        bot.send_message(
                                            msg.chat.id,
                                            "❌ Couldn't understand the time\n\n💡 Try: \"завтра в 9\" or \"in 2 hours\"",
                                        )
                                        .await?;
                                    }
                                }
                            } else {
                                bot.send_message(msg.chat.id, "❌ AI service required for custom reminders")
                                    .await?;
                            }
                            return Ok(());
                        }
                        PendingEdit::Timezone => {
                            // Parse timezone from city name with AI
                            if let Some(ai) = &ai_service {
                                let _ = bot.send_chat_action(msg.chat.id, ChatAction::Typing).await;

                                match ai.parse_timezone(text).await {
                                    Ok(timezone) => {
                                        let _ = User::update_timezone(&pool, user.id, &timezone).await;

                                        bot.send_message(
                                            msg.chat.id,
                                            format!("✅ Timezone set to: {}\n\nYour reminders will now use this timezone.", timezone),
                                        )
                                        .reply_markup(KeyboardRemove::new())
                                        .await?;
                                    }
                                    Err(e) => {
                                        tracing::warn!("Failed to parse timezone: {}", e);
                                        bot.send_message(
                                            msg.chat.id,
                                            "❌ Couldn't determine timezone\n\n💡 Try a major city name like \"Moscow\" or \"New York\"",
                                        )
                                        .await?;
                                    }
                                }
                            } else {
                                bot.send_message(msg.chat.id, "❌ AI service required")
                                    .await?;
                            }
                            return Ok(());
                        }
                    }
                }

                // Natural language input
                if let Some(ai) = &ai_service {
                    // Show typing indicator while AI processes
                    let _ = bot.send_chat_action(msg.chat.id, ChatAction::Typing).await;

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
                                    bot.send_message(msg.chat.id, "❌ Couldn't create task\n\nSomething went wrong on our end.\n\n💡 Try: \"Buy milk tomorrow at 5pm\"")
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
                        Ok(ParsedInput::Rejected { reason }) => {
                            tracing::info!("AI rejected input: {}", reason);
                            bot.send_message(
                                msg.chat.id,
                                format!("🤖 {}", reason),
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
                                    bot.send_message(msg.chat.id, "❌ Couldn't create task\n\nSomething went wrong on our end.\n\n💡 Try: \"Buy milk tomorrow at 5pm\"")
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
                            bot.send_message(msg.chat.id, "❌ Couldn't create task\n\nSomething went wrong on our end.\n\n💡 Try: \"Buy milk tomorrow at 5pm\"")
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
    context: Arc<ConversationContext>,
) -> ResponseResult<()> {
    let data = q.data.unwrap_or_default();

    if let Some(task_id_str) = data.strip_prefix("done:") {
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                let _ = Task::update_status(&pool, task_id, TaskStatus::Done).await;

                // Update message
                if let Some(msg) = &q.message {
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        format!("✅ {}", task.title),
                    )
                    .await?;
                }

                // Show next task suggestion
                let telegram_id = q.from.id.0 as i64;
                if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
                    let pending = Task::find_pending_by_user(&pool, user.id).await.unwrap_or_default();

                    if let Some(msg) = &q.message {
                        if let Some(next_task) = pending.first() {
                            let due_str = next_task.due_at.as_ref()
                                .map(|d| format!("\n📅 {}", d))
                                .unwrap_or_default();

                            let next_keyboard = InlineKeyboardMarkup::new(vec![vec![
                                InlineKeyboardButton::callback("✅ Do it", format!("done:{}", next_task.id)),
                                InlineKeyboardButton::callback("📋 View all", "view_tasks".to_string()),
                            ]]);

                            bot.send_message(
                                msg.chat().id,
                                format!("🎯 Next up:\n\n📝 {}{}", next_task.title, due_str),
                            )
                            .reply_markup(next_keyboard)
                            .await?;
                        } else {
                            bot.send_message(
                                msg.chat().id,
                                "🎉 All done! No more pending tasks.",
                            )
                            .await?;
                        }
                    }
                }

                bot.answer_callback_query(q.id).text("✅ Done!").await?;
            }
        }
    } else if let Some(task_id_str) = data.strip_prefix("delete:") {
        // Show confirmation dialog instead of deleting immediately
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                if let Some(msg) = q.message {
                    let confirm_keyboard = InlineKeyboardMarkup::new(vec![vec![
                        InlineKeyboardButton::callback("🗑 Yes, delete", format!("confirm_delete:{}", task_id)),
                        InlineKeyboardButton::callback("↩️ Cancel", format!("cancel_delete:{}", task_id)),
                    ]]);

                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        format!("⚠️ Delete \"{}\"?", task.title),
                    )
                    .reply_markup(confirm_keyboard)
                    .await?;
                }

                bot.answer_callback_query(q.id).await?;
            }
        }
    } else if let Some(task_id_str) = data.strip_prefix("confirm_delete:") {
        // Actually delete the task
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
    } else if let Some(task_id_str) = data.strip_prefix("cancel_delete:") {
        // Restore original task view
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                if let Some(msg) = q.message {
                    let due_str = task.due_at.as_ref()
                        .map(|d| format!("\n📅 {}", d))
                        .unwrap_or_default();

                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        format!("📝 {}{}", task.title, due_str),
                    )
                    .reply_markup(task_keyboard(task_id))
                    .await?;
                }

                bot.answer_callback_query(q.id).text("Cancelled").await?;
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
    } else if data == "view_tasks" {
        // Show all pending tasks
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let tasks = Task::find_pending_by_user(&pool, user.id).await.unwrap_or_default();

            if let Some(msg) = &q.message {
                if tasks.is_empty() {
                    bot.send_message(msg.chat().id, "📋 No tasks yet!\n\nSend me a message to create one.")
                        .await?;
                } else {
                    for task in &tasks {
                        let due = task.due_at.as_ref()
                            .map(|d| format!("\n📅 {}", d))
                            .unwrap_or_default();

                        bot.send_message(
                            msg.chat().id,
                            format!("📝 {}{}", task.title, due),
                        )
                        .reply_markup(task_keyboard(task.id))
                        .await?;
                    }
                }
            }
        }

        bot.answer_callback_query(q.id).await?;
    } else if data == "settings:timezone" {
        // Show timezone options with auto-detect
        let tz_keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("📍 Auto-detect", "tz:auto"),
                InlineKeyboardButton::callback("🏙 Type city", "tz:city"),
            ],
            vec![
                InlineKeyboardButton::callback("🇺🇸 New York", "tz:America/New_York"),
                InlineKeyboardButton::callback("🇺🇸 Los Angeles", "tz:America/Los_Angeles"),
            ],
            vec![
                InlineKeyboardButton::callback("🇬🇧 London", "tz:Europe/London"),
                InlineKeyboardButton::callback("🇪🇺 Berlin", "tz:Europe/Berlin"),
            ],
            vec![
                InlineKeyboardButton::callback("🇷🇺 Moscow", "tz:Europe/Moscow"),
                InlineKeyboardButton::callback("🇺🇦 Kyiv", "tz:Europe/Kyiv"),
            ],
            vec![
                InlineKeyboardButton::callback("🇯🇵 Tokyo", "tz:Asia/Tokyo"),
                InlineKeyboardButton::callback("↩️ Back", "settings:back"),
            ],
        ]);

        if let Some(msg) = &q.message {
            bot.edit_message_text(
                msg.chat().id,
                msg.id(),
                "🌍 Select your timezone:\n\n📍 Auto-detect uses your location\n🏙 Type city lets you enter any city",
            )
            .reply_markup(tz_keyboard)
            .await?;
        }

        bot.answer_callback_query(q.id).await?;
    } else if data == "tz:auto" {
        // Request location for auto-detect
        if let Some(msg) = &q.message {
            let location_keyboard = KeyboardMarkup::new(vec![vec![
                KeyboardButton::new("📍 Share my location").request(teloxide::types::ButtonRequest::Location),
            ]])
            .resize_keyboard()
            .one_time_keyboard();

            bot.send_message(
                msg.chat().id,
                "📍 Tap the button below to share your location.\n\nI'll detect your timezone automatically.",
            )
            .reply_markup(location_keyboard)
            .await?;
        }

        bot.answer_callback_query(q.id).await?;
    } else if data == "tz:city" {
        // Ask for city name
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            context.set_pending_edit(user.id, PendingEdit::Timezone);
        }

        if let Some(msg) = &q.message {
            bot.send_message(
                msg.chat().id,
                "🏙 Type your city name:\n\nExamples: Moscow, New York, Tokyo, Dubai",
            )
            .await?;
        }

        bot.answer_callback_query(q.id).text("Type your city").await?;
    } else if let Some(tz) = data.strip_prefix("tz:") {
        // Set timezone
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let _ = User::update_timezone(&pool, user.id, tz).await;

            if let Some(msg) = &q.message {
                bot.edit_message_text(
                    msg.chat().id,
                    msg.id(),
                    format!("✅ Timezone set to: {}", tz),
                )
                .await?;
            }

            bot.answer_callback_query(q.id).text("✅ Timezone updated").await?;
        }
    } else if data == "settings:brief_time" {
        // Show brief time options
        let time_keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("06:00", "brief:06:00"),
                InlineKeyboardButton::callback("07:00", "brief:07:00"),
                InlineKeyboardButton::callback("08:00", "brief:08:00"),
            ],
            vec![
                InlineKeyboardButton::callback("09:00", "brief:09:00"),
                InlineKeyboardButton::callback("10:00", "brief:10:00"),
                InlineKeyboardButton::callback("11:00", "brief:11:00"),
            ],
            vec![
                InlineKeyboardButton::callback("↩️ Back", "settings:back"),
            ],
        ]);

        if let Some(msg) = &q.message {
            bot.edit_message_text(
                msg.chat().id,
                msg.id(),
                "⏰ Select morning brief time (UTC):",
            )
            .reply_markup(time_keyboard)
            .await?;
        }

        bot.answer_callback_query(q.id).await?;
    } else if let Some(time) = data.strip_prefix("brief:") {
        // Set brief time
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let _ = User::update_morning_brief_time(&pool, user.id, time).await;

            if let Some(msg) = &q.message {
                bot.edit_message_text(
                    msg.chat().id,
                    msg.id(),
                    format!("✅ Morning brief time set to: {} UTC", time),
                )
                .await?;
            }

            bot.answer_callback_query(q.id).text("✅ Brief time updated").await?;
        }
    } else if data == "settings:back" {
        // Back to settings
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let settings_keyboard = InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::callback("🌍 Change timezone", "settings:timezone"),
                ],
                vec![
                    InlineKeyboardButton::callback("⏰ Change brief time", "settings:brief_time"),
                ],
            ]);

            if let Some(msg) = &q.message {
                bot.edit_message_text(
                    msg.chat().id,
                    msg.id(),
                    format!(
                        "⚙️ Settings\n\n\
                        🌍 Timezone: {}\n\
                        ⏰ Morning brief: {}\n\n\
                        Tap a button to change:",
                        user.timezone,
                        user.morning_brief_time
                    ),
                )
                .reply_markup(settings_keyboard)
                .await?;
            }
        }

        bot.answer_callback_query(q.id).await?;
    } else if let Some(task_id_str) = data.strip_prefix("edit:") {
        // Show edit options
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                let edit_keyboard = InlineKeyboardMarkup::new(vec![
                    vec![
                        InlineKeyboardButton::callback("📝 Edit title", format!("edit_title:{}", task_id)),
                        InlineKeyboardButton::callback("📅 Edit date", format!("edit_date:{}", task_id)),
                    ],
                    vec![
                        InlineKeyboardButton::callback("↩️ Back", format!("cancel_delete:{}", task_id)),
                    ],
                ]);

                if let Some(msg) = &q.message {
                    let due_str = task.due_at.as_ref()
                        .map(|d| format!("\n📅 {}", d))
                        .unwrap_or_else(|| "\n📅 No due date".to_string());

                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        format!("✏️ Edit task:\n\n📝 {}{}\n\nWhat would you like to change?", task.title, due_str),
                    )
                    .reply_markup(edit_keyboard)
                    .await?;
                }

                bot.answer_callback_query(q.id).await?;
            }
        }
    } else if let Some(task_id_str) = data.strip_prefix("edit_title:") {
        // Prompt user to send new title
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                // Get user to set pending edit
                let telegram_id = q.from.id.0 as i64;
                if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
                    context.set_pending_edit(user.id, PendingEdit::Title(task_id));
                }

                if let Some(msg) = &q.message {
                    let cancel_keyboard = InlineKeyboardMarkup::new(vec![vec![
                        InlineKeyboardButton::callback("↩️ Cancel", format!("cancel_edit:{}", task_id)),
                    ]]);

                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        format!("📝 Current: {}\n\nSend new title:", task.title),
                    )
                    .reply_markup(cancel_keyboard)
                    .await?;
                }

                bot.answer_callback_query(q.id).text("Send new title").await?;
            }
        }
    } else if let Some(task_id_str) = data.strip_prefix("confirm_edit:") {
        // Apply confirmed edit
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            let telegram_id = q.from.id.0 as i64;
            if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
                if let Some(PendingEdit::ConfirmEdit(proposed)) = context.take_pending_edit(user.id) {
                    // Apply the changes
                    let _ = Task::update(&pool, task_id, Some(&proposed.new_title), Some(proposed.new_due_at.as_deref())).await;

                    if let Some(task) = Task::find_by_id(&pool, task_id).await {
                        if let Some(msg) = &q.message {
                            let due_str = task.due_at.as_ref()
                                .map(|d| format!("\n📅 {}", d))
                                .unwrap_or_default();

                            bot.edit_message_text(
                                msg.chat().id,
                                msg.id(),
                                format!("✅ Updated!\n\n📝 {}{}", task.title, due_str),
                            )
                            .reply_markup(task_keyboard(task_id))
                            .await?;
                        }
                    }

                    bot.answer_callback_query(q.id).text("✅ Changes applied").await?;
                }
            }
        }
    } else if let Some(task_id_str) = data.strip_prefix("cancel_edit:") {
        // Cancel pending edit and restore task view
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            // Clear pending edit
            let telegram_id = q.from.id.0 as i64;
            if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
                let _ = context.take_pending_edit(user.id);
            }

            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                if let Some(msg) = &q.message {
                    let due_str = task.due_at.as_ref()
                        .map(|d| format!("\n📅 {}", d))
                        .unwrap_or_default();

                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        format!("📝 {}{}", task.title, due_str),
                    )
                    .reply_markup(task_keyboard(task_id))
                    .await?;
                }
            }

            bot.answer_callback_query(q.id).text("Cancelled").await?;
        }
    } else if let Some(task_id_str) = data.strip_prefix("edit_date:") {
        // Show date options
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if Task::find_by_id(&pool, task_id).await.is_some() {
                let date_keyboard = InlineKeyboardMarkup::new(vec![
                    vec![
                        InlineKeyboardButton::callback("📅 Today", format!("set_date:{}:today", task_id)),
                        InlineKeyboardButton::callback("📅 Tomorrow", format!("set_date:{}:tomorrow", task_id)),
                    ],
                    vec![
                        InlineKeyboardButton::callback("📅 Next week", format!("set_date:{}:next_week", task_id)),
                        InlineKeyboardButton::callback("🚫 Remove date", format!("set_date:{}:none", task_id)),
                    ],
                    vec![
                        InlineKeyboardButton::callback("↩️ Back", format!("edit:{}", task_id)),
                    ],
                ]);

                if let Some(msg) = &q.message {
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        "📅 Select new due date:",
                    )
                    .reply_markup(date_keyboard)
                    .await?;
                }

                bot.answer_callback_query(q.id).await?;
            }
        }
    } else if let Some(date_data) = data.strip_prefix("set_date:") {
        // Format: set_date:task_id:option
        let parts: Vec<&str> = date_data.split(':').collect();
        if parts.len() == 2 {
            if let Ok(task_id) = parts[0].parse::<i64>() {
                let option = parts[1];
                let new_due = match option {
                    "today" => Some(Utc::now().format("%Y-%m-%d 18:00").to_string()),
                    "tomorrow" => Some((Utc::now() + chrono::Duration::days(1)).format("%Y-%m-%d 18:00").to_string()),
                    "next_week" => Some((Utc::now() + chrono::Duration::days(7)).format("%Y-%m-%d 18:00").to_string()),
                    "none" => None,
                    _ => None,
                };

                let _ = Task::update(&pool, task_id, None, Some(new_due.as_deref())).await;

                // Update reminder if due date changed
                if new_due.is_some() {
                    let _ = Task::set_reminder_from_due(&pool, task_id).await;
                } else {
                    let _ = Task::set_reminder(&pool, task_id, None).await;
                }

                if let Some(task) = Task::find_by_id(&pool, task_id).await {
                    if let Some(msg) = &q.message {
                        let due_str = task.due_at.as_ref()
                            .map(|d| format!("\n📅 {}", d))
                            .unwrap_or_default();

                        bot.edit_message_text(
                            msg.chat().id,
                            msg.id(),
                            format!("✅ Updated!\n\n📝 {}{}", task.title, due_str),
                        )
                        .reply_markup(task_keyboard(task_id))
                        .await?;
                    }
                }

                bot.answer_callback_query(q.id).text("✅ Date updated").await?;
            }
        }
    } else if let Some(task_id_str) = data.strip_prefix("remind:") {
        // Show reminder options
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                let reminder_str = task.reminder_at.as_ref()
                    .map(|r| format!("🔔 Current: {}", r))
                    .unwrap_or_else(|| "No reminder set".to_string());

                let remind_keyboard = InlineKeyboardMarkup::new(vec![
                    vec![
                        InlineKeyboardButton::callback("⏰ 30 min", format!("set_remind:{}:30", task_id)),
                        InlineKeyboardButton::callback("⏰ 1 hour", format!("set_remind:{}:60", task_id)),
                        InlineKeyboardButton::callback("⏰ 3 hours", format!("set_remind:{}:180", task_id)),
                    ],
                    vec![
                        InlineKeyboardButton::callback("⏰ Tomorrow 9am", format!("set_remind:{}:tomorrow", task_id)),
                        InlineKeyboardButton::callback("✍️ Custom", format!("set_remind:{}:custom", task_id)),
                    ],
                    vec![
                        InlineKeyboardButton::callback("🚫 Remove", format!("set_remind:{}:none", task_id)),
                        InlineKeyboardButton::callback("↩️ Back", format!("cancel_delete:{}", task_id)),
                    ],
                ]);

                if let Some(msg) = &q.message {
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        format!("⏰ Set reminder for:\n\n📝 {}\n\n{}\n\n✍️ Custom: send text or 🎤 voice", task.title, reminder_str),
                    )
                    .reply_markup(remind_keyboard)
                    .await?;
                }

                bot.answer_callback_query(q.id).await?;
            }
        }
    } else if let Some(remind_data) = data.strip_prefix("set_remind:") {
        // Format: set_remind:task_id:option
        let parts: Vec<&str> = remind_data.split(':').collect();
        if parts.len() == 2 {
            if let Ok(task_id) = parts[0].parse::<i64>() {
                let option = parts[1];

                // Handle custom reminder - set pending edit and ask for input
                if option == "custom" {
                    let telegram_id = q.from.id.0 as i64;
                    if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
                        context.set_pending_edit(user.id, PendingEdit::Reminder(task_id));
                    }

                    if let Some(task) = Task::find_by_id(&pool, task_id).await {
                        if let Some(msg) = &q.message {
                            let cancel_keyboard = InlineKeyboardMarkup::new(vec![vec![
                                InlineKeyboardButton::callback("↩️ Cancel", format!("cancel_edit:{}", task_id)),
                            ]]);

                            bot.edit_message_text(
                                msg.chat().id,
                                msg.id(),
                                format!("⏰ When to remind about:\n📝 {}\n\nSend time (text or 🎤 voice):\n• \"завтра в 15:00\"\n• \"через 2 часа\"\n• \"monday morning\"", task.title),
                            )
                            .reply_markup(cancel_keyboard)
                            .await?;
                        }
                    }

                    bot.answer_callback_query(q.id).text("Send reminder time").await?;
                    return Ok(());
                }

                let (reminder_time, confirm_text) = match option {
                    "30" => {
                        let time = (Utc::now() + chrono::Duration::minutes(30)).format("%Y-%m-%d %H:%M").to_string();
                        (Some(time), "in 30 minutes")
                    },
                    "60" => {
                        let time = (Utc::now() + chrono::Duration::minutes(60)).format("%Y-%m-%d %H:%M").to_string();
                        (Some(time), "in 1 hour")
                    },
                    "180" => {
                        let time = (Utc::now() + chrono::Duration::minutes(180)).format("%Y-%m-%d %H:%M").to_string();
                        (Some(time), "in 3 hours")
                    },
                    "tomorrow" => {
                        let tomorrow = Utc::now() + chrono::Duration::days(1);
                        let time = tomorrow.format("%Y-%m-%d").to_string() + " 09:00";
                        (Some(time), "tomorrow at 9am")
                    },
                    "none" => (None, "removed"),
                    _ => (None, "removed"),
                };

                let _ = Task::set_reminder(&pool, task_id, reminder_time.as_deref()).await;

                if let Some(task) = Task::find_by_id(&pool, task_id).await {
                    if let Some(msg) = &q.message {
                        let due_str = task.due_at.as_ref()
                            .map(|d| format!("\n📅 {}", d))
                            .unwrap_or_default();

                        let reminder_msg = if option == "none" {
                            "🔕 Reminder removed".to_string()
                        } else {
                            format!("⏰ Reminder set {}", confirm_text)
                        };

                        bot.edit_message_text(
                            msg.chat().id,
                            msg.id(),
                            format!("{}\n\n📝 {}{}", reminder_msg, task.title, due_str),
                        )
                        .reply_markup(task_keyboard(task_id))
                        .await?;
                    }
                }

                bot.answer_callback_query(q.id).text(format!("⏰ Reminder {}", confirm_text)).await?;
            }
        }
    }

    Ok(())
}
