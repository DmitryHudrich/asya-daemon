use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

#[allow(dead_code)]
mod configuration;

pub fn init_logging() {
    let console_pattern = match configuration::LOG_PLACE.value() {
        true => "{f}:{L}: {d(%Y-%m-%d %H:%M:%S)} SERVER {h({l}):5.5}>>> {m}\n",
        false => "{d(%Y-%m-%d %H:%M:%S)} SERVER {h({l}):5.5}>>> {m}\n",
    };
    let console = enable_console(console_pattern);
    let logfile = enable_file();
    let config = match configuration::LOGGING_STDOUT.value() {
        true => Config::builder().appender(Appender::builder().build("console", Box::new(console))),
        false => Config::builder(),
    };
    let builded = build_config(config, logfile);
    log4rs::init_config(builded).unwrap();
    if log_enabled!(log::Level::Trace) {
        trace!("trace logging examble (THIS ISN'T ERROR) - - - - - - OK");
        debug!("debug logging examble (THIS ISN'T ERROR) - - - - - - OK");
        info!("info  logging examble (THIS ISN'T ERROR) - - - - - - OK");
        warn!("warn  logging examble (THIS ISN'T ERROR) - - - - - - OK");
        error!("error logging examble (THIS ISN'T ERROR) - - - - - - OK\n------------------------------------------------------------");
    }
}

fn build_config(config: log4rs::config::runtime::ConfigBuilder, logfile: FileAppender) -> Config {
    config
        .appender(Appender::builder().build("file", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("console")
                .appender("file")
                .build(configuration::LOGGING_LEVEL.value()),
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
            configuration::LOGGING_FOLDER.value(),
            chrono::Local::now().format("%Y-%m-%d_%H:%M:%S_")
        ))
        .unwrap()
}

fn enable_console(console_pattern: &str) -> ConsoleAppender {
    ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(console_pattern)))
        .build()
}
