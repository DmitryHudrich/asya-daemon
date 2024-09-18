use log::info;
use shared::types::PinnedFuture;
use std::{collections::HashMap, time::Duration};
use tokio::{task::JoinSet, time::sleep};
use worker_states::WorkerState;

pub mod worker_states;

#[derive(Clone)]
pub struct Worker {
    state: WorkerState,
    initialize: fn() -> PinnedFuture<WorkerState>,
    work: fn(WorkerState) -> PinnedFuture<WorkerState>,
    sleepness: Duration,
}

impl Worker {
    pub fn new(
        initialize: fn() -> PinnedFuture<WorkerState>,
        work: fn(WorkerState) -> PinnedFuture<WorkerState>,
        sleepness: Duration,
    ) -> Self {
        Self {
            state: WorkerState::Empty,
            initialize,
            sleepness,
            work,
        }
    }
}

#[derive(Clone)]
pub struct WorkerRunner {
    workers: HashMap<String, Worker>,
}

impl WorkerRunner {
    pub fn new() -> Self {
        Self {
            workers: HashMap::new(),
        }
    }

    // maybe we can make more convinient error handling
    pub fn push_worker(
        &mut self,
        descriptor: String,
        worker: Worker,
    ) -> Result<(), String /* descriptor */> {
        match self.workers.insert(descriptor.clone(), worker) {
            Some(_) => Result::Err(descriptor),
            None => Result::Ok(()),
        }
    }

    pub fn push_many_workers(
        &mut self,
        workers: Vec<(String, Worker)>,
    ) -> HashMap<String, Result<(), String>> {
        let mut func_res = HashMap::new();
        for (descriptor, worker) in workers {
            let loop_res = self.push_worker(descriptor.clone(), worker);
            func_res.insert(descriptor, loop_res);
        }
        func_res
    }

    pub async fn run_workers(&mut self) {
        let mut worker_tasks = JoinSet::new();

        for (descriptor, mut worker) in self.workers.drain() {
            worker_tasks.spawn(async move {
                worker.state = (worker.initialize)().await;
                info!("Starting worker initialization: {}", descriptor);
                loop {
                    worker.state = (worker.work)(worker.state).await;
                    sleep(worker.sleepness).await;
                }
            });
        }
        while (worker_tasks.join_next().await).is_some() {}
    }
}

impl Default for WorkerRunner {
    fn default() -> Self {
        Self::new()
    }
}
