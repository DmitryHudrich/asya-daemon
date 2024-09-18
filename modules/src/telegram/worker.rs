use std::time::Duration;

use log::info;
use logic::workers_infrastructure::{worker_states::WorkerState, Worker, WorkerRunner};
use tokio::{spawn, time::sleep};

pub async fn get_runner() -> WorkerRunner {
    let mut worker_runner = WorkerRunner::new();
    _ = worker_runner.push_worker(
        "main_telegram".to_owned(),
        Worker::new(
            || Box::pin(async move { WorkerState::Telegram }),
            |state| {
                spawn(async move {
                    loop {
                        _ = state;
                        info!("telegram still running...");
                        sleep(Duration::from_secs(3)).await;
                    }
                });
            },
            Duration::MAX,
        ),
    );
    worker_runner
}
