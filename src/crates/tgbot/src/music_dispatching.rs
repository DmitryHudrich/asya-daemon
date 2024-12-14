use std::sync::Arc;

use services::{
    llm_api,
    services::commands::music::{self, MediaPlayingStatus},
};
use shared::{llm, traits::Beautify};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Message, ParseMode},
    Bot,
};
use tokio::task;
use usecases::AsyaResponse;

pub async fn dispatch_music_command(command: String, msg: Message, bot: Bot) {
    // usecases::music_control::dispatch_usecase(command, msg.text().unwrap().to_string()).await;
    // usecases::subscribe(move |event: Arc<AsyaResponse>| {
    //     let bot = bot.clone();
    //     let msg = msg.clone();
    //     task::spawn(async move {
    //         bot.send_message(msg.chat.id, format!("{:?}", event))
    //             .parse_mode(ParseMode::Html)
    //             .await
    //             .unwrap();
    //     })
    // })
    // .await;
}
