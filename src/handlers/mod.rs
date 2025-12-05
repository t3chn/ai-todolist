use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Show help")]
    Help,
}

pub async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let text = msg.text().unwrap_or_default();

    if let Ok(cmd) = Command::parse(text, "") {
        match cmd {
            Command::Start => {
                bot.send_message(
                    msg.chat.id,
                    "👋 Welcome to AI Todolist!\n\n\
                    I'm your smart task assistant. Just send me tasks in natural language:\n\n\
                    • \"Call mom tomorrow at 5pm\"\n\
                    • \"Buy groceries\"\n\
                    • \"Finish report by Friday\"\n\n\
                    Type /help for more commands.",
                )
                .await?;
            }
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
        }
    }

    Ok(())
}
