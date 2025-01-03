use log::*;
use macros::Stringify;
use serde::{Deserialize, Serialize};

use crate::scenarios::*;

/// Usecases are the main business logic of the application.
///
/// This usecases module contains all the possible actions that the user can perform from client.
#[derive(Debug, Stringify, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Usecases {
    TurnOffMusic,
    TurnOnMusic,
    GetMusicStatus,
    PlayNextTrack,
    PlayPrevTrack,

    #[serde(rename_all = "camelCase")]
    Open {
        app_kind: AppKind,
    },

    StartBasicSystemMonitoring,
}

#[derive(Serialize, Stringify, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AppKind {
    Terminal,
    Browser,
    Steam,
    Discord,
    Telegram,
    Specific(App),
}

#[derive(Serialize, Stringify, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum App {
    Tui(String),
    Gui(String),
}

impl Usecases {
    pub async fn execute(self, userinput: String) {
        let command = self;
        debug!("Dispatching command: {:?}", command);
        match command {
            Usecases::TurnOffMusic | Usecases::TurnOnMusic => {
                music_control::play_or_resume_music(userinput).await;
            }
            Usecases::GetMusicStatus => {
                music_control::get_music_status(userinput).await;
            }
            Usecases::PlayNextTrack => music_control::play_next_track(userinput).await,
            Usecases::PlayPrevTrack => music_control::play_previous_track(userinput).await,
            Usecases::StartBasicSystemMonitoring => {
                system_monitoring::start_basic_monitoring(userinput).await
            }
            Usecases::Open { app_kind } => open::open(app_kind).await,
        }
    }
}

// if new usecases with some params will be added, they should be added as example to the `Requests` enum in `requests.rs`
