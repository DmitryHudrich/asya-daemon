use tokio::task;

pub async fn start_basic_monitoring(_: String) {
    task::spawn(async {
        loop {
            
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });
}
