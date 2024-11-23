use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MusicAction {
    PlayPause,
    GetStatus,
}

#[derive(Serialize, Deserialize)]
pub enum Requests {
    Music { action: MusicAction },
    Empty,
}
