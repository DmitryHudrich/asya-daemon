use features::{
    llm_api,
    services::commands::music::{self, MediaPlayingStatus},
};
use shared::{traits::Beautify, utils::llm_utils};
use teloxide::types::Message;
use crate::lexicon::get_lexicon;

pub async fn dispatch_music_command(command: String, msg: &Message) -> String {
    // FIXME: this is probably bad
    // maybe static variable will be better
    // but i huy znaet how to do it
    let lexicon = get_lexicon();
    match command.as_str() {
        "pause" | "resume" => {
            let music_status = music::get_status();
            music::play_pause();
            match music_status {
                MediaPlayingStatus::Stopped => lexicon["music_stopped"].clone(),
                MediaPlayingStatus::Paused(_) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/resume");
                    let formatted_prompt = prompt.replace("{command}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or(lexicon["music_resume"].clone())
                }
                MediaPlayingStatus::Playing(_) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/pause");
                    let formatted_prompt = prompt.replace("{command}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or(lexicon["music_pause"].clone())
                }
                MediaPlayingStatus::Unknown => lexicon["music_stopped"].clone(),
            }
        }
        "status" => {
            let music_status = music::get_status();
            match music_status {
                MediaPlayingStatus::Stopped => lexicon["music_stopped"].clone(),
                MediaPlayingStatus::Paused(status) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/status");
                    let formatted_prompt = prompt
                        .replace("{status}", format!("{:?}", status).as_str())
                        .replace("{message}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or(status.beautiful_out())
                    // todo: beautify
                    // music output
                }
                MediaPlayingStatus::Playing(status) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/status");
                    let formatted_prompt = prompt
                        .replace("{status}", format!("{:?}", status).as_str())
                        .replace("{message}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or(status.beautiful_out())
                    // todo: beautify
                    // music output
                }
                MediaPlayingStatus::Unknown => lexicon["music_stopped"].clone(),
            }
        }
        _ => lexicon["error"].clone(),
    }
}
