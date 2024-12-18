use log::debug;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task;

use super::AsyncEventDispatcher;
// maybe we should make another version `subscribe` and `publish` methods which works with
// multiple events.
impl AsyncEventDispatcher {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    //change &str to enum
    pub async fn subscribe<E, F>(&self, handler: F)
    where
        E: 'static + Any + Send + Sync,
        F: Fn(Arc<E>) -> task::JoinHandle<()> + Send + Sync + 'static,
    {
        let mut listeners = self.listeners.write().await;
        // todo:
        // dedicate entries for platforms. that each platform has its own listeners.
        let event_type = std::any::type_name::<E>().to_string();
        if !listeners.contains_key(&event_type) {
            listeners.entry(event_type).or_default().push(Box::new(
                move |event: Arc<dyn Any + Send + Sync>| {
                    if let Ok(event) = Arc::downcast::<E>(event.clone()) {
                        handler(event)
                    } else {
                        panic!();
                    }
                },
            ));
        }
    }

    pub async fn unsubscribe_all(&self) {
        let mut listeners = self.listeners.write().await;
        listeners.clear();
    }

    pub async fn publish<E>(&self, event: E)
    where
        E: 'static + Any + Send + Sync + std::fmt::Debug,
    {
        let listeners = self.listeners.read().await;
        let event_type = std::any::type_name::<E>().to_string();
        if let Some(handlers) = listeners.get(&event_type) {
            let event = Arc::new(event);

            for handler in handlers {
                let cloned_event = event.clone();
                debug!("Publishing event: {} - {:?}", event_type, cloned_event);
                handler(cloned_event).await.unwrap();
            }
        }
    }
}
