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
                    warn!("Failed to parse port. {name}: {}", err);
                    use_default()
                },
            }
        },
        name: vec!["NET", "PORT"],
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
        name: vec!["LOGGING", "LEVEL"],
    };

    pub static ref LOGGING_FOLDER: EnvVariable<String> = EnvVariable {
        value: EnvValue::Default("logs".to_string()),
        evaluater: |_, value| Some(value),
        name: vec!["LOGGING", "FOLDER"]
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
        name: vec!["LOGGING", "FILESCOUNT"],
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
        name: vec!["LOGGING", "STDOUT"],
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
        name: vec!["LOGGING", "PLACE"],
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
    name: Vec<&'static str>,
}

impl<T> EnvVariable<T>
where
    T: Clone,
{
    // TODO: data here isn't cached now and every call will read from env.
    // TODO: read message below.
    pub fn value(&self) -> T {
        /*
         * я короче хз как сделать находящийся ниже вывод лога потому что логгер билдится исходя из параметров
         * конфигурации, т.е. логгировать доступ к этим параметрам нельзя потому что логгера еще не
         * суещствует во время доступа к конфигу. из за этого не получается выводить некоторые
         * важные сообщения и все такое. скорее всего мы будем вытаскивать эти значения напрямую
         * при постройке логгера, обходя
         * логику в этом файле.
         */

        // debug!(
        //     "CONFIGURATION DEBUG:EnvVariable\t\t{}: {}",
        //     self.name,
        //     std::env::var(self.name).unwrap_or("ERROR DUE PARSING".to_string())
        // );

        let env_variable = self.name.clone().join("_").to_uppercase();

        match &self.value {
            EnvValue::Default(default_value) => match std::env::var(env_variable.clone()) {
                Ok(env_var_value) => self.handle_ok(env_variable, env_var_value, default_value),
                Err(error) => self.handle_err(error, env_variable, default_value),
            },

            EnvValue::Some(v) => v.clone(),
        }
    }

    fn handle_ok(&self, env_variable: String, env_var_value: String, default_value: &T) -> T {
        let evaluated_result = match (self.evaluater)(env_variable.clone(), env_var_value) {
            Some(v) => v,
            None => {
                warn!(
                    "Failed to evaluate variable '{}'. Using default value.",
                    env_variable,
                );
                default_value.clone()
            }
        };
        evaluated_result.clone()
    }

    fn handle_err(&self, error: VarError, env_variable: String, default_value: &T) -> T {
        if error != VarError::NotPresent {
            warn!(
                "Failed to read variable '{}'. Error: {error}. Using default value.",
                env_variable,
            );
        }
        default_value.clone()
    }

    fn name(&self) -> Vec<String> {
        self.name
            .clone()
            .into_iter()
            .map(|element| element.to_string())
            .collect()
    }
}
