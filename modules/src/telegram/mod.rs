use logic::workers_infrastructure::WorkerRunner;
use tokio::{sync::MutexGuard, task::JoinHandle};

pub mod worker;

pub fn init_module(runner: MutexGuard<'_, WorkerRunner>) -> Result<JoinHandle<()>, String> {
    let mut runner_clone = runner.clone();
    let handle: JoinHandle<()> = tokio::spawn(async move {
        runner_clone.run_workers().await;
    });
    Ok(handle)
}

