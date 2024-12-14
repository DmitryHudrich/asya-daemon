use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task;

type AsyncEventHandler =
    Box<dyn Fn(Arc<dyn Any + Send + Sync>) -> task::JoinHandle<()> + Send + Sync>;

#[derive(Default)]
pub struct AsyncEventDispatcher {
    listeners: Arc<RwLock<HashMap<String, Vec<AsyncEventHandler>>>>,
}

impl AsyncEventDispatcher {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn subscribe<E, F>(&self, handler: F)
    where
        E: 'static + Any + Send + Sync,
        F: Fn(Arc<E>) -> task::JoinHandle<()> + Send + Sync + 'static,
    {
        let mut listeners = self.listeners.write().await;
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

    pub async fn publish<E>(&self, event: E)
    where
        E: 'static + Any + Send + Sync,
    {
        let listeners = self.listeners.read().await;
        let event_type = std::any::type_name::<E>().to_string();

        if let Some(handlers) = listeners.get(&event_type) {
            let event = Arc::new(event);

            for handler in handlers {
                let cloned_event = event.clone();
                handler(cloned_event).await.unwrap();
            }
        }
    }
}
