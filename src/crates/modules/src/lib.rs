use log::warn;
use logic::workers_infrastructure::WorkerRunner;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::{Mutex, MutexGuard},
    task::JoinHandle,
};

// use utils::arc_mtx;

pub mod systeminfo;
pub mod debug;

// mod utils;
//
// pub struct AskaModule {
//     name: String,
//     initializer: fn(WorkerRunner) -> Result<(), String>,
//     worker_runner: Arc<Mutex<WorkerRunner>>,
// }
//
// impl AskaModule {
//     pub fn new(
//         name: &str,
//         worker_runner: Arc<Mutex<WorkerRunner>>,
//         initializer: fn(WorkerRunner) -> Result<(), String>,
//     ) -> Self {
//         Self {
//             name: name.to_owned(),
//             worker_runner,
//             initializer,
//         }
//     }
// }
//
// const DEBUG_MODULE: &str = "debug";
// const TELEGRAM_MODULE: &str = "telegram";
//
// pub async fn get_modules() -> HashMap<String, AskaModule> {
//     let mut modules = HashMap::new();
//
//     let debug_worker_runner = arc_mtx(debug::workers::get_runner().await);
//     modules.insert(
//         DEBUG_MODULE.to_owned(),
//         AskaModule::new(
//             DEBUG_MODULE,
//             debug_worker_runner.clone(),
//             debug::init_module,
//         ),
//     );
//
//     let telegram_worker_runner = arc_mtx(telegram::worker::get_runner().await);
//     modules.insert(
//         TELEGRAM_MODULE.to_owned(),
//         AskaModule::new(
//             TELEGRAM_MODULE,
//             telegram_worker_runner.clone(),
//             telegram::init_module,
//         ),
//     );
//
//     init_all_modules(&modules).await;
//
//     modules
// }
//
// async fn init_all_modules(modules: &HashMap<String, AskaModule>) {
//     for (module_name, module) in modules {
//         let init_res = (module.initializer)(module.worker_runner.lock().await);
//         match init_res {
//             Ok(join_handle) => join_handle.await.expect("wtf???"),
//             Err(err) => warn!("module {} wasn't loaded: {}", module_name, err),
//         }
//     }
// }
