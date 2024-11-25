use features::{
    llm_api,
    services::commands::music::{self, MediaPlayingStatus},
};
use shared::utils::llm_utils;
use teloxide::types::Message;

pub async fn dispatch_music_command(command: String, msg: &Message) -> String {
    match command.as_str() {
        "pause" | "resume" => {
            let music_status = music::get_status();
            music::play_pause();
            match music_status {
                MediaPlayingStatus::Stopped => "music is not playing".to_string(),
                MediaPlayingStatus::Paused(_) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/resume");
                    let formatted_prompt = prompt.replace("{command}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or("play".to_owned())
                }
                MediaPlayingStatus::Playing(_) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/pause");
                    let formatted_prompt = prompt.replace("{command}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or("paused".to_owned())
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
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or(format!("{:?}", status))
                    // todo: beautify
                    // music output
                }
                MediaPlayingStatus::Playing(status) => {
                    let prompt = llm_utils::get_prompt("/telegram/music/status");
                    let formatted_prompt = prompt
                        .replace("{status}", format!("{:?}", status).as_str())
                        .replace("{message}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or(format!("{:?}", status))
                    // todo: beautify
                    // music output
                }
                MediaPlayingStatus::Unknown => "music is not playing".to_string(),
            }
        }
        _ => "не понял".to_string(),
    }
}
