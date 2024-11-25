use core::panic;
use std::{fs::File, io::Read};

use clap::Parser;

pub fn get_json_value(content: &str, path: &str) -> Option<String> {
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(content) {
        let pointer = val.pointer(path)?;
        Some(pointer.to_string().replace("\"", ""))
    } else {
        None
    }
}

pub fn get_yaml_value(content: &str, path: &str) -> Option<String> {
    if let Ok(val) = serde_yaml::from_str::<serde_json::Value>(content) {
        let pointer = val.pointer(path)?;
        Some(pointer.to_string().replace("\"", ""))
    } else {
        None
    }
}

pub fn load_file(path: &str) -> Result<String, String> {
    let file = File::open(path);
    match file {
        Ok(mut opened_file) => {
            let mut contents = String::new();
            if let Err(e) = opened_file.read_to_string(&mut contents) {
                return Err(e.to_string());
            };
            Ok(contents)
        }
        Err(e) => Err(e.to_string()),
    }
}
pub fn load_files(pathes: Vec<String>) -> Result<(String, String), String> {
    for path in pathes {
        if let Ok(data) = load_file(path.as_str()) {
            return Ok((path.to_owned(), data));
        };
    }
    panic!("Config file not found.");
}

pub fn shell_args() -> Args {
    Args::parse()
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "AskaConfig.toml")]
    pub config: String,
}
