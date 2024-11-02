//! Environment variables.

use std::fmt::Debug;

use crate::utils::file_utils;
use lazy_static::lazy_static;
use log::LevelFilter;
use log::*;
use mlua::{Lua, Table, ToLua};
use mlua_serde;
use serde::{Deserialize, Serialize};
use serde_env::from_env;

const CONFIGS_PATH: [&str; 1] = ["aska-config-internal.lua"];

lazy_static! {
    static ref ENV: Config = {
        let lua_config = {
            let lua = Lua::new();
            let (_, lua_file_content) =
                file_utils::load_files(CONFIGS_PATH.to_vec()).expect("на всякий");

            let config_lua: Table = lua
                .load(&lua_file_content)
                .eval()
                .expect("Failed to evaluate Lua configuration.");

            let config: Config = mlua_serde::from_value(config_lua.to_lua(&lua).unwrap())
                .expect("Failed to deserialize Lua config to Rust structure.");
            config
        };
        merge_struct::merge(&lua_config, &from_env::<Config>().unwrap()).unwrap()
    };
}

pub fn get() -> &'static Config {
    &ENV
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    http_port: Option<u16>,
    grpc_port: Option<u16>,
}

impl Net {
    pub fn http_port(&self) -> u16 {
        self.http_port
            .unwrap_or_else(|| use_default("http_port", 3001))
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

    // pub fn filescount(&self) -> usize {
    //     self.filescount
    //         .unwrap_or_else(|| use_default("filescount", 10))
    // }

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
        _ = config.net().http_port();
        _ = config.logging().place();
        _ = config.logging().level();
        // _ = config.logging().filescount();
        _ = config.logging().stdout();
        _ = config.logging().folder();
    }

    #[test]
    fn port() {
        let config_with_default_port = default_config();
        let config_without_port = {
            let mut c = default_config();
            c.net.as_mut().unwrap().http_port = None;
            c
        };

        let config_wit_custom_port = {
            let mut c = default_config();
            c.net.as_mut().unwrap().http_port = Some(2000);
            c
        };

        assert_eq!(config_with_default_port.net().http_port(), 3000);
        assert_eq!(config_without_port.net().http_port(), 3000);
        assert_eq!(config_wit_custom_port.net().http_port(), 2000);
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

    // #[test]
    // fn filescount() {
    //     let config_with_default_filescount = default_config();
    //     let config_without_filescount = {
    //         let mut c = default_config();
    //         c.logging.as_mut().unwrap().filescount = None;
    //         c
    //     };
    //     let config_wit_custom_filescount = {
    //         let mut c = default_config();
    //         c.logging.as_mut().unwrap().filescount = Some(15);
    //         c
    //     };
    //
    //     assert_eq!(config_with_default_filescount.logging().filescount(), 10);
    //     assert_eq!(config_without_filescount.logging().filescount(), 10);
    //     assert_eq!(config_wit_custom_filescount.logging().filescount(), 15);
    // }

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
            net: Some(Net {
                http_port: Some(3000),
                grpc_port: Some(50051),
            }),
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
