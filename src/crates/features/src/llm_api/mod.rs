use std::str::FromStr;

use log::error;
use reqwest::{Client, Proxy};
use serde_json::json;
use shared::{state, utils::file_utils};

pub async fn send_request(req: String) -> String {
    let client = construct_reqwest_client().await;
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

    let response = request(client, url, api_key, body).await;

    let response_res = response.text().await;
    let temp = &response_res.unwrap();
    let response_text = temp.as_str();
    file_utils::get_json_value(response_text, "/choices/0/message/content")
        .expect("Error while handling response from llm api.")
}

async fn request(
    client: Client,
    url: &str,
    api_key: String,
    body: serde_json::Value,
) -> reqwest::Response {
    client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await
        .unwrap()
}

async fn construct_reqwest_client() -> Client {
    if let Some(proxy_addr) = state::get_proxy_addr().await {
        let proxy = Proxy::http(proxy_addr).expect("Error while proxy setup.");
        Client::builder()
            .proxy(proxy)
            .build()
            .expect("Error while building client with proxy.")
    } else {
        Client::new()
    }
}
