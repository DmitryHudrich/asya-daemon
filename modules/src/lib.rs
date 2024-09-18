use log::warn;
use logic::workers_infrastructure::WorkerRunner;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::{Mutex, MutexGuard},
    task::JoinHandle,
};

pub mod debug;
pub mod systeminfo;
mod telegram;

pub struct AskaModule {
    name: String,
    initializer: fn(MutexGuard<WorkerRunner>) -> Result<JoinHandle<()>, String>,
    worker_runner: Arc<Mutex<WorkerRunner>>,
}

impl AskaModule {
    pub fn new(
        name: &str,
        worker_runner: Arc<Mutex<WorkerRunner>>,
        initializer: fn(MutexGuard<WorkerRunner>) -> Result<JoinHandle<()>, String>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            worker_runner,
            initializer,
        }
    }
}

const DEBUG_MODULE: &str = "debug";

pub async fn get_modules() -> HashMap<String, AskaModule> {
    let mut modules = HashMap::new();

    let debug_worker_runner = Arc::new(Mutex::new(debug::workers::get_runner().await));
    modules.insert(
        DEBUG_MODULE.to_owned(),
        AskaModule::new(
            DEBUG_MODULE,
            debug_worker_runner.clone(),
            debug::init_module,
        ),
    );

    for (module_name, module) in &modules {
        let handle = (module.initializer)(debug_worker_runner.lock().await);
        match handle {
            Ok(h) => h.await.expect("wtf???"),
            Err(err) => warn!("module {} wasn;t loaded: {}", module_name, err)
        }
    }

    modules
}
