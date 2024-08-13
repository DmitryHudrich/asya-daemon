use std::env;

use log::LevelFilter;

const LOGGING_LEVEL: &str = "RUST_LOG";

const LOGGING_FILE: &str = "LOGGING_FILE";

const LOGGING_FILES_COUNT: &str = "LOGGING_FILES_COUNT";

const LOGGING_STDOUT: &str = "LOGGING_STDOUT";

const LOGGING_FORMAT: &str = "LOGGING_FORMAT";

// Env variables:
pub(crate) struct LoggingParams {
    // TODO:
    // LOG_FORMAT - main info (or even server-side ui) exists separately from logging.
    // format: String,

    // LOGGING_FILE - 1 or 0.
    // Determines should we log to file or not. 1 by default.
    // Default name is aska.log in ./logs folder.
    // Logs wroten in file includes modules info. */
    pub file: bool,

    // LOGGING_FILES_COUNT - max number of logs to keep. 20 by default.
    pub files_count: usize,

    // RUST_LOG - logging level. maybe name will be changed in future to ASKA_LOG or something like that.
    pub level: log::LevelFilter,

    // LOGGING_STDOUT - 1 or 0. determines should we log to stdout or not. 1 by default.
    pub stdout: bool,
    // maybe more??????
}

impl LoggingParams {
    pub(crate) fn new() -> LoggingParams {
        LoggingParams {
            file: match env::var(LOGGING_FILE) {
                Ok(v) if v == "0" => false,
                Ok(_) => true,
                Err(err) => {
                    warn!("LOG_FORMAT: {}", err);
                    true
                }
            },
            files_count: match env::var(LOGGING_FILES_COUNT) {
                Ok(v) => v.parse::<usize>().unwrap_or({
                    warn!(
                        "LOGGING_FILES_COUNT: '{}' wasn't succesfully parsed. Using default value.",
                        v
                    );
                    20
                }),
                Err(err) => {
                    warn!("LOG_FORMAT: {}", err);
                    20
                }
            },
            level: match env::var(LOGGING_LEVEL) {
                Ok(v) => v.parse().unwrap_or({
                    warn!(
                        "RUST_LOG: '{}' wasn't succesfully parsed. Using default value.",
                        v
                    );
                    LevelFilter::Info
                }),
                Err(err) => {
                    warn!("LOG_FORMAT: {}", err);
                    LevelFilter::Info
                }
            },
            stdout: match env::var(LOGGING_STDOUT) {
                Ok(v) if v == "0" => false,
                Ok(_) => true,
                Err(err) => {
                    warn!("LOGGING_STDOUT: {}", err);
                    true
                }
            },
        }
    }
}
