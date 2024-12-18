use crate::scenarios::music_control;
use log::*;
use crate::usecases::Usecases;

pub mod scenarios;
pub mod workers;
pub mod usecases;
mod tools;


pub fn run_backgorund_workers() {
    tokio::spawn(workers::action_worker::run());
}

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
            Usecases::PlayPreviousTrack => todo!(),
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
