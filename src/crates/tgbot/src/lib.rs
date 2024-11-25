use async_trait::async_trait;
use features::{
    llm_api,
    services::commands::music::{self, MediaPlayingStatus},
    workers::Observer,
};
use shared::{
    state::{self, get_tg_accepted_users},
    utils::{llm_utils, shell_utils},
};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::{Requester, *},
    types::{Message, ParseMode},
    utils::command::BotCommands,
    Bot,
};

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

async fn security_check(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(username) = msg.chat.username() {
        let username = username.to_string();
        // sub_to_getactionworker(&msg, &bot).await; // регулярные сообщения от аси
        let accepted_users = get_tg_accepted_users()
            .await
            .expect("Accepted users was checked, but empty.");

        if !accepted_users.contains(&username.to_owned()) {
            bot.send_message(msg.chat.id, "This is not your pc, go away.")
                .await?;
        } else if let Some(text) = msg.text() {
            handle_command(text, username, bot, &msg).await?;
        }
    }
    Ok(())
}

async fn handle_command(
    text: &str,
    username: String,
    bot: Bot,
    msg: &Message,
) -> Result<(), teloxide::RequestError> {
    let slash_command = if state::get_mistral_token().await.is_some() {
        recognize_command_with_llm(text.to_string()).await
    } else {
        text.to_string()
    };

    if text.starts_with('/') || slash_command.starts_with('/') {
        let cmd = Command::parse(&slash_command, username.as_str()).unwrap();
        dispatch(cmd, &bot, msg).await?;
    } else {
        bot.send_message(msg.chat.id, slash_command).await?;
    };

    Ok(())
}

async fn dispatch(cmd: Command, bot: &Bot, msg: &Message) -> Result<(), teloxide::RequestError> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Music(command) => {
            let response = dispatch_music_command(command, msg).await;
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

async fn recognize_command_with_llm(msg: String) -> String {
    let commands = r#"
        /music resume, 
        /music pause
        /music status",
    "#;
    let prompt = llm_utils::get_prompt("/telegram/recognize_command");
    let formatted_prompt = prompt
        .replace("{commands}", commands)
        .replace("{message}", msg.as_str());
    let response = llm_api::send_request(formatted_prompt);
    response.await
}

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

async fn dispatch_music_command(command: String, msg: &Message) -> String {
    match command.as_str() {
        "pause" | "resume" => {
            let music_status = music::get_status();
            music::play_pause();
            match music_status {
                MediaPlayingStatus::Stopped => "music is not playing".to_string(),
                MediaPlayingStatus::Paused(_) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/resume");
                    let formatted_prompt = prompt.replace("{command}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt);
                    response.await
                }

                MediaPlayingStatus::Playing(_) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/pause");
                    let formatted_prompt = prompt.replace("{command}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt);
                    response.await
                }
                MediaPlayingStatus::Unknown => "music is not playing".to_string(),
            }
        }
        "status" => {
            let music_status = music::get_status();
            match music_status {
                MediaPlayingStatus::Stopped => "music is not playing".to_string(),
                MediaPlayingStatus::Paused(status) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/status");
                    let formatted_prompt = prompt
                        .replace("{status}", format!("{:?}", status).as_str())
                        .replace("{message}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt);
                    response.await
                }

                MediaPlayingStatus::Playing(status) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/status");
                    let formatted_prompt = prompt
                        .replace("{status}", format!("{:?}", status).as_str())
                        .replace("{message}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt);
                    response.await
                }
                MediaPlayingStatus::Unknown => "music is not playing".to_string(),
            }
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
