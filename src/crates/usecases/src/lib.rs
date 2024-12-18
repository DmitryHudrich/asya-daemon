use crate::scenarios::music_control;
use crate::usecases::Usecases;
use log::*;
use scenarios::system_monitoring;

pub mod scenarios;
mod tools;
pub mod usecases;

pub async fn dispatch_usecase(command: String, userinput: String) {
    debug!("Dispatching command: {:?}", command);
    let usecase = command.parse::<Usecases>();
    match usecase {
        Ok(usecase) => match usecase {
            Usecases::TurnOffMusic | Usecases::TurnOnMusic => {
                music_control::play_or_resume_music(command).await;
            }
            Usecases::GetMusicStatus => {
                music_control::get_music_status(userinput).await;
            }
            Usecases::PlayNextTrack => music_control::play_next_track(userinput).await,
            Usecases::PlayPrevTrack => music_control::play_previous_track(userinput).await,
            Usecases::StartBasicSystemMonitoring => {
                system_monitoring::start_basic_monitoring(userinput).await
            } // todo. я хочу сделать так, чтобы можно было передавать параметры в сценарии
        },

        Err(err) => warn!("Error parsing usecase: {:?}", err),
        // _ => Lexicon::Error.describe().to_string(),
    }
}

// general purpose events

/// General response event. Should be used to send only responses to the user.
#[derive(Debug, parse_display::Display)]
pub enum AsyaResponse {
    #[display("{message}")]
    Ok { message: String },
}
