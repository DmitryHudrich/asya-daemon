use shared::utils::shell_utils;

pub enum MediaPlayingStatus {
    /// Media is currently playing.
    Playing,
    /// Media is currently paused.
    Paused,
    /// Media is currently stopped.
    Stopped,
    /// Maybe media is currently destroyed.
    Unknown,
}

#[cfg(target_family = "unix")]
pub fn play_pause() {
    shell_utils::execute_command(vec!["playerctl", "play-pause"]);
}

#[cfg(target_family = "unix")]
pub fn get_status() -> MediaPlayingStatus {
    let status_opt = shell_utils::execute_command(vec!["playerctl", "status"]);
    match status_opt {
        Some(status) => match status.as_str().trim() {
            "Playing" => MediaPlayingStatus::Playing,
            "Paused" => MediaPlayingStatus::Paused,
            "Stopped" => MediaPlayingStatus::Stopped,
            _ => MediaPlayingStatus::Unknown,
        },
        None => todo!()
    }
}

#[cfg(target_family = "windows")]
pub fn play_pause() {
    todo!()
}
