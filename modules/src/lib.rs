pub mod workers;
pub mod handlers;

pub async fn init_modules() {
    workers::run_workers().await;
}
