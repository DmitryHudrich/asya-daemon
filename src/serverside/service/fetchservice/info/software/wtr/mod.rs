use serde_json::json;
use core::str;
use std::process::Command;
use reqwest::blocking::get;


type Json = serde_json::Value;


fn get_ip () -> String {
    let response = get("http://ipinfo.io/json")?.text()?;
    let json: Value = serde_json::from_str(&response)?;
    Ok(json["city"].as_str().unwrap_or("Unknown").to_string())
}


pub fn get_wthr(_: String) -> Json {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("curl -s 'wttr.in/{}?format=2'", get_ip()))
        .output()
        .unwrap();

    let out = str::from_utf8(&output.stdout).expect("Cant parse");
    json!({"weather": out.trim()})
}
