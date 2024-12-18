use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};
use tokio::task;

type AsyncEventHandler =
    Box<dyn Fn(Arc<dyn Any + Send + Sync>) -> task::JoinHandle<()> + Send + Sync>;

#[derive(Default)]
pub struct AsyncEventDispatcher {
    listeners: Arc<RwLock<HashMap<String, Vec<AsyncEventHandler>>>>,
}

mod dispatching;

async fn get_event_dispatcher() -> Arc<AsyncEventDispatcher> {
    static EVENT_DISPATCHER: OnceCell<Arc<AsyncEventDispatcher>> = OnceCell::const_new();
    let dispatcher = EVENT_DISPATCHER
        .get_or_init(|| async { Arc::new(AsyncEventDispatcher::new()) })
        .await;
    dispatcher.clone()
}

pub async fn unsubscribe_all()
where
{
    get_event_dispatcher()
        .await
        .unsubscribe_all()
        .await;
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
    get_event_dispatcher().await.publish(event).await;
}
