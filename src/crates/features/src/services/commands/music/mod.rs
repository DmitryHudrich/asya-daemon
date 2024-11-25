use shared::utils::shell_utils;

#[derive(Debug)]
pub struct TrackInfo {
    title: String,
    artist: String,
    album: String,
}

#[derive(Debug)]
pub enum MediaPlayingStatus {
    /// Media is currently playing.
    Playing(TrackInfo),
    /// Media is currently paused.
    Paused(TrackInfo),
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
    use regex::Regex;

    let status_opt = shell_utils::execute_command(vec!["playerctl", "status"]);
    let metadata = shell_utils::execute_command(vec!["playerctl", "metadata"]);
    let re = Regex::new(r"(?m)YoutubeMusic xesam:(title|album|artist)\s+(.*)").unwrap();

    let mut results = std::collections::HashMap::new();

    for cap in re.captures_iter(metadata.unwrap().as_str()) {
        let key = cap[1].to_string();
        let value = cap[2].trim().to_string();
        results.insert(key, value);
    }
    let track_info = TrackInfo {
        title: results["title"].clone(),
        artist: results["artist"].clone(),
        album: results["album"].clone(),
    };
    match status_opt {
        Some(status) => match status.as_str().trim() {
            "Playing" => MediaPlayingStatus::Playing(track_info),
            "Paused" => MediaPlayingStatus::Paused(track_info),
            "Stopped" => MediaPlayingStatus::Stopped,
            _ => MediaPlayingStatus::Unknown,
        },
        None => todo!(),
    }
}

#[cfg(target_family = "windows")]
pub fn play_pause() {
    todo!()
}
