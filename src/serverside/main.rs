#[macro_use]
extern crate log;
extern crate env_logger;

mod server;
mod logging_engine;

pub mod service;

#[tokio::main]
async fn main() {
    logging_engine::init_logging();
    let server_launching = server::launch_server();
    info!("Bootstrapping");
    let _ = server_launching.await;
}
