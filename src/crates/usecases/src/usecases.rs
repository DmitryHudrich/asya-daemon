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


