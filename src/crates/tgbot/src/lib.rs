use async_trait::async_trait;
use features::{
    llm_api,
    services::commands::music::{self, MediaPlayingStatus},
    workers::{self, Observer},
};
use shared::{
    state::{self, get_tg_accepted_users},
    utils::shell_utils,
};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::{Requester, *},
    types::{Message, ParseMode},
    utils::command::BotCommands,
    Bot,
};
use tokio::sync::OnceCell;

pub mod prerun;

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
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

async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(username) = msg.chat.username() {
        // sub_to_getactionworker(&msg, &bot).await; // регулярные сообщения от аси
        let accepted_users = get_tg_accepted_users()
            .await
            .expect("Accepted users was checked, but empty.");

        if !accepted_users.contains(&username.to_owned()) {
            bot.send_message(msg.chat.id, "This is not your pc, go away.")
                .await?;
        } else if let Some(text) = msg.text() {
            let command = if state::get_mistral_token().await.is_some() {
                llm_api_response(text.to_string()).await
            } else {
                text.to_string()
            };

            if text.starts_with('/') || command.starts_with('/') {
                let cmd = Command::parse(&command, username).unwrap();
                handle_command(cmd, bot.clone(), msg.clone()).await?;
            } else {
                bot.send_message(msg.chat.id, command).await?;
            }
        }
    }
    Ok(())
}

async fn llm_api_response(msg: String) -> String {
    let req = format!("Determine which Telegram command from the list the user query is most similar to.
        Return only the name of the command without any explanations or extra text. Here is the list of commands: 
        {}

        Command for recognizing: {}.", 
        "/music resume, 
        /music pause", 
        msg);
    llm_api::send_request(req.clone()).await
}

async fn sub_to_getactionworker(msg: &Message, bot: &Bot) {
    static INIT: OnceCell<()> = OnceCell::const_new();
    let worker = workers::get_actionworker().await;
    let observer = Box::new(PrintObserver {
        chatid: msg.chat.id,
        bot: bot.clone(), // todo: fix cloning
    });
    INIT.get_or_init(|| async {
        worker.subscribe(observer).await;
    })
    .await;
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
            let _ = dispatch_music_command(command);
            let response = llm_api::send_request(format!(
                "Keep in mind that your name is Asya, you are a sweet, flirtatious, kind, always ready to help girl. 
                Answer moderately briefly. Be sure to respond in the language of the sent request please. Cutely answer as if you have executed this command: {}",
                msg.text().unwrap()
            ))
            .await;
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
        "pause" | "resume" => {
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

pub struct PrintObserver {
    chatid: ChatId,
    bot: Bot,
}

#[async_trait]
impl Observer<String> for PrintObserver {
    async fn update(&self, phrase: &String) {
        self.bot.send_message(self.chatid, phrase).await.unwrap();
    }
}
