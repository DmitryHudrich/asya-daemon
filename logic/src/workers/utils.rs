use std::{future::Future, pin::Pin, time::Duration};

use log::info;
use tokio::{task::JoinSet, time::sleep};

use super::WorkerState;

type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

pub struct Worker {
    state: WorkerState,
    initializer: fn() -> PinnedFuture<WorkerState>,
    work: fn() -> PinnedFuture<WorkerState>,
}

impl Worker {
    pub fn new(
        initializer: fn() -> PinnedFuture<WorkerState>,
        work: fn() -> PinnedFuture<WorkerState>,
    ) -> Self {
        Self {
            state: WorkerState::Empty,
            initializer,
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
                worker.state = (worker.initializer)().await;
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
