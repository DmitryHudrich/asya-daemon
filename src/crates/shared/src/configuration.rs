//! Environment variables.

use serde::Deserialize;
use std::fmt::Debug;

use crate::{types::AiRecognizeMethod, utils::file_utils};
use homedir::my_home;
use lazy_static::lazy_static;
use log::LevelFilter;
use mlua::{Lua, Table, ToLua};
use serde::{de::DeserializeOwned, Serialize};
use serde_env::from_env;

lazy_static! {
    #[derive(Serialize)]
    static ref ENV: serde_json::Value = {
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
        let merged = merge_struct::merge(&lua_config, &from_env::<Config>().unwrap()).unwrap();
        serde_json::to_value(merged).unwrap()
    };
}

pub(crate) fn get<T>(pointer: &str) -> T
where
    T: DeserializeOwned,
{
    serde_json::from_value::<T>(
        ENV.pointer(pointer)
            .unwrap_or_else(|| panic!("Failed to get config: {}", pointer))
            .clone(),
    )
    .unwrap_or_else(|_| panic!("Failed to get config: {}", pointer))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub(crate) net: Option<Net>,
    pub(crate) logging: Option<Logging>,
    pub(crate) telegram: Option<Telegram>,
    pub(crate) ai: Option<Ai>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ai {
    pub(crate) groq_token: Option<String>,
    pub(crate) recognize_method: Option<AiRecognizeMethod>,
    pub(crate) alta_s_addr: Option<String>,
    pub(crate) autolaunch_alta_s: Option<bool>,
    pub(crate) alta_s_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Telegram {
    pub(crate) token: Option<String>,
    pub(crate) accepted_users: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Net {
    pub(crate) http_port: Option<u16>,
    pub(crate) grpc_port: Option<u16>,
    pub(crate) proxy_addr: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Logging {
    pub(crate) place: Option<bool>,
    pub(crate) level: Option<LevelFilter>,
    pub(crate) folder: Option<String>,
    pub(crate) filescount: Option<usize>,
    pub(crate) stdout: Option<bool>,
}
