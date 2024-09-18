pub mod workers;
pub mod handlers;

pub async fn init_usecases() {
    workers::run_workers().await;
}
