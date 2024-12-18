use crate::scenarios::music_control;
use crate::usecases::Usecases;
use log::*;
use scenarios::system_monitoring;

pub mod scenarios;
pub mod shared_workers;
mod tools;
pub mod usecases;

/// Dispatches the usecase to the appropriate scenario.
pub async fn dispatch_usecase(command: Usecases, userinput: String) {
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
    }
}

// general purpose events

/// General response event. Use it to send responses to the client.
/// How event works see [`shared::event_system`].
#[derive(Debug, parse_display::Display)]
pub enum AsyaResponse {
    /// Success response with message from Asya.
    ///
    /// # Arguments
    ///     * `message` - human readable message from Asya, e.g.
    ///         "I've turned off the music. Don't listen this shit anymore."
    ///
    /// # Example
    ///
    /// ```
    /// event_system::publish(AsyaResponse::Ok {
    ///     message: "Hi, Vitaliy! I heard that u like thinkpads? Me too!"
    /// }
    ///
    /// ```
    #[display("{message}")]
    Ok { message: String },
}
