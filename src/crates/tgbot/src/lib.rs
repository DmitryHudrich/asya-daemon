use log::{info, warn};
use features::services::commands::music::{self, MediaPlayingStatus};
use shared::{configuration, utils::shell_utils};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::{Requester, *},
    types::{Message, ParseMode},
    utils::command::BotCommands,
    Bot,
};

pub async fn run_telegram() {
    let bot_token_opt = configuration::get().telegram().token();
    let accepted_users = match get_users_safely() {
        Some(value) => value,
        None => return,
    };
    info!("Users who can use your bot: {:?}", accepted_users);
    check_token_and_launch(bot_token_opt).await;
}

async fn check_token_and_launch(bot_token_opt: Option<String>) {
    match bot_token_opt {
        Some(token) => {
            shared::ASYA_STATUS.write().await.tgtoken_obtained = true;
            info!("Telegram token obtained successfully.");
            let bot = Bot::new(token);
            Command::repl(bot, answer).await;
        }
        None => {
            warn!("Token not found. Skip telegram bot launch.");
        }
    }
}

fn get_users_safely() -> Option<Vec<String>> {
    let accepted_users = configuration::get().telegram().accepted_users();
    match accepted_users {
        Some(users) => match users.is_empty() {
            true => no_auth_users(),
            false => Some(users),
        },
        None => no_auth_users(),
    }
}

fn no_auth_users() -> Option<Vec<String>> {
    warn!("Authorized users not specified. No one can use your bot.");
    None
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
    #[command(description = "execute shell command")]
    Execute(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let username = msg.chat.username().unwrap();
    let accepted_users = configuration::get()
        .telegram()
        .accepted_users()
        .expect("Accepted users was checked, but empty.");
    if !accepted_users.contains(&username.to_owned()) {
        bot.send_message(msg.chat.id, "This is not your pc, go away.")
            .await?;
        Ok(())
    } else {
        handle_command(cmd, bot, msg).await?;
        Ok(())
    }
}

async fn handle_command(
    cmd: Command,
    bot: Bot,
    msg: Message,
) -> Result<(), teloxide::RequestError> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Music(command) => {
            let response = dispatch_music_command(command);
            bot.send_message(msg.chat.id, response).await?;
        }
        Command::Execute(command) => {
            let args = command.split_whitespace().collect();
            let response = format!("```\n{}\n```", shell_utils::execute_command(args).unwrap());
            bot.send_message(msg.chat.id, response)
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
        }
    };
    Ok(())
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
