// G5d7L3yHNcBkiqGjhF563V1o2IUmshyI

use log::{error, warn};
use reqwest::Client;
use serde_json::json;
use shared::state;

pub async fn send_request(req: String) -> String {
    let client = Client::new();

    // URL и заголовки
    let url = "https://api.mistral.ai/v1/agents/completions";
    let api_key = state::get_mistral_token().await.unwrap_or_else(|| {
        error!("Mistral api key not found. Skip.");
        String::default()
    });

    let body = json!({
        "agent_id": "ag:f85c857b:20241008:untitled-agent:1990d69c",
        "messages": [
            {
                "role": "user",
                "content": req
            }
        ]
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

    response.text().await.unwrap()
}
