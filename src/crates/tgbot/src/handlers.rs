use core::panic;
use std::sync::Arc;

use async_trait::async_trait;
use log::{info, warn};
use services::{
    lexicon::Lexicon,
    llm_api::{self},
    workers::Observer,
};
use shared::{
    llm, shell,
    state::{self, get_tg_accepted_users},
    types::AiRecognizeMethod,
};
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
    Music(String),
    #[command(description = "execute shell command")]
    Execute(String),
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
    let slash_command = if state::is_llm_obtained() && !text.starts_with('/') {
        recognize_command_with_llm(text.to_string()).await
    } else {
        text.to_string()
    };
    info!("Recognized command: {}", slash_command);

    if slash_command.starts_with('/') {
        // let bot_username = bot.get_me().await?.username().to_owned();
        // let cmd = Command::parse(&slash_command, &bot_username).unwrap();
        dispatch(text.to_string(), &bot, msg).await?;
    } else {
        bot.send_message(msg.chat.id, Lexicon::Help.describe())
            .parse_mode(ParseMode::Html)
            .await?;
    };

    Ok(())
}

async fn dispatch(cmd: String, bot: &Bot, msg: &Message) -> Result<(), teloxide::RequestError> {
    usecases::dispatch_usecase(cmd, msg.text().unwrap().to_string()).await;
    let bot_clone = bot.clone();
    let chat_id = msg.chat.id;
    usecases::subscribe(move |event: Arc<AsyaResponse>| {
        let bot_clone = bot_clone.clone();
        task::spawn(async move {
            bot_clone.send_message(chat_id, format!("{:?}", event))
                .parse_mode(ParseMode::Html)
                .await
                .unwrap();
        })
    })
    .await;
    Ok(())
}

async fn recognize_command_with_llm(msg: String) -> String {
    let prompt = match state::get_ai_recognize_method().unwrap() {
        AiRecognizeMethod::Groq => format_for_groq(msg),
        AiRecognizeMethod::AltaS => msg,
        AiRecognizeMethod::None => panic!("Ai recognizing with unspecified model"),
    };

    llm_api::send_request(prompt)
        .await
        .expect("The LLM should recognize arbitrary command")
}

fn format_for_groq(msg: String) -> String {
    const COMMANDS: &str = r#"
        /music resume, 
        /music pause
        /music status",
    "#;

    let prompt = llm::get_prompt("/telegram/recognize_command");

    prompt
        .replace("{commands}", COMMANDS)
        .replace("{message}", &msg)
}

// TODO: use it in future
// async fn sub_to_getactionworker(msg: &Message, bot: &Bot) {
//     static INIT: OnceCell<()> = OnceCell::const_new();
//     let worker = workers::get_actionworker().await;
//     let observer = Box::new(PrintObserver {
//         chatid: msg.chat.id,
//         bot: bot.clone(), // todo: fix cloning
//     });
//     INIT.get_or_init(|| async {
//         worker.subscribe(observer).await;
//     })
//     .await;
// }

pub struct PrintObserver {
    chat_id: ChatId,
    bot: Bot,
}

#[async_trait]
impl Observer<String> for PrintObserver {
    async fn update(&self, phrase: &String) {
        self.bot.send_message(self.chat_id, phrase).await.unwrap();
    }
}
