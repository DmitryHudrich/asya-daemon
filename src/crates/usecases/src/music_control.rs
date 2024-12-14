use crate::AsyaResponse;
use services::{
    lexicon::Lexicon,
    llm_api,
    services::commands::music::{self, MediaPlayingStatus},
};
use shared::llm;
use shared::traits::Beautify;

// for future use

// #[derive(Debug)]
// pub enum MusicControlEvent {
//     MusicPaused { song_name: String },
//     MusicResumed { song_name: String },
// }

#[derive(Debug, parse_display::FromStr)]
#[display(style = "snake_case")]
pub enum Usecases {
    TurnOffMusic,
    TurnOnMusic,
    GetMusicStatus,
    // MusicPrevious,
}

pub async fn play_or_resume_music(executed_command: String, userinput: String) {
    let music_status = music::get_status();
    music::play_pause();
    match music_status {
        MediaPlayingStatus::Stopped => (),
        MediaPlayingStatus::Paused(_) => {
            let prompt = llm::get_prompt("/telegram/music/resume");
            let formatted_prompt = prompt.replace("{command}", executed_command.as_str());
            let response = llm_api::send_request(formatted_prompt).await;
            let res = response.unwrap_or(Lexicon::MusicResume.describe().to_string());
            crate::publish(AsyaResponse::Ok {
                message: res.to_string(),
            })
            .await;
        }
        MediaPlayingStatus::Playing(_) => {
            let prompt = llm::get_prompt("/telegram/music/pause");
            let formatted_prompt = prompt.replace("{command}", executed_command.as_str());
            let response = llm_api::send_request(formatted_prompt).await;
            let res = response.unwrap_or(Lexicon::MusicPause.describe().to_string());
            crate::publish(AsyaResponse::Ok {
                message: res.to_string(),
            })
            .await;
        }
        MediaPlayingStatus::Unknown => (),
    };
}

pub async fn get_music_status(executed_command: String, userinput: String) {
    let music_status = music::get_status();
    match music_status {
        // MediaPlayingStatus::Stopped => Lexicon::MusicStopped.describe().to_string(),
        MediaPlayingStatus::Stopped => (),
        MediaPlayingStatus::Paused(status) => {
            let prompt = llm::get_prompt("/telegram/music/status");
            let formatted_prompt = prompt
                .replace("{status}", format!("{}", status).as_str())
                .replace("{message}", userinput.as_str());
            let response = llm_api::send_request(formatted_prompt).await;
            let res = response.unwrap_or(status.beautiful_out());
            crate::publish(AsyaResponse::Ok {
                message: res.to_string(),
            })
            .await;

            // todo: beautify
            // music output
        }
        MediaPlayingStatus::Playing(status) => {
            let prompt = llm::get_prompt("/telegram/music/status");
            let formatted_prompt = prompt
                .replace("{status}", format!("{}", status).as_str())
                .replace("{message}", userinput.as_str());
            let response = llm_api::send_request(formatted_prompt).await;
            let res = response.unwrap_or(status.beautiful_out());
            crate::publish(AsyaResponse::Ok {
                message: res.to_string(),
            })
            .await;
            // todo: beautify
            // music output
        }
        // MediaPlayingStatus::Unknown => Lexicon::MusicStopped.describe().to_string(),
        MediaPlayingStatus::Unknown => (),
    };
}
