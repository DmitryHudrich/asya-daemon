pub enum Lexicon {
    Help,
    MusicStopped,
    MusicPause,
    MusicResume,
    ExecuteSuccess,
    ExecuteError,
    Error,
    Unauthorized,
    KostinError,
}

impl Lexicon {
    pub fn describe(&self) -> &'static str {
        match self {
            Lexicon::Help  => "<b>Доступные команды</b>:\n<code>/help</code>  —  справка\n<code>/do [descriptor]</code>  —  выполнить команду\n<code>/execute</code>  —  выполнить команду в терминале",
            Lexicon::MusicStopped => "Музыка не играет",
            Lexicon::MusicPause => "Остановила музыку",
            Lexicon::MusicResume => "Включила музыку",
            Lexicon::ExecuteSuccess => "Выполнила команду!",
            Lexicon::ExecuteError => "Не удалось выполнить команду!",
            Lexicon::Error => "Произошла ошибка!",
            Lexicon::Unauthorized => "Это не твой компьютер. Не могу тебе помочь, ковбой!",
            Lexicon::KostinError => "Что-то пошло не так...\n=> Пропишите /help",
        }
    }
}
