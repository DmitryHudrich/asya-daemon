use std::sync::Arc;

use logic::workers_infrastructure::WorkerRunner;
use tokio::sync::Mutex;

#[inline]
pub fn arc_mtx(runner: WorkerRunner) -> Arc<Mutex<WorkerRunner>> {
    Arc::new(Mutex::new(runner))
}
