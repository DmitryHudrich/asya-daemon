use std::{future::Future, pin::Pin, time::Duration};

use log::info;
use tokio::{task::JoinSet, time::sleep};

use super::WorkerState;

// this is used for asynchronous initialization and worker operation
type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

pub struct Worker {
    state: WorkerState,
    initialize: fn() -> PinnedFuture<WorkerState>,
    work: fn() -> PinnedFuture<WorkerState>,
}

impl Worker {
    pub fn new(
        initialize: fn() -> PinnedFuture<WorkerState>,
        work: fn() -> PinnedFuture<WorkerState>,
    ) -> Self {
        Self {
            state: WorkerState::Empty,
            initialize,
            work,
        }
    }
}

pub struct WorkerRunner {
    workers: Vec<Worker>,
}

impl WorkerRunner {
    pub fn new(workers: Vec<Worker>) -> Self {
        Self { workers }
    }

    pub async fn run_workers(&mut self) {
        let mut worker_tasks = JoinSet::new();

        for mut worker in self.workers.drain(..) {
            worker_tasks.spawn(async move {
                worker.state = (worker.initialize)().await;
                info!("worker was init");
                loop {
                    worker.state = (worker.work)().await;
                    sleep(Duration::from_secs(1)).await;
                }
            });
        }
        while (worker_tasks.join_next().await).is_some() {}
    }
}
