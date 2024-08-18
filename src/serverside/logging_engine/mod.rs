use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use crate::configuration;

pub fn init_logging() {
    if configuration::get().logging().stdout() {
        println!("-- logging bootstrapping >> logging enabled with stdout, but not configured yet,");
        println!("                            so you can't see some logs or see them incorrectly.");
    }

    let console_pattern = match configuration::get().logging().place() {
        true => "{f}:{L}: {d(%Y-%m-%d %H:%M:%S)} SERVER {h({l}):5.5}>>> {m}\n",
        false => "{d(%Y-%m-%d %H:%M:%S)} SERVER {h({l}):5.5}>>> {m}\n",
    };
    let console = enable_console(console_pattern);
    let logfile = enable_file();
    let config = match configuration::get().logging().stdout() {
        true => Config::builder().appender(Appender::builder().build("console", Box::new(console))),
        false => Config::builder(),
    };
    let built = build_config(config, logfile);
    log4rs::init_config(built).unwrap();

    if configuration::get().logging().stdout() {
        println!("-- logging bootstrapping >> configured log level: {}", configuration::get().logging().level());
        println!("-- logging bootstrapping >> write log place: {}", configuration::get().logging().place());
        println!("-- logging bootstrapping >> folder: {}", configuration::get().logging().folder());
        println!("-- logging bootstrapping >> stdout: {}", configuration::get().logging().stdout());
        println!("-- logging bootstrapping >> bootstrapped logging system successfully. now you can see all logs with configured level in your terminal.");
    }

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

fn build_config(config: log4rs::config::runtime::ConfigBuilder, logfile: FileAppender) -> Config {
    config
        .appender(Appender::builder().build("file", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("console")
                .appender("file")
                .build(configuration::get().logging().level().to_owned()),
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
            configuration::get().logging().folder(),
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S_")
        ))
        .unwrap()
}

fn enable_console(console_pattern: &str) -> ConsoleAppender {
    ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(console_pattern)))
        .build()
}
