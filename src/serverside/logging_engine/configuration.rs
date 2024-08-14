//! Environment variables.

use lazy_static::lazy_static;
use log::LevelFilter;

lazy_static! {
    pub static ref LOGGING_LEVEL: EnvVariable<LevelFilter> = EnvVariable {
        value: EnvValue::Default(LevelFilter::Info),
        evaluater: |name, value| {
            match value.as_str() {
                "TRACE" => Some(LevelFilter::Trace),
                "DEBUG" => Some(LevelFilter::Debug),
                "INFO" => Some(LevelFilter::Info),
                "WARN" => Some(LevelFilter::Warn),
                "ERROR" => Some(LevelFilter::Error),
                _ => use_default(name),
            }
        },
        name: "LOG_LEVEL",
    };
    pub static ref LOGGING_FOLDER: EnvVariable<String> = EnvVariable {
        value: EnvValue::Default("logs".to_string()),
        evaluater: |_, value| Some(value),
        name: "LOG_PLACE",
    };
    pub static ref LOGGING_FILESCOUNT: EnvVariable<usize> = EnvVariable {
        value: EnvValue::Default(20),
        evaluater: |name, value| {
            match value.parse::<usize>() {
                Ok(number) => Some(number),
                Err(e) => {
                    println!("Failed to parse {name}: {}", e);
                    use_default(name)
                }
            }
        },
        name: "LOGFILES_COUNT",
    };
    pub static ref LOGGING_STDOUT: EnvVariable<bool> = EnvVariable {
        value: EnvValue::Default(true),
        evaluater: |name, value| {
            match value.as_str() {
                "0" => Some(false),
                "1" => Some(true),
                _ => use_default(name),
            }
        },
        name: "LOG_CONSOLE",
    };
    pub static ref LOG_PLACE: EnvVariable<bool> = EnvVariable {
        value: EnvValue::Default(false),
        evaluater: |name, value| {
            match value.as_str() {
                "0" => Some(false),
                "1" => Some(true),
                _ => use_default(name),
            }
        },
        name: "LOG_PLACE",
    };
}

/*------------------end of configuration-----------------------------------------------*/
/* ------------------------------------------------------------------------------------*/

fn use_default<T>(env_name: String) -> Option<T> {
    warn!("Failed to evaluate variable '{env_name}'. Using default value.");
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
                    warn!(
                        "Failed to read variable '{}'. Error: {e}. Using default value.",
                        self.name
                    );
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
