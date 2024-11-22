use log::info;
use shared::configuration;

pub fn run_telegram() {
    let bot_token = configuration::get().telegram().token();
    info!("Telegram token obtained successfully.")
}