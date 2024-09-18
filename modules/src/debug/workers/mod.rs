use logic::workers_infrastructure::WorkerRunner;

pub mod processes_workers;

pub async fn get_runner() -> WorkerRunner {
    let mut worker_runner = WorkerRunner::new();
    processes_workers::set(&mut worker_runner);
    worker_runner.run_workers().await;
    worker_runner
}
