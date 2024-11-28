use std::collections::HashMap;

pub fn get_lexicon() -> HashMap<String, String> {
    let lexicon: HashMap<String, String> = HashMap::from([
        ("help".into(), "<b>Доступные команды</b>:\n<code>/help</code>  —  справка\n<code>/music [ pause | resume | status ]</code>  —  управление музыкой\n<code>/execute</code>  —  выполнить команду в терминале".into()),
        ("music_stopped".into(), "Музыка не играет".into()),
        ("music_pause".into(), "Остановила музыку".into()),
        ("music_resume".into(), "Включила музыку".into()),
        ("execute_success".into(), "Выполнила команду!".into()),
        ("execute_error".into(), "Не удалось выполнить команду!".into()),
        ("error".into(), "Произошла ошибка!".into()),
        ("unauthorized".into(), "Это не твой компьютер. Не могу тебе помочь, ковбой!".into()),
        ("kostin_error".into(), "Что-то пошло не так...\n=> Пропишите /help".into()),
    ]);
        
    lexicon
}
