use logic::workers_infrastructure::WorkerRunner;
use shared::types::PinnedFuture;
use std::{collections::HashMap, rc::Rc};

pub mod debug;
pub mod systeminfo;
mod telegram;

pub struct AskaModule {
    name: String,
    initializer: PinnedFuture<Result<String, String>>,
    worker_runner: Rc<WorkerRunner>,
}

impl AskaModule {
    pub fn new(
        name: &str,
        worker_runner: WorkerRunner,
        initializer: PinnedFuture<Result<String, String>>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            worker_runner: Rc::new(worker_runner),
            initializer,
        }
    }
}

const DEBUG_MODULE: &str = "debug";

pub async fn init_modules() -> HashMap<String, AskaModule> {
    let mut modules = HashMap::new();
    modules.insert(
        DEBUG_MODULE.to_owned(),
        AskaModule::new(
            DEBUG_MODULE,
            debug::workers::get_runner().await,
            Box::pin(async move { debug::init_module().await }),
        ),
    );
    modules
}
