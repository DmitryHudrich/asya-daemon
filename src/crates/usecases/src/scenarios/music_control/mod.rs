use crate::tools::PromptBuilder;
use crate::AsyaResponse;
use services::{
    lexicon::Lexicon,
    services::commands::music::{self, MediaPlayingStatus},
};
use shared::traits::Beautify;

// for future use

// #[derive(Debug)]
// pub enum MusicControlEvent {
//     MusicPaused { song_name: String },
//     MusicResumed { song_name: String },
// }

pub async fn play_or_resume_music(executed_command: String) {
    let music_status = music::get_status();
    music::play_pause();
    match music_status {
        MediaPlayingStatus::Stopped => (),
        MediaPlayingStatus::Paused(_) => {
            let res = PromptBuilder::new()
                .set_path("/telegram/music/resume")
                .set_variable("{command}", executed_command.as_str())
                .set_fallback_phrase(Lexicon::MusicResume)
                .get_result()
                .await;

            crate::publish(AsyaResponse::Ok {
                message: res.to_string(),
            }).await;
        }
        MediaPlayingStatus::Playing(_) => {
            let res = PromptBuilder::new()
                .set_path("/telegram/music/pause")
                .set_variable("{command}", executed_command.as_str())
                .set_fallback_phrase(Lexicon::MusicPause)
                .get_result()
                .await;

            crate::publish(AsyaResponse::Ok {
                message: res.to_string(),
            }).await;
        }
        MediaPlayingStatus::Unknown => (),
    };
}

pub async fn get_music_status(userinput: String) {
    let music_status = music::get_status();
    match music_status {
        // MediaPlayingStatus::Stopped => Lexicon::MusicStopped.describe().to_string(),
        MediaPlayingStatus::Stopped => (),
        MediaPlayingStatus::Paused(status) => {
            publish_music_status(status, &userinput).await;
        }
        MediaPlayingStatus::Playing(status) => {
            publish_music_status(status, &userinput).await;
        }
        // MediaPlayingStatus::Unknown => Lexicon::MusicStopped.describe().to_string(),
        MediaPlayingStatus::Unknown => (),
    };
}

async fn publish_music_status(status: music::TrackInfo, userinput: &str) {
    let res = PromptBuilder::new()
        .set_path("/telegram/music/status")
        .set_variable("{status}", status.beautiful_out().as_str())
        .set_variable("{message}", userinput)
        .set_fallback_phrase(Lexicon::ExecuteSuccess)
        .get_result()
        .await;
    crate::publish(AsyaResponse::Ok {
        message: res.to_string(),
    })
    .await;
}

