//! Config database.

use macros::Property;
use serde::Deserialize;
use std::{collections::HashMap, fmt::Debug};

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
        merged_config.unwrap_or_default()
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

/// Represents the configuration of the server.
///
/// For details see `asya-daemon/src/crates/macros/README.md`
#[derive(Debug, Property)]
#[property(name(ConfigProperty), derive(Deserialize, Default, Clone))]
pub struct Config {
    /// Net config group.
    #[property(default, use_type(NetProperty), mergeable)]
    pub net: Net,

    /// Logging config group.
    #[property(default, use_type(LoggingProperty), mergeable)]
    pub logging: Logging,

    // tg bot will be removed
    #[property(default, use_type(TelegramProperty), mergeable)]
    pub telegram: Telegram,

    /// Ai config group.
    #[property(default, use_type(AiProperty), mergeable)]
    pub ai: Ai,

    /// Plugins config group.
    #[property(default, use_type(PluginsProperty), mergeable)]
    pub plugins: Plugins,
}

#[derive(Debug, Property)]
#[property(name(PluginsProperty), derive(Deserialize, Default, Clone))]
pub struct Plugins {
    /// Folder which contains plugins.
    #[property(default("plugins".to_string()))]
    pub plugins_folder: String,

    #[property(default)]
    pub config: HashMap<String, String>,
}

#[derive(Debug, Property)]
#[property(name(AiProperty), derive(Deserialize, Default, Clone))]
pub struct Ai {
    /// Groq API token.
    ///
    /// Groq used for recognizing user input to commands and generating answers.
    #[property(default)]
    pub groq_token: String,

    /// Method used for ai recognizing user input.
    ///
    /// Currently we have:
    ///     * `Groq` - uses `console.groq.com`.
    ///     * `AltaS` - uses own model made by alta_s, currently work in progress.
    ///     * `None` - ai will not be used. This means you will be able to use only commands.
    #[property(default)]
    pub recognize_method: AiRecognizeMethod,

    /// The address where the alta_s model is hosted.
    #[property(default)]
    pub alta_s_addr: String,

    // Maybe remove this? alta_s model always should launch automatically.
    #[property(default)]
    pub autolaunch_alta_s: bool,

    /// Path with alta_s model for automatically launch.
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
    pub proxy_addr: String, // todo
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
    pub filescount: usize, // todo

    #[property(default)]
    pub stdout: bool,
}
