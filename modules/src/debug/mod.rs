use logic::workers_infrastructure::WorkerRunner;
use tokio::{sync::MutexGuard, task::JoinHandle};

pub(crate) mod workers;

pub fn init_module(runner: MutexGuard<WorkerRunner>) -> Result<JoinHandle<()>, String> {
    let mut runner_clone = runner.clone();
    let handle: JoinHandle<()> = tokio::spawn(async move {
        runner_clone.run_workers().await;
    });
    Ok(handle)
}
