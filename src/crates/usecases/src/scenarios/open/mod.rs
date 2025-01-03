use std::{env, process::Command};

use shared::{configuration::CONFIG, event_system};

use crate::{
    usecases::{App, AppKind, AppUI},
    AsyaResponse,
};

pub async fn open(kind: AppKind) {
    let fallback = match kind {
        AppKind::Terminal => vec![
            "kitty",
            "foot",
            "gnome-terminal",
            "urxvt",
            "xterm",
            "alacritty",
            "powershell",
        ],
        AppKind::Browser => vec!["firefox", "chromium", "yandex"],
        AppKind::Steam => vec!["steam"],
        AppKind::Discord => vec!["discord", "vesktop"],
        AppKind::Telegram => vec!["telegram-desktop"],
        AppKind::Specific(_) => vec![],
    };
    let res = if let AppKind::Specific(app) = kind.clone() {
        open_specific(app).await
    } else {
        open_generic(kind.clone(), fallback).await
    };
    match res {
        Ok(ok) => {
            event_system::publish(AsyaResponse::Ok {
                message: format!("I've opened {}.", ok.name),
            })
            .await
        }
        Err(err) => match err {
            OpenError::NotFound => {
                event_system::publish(AsyaResponse::Ok {
                    message: format!("Couldn't find {} on your pc.", kind),
                })
                .await
            }
            OpenError::Other { name, message } => {
                event_system::publish(AsyaResponse::Ok {
                    message: format!("Couldn't open {}: {}", name, message),
                })
                .await
            }
        },
    }
}

struct OpenOk {
    name: String,
}

enum OpenError {
    NotFound,
    Other { name: String, message: String },
}

async fn open_generic(app: AppKind, fallback: Vec<&str>) -> Result<OpenOk, OpenError> {
    let config = match app {
        AppKind::Terminal => CONFIG.open.terminal.as_str(),
        AppKind::Browser => CONFIG.open.browser.as_str(),
        _ => "",
    };
    let config = match config.is_empty() {
        true => None,
        false => Some(config),
    };

    if let Some(app) = config {
        let res = Command::new(app)
            .current_dir(env::var("HOME").unwrap())
            .spawn();
        match res {
            Ok(_) => Ok(OpenOk {
                name: app.to_string(),
            }),
            Err(err) => Err(OpenError::Other {
                name: app.to_string(),
                message: err.to_string(),
            }),
        }
    } else {
        for app in fallback {
            let res = Command::new(app)
                .current_dir(env::var("HOME").unwrap())
                .spawn();
            if res.is_ok() {
                return Ok(OpenOk {
                    name: app.to_string(),
                });
            }
        }
        Err(OpenError::NotFound)
    }
}

async fn open_gui(app: App) -> Result<OpenOk, OpenError> {
    let res = Command::new(app.command.clone())
        .current_dir(env::var("HOME").unwrap())
        .spawn();
    match res {
        Ok(_) => Ok(OpenOk { name: app.command }),
        Err(err) => Err(OpenError::Other {
            name: app.command,
            message: err.to_string(),
        }),
    }
}

async fn open_specific(app: App) -> Result<OpenOk, OpenError> {
    match app.ui {
        AppUI::G => open_gui(app).await,
        _ => todo!(), // FIXME: кажется я обосрался с архитектурой. чтобы туи приложу открыть надо сначала
                      // открыть терминал функцией open_generic(AppKind::Terminal),
                      // потом в нем уже то что надо. а чтобы в терминале чето открыть, надо
                      // ему аргументы передать, а функция такого не предполагает. кароче мне щас лень думать
                      // чето. c гуи приложухами все работает
    }
}
