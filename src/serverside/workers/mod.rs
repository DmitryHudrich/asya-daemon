use tokio::join;

pub async fn bootstrap_all() {
    join!(clock::init());
}

mod clock {
    use std::time::Duration;

    use tokio::join;

    use crate::polling::{self, SignalContent};

    pub async fn init() -> String {
        loop {
            debug!("clock polling");
            join!(
                polling::Signal::info(SignalContent::new(chrono::Local::now().to_string())),
                tokio::time::sleep(Duration::from_secs(1))
            );
        }
    }
}
