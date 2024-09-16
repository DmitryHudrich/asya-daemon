use std::time::Duration;
use log::info;
use tokio::time::sleep;
use utils::{Worker, WorkerRunner};

mod utils;

pub enum WorkerState {
    Empty,
    Mouse { x: u64, y: u64 },
    Weather { weather: String, celsius: i64 },
    Timer { current_time: u64 },
}

pub async fn setup_workers() {
    let workers = vec![
        Worker::new(
            || {
                Box::pin(async move {
                    info!("init worker mouse");
                    sleep(Duration::from_secs(10)).await;
                    WorkerState::Mouse { x: 13, y: 4 }
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
        ),
        Worker::new(
            || {
                Box::pin(async move {
                    info!("init worker time");
                    sleep(Duration::from_secs(10)).await;
                    WorkerState::Mouse { x: 13, y: 4 }
                })
            },
            || Box::pin(async move { WorkerState::Timer { current_time: 1488 } }),
        ),
    ];

    let mut runner = WorkerRunner::new(workers);
    runner.run_workers().await;
}


