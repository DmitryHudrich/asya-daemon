// G5d7L3yHNcBkiqGjhF563V1o2IUmshyI

use std::str::FromStr;

use log::error;
use reqwest::Client;
use serde_json::json;
use shared::state;

pub async fn send_request(req: String) -> String {
    let client = Client::new();
    let url = "https://api.groq.com/openai/v1/chat/completions";
    let api_key = state::get_mistral_token().await.unwrap_or_else(|| {
        error!("Mistral api key not found. Skip.");
        String::default()
    });
    let r = req + &String::from_str("").unwrap();

    let body = json!({
        "messages": [
            {
                "role": "user",
                "content": r
            }
        ],
        "model": "llama-3.1-70b-versatile",
        "temperature": 0.7
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await
        .unwrap();

    let val: serde_json::Value = serde_json::from_str(response.text().await.unwrap().as_str()).unwrap();
    val
        .pointer("/choices/0/message/content")
        .unwrap()
        .to_string()
        .replace("\"", "")
}
