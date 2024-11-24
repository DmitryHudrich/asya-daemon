use action_worker::ActionsWorker;
use async_trait::async_trait;
use std::sync::Arc;

use lazy_static::lazy_static;
use tokio::sync::RwLock;

mod action_worker;

lazy_static! {
    pub static ref ACTION_WORKER: Arc<RwLock<ActionsWorker>> =
        Arc::new(RwLock::new(ActionsWorker::default()));
}

pub async fn run_workers() {
    let mut action_worker = ACTION_WORKER.write().await;
    *action_worker = ActionsWorker::new().await;
    let action_worker = action_worker.downgrade();
    action_worker.run().await;
}

#[async_trait]
pub trait Observer<T> {
    async fn update(&self, phrase: &T);
}
