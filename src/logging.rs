use log::*;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use shared::configuration::CONFIG;

pub fn init_logging() {
    let console_pattern = match CONFIG.logging.place {
        true => "{f}:{L}: {d(%Y-%m-%d %H:%M:%S)} SERVER {h({l}):5.5}>>> {m}\n",
        false => "{d(%Y-%m-%d %H:%M:%S)} SERVER {h({l}):5.5}>>> {m}\n",
    };
    let config = match CONFIG.logging.stdout {
        true => Config::builder().appender(
            Appender::builder().build("console", Box::new(enable_console(console_pattern))),
        ),
        false => Config::builder(),
    };

    log4rs::init_config(build_config(config, enable_file())).unwrap();

    info!("Logging level: {}", CONFIG.logging.level);
    info!("Logging to: {}", CONFIG.logging.folder);

    if log_enabled!(log::Level::Trace) {
        log_check();
    }
}

fn log_check() {
    trace!("trace logging example (THIS ISN'T ERROR) - - - - - - OK");
    debug!("debug logging example (THIS ISN'T ERROR) - - - - - - OK");
    info!("info  logging example (THIS ISN'T ERROR) - - - - - - OK");
    warn!("warn  logging example (THIS ISN'T ERROR) - - - - - - OK");
    error!("error logging example (THIS ISN'T ERROR) - - - - - - OK\n------------------------------------------------------------");
}

fn build_config(config: log4rs::config::runtime::ConfigBuilder, logfile: FileAppender) -> Config {
    config
        .appender(Appender::builder().build("file", Box::new(logfile)))
        .logger(log4rs::config::Logger::builder().build("teloxide", log::LevelFilter::Off))
        .logger(log4rs::config::Logger::builder().build("hyper", log::LevelFilter::Off))
        .logger(log4rs::config::Logger::builder().build("reqwest", log::LevelFilter::Off))
        .build(
            Root::builder()
                .appender("console")
                .appender("file")
                .build(CONFIG.logging.level),
        )
        .unwrap()
}

fn enable_file() -> FileAppender {
    FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{f}:{L}: {d(%Y-%m-%d %H:%M:%S)} {h(SERVER)} - {l} > {m}\n",
        )))
        .build(format!(
            "{}/{}aska_logs.log",
            CONFIG.logging.folder,
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S_")
        ))
        .unwrap()
}

fn enable_console(console_pattern: &str) -> ConsoleAppender {
    ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(console_pattern)))
        .build()
}
