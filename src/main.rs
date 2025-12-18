use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod handlers;
mod i18n;
mod models;
mod services;

use i18n::I18n;
use services::{AiService, ConversationContext, morning_brief, reminder, weekly_review};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting AI Todolist bot...");

    // Initialize database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:data/bot.db?mode=rwc".into());

    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to initialize database");

    // Initialize AI service (optional - works without it)
    let ai_service = std::env::var("OPENAI_API_KEY")
        .ok()
        .map(|key| Arc::new(AiService::new(key)));

    let pool = Arc::new(pool);
    let bot = Bot::from_env();
    let context = Arc::new(ConversationContext::new());
    let i18n = Arc::new(I18n::new());

    // Register bot commands for menu
    if let Err(e) = bot.set_my_commands(handlers::Command::bot_commands()).await {
        tracing::warn!("Failed to set bot commands: {}", e);
    }

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handlers::message_handler))
        .branch(Update::filter_callback_query().endpoint(handlers::callback_handler))
        .branch(Update::filter_pre_checkout_query().endpoint(handlers::pre_checkout_handler));

    // Start reminder service in background
    let reminder_bot = bot.clone();
    let reminder_pool = pool.clone();
    let reminder_i18n = i18n.clone();
    tokio::spawn(async move {
        reminder::start_reminder_loop(reminder_bot, reminder_pool, reminder_i18n).await;
    });

    // Start morning brief service in background
    let brief_bot = bot.clone();
    let brief_pool = pool.clone();
    let brief_i18n = i18n.clone();
    tokio::spawn(async move {
        morning_brief::start_morning_brief_loop(brief_bot, brief_pool, brief_i18n).await;
    });

    // Start weekly review service in background
    let weekly_bot = bot.clone();
    let weekly_pool = pool.clone();
    let weekly_i18n = i18n.clone();
    tokio::spawn(async move {
        weekly_review::start_weekly_review_loop(weekly_bot, weekly_pool, weekly_i18n).await;
    });

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![pool.clone(), ai_service, context, i18n, pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
