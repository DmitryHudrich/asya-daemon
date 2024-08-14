use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

#[allow(dead_code)]
mod configuration;

pub fn init_logging() {
    let console_pattern = "{d(%Y-%m-%d %H:%M:%S)} SERVER {h({l}):5.5}>>> {m}\n";

    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(console_pattern)))
        .build();

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} {h(SERVER)} - {l} > {m}\n",
        )))
        .build("log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .appender(Appender::builder().build("file", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("console")
                .appender("file")
                .build(configuration::LOGGING_LEVEL.value()),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    if log_enabled!(log::Level::Trace) {
        trace!("trace logging examble - OK");
        debug!("debug logging examble - OK");
        info!("info logging examble - OK");
        warn!("warn logging examble - OK");
        error!("error logging examble - OK");
    }
}
