//! Environment variables.

use serde::Deserialize;
use std::fmt::Debug;

use crate::types::AiRecognizeMethod;
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
            let (_config_path, lua_file_content) = load_any_file(config_path).expect("Config file must be reachable");

            let config_lua: Table = lua
                .load(&lua_file_content)
                .eval()
                .expect("Lua configuration file must be correct to evaluate");

            let config: Config = mlua_serde::from_value(config_lua.to_lua(&lua).unwrap())
                .expect("Lua config table must be correct to desiralize into Rust struct");

            config
        };

        let merged = merge_struct::merge(&lua_config, &from_env::<Config>().unwrap()).unwrap();
        serde_json::to_value(merged).unwrap()
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

pub(crate) fn get<T>(pointer: &str) -> T
where
    T: DeserializeOwned,
{
    ENV.pointer(pointer)
        .and_then(|ref_val| serde_json::from_value::<T>(ref_val.to_owned()).ok())
        .expect(&format!("The config '{pointer}' must be reachable"))
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
