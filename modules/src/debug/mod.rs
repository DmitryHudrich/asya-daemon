mod workers;
mod handlers;

pub async fn init_module() {
    workers::run_workers().await;
}

