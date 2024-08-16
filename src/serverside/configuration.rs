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
            toml::de::from_str::<Config>(utils::load_file("AskaConfig.toml").unwrap().as_str())
                .unwrap();
        let env_config = from_env::<Config>().unwrap();
        merge_struct::merge(&toml_config, &env_config).unwrap()
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
        self.port.unwrap()
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
        self.place.unwrap()
    }

    pub fn level(&self) -> LevelFilter {
        self.level.unwrap()
    }

    pub fn folder(&self) -> &String {
        self.folder.as_ref().expect("bebra")
    }

    pub fn filescount(&self) -> usize {
        self.filescount.unwrap()
    }

    pub fn stdout(&self) -> bool {
        self.stdout.unwrap()
    }
}
