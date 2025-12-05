use sqlx::SqlitePool;
use std::sync::Arc;
use teloxide::prelude::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod handlers;
mod models;
mod services;

use services::{AiService, reminder};

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

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handlers::message_handler))
        .branch(Update::filter_callback_query().endpoint(handlers::callback_handler));

    // Start reminder service in background
    let reminder_bot = bot.clone();
    let reminder_pool = pool.clone();
    tokio::spawn(async move {
        reminder::start_reminder_loop(reminder_bot, reminder_pool).await;
    });

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![pool.clone(), ai_service, pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
