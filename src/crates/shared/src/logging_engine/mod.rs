use log::*;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use crate::state;

pub async fn init_logging() {
    let console_pattern = match state::get_logging_place().await.expect("missing config position. todo: remove default values") {
        true => "{f}:{L}: {d(%Y-%m-%d %H:%M:%S)} SERVER {h({l}):5.5}>>> {m}\n",
        false => "{d(%Y-%m-%d %H:%M:%S)} SERVER {h({l}):5.5}>>> {m}\n",
    };
    let config = match state::get_logging_stdout().await.unwrap() {
        true => Config::builder().appender(
            Appender::builder().build("console", Box::new(enable_console(console_pattern))),
        ),
        false => Config::builder(),
    };

    log4rs::init_config(build_config(config, enable_file().await).await).unwrap();

    info!("Logging level: {}", state::get_logging_level().await.unwrap());
    info!("Logging to: {}", state::get_logging_folder().await.unwrap());

    log_check();
}

fn log_check() {
    if log_enabled!(log::Level::Trace) {
        trace!("trace logging example (THIS ISN'T ERROR) - - - - - - OK");
        debug!("debug logging example (THIS ISN'T ERROR) - - - - - - OK");
        info!("info  logging example (THIS ISN'T ERROR) - - - - - - OK");
        warn!("warn  logging example (THIS ISN'T ERROR) - - - - - - OK");
        error!("error logging example (THIS ISN'T ERROR) - - - - - - OK\n------------------------------------------------------------");
    }
}

async fn build_config(config: log4rs::config::runtime::ConfigBuilder, logfile: FileAppender) -> Config {
    config
        .appender(Appender::builder().build("file", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("console")
                .appender("file")
                .build(state::get_logging_level().await.unwrap()),
        )
        .unwrap()
}

async fn enable_file() -> FileAppender {
    FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{f}:{L}: {d(%Y-%m-%d %H:%M:%S)} {h(SERVER)} - {l} > {m}\n",
        )))
        .build(format!(
            "{}/{}aska_logs.log",
            state::get_logging_folder().await.unwrap(),
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S_")
        ))
        .unwrap()
}

fn enable_console(console_pattern: &str) -> ConsoleAppender {
    ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(console_pattern)))
        .build()
}
