//! Environment variables.

use lazy_static::lazy_static;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use serde_env::from_env;

use crate::utils;

/*
* Нужно как то уменьшить весь этот бойлерплейт из геттеров, глаза мазолит. может быть макросами?
*/

lazy_static! {
    static ref ENV: Config = {
        let toml_config =
            toml::de::from_str::<Config>(utils::load_file(utils::shell_args()
                                                            .config
                                                            .as_str()
                                                        ).unwrap()
                                                        .as_str())
                                                        .unwrap();
        merge_struct::merge(&toml_config, &from_env::<Config>().unwrap()).unwrap()
    };
}

pub fn get() -> &'static Config {
    &ENV
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    net: Option<Net>,
    logging: Option<Logging>,
}

impl Config {
    pub fn net(&self) -> Net {
        self.net.clone().unwrap()
    }

    pub fn logging(&self) -> Logging {
        self.logging.clone().unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Net {
    port: Option<u16>,
}

impl Net {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or_else(|| use_default("port", 3000))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Logging {
    place: Option<bool>,
    level: Option<LevelFilter>,
    folder: Option<String>,
    filescount: Option<usize>,
    stdout: Option<bool>,
}

impl Logging {
    pub fn place(&self) -> bool {
        debug!("getting config property: logging.place");
        self.place.unwrap_or_else(|| use_default("place", false))
    }

    pub fn level(&self) -> LevelFilter {
        self.level
            .unwrap_or_else(|| use_default("level", LevelFilter::Info))
    }

    pub fn folder(&self) -> String {
        self.folder
            .clone()
            .unwrap_or_else(|| use_default("folder", String::from("logs")))
    }

    pub fn filescount(&self) -> usize {
        self.filescount
            .unwrap_or_else(|| use_default("filescount", 10))
    }

    pub fn stdout(&self) -> bool {
        self.stdout.unwrap_or_else(|| use_default("stdout", true))
    }
}

fn use_default<T>(field: &str, value: T) -> T
where
    T: std::fmt::Debug,
{
    warn!(
        "Field {} not found, using default value: {:?}",
        field, value
    );
    value
}

#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use super::{get, Config, Logging, Net};

    #[test] // Should not paniced
    fn config_getters() {
        let config = get();
        _ = config.net().port();
        _ = config.logging().place();
        _ = config.logging().level();
        _ = config.logging().filescount();
        _ = config.logging().stdout();
        _ = config.logging().folder();
    }

    #[test]
    fn port() {
        let config_with_default_port = default_config();
        let config_without_port = {
            let mut c = default_config();
            c.net.as_mut().unwrap().port = None;
            c
        };

        let config_wit_custom_port = {
            let mut c = default_config();
            c.net.as_mut().unwrap().port = Some(2000);
            c
        };

        assert_eq!(config_with_default_port.net().port(), 3000);
        assert_eq!(config_without_port.net().port(), 3000);
        assert_eq!(config_wit_custom_port.net().port(), 2000);
    }

    #[test]
    fn place() {
        let config_with_default_place = default_config();
        let config_without_place = {
            let mut c = default_config();
            c.logging.as_mut().unwrap().place = None;
            c
        };
        let config_wit_custom_place = {
            let mut c = default_config();
            c.logging.as_mut().unwrap().place = Some(true);
            c
        };

        assert!(!config_with_default_place.logging().place());
        assert!(!config_without_place.logging().place());
        assert!(config_wit_custom_place.logging().place());
    }

    #[test]
    fn level() {
        let config_with_default_level = default_config();
        let config_without_level = {
            let mut c = default_config();
            c.logging.as_mut().unwrap().level = None;
            c
        };
        let config_with_warn = {
            let mut c = default_config();
            c.logging.as_mut().unwrap().level = Some(LevelFilter::Warn);
            c
        };

        assert_eq!(
            config_with_default_level.logging().level(),
            LevelFilter::Info
        );
        assert_eq!(config_without_level.logging().level(), LevelFilter::Info);
        assert_eq!(config_with_warn.logging().level(), LevelFilter::Warn);
    }

    #[test]
    fn folder() {
        let config_with_default_folder = default_config();
        let config_without_folder = {
            let mut c = default_config();
            c.logging.as_mut().unwrap().folder = None;
            c
        };
        let config_wit_custom_folder = {
            let mut c = default_config();
            c.logging.as_mut().unwrap().folder = Some("aska_logs".to_owned());
            c
        };
        assert_eq!(config_with_default_folder.logging().folder(), "logs");
        assert_eq!(config_without_folder.logging().folder(), "logs");
        assert_eq!(config_wit_custom_folder.logging().folder(), "aska_logs");
    }

    #[test]
    fn filescount() {
        let config_with_default_filescount = default_config();
        let config_without_filescount = {
            let mut c = default_config();
            c.logging.as_mut().unwrap().filescount = None;
            c
        };
        let config_wit_custom_filescount = {
            let mut c = default_config();
            c.logging.as_mut().unwrap().filescount = Some(15);
            c
        };

        assert_eq!(config_with_default_filescount.logging().filescount(), 10);
        assert_eq!(config_without_filescount.logging().filescount(), 10);
        assert_eq!(config_wit_custom_filescount.logging().filescount(), 15);
    }

    #[test]
    fn stdout() {
        let config_with_default_stdout = default_config();
        let config_without_stdout = {
            let mut c = default_config();
            c.logging.as_mut().unwrap().stdout = None;
            c
        };
        let config_wit_custom_stdout = {
            let mut c = default_config();
            c.logging.as_mut().unwrap().stdout = Some(false);
            c
        };

        assert!(config_with_default_stdout.logging().stdout());
        assert!(config_without_stdout.logging().stdout());
        assert!(!config_wit_custom_stdout.logging().stdout());
    }

    fn default_config() -> Config {
        Config {
            net: Some(Net { port: Some(3000) }),
            logging: Some(Logging {
                place: Some(false),
                level: Some(log::LevelFilter::Info),
                folder: Some("logs".to_owned()),
                filescount: Some(10),
                stdout: Some(true),
            }),
        }
    }
}
