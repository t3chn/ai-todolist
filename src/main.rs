use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use std::net::SocketAddr;
use teloxide::prelude::*;

mod handlers;

struct BotService {
    bot: Bot,
    #[allow(dead_code)]
    pool: PgPool,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for BotService {
    async fn bind(self, _addr: SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let handler = dptree::entry()
            .branch(Update::filter_message().endpoint(handlers::message_handler));

        Dispatcher::builder(self.bot, handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;

        Ok(())
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> Result<BotService, shuttle_runtime::Error> {
    let token = secrets
        .get("TELOXIDE_TOKEN")
        .expect("TELOXIDE_TOKEN not found in Secrets.toml");

    tracing::info!("Starting AI Todolist bot...");

    let bot = Bot::new(token);

    Ok(BotService { bot, pool })
}
