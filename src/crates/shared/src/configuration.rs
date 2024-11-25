//! Environment variables.

use std::fmt::Debug;

use crate::utils::file_utils;
use homedir::my_home;
use lazy_static::lazy_static;
use log::LevelFilter;
use log::*;
use mlua::{Lua, Table, ToLua};
use serde::{Deserialize, Serialize};
use serde_env::from_env;

lazy_static! {
    static ref ENV: Config = {
        let config_path = vec![format!(
            "{}/.config/asya/asya-config.lua",
            my_home().unwrap().unwrap().to_str().unwrap().to_string()
        )];
        let lua_config = {
            let lua = Lua::new();
            let (_, lua_file_content) = file_utils::load_files(config_path).expect("на всякий");

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

pub(crate) fn get() -> &'static Config {
    &ENV
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    net: Option<Net>,
    logging: Option<Logging>,
    telegram: Option<Telegram>,
    ai: Option<Ai>,
}

impl Config {
    pub fn net(&self) -> Net {
        self.net.clone().unwrap()
    }

    pub fn logging(&self) -> Logging {
        self.logging.clone().unwrap()
    }
    pub fn telegram(&self) -> Telegram {
        self.telegram.clone().unwrap()
    }

    pub fn ai(&self) -> Ai {
        self.ai.clone().unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ai {
    groq_token: Option<String>,
    recognize_method: Option<AiRecognizeMethod>,
    alta_s_addr: Option<String>,
    autolaunch_alta_s: Option<bool>,
    alta_s_path: Option<String>,
}

impl Ai {
    pub fn recognize_method(&self) -> Option<AiRecognizeMethod> {
        self.recognize_method.clone()
    }

    pub fn groq_token(&self) -> Option<String> {
        self.groq_token.clone()
    }

    pub fn autolaunch_alta_s(&self) -> Option<bool> {
        self.autolaunch_alta_s
    }

    pub fn alta_s_path(&self) -> Option<String> {
        self.alta_s_path.clone()
    }

    pub fn alta_s_addr(&self) -> Option<String> {
        self.alta_s_addr.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum AiRecognizeMethod {
    Groq,
    AltaS,
    #[default]
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Telegram {
    token: Option<String>,
    accepted_users: Option<Vec<String>>,
}

impl Telegram {
    pub fn token(&self) -> Option<String> {
        self.token.clone()
    }
    pub fn accepted_users(&self) -> Option<Vec<String>> {
        self.accepted_users.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Net {
    http_port: Option<u16>,
    grpc_port: Option<u16>,
    proxy_addr: Option<String>,
}

impl Net {
    pub fn http_port(&self) -> Option<u16> {
        Some(
            self.http_port
                .unwrap_or_else(|| use_default("http_port", 3001)),
        )
    }
    pub fn proxy_addr(&self) -> Option<String> {
        self.proxy_addr.clone()
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
    pub fn place(&self) -> Option<bool> {
        Some(self.place.unwrap_or_else(|| use_default("place", false)))
    }

    pub fn level(&self) -> Option<LevelFilter> {
        Some(
            self.level
                .unwrap_or_else(|| use_default("level", LevelFilter::Info)),
        )
    }

    pub fn folder(&self) -> Option<String> {
        Some(
            self.folder
                .clone()
                .unwrap_or_else(|| use_default("folder", String::from("logs"))),
        )
    }

    // pub fn filescount(&self) -> usize {
    //     self.filescount
    //         .unwrap_or_else(|| use_default("filescount", 10))
    // }

    pub fn stdout(&self) -> Option<bool> {
        Some(self.stdout.unwrap_or_else(|| use_default("stdout", true)))
    }
}

// todo: нужно сделать так, чтобы значения по умолчанию объявлялись на месте, а не тут
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
