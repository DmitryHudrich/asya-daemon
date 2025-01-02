use std::{env, process::Command};

use shared::{configuration::CONFIG, event_system};

use crate::{usecases::App, AsyaResponse};

pub async fn open(app_name: App) {
    match app_name {
        App::Terminal => open_terminal().await,
        _ => todo!(),
    };
}

// TODO: чтобы для винды сделать открытие вначале команды надо просtо добавить cmd
async fn open_terminal() {
    let terminals = if CONFIG.open.terminal.is_empty() {
        vec![
            "kitty",
            "foot",
            "gnome-terminal",
            "urxvt",
            "xterm",
            "alacritty",
            "powershell",
        ]
    } else {
        vec![CONFIG.open.terminal.as_str()]
    };
    let mut spawned_term = None;
    for term in terminals {
        let res = Command::new(term)
            .current_dir(env::var("HOME").unwrap())
            .spawn();
        if res.is_ok() {
            spawned_term = Some(term);
            break;
        }
    }

    // FIX: хардкоженные ответы, сделать промпт
    match spawned_term {
        None => {
            event_system::publish(AsyaResponse::Ok {
                message: format!("Could not open terminal {}!", CONFIG.open.terminal),
            })
            .await
        }
        Some(term) => {
            event_system::publish(AsyaResponse::Ok {
                message: format!("Spawned {}", term),
            })
            .await
        }
    }
}
