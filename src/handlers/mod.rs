use crate::i18n::I18n;
use crate::models::{Task, TaskStatus, User};
use crate::services::{AiService, ConversationContext, ParsedInput, PendingEdit, PendingTask, ProposedEdit, RateLimiter, RateLimits};
use fluent::FluentArgs;
use chrono::Utc;
use sqlx::SqlitePool;
use std::sync::Arc;
use teloxide::{
    net::Download,
    prelude::*,
    types::{ChatAction, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup, KeyboardRemove, LabeledPrice, PreCheckoutQuery},
    utils::command::BotCommands,
};

// Subscription prices in Telegram Stars
const PRICE_1_MONTH: i32 = 250;  // ~$5
const PRICE_3_MONTHS: i32 = 600; // ~$12 (save ~$3)
const PRICE_12_MONTHS: i32 = 2000; // ~$40 (save ~$20)

/// Check subscription status and return appropriate message if expired
fn check_subscription(user: &User, i18n: &I18n) -> Option<String> {
    if user.has_active_subscription() {
        None
    } else {
        Some(i18n.t(user.lang(), "subscription-expired-full"))
    }
}

/// Get rate limits based on subscription status
fn get_rate_limits(user: &User) -> RateLimits {
    if user.subscription_type.as_deref() == Some("trial") {
        RateLimits::trial()
    } else {
        RateLimits::paid()
    }
}

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
    #[command(description = "Contact support")]
    Support,
    #[command(description = "Invite friends & get bonus")]
    Invite,
    #[command(description = "Admin panel", hide)]
    Admin,
}

const BOT_USERNAME: &str = "aitodolist_bot";

/// Check if user is admin
fn is_admin(telegram_id: i64) -> bool {
    std::env::var("ADMIN_IDS")
        .unwrap_or_default()
        .split(',')
        .any(|id| id.trim().parse::<i64>().ok() == Some(telegram_id))
}

/// Get support chat ID from environment
fn get_support_chat_id() -> Option<ChatId> {
    std::env::var("SUPPORT_CHAT_ID")
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .map(ChatId)
}

