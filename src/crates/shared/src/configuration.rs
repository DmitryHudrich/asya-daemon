//! Config database.

use macros::Property;
use serde::Deserialize;
use std::fmt::Debug;

use crate::types::AiRecognizeMethod;
use homedir::my_home;
use lazy_static::lazy_static;
use log::LevelFilter;
use mlua::{Lua, Table, ToLua};

lazy_static! {
    pub static ref CONFIG: Config = {
        let config_path = vec![format!(
            "{}/.config/asya/asya-config.lua",
            my_home().unwrap().unwrap().to_str().unwrap().to_string()
        )];

        let lua_config = {
            let lua = Lua::new();
            let (_config_path, lua_file_content) =
                load_any_file(config_path).expect("Config file must be reachable");

            let config_lua: Table = lua
                .load(&lua_file_content)
                .eval()
                .expect("Lua configuration file must be correct to evaluate");

            let config: ConfigProperty = mlua_serde::from_value(config_lua.to_lua(&lua).unwrap())
                .expect("Lua config table must be correct to desiralize into Rust struct");

            config
        };

        let merged_config = lua_config.merge(serde_env::from_env().unwrap());
        merged_config.verify().unwrap();
        dbg!(merged_config.unwrap_or_default())
    };
}

pub fn load_any_file(pathes: Vec<String>) -> Result<(String, String), String> {
    pathes
        .into_iter()
        .find_map(|path| {
            std::fs::read_to_string(&path)
                .map(|content| (path, content))
                .ok()
        })
        .ok_or("Config file not found".to_owned())
}

#[derive(Debug, Property)]
#[property(name(ConfigProperty), derive(Deserialize, Default, Clone))]
pub struct Config {
    #[property(default, use_type(NetProperty), mergeable)]
    pub net: Net,

    #[property(default, use_type(LoggingProperty), mergeable)]
    pub logging: Logging,

    #[property(default, use_type(TelegramProperty), mergeable)]
    pub telegram: Telegram,

    #[property(default, use_type(AiProperty), mergeable)]
    pub ai: Ai,
}

#[derive(Debug, Property)]
#[property(name(AiProperty), derive(Deserialize, Default, Clone))]
pub struct Ai {
    #[property(default)]
    pub groq_token: String,

    #[property(default)]
    pub recognize_method: AiRecognizeMethod,

    #[property(default)]
    pub alta_s_addr: String,

    #[property(default)]
    pub autolaunch_alta_s: bool,

    #[property(default)]
    pub alta_s_path: String,
}

#[derive(Debug, Property)]
#[property(name(TelegramProperty), derive(Deserialize, Default, Clone))]
pub struct Telegram {
    #[property(default)]
    pub token: String,

    #[property(default)]
    pub accepted_users: Vec<String>,
}

#[derive(Debug, Property)]
#[property(name(NetProperty), derive(Deserialize, Default, Clone))]
pub struct Net {
    #[property(default)]
    pub http_port: u16,

    #[property(default)]
    pub grpc_port: u16,

    #[property(default)]
    pub proxy_addr: String,
}

#[derive(Debug, Property)]
#[property(name(LoggingProperty), derive(Deserialize, Default, Clone))]
pub struct Logging {
    #[property(default)]
    pub place: bool,

    #[property(default(LevelFilter::Info))]
    pub level: LevelFilter,

    #[property(default)]
    pub folder: String,

    #[property(default)]
    pub filescount: usize,

    #[property(default)]
    pub stdout: bool,
}
