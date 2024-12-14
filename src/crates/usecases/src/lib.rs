use log::*;
use music_control::Usecases;
use shared::event_system::AsyncEventDispatcher;
use std::{any::Any, sync::Arc};
use tokio::{
    sync::{OnceCell, RwLock},
    task,
};

pub mod music_control;

static EVENT_DISPATCHER: OnceCell<Arc<RwLock<AsyncEventDispatcher>>> = OnceCell::const_new();

pub async fn dispatch_usecase(command: String, userinput: String) {
    debug!("Dispatching music command: {:?}", command);
    let usecase = command.parse::<Usecases>();
    match usecase {
        Ok(usecase) => match usecase {
            Usecases::TurnOffMusic | Usecases::TurnOnMusic => {
                music_control::play_or_resume_music(command, userinput).await;
            }
            Usecases::GetMusicStatus => {
                music_control::get_music_status(command, userinput).await;
            }
        },

        Err(err) => warn!("Error parsing usecase: {:?}", err),
        // _ => Lexicon::Error.describe().to_string(),
    }
}


async fn get_event_dispatcher() -> Arc<RwLock<AsyncEventDispatcher>> {
    let dispatcher = EVENT_DISPATCHER
        .get_or_init(|| async { Arc::new(RwLock::new(AsyncEventDispatcher::new())) })
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
        .read()
        .await
        .subscribe(handler)
        .await;
}

pub async fn publish<E>(event: E)
where
    E: 'static + Any + Send + Sync,
{
    get_event_dispatcher()
        .await
        .read()
        .await
        .publish(event)
        .await;
}

// general purpose events


/// General response event. Should be used to send only responses to the user.
#[derive(Debug, parse_display::Display)]
pub enum AsyaResponse {
    #[display("{}")]
    Ok { message: String },
}
