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

pub fn shell_args() -> Args {
    Args::parse()
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "AskaConfig.toml")]
    pub config: String,
}
