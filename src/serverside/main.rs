#[macro_use]
extern crate log;
extern crate env_logger;

use chrono::Local;
use colored::{ColoredString, Colorize};
use log::Level;
use std::{env, io::Write, str::FromStr};

mod server;
pub mod service;

#[tokio::main]
async fn main() {
    init_logging();
    let server_launching = server::launch_server();
    info!("Bootstrapping");
    let _ = server_launching.await;
}

fn init_logging() {
    if env::var_os("RUST_LOG").is_none() {
        log::set_max_level(log::LevelFilter::Info);
    } else {
        log::set_max_level(
            log::LevelFilter::from_str(env::var_os("RUST_LOG").unwrap().to_str().unwrap()).unwrap(),
        );
    }

    env_logger::Builder::from_default_env()
        .filter_level(log::max_level())
        .format(|buf, record| {
            writeln!(
                buf,
                "{} SERVER {}\t{}",
                Local::now().format("%d/%m/%Y %H:%M"),
                colourful_loglevel(record.level()),
                record.args()
            )
        })
        .init();

    println!(
        "----------------------------------------------------------------------------------------|"
    );
    println!("-> Log level: {}", log::max_level());
    println!("\n");
    info!("Logging rabotaet");
}

fn colourful_loglevel(level: Level) -> ColoredString {
    match level {
        Level::Error => level.to_string().red(),
        Level::Warn => level.to_string().yellow(),
        Level::Info => level.to_string().blue(),
        Level::Debug => level.to_string().cyan(),
        Level::Trace => level.to_string().magenta(),
    }
}
