// G5d7L3yHNcBkiqGjhF563V1o2IUmshyI

use log::error;
use reqwest::Client;
use serde_json::json;
use shared::state;

pub async fn send_request(req: String) -> String {
    let client = Client::new();

    // URL и заголовки
    let url = "https://api.groq.com/openai/v1/chat/completions";
    let api_key = state::get_mistral_token().await.unwrap_or_else(|| {
        error!("Mistral api key not found. Skip.");
        String::default()
    });

    let body = json!({
        "messages": [
            { 
                "role": "user", 
                "content": req
            }
        ],
        "model": "llama3-8b-8192",
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

    response.text().await.unwrap()
}
