use std::sync::Arc;
use log::{debug, info, warn};
use services::lexicon::Lexicon;
use shared::{event_system, 
    state::get_tg_accepted_users}
;
use teloxide::{
    dispatching::dialogue::GetChatId,
    payloads::SendMessageSetters,
    prelude::{Requester, *},
    types::{Message, ParseMode},
    utils::command::BotCommands,
    Bot,
};
use tokio::task;
use usecases::AsyaResponse;

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "snake_case",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "control music. examples: \n\t/music pause\n\t/music resume")]
    Do(String),
    #[command(description = "execute shell command")]
    Execute,
}

pub(crate) async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        handle_command(text, bot, &msg).await?;
    }

    Ok(())
}

pub(crate) async fn check_user_authority(bot: Bot, msg: Message) -> bool {
    let chat_id = msg.chat_id().expect("The chat ID should be available");
    let authorized_user = msg
        .chat
        .username()
        .map(ToString::to_string)
        .unwrap_or(chat_id.to_string());

    let accepted_users =
        get_tg_accepted_users().expect("There should be at least one accepted user");

    let is_authorized_user = accepted_users.contains(&authorized_user);

    if !is_authorized_user {
        warn!("User without access: {}", authorized_user);
        bot.send_message(chat_id, Lexicon::Unauthorized.describe())
            .await
            .expect("The bot should be able to send message to user");
    }

    is_authorized_user
}

async fn handle_command(text: &str, bot: Bot, msg: &Message) -> Result<(), teloxide::RequestError> {
    info!("Received message: {}", text);
    // here should be the logic to handle the command with ai

    if text.starts_with('/') {
        let bot_username = bot.get_me().await?.username().to_owned();
        let cmd = Command::parse(text, &bot_username).unwrap();
        dispatch(cmd, &bot, msg).await?;
    } else {
        bot.send_message(msg.chat.id, Lexicon::Help.describe())
            .parse_mode(ParseMode::Html)
            .await?;
    };

    Ok(())
}

async fn dispatch(cmd: Command, bot: &Bot, msg: &Message) -> Result<(), teloxide::RequestError> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Lexicon::Help.describe())
                .parse_mode(ParseMode::Html)
                .await?;
            return Ok(());
        }
        Command::Do(string_cmd) => {
            let bot_clone = bot.clone();
            let chat_id = msg.chat.id;
            event_system::subscribe_once(move |event: Arc<AsyaResponse>| {
                let bot_clone = bot_clone.clone();
                task::spawn(async move {
                    debug!("Received event: {:?}", event);
                    let AsyaResponse::Ok { message, .. } = event.as_ref();
                    bot_clone
                        .send_message(chat_id, message.to_string())
                        .parse_mode(ParseMode::Html)
                        .await
                        .unwrap();
                })
            })
            .await;
            usecases::dispatch_usecase(string_cmd, msg.text().unwrap().to_string()).await;
        }
        _ => (),
    }
    Ok(())
}
