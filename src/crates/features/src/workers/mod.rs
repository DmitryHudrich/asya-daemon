use action_worker::ActionsWorker;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::{
    join,
    sync::{OnceCell, RwLock, RwLockReadGuard},
};

mod action_worker;

pub type WorkerCell = OnceCell<Arc<RwLock<ActionsWorker>>>;
pub type WorkerArc = Arc<RwLock<ActionsWorker>>;
pub type WorkerToWrite<'a> = RwLockReadGuard<'a, ActionsWorker>;

static ACTION_WORKER: WorkerCell = OnceCell::const_new();
pub async fn get_actionworker<'a>() -> WorkerToWrite<'a> {
    ACTION_WORKER
        .get_or_init(|| async { Arc::new(RwLock::new(ActionsWorker::new().await)) })
        .await;
    ACTION_WORKER.get().unwrap().read().await
}

// lazy_static! {
//     pub static ref ACTION_WORKER: Arc<RwLock<ActionsWorker>> =
//         Arc::new(RwLock::new(ActionsWorker::new().await));
// }

pub async fn run_workers() {
    let action_worker = &get_actionworker().await;
    join!(action_worker.run());
}

#[async_trait]
pub trait Observer<T> {
    async fn update(&self, phrase: &T);
}
