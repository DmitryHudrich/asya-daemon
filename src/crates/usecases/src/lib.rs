use log::*;
use music_control::Usecases;
use services::{lexicon::Lexicon, llm_api};
use shared::{event_system::AsyncEventDispatcher, llm};
use std::{any::Any, collections::HashMap, sync::Arc};
use tokio::{
    sync::OnceCell,
    task,
};

pub mod music_control;
pub mod workers;

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

#[derive(Debug, Default)]
pub struct PromptBuilder {
    varibles: HashMap<String, String>,
    fallback_phrase: Lexicon,
    prompt_path: String,
}

impl PromptBuilder {
    pub fn new() -> Self {
        PromptBuilder::default()
    }

    pub fn set_path(&mut self, path: &str) -> &mut Self {
        self.prompt_path = path.to_string();
        self
    }

    pub fn set_fallback_phrase(&mut self, phrase: Lexicon) -> &mut Self {
        self.fallback_phrase = phrase;
        self
    }

    pub fn set_variable(&mut self, key: &str, value: &str) -> &mut Self {
        self.varibles.insert(key.to_string(), value.to_string());
        self
    }

    pub async fn get_result(&self) -> String {
        let mut prompt = llm::get_prompt(self.prompt_path.as_str());
        for (key, value) in &self.varibles {
            prompt = prompt.replace(key, value);
        }
        let response = llm_api::send_request(prompt).await;
        response.unwrap_or(Lexicon::MusicResume.describe().to_string())
    }
}

// general purpose events

/// General response event. Should be used to send only responses to the user.
#[derive(Debug, parse_display::Display)]
pub enum AsyaResponse {
    #[display("{message}")]
    Ok { message: String },
}
