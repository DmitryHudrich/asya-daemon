use shared::configuration::CONFIG;
use teloxide::Bot;

mod handlers;

use log::info;
use log::warn;
use teloxide::dispatching::UpdateFilterExt;
use teloxide::prelude::Dispatcher;
use teloxide::types::Update;

pub async fn run_telegram_bot() {
    if CONFIG.telegram.token.is_empty() {
        warn!("The telegram token is empty! Skip bot launch.");
        return;
    }

    info!("Telegram token obtained successfully.");

    let Some(accepted_users) = get_users_safely() else {
        return;
    };

    info!("Users who can use your bot: {:?}", accepted_users);
    launch(CONFIG.telegram.token.clone()).await;
}

pub(crate) async fn launch(bot_token: String) {
    let bot = Bot::new(bot_token);
    let handler = Update::filter_message()
        .filter_async(handlers::check_user_authority)
        .endpoint(handlers::handle_message);

    Dispatcher::builder(bot, handler)
        .default_handler(|_update| async {
            // INFO: this is intentional because default handler always produces WARN outputs in
            // logs with any unhandled Update event. The unhandled Update event means that it have
            // not reached to any `.endpoint` function, so it falls into `default_handler`.
        })
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    // Command::repl(bot, answer).await;
}

pub(crate) fn get_users_safely() -> Option<&'static Vec<String>> {
    fn no_auth_users() -> Option<&'static Vec<String>> {
        warn!("Authorized users not specified. No one can use your bot.");
        None
    }

    let accepted_users = &CONFIG.telegram.accepted_users;
    if accepted_users.is_empty() {
        no_auth_users()
    } else {
        Some(accepted_users)
    }
}
