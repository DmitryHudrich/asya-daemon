use crate::scenarios::music_control;
use log::*;
use crate::usecases::Usecases;
use shared::event_system::AsyncEventDispatcher;
use std::{any::Any, sync::Arc};
use tokio::{
    sync::OnceCell,
    task,
};

pub mod scenarios;
pub mod workers;
pub mod usecases;
mod tools;

static EVENT_DISPATCHER: OnceCell<Arc<AsyncEventDispatcher>> = OnceCell::const_new();

pub fn run_backgorund_workers() {
    tokio::spawn(workers::action_worker::run());
}

pub async fn dispatch_usecase(command: String, userinput: String) {
    debug!("Dispatching command: {:?}", command);
    let usecase = command.parse::<Usecases>();
    match usecase {
        Ok(usecase) => match usecase {
            Usecases::TurnOffMusic | Usecases::TurnOnMusic => {
                music_control::play_or_resume_music(command).await;
            }
            Usecases::GetMusicStatus => {
                music_control::get_music_status(userinput).await;
            }
        },

        Err(err) => warn!("Error parsing usecase: {:?}", err),
        // _ => Lexicon::Error.describe().to_string(),
    }
}

async fn get_event_dispatcher() -> Arc<AsyncEventDispatcher> {
    let dispatcher = EVENT_DISPATCHER
        .get_or_init(|| async { Arc::new(AsyncEventDispatcher::new()) })
        .await;
    dispatcher.clone()
}

pub async fn subscribe_once<E, F>(handler: F)
where
    E: 'static + Any + Send + Sync,
    F: Fn(Arc<E>) -> task::JoinHandle<()> + Send + Sync + 'static,
{
    get_event_dispatcher()
        .await
        .subscribe(handler)
        .await;
}

pub async fn publish<E>(event: E)
where
    E: 'static + Any + Send + Sync + std::fmt::Debug,
{
    get_event_dispatcher()
        .await
        .publish(event)
        .await;
}

// general purpose events

/// General response event. Should be used to send only responses to the user.
#[derive(Debug, parse_display::Display)]
pub enum AsyaResponse {
    #[display("{message}")]
    Ok { message: String },
}
