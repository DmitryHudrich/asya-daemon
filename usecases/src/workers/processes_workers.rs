use std::time::Duration;

use logic::workers_infrastructure::{worker_states::WorkerState, Worker, WorkerRunner};

pub fn set(worker_runner: &mut WorkerRunner) {
    let proc_worker = Worker::new(
        || {
            Box::pin(async move {
                WorkerState::ProcMonitor { processes: vec![] }
            })
        },
        || {
            Box::pin(async move {
                WorkerState::Mouse {
                    x: rand::random(),
                    y: rand::random(),
                }
            })
        },
        Duration::from_secs(1),
    );

    _ = worker_runner.push_worker("proccess_monitor".to_string(), proc_worker);
}
