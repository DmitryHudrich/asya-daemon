use playerctl::{self, PlayerCtl};

#[cfg(target_family = "unix")]
pub fn pause_track() {
    PlayerCtl::pause();
}

#[cfg(target_family = "windows")]
pub fn pause_track() {
    todo!()
}

