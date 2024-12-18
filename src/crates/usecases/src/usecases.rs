/// Usecases are the main business logic of the application.
///
/// This usecases module contains all the possible actions that the user can perform from client. 
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Usecases {
    TurnOffMusic,
    TurnOnMusic,
    GetMusicStatus,
    PlayNextTrack,
    PlayPrevTrack,

    StartBasicSystemMonitoring,
}

// if new usecases with some params will be added, they should be added as example to the `Requests` enum in `requests.rs`
