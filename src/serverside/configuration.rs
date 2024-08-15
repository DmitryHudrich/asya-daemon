//! Environment variables.

use std::env::VarError;

use lazy_static::lazy_static;
use log::LevelFilter;

lazy_static! {
    pub static ref PORT: EnvVariable<u16> = EnvVariable {
        value: EnvValue::Default(3000),
        evaluater: |name, value| {
            let port = value.as_str().parse();
            match port {
                Ok(port) => Some(port),
                Err(err) => {
                   warn!("Failed to parse port. {name}: {}", err); use_default()
                },
            }
        },
        name: "PORT",
    };

    pub static ref LOGGING_LEVEL: EnvVariable<LevelFilter> = EnvVariable {
        value: EnvValue::Default(LevelFilter::Info),
        evaluater: |_, value| {
            match value.as_str() {
                "TRACE" => Some(LevelFilter::Trace),
                "DEBUG" => Some(LevelFilter::Debug),
                "INFO" => Some(LevelFilter::Info),
                "WARN" => Some(LevelFilter::Warn),
                "ERROR" => Some(LevelFilter::Error),
                _ => use_default(),
            }
        },
        name: "LOG_LEVEL",
    };

    pub static ref LOGGING_FOLDER: EnvVariable<String> = EnvVariable {
        value: EnvValue::Default("logs".to_string()),
        evaluater: |_, value| Some(value),
        name: "LOG_FOLDER",
    };

    // TODO: this is placeholder. Variable isn't used yet.
    pub static ref LOGGING_FILESCOUNT: EnvVariable<usize> = EnvVariable {
        value: EnvValue::Default(20),
        evaluater: |name, value| {
            match value.parse::<usize>() {
                Ok(number) => Some(number),
                Err(e) => {
                    warn!("Failed to parse {name}: {}", e);
                    use_default()
                }
            }
        },
        name: "LOGFILES_COUNT",
    };

    pub static ref LOGGING_STDOUT: EnvVariable<bool> = EnvVariable {
        value: EnvValue::Default(true),
        evaluater: |_, value| {
            match value.as_str() {
                "0" => Some(false),
                "1" => Some(true),
                _ => use_default(),
            }
        },
        name: "LOG_CONSOLE",
    };

    pub static ref LOG_PLACE: EnvVariable<bool> = EnvVariable {
        value: EnvValue::Default(false),
        evaluater: |_, value| {
            match value.as_str() {
                "0" => Some(false),
                "1" => Some(true),
                _ => use_default(),
            }
        },
        name: "LOG_PLACE",
    };
}

/*------------------end of configuration options---------------------------------------*/
/* ------------------------------------------------------------------------------------*/

fn use_default<T>() -> Option<T> {
    None
}

enum EnvValue<T> {
    Some(T),
    Default(T),
}

pub struct EnvVariable<T> {
    value: EnvValue<T>,
    evaluater: fn(env_name: String, value: String) -> Option<T>,
    name: &'static str,
}

impl<T> EnvVariable<T>
where
    T: Clone,
{
    // TODO: data here isn't cached now and every call will read from env.
    pub fn value(&self) -> T {
        match &self.value {
            EnvValue::Default(default_value) => match std::env::var(self.name) {
                Ok(ok_v) => {
                    let evaluate_result = match (self.evaluater)(String::from(self.name), ok_v) {
                        Some(v) => v,
                        None => {
                            warn!(
                                "Failed to evaluate variable '{}'. Using default value.",
                                self.name,
                            );
                            default_value.clone()
                        }
                    };
                    evaluate_result.clone()
                }
                Err(e) => {
                    if e != VarError::NotPresent {
                        warn!(
                            "Failed to read variable '{}'. Error: {e}. Using default value.",
                            self.name
                        );
                    }
                    default_value.clone()
                }
            },

            EnvValue::Some(v) => v.clone(),
        }
    }

    fn name(&self) -> &str {
        self.name
    }
}
