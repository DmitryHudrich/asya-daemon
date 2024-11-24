use async_trait::async_trait;
use features::{
    services::commands::music::{self, MediaPlayingStatus},
    workers::{self, Observer},
};
use shared::utils::shell_utils;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::{Requester, *},
    types::{Message, ParseMode},
    utils::command::BotCommands,
    Bot,
};
use tokio::sync::OnceCell;

pub mod prerun;

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
    sub_to_getactionworker(&msg, &bot).await;
    let username = msg.chat.username().unwrap();
    let accepted_users = shared::state::get_tg_accepted_users()
        .await
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
