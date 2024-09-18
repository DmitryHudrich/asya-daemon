use log::{debug, info};
use shared::types::PinnedFuture;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{spawn, sync::RwLock, task::JoinSet, time::sleep};
use worker_states::WorkerState;

pub mod worker_states;

#[derive(Clone)]
pub struct Worker {
    state: Arc<RwLock<WorkerState>>,
    initialize: fn() -> PinnedFuture<WorkerState>,
    work: fn(Arc<RwLock<WorkerState>>),
    sleepness: Duration,
}

impl Worker {
    pub fn new(
        initialize: fn() -> PinnedFuture<WorkerState>,
        work: fn(Arc<RwLock<WorkerState>>),
        sleepness: Duration,
    ) -> Self {
        Self {
            state: Arc::new(RwLock::new(WorkerState::Empty)),
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
        for (descriptor, worker) in self.workers.drain() {
            let mut state_write = worker.state.write().await;
            *state_write = (worker.initialize)().await;
            drop(state_write);
            spawn(async move {
                info!("Starting worker initialization: {}", descriptor);
                loop {
                    (worker.work)(worker.state.clone());
                    sleep(worker.sleepness).await;
                    debug!("worker {} inited", descriptor);
                }
            });
        }
    }
}

impl Default for WorkerRunner {
    fn default() -> Self {
        Self::new()
    }
}
