use std::collections::HashMap;

pub fn get_lexicon(key: &str) -> &str {
    let lexicon: HashMap<&'static str, &'static str> = HashMap::from([
        ("help", "<b>Доступные команды</b>:\n<code>/help</code>  —  справка\n<code>/music [ pause | resume | status ]</code>  —  управление музыкой\n<code>/execute</code>  —  выполнить команду в терминале"),
        ("music_stopped", "Музыка не играет"),
        ("music_pause", "Остановила музыку"),
        ("music_resume", "Включила музыку"),
        ("execute_success", "Выполнила команду!"),
        ("execute_error", "Не удалось выполнить команду!"),
        ("error", "Произошла ошибка!"),
        ("unauthorized", "Это не твой компьютер. Не могу тебе помочь, ковбой!"),
        ("kostin_error", "Что-то пошло не так...\n=> Пропишите /help"),
    ]);

    lexicon[key]
}
