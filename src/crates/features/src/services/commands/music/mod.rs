use shared::utils::shell_utils;

#[derive(Debug)]
pub struct TrackInfo {
    title: String,
    artist: String,
    album: String,
    // platform: String,
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
    //playerctl metadata --format "{{ artist }}{{ album }}{{ title }}"
    let status_opt = shell_utils::execute_command(vec!["playerctl", "status"]);
    let metadata = shell_utils::execute_command(vec![
        "playerctl",
        "metadata",
        "--format",
        "{{ artist }};{{ album }};{{ title }}",
    ])
    .expect("Error while unwrapping metadata from current track.");
    let splited_metadata = Vec::from_iter(metadata.split_terminator(';'))
        .iter_mut()
        .map(|el| el.to_string())
        .collect::<Vec<String>>();
    let track_info = TrackInfo {
        title: splited_metadata[0].clone(),
        artist: splited_metadata[1].clone(),
        album: splited_metadata[2].clone(),
    };
    println!("{:?}", track_info);
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
