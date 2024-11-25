use log::info;
use log::warn;
use teloxide::dispatching::UpdateFilterExt;
use teloxide::dptree;
use teloxide::prelude::Dispatcher;
use teloxide::types::Update;
use teloxide::Bot;

pub async fn run_telegram() {
    let bot_token_opt = shared::state::get_tgtoken().await;
    let accepted_users = match get_users_safely().await {
        Some(value) => value,
        None => return,
    };
    info!("Users who can use your bot: {:?}", accepted_users);
    check_token_and_launch(bot_token_opt).await;
}

pub(crate) async fn check_token_and_launch(bot_token_opt: Option<String>) {
    match bot_token_opt {
        Some(token) => {
            info!("Telegram token obtained successfully.");
            let bot = Bot::new(token);
            let handler =
                Update::filter_message().branch(dptree::entry().endpoint(super::security_check));
            Dispatcher::builder(bot, handler)
                .enable_ctrlc_handler()
                .build()
                .dispatch()
                .await;
            // Command::repl(bot, answer).await;
        }
        None => {
            warn!("Token not found. Skip telegram bot launch.");
        }
    }
}

pub(crate) async fn get_users_safely() -> Option<Vec<String>> {
    let accepted_users = shared::state::get_tg_accepted_users().await;
    match accepted_users {
        Some(users) => match users.is_empty() {
            true => no_auth_users(),
            false => Some(users),
        },
        None => no_auth_users(),
    }
}

pub(crate) fn no_auth_users() -> Option<Vec<String>> {
    warn!("Authorized users not specified. No one can use your bot.");
    None
}
