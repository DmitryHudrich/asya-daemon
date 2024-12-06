use features::{
    lexicon::Lexicon, llm_api, services::commands::music::{self, MediaPlayingStatus}
};
use log::info;
use shared::{llm, traits::Beautify};
use teloxide::types::Message;

pub async fn dispatch_music_command(command: String, msg: &Message) -> String {
    match command.as_str() {
        "pause" | "resume" => {
            let music_status = music::get_status();
            music::play_pause();
            match music_status {
                MediaPlayingStatus::Stopped => Lexicon::MusicStopped.describe().to_string(),
                MediaPlayingStatus::Paused(_) => {
                    let prompt = llm::get_prompt("/telegram/music/resume");
                    let formatted_prompt = prompt.replace("{command}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or(Lexicon::MusicResume.describe().to_string())
                }
                MediaPlayingStatus::Playing(_) => {
                    let prompt = llm::get_prompt("/telegram/music/pause");
                    let formatted_prompt = prompt.replace("{command}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or(Lexicon::MusicPause.describe().to_string())
                }
                MediaPlayingStatus::Unknown => Lexicon::MusicStopped.describe().to_string(),
            }
        }
        "status" => {
            let music_status = music::get_status();
            match music_status {
                MediaPlayingStatus::Stopped => Lexicon::MusicStopped.describe().to_string(),
                MediaPlayingStatus::Paused(status) => {
                    let prompt = llm::get_prompt("/telegram/music/status");
                    let formatted_prompt = prompt
                        .replace("{status}", format!("{}", status).as_str())
                        .replace("{message}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or(status.beautiful_out())
                    // todo: beautify
                    // music output
                }
                MediaPlayingStatus::Playing(status) => {
                    let prompt = llm::get_prompt("/telegram/music/status");
                    let formatted_prompt = prompt
                        .replace("{status}", format!("{}", status).as_str())
                        .replace("{message}", msg.text().unwrap());
                    let response = llm_api::send_request(formatted_prompt).await;
                    response.unwrap_or(status.beautiful_out())
                    // todo: beautify
                    // music output
                }
                MediaPlayingStatus::Unknown => Lexicon::MusicStopped.describe().to_string(),
            }
        }
        _ => Lexicon::Error.describe().to_string(),
    }
}