fn task_keyboard(task_id: i64, i18n: &I18n, lang: &str) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback(&i18n.t(lang, "btn-done"), format!("done:{}", task_id)),
            InlineKeyboardButton::callback(&i18n.t(lang, "btn-delete"), format!("delete:{}", task_id)),
        ],
        vec![
            InlineKeyboardButton::callback(&i18n.t(lang, "btn-edit"), format!("edit:{}", task_id)),
            InlineKeyboardButton::callback("⏰", format!("remind:{}", task_id)),
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

/// Send subscription invoice with Telegram Stars
async fn send_subscription_invoice(
    bot: &Bot,
    chat_id: ChatId,
    months: i32,
    user_id: i64,
) -> ResponseResult<Message> {
    let (title, description, price, payload_months): (&str, &str, u32, &str) = match months {
        1 => (
            "AI Todolist — 1 Month",
            "Full access to AI-powered task management for 1 month",
            PRICE_1_MONTH as u32,
            "1",
        ),
        3 => (
            "AI Todolist — 3 Months",
            "Full access to AI-powered task management for 3 months. Save ~$1!",
            PRICE_3_MONTHS as u32,
            "3",
        ),
        12 => (
            "AI Todolist — 12 Months",
            "Full access to AI-powered task management for 12 months. Save ~$11!",
            PRICE_12_MONTHS as u32,
            "12",
        ),
        _ => {
            // Should not happen - invalid months value
            tracing::error!("Invalid subscription months: {}", months);
            return bot.send_message(chat_id, "❌ Invalid subscription option").await;
        }
    };

    // payload format: "sub:{user_id}:{months}"
    let payload = format!("sub:{}:{}", user_id, payload_months);

    bot.send_invoice(
        chat_id,
        title,
        description,
        payload,
        "", // Empty provider_token for Telegram Stars
        "XTR", // Telegram Stars currency
        vec![LabeledPrice::new("Subscription", price)],
    )
    .await
}

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    pool: Arc<SqlitePool>,
    ai_service: Option<Arc<AiService>>,
    context: Arc<ConversationContext>,
    i18n: Arc<I18n>,
) -> ResponseResult<()> {
    let text = msg.text().unwrap_or_default();
    let telegram_user = msg.from.as_ref();

    // Handle successful payment
    if let Some(payment) = msg.successful_payment() {
        tracing::info!("Successful payment: {:?}", payment);

        // Parse payload: "sub:{user_id}:{months}"
        let parts: Vec<&str> = payment.invoice_payload.split(':').collect();
        if parts.len() == 3 && parts[0] == "sub" {
            if let (Ok(user_id), Ok(months)) = (parts[1].parse::<i64>(), parts[2].parse::<i64>()) {
                // Activate subscription
                if let Err(e) = User::activate_subscription(&pool, user_id, months).await {
                    tracing::error!("Failed to activate subscription: {}", e);
                    bot.send_message(msg.chat.id, "❌ Payment received but failed to activate subscription. Please contact support.")
                        .await?;
                } else {
                    let months_text = match months {
                        1 => "1 month",
                        3 => "3 months",
                        12 => "12 months",
                        _ => "subscription",
                    };
                    bot.send_message(
                        msg.chat.id,
                        format!(
                            "🎉 Thank you for subscribing!\n\n\
                            ✅ Your {} subscription is now active.\n\n\
                            Enjoy unlimited AI-powered task management!\n\n\
                            💡 Send me a task to get started.",
                            months_text
                        ),
                    )
                    .await?;
                    tracing::info!("Subscription activated for user {} for {} months", user_id, months);
                }
            }
        }
        return Ok(());
    }

    // Handle admin reply to support message
    if let Some(support_chat) = get_support_chat_id() {
        if msg.chat.id == support_chat {
            if let Some(reply_to) = msg.reply_to_message() {
                // Check if this is a reply to a support message
                if let Some(reply_text) = reply_to.text() {
                    // Extract chat_id from "chat:XXXXXXX" pattern
                    if let Some(chat_id_str) = reply_text
                        .split("chat:")
                        .nth(1)
                        .and_then(|s| s.split(|c: char| !c.is_ascii_digit() && c != '-').next())
                    {
                        if let Ok(user_chat_id) = chat_id_str.parse::<i64>() {
                            // Forward admin's reply to user
                            let admin_reply = format!(
                                "📬 <b>Support Response</b>\n\n{}",
                                text
                            );

                            match bot.send_message(ChatId(user_chat_id), &admin_reply)
                                .parse_mode(teloxide::types::ParseMode::Html)
                                .await
                            {
                                Ok(_) => {
                                    bot.send_message(msg.chat.id, "✅ Reply sent to user")
                                        .await?;
                                }
                                Err(e) => {
                                    tracing::error!("Failed to send reply to user: {}", e);
                                    bot.send_message(msg.chat.id, "❌ Failed to send reply")
                                        .await?;
                                }
                            }
                            return Ok(());
                        }
                    }
                }
            }
        }
    }

    if let Ok(cmd) = Command::parse(text, "") {
        match cmd {
            Command::Start => {
                if let Some(tg_user) = telegram_user {
                    // Check for referral deep link: /start ref...
                    let referral_code = text.strip_prefix("/start ")
                        .filter(|s| s.starts_with("ref"));

                    let (user, is_new_referral) = if let Some(code) = referral_code {
                        // Check if user already exists
                        if let Some(existing) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                            (Ok(existing), false)
                        } else if let Some(referrer) = User::find_by_referral_code(&pool, code).await {
                            // New user with valid referral
                            let new_user = User::create_with_referral(
                                &pool,
                                tg_user.id.0 as i64,
                                tg_user.username.as_deref(),
                                Some(&tg_user.first_name),
                                referrer.id,
                            ).await;
                            (new_user, true)
                        } else {
                            // Invalid referral code, create normally
                            (User::create(&pool, tg_user.id.0 as i64, tg_user.username.as_deref(), Some(&tg_user.first_name)).await, false)
                        }
                    } else {
                        (User::get_or_create(
                            &pool,
                            tg_user.id.0 as i64,
                            tg_user.username.as_deref(),
                            Some(&tg_user.first_name),
                        ).await, false)
                    };

                    let lang = if let Ok(ref u) = user { u.lang() } else { "en" };

                    let trial_info = if let Ok(ref u) = user {
                        if let Some(days) = u.trial_days_remaining() {
                            i18n.t(lang, "welcome-trial")
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };

                    let referral_bonus = if is_new_referral {
                        format!("\n{}", i18n.t(lang, "welcome-referral-bonus"))
                    } else {
                        String::new()
                    };

                    let welcome_keyboard = InlineKeyboardMarkup::new(vec![
                        vec![
                            InlineKeyboardButton::callback(
                                &i18n.t(lang, "btn-settings"),
                                "settings"
                            ),
                        ],
                    ]);

                    let mut args = FluentArgs::new();
                    args.set("name", tg_user.first_name.clone());
                    let welcome_text = i18n.t_args(lang, "welcome", &args);

                    bot.send_message(
                        msg.chat.id,
                        format!("{}\n\n{}{}", welcome_text, trial_info, referral_bonus),
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
                        let lang = user.lang();
                        let tasks = Task::find_pending_by_user(&pool, user.id).await.unwrap_or_default();

                        if tasks.is_empty() {
                            bot.send_message(msg.chat.id, i18n.t(lang, "tasks-empty"))
                                .await?;
                        } else {
                            // Check for stale tasks first
                            let stale_tasks = Task::find_stale(&pool, user.id, 7).await.unwrap_or_default();
                            if !stale_tasks.is_empty() {
                                let stale_keyboard = InlineKeyboardMarkup::new(vec![
                                    vec![
                                        InlineKeyboardButton::callback(&i18n.t(lang, "btn-stale-review"), "stale:review"),
                                        InlineKeyboardButton::callback(&i18n.t(lang, "btn-stale-keep-all"), "stale:keep"),
                                    ]
                                ]);

                                let mut args = FluentArgs::new();
                                args.set("count", stale_tasks.len() as i64);

                                bot.send_message(
                                    msg.chat.id,
                                    i18n.t_args(lang, "stale-warning-inline", &args),
                                )
                                .reply_markup(stale_keyboard)
                                .await?;
                            }

                            // Group tasks by first tag
                            let mut grouped: std::collections::HashMap<String, Vec<&Task>> = std::collections::HashMap::new();
                            for task in &tasks {
                                let tag = task.tags.as_ref()
                                    .and_then(|t| t.split(',').next())
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .unwrap_or_else(|| "other".to_string());
                                grouped.entry(tag).or_default().push(task);
                            }

                            // Sort tags: known tags first, "other" last
                            let mut tags: Vec<_> = grouped.keys().cloned().collect();
                            tags.sort_by(|a, b| {
                                if a == "other" { std::cmp::Ordering::Greater }
                                else if b == "other" { std::cmp::Ordering::Less }
                                else { a.cmp(b) }
                            });

                            // Send grouped tasks
                            for tag in tags {
                                if let Some(tag_tasks) = grouped.get(&tag) {
                                    let tag_emoji = match tag.as_str() {
                                        "work" => "💼",
                                        "personal" => "👤",
                                        "shopping" => "🛒",
                                        "health" => "❤️",
                                        "home" => "🏠",
                                        "finance" => "💰",
                                        "other" => "📦",
                                        _ => "🏷️",
                                    };

                                    // Header with tag
                                    bot.send_message(
                                        msg.chat.id,
                                        format!("{} {} ({})", tag_emoji, tag.to_uppercase(), tag_tasks.len()),
                                    ).await?;

                                    // Tasks in this group
                                    for task in tag_tasks {
                                        let due = task.due_at.as_ref()
                                            .map(|d| format!("\n📅 {}", d))
                                            .unwrap_or_default();

                                        bot.send_message(
                                            msg.chat.id,
                                            format!("  📝 {}{}", task.title, due),
                                        )
                                        .reply_markup(task_keyboard(task.id, &i18n, lang))
                                        .await?;
                                    }
                                }
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
                        let lang = user.lang();
                        let tasks = Task::find_today_tasks(&pool, user.id).await.unwrap_or_default();

                        if tasks.is_empty() {
                            bot.send_message(msg.chat.id, i18n.t(lang, "tasks-today-empty"))
                                .await?;
                        } else {
                            let mut args = FluentArgs::new();
                            args.set("count", tasks.len() as i64);
                            bot.send_message(msg.chat.id, i18n.t_args(lang, "tasks-today-header", &args))
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
                                .reply_markup(task_keyboard(task.id, &i18n, lang))
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
                        let lang = user.lang();
                        let show_subscribe = !user.has_active_subscription()
                            || user.trial_days_remaining().is_some();

                        let mut keyboard_rows = vec![
                            vec![
                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-timezone"), "settings:timezone"),
                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-brief-time"), "settings:brief_time"),
                            ],
                            vec![
                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-language"), "settings:language"),
                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-invite"), "settings:invite"),
                            ],
                        ];

                        if show_subscribe {
                            keyboard_rows.push(vec![
                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-subscribe"), "subscribe"),
                            ]);
                        }

                        let settings_keyboard = InlineKeyboardMarkup::new(keyboard_rows);

                        let lang_display = match lang {
                            "ru" => "🇷🇺 Русский",
                            _ => "🇬🇧 English",
                        };

                        let mut args = FluentArgs::new();
                        args.set("status", user.subscription_status());
                        args.set("tz", user.timezone.clone());
                        args.set("time", user.morning_brief_time.clone());
                        args.set("lang", lang_display);

                        bot.send_message(msg.chat.id, i18n.t_args(lang, "settings-full", &args))
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .reply_markup(settings_keyboard)
                        .await?;
                    } else {
                        bot.send_message(msg.chat.id, i18n.t("en", "error-start-first"))
                            .await?;
                    }
                }
            }
            Command::Support => {
                // /support [message] — send message to support
                let support_text = text.strip_prefix("/support").map(|s| s.trim()).unwrap_or("");

                // Get user language for support messages
                let lang = if let Some(tg_user) = telegram_user {
                    User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await
                        .map(|u| u.lang().to_string())
                        .unwrap_or_else(|| "en".to_string())
                } else {
                    "en".to_string()
                };

                if support_text.is_empty() {
                    // Set pending state and ask for message
                    if let Some(tg_user) = telegram_user {
                        if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                            context.set_pending_edit(user.id, PendingEdit::Support);
                        }
                    }

                    let cancel_keyboard = InlineKeyboardMarkup::new(vec![vec![
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-cancel"), "cancel_support"),
                    ]]);

                    bot.send_message(
                        msg.chat.id,
                        i18n.t(&lang, "support-prompt"),
                    )
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .reply_markup(cancel_keyboard)
                    .await?;
                } else if let Some(support_chat) = get_support_chat_id() {
                    // Get user info for the support message
                    let user_info = if let Some(tg_user) = telegram_user {
                        let username = tg_user.username.as_ref()
                            .map(|u| format!(" @{}", u))
                            .unwrap_or_default();
                        format!("User #{}{} ({})", tg_user.id, username, tg_user.first_name)
                    } else {
                        format!("Chat #{}", msg.chat.id)
                    };

                    // Send to support chat (include chat_id for reply routing)
                    let support_msg = format!(
                        "📨 <b>[AI-Todolist Support]</b>\n\n\
                        👤 {}\n\
                        💬 {}\n\n\
                        <i>Reply to respond • chat:{}</i>",
                        user_info, support_text, msg.chat.id
                    );

                    match bot.send_message(support_chat, &support_msg)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await
                    {
                        Ok(sent_msg) => {
                            tracing::info!(
                                "Support message from chat {} forwarded as msg {}",
                                msg.chat.id, sent_msg.id
                            );

                            bot.send_message(
                                msg.chat.id,
                                i18n.t(&lang, "support-sent-success"),
                            )
                            .await?;
                        }
                        Err(e) => {
                            tracing::error!("Failed to forward support message: {}", e);
                            bot.send_message(
                                msg.chat.id,
                                i18n.t(&lang, "support-failed"),
                            )
                            .await?;
                        }
                    }
                } else {
                    tracing::warn!("SUPPORT_CHAT_ID not configured");
                    bot.send_message(
                        msg.chat.id,
                        i18n.t(&lang, "support-unavailable"),
                    )
                    .await?;
                }
            }
            Command::Invite => {
                if let Some(tg_user) = telegram_user {
                    if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                        // Generate or get referral code
                        match User::ensure_referral_code(&pool, user.id).await {
                            Ok(code) => {
                                let (referrals, bonus_days) = user.referral_stats();
                                let invite_link = format!("https://t.me/{}?start={}", BOT_USERNAME, code);

                                let share_text = format!(
                                    "🚀 Try AI Todolist - smart task manager with voice input!\n\n{}",
                                    invite_link
                                );

                                let share_keyboard = InlineKeyboardMarkup::new(vec![
                                    vec![
                                        InlineKeyboardButton::url(
                                            "📤 Share",
                                            format!("https://t.me/share/url?url={}&text=🚀%20Try%20AI%20Todolist!", invite_link).parse().unwrap(),
                                        ),
                                    ],
                                ]);

                                bot.send_message(
                                    msg.chat.id,
                                    format!(
"🎁 <b>Invite Friends</b>

Share your link and get <b>+7 days</b> for each friend who joins!

━━━━━━━━━━━━━━━━━━━━
🔗 <b>Your invite link:</b>
<code>{}</code>

━━━━━━━━━━━━━━━━━━━━
📊 <b>Your stats:</b>

👥 Friends invited: <b>{}</b>
🎁 Bonus days earned: <b>{}</b>

━━━━━━━━━━━━━━━━━━━━
💡 Tap the link to copy, or use the Share button!", invite_link, referrals, bonus_days),
                                )
                                .parse_mode(teloxide::types::ParseMode::Html)
                                .reply_markup(share_keyboard)
                                .await?;
                            }
                            Err(e) => {
                                tracing::error!("Failed to generate referral code: {}", e);
                                bot.send_message(msg.chat.id, "❌ Failed to generate invite link")
                                    .await?;
                            }
                        }
                    } else {
                        bot.send_message(msg.chat.id, "Please /start first")
                            .await?;
                    }
                }
            }
            Command::Admin => {
                if let Some(tg_user) = telegram_user {
                    if !is_admin(tg_user.id.0 as i64) {
                        return Ok(());
                    }

                    let admin_keyboard = InlineKeyboardMarkup::new(vec![
                        vec![
                            InlineKeyboardButton::callback("📊 Stats", "admin:stats"),
                            InlineKeyboardButton::callback("👥 Users", "admin:users"),
                        ],
                        vec![
                            InlineKeyboardButton::callback("📢 Broadcast", "admin:broadcast"),
                            InlineKeyboardButton::callback("🚫 Banned", "admin:banned"),
                        ],
                    ]);

                    bot.send_message(
                        msg.chat.id,
                        "🔐 <b>Admin Panel</b>\n\nSelect an option:",
                    )
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .reply_markup(admin_keyboard)
                    .await?;
                }
            }
        }
    } else if let Some(voice) = msg.voice() {
        // Voice message handling with progressive updates
        if let Some(tg_user) = telegram_user {
            if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                // Check subscription
                if let Some(expired_msg) = check_subscription(&user, &i18n) {
                    bot.send_message(msg.chat.id, expired_msg).await?;
                    return Ok(());
                }

                // Check voice rate limit
                let limits = get_rate_limits(&user);
                if let Err(limit_msg) = RateLimiter::check_and_increment(
                    &pool, user.id, "voice", limits.voice_per_day, 1440
                ).await {
                    bot.send_message(msg.chat.id, format!("⚠️ {}", limit_msg)).await?;
                    return Ok(());
                }

                if let Some(ai) = &ai_service {
                    // Check AI rate limit
                    if let Err(limit_msg) = RateLimiter::check_and_increment(
                        &pool, user.id, "ai_call", limits.ai_calls_per_hour, 60
                    ).await {
                        bot.send_message(msg.chat.id, format!("⚠️ {}", limit_msg)).await?;
                        return Ok(());
                    }

                    // Send initial processing message
                    let progress_msg = bot.send_message(msg.chat.id, i18n.t(user.lang(), "voice-processing"))
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

                                                    let lang = user.lang();
                                                    let confirm_keyboard = InlineKeyboardMarkup::new(vec![vec![
                                                        InlineKeyboardButton::callback(&i18n.t(lang, "btn-apply"), format!("confirm_edit:{}", task_id)),
                                                        InlineKeyboardButton::callback(&i18n.t(lang, "btn-cancel"), format!("cancel_edit:{}", task_id)),
                                                    ]]);

                                                    let mut args = FluentArgs::new();
                                                    args.set("text", transcript.clone());
                                                    args.set("old_title", task.title.clone());
                                                    args.set("new_title", parsed.title.clone());
                                                    args.set("due_change", due_change.clone());

                                                    bot.edit_message_text(
                                                        msg.chat.id,
                                                        progress_msg.id,
                                                        i18n.t_args(lang, "edit-preview-voice", &args),
                                                    )
                                                    .parse_mode(teloxide::types::ParseMode::Html)
                                                    .reply_markup(confirm_keyboard)
                                                    .await?;
                                                }
                                                Err(e) => {
                                                    tracing::warn!("Failed to parse voice edit: {}", e);
                                                    let mut args = FluentArgs::new();
                                                    args.set("text", transcript.clone());
                                                    let _ = bot.edit_message_text(
                                                        msg.chat.id,
                                                        progress_msg.id,
                                                        i18n.t_args(user.lang(), "voice-transcribed-error", &args),
                                                    ).await;
                                                }
                                            }
                                        }
                                        return Ok(());
                                    }
                                    PendingEdit::Reminder(task_id) => {
                                        // Parse reminder time from transcript
                                        let current_date = Utc::now().format("%Y-%m-%d %H:%M").to_string();
                                        let lang = user.lang();

                                        match ai.parse_reminder_time(&transcript, &current_date).await {
                                            Ok(reminder_at) => {
                                                let _ = Task::set_reminder(&pool, task_id, Some(&reminder_at)).await;

                                                if let Some(task) = Task::find_by_id(&pool, task_id).await {
                                                    let mut args = FluentArgs::new();
                                                    args.set("text", transcript.clone());
                                                    args.set("title", task.title.clone());
                                                    args.set("reminder", reminder_at.clone());

                                                    bot.edit_message_text(
                                                        msg.chat.id,
                                                        progress_msg.id,
                                                        i18n.t_args(lang, "voice-transcribed-reminder-set", &args),
                                                    )
                                                    .reply_markup(task_keyboard(task_id, &i18n, lang))
                                                    .await?;
                                                }
                                            }
                                            Err(e) => {
                                                tracing::warn!("Failed to parse reminder time: {}", e);
                                                let mut args = FluentArgs::new();
                                                args.set("text", transcript.clone());
                                                let _ = bot.edit_message_text(
                                                    msg.chat.id,
                                                    progress_msg.id,
                                                    i18n.t_args(lang, "voice-transcribed-reminder-failed", &args),
                                                ).await;
                                            }
                                        }
                                        return Ok(());
                                    }
                                    PendingEdit::Timezone => {
                                        // Parse timezone from transcript
                                        let lang = user.lang();
                                        match ai.parse_timezone(&transcript).await {
                                            Ok(timezone) => {
                                                let _ = User::update_timezone(&pool, user.id, &timezone).await;

                                                let mut args = FluentArgs::new();
                                                args.set("text", transcript.clone());
                                                args.set("timezone", timezone.clone());

                                                bot.edit_message_text(
                                                    msg.chat.id,
                                                    progress_msg.id,
                                                    i18n.t_args(lang, "voice-transcribed-timezone-set", &args),
                                                )
                                                .await?;
                                            }
                                            Err(e) => {
                                                tracing::warn!("Failed to parse timezone: {}", e);
                                                let mut args = FluentArgs::new();
                                                args.set("text", transcript.clone());
                                                let _ = bot.edit_message_text(
                                                    msg.chat.id,
                                                    progress_msg.id,
                                                    i18n.t_args(lang, "voice-transcribed-timezone-failed", &args),
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
                                            i18n.t(user.lang(), "error-use-buttons"),
                                        ).await;
                                        return Ok(());
                                    }
                                    PendingEdit::Support => {
                                        // Voice support message
                                        let lang = user.lang();
                                        if let Some(support_chat) = get_support_chat_id() {
                                            let user_info = if let Some(tg_user) = telegram_user {
                                                let username = tg_user.username.as_ref()
                                                    .map(|u| format!(" @{}", u))
                                                    .unwrap_or_default();
                                                format!("User #{}{} ({})", tg_user.id, username, tg_user.first_name)
                                            } else {
                                                format!("Chat #{}", msg.chat.id)
                                            };

                                            let support_msg = format!(
                                                "📨 <b>[AI-Todolist Support]</b>\n\n\
                                                👤 {}\n\
                                                🎤 {}\n\n\
                                                <i>Reply to respond • chat:{}</i>",
                                                user_info, transcript, msg.chat.id
                                            );

                                            match bot.send_message(support_chat, &support_msg)
                                                .parse_mode(teloxide::types::ParseMode::Html)
                                                .await
                                            {
                                                Ok(_) => {
                                                    let mut args = FluentArgs::new();
                                                    args.set("text", transcript.clone());
                                                    bot.edit_message_text(
                                                        msg.chat.id,
                                                        progress_msg.id,
                                                        i18n.t_args(lang, "voice-transcribed-support-sent", &args),
                                                    )
                                                    .await?;
                                                }
                                                Err(e) => {
                                                    tracing::error!("Failed to forward voice support: {}", e);
                                                    let _ = bot.edit_message_text(
                                                        msg.chat.id,
                                                        progress_msg.id,
                                                        i18n.t(lang, "voice-transcribed-support-failed"),
                                                    ).await;
                                                }
                                            }
                                        } else {
                                            let _ = bot.edit_message_text(
                                                msg.chat.id,
                                                progress_msg.id,
                                                i18n.t(lang, "support-unavailable"),
                                            ).await;
                                        }
                                        return Ok(());
                                    }
                                    // Admin pending edits - ignore voice for these
                                    PendingEdit::AdminSearch | PendingEdit::AdminBroadcast(_) | PendingEdit::AdminMessage(_) => {
                                        // Admin features use text only (keep in English for admins)
                                        let _ = bot.edit_message_text(
                                            msg.chat.id,
                                            progress_msg.id,
                                            "⚠️ Please send text message for admin actions.",
                                        ).await;
                                        return Ok(());
                                    }
                                    // Duplicate confirmation - ignore voice, use buttons
                                    PendingEdit::ConfirmDuplicate(_) => {
                                        let _ = bot.edit_message_text(
                                            msg.chat.id,
                                            progress_msg.id,
                                            i18n.t(user.lang(), "error-use-buttons"),
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
                                    // Check task creation rate limit
                                    if let Err(limit_msg) = RateLimiter::check_and_increment(
                                        &pool, user.id, "task", limits.tasks_per_day, 1440
                                    ).await {
                                        let _ = bot.edit_message_text(
                                            msg.chat.id,
                                            progress_msg.id,
                                            format!("⚠️ {}", limit_msg),
                                        ).await;
                                        return Ok(());
                                    }

                                    let tags_str = if parsed.tags.is_empty() { None } else { Some(parsed.tags.join(",")) };

                                    // Check for duplicate
                                    let lang = user.lang();
                                    if let Some(existing) = Task::find_similar(&pool, user.id, &parsed.title).await {
                                        // Store pending task and ask for confirmation
                                        context.set_pending_edit(user.id, PendingEdit::ConfirmDuplicate(PendingTask {
                                            title: parsed.title.clone(),
                                            due_at: parsed.due_at.clone(),
                                            tags: tags_str.clone(),
                                        }));

                                        let keyboard = InlineKeyboardMarkup::new(vec![
                                            vec![
                                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-create-anyway"), "dup:create"),
                                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-cancel"), "dup:cancel"),
                                            ]
                                        ]);

                                        let mut args = FluentArgs::new();
                                        args.set("title", existing.title.clone());

                                        let _ = bot.edit_message_text(
                                            msg.chat.id,
                                            progress_msg.id,
                                            i18n.t_args(lang, "duplicate-warning", &args),
                                        )
                                        .reply_markup(keyboard)
                                        .await;
                                        return Ok(());
                                    }

                                    match Task::create(&pool, user.id, &parsed.title, None, parsed.due_at.as_deref(), tags_str.as_deref()).await {
                                        Ok(task) => {
                                            if task.due_at.is_some() {
                                                let _ = Task::set_reminder_from_due(&pool, task.id).await;
                                            }

                                            let due_str = task.due_at.as_ref()
                                                .map(|d| format!("\n📅 {}", d))
                                                .unwrap_or_default();

                                            let mut args = FluentArgs::new();
                                            args.set("text", transcript.clone());
                                            args.set("title", task.title.clone());
                                            args.set("due", due_str.clone());

                                            bot.edit_message_text(
                                                msg.chat.id,
                                                progress_msg.id,
                                                i18n.t_args(lang, "voice-transcribed-added", &args),
                                            )
                                            .reply_markup(task_keyboard(task.id, &i18n, lang))
                                            .await?;
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to create task: {}", e);
                                            let _ = bot.edit_message_text(
                                                msg.chat.id,
                                                progress_msg.id,
                                                i18n.t(lang, "voice-task-create-failed"),
                                            ).await;
                                        }
                                    }
                                }
                                Ok(ParsedInput::Draft { recipient, context: _, draft }) => {
                                    let mut args = FluentArgs::new();
                                    args.set("text", transcript.clone());
                                    args.set("recipient", recipient.clone());
                                    args.set("draft", draft.clone());

                                    let _ = bot.edit_message_text(
                                        msg.chat.id,
                                        progress_msg.id,
                                        i18n.t_args(user.lang(), "voice-transcribed-draft", &args),
                                    ).await;
                                }
                                Ok(ParsedInput::Clarify { original: _, question, suggestions }) => {
                                    let lang = user.lang();
                                    // Build suggestion buttons
                                    let mut keyboard_rows: Vec<Vec<InlineKeyboardButton>> = suggestions.iter()
                                        .map(|s| vec![InlineKeyboardButton::callback(s.clone(), format!("clarify:{}", s))])
                                        .collect();
                                    keyboard_rows.push(vec![
                                        InlineKeyboardButton::callback(&i18n.t(lang, "btn-create-as-is"), format!("clarify:asis:{}", transcript))
                                    ]);

                                    let mut args = FluentArgs::new();
                                    args.set("text", transcript.clone());
                                    args.set("question", question.clone());

                                    let _ = bot.edit_message_text(
                                        msg.chat.id,
                                        progress_msg.id,
                                        i18n.t_args(lang, "voice-transcribed-clarify", &args),
                                    )
                                    .reply_markup(InlineKeyboardMarkup::new(keyboard_rows))
                                    .await;
                                }
                                Ok(ParsedInput::Command { action }) => {
                                    // Delete progress message and handle command
                                    let _ = bot.delete_message(msg.chat.id, progress_msg.id).await;
                                    let lang = user.lang();

                                    match action.as_str() {
                                        "show_tasks" => {
                                            let tasks = Task::find_pending_by_user(&pool, user.id).await.unwrap_or_default();
                                            if tasks.is_empty() {
                                                bot.send_message(msg.chat.id, i18n.t(lang, "ai-tasks-empty"))
                                                    .await?;
                                            } else {
                                                for task in tasks.iter().take(10) {
                                                    let due_str = task.due_at.as_ref()
                                                        .map(|d| format!("\n📅 {}", d))
                                                        .unwrap_or_default();

                                                    bot.send_message(
                                                        msg.chat.id,
                                                        format!("📝 {}{}", task.title, due_str),
                                                    )
                                                    .reply_markup(task_keyboard(task.id, &i18n, lang))
                                                    .await?;
                                                }
                                            }
                                        }
                                        "show_today" => {
                                            let tasks = Task::find_today_tasks(&pool, user.id).await.unwrap_or_default();
                                            if tasks.is_empty() {
                                                bot.send_message(msg.chat.id, i18n.t(lang, "ai-today-empty"))
                                                    .await?;
                                            } else {
                                                for task in tasks.iter() {
                                                    let due_str = task.due_at.as_ref()
                                                        .map(|d| format!("\n📅 {}", d))
                                                        .unwrap_or_default();

                                                    bot.send_message(
                                                        msg.chat.id,
                                                        format!("📝 {}{}", task.title, due_str),
                                                    )
                                                    .reply_markup(task_keyboard(task.id, &i18n, lang))
                                                    .await?;
                                                }
                                            }
                                        }
                                        "settings" => {
                                            let settings_keyboard = InlineKeyboardMarkup::new(vec![
                                                vec![
                                                    InlineKeyboardButton::callback(&i18n.t(lang, "btn-timezone"), "settings:timezone"),
                                                    InlineKeyboardButton::callback(&i18n.t(lang, "btn-brief-time"), "settings:brief"),
                                                ],
                                            ]);
                                            bot.send_message(msg.chat.id, i18n.t(lang, "settings-title"))
                                                .reply_markup(settings_keyboard)
                                                .await?;
                                        }
                                        "help" => {
                                            bot.send_message(msg.chat.id, i18n.t(lang, "help-text"))
                                                .parse_mode(teloxide::types::ParseMode::Html)
                                                .await?;
                                        }
                                        _ => {
                                            bot.send_message(msg.chat.id, i18n.t(lang, "ai-unknown-command"))
                                                .await?;
                                        }
                                    }
                                }
                                Ok(ParsedInput::Rejected { reason }) => {
                                    let mut args = FluentArgs::new();
                                    args.set("text", transcript.clone());
                                    args.set("reason", reason.clone());

                                    let _ = bot.edit_message_text(
                                        msg.chat.id,
                                        progress_msg.id,
                                        i18n.t_args(user.lang(), "voice-transcribed-unknown", &args),
                                    ).await;
                                }
                                Err(e) => {
                                    tracing::warn!("AI parse failed: {}, using transcript as task", e);
                                    let lang = user.lang();
                                    // Check task creation rate limit
                                    if let Err(limit_msg) = RateLimiter::check_and_increment(
                                        &pool, user.id, "task", limits.tasks_per_day, 1440
                                    ).await {
                                        let _ = bot.edit_message_text(
                                            msg.chat.id,
                                            progress_msg.id,
                                            format!("⚠️ {}", limit_msg),
                                        ).await;
                                        return Ok(());
                                    }

                                    match Task::create(&pool, user.id, &transcript, None, None, None).await {
                                        Ok(task) => {
                                            let mut args = FluentArgs::new();
                                            args.set("text", transcript.clone());
                                            args.set("title", task.title.clone());
                                            args.set("due", "".to_string());

                                            bot.edit_message_text(
                                                msg.chat.id,
                                                progress_msg.id,
                                                i18n.t_args(lang, "voice-transcribed-added", &args),
                                            )
                                            .reply_markup(task_keyboard(task.id, &i18n, lang))
                                            .await?;
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to create task: {}", e);
                                            let _ = bot.edit_message_text(
                                                msg.chat.id,
                                                progress_msg.id,
                                                i18n.t(lang, "voice-task-create-failed"),
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
                                i18n.t(user.lang(), "error-voice-failed"),
                            ).await;
                        }
                    }
                } else {
                    bot.send_message(msg.chat.id, i18n.t(user.lang(), "error-voice-requires-ai"))
                        .await?;
                }
            } else {
                bot.send_message(msg.chat.id, i18n.t("en", "error-start-first"))
                    .await?;
            }
        }
    } else if let Some(location) = msg.location() {
        // Handle location for timezone detection
        if let Some(tg_user) = telegram_user {
            if let Some(user) = User::find_by_telegram_id(&pool, tg_user.id.0 as i64).await {
                let timezone = timezone_from_coords(location.latitude, location.longitude);
                let _ = User::update_timezone(&pool, user.id, &timezone).await;

                let mut args = FluentArgs::new();
                args.set("timezone", timezone.clone());

                // Remove keyboard
                bot.send_message(
                    msg.chat.id,
                    i18n.t_args(user.lang(), "timezone-set-success", &args),
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
                                            let lang = user.lang();
                                            let proposed = ProposedEdit {
                                                task_id,
                                                old_title: task.title.clone(),
                                                new_title: parsed.title.clone(),
                                                new_due_at: parsed.due_at.clone(),
                                            };

                                            // Store proposed edit for confirmation
                                            context.set_pending_edit(user.id, PendingEdit::ConfirmEdit(proposed.clone()));

                                            let due_change = match (&task.due_at, &parsed.due_at) {
                                                (Some(old), Some(new)) if old != new => {
                                                    let mut args = FluentArgs::new();
                                                    args.set("old", old.clone());
                                                    args.set("new", new.clone());
                                                    format!("\n{}", i18n.t_args(lang, "due-change-update", &args))
                                                }
                                                (None, Some(new)) => {
                                                    let mut args = FluentArgs::new();
                                                    args.set("new", new.clone());
                                                    format!("\n{}", i18n.t_args(lang, "due-change-new", &args))
                                                }
                                                (Some(old), None) => {
                                                    let mut args = FluentArgs::new();
                                                    args.set("old", old.clone());
                                                    format!("\n{}", i18n.t_args(lang, "due-change-remove", &args))
                                                }
                                                _ => String::new(),
                                            };

                                            let confirm_keyboard = InlineKeyboardMarkup::new(vec![vec![
                                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-apply"), format!("confirm_edit:{}", task_id)),
                                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-cancel"), format!("cancel_edit:{}", task_id)),
                                            ]]);

                                            let mut args = FluentArgs::new();
                                            args.set("old_title", task.title.clone());
                                            args.set("new_title", parsed.title.clone());
                                            args.set("due_change", due_change.clone());

                                            bot.send_message(
                                                msg.chat.id,
                                                i18n.t_args(lang, "edit-preview-text", &args),
                                            )
                                            .parse_mode(teloxide::types::ParseMode::Html)
                                            .reply_markup(confirm_keyboard)
                                            .await?;
                                        }
                                        Err(e) => {
                                            tracing::warn!("Failed to parse edit: {}", e);
                                            bot.send_message(
                                                msg.chat.id,
                                                i18n.t(user.lang(), "error-edit-failed"),
                                            )
                                            .await?;
                                        }
                                    }
                                } else {
                                    // No AI - just replace title
                                    let _ = Task::update(&pool, task_id, Some(text), None).await;
                                    let mut args = FluentArgs::new();
                                    args.set("title", text.to_string());
                                    bot.send_message(msg.chat.id, i18n.t_args(user.lang(), "task-updated-simple", &args)).await?;
                                }
                            } else {
                                bot.send_message(msg.chat.id, i18n.t(user.lang(), "error-not-found")).await?;
                            }
                            return Ok(());
                        }
                        PendingEdit::ConfirmEdit(_) => {
                            // User sent text instead of clicking button - cancel
                            bot.send_message(
                                msg.chat.id,
                                i18n.t(user.lang(), "error-use-buttons"),
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
                                            let lang = user.lang();
                                            let mut args = FluentArgs::new();
                                            args.set("title", task.title.clone());
                                            args.set("reminder", reminder_at.clone());

                                            bot.send_message(
                                                msg.chat.id,
                                                i18n.t_args(lang, "reminder-set-confirm", &args),
                                            )
                                            .reply_markup(task_keyboard(task_id, &i18n, lang))
                                            .await?;
                                        }
                                    }
                                    Err(e) => {
                                        tracing::warn!("Failed to parse reminder time: {}", e);
                                        bot.send_message(
                                            msg.chat.id,
                                            i18n.t(user.lang(), "error-reminder-time-failed"),
                                        )
                                        .await?;
                                    }
                                }
                            } else {
                                bot.send_message(msg.chat.id, i18n.t(user.lang(), "error-ai-required"))
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

                                        let mut args = FluentArgs::new();
                                        args.set("timezone", timezone.clone());

                                        bot.send_message(
                                            msg.chat.id,
                                            i18n.t_args(user.lang(), "timezone-set-success", &args),
                                        )
                                        .reply_markup(KeyboardRemove::new())
                                        .await?;
                                    }
                                    Err(e) => {
                                        tracing::warn!("Failed to parse timezone: {}", e);
                                        bot.send_message(
                                            msg.chat.id,
                                            i18n.t(user.lang(), "error-timezone-failed"),
                                        )
                                        .await?;
                                    }
                                }
                            } else {
                                bot.send_message(msg.chat.id, i18n.t(user.lang(), "error-ai-required"))
                                    .await?;
                            }
                            return Ok(());
                        }
                        PendingEdit::Support => {
                            // Send support message
                            let lang = user.lang();
                            if let Some(support_chat) = get_support_chat_id() {
                                let user_info = if let Some(tg_user) = telegram_user {
                                    let username = tg_user.username.as_ref()
                                        .map(|u| format!(" @{}", u))
                                        .unwrap_or_default();
                                    format!("User #{}{} ({})", tg_user.id, username, tg_user.first_name)
                                } else {
                                    format!("Chat #{}", msg.chat.id)
                                };

                                let support_msg = format!(
                                    "📨 <b>[AI-Todolist Support]</b>\n\n\
                                    👤 {}\n\
                                    💬 {}\n\n\
                                    <i>Reply to respond • chat:{}</i>",
                                    user_info, text, msg.chat.id
                                );

                                match bot.send_message(support_chat, &support_msg)
                                    .parse_mode(teloxide::types::ParseMode::Html)
                                    .await
                                {
                                    Ok(_) => {
                                        bot.send_message(
                                            msg.chat.id,
                                            i18n.t(lang, "support-sent-success"),
                                        )
                                        .await?;
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to forward support message: {}", e);
                                        bot.send_message(
                                            msg.chat.id,
                                            i18n.t(lang, "support-failed"),
                                        )
                                        .await?;
                                    }
                                }
                            } else {
                                bot.send_message(
                                    msg.chat.id,
                                    i18n.t(lang, "support-unavailable"),
                                )
                                .await?;
                            }
                            return Ok(());
                        }
                        PendingEdit::AdminSearch => {
                            // Admin user search
                            let query = text.trim().trim_start_matches('@');
                            if let Ok(users) = User::search(&pool, query).await {
                                if users.is_empty() {
                                    bot.send_message(msg.chat.id, "🔍 No users found.")
                                        .await?;
                                } else {
                                    let mut keyboard_rows = Vec::new();
                                    for u in users.iter().take(10) {
                                        let status = if u.is_banned() { "🚫" }
                                            else if u.subscription_days_remaining().is_some() { "✅" }
                                            else if u.trial_days_remaining().is_some() { "🎁" }
                                            else { "❌" };
                                        keyboard_rows.push(vec![
                                            InlineKeyboardButton::callback(
                                                format!("{} {}", status, u.display_name()),
                                                format!("admin:user:{}", u.id)
                                            )
                                        ]);
                                    }
                                    keyboard_rows.push(vec![
                                        InlineKeyboardButton::callback("↩️ Back", "admin:users")
                                    ]);

                                    let results_keyboard = InlineKeyboardMarkup::new(keyboard_rows);
                                    bot.send_message(
                                        msg.chat.id,
                                        format!("🔍 Found {} user(s):", users.len()),
                                    )
                                    .reply_markup(results_keyboard)
                                    .await?;
                                }
                            }
                            return Ok(());
                        }
                        PendingEdit::AdminBroadcast(segment) => {
                            // Send broadcast
                            if let Ok(users) = User::list_by_segment(&pool, &segment).await {
                                let total = users.len();
                                let mut sent = 0;
                                let mut failed = 0;

                                bot.send_message(
                                    msg.chat.id,
                                    format!("📢 Sending to {} users...", total),
                                ).await?;

                                for target_user in users {
                                    match bot.send_message(
                                        ChatId(target_user.telegram_id),
                                        text,
                                    ).await {
                                        Ok(_) => sent += 1,
                                        Err(_) => failed += 1,
                                    }
                                    // Small delay to avoid rate limits
                                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                                }

                                bot.send_message(
                                    msg.chat.id,
                                    format!("✅ Broadcast complete!\n\n📨 Sent: {}\n❌ Failed: {}", sent, failed),
                                ).await?;
                            }
                            return Ok(());
                        }
                        PendingEdit::AdminMessage(target_user_id) => {
                            // Send message to specific user
                            if let Some(target_user) = User::find_by_id(&pool, target_user_id).await {
                                match bot.send_message(
                                    ChatId(target_user.telegram_id),
                                    format!("📨 <b>Message from Admin:</b>\n\n{}", text),
                                )
                                .parse_mode(teloxide::types::ParseMode::Html)
                                .await {
                                    Ok(_) => {
                                        bot.send_message(
                                            msg.chat.id,
                                            format!("✅ Message sent to {}", target_user.display_name()),
                                        ).await?;
                                    }
                                    Err(e) => {
                                        bot.send_message(
                                            msg.chat.id,
                                            format!("❌ Failed to send: {}", e),
                                        ).await?;
                                    }
                                }
                            }
                            return Ok(());
                        }
                        PendingEdit::ConfirmDuplicate(_) => {
                            // Waiting for button press, ignore text
                            bot.send_message(
                                msg.chat.id,
                                "⚠️ Please use the buttons above to confirm or cancel.",
                            ).await?;
                            return Ok(());
                        }
                    }
                }

                // Natural language input
                if let Some(ai) = &ai_service {
                    // Check subscription
                    if let Some(expired_msg) = check_subscription(&user, &i18n) {
                        bot.send_message(msg.chat.id, expired_msg).await?;
                        return Ok(());
                    }

                    // Check rate limit
                    let limits = get_rate_limits(&user);
                    if let Err(limit_msg) = RateLimiter::check_and_increment(
                        &pool, user.id, "ai_call", limits.ai_calls_per_hour, 60
                    ).await {
                        bot.send_message(msg.chat.id, format!("⚠️ {}", limit_msg)).await?;
                        return Ok(());
                    }

                    // Show typing indicator while AI processes
                    let _ = bot.send_chat_action(msg.chat.id, ChatAction::Typing).await;

                    let current_date = Utc::now().format("%Y-%m-%d %H:%M").to_string();
                    let user_context = context.get_context(user.id);

                    // Add user message to context
                    context.add_message(user.id, "user", text);

                    match ai.parse_input(text, &current_date, user_context.as_deref()).await {
                        Ok(ParsedInput::Task(parsed)) => {
                            tracing::info!("AI parsed task: {:?}", parsed);
                            // Check task creation rate limit
                            if let Err(limit_msg) = RateLimiter::check_and_increment(
                                &pool, user.id, "task", limits.tasks_per_day, 1440
                            ).await {
                                bot.send_message(msg.chat.id, format!("⚠️ {}", limit_msg)).await?;
                                return Ok(());
                            }

                            let tags_str = if parsed.tags.is_empty() { None } else { Some(parsed.tags.join(",")) };

                            let lang = user.lang();

                            // Check for duplicate
                            if let Some(existing) = Task::find_similar(&pool, user.id, &parsed.title).await {
                                context.set_pending_edit(user.id, PendingEdit::ConfirmDuplicate(PendingTask {
                                    title: parsed.title.clone(),
                                    due_at: parsed.due_at.clone(),
                                    tags: tags_str.clone(),
                                }));

                                let keyboard = InlineKeyboardMarkup::new(vec![
                                    vec![
                                        InlineKeyboardButton::callback(&i18n.t(lang, "btn-create-anyway"), "dup:create"),
                                        InlineKeyboardButton::callback(&i18n.t(lang, "btn-cancel"), "dup:cancel"),
                                    ]
                                ]);

                                let mut args = FluentArgs::new();
                                args.set("title", existing.title.clone());
                                bot.send_message(
                                    msg.chat.id,
                                    i18n.t_args(lang, "duplicate-warning", &args),
                                )
                                .reply_markup(keyboard)
                                .await?;
                                return Ok(());
                            }

                            match Task::create(&pool, user.id, &parsed.title, None, parsed.due_at.as_deref(), tags_str.as_deref()).await {
                                Ok(task) => {
                                    if task.due_at.is_some() {
                                        let _ = Task::set_reminder_from_due(&pool, task.id).await;
                                    }

                                    let due_str = task.due_at.as_ref()
                                        .map(|d| format!("\n📅 {}", d))
                                        .unwrap_or_default();

                                    let response = format!("Added task: {}{}", task.title, due_str);
                                    context.add_message(user.id, "assistant", &response);

                                    let mut args = FluentArgs::new();
                                    args.set("title", task.title.clone());
                                    args.set("due", due_str);
                                    bot.send_message(
                                        msg.chat.id,
                                        i18n.t_args(lang, "task-added", &args),
                                    )
                                    .reply_markup(task_keyboard(task.id, &i18n, lang))
                                    .await?;
                                }
                                Err(e) => {
                                    tracing::error!("Failed to create task: {}", e);
                                    bot.send_message(msg.chat.id, i18n.t(lang, "error-task-create"))
                                        .await?;
                                }
                            }
                        }
                        Ok(ParsedInput::Draft { recipient, context: ctx, draft }) => {
                            tracing::info!("AI generated draft for: {}", recipient);
                            let response = format!("Draft for {}: {}", recipient, ctx);
                            context.add_message(user.id, "assistant", &response);

                            let lang = user.lang();
                            let mut args = FluentArgs::new();
                            args.set("recipient", recipient.clone());
                            args.set("text", draft.clone());
                            bot.send_message(
                                msg.chat.id,
                                i18n.t_args(lang, "draft-message", &args),
                            )
                            .await?;
                        }
                        Ok(ParsedInput::Clarify { original, question, suggestions }) => {
                            tracing::info!("AI needs clarification for: {}", original);
                            let lang = user.lang();
                            // Build suggestion buttons
                            let mut keyboard_rows: Vec<Vec<InlineKeyboardButton>> = suggestions.iter()
                                .map(|s| vec![InlineKeyboardButton::callback(s.clone(), format!("clarify:{}", s))])
                                .collect();
                            keyboard_rows.push(vec![
                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-create-as-is"), format!("clarify:asis:{}", original))
                            ]);

                            let mut args = FluentArgs::new();
                            args.set("question", question.clone());
                            bot.send_message(
                                msg.chat.id,
                                i18n.t_args(lang, "clarify-prompt", &args),
                            )
                            .reply_markup(InlineKeyboardMarkup::new(keyboard_rows))
                            .await?;
                        }
                        Ok(ParsedInput::Command { action }) => {
                            tracing::info!("AI command: {}", action);
                            let lang = user.lang();
                            match action.as_str() {
                                "show_tasks" => {
                                    let tasks = Task::find_pending_by_user(&pool, user.id).await.unwrap_or_default();
                                    if tasks.is_empty() {
                                        bot.send_message(msg.chat.id, i18n.t(lang, "ai-tasks-empty"))
                                            .await?;
                                    } else {
                                        for task in tasks.iter().take(10) {
                                            let due_str = task.due_at.as_ref()
                                                .map(|d| format!("\n📅 {}", d))
                                                .unwrap_or_default();
                                            let reminder_str = task.reminder_at.as_ref()
                                                .map(|r| format!("\n🔔 {}", r))
                                                .unwrap_or_default();

                                            bot.send_message(
                                                msg.chat.id,
                                                format!("📝 {}{}{}", task.title, due_str, reminder_str),
                                            )
                                            .reply_markup(task_keyboard(task.id, &i18n, lang))
                                            .await?;
                                        }
                                        if tasks.len() > 10 {
                                            let mut args = FluentArgs::new();
                                            args.set("count", (tasks.len() - 10) as i64);
                                            bot.send_message(msg.chat.id, i18n.t_args(lang, "ai-tasks-more", &args))
                                                .await?;
                                        }
                                    }
                                }
                                "show_today" => {
                                    let tasks = Task::find_today_tasks(&pool, user.id).await.unwrap_or_default();
                                    if tasks.is_empty() {
                                        bot.send_message(msg.chat.id, i18n.t(lang, "ai-today-empty"))
                                            .await?;
                                    } else {
                                        let mut args = FluentArgs::new();
                                        args.set("count", tasks.len() as i64);
                                        bot.send_message(msg.chat.id, i18n.t_args(lang, "ai-today-header", &args))
                                            .await?;
                                        for task in tasks.iter() {
                                            let due_str = task.due_at.as_ref()
                                                .map(|d| format!("\n📅 {}", d))
                                                .unwrap_or_default();

                                            bot.send_message(
                                                msg.chat.id,
                                                format!("📝 {}{}", task.title, due_str),
                                            )
                                            .reply_markup(task_keyboard(task.id, &i18n, lang))
                                            .await?;
                                        }
                                    }
                                }
                                "settings" => {
                                    let settings_keyboard = InlineKeyboardMarkup::new(vec![
                                        vec![
                                            InlineKeyboardButton::callback(&i18n.t(lang, "btn-timezone"), "settings:timezone"),
                                            InlineKeyboardButton::callback(&i18n.t(lang, "btn-brief-time"), "settings:brief_time"),
                                        ],
                                    ]);
                                    bot.send_message(msg.chat.id, i18n.t(lang, "settings-title"))
                                        .reply_markup(settings_keyboard)
                                        .parse_mode(teloxide::types::ParseMode::Html)
                                        .await?;
                                }
                                "help" => {
                                    bot.send_message(
                                        msg.chat.id,
                                        i18n.t(lang, "help-text"),
                                    )
                                    .parse_mode(teloxide::types::ParseMode::Html)
                                    .await?;
                                }
                                _ => {
                                    bot.send_message(msg.chat.id, i18n.t(lang, "ai-unknown-command")).await?;
                                }
                            }
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
                            let lang = user.lang();
                            // Check task creation rate limit
                            if let Err(limit_msg) = RateLimiter::check_and_increment(
                                &pool, user.id, "task", limits.tasks_per_day, 1440
                            ).await {
                                bot.send_message(msg.chat.id, format!("⚠️ {}", limit_msg)).await?;
                                return Ok(());
                            }

                            match Task::create(&pool, user.id, text, None, None, None).await {
                                Ok(task) => {
                                    let mut args = FluentArgs::new();
                                    args.set("title", task.title.clone());
                                    args.set("due", "");
                                    bot.send_message(
                                        msg.chat.id,
                                        i18n.t_args(lang, "task-added", &args),
                                    )
                                    .reply_markup(task_keyboard(task.id, &i18n, lang))
                                    .await?;
                                }
                                Err(e) => {
                                    tracing::error!("Failed to create task: {}", e);
                                    bot.send_message(msg.chat.id, i18n.t(lang, "error-task-create"))
                                        .await?;
                                }
                            }
                        }
                    }
                } else {
                    // No AI service, create raw task
                    // Check subscription
                    if let Some(expired_msg) = check_subscription(&user, &i18n) {
                        bot.send_message(msg.chat.id, expired_msg).await?;
                        return Ok(());
                    }

                    // Check task creation rate limit
                    let limits = get_rate_limits(&user);
                    if let Err(limit_msg) = RateLimiter::check_and_increment(
                        &pool, user.id, "task", limits.tasks_per_day, 1440
                    ).await {
                        bot.send_message(msg.chat.id, format!("⚠️ {}", limit_msg)).await?;
                        return Ok(());
                    }

                    let lang = user.lang();
                    match Task::create(&pool, user.id, text, None, None, None).await {
                        Ok(task) => {
                            let mut args = FluentArgs::new();
                            args.set("title", task.title.clone());
                            args.set("due", "");
                            bot.send_message(
                                msg.chat.id,
                                i18n.t_args(lang, "task-added", &args),
                            )
                            .reply_markup(task_keyboard(task.id, &i18n, lang))
                            .await?;
                        }
                        Err(e) => {
                            tracing::error!("Failed to create task: {}", e);
                            bot.send_message(msg.chat.id, i18n.t(lang, "error-task-create"))
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
    i18n: Arc<I18n>,
) -> ResponseResult<()> {
    let data = q.data.unwrap_or_default();

    if let Some(task_id_str) = data.strip_prefix("done:") {
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                let _ = Task::update_status(&pool, task_id, TaskStatus::Done).await;

                let telegram_id = q.from.id.0 as i64;
                if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
                    let lang = user.lang();
                    // Get stats
                    let completed_today = Task::count_completed_today(&pool, user.id).await;
                    let pending = Task::find_pending_by_user(&pool, user.id).await.unwrap_or_default();
                    let total_today = completed_today + pending.len() as i64;

                    // Celebration message
                    let celebration = i18n.t(lang, match completed_today {
                        1 => "celebrate-first",
                        2..=4 => "celebrate-keep-going",
                        5..=9 => "celebrate-on-fire",
                        _ => "celebrate-unstoppable",
                    });

                    // Update message with celebration
                    if let Some(msg) = &q.message {
                        let mut args = FluentArgs::new();
                        args.set("title", task.title.clone());
                        args.set("done", completed_today);
                        args.set("total", total_today);
                        args.set("celebration", celebration);

                        bot.edit_message_text(
                            msg.chat().id,
                            msg.id(),
                            i18n.t_args(lang, "task-completed-stats", &args),
                        )
                        .await?;

                        // Show next task
                        if let Some(next_task) = pending.first() {
                            let due_str = next_task.due_at.as_ref()
                                .map(|d| format!(" 📅 {}", d))
                                .unwrap_or_default();

                            let next_keyboard = InlineKeyboardMarkup::new(vec![vec![
                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-do-it"), format!("done:{}", next_task.id)),
                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-all-tasks"), "view_tasks".to_string()),
                            ]]);

                            let mut args = FluentArgs::new();
                            args.set("title", next_task.title.clone());
                            args.set("due", due_str);
                            bot.send_message(msg.chat().id, i18n.t_args(lang, "task-next", &args))
                            .reply_markup(next_keyboard)
                            .await?;
                        } else {
                            let mut args = FluentArgs::new();
                            args.set("count", completed_today);
                            bot.send_message(msg.chat().id, i18n.t_args(lang, "task-all-done", &args))
                            .await?;
                        }
                    }
                }

                bot.answer_callback_query(q.id).text("✅").await?;
            }
        }
    } else if let Some(task_id_str) = data.strip_prefix("delete:") {
        // Show confirmation dialog instead of deleting immediately
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                let telegram_id = q.from.id.0 as i64;
                let lang = User::find_by_telegram_id(&pool, telegram_id).await
                    .map(|u| u.lang().to_string())
                    .unwrap_or_else(|| "en".to_string());

                if let Some(msg) = q.message {
                    let confirm_keyboard = InlineKeyboardMarkup::new(vec![vec![
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-yes-delete"), format!("confirm_delete:{}", task_id)),
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-cancel"), format!("cancel_delete:{}", task_id)),
                    ]]);

                    let mut args = FluentArgs::new();
                    args.set("title", task.title.clone());
                    bot.edit_message_text(msg.chat().id, msg.id(), i18n.t_args(&lang, "task-delete-confirm", &args))
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

                let telegram_id = q.from.id.0 as i64;
                let lang = User::find_by_telegram_id(&pool, telegram_id).await
                    .map(|u| u.lang().to_string())
                    .unwrap_or_else(|| "en".to_string());

                if let Some(msg) = q.message {
                    let mut args = FluentArgs::new();
                    args.set("title", task.title.clone());
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        i18n.t_args(&lang, "task-deleted-msg", &args),
                    )
                    .await?;
                }

                bot.answer_callback_query(q.id).text("🗑").await?;
            }
        }
    } else if let Some(task_id_str) = data.strip_prefix("cancel_delete:") {
        // Restore original task view
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                let telegram_id = q.from.id.0 as i64;
                let lang = User::find_by_telegram_id(&pool, telegram_id).await
                    .map(|u| u.lang().to_string())
                    .unwrap_or_else(|| "en".to_string());

                if let Some(msg) = q.message {
                    let due_str = task.due_at.as_ref()
                        .map(|d| format!("\n📅 {}", d))
                        .unwrap_or_default();

                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        format!("📝 {}{}", task.title, due_str),
                    )
                    .reply_markup(task_keyboard(task_id, &i18n, &lang))
                    .await?;
                }

                bot.answer_callback_query(q.id).text(&i18n.t(&lang, "task-cancelled")).await?;
            }
        }
    } else if let Some(snooze_data) = data.strip_prefix("snooze:") {
        // Format: snooze:task_id:minutes
        let parts: Vec<&str> = snooze_data.split(':').collect();
        if parts.len() == 2 {
            if let (Ok(task_id), Ok(minutes)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                if let Some(task) = Task::find_by_id(&pool, task_id).await {
                    let _ = Task::snooze_reminder(&pool, task_id, minutes).await;

                    let telegram_id = q.from.id.0 as i64;
                    let lang = User::find_by_telegram_id(&pool, telegram_id).await
                        .map(|u| u.lang().to_string())
                        .unwrap_or_else(|| "en".to_string());

                    let snooze_key = if minutes == 60 {
                        "snooze-1h"
                    } else if minutes == 1440 {
                        "snooze-tomorrow"
                    } else {
                        "snooze-later"
                    };
                    let snooze_text = i18n.t(&lang, snooze_key);

                    if let Some(msg) = q.message {
                        let mut args = FluentArgs::new();
                        args.set("title", task.title.clone());
                        args.set("when", snooze_text.clone());
                        bot.edit_message_text(
                            msg.chat().id,
                            msg.id(),
                            i18n.t_args(&lang, "task-snoozed", &args),
                        )
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?;
                    }

                    bot.answer_callback_query(q.id).text(format!("⏰ {}", snooze_text)).await?;
                }
            }
        }
    } else if data == "view_tasks" {
        // Show all pending tasks
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let lang = user.lang();
            let tasks = Task::find_pending_by_user(&pool, user.id).await.unwrap_or_default();

            if let Some(msg) = &q.message {
                if tasks.is_empty() {
                    bot.send_message(msg.chat().id, i18n.t(lang, "tasks-empty"))
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
                        .reply_markup(task_keyboard(task.id, &i18n, lang))
                        .await?;
                    }
                }
            }
        }

        bot.answer_callback_query(q.id).await?;
    } else if data == "dup:create" {
        // User confirmed creating duplicate task
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let lang = user.lang().to_string();
            if let Some(PendingEdit::ConfirmDuplicate(pending)) = context.take_pending_edit(user.id) {
                match Task::create(&pool, user.id, &pending.title, None, pending.due_at.as_deref(), pending.tags.as_deref()).await {
                    Ok(task) => {
                        if task.due_at.is_some() {
                            let _ = Task::set_reminder_from_due(&pool, task.id).await;
                        }

                        let due_str = task.due_at.as_ref()
                            .map(|d| format!("\n📅 {}", d))
                            .unwrap_or_default();

                        if let Some(msg) = q.message {
                            let mut args = FluentArgs::new();
                            args.set("title", task.title.clone());
                            args.set("due", due_str);
                            bot.edit_message_text(
                                msg.chat().id,
                                msg.id(),
                                i18n.t_args(&lang, "task-added", &args),
                            )
                            .reply_markup(task_keyboard(task.id, &i18n, &lang))
                            .await?;
                        }

                        bot.answer_callback_query(q.id).text("✅").await?;
                    }
                    Err(e) => {
                        tracing::error!("Failed to create task: {}", e);
                        if let Some(msg) = q.message {
                            bot.edit_message_text(
                                msg.chat().id,
                                msg.id(),
                                i18n.t(&lang, "task-create-failed"),
                            ).await?;
                        }
                        bot.answer_callback_query(q.id).text("❌").await?;
                    }
                }
            } else {
                bot.answer_callback_query(q.id).text(&i18n.t(&lang, "session-expired")).await?;
            }
        }
    } else if data == "dup:cancel" {
        // User cancelled duplicate task creation
        let telegram_id = q.from.id.0 as i64;
        let lang = User::find_by_telegram_id(&pool, telegram_id).await
            .map(|u| u.lang().to_string())
            .unwrap_or_else(|| "en".to_string());

        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            context.take_pending_edit(user.id); // Clear pending
        }

        if let Some(msg) = q.message {
            bot.edit_message_text(
                msg.chat().id,
                msg.id(),
                format!("↩️ {}", i18n.t(&lang, "task-cancelled")),
            ).await?;
        }

        bot.answer_callback_query(q.id).text(&i18n.t(&lang, "task-cancelled")).await?;
    } else if data == "stale:review" {
        // Show stale tasks one by one for review
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let lang = user.lang();
            let stale_tasks = Task::find_stale(&pool, user.id, 7).await.unwrap_or_default();

            if let Some(msg) = &q.message {
                if stale_tasks.is_empty() {
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        i18n.t(lang, "stale-no-tasks"),
                    ).await?;
                } else {
                    let mut args = FluentArgs::new();
                    args.set("count", stale_tasks.len() as i64);
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        i18n.t_args(lang, "stale-reviewing", &args),
                    ).await?;

                    // Show each stale task with actions
                    for task in stale_tasks {
                        let keyboard = InlineKeyboardMarkup::new(vec![
                            vec![
                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-keep"), format!("stale:touch:{}", task.id)),
                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-done"), format!("done:{}", task.id)),
                                InlineKeyboardButton::callback(&i18n.t(lang, "btn-delete"), format!("delete:{}", task.id)),
                            ]
                        ]);

                        let mut args = FluentArgs::new();
                        args.set("title", task.title.clone());
                        args.set("updated", task.updated_at.clone());
                        bot.send_message(
                            msg.chat().id,
                            i18n.t_args(lang, "stale-task-item", &args),
                        )
                        .reply_markup(keyboard)
                        .await?;
                    }
                }
            }
        }

        bot.answer_callback_query(q.id).await?;
    } else if data == "stale:keep" {
        // Touch all stale tasks to mark them as reviewed
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let lang = user.lang();
            let stale_tasks = Task::find_stale(&pool, user.id, 7).await.unwrap_or_default();
            let count = stale_tasks.len();

            for task in stale_tasks {
                let _ = Task::touch(&pool, task.id).await;
            }

            if let Some(msg) = q.message {
                let mut args = FluentArgs::new();
                args.set("count", count as i64);
                bot.edit_message_text(
                    msg.chat().id,
                    msg.id(),
                    i18n.t_args(lang, "stale-kept-all", &args),
                ).await?;
            }
        }

        bot.answer_callback_query(q.id).text("✅").await?;
    } else if let Some(task_id_str) = data.strip_prefix("stale:touch:") {
        // Touch single stale task
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                let _ = Task::touch(&pool, task_id).await;

                let telegram_id = q.from.id.0 as i64;
                let lang = User::find_by_telegram_id(&pool, telegram_id).await
                    .map(|u| u.lang().to_string())
                    .unwrap_or_else(|| "en".to_string());

                if let Some(msg) = q.message {
                    let mut args = FluentArgs::new();
                    args.set("title", task.title.clone());
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        i18n.t_args(&lang, "stale-kept-one", &args),
                    ).await?;
                }

                bot.answer_callback_query(q.id).text("✅").await?;
            }
        }
    } else if let Some(title) = data.strip_prefix("clarify:asis:") {
        // Create task with original vague title
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let lang = user.lang();
            match Task::create(&pool, user.id, title, None, None, None).await {
                Ok(task) => {
                    if let Some(msg) = q.message {
                        let mut args = FluentArgs::new();
                        args.set("title", task.title.clone());
                        args.set("due", "");
                        bot.edit_message_text(
                            msg.chat().id,
                            msg.id(),
                            i18n.t_args(lang, "task-added", &args),
                        )
                        .reply_markup(task_keyboard(task.id, &i18n, lang))
                        .await?;
                    }
                    bot.answer_callback_query(q.id).text("✅").await?;
                }
                Err(e) => {
                    tracing::error!("Failed to create task: {}", e);
                    bot.answer_callback_query(q.id).text("❌").await?;
                }
            }
        }
    } else if let Some(title) = data.strip_prefix("clarify:") {
        // Create task with suggested specific title
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let lang = user.lang();
            match Task::create(&pool, user.id, title, None, None, None).await {
                Ok(task) => {
                    if let Some(msg) = q.message {
                        let mut args = FluentArgs::new();
                        args.set("title", task.title.clone());
                        args.set("due", "");
                        bot.edit_message_text(
                            msg.chat().id,
                            msg.id(),
                            i18n.t_args(lang, "task-added", &args),
                        )
                        .reply_markup(task_keyboard(task.id, &i18n, lang))
                        .await?;
                    }
                    bot.answer_callback_query(q.id).text("✅").await?;
                }
                Err(e) => {
                    tracing::error!("Failed to create task: {}", e);
                    bot.answer_callback_query(q.id).text("❌").await?;
                }
            }
        }
    } else if data == "settings:timezone" {
        // Show timezone options with auto-detect
        let telegram_id = q.from.id.0 as i64;
        let lang = User::find_by_telegram_id(&pool, telegram_id).await
            .map(|u| u.lang().to_string())
            .unwrap_or_else(|| "en".to_string());

        let tz_keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback(&i18n.t(&lang, "btn-auto-detect"), "tz:auto"),
                InlineKeyboardButton::callback(&i18n.t(&lang, "btn-type-city"), "tz:city"),
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
                InlineKeyboardButton::callback(&i18n.t(&lang, "btn-back"), "settings:back"),
            ],
        ]);

        if let Some(msg) = &q.message {
            bot.edit_message_text(
                msg.chat().id,
                msg.id(),
                i18n.t(&lang, "tz-select-title"),
            )
            .reply_markup(tz_keyboard)
            .await?;
        }

        bot.answer_callback_query(q.id).await?;
    } else if data == "tz:auto" {
        // Request location for auto-detect
        let telegram_id = q.from.id.0 as i64;
        let lang = User::find_by_telegram_id(&pool, telegram_id).await
            .map(|u| u.lang().to_string())
            .unwrap_or_else(|| "en".to_string());

        if let Some(msg) = &q.message {
            let btn_text = if lang == "ru" { "📍 Поделиться геолокацией" } else { "📍 Share my location" };
            let location_keyboard = KeyboardMarkup::new(vec![vec![
                KeyboardButton::new(btn_text).request(teloxide::types::ButtonRequest::Location),
            ]])
            .resize_keyboard()
            .one_time_keyboard();

            bot.send_message(
                msg.chat().id,
                i18n.t(&lang, "tz-auto-prompt"),
            )
            .reply_markup(location_keyboard)
            .await?;
        }

        bot.answer_callback_query(q.id).await?;
    } else if data == "tz:city" {
        // Ask for city name
        let telegram_id = q.from.id.0 as i64;
        let lang = User::find_by_telegram_id(&pool, telegram_id).await
            .map(|u| u.lang().to_string())
            .unwrap_or_else(|| "en".to_string());

        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            context.set_pending_edit(user.id, PendingEdit::Timezone);
        }

        if let Some(msg) = &q.message {
            bot.send_message(
                msg.chat().id,
                i18n.t(&lang, "tz-city-prompt"),
            )
            .await?;
        }

        bot.answer_callback_query(q.id).await?;
    } else if let Some(tz) = data.strip_prefix("tz:") {
        // Set timezone
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let lang = user.lang();
            let _ = User::update_timezone(&pool, user.id, tz).await;

            if let Some(msg) = &q.message {
                let mut args = FluentArgs::new();
                args.set("tz", tz.to_string());
                bot.edit_message_text(
                    msg.chat().id,
                    msg.id(),
                    i18n.t_args(lang, "tz-updated", &args),
                )
                .await?;
            }

            bot.answer_callback_query(q.id).text("✅").await?;
        }
    } else if data == "settings:brief_time" {
        // Show brief time options
        let telegram_id = q.from.id.0 as i64;
        let lang = User::find_by_telegram_id(&pool, telegram_id).await
            .map(|u| u.lang().to_string())
            .unwrap_or_else(|| "en".to_string());

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
                InlineKeyboardButton::callback(&i18n.t(&lang, "btn-back"), "settings:back"),
            ],
        ]);

        if let Some(msg) = &q.message {
            bot.edit_message_text(
                msg.chat().id,
                msg.id(),
                i18n.t(&lang, "brief-select-title"),
            )
            .reply_markup(time_keyboard)
            .await?;
        }

        bot.answer_callback_query(q.id).await?;
    } else if let Some(time) = data.strip_prefix("brief:") {
        // Set brief time
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let lang = user.lang();
            let _ = User::update_morning_brief_time(&pool, user.id, time).await;

            if let Some(msg) = &q.message {
                let mut args = FluentArgs::new();
                args.set("time", time.to_string());
                bot.edit_message_text(
                    msg.chat().id,
                    msg.id(),
                    i18n.t_args(lang, "brief-updated", &args),
                )
                .await?;
            }

            bot.answer_callback_query(q.id).text("✅").await?;
        }
    } else if data == "settings:language" {
        // Show language options
        let lang_keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("🇬🇧 English", "lang:en"),
                InlineKeyboardButton::callback("🇷🇺 Русский", "lang:ru"),
            ],
            vec![
                InlineKeyboardButton::callback("↩️ Back", "settings:back"),
            ],
        ]);

        if let Some(msg) = &q.message {
            bot.edit_message_text(
                msg.chat().id,
                msg.id(),
                i18n.t("en", "lang-select-title"), // This is intentionally bilingual
            )
            .reply_markup(lang_keyboard)
            .await?;
        }

        bot.answer_callback_query(q.id).await?;
    } else if let Some(lang) = data.strip_prefix("lang:") {
        // Set language
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let _ = User::update_language(&pool, user.id, lang).await;

            // Use the NEW language for confirmation
            let confirmation_key = match lang {
                "ru" => "lang-updated-ru",
                _ => "lang-updated-en",
            };

            if let Some(msg) = &q.message {
                bot.edit_message_text(
                    msg.chat().id,
                    msg.id(),
                    i18n.t(lang, confirmation_key),
                )
                .await?;
            }

            bot.answer_callback_query(q.id).text("✅").await?;
        }
    } else if data == "settings:back" {
        // Back to settings - redirect to settings handler
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let lang = user.lang();
            let show_subscribe = !user.has_active_subscription()
                || user.trial_days_remaining().is_some();

            let mut keyboard_rows = vec![
                vec![
                    InlineKeyboardButton::callback(&i18n.t(lang, "btn-timezone"), "settings:timezone"),
                    InlineKeyboardButton::callback(&i18n.t(lang, "btn-brief-time"), "settings:brief_time"),
                ],
                vec![
                    InlineKeyboardButton::callback(&i18n.t(lang, "btn-language"), "settings:language"),
                    InlineKeyboardButton::callback(&i18n.t(lang, "btn-invite"), "settings:invite"),
                ],
            ];

            if show_subscribe {
                keyboard_rows.push(vec![
                    InlineKeyboardButton::callback(&i18n.t(lang, "btn-subscribe"), "subscribe"),
                ]);
            }

            let settings_keyboard = InlineKeyboardMarkup::new(keyboard_rows);

            let lang_display = match lang {
                "ru" => "🇷🇺 Русский",
                _ => "🇬🇧 English",
            };

            let mut args = FluentArgs::new();
            args.set("status", user.subscription_status());
            args.set("tz", user.timezone.clone());
            args.set("time", user.morning_brief_time.clone());
            args.set("lang", lang_display);

            if let Some(msg) = &q.message {
                bot.edit_message_text(
                    msg.chat().id,
                    msg.id(),
                    i18n.t_args(lang, "settings-full", &args),
                )
                .parse_mode(teloxide::types::ParseMode::Html)
                .reply_markup(settings_keyboard)
                .await?;
            }
        }

        bot.answer_callback_query(q.id).await?;
    } else if data == "settings:invite" {
        // Show invite/referral info
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let lang = user.lang();
            let referral_code = User::ensure_referral_code(&pool, user.id).await.ok();
            let (referral_count, bonus_days) = user.referral_stats();

            let code = referral_code.as_deref().unwrap_or("");
            let mut args = FluentArgs::new();
            args.set("bot", BOT_USERNAME);
            args.set("code", code.to_string());
            args.set("count", referral_count);
            args.set("bonus", bonus_days);

            let invite_keyboard = InlineKeyboardMarkup::new(vec![
                vec![InlineKeyboardButton::callback(&i18n.t(lang, "btn-back"), "settings:back")],
            ]);

            if let Some(msg) = &q.message {
                bot.edit_message_text(msg.chat().id, msg.id(), i18n.t_args(lang, "invite-full", &args))
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .reply_markup(invite_keyboard)
                    .await?;
            }
        }

        bot.answer_callback_query(q.id).await?;
    } else if data == "settings" {
        // Show settings menu
        let telegram_id = q.from.id.0 as i64;
        if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let lang = user.lang();
            let show_subscribe = !user.has_active_subscription()
                || user.trial_days_remaining().is_some();

            let mut keyboard_rows = vec![
                vec![
                    InlineKeyboardButton::callback(&i18n.t(lang, "btn-timezone"), "settings:timezone"),
                    InlineKeyboardButton::callback(&i18n.t(lang, "btn-brief-time"), "settings:brief_time"),
                ],
                vec![
                    InlineKeyboardButton::callback(&i18n.t(lang, "btn-language"), "settings:language"),
                    InlineKeyboardButton::callback(&i18n.t(lang, "btn-invite"), "settings:invite"),
                ],
            ];

            if show_subscribe {
                keyboard_rows.push(vec![
                    InlineKeyboardButton::callback(&i18n.t(lang, "btn-subscribe"), "subscribe"),
                ]);
            }

            let settings_keyboard = InlineKeyboardMarkup::new(keyboard_rows);

            let lang_display = match lang {
                "ru" => "🇷🇺 Русский",
                _ => "🇬🇧 English",
            };

            let mut args = FluentArgs::new();
            args.set("status", user.subscription_status());
            args.set("tz", user.timezone.clone());
            args.set("time", user.morning_brief_time.clone());
            args.set("lang", lang_display);

            if let Some(msg) = &q.message {
                bot.edit_message_text(
                    msg.chat().id,
                    msg.id(),
                    i18n.t_args(lang, "settings-full", &args),
                )
                .parse_mode(teloxide::types::ParseMode::Html)
                .reply_markup(settings_keyboard)
                .await?;
            }
        }

        bot.answer_callback_query(q.id).await?;
    } else if let Some(task_id_str) = data.strip_prefix("edit:") {
        // Show edit options
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                let telegram_id = q.from.id.0 as i64;
                let lang = User::find_by_telegram_id(&pool, telegram_id).await
                    .map(|u| u.lang().to_string())
                    .unwrap_or_else(|| "en".to_string());

                let edit_keyboard = InlineKeyboardMarkup::new(vec![
                    vec![
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-edit-title"), format!("edit_title:{}", task_id)),
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-edit-date"), format!("edit_date:{}", task_id)),
                    ],
                    vec![
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-back"), format!("cancel_delete:{}", task_id)),
                    ],
                ]);

                if let Some(msg) = &q.message {
                    let due_str = task.due_at.as_ref()
                        .map(|d| format!("\n📅 {}", d))
                        .unwrap_or_default();

                    let mut args = FluentArgs::new();
                    args.set("title", task.title.clone());
                    args.set("due", due_str.clone());

                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        i18n.t_args(&lang, "edit-title", &args),
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
                let lang = if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
                    context.set_pending_edit(user.id, PendingEdit::Title(task_id));
                    user.lang().to_string()
                } else {
                    "en".to_string()
                };

                if let Some(msg) = &q.message {
                    let cancel_keyboard = InlineKeyboardMarkup::new(vec![vec![
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-cancel"), format!("cancel_edit:{}", task_id)),
                    ]]);

                    let mut args = FluentArgs::new();
                    args.set("title", task.title.clone());

                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        i18n.t_args(&lang, "edit-send-title", &args),
                    )
                    .reply_markup(cancel_keyboard)
                    .await?;
                }

                bot.answer_callback_query(q.id).await?;
            }
        }
    } else if let Some(task_id_str) = data.strip_prefix("confirm_edit:") {
        // Apply confirmed edit
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            let telegram_id = q.from.id.0 as i64;
            if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
                let lang = user.lang();
                if let Some(PendingEdit::ConfirmEdit(proposed)) = context.take_pending_edit(user.id) {
                    // Apply the changes
                    let _ = Task::update(&pool, task_id, Some(&proposed.new_title), Some(proposed.new_due_at.as_deref())).await;

                    if let Some(task) = Task::find_by_id(&pool, task_id).await {
                        if let Some(msg) = &q.message {
                            let due_str = task.due_at.as_ref()
                                .map(|d| format!("\n📅 {}", d))
                                .unwrap_or_default();

                            let mut args = FluentArgs::new();
                            args.set("title", task.title.clone());
                            args.set("due", due_str.clone());

                            bot.edit_message_text(
                                msg.chat().id,
                                msg.id(),
                                i18n.t_args(lang, "edit-applied", &args),
                            )
                            .reply_markup(task_keyboard(task_id, &i18n, lang))
                            .await?;
                        }
                    }

                    bot.answer_callback_query(q.id).text("✅").await?;
                }
            }
        }
    } else if data == "cancel_support" {
        // Cancel support message
        let telegram_id = q.from.id.0 as i64;
        let lang = if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
            let _ = context.take_pending_edit(user.id);
            user.lang().to_string()
        } else {
            "en".to_string()
        };

        if let Some(msg) = &q.message {
            bot.edit_message_text(
                msg.chat().id,
                msg.id(),
                i18n.t(&lang, "support-cancelled"),
            )
            .await?;
        }

        bot.answer_callback_query(q.id).await?;
    } else if let Some(task_id_str) = data.strip_prefix("cancel_edit:") {
        // Cancel pending edit and restore task view
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            // Clear pending edit and get user lang
            let telegram_id = q.from.id.0 as i64;
            let lang = if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
                let _ = context.take_pending_edit(user.id);
                user.lang().to_string()
            } else {
                "en".to_string()
            };

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
                    .reply_markup(task_keyboard(task_id, &i18n, &lang))
                    .await?;
                }
            }

            bot.answer_callback_query(q.id).text("Cancelled").await?;
        }
    } else if let Some(task_id_str) = data.strip_prefix("edit_date:") {
        // Show date options
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if Task::find_by_id(&pool, task_id).await.is_some() {
                let telegram_id = q.from.id.0 as i64;
                let lang = User::find_by_telegram_id(&pool, telegram_id).await
                    .map(|u| u.lang().to_string())
                    .unwrap_or_else(|| "en".to_string());

                let date_keyboard = InlineKeyboardMarkup::new(vec![
                    vec![
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-date-today"), format!("set_date:{}:today", task_id)),
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-date-tomorrow"), format!("set_date:{}:tomorrow", task_id)),
                    ],
                    vec![
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-date-next-week"), format!("set_date:{}:next_week", task_id)),
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-date-remove"), format!("set_date:{}:none", task_id)),
                    ],
                    vec![
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-back"), format!("edit:{}", task_id)),
                    ],
                ]);

                if let Some(msg) = &q.message {
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        i18n.t(&lang, "date-select-title"),
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

                let telegram_id = q.from.id.0 as i64;
                let lang = User::find_by_telegram_id(&pool, telegram_id).await
                    .map(|u| u.lang().to_string())
                    .unwrap_or_else(|| "en".to_string());

                if let Some(task) = Task::find_by_id(&pool, task_id).await {
                    if let Some(msg) = &q.message {
                        let due_str = task.due_at.as_ref()
                            .map(|d| format!("\n📅 {}", d))
                            .unwrap_or_default();

                        let mut args = FluentArgs::new();
                        args.set("title", task.title.clone());
                        args.set("due", due_str.clone());

                        bot.edit_message_text(
                            msg.chat().id,
                            msg.id(),
                            i18n.t_args(&lang, "date-updated-confirm", &args),
                        )
                        .reply_markup(task_keyboard(task_id, &i18n, &lang))
                        .await?;
                    }
                }

                bot.answer_callback_query(q.id).text(&i18n.t(&lang, "date-updated")).await?;
            }
        }
    } else if let Some(task_id_str) = data.strip_prefix("remind:") {
        // Show reminder options
        if let Ok(task_id) = task_id_str.parse::<i64>() {
            if let Some(task) = Task::find_by_id(&pool, task_id).await {
                let telegram_id = q.from.id.0 as i64;
                let lang = User::find_by_telegram_id(&pool, telegram_id).await
                    .map(|u| u.lang().to_string())
                    .unwrap_or_else(|| "en".to_string());

                let reminder_str = task.reminder_at.as_ref()
                    .map(|r| {
                        let mut args = FluentArgs::new();
                        args.set("reminder", r.clone());
                        i18n.t_args(&lang, "reminder-current", &args)
                    })
                    .unwrap_or_else(|| i18n.t(&lang, "no-reminder"));

                let remind_keyboard = InlineKeyboardMarkup::new(vec![
                    vec![
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-remind-30min"), format!("set_remind:{}:30", task_id)),
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-remind-1h"), format!("set_remind:{}:60", task_id)),
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-remind-3h"), format!("set_remind:{}:180", task_id)),
                    ],
                    vec![
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-remind-tomorrow"), format!("set_remind:{}:tomorrow", task_id)),
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-remind-custom"), format!("set_remind:{}:custom", task_id)),
                    ],
                    vec![
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-remind-remove"), format!("set_remind:{}:none", task_id)),
                        InlineKeyboardButton::callback(&i18n.t(&lang, "btn-back"), format!("cancel_delete:{}", task_id)),
                    ],
                ]);

                if let Some(msg) = &q.message {
                    let mut args = FluentArgs::new();
                    args.set("title", task.title.clone());
                    args.set("current", reminder_str.clone());

                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        i18n.t_args(&lang, "remind-select-title", &args),
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
                    let lang = if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
                        context.set_pending_edit(user.id, PendingEdit::Reminder(task_id));
                        user.lang().to_string()
                    } else {
                        "en".to_string()
                    };

                    if let Some(task) = Task::find_by_id(&pool, task_id).await {
                        if let Some(msg) = &q.message {
                            let cancel_keyboard = InlineKeyboardMarkup::new(vec![vec![
                                InlineKeyboardButton::callback(&i18n.t(&lang, "btn-cancel"), format!("cancel_edit:{}", task_id)),
                            ]]);

                            let mut args = FluentArgs::new();
                            args.set("title", task.title.clone());

                            bot.edit_message_text(
                                msg.chat().id,
                                msg.id(),
                                i18n.t_args(&lang, "remind-custom-prompt", &args),
                            )
                            .reply_markup(cancel_keyboard)
                            .await?;
                        }
                    }

                    bot.answer_callback_query(q.id).await?;
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

                let telegram_id = q.from.id.0 as i64;
                let lang = User::find_by_telegram_id(&pool, telegram_id).await
                    .map(|u| u.lang().to_string())
                    .unwrap_or_else(|| "en".to_string());

                if let Some(task) = Task::find_by_id(&pool, task_id).await {
                    if let Some(msg) = &q.message {
                        let due_str = task.due_at.as_ref()
                            .map(|d| format!("\n📅 {}", d))
                            .unwrap_or_default();

                        let reminder_msg = if option == "none" {
                            i18n.t(&lang, "remind-removed")
                        } else {
                            let mut args = FluentArgs::new();
                            args.set("when", confirm_text.to_string());
                            i18n.t_args(&lang, "remind-set", &args)
                        };

                        bot.edit_message_text(
                            msg.chat().id,
                            msg.id(),
                            format!("{}\n\n📝 {}{}", reminder_msg, task.title, due_str),
                        )
                        .reply_markup(task_keyboard(task_id, &i18n, &lang))
                        .await?;
                    }
                }

                bot.answer_callback_query(q.id).text("⏰").await?;
            }
        }
    } else if data == "subscribe" {
        // Show subscription options
        let subscribe_keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("⭐ 1 month — 250 Stars", "buy:1"),
            ],
            vec![
                InlineKeyboardButton::callback("⭐ 3 months — 600 Stars (save $3)", "buy:3"),
            ],
            vec![
                InlineKeyboardButton::callback("⭐ 12 months — 2000 Stars (save $20)", "buy:12"),
            ],
            vec![
                InlineKeyboardButton::callback("↩️ Back", "settings:back"),
            ],
        ]);

        if let Some(msg) = &q.message {
            bot.edit_message_text(
                msg.chat().id,
                msg.id(),
                "⭐ <b>Choose your plan</b>\n\n\
                All plans include:\n\
                • Unlimited AI task creation\n\
                • Voice messages\n\
                • Smart reminders\n\
                • Message drafts\n\n\
                Pay with Telegram Stars:",
            )
            .parse_mode(teloxide::types::ParseMode::Html)
            .reply_markup(subscribe_keyboard)
            .await?;
        }

        bot.answer_callback_query(q.id).await?;
    } else if let Some(months_str) = data.strip_prefix("buy:") {
        // Send invoice for selected plan
        if let Ok(months) = months_str.parse::<i32>() {
            let telegram_id = q.from.id.0 as i64;
            if let Some(user) = User::find_by_telegram_id(&pool, telegram_id).await {
                if let Some(msg) = &q.message {
                    // Delete the menu message
                    let _ = bot.delete_message(msg.chat().id, msg.id()).await;

                    // Send invoice
                    match send_subscription_invoice(&bot, msg.chat().id, months, user.id).await {
                        Ok(_) => {
                            tracing::info!("Invoice sent to user {} for {} months", user.id, months);
                        }
                        Err(e) => {
                            tracing::error!("Failed to send invoice: {}", e);
                            bot.send_message(msg.chat().id, "❌ Failed to create invoice. Please try again.")
                                .await?;
                        }
                    }
                }
            }
        }

        bot.answer_callback_query(q.id).await?;
    } else if data.starts_with("admin:") {
        // Admin callbacks - check permission first
        let telegram_id = q.from.id.0 as i64;
        if !is_admin(telegram_id) {
            bot.answer_callback_query(q.id).text("⛔ Access denied").await?;
            return Ok(());
        }

        let action = data.strip_prefix("admin:").unwrap_or("");

        match action {
            "stats" => {
                if let Ok(stats) = User::admin_stats(&pool).await {
                    let stats_text = format!(
"📊 <b>Statistics</b>

👥 <b>Users</b>
├─ Total: <b>{}</b>
├─ 🎁 Trial: <b>{}</b>
├─ ✅ Paid: <b>{}</b>
├─ ❌ Expired: <b>{}</b>
└─ 🚫 Banned: <b>{}</b>

📈 <b>Activity</b>
├─ Active 7d: <b>{}</b>
└─ Active 30d: <b>{}</b>

🆕 <b>Growth</b>
├─ Today: <b>+{}</b>
└─ This week: <b>+{}</b>",
                        stats.total,
                        stats.trial,
                        stats.paid,
                        stats.expired,
                        stats.banned,
                        stats.active_7d,
                        stats.active_30d,
                        stats.new_today,
                        stats.new_week
                    );

                    let back_keyboard = InlineKeyboardMarkup::new(vec![
                        vec![InlineKeyboardButton::callback("↩️ Back", "admin:menu")],
                    ]);

                    if let Some(msg) = &q.message {
                        bot.edit_message_text(msg.chat().id, msg.id(), stats_text)
                            .parse_mode(teloxide::types::ParseMode::Html)
                            .reply_markup(back_keyboard)
                            .await?;
                    }
                }
                bot.answer_callback_query(q.id).await?;
            }
            "users" => {
                let users_keyboard = InlineKeyboardMarkup::new(vec![
                    vec![
                        InlineKeyboardButton::callback("🔍 Search", "admin:search"),
                        InlineKeyboardButton::callback("📋 Recent", "admin:recent"),
                    ],
                    vec![InlineKeyboardButton::callback("↩️ Back", "admin:menu")],
                ]);

                if let Some(msg) = &q.message {
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        "👥 <b>User Management</b>\n\nSearch by @username, name or ID:",
                    )
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .reply_markup(users_keyboard)
                    .await?;
                }
                bot.answer_callback_query(q.id).await?;
            }
            "search" => {
                // Set pending admin search
                context.set_pending_edit(telegram_id, PendingEdit::AdminSearch);

                let cancel_keyboard = InlineKeyboardMarkup::new(vec![
                    vec![InlineKeyboardButton::callback("↩️ Cancel", "admin:users")],
                ]);

                if let Some(msg) = &q.message {
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        "🔍 <b>Search User</b>\n\nSend @username, name or telegram ID:",
                    )
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .reply_markup(cancel_keyboard)
                    .await?;
                }
                bot.answer_callback_query(q.id).text("Enter search query").await?;
            }
            "recent" => {
                if let Ok(users) = User::list_all(&pool, 10, 0).await {
                    let mut text = "📋 <b>Recent Users</b>\n\n".to_string();
                    for u in &users {
                        let status = if u.is_banned() {
                            "🚫"
                        } else if u.subscription_days_remaining().is_some() {
                            "✅"
                        } else if u.trial_days_remaining().is_some() {
                            "🎁"
                        } else {
                            "❌"
                        };
                        text.push_str(&format!(
                            "{} {} (ID: {})\n",
                            status,
                            u.display_name(),
                            u.id
                        ));
                    }

                    let back_keyboard = InlineKeyboardMarkup::new(vec![
                        vec![InlineKeyboardButton::callback("↩️ Back", "admin:users")],
                    ]);

                    if let Some(msg) = &q.message {
                        bot.edit_message_text(msg.chat().id, msg.id(), text)
                            .parse_mode(teloxide::types::ParseMode::Html)
                            .reply_markup(back_keyboard)
                            .await?;
                    }
                }
                bot.answer_callback_query(q.id).await?;
            }
            "broadcast" => {
                let broadcast_keyboard = InlineKeyboardMarkup::new(vec![
                    vec![
                        InlineKeyboardButton::callback("📢 All users", "admin:bc:all"),
                        InlineKeyboardButton::callback("🎁 Trial", "admin:bc:trial"),
                    ],
                    vec![
                        InlineKeyboardButton::callback("✅ Paid", "admin:bc:paid"),
                        InlineKeyboardButton::callback("❌ Expired", "admin:bc:expired"),
                    ],
                    vec![InlineKeyboardButton::callback("↩️ Back", "admin:menu")],
                ]);

                if let Some(msg) = &q.message {
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        "📢 <b>Broadcast</b>\n\nSelect target audience:",
                    )
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .reply_markup(broadcast_keyboard)
                    .await?;
                }
                bot.answer_callback_query(q.id).await?;
            }
            "banned" => {
                if let Ok(banned) = User::list_banned(&pool).await {
                    let text = if banned.is_empty() {
                        "🚫 <b>Banned Users</b>\n\nNo banned users.".to_string()
                    } else {
                        let mut t = "🚫 <b>Banned Users</b>\n\n".to_string();
                        for u in &banned {
                            t.push_str(&format!(
                                "• {} — {}\n",
                                u.display_name(),
                                u.ban_reason.as_deref().unwrap_or("No reason")
                            ));
                        }
                        t
                    };

                    let back_keyboard = InlineKeyboardMarkup::new(vec![
                        vec![InlineKeyboardButton::callback("↩️ Back", "admin:menu")],
                    ]);

                    if let Some(msg) = &q.message {
                        bot.edit_message_text(msg.chat().id, msg.id(), text)
                            .parse_mode(teloxide::types::ParseMode::Html)
                            .reply_markup(back_keyboard)
                            .await?;
                    }
                }
                bot.answer_callback_query(q.id).await?;
            }
            "menu" => {
                let admin_keyboard = InlineKeyboardMarkup::new(vec![
                    vec![
                        InlineKeyboardButton::callback("📊 Stats", "admin:stats"),
                        InlineKeyboardButton::callback("👥 Users", "admin:users"),
                    ],
                    vec![
                        InlineKeyboardButton::callback("📢 Broadcast", "admin:broadcast"),
                        InlineKeyboardButton::callback("🚫 Banned", "admin:banned"),
                    ],
                ]);

                if let Some(msg) = &q.message {
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        "🔐 <b>Admin Panel</b>\n\nSelect an option:",
                    )
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .reply_markup(admin_keyboard)
                    .await?;
                }
                bot.answer_callback_query(q.id).await?;
            }
            _ => {
                bot.answer_callback_query(q.id).await?;
            }
        }
    } else if let Some(bc_segment) = data.strip_prefix("admin:bc:") {
        // Broadcast segment selection
        let telegram_id = q.from.id.0 as i64;
        if !is_admin(telegram_id) {
            bot.answer_callback_query(q.id).text("⛔ Access denied").await?;
            return Ok(());
        }

        // Set pending broadcast with segment
        context.set_pending_edit(telegram_id, PendingEdit::AdminBroadcast(bc_segment.to_string()));

        let segment_name = match bc_segment {
            "all" => "all users",
            "trial" => "trial users",
            "paid" => "paid users",
            "expired" => "expired users",
            _ => "users",
        };

        let cancel_keyboard = InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback("↩️ Cancel", "admin:broadcast")],
        ]);

        if let Some(msg) = &q.message {
            bot.edit_message_text(
                msg.chat().id,
                msg.id(),
                format!("📢 <b>Broadcast to {}</b>\n\nSend your message:", segment_name),
            )
            .parse_mode(teloxide::types::ParseMode::Html)
            .reply_markup(cancel_keyboard)
            .await?;
        }

        bot.answer_callback_query(q.id).text("Send broadcast message").await?;
    } else if let Some(user_id_str) = data.strip_prefix("admin:user:") {
        // Show user card
        let telegram_id = q.from.id.0 as i64;
        if !is_admin(telegram_id) {
            bot.answer_callback_query(q.id).text("⛔ Access denied").await?;
            return Ok(());
        }

        if let Ok(user_id) = user_id_str.parse::<i64>() {
            if let Some(user) = User::find_by_id(&pool, user_id).await {
                let status = user.subscription_status();
                let tasks_count = Task::count_by_user(&pool, user.id).await.unwrap_or(0);

                let card_text = format!(
"👤 <b>{}</b>
ID: <code>{}</code> | TG: <code>{}</code>

📊 Status: {}
📅 Joined: {}
📝 Tasks: {}
🔗 Referrals: {}",
                    user.display_name(),
                    user.id,
                    user.telegram_id,
                    status,
                    user.created_at.split(' ').next().unwrap_or(&user.created_at),
                    tasks_count,
                    user.referral_count.unwrap_or(0)
                );

                let mut buttons = vec![
                    vec![
                        InlineKeyboardButton::callback("📅 +30 days", format!("admin:grant:{}:30", user.id)),
                        InlineKeyboardButton::callback("📅 +90 days", format!("admin:grant:{}:90", user.id)),
                    ],
                    vec![
                        InlineKeyboardButton::callback("📅 +365 days", format!("admin:grant:{}:365", user.id)),
                    ],
                ];

                if user.is_banned() {
                    buttons.push(vec![
                        InlineKeyboardButton::callback("✅ Unban", format!("admin:unban:{}", user.id)),
                    ]);
                } else {
                    buttons.push(vec![
                        InlineKeyboardButton::callback("🚫 Ban", format!("admin:ban:{}", user.id)),
                        InlineKeyboardButton::callback("💬 Message", format!("admin:msg:{}", user.id)),
                    ]);
                }

                buttons.push(vec![
                    InlineKeyboardButton::callback("↩️ Back", "admin:users"),
                ]);

                let user_keyboard = InlineKeyboardMarkup::new(buttons);

                if let Some(msg) = &q.message {
                    bot.edit_message_text(msg.chat().id, msg.id(), card_text)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .reply_markup(user_keyboard)
                        .await?;
                }
            }
        }

        bot.answer_callback_query(q.id).await?;
    } else if let Some(grant_data) = data.strip_prefix("admin:grant:") {
        // Grant subscription
        let telegram_id = q.from.id.0 as i64;
        if !is_admin(telegram_id) {
            bot.answer_callback_query(q.id).text("⛔ Access denied").await?;
            return Ok(());
        }

        let parts: Vec<&str> = grant_data.split(':').collect();
        if parts.len() == 2 {
            if let (Ok(user_id), Ok(days)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                if User::grant_subscription(&pool, user_id, days).await.is_ok() {
                    bot.answer_callback_query(q.id).text(format!("✅ Granted {} days", days)).await?;

                    // Refresh user card
                    if let Some(user) = User::find_by_id(&pool, user_id).await {
                        let status = user.subscription_status();
                        if let Some(msg) = &q.message {
                            bot.edit_message_text(
                                msg.chat().id,
                                msg.id(),
                                format!("✅ Subscription granted!\n\n👤 {} — {}", user.display_name(), status),
                            )
                            .reply_markup(InlineKeyboardMarkup::new(vec![
                                vec![InlineKeyboardButton::callback("↩️ Back", "admin:users")],
                            ]))
                            .await?;
                        }
                    }
                } else {
                    bot.answer_callback_query(q.id).text("❌ Failed").await?;
                }
            }
        }
    } else if let Some(user_id_str) = data.strip_prefix("admin:ban:") {
        // Ban user
        let telegram_id = q.from.id.0 as i64;
        if !is_admin(telegram_id) {
            bot.answer_callback_query(q.id).text("⛔ Access denied").await?;
            return Ok(());
        }

        if let Ok(user_id) = user_id_str.parse::<i64>() {
            if User::ban(&pool, user_id, Some("Banned by admin")).await.is_ok() {
                bot.answer_callback_query(q.id).text("🚫 User banned").await?;

                if let Some(msg) = &q.message {
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        "🚫 User has been banned.",
                    )
                    .reply_markup(InlineKeyboardMarkup::new(vec![
                        vec![InlineKeyboardButton::callback("↩️ Back", "admin:users")],
                    ]))
                    .await?;
                }
            }
        }
    } else if let Some(user_id_str) = data.strip_prefix("admin:unban:") {
        // Unban user
        let telegram_id = q.from.id.0 as i64;
        if !is_admin(telegram_id) {
            bot.answer_callback_query(q.id).text("⛔ Access denied").await?;
            return Ok(());
        }

        if let Ok(user_id) = user_id_str.parse::<i64>() {
            if User::unban(&pool, user_id).await.is_ok() {
                bot.answer_callback_query(q.id).text("✅ User unbanned").await?;

                if let Some(msg) = &q.message {
                    bot.edit_message_text(
                        msg.chat().id,
                        msg.id(),
                        "✅ User has been unbanned.",
                    )
                    .reply_markup(InlineKeyboardMarkup::new(vec![
                        vec![InlineKeyboardButton::callback("↩️ Back", "admin:users")],
                    ]))
                    .await?;
                }
            }
        }
    } else if let Some(user_id_str) = data.strip_prefix("admin:msg:") {
        // Set pending admin message to user
        let telegram_id = q.from.id.0 as i64;
        if !is_admin(telegram_id) {
            bot.answer_callback_query(q.id).text("⛔ Access denied").await?;
            return Ok(());
        }

        if let Ok(user_id) = user_id_str.parse::<i64>() {
            context.set_pending_edit(telegram_id, PendingEdit::AdminMessage(user_id));

            let cancel_keyboard = InlineKeyboardMarkup::new(vec![
                vec![InlineKeyboardButton::callback("↩️ Cancel", format!("admin:user:{}", user_id))],
            ]);

            if let Some(msg) = &q.message {
                bot.edit_message_text(
                    msg.chat().id,
                    msg.id(),
                    "💬 <b>Send Message</b>\n\nType your message to send to this user:",
                )
                .parse_mode(teloxide::types::ParseMode::Html)
                .reply_markup(cancel_keyboard)
                .await?;
            }
        }

        bot.answer_callback_query(q.id).text("Type message").await?;
    }

    Ok(())
}

/// Handle pre-checkout query for Telegram Stars payments
pub async fn pre_checkout_handler(
    bot: Bot,
    q: PreCheckoutQuery,
) -> ResponseResult<()> {
    // For Stars payments, we always approve
    // In production, you might want to validate the payload
    tracing::info!("Pre-checkout query: {:?}", q);

    // Validate payload format
    let parts: Vec<&str> = q.invoice_payload.split(':').collect();
    if parts.len() == 3 && parts[0] == "sub" {
        // Valid subscription payload
        bot.answer_pre_checkout_query(q.id, true).await?;
    } else {
        bot.answer_pre_checkout_query(q.id, false)
            .error_message("Invalid payment payload")
            .await?;
    }

    Ok(())
}
