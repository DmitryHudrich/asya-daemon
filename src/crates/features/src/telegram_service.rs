use log::info;
use service::services::commands::music;
use shared::configuration;
use teloxide::{
    prelude::{Requester, *},
    types::Message,
    utils::command::BotCommands,
    Bot,
};

pub async fn run_telegram() {
    let bot_token = configuration::get().telegram().token();
    info!("Telegram token obtained successfully.");
    let bot = Bot::new(bot_token);

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "pause music")]
    MusicPause,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::MusicPause => {
            music::pause_track();
            bot.send_message(msg.chat.id, "track paused").await?;
        }
    };

    Ok(())
}
