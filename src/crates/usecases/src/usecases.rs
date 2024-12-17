#[derive(Debug, parse_display::FromStr)]
#[display(style = "snake_case")]
pub enum Usecases {
    TurnOffMusic,
    TurnOnMusic,
    GetMusicStatus,
    // MusicPrevious,
}


