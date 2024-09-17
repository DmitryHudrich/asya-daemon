pub mod workers;

pub async fn init_usecases() {
    workers::run_workers().await;
}
