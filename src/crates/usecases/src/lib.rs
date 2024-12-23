use crate::scenarios::music_control;
use crate::usecases::Usecases;
use log::*;
use scenarios::system_monitoring;
use services::llm_api;

pub mod scenarios;
pub mod shared_workers;
mod tools;
pub mod usecases;

fn process_response(llm_response: &str) -> Result<Usecases, Box<dyn std::error::Error>> {
    let temp_value = serde_json::from_str::<serde_json::Value>(llm_response)?;
    let command = temp_value.pointer("/Command").ok_or("Command not found")?;
    let usecase = serde_json::from_value::<Usecases>(command.clone())?;
    Ok(usecase)
}

pub async fn dispatch_by_user_message(message: String) {
    // should be autogenerated.
    let stringified_usecases = r#"
                    turnOffMusic,
                    turnOnMusic,
                    getMusicStatus,
                    playNextTrack,
                    playPrevTrack,

                    startBasicSystemMonitoring,
            "#;

    let llm_response = llm_api::send_request(format!(
        "generate json. your json will be used for parsing, so don't use any markdown.
                use json templates: 
                ```
                    {{ \"Command\": \"turnOffMusic\" }}
                ```
                Generate for the following query:
                Here is commands which are available: {}. 
                which of these commands is this message most similar to {}? 
                SEND ME ONLY COMMAND AND POSSIBLE PARAMETERS",
        stringified_usecases, message
    ))
    .await;

    if llm_response.is_err() {
        warn!("Error sending request to LLM: {:?}", llm_response.err());
        return;
    }
    let llm_response = llm_response.unwrap();

    let usecase = process_response(&llm_response);
    if let Err(err) = usecase {
        warn!("Error parsing response from LLM: {:?}", err);
        return;
    }
    let usecase = usecase.unwrap();
    dispatch_usecase(usecase, message).await;
}

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
