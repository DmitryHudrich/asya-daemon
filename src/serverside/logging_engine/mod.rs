use colored::Colorize;
use configuration::LoggingParams;
use std::io::Write;
use lazy_static::lazy_static;
use chrono::Local;
use colored::ColoredString;
use log::Level;

mod configuration;

lazy_static!{
    static ref CONFIGURATION: LoggingParams = LoggingParams::new();
}

pub fn init_logging() {
    log::set_max_level(CONFIGURATION.level);

    build_logger();

    println!(
        "----------------------------------------------------------------------------------------|"
    );
    println!("-> Log level: {}", log::max_level());
    println!("\n");
    info!("Logging rabotaet");
}

fn build_logger() {
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
