use log::info;
use logic::workers_infrastructure::{worker_states::WorkerState, Worker, WorkerRunner};
use std::time::Duration;

pub fn set(worker_runner: &mut WorkerRunner) {
    let proc_worker = Worker::new(
        || Box::pin(async move { WorkerState::ProcMonitor { processes: vec![] } }),
        |worker_state| {
            Box::pin(async move {
                let state = match worker_state {
                    WorkerState::Counter { count: c } => c,
                    _ => 0,
                };
                info!("last state: {:?}\t current state: {:?}", state, state + 1);
                WorkerState::Counter { count: state + 1 }
            })
        },
        Duration::from_secs(1),
    );

    _ = worker_runner.push_worker("proccess_monitor".to_string(), proc_worker);
}
