use log::info;
use service::services::commands::music::{self, MediaPlayingStatus};
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
    #[command(description = "control music. u can:\npause / resume: __p__")]
    Music(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let accepted_users = configuration::get().telegram().accepted_users();
    let username = msg.chat.username().unwrap();
    if !accepted_users.contains(&username.to_owned()) {
        bot.send_message(msg.chat.id, "This is not your pc, go away.").await?;
        Ok(())
    } else {
        match cmd {
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Command::Music(command) => {
                let response = dispatch_music_command(command);
                bot.send_message(msg.chat.id, response).await?;
            }
        };

        Ok(())
    }
}

fn dispatch_music_command(command: String) -> String {
    match command.as_str() {
        "p" => {
            let music_status = music::get_status();
            music::play_pause();
            let res = match music_status {
                MediaPlayingStatus::Stopped => "music is not playing",
                MediaPlayingStatus::Paused => "resumed",
                MediaPlayingStatus::Playing => "stopped",
                MediaPlayingStatus::Unknown => "music is not playing",
            };
            res.to_owned()
        }
        _ => todo!(),
    }
}
