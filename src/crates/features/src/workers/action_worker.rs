use std::sync::Arc;

use rand::seq::SliceRandom;
use tokio::sync::RwLock;

use super::Observer;

static PHRASES: &[&str] = &["Hi!", "How r u?", "ну шо ти епта"];

#[derive(Default)]
pub struct ActionsWorker {
    observers: Arc<RwLock<Vec<Box<dyn Observer<String> + Send + Sync>>>>,
}

impl ActionsWorker {
    pub async fn new() -> Self {
        Self {
            observers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn subscribe(&self, observer: Box<dyn Observer<String> + Send + Sync>) {
        let mut p = self.observers.write().await;
        p.push(observer);
    }

    pub async fn run(&self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;

            // Выбор фразы
            let phrase = PHRASES.choose(&mut rand::thread_rng()).unwrap();

            // Уведомление подписчиков
            self.notify_observers(phrase).await;
        }
    }

    async fn notify_observers(&self, phrase: &str) {
        let observers = self.observers.read().await;
        for observer in observers.iter() {
            observer.update(&phrase.to_owned()).await;
        }
    }
}
